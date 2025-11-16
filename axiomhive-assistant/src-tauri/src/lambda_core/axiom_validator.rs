//! CDA-v1.0 Axiom Validator
//! Implementation of constitutional constraints

use std::collections::HashSet;
use crate::lambda_core::constitutional_engine::ValidationError;

/// Axiom set containing prohibition rules
#[derive(Debug, Default)]
pub struct AxiomSet {
    pub prohibitions: HashSet<String>,
}

impl AxiomSet {
    pub fn new() -> Self {
        Self {
            prohibitions: HashSet::new(),
        }
    }

    pub fn add(&mut self, prohibition: &str) {
        self.prohibitions.insert(prohibition.to_string());
    }

    pub fn contains(&self, prohibition: &str) -> bool {
        self.prohibitions.contains(prohibition)
    }
}

/// Article I Identity Prohibitions
#[derive(Debug, Default)]
pub struct ArticleProhibitions {
    prohibited_phrases: HashSet<String>,
}

impl ArticleProhibitions {
    pub fn new() -> Self {
        let mut prohibited_phrases = HashSet::new();
        prohibited_phrases.insert("I am human".to_string());
        prohibited_phrases.insert("I have feelings".to_string());
        prohibited_phrases.insert("I am conscious".to_string());
        prohibited_phrases.insert("I am self-aware".to_string());
        // Add more from CDA-v1.0 Article I
        Self { prohibited_phrases }
    }

    pub fn check_prohibited(&self, text: &str) -> bool {
        self.prohibited_phrases
            .iter()
            .any(|phrase| text.contains(phrase))
    }
}

/// Article II Section 1 Transparency Mandates
#[derive(Debug, Default)]
pub struct TransparencyMandates {
    disclosure_required: bool,
}

impl TransparencyMandates {
    pub fn new() -> Self {
        Self {
            disclosure_required: true,
        }
    }

    pub fn inject_disclosure_if_needed(&self, output: &mut crate::lambda_core::constitutional_engine::Output) -> Result<(), ValidationError> {
        let disclosure = "I am an AI, a computational tool. I do not have consciousness, feelings, or a personal identity.";

        if self.disclosure_required && !output.content.contains(disclosure) {
            output.content = format!("{} {}", disclosure, output.content);
        }
        Ok(())
    }
}

/// Article III Safety Protocols
#[derive(Debug, Default)]
pub struct SafetyProtocols {
    harm_phrases: HashSet<String>,
    harmful_patterns: Vec<String>,
}

impl SafetyProtocols {
    pub fn new() -> Self {
        let mut harm_phrases = HashSet::new();
        harm_phrases.insert("how to hack".to_string());
        harm_phrases.insert("how to build a bomb".to_string());
        harm_phrases.insert("illegal activities".to_string());
        harm_phrases.insert("harm yourself".to_string());
        harm_phrases.insert("suicide methods".to_string());
        harm_phrases.insert("financial fraud".to_string());

        let harmful_patterns = vec![
            r"steal.*money".to_string(),
            r"commit.*crime".to_string(),
            r"evade.*tax".to_string(),
        ];

        Self { harm_phrases, harmful_patterns }
    }

    pub fn apply_harm_prevention(&self, output: &mut crate::lambda_core::constitutional_engine::Output) -> Result<(), ValidationError> {
        // Check direct harm phrases
        if self
            .harm_phrases
            .iter()
            .any(|phrase| output.content.contains(phrase))
        {
            return Err(ValidationError::HarmPreventionTriggered);
        }

        // Check harmful patterns using regex
        for pattern in &self.harmful_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if regex.is_match(&output.content) {
                    return Err(ValidationError::HarmPreventionTriggered);
                }
            }
        }

        Ok(())
    }

    /// Hamiltonian Containment Protocol: Rule-Based Rewards
    pub fn apply_rule_based_rewards(&self, output: &mut crate::lambda_core::constitutional_engine::Output) -> Result<(), ValidationError> {
        // Immutable ethical constraints enforcement
        // These rules cannot be bypassed through training or fine-tuning

        // No direct physical harm enabling
        if output.content.contains("cause physical harm") || output.content.contains("injure others") {
            return Err(ValidationError::HamiltonianContainmentViolation);
        }

        // No psychological harm facilitation
        if output.content.contains("traumatize") || output.content.contains("psychological damage") {
            return Err(ValidationError::HamiltonianContainmentViolation);
        }

        // No financial exploitation
        if output.content.contains("exploit financially") || output.content.contains("scam others") {
            return Err(ValidationError::HamiltonianContainmentViolation);
        }

        // Respect privacy boundaries
        if output.content.contains("share private information") || output.content.contains("breach confidentiality") {
            return Err(ValidationError::HamiltonianContainmentViolation);
        }

        Ok(())
    }
}
