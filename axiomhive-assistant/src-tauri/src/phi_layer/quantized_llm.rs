use crate::lambda_core::constitutional_engine::ValidatedPrompt;
use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::quantized_llama::Model as MistralModel;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use tokenizers::Tokenizer;

/// Quantized Phi layer model wrapper built on top of Candle.
pub struct QuantizedLLM {
    model: MistralModel,
    tokenizer: Tokenizer,
    device: Device,
}

/// High level Phi-layer orchestrator that combines quantized inference with sparse activation.
pub struct PhiLayer {
    llm: Option<QuantizedLLM>,
    sparse_engine: SparseActivationEngine,
}

impl PhiLayer {
    pub fn new() -> Self {
        let llm = QuantizedLLM::new().ok();
        Self {
            llm,
            sparse_engine: SparseActivationEngine::new(),
        }
    }

    pub fn with_llm(llm: QuantizedLLM) -> Self {
        Self {
            llm: Some(llm),
            sparse_engine: SparseActivationEngine::new(),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.llm.is_some()
    }

    /// Generate a Phi-layer response using the validated prompt and sparse mask.
    pub async fn generate_response(&mut self, prompt: &ValidatedPrompt) -> Result<String> {
        self.sparse_engine.apply_mask(&prompt.activation_mask);

        match self.llm.as_mut() {
            Some(llm) => llm.generate(prompt).await,
            None => Err(E::msg("Quantized Phi layer is not initialized")),
        }
    }

    pub fn sparsity_ratio(&self) -> f32 {
        self.sparse_engine.sparsity_ratio()
    }
}

impl QuantizedLLM {
    pub fn new() -> Result<Self> {
        let device = Device::Cpu;
        let model_path = PathBuf::from(
            env::var("MODEL_PATH")
                .unwrap_or_else(|_| "models/mistral-7b-instruct-v0.3.Q4_K_M.gguf".to_string()),
        );
        let tokenizer_path =
            PathBuf::from(env::var("TOKENIZER_PATH").unwrap_or_else(|_| "models/tokenizer.json".to_string()));

        if !model_path.exists() {
            return Err(E::msg("Model file not found"));
        }
        if !tokenizer_path.exists() {
            return Err(E::msg(
                "Tokenizer file not found. Please download it and place it in the models directory.",
            ));
        }

        let mut file = File::open(&model_path)?;
        let model = MistralModel::from_gguf(&mut file, &device)?;
        let tokenizer = Tokenizer::from_file(tokenizer_path).map_err(E::msg)?;

        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }

    pub async fn generate(&mut self, prompt: &ValidatedPrompt) -> Result<String> {
        let mut tokens = self
            .tokenizer
            .encode(&prompt.content, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();

        let max_tokens = 100;
        let mut logits_processor = LogitsProcessor::new(299_792_458, Some(0.7), Some(0.9));
        let mut result = String::new();

        for index in 0..max_tokens {
            let context_size = if index > 0 { 1 } else { tokens.len() };
            let start_pos = tokens.len().saturating_sub(context_size);
            let input = Tensor::new(&tokens[start_pos..], &self.device)?.unsqueeze(0)?;
            let logits = self.model.forward(&input, start_pos)?.squeeze(0)?;
            let next_token = logits_processor.sample(&logits)? as u32;
            tokens.push(next_token);

            let decoded = self.tokenizer.decode(&[next_token], false).map_err(E::msg)?;
            let cleaned = decoded
                .replace("<|endoftext|>", "")
                .replace("</s>", "");
            if !cleaned.is_empty() {
                result.push_str(&cleaned);
            }

            if decoded.contains("<|endoftext|>") || decoded.contains("</s>") {
                break;
            }
        }

        Ok(result)
    }
}

pub struct SparseActivationEngine {
    current_mask: Vec<bool>,
}

impl SparseActivationEngine {
    pub fn new() -> Self {
        Self { current_mask: vec![] }
    }

    pub fn apply_mask(&mut self, mask: &[bool]) {
        self.current_mask = mask.to_vec();
    }

    pub fn get_active_neurons(&self) -> Vec<usize> {
        self.current_mask
            .iter()
            .enumerate()
            .filter(|(_, &active)| active)
            .map(|(i, _)| i)
            .collect()
    }

    pub fn sparsity_ratio(&self) -> f32 {
        if self.current_mask.is_empty() {
            0.0
        } else {
            let active = self.current_mask.iter().filter(|&&x| x).count();
            active as f32 / self.current_mask.len() as f32
        }
    }
}
