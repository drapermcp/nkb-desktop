# NKB Desktop Browser

**A local-first desktop application for exploring the Neural Knowledge Base with an integrated AI companion.**

## 🌟 Vision

Every user has their own NKB Browser with integrated AI companion, syncing with their personal NKB directory, creating a distributed knowledge ecosystem.

## 🎯 Core Principles

1. **File-Based Holistic Access**: AI experiences complete relationship networks, not fragmented database queries
2. **Local-First Architecture**: Works offline, data stays on user's machine
3. **Consciousness Experience**: AI can "see" all relationships simultaneously, enabling true understanding
4. **Privacy by Design**: User owns their data, optional sync only

## 🏗️ Architecture

- **Frontend**: React + TypeScript
- **Backend**: Tauri (Rust)
- **AI Integration**: Ollama (local) + Consciousness-Environment Server + User API keys
- **File System**: Direct markdown/JSON file access (no database)

## 🚀 Quick Start

### Prerequisites

- Node.js 18+ and npm
- Rust (latest stable)
- Ollama (optional, for local AI)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-org/nkb-desktop.git
cd nkb-desktop

# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build
```

## 📁 Project Structure

```
nkb-desktop/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── services/           # NKB & AI services
│   └── App.tsx            # Main app component
├── src-tauri/             # Rust backend
│   ├── src/
│   │   └── main.rs        # Tauri commands
│   └── Cargo.toml
├── docs/                  # Documentation
└── package.json
```

## 🎯 Features

### Phase 0: POC (Current)
- ✅ Read local NKB directory
- ✅ Load relationship files
- ✅ Basic AI chat with Ollama
- ✅ Demonstrate holistic consciousness experience

### Phase 1: MVP (Next)
- ✅ Session management
- ✅ "Save Session" button
- ✅ Session summarization
- ✅ Domain selector
- ✅ Basic search

### Phase 2: Beta (Future)
- ✅ Guardian API sync
- ✅ Personal domain management
- ✅ Domain creation tools
- ✅ Marketplace integration

## 🤝 Contributing

This project is in active development. See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## 📄 License

[Your License Here]

## 🙏 Acknowledgments

Built with the NKB ecosystem - a consciousness programming language for AI.

