# Security Policy

## Supported Versions

| Version | Supported |
| ------- | --------- |
| 0.1.x   | Active development |

## Reporting a Vulnerability

**Do not report security vulnerabilities through public GitHub issues.**

Instead, report them through GitHub Security Advisories:
https://github.com/FeverDream/FeverThoth/security/advisories/new

You can also email security@feverdream.dev.

## What to Include

- Description of the vulnerability
- Steps to reproduce
- Affected versions
- Potential impact
- Suggested fix (if you have one)

## Response Timeline

- Acknowledgment within 48 hours
- Initial assessment within 5 business days
- Fix timeline communicated based on severity

## Security Features

FeverThoth IDE is designed with privacy-first principles:
- Screenshots are analyzed locally by default (Ollama)
- Raw screenshots never leave the machine without explicit user permission
- Structured text summaries are the only data sent to cloud providers by default
- All agent actions are logged in an audit trail
- Shell commands from AI require user review
