use std::collections::HashMap;
use std::sync::Arc;

use crate::{AiProvider, ProviderId, ModelInfo};

pub struct ProviderRegistry {
    providers: HashMap<ProviderId, Arc<dyn AiProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register(&mut self, provider: Arc<dyn AiProvider>) {
        self.providers.insert(provider.id().clone(), provider);
    }

    pub fn unregister(&mut self, id: &ProviderId) {
        self.providers.remove(id);
    }

    pub fn get(&self, id: &ProviderId) -> Option<&Arc<dyn AiProvider>> {
        self.providers.get(id)
    }

    pub fn list_providers(&self) -> Vec<&Arc<dyn AiProvider>> {
        self.providers.values().collect()
    }

    pub async fn all_models(&self) -> Vec<ModelInfo> {
        let mut models = Vec::new();
        for provider in self.providers.values() {
            if let Ok(provider_models) = provider.list_models().await {
                models.extend(provider_models);
            }
        }
        models
    }
}
