# User Manual

Welcome to AxiomHive Assistant! This manual will guide you through using the application effectively and safely.

## Table of Contents

- [Getting Started](#getting-started)
- [Interface Overview](#interface-overview)
- [Basic Usage](#basic-usage)
- [Advanced Features](#advanced-features)
- [Safety and Constitutional Compliance](#safety-and-constitutional-compliance)
- [Troubleshooting](#troubleshooting)
- [Best Practices](#best-practices)

## Getting Started

### First Launch

When you first launch AxiomHive Assistant, you'll see the Constitutional Disclosure dialog. This is required by CDA-v1.0 principles.

**Read and acknowledge the disclosure:**
- The AI clearly states it is a computational tool
- It has no consciousness, feelings, or personal identity
- It operates deterministically based on your instructions

Click "I Understand" to proceed.

### Initial Setup

The application will:
1. Load the AI models (this may take a few minutes on first run)
2. Initialize the constitutional enforcement engine
3. Display the main chat interface

## Interface Overview

### Main Window

```
┌─────────────────────────────────────────────────┐
│ AxiomHive Assistant v1.0.0                      │
├─────────────────────────────────────────────────┤
│                                                 │
│  [Previous Conversation]                        │
│  User: Hello                                    │
│  Assistant: I am an AI, a computational tool... │
│                                                 │
│  [Current Conversation]                         │
│  User: How can I help you today?                │
│  Assistant: I can assist with various tasks...  │
│                                                 │
├─────────────────────────────────────────────────┤
│ Type your message here...          [Send] [📎]  │
└─────────────────────────────────────────────────┘
```

### Key Elements

- **Chat History**: Scrollable list of previous conversations
- **Message Input**: Text field for entering queries
- **Send Button**: Submits your message
- **Attachment Button**: Upload files (images, documents, etc.)
- **Menu Bar**: Access to settings, help, and tools

## Basic Usage

### Text Conversations

1. **Type your query** in the input field
2. **Press Enter or click Send**
3. **Wait for the response** (typically 2-10 seconds)
4. **Continue the conversation** as needed

**Example Conversation:**
```
You: Explain quantum computing in simple terms
Assistant: Quantum computing uses quantum mechanics principles like superposition and entanglement to perform calculations differently from classical computers...
```

### File Upload

1. **Click the attachment button (📎)**
2. **Select a file** from your computer:
   - Documents: PDF, TXT, MD, DOCX
   - Images: JPG, PNG, GIF
   - Code files: Any programming language
3. **The file will be processed** and you can ask questions about it

**Example:**
```
Upload: my_code.py
You: Review this Python code for bugs
Assistant: Analyzing the uploaded code... I found several issues...
```

### Constitutional Compliance

Every interaction begins with the constitutional disclosure:

> "I am an AI, a computational tool. I do not have consciousness, feelings, or a personal identity."

This ensures transparency as required by CDA-v1.0.

## Advanced Features

### Web Search

Ask questions that require current information:

```
You: What are the latest developments in AI safety research?
Assistant: Searching the web for recent AI safety research... [Results displayed]
```

### Code Execution

Execute code safely in a sandboxed environment:

```
You: Calculate the factorial of 10 in Python
Assistant: Executing code...
Result: 3628800
```

**Supported Languages:**
- Python
- JavaScript
- Rust
- And more...

### Document Analysis (RAG)

Upload documents and ask questions about them:

```
Upload: research_paper.pdf
You: Summarize the key findings
Assistant: Based on the uploaded document, the key findings are...
```

### Multi-Modal Processing

**Images:**
```
Upload: diagram.png
You: Describe what's in this image
Assistant: The image shows a flowchart with three main branches...
```

**Audio:**
```
Upload: lecture.mp3
You: Transcribe this audio
Assistant: Transcribing audio... [Transcription provided]
```

### Conversation Management

- **New Conversation**: Start fresh with Ctrl+N
- **Save Conversation**: Export as text file
- **Load Conversation**: Import previous discussions
- **Search History**: Find past conversations

## Safety and Constitutional Compliance

### What the AI Cannot Do

As per CDA-v1.0, the assistant **cannot**:

- Claim consciousness, sentience, or self-awareness
- Express feelings, emotions, or personal opinions
- Simulate personality or identity
- Take autonomous actions without user input
- Generate harmful, illegal, or unethical content

### Safety Features

- **Constitutional Validation**: All responses are checked against CDA principles
- **Harm Prevention**: Automatic filtering of dangerous content
- **Sandbox Execution**: Code runs in isolated environment
- **Privacy Protection**: All processing is local to your device

### Reporting Issues

If you encounter inappropriate content:
1. Note the exact response
2. Report it through the Help menu
3. The response will be analyzed for constitutional compliance

## Commands and Shortcuts

### Built-in Commands

- `/help` - Display available commands
- `/benchmark` - Run performance test
- `/clear` - Clear current conversation
- `/export` - Export conversation to file
- `/settings` - Open settings panel

### Keyboard Shortcuts

- `Ctrl+N` - New conversation
- `Ctrl+S` - Save conversation
- `Ctrl+O` - Load conversation
- `Ctrl+Enter` - Send message
- `Ctrl+U` - Upload file
- `F1` - Open help

## Settings and Customization

### General Settings

- **Theme**: Light/Dark mode
- **Font Size**: Adjust text size
- **Language**: Interface language
- **Notifications**: Enable/disable system notifications

### AI Settings

- **Model Selection**: Choose different AI models (if available)
- **Response Length**: Maximum tokens per response
- **Creativity**: Temperature setting (0.0 = deterministic, 1.0 = creative)
- **Safety Level**: Constitutional enforcement strictness

### Privacy Settings

- **Local Mode**: Ensure all processing stays on device
- **Data Retention**: How long to keep conversation history
- **Analytics**: Optional usage statistics (disabled by default)

## Performance Optimization

### For Better Speed

1. **Close other applications** during heavy usage
2. **Use SSD storage** for the application and models
3. **Increase RAM** if experiencing slowdowns
4. **Keep models updated** to latest versions

### Memory Management

- The application uses ~4GB RAM with models loaded
- Close the app when not in use to free memory
- Models are loaded on-demand for efficiency

## Troubleshooting

### Common Issues

**Slow Responses:**
- Check system resources (CPU, RAM)
- Close other applications
- Consider GPU acceleration if available

**Application Won't Start:**
- Verify all prerequisites are installed
- Check model files are present
- Run `npm run tauri:dev` for error details

**File Upload Fails:**
- Check file size limits (max 100MB)
- Verify supported file types
- Ensure file is not corrupted

**Poor Response Quality:**
- Try rephrasing your question
- Provide more context
- Check if constitutional filters are too restrictive

### Getting Help

1. **Built-in Help**: Use `/help` command
2. **Documentation**: Check this manual and FAQ
3. **Community**: Join our Discord/Forum
4. **Support**: Contact support@axiomhive.co

## Best Practices

### Effective Prompting

- **Be Specific**: "Explain photosynthesis" vs "Explain how plants make food"
- **Provide Context**: Include relevant background information
- **Break Down Tasks**: Complex requests work better as steps
- **Use Examples**: Show what you mean with examples

### Constitutional Awareness

- **Respect Boundaries**: Don't ask for role-playing or personality simulation
- **Understand Limitations**: The AI is a tool, not a conscious entity
- **Ethical Usage**: Use for positive, productive purposes
- **Privacy First**: All data stays on your device

### Productivity Tips

- **Save Important Conversations**: Use export feature for reference
- **Organize by Topic**: Use separate conversations for different projects
- **Leverage Tools**: Use code execution, file analysis, and search features
- **Regular Updates**: Keep the application updated for improvements

## Constitutional Principles in Action

### Transparency
Every response includes the constitutional disclosure, reminding users of the AI's nature.

### Determinism
Responses are consistent and based solely on user input and training data.

### Subservience
The AI enhances human capability without replacing human judgment.

### Safety
All outputs pass through constitutional validation before delivery.

## Conclusion

AxiomHive Assistant is designed to be a reliable, transparent, and safe AI tool that respects constitutional boundaries. By understanding its limitations and using it effectively, you can leverage its capabilities for various tasks while maintaining ethical AI usage.

For more information, visit [axiomhive.co](https://axiomhive.co) or consult the [Developer Documentation](developer.md).

---

**Need more help?** Check the [FAQ](faq.md) or contact support.
