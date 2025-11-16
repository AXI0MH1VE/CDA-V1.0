# Frequently Asked Questions (FAQ)

This FAQ addresses common questions about AxiomHive Assistant, CDA-v1.0, and constitutional AI.

## Table of Contents

- [General Questions](#general-questions)
- [Technical Questions](#technical-questions)
- [Constitutional Questions](#constitutional-questions)
- [Privacy and Security](#privacy-and-security)
- [Performance and Compatibility](#performance-and-compatibility)
- [Development and Contributing](#development-and-contributing)
- [Legal and Business](#legal-and-business)
- [Troubleshooting](#troubleshooting)
- [Getting Help](#getting-help)

## General Questions

### What is AxiomHive Assistant?

AxiomHive Assistant is a desktop AI application that implements the Constitution of a Deterministic Assistant (CDA-v1.0). It provides AI capabilities while maintaining strict constitutional boundaries to ensure it remains a transparent, deterministic tool without simulating consciousness or personality.

### How is it different from other AI assistants?

Unlike traditional AI assistants that may simulate personality or consciousness, AxiomHive Assistant:

- Operates under formal constitutional constraints
- Provides mandatory transparency about its nature
- Processes everything locally on your device
- Cannot claim consciousness, feelings, or identity
- Focuses on being a reliable computational tool

### What does "Constitutional AI" mean?

Constitutional AI refers to AI systems that operate under formal, codified rules (a "constitution") that define their boundaries and behavior. CDA-v1.0 establishes that the AI is purely a computational tool, not an entity with consciousness, personality, or autonomous agency.

### Is it free to use?

Yes, AxiomHive Assistant is free to download and use for personal, research, and internal evaluation under the [AxiomHive Community License](../LICENSE). Commercial deployments require a separate agreement, but there are no usage-based fees for non-commercial work. All processing happens locally on your device.

### Which platforms are supported?

AxiomHive Assistant supports:

- **Windows**: 10 and 11 (64-bit)
- **macOS**: 10.15 and later
- **Linux**: Ubuntu 18.04+, Fedora 30+, and other major distributions

## Technical Questions

### What are the system requirements?

**Minimum Requirements:**

- CPU: x64 processor (Intel/AMD)
- RAM: 8 GB
- Storage: 10 GB free space
- OS: Windows 10, macOS 10.15, or Ubuntu 18.04+

**Recommended:**

- CPU: Multi-core processor (i5/Ryzen 5 or better)
- RAM: 16 GB
- Storage: NVMe SSD
- OS: Latest versions

### How does it work technically?

AxiomHive Assistant uses a dual-layer architecture:

1. **Lambda Core**: Constitutional enforcement engine using formal logic and the Z3 SMT solver
2. **Phi Layer**: Generative AI with a quantized Mistral-7B model and sparse activation

All processing occurs locally with zero external data transmission.

### What AI model does it use?

The application uses a 4-bit quantized version of Mistral-7B-Instruct-v0.3, optimized for efficient local inference on consumer hardware.

### Can I use my own AI models?

Currently, the application is optimized for the included Mistral model. Future versions may support additional models through the extensible architecture.

### How fast is the inference?

Performance varies by hardware:

- Consumer CPUs: 20-35 tokens/second
- High-end CPUs: 35-50 tokens/second
- Future GPU support planned

### Does it require an internet connection?

No, AxiomHive Assistant works completely offline. The only optional internet use is for:

- Downloading the application
- Checking for updates (can be disabled)
- Web search features (requires explicit user consent)

## Constitutional Questions

### What is CDA-v1.0?

CDA-v1.0 (Constitution of a Deterministic Assistant) is a formal framework that defines the operational boundaries of AI systems. It ensures AI remains a transparent computational tool without simulating consciousness, personality, or selfhood.

### Why constitutional constraints?

Constitutional constraints provide:

- **Safety**: Prevent harmful or deceptive behavior
- **Transparency**: Clear understanding of AI limitations
- **Reliability**: Deterministic, predictable operation
- **Ethics**: Formal boundaries for responsible AI use

### What can't the AI do?

The AI cannot:

- Claim consciousness, sentience, or self-awareness
- Express feelings, emotions, or personal opinions
- Simulate personality or identity
- Take autonomous actions without user input
- Generate content that violates ethical boundaries

### How does constitutional enforcement work?

Constitutional rules are encoded as formal constraints validated by:

- **Pre-computation checks**: Input validation before processing
- **Z3 SMT solver**: Formal verification of logical constraints
- **Merkle state tracking**: Immutable audit trails
- **Multi-layer validation**: Defense in depth approach

### What happens if constitutional rules are violated?

The system:

1. Detects the violation attempt
2. Blocks the operation
3. Provides clear explanation
4. Logs the incident for transparency
5. Maintains safe operation

## Privacy and Security

### Is my data private?

Yes, AxiomHive Assistant is designed for maximum privacy:

- **Local Processing**: All AI inference happens on your device
- **Zero Telemetry**: No usage data collected by default
- **No External APIs**: No data sent to external services
- **Encrypted Storage**: Local data stored encrypted
- **User Control**: Complete control over data retention

### Does it collect any data?

By default, no data is collected or transmitted. Optional features (like web search) require explicit user consent and can be disabled.

### How secure is the application?

Security features include:

- **Local-only processing**: No external data exposure
- **Constitutional safeguards**: Formal harm prevention
- **Sandboxed execution**: Isolated code execution
- **Encrypted storage**: Protected local data
- **Regular audits**: Ongoing security validation

### Can I trust the AI not to be harmful?

The constitutional framework provides multiple layers of protection:

- **Formal constraints**: Mathematically verified safety rules
- **Harm prevention**: Built-in content filtering
- **Transparency**: Clear disclosure of limitations
- **User authority**: Human maintains final control

### What about updates and security patches?

Updates are:

- **Cryptographically signed** for integrity
- **Optional**: Users control update installation
- **Transparent**: Clear changelog for all changes
- **Secure**: Delivered through trusted channels

## Performance and Compatibility

### Why is it slow on my computer?

Common performance issues:

- **Insufficient RAM**: Need at least 8GB, 16GB recommended
- **CPU bottleneck**: Older or low-core CPUs struggle
- **Storage speed**: HDD vs SSD makes big difference
- **Background processes**: Other applications consuming resources

### Can it run on older hardware?

Minimum requirements are enforced, but performance will be limited on older hardware. The application includes performance monitoring to help users understand bottlenecks.

### Does it support GPU acceleration?

GPU support is planned for future versions. Currently optimized for CPU inference with techniques like sparse activation and quantization for efficiency.

### How much storage space does it need?

- **Installation**: ~500 MB
- **AI Models**: ~4 GB (downloaded separately)
- **Working space**: ~1 GB temporary files
- **Total**: ~5.5 GB

### Can I run it on a virtual machine?

Yes, but performance may be reduced due to virtualization overhead. Ensure the VM has adequate resources allocated.

## Development and Contributing

### Is the code open source?

The code is source-available under the [AxiomHive Community License](../LICENSE). You can inspect, fork, and modify it for personal, research, or internal evaluation, but commercial use requires a separate license.

### How can I contribute?

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines. Areas for contribution:

- Code improvements
- Documentation
- Testing
- Bug reports
- Feature suggestions

### What programming languages are used?

- **Backend**: Rust (performance-critical AI and constitutional logic)
- **Frontend**: TypeScript/React (user interface)
- **Build System**: Tauri (cross-platform desktop app framework)

### How do I build it from source?

See the [Installation Guide](installation.md) and [Developer Documentation](developer.md) for detailed build instructions.

### Are there API endpoints I can use?

The application provides Tauri commands for integration. See the [Developer Documentation](developer.md) for API reference.

## Legal and Business

### Who owns AxiomHive Assistant?

AxiomHive Assistant is developed by AxiomHive, founded by Alexis Adams. The constitutional AI technology is proprietary intellectual property.

### Is it legal to use?

Yes, AxiomHive Assistant is legal to download, install, and use in all jurisdictions. It operates locally and doesn't violate any known regulations.

### Can I use it commercially?

Commercial deployments require a paid license from AxiomHive. Contact [licensing@axiomhive.co](mailto:licensing@axiomhive.co) with your use case to receive a commercial agreement.

### What about the AI models?

The application uses open-source models (Mistral-7B) with permissive licenses. All model usage complies with their respective licenses.

### Does it comply with data protection laws?

Yes, AxiomHive Assistant is designed to comply with:

- **GDPR** (European privacy regulation)
- **CCPA** (California privacy law)
- **Local data processing** ensures compliance
- **Zero data collection** by default

### What if I have legal concerns?

Contact our legal team at [legal@axiomhive.co](mailto:legal@axiomhive.co) for specific legal questions or concerns.

## Troubleshooting

### The app won't start

Common solutions:

1. Check system requirements
2. Verify model files are downloaded
3. Run as administrator (Windows)
4. Check antivirus exclusions
5. Try development mode: `npm run tauri:dev`

### Responses are slow or poor quality

Try:

1. Close other applications
2. Check RAM usage
3. Restart the application
4. Verify model integrity
5. Check constitutional settings

### File upload doesn't work

Ensure:

1. File is under size limit (100MB)
2. Supported file type
3. File is not corrupted
4. Sufficient disk space

### Constitutional errors

If you see constitutional validation errors:

1. Rephrase your query
2. Avoid identity-related terms
3. Check the transparency disclosure
4. Report false positives

## Getting Help

### Where can I find more information?

- **Documentation**: docs/ folder in the repository
- **User Manual**: [user-manual.md](user-manual.md)
- **Developer Docs**: [developer.md](developer.md)
- **Troubleshooting**: [troubleshooting.md](troubleshooting.md)

### How do I report bugs?

Use GitHub Issues with:

- Clear description
- Steps to reproduce
- System information
- Error logs

### Community support

- **GitHub Discussions**: General questions
- **Discord**: Real-time community chat
- **Email**: [support@axiomhive.co](mailto:support@axiomhive.co)

### Professional support

For enterprise or commercial support:

- **Email**: [enterprise@axiomhive.co](mailto:enterprise@axiomhive.co)
- **Website**: [axiomhive.co/support](https://axiomhive.co/support)

---

**Still have questions?** Check the [User Manual](user-manual.md) or create a discussion on GitHub. We're here to help!

