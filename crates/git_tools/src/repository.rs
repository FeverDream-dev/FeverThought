use crate::types::*;
use anyhow::{Context, Result};
use std::path::Path;
use tokio::process::Command;

pub struct GitRepository {
    path: std::path::PathBuf,
}

impl GitRepository {
    pub async fn open(path: &Path) -> Result<Self> {
        let repo = Self { path: path.to_path_buf() };
        repo.run_git(&["rev-parse", "--git-dir"]).await?;
        Ok(repo)
    }

    pub async fn current_branch(&self) -> Result<String> {
        let output = self.run_git(&["rev-parse", "--abbrev-ref", "HEAD"]).await?;
        Ok(output.trim().to_string())
    }

    pub async fn status(&self) -> Result<GitStatus> {
        let output = self.run_git(&["status", "--porcelain=v1"]).await?;
        let mut staged = Vec::new();
        let mut unstaged = Vec::new();
        let mut untracked = Vec::new();

        for line in output.lines() {
            if line.len() < 4 {
                continue;
            }
            let index_status = line.chars().next().unwrap_or(' ');
            let worktree_status = line.chars().nth(1).unwrap_or(' ');
            let file_path = line[3..].to_string();

            match index_status {
                'A' | 'M' | 'D' | 'R' | 'C' => {
                    staged.push(FileStatus {
                        path: file_path.clone(),
                        status: parse_change_type(index_status),
                    });
                }
                _ => {}
            }

            match worktree_status {
                'M' | 'D' => {
                    unstaged.push(FileStatus {
                        path: file_path.clone(),
                        status: parse_change_type(worktree_status),
                    });
                }
                '?' => {
                    untracked.push(file_path);
                }
                _ => {}
            }
        }

        Ok(GitStatus { staged, unstaged, untracked })
    }

    pub async fn diff(&self, file: Option<&str>) -> Result<String> {
        let mut args = vec!["diff"];
        if let Some(f) = file {
            args.push("--");
            args.push(f);
        }
        self.run_git(&args).await
    }

    pub async fn commit(&self, message: &str, files: &[&str]) -> Result<()> {
        if !files.is_empty() {
            let mut add_args = vec!["add", "--"];
            add_args.extend(files.iter().map(|s| *s));
            self.run_git(&add_args).await?;
        }
        self.run_git(&["commit", "-m", message]).await?;
        Ok(())
    }

    pub async fn checkout(&self, branch: &str) -> Result<()> {
        self.run_git(&["checkout", branch]).await?;
        Ok(())
    }

    pub async fn list_branches(&self) -> Result<Vec<BranchInfo>> {
        let output = self.run_git(&["branch", "-a", "--no-color"]).await?;
        let mut branches = Vec::new();

        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let is_current = line.starts_with('*');
            let name = trimmed.trim_start_matches("* ").trim();

            if name.starts_with("remotes/") {
                branches.push(BranchInfo {
                    name: name.to_string(),
                    is_current: false,
                    is_remote: true,
                });
            } else if name != "" {
                branches.push(BranchInfo {
                    name: name.to_string(),
                    is_current,
                    is_remote: false,
                });
            }
        }

        Ok(branches)
    }

    pub async fn log(&self, count: usize) -> Result<Vec<CommitInfo>> {
        let count_str = count.to_string();
        let format = "--pretty=format:%H%n%s%n%an%n%aI%n---";
        let output = self.run_git(&["log", &count_str, format]).await?;

        let mut commits = Vec::new();
        for entry in output.split("---") {
            let lines: Vec<&str> = entry.lines().collect();
            if lines.len() >= 4 {
                commits.push(CommitInfo {
                    hash: lines[0].trim().to_string(),
                    message: lines[1].trim().to_string(),
                    author: lines[2].trim().to_string(),
                    timestamp: lines[3].trim().to_string(),
                });
            }
        }

        Ok(commits)
    }

    async fn run_git(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.path)
            .output()
            .await
            .with_context(|| format!("Failed to execute git {}", args.join(" ")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git {} failed: {}", args.join(" "), stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

fn parse_change_type(c: char) -> FileChangeType {
    match c {
        'A' => FileChangeType::Added,
        'M' => FileChangeType::Modified,
        'D' => FileChangeType::Deleted,
        'R' => FileChangeType::Renamed,
        'C' => FileChangeType::Copied,
        _ => FileChangeType::Modified,
    }
}
