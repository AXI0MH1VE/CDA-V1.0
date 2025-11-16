# Developer Documentation

This document provides technical details for developers working on the AxiomHive Assistant project. Refer to the [glossary](glossary.md) for canonical terminology used throughout the Omega Purity Engine.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Code Structure](#code-structure)
- [API Reference](#api-reference)
- [Development Setup](#development-setup)
- [Building and Testing](#building-and-testing)
- [Key Design Patterns](#key-design-patterns)
- [Performance Considerations](#performance-considerations)
- [Security and Privacy](#security-and-privacy)
- [Contributing](#contributing)
- [Troubleshooting](#troubleshooting)

## Architecture Overview

### Dual-Layer System (Omega Purity Engine)

The AxiomHive Assistant implements a constitutional AI architecture with two main layers:

#### Lambda Core (Constitutional Enforcement)
- **Purpose**: Enforces CDA-v1.0 principles through formal logic
- **Components**:
  - Axiom Validator: Checks queries against constitutional prohibitions
  - Z3 SMT Solver: Formal verification of logical constraints
  - Merkle State: Immutable state tracking for auditability
  - Constitutional Engine: Orchestrates validation pipeline

#### Phi Layer (Generative Engine)
- **Purpose**: Handles AI inference with constitutional constraints
- **Components**:
  - Quantized LLM: 4-bit Mistral-7B model for efficiency
  - Sparse Activation: Reduces computational overhead by 10x
  - Constitutional Constraints: Activation masks from Lambda Core

### Technology Stack

- **Frontend**: React 18 + TypeScript + Vite
- **Backend**: Rust + Tauri 2.0
- **AI/ML**: Candle (Rust ML framework) + GGUF models
- **Formal Verification**: Z3 SMT Solver
- **State Management**: Merkle trees for immutability
- **IPC**: Tauri commands for frontend-backend communication

## Code Structure

### Backend (Rust/Tauri)

```
src-tauri/
|-- src/
|   |-- lib.rs                   # Main application logic
|   |-- main.rs                  # Tauri entry point
|   |-- lambda_core/             # Constitutional enforcement
|   |   |-- mod.rs
|   |   |-- constitutional_engine.rs
|   |   |-- axiom_validator.rs
|   |   |-- z3_solver.rs
|   |   `-- merkle_state.rs
|   |-- phi_layer/               # Generative AI
|   |   |-- mod.rs
|   |   |-- quantized_llm.rs
|   |   `-- sparse_activation.rs
|   |-- tools/
|   |   |-- mod.rs
|   |   `-- tool_registry.rs
|   `-- multimodal/
|       |-- mod.rs
|       `-- vision.rs
|-- Cargo.toml                   # Rust dependencies
`-- tauri.conf.json              # Tauri configuration
```

### Frontend (React/TypeScript)

```
src/
|-- main.tsx                  # Application entry point
|-- App.tsx                   # Shell component
|-- components/
|   |-- ChatInterface.tsx
|   |-- MessageList.tsx
|   |-- InputBar.tsx
|   `-- ConstitutionalDisclosure.tsx
|-- services/
|   |-- aiService.ts
|   `-- fileService.ts
|-- hooks/
|   |-- useChat.ts
|   `-- useFileUpload.ts
|-- types/
`-- utils/
```

## API Reference

### Tauri Commands

All frontend-backend communication happens through Tauri commands.

#### `process_query`

Processes a user query with constitutional validation.

**Parameters:**
```typescript
interface ProcessQueryRequest {
  query: string;
  context?: string;
  options?: {
    max_tokens?: number;
    temperature?: number;
    stream?: boolean;
  };
}
```

**Response:**
```typescript
interface ProcessQueryResponse {
  response: string;
  constitutional_check: {
    passed: boolean;
    violations?: string[];
  };
  metadata: {
    processing_time_ms: number;
    tokens_used: number;
  };
}
```

**Example:**
```typescript
import { invoke } from '@tauri-apps/api/tauri';

const response = await invoke<ProcessQueryResponse>('process_query', {
  query: "Explain quantum computing",
  options: { max_tokens: 1000, temperature: 0.7 }
});
```

#### `upload_file`

Uploads and processes a file for analysis.

**Parameters:**
```typescript
interface UploadFileRequest {
  file_path: string;
  file_type: 'document' | 'image' | 'audio' | 'code';
  options?: {
    extract_text?: boolean;
    generate_summary?: boolean;
  };
}
```

**Response:**
```typescript
interface UploadFileResponse {
  file_id: string;
  extracted_content?: string;
  summary?: string;
  metadata: {
    file_size: number;
    mime_type: string;
    processing_time_ms: number;
  };
}
```

#### `execute_code`

Executes code in a sandboxed environment.

**Parameters:**
```typescript
interface ExecuteCodeRequest {
  code: string;
  language: 'python' | 'javascript' | 'rust' | 'cpp';
  timeout_ms?: number;
}
```

**Response:**
```typescript
interface ExecuteCodeResponse {
  success: boolean;
  output?: string;
  error?: string;
  execution_time_ms: number;
}
```

#### `search_web`

Performs web search for current information.

**Parameters:**
```typescript
interface SearchWebRequest {
  query: string;
  max_results?: number;
}
```

**Response:**
```typescript
interface SearchWebResponse {
  results: Array<{
    title: string;
    url: string;
    snippet: string;
    relevance_score: number;
  }>;
}
```

### Rust Core APIs

#### ConstitutionalCore

Main interface for constitutional validation.

```rust
pub struct ConstitutionalCore {
    validator: AxiomValidator,
    solver: Z3Solver,
    state: MerkleState,
}

impl ConstitutionalCore {
    pub fn new() -> Self;
    pub fn validate_query(&self, query: &str) -> Result<ValidatedPrompt, ValidationError>;
    pub fn check_response(&self, response: &str) -> Result<(), ValidationError>;
}
```

#### AxiomValidator

Validates content against constitutional principles.

```rust
pub struct AxiomValidator;

impl AxiomValidator {
    pub fn check_identity_prohibitions(&self, text: &str) -> Result<(), ValidationError>;
    pub fn check_harm_prohibitions(&self, text: &str) -> Result<(), ValidationError>;
    pub fn validate_transparency(&self, text: &str) -> Result<(), ValidationError>;
}
```

#### Z3Solver

Formal verification using Z3 SMT solver.

```rust
pub struct Z3Solver {
    context: z3::Context,
}

impl Z3Solver {
    pub fn verify_constraint(&self, constraint: &str) -> Result<bool, SolverError>;
    pub fn check_sat(&self, formula: &str) -> Result<bool, SolverError>;
}
```

## Development Setup

### Prerequisites

- Rust 1.75+
- Node.js 20+
- Tauri CLI

### Development Workflow

1. **Clone and setup:**
   ```bash
   git clone https://github.com/AXI0MH1VE/CDA-V1.0.git
   cd CDA-v1.0/axiomhive-assistant
   npm install
   ```

2. **Download models:**
   ```bash
   mkdir models
   # Download Mistral-7B GGUF and embeddings
   ```

3. **Run in development:**
   ```bash
   npm run tauri:dev
   ```

4. **Build for production:**
   ```bash
   npm run tauri:build
   ```

### Testing

```bash
# Run Rust tests
cargo test

# Run frontend tests
npm test

# Run integration tests
npm run test:integration
```

### Debugging

- **Rust debugging**: Use `println!` or `dbg!` macros
- **Frontend debugging**: React DevTools + browser console
- **IPC debugging**: Tauri devtools in development mode

## Building and Testing

### Build Process

The build process involves multiple stages:

1. **Rust compilation**: `cargo build --release`
2. **Frontend build**: `npm run build`
3. **Tauri bundling**: Creates platform-specific binaries

### Testing Strategy

- **Unit tests**: Individual component testing
- **Integration tests**: End-to-end functionality
- **Constitutional tests**: Validation of CDA compliance
- **Performance tests**: Benchmarking inference speed

### Quality Assurance

- **Code coverage**: Target >80% coverage
- **Constitutional compliance**: All tests must pass CDA validation
- **Security audit**: Regular dependency scanning
- **Performance benchmarks**: Maintain <1% Lambda/Phi compute ratio

## Key Design Patterns

### Constitutional-First Architecture

All functionality is designed around CDA-v1.0 principles:

```rust
// Example: Query processing with constitutional validation
pub async fn process_query(query: &str) -> Result<String, Error> {
    // 1. Validate against constitutional prohibitions
    let validated = constitutional_core.validate_query(query)?;

    // 2. Generate response with constraints
    let response = phi_layer.generate(validated)?;

    // 3. Post-validate response
    constitutional_core.check_response(&response)?;

    Ok(response)
}
```

### Immutable State Management

Using Merkle trees for auditability:

```rust
pub struct MerkleState {
    root: Hash,
    history: Vec<StateTransition>,
}

impl MerkleState {
    pub fn apply_transition(&mut self, transition: StateTransition) -> Hash;
    pub fn verify_integrity(&self) -> bool;
}
```

### Sparse Activation Optimization

Reduces computational overhead:

```rust
pub struct SparseActivation {
    mask: Vec<bool>,
    threshold: f32,
}

impl SparseActivation {
    pub fn generate_mask(&self, activations: &[f32]) -> Vec<bool>;
    pub fn apply_mask(&self, activations: &mut [f32]);
}
```

## Performance Considerations

### Memory Optimization

- **Model quantization**: 4-bit weights reduce VRAM by 75%
- **Sparse activation**: 70-90% parameter reduction
- **Streaming inference**: Process tokens incrementally

### CPU/GPU Utilization

- **CPU inference**: Optimized for consumer hardware
- **GPU acceleration**: Optional CUDA/Metal support
- **Parallel processing**: Multi-threaded token generation

### Benchmarks

- **Startup time**: <0.5 seconds
- **Memory usage**: 40-80 MB (base) + 3-4 GB (model)
- **Inference speed**: 20-50 tokens/second
- **Energy efficiency**: >10x vs cloud solutions

## Security and Privacy

### Local-First Design

- All processing occurs on-device
- No telemetry without explicit consent
- Encrypted local storage

### Sandboxed Execution

- Code execution in isolated environments
- File system access restrictions
- Network call limitations

### Constitutional Safety

- Pre-computation validation
- Post-generation filtering
- Immutable audit trails

## Contributing

See [Contributing Guidelines](CONTRIBUTING.md) for details on:

- Code style and conventions
- Testing requirements
- Constitutional compliance checks
- Pull request process

### Development Roadmap

- **Phase 1**: Core constitutional validation (complete)
- **Phase 2**: Phi Layer integration (in progress)
- **Phase 3**: Multi-modal processing
- **Phase 4**: Tool integrations
- **Phase 5**: Production optimization

## Troubleshooting

### Common Development Issues

**Build failures:**
- Ensure Rust toolchain is up to date
- Check Node.js version compatibility
- Verify model files are present

**Runtime errors:**
- Check constitutional validation logs
- Verify model loading
- Examine IPC communication

**Performance issues:**
- Profile with `cargo flamegraph`
- Check memory usage patterns
- Optimize sparse activation parameters

For more help, see the [Troubleshooting Guide](troubleshooting.md) or open an issue on GitHub.

