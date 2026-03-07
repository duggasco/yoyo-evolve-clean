//! Local LLM provider wrapper for OpenAI-compatible endpoints (e.g., llama.cpp).

use std::collections::HashMap;
use yoagent::provider::{
    ApiProtocol, CostConfig, ModelConfig, OpenAiCompat, OpenAiCompatProvider, ProviderError,
    StreamConfig, StreamEvent, StreamProvider,
};
use yoagent::Message;

/// A provider that wraps `OpenAiCompatProvider` and injects a `ModelConfig`
/// pointing at a local OpenAI-compatible endpoint (llama.cpp, Ollama, vLLM, etc.).
///
/// yoagent's agent loop sets `StreamConfig::model_config` to `None`, which causes
/// `OpenAiCompatProvider` to fail. This wrapper fills in the config before delegating.
pub struct LocalProvider {
    base_url: String,
    context_window: u32,
    headers: HashMap<String, String>,
}

impl LocalProvider {
    pub fn new(base_url: &str, context_window: u32) -> Self {
        // Auto-inject OpenRouter recommended headers if targeting openrouter.ai
        let mut headers = HashMap::new();
        if base_url.contains("openrouter.ai") {
            headers.insert(
                "HTTP-Referer".to_string(),
                "https://github.com/duggasco/yoyo-evolve".to_string(),
            );
            headers.insert("X-Title".to_string(), "yoyo-evolve".to_string());
        }
        Self {
            base_url: base_url.to_string(),
            context_window,
            headers,
        }
    }

    fn model_config(&self, model: &str) -> ModelConfig {
        ModelConfig {
            id: model.to_string(),
            name: model.to_string(),
            api: ApiProtocol::OpenAiCompletions,
            provider: "local".to_string(),
            base_url: self.base_url.clone(),
            reasoning: false,
            context_window: self.context_window,
            max_tokens: 4096,
            cost: CostConfig::default(),
            headers: self.headers.clone(),
            compat: Some(OpenAiCompat::default()),
        }
    }
}

#[async_trait::async_trait]
impl StreamProvider for LocalProvider {
    async fn stream(
        &self,
        mut config: StreamConfig,
        tx: tokio::sync::mpsc::UnboundedSender<StreamEvent>,
        cancel: tokio_util::sync::CancellationToken,
    ) -> Result<Message, ProviderError> {
        config.model_config = Some(self.model_config(&config.model));
        OpenAiCompatProvider.stream(config, tx, cancel).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_provider_model_config() {
        let provider = LocalProvider::new("http://192.168.1.128:8080", 32768);
        let config = provider.model_config("qwen3-coder-next");

        assert_eq!(config.id, "qwen3-coder-next");
        assert_eq!(config.base_url, "http://192.168.1.128:8080");
        assert_eq!(config.context_window, 32768);
        assert_eq!(config.provider, "local");
        assert_eq!(config.api, ApiProtocol::OpenAiCompletions);
        assert!(config.compat.is_some());
    }

    #[test]
    fn test_local_provider_different_url() {
        let provider = LocalProvider::new("http://localhost:11434", 65536);
        let config = provider.model_config("llama3");

        assert_eq!(config.base_url, "http://localhost:11434");
        assert_eq!(config.context_window, 65536);
    }
}
