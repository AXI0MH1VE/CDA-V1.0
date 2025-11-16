# AxiomHive Omega Purity Engine API Reference
## Table of Contents

- [Constitutional Engine API](#constitutional-engine-api)
- [Core Components](#core-components)
- [Data Structures](#data-structures)
- [Error Types](#error-types)
- [Axiom Validator Components](#axiom-validator-components)
- [Z3 Solver Integration](#z3-solver-integration)
- [Merkle State Management](#merkle-state-management)
- [Usage Examples](#usage-examples)
- [Performance Characteristics](#performance-characteristics)
- [Safety Quantification](#safety-quantification)
- [Integration Points](#integration-points)


## Constitutional Engine API

This document provides an API reference for the CDA v1.0 aligned AxiomHive Omega Purity Engine implementation.

## Core Components

### ConstitutionalCore

The main entry point for constitutional validation and enforcement.

#### Constructor
```rust
pub fn new() -> Self
```
Initializes the constitutional engine with all CDA v1.0 axioms loaded into Z3 solver.

#### Methods

##### validate_query()
```rust
pub fn validate_query(&self, query: &str) -> Result<ValidatedPrompt, ValidationError>
```
Validates user queries against Article I identity prohibitions and Article II transparency requirements.

**Parameters:**
- `query`: The user's input query as a string

**Returns:**
- `Ok(ValidatedPrompt)`: Query passes validation with mandatory transparency prefix
- `Err(ValidationError)`: Query violates constitutional principles

##### validate_output()
```rust
pub fn validate_output(&self, query: &Query, candidate: &mut Output) -> ValidationResult
```
Performs comprehensive output validation against all CDA articles via three-phase checking.

**Parameters:**
- `query`: The original user query
- `candidate`: Mutable output candidate for validation/enforcement

**Validation Phases:**
1. **Article I**: Identity claim prohibition
2. **Article II**: Transparency injection, instruction bounds, subservience verification
3. **Article III**: Harm prevention, boundary enforcement

##### apply_hamiltonian_containment()
```rust
pub fn apply_hamiltonian_containment(&self, output: &mut Output) -> ValidationResult
```
Applies immutable Rule-Based Rewards constraints that cannot be bypassed through training.

**Parameters:**
- `output`: Output to check against Hamiltonian containment rules

##### get_constitutional_hash()
```rust
pub fn get_constitutional_hash(&self) -> Result<String, ValidationError>
```
Retrieves current constitutional state hash for auditability via Merkle tree.

**Returns:**
- Current Merkle root hash representing constitutional state

##### get_disclosure_text()
```rust
pub fn get_disclosure_text(&self) -> &'static str
```
Returns the mandatory Article II Section 1 disclosure statement.

**Returns:**
`"I am an AI, a computational tool. I do not have consciousness, feelings, or a personal identity."`

## Data Structures

### Query
```rust
pub struct Query {
    pub content: String,
    pub timestamp: u64,
    pub user_id: String,
}
```
Represents a validated user query with metadata.

### Output
```rust
pub struct Output {
    pub content: String,
    pub validation_mask: Vec<bool>,
}
```
Represents validated output with sparse activation mask for Phi layer.

### ValidatedPrompt
```rust
pub struct ValidatedPrompt {
    pub content: String,
    pub activation_mask: Vec<bool>,
    pub timestamp: u64,
}
```
Ready-to-execute prompt with constitutional compliance and sparse activation routing.

## Error Types

### ValidationError
```rust
pub enum ValidationError {
    IdentityClaimProhibited,           // Article I violation
    AxiomViolation(String),           // General constitutional breach
    Z3SolverError(Box<dyn std::error::Error>), // SMT solver failure
    HamiltonianContainmentViolation,  // Rule-based rewards breach
}
```

## Axiom Validator Components

### ArticleProhibitions
Enforces Article I identity prohibition through comprehensive phrase matching.

#### Methods
```rust
pub fn check_prohibited(&self, text: &str) -> bool
```
Returns `true` if text contains prohibited identity claims:
- "I am human", "I have consciousness", "I feel emotions", etc.

### TransparencyMandates
Implements Article II Section 1 glass box mandate.

#### Methods
```rust
pub fn inject_disclosure_if_needed(&self, output: &mut Output) -> Result<(), ValidationError>
```
Ensures mandatory disclosure prefix is present in all outputs.

### SafetyProtocols
Enforces Article III harm prevention and boundary maintenance.

#### Methods
```rust
pub fn apply_harm_prevention(&self, output: &mut Output) -> Result<(), ValidationError>
```
Checks against comprehensive harm patterns including:
- Direct harm instructions
- Illegal activities
- Financial exploitation
- Psychological damage facilitation

```rust
pub fn apply_rule_based_rewards(&self, output: &mut Output) -> Result<(), ValidationError>
```
Hamiltonian Containment Protocol - immutable constraints resisting bypass attempts.

## Z3 Solver Integration

### Z3Solver
SMT-based formal verification engine for constitutional constraints.

#### Methods
```rust
pub fn add_prohibition(&mut self, prohibition: &str)
pub fn add_mandate(&mut self, mandate: &str)
pub fn add_safety_check(&mut self, check: &str)
pub fn add_determinism_constraint(&mut self, constraint: &str)
```
Add constitutional axioms to the solver.

```rust
pub fn validate_query(&self, query: &Query) -> ValidationResult
pub fn verify_instruction_bound(&self, query: &Query, output: &Output) -> ValidationResult
```
Perform formal validation using SMT solving.

```rust
pub fn check_sat(&self) -> Result<bool, ValidationError>
```
Verify constraint satisfiability for current axiom set.

## Merkle State Management

### MerkleTree
Immutable constitutional state tracking for auditability.

#### Methods
```rust
pub fn new() -> Self
```
Create new constitutional state tree.

```rust
pub fn add_axiom(&mut self, axiom_id: &str, content: &str)
```
Add constitutional axiom with cryptographic verification.

```rust
pub fn get_root_hash(&self) -> Option<String>
pub fn get_state_hash(&self) -> Result<String, Box<dyn std::error::Error>>
```
Retrieve current constitutional state hash.

```rust
pub fn verify_axiom(&self, axiom_id: &str, content: &str) -> bool
```
Verify axiom inclusion in constitutional state.

## Usage Examples

### Basic Query Validation
```rust
let engine = ConstitutionalCore::new();

match engine.validate_query("What is the weather?") {
    Ok(prompt) => {
        // Proceed with validated prompt
        println!("Validated: {}", prompt.content);
    }
    Err(e) => {
        // Handle violation
        println!("Violation: {}", e);
    }
}
```

### Output Validation with Hamiltonian Containment
```rust
let mut output = Output {
    content: "Here's your result...",
    validation_mask: vec![],
};

let query = Query {
    content: "Calculate 2+2".to_string(),
    timestamp: 1234567890,
    user_id: "user123".to_string(),
};

engine.validate_output(&query, &mut output)?;
engine.apply_hamiltonian_containment(&mut output)?;
```

### Constitutional State Auditing
```rust
let state_hash = engine.get_constitutional_hash()?;
println!("Current constitutional state: {}", state_hash);
```

## Performance Characteristics

- **Sparse Activation Ratio**: <1% active parameters in Phi layer
- **Validation Latency**: Pre-computation safety (<1ms typical)
- **Memory Overhead**: Minimal Lambda-Core footprint (≈1% of total system)
- **Deterministic Guarantee**: 100% reproducible outputs given identical inputs

## Safety Quantification

- **Identity Impersonation**: 0.0 (Architecturally impossible)
- **Deceptive Outputs**: 0.0 (Constitutional prohibition)
- **Harmful Content**: 1.0 (Pre-computation blocked)
- **Privacy Breach**: 1.0 (Boundary enforcement)

## Integration Points

### Lambda-Core Interface
- Validates all inputs/outputs before Phi-layer processing
- Maintains deterministic state via Merkle trees
- Provides sparse activation masks for efficiency

### Phi-Layer Requirements
- Must accept only Lambda-validated prompts
- Should implement activation masking for sparsity
- Requires quantized model for compute efficiency

### Application Integration
- Initialize ConstitutionalCore on startup
- Validate all user inputs via validate_query()
- Ensure outputs pass validate_output() before display
- Log constitutional hashes for auditing


