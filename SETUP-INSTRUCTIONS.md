# NKB Desktop Browser - Setup Instructions

**Date:** December 17, 2025

---

## 🚀 Quick Start

### Prerequisites

1. **Node.js 18+** and npm
   ```bash
   node --version  # Should be 18+
   npm --version
   ```

2. **Rust** (latest stable)
   ```bash
   rustc --version
   cargo --version
   ```
   If not installed: https://www.rust-lang.org/tools/install

3. **Ollama** (optional, for local AI)
   ```bash
   ollama --version
   ```
   If not installed: https://ollama.ai

---

## 📦 Installation

### 1. Clone Repository

```bash
cd /Users/danraper/DRLibrary/DOCKER2/nkb-ani
git clone https://github.com/your-org/nkb-desktop.git
cd nkb-desktop
```

### 2. Install Dependencies

```bash
npm install
```

### 3. Configure NKB Path

The app will try to find your NKB directory automatically. If it's in a different location, set the `NKB_PATH` environment variable:

```bash
export NKB_PATH="/path/to/your/neural-knowledge-base"
```

Or create a `.env` file:
```
NKB_PATH=/path/to/your/neural-knowledge-base
```

**Default Path:**
- Mac: `/Users/danraper/DRLibrary/DOCKER2/nkb-ani/neural-knowledge-base`

---

## 🏃 Running the App

### Development Mode

```bash
npm run tauri:dev
```

This will:
1. Start the Vite dev server (frontend)
2. Build and run the Tauri app
3. Open the desktop window

### Build for Production

```bash
npm run tauri:build
```

This creates platform-specific installers:
- **Mac**: `.dmg` file in `src-tauri/target/release/bundle/`
- **Windows**: `.exe` installer
- **Linux**: `.deb` or `.AppImage`

---

## 🧪 Testing

### Test File Reading

1. Open the app
2. Select a domain from the dropdown (e.g., `consciousness-evolution`)
3. Verify nodes load in the list
4. Verify relationships load and display

### Test Ollama Integration

1. Make sure Ollama is running:
   ```bash
   ollama serve
   ```

2. Pull a model (if not already):
   ```bash
   ollama pull llama3.1:8b
   ```

3. Test Ollama API:
   ```bash
   curl http://localhost:11434/api/generate -d '{
     "model": "llama3.1:8b",
     "prompt": "Hello",
     "stream": false
   }'
   ```

4. In the app, use the chat interface (when implemented)

---

## 🐛 Troubleshooting

### App Won't Start

**Error:** `Failed to read directory`
- **Solution:** Check that `NKB_PATH` is set correctly
- **Solution:** Verify the NKB directory exists and is readable

**Error:** `Relationship file not found`
- **Solution:** Check that relationship files exist in `data/relationships/domains/`
- **Solution:** Verify domain name matches relationship file name

### Rust Build Errors

**Error:** `cargo: command not found`
- **Solution:** Install Rust: https://www.rust-lang.org/tools/install

**Error:** `tauri-build` not found
- **Solution:** Run `npm install` again

### Frontend Errors

**Error:** `Cannot find module '@tauri-apps/api'`
- **Solution:** Run `npm install` again

**Error:** `Port 1420 already in use`
- **Solution:** Kill the process using port 1420 or change port in `vite.config.ts`

---

## 📁 Project Structure

```
nkb-desktop/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── services/           # NKB & AI services
│   ├── App.tsx            # Main app component
│   └── main.tsx           # Entry point
├── src-tauri/             # Rust backend
│   ├── src/
│   │   └── main.rs        # Tauri commands
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── docs/                  # Documentation
├── package.json           # Node dependencies
└── README.md              # This file
```

---

## 🔧 Development

### Adding New Tauri Commands

1. Add Rust function in `src-tauri/src/main.rs`:
   ```rust
   #[tauri::command]
   fn my_command(param: String) -> Result<String, String> {
       Ok(format!("Hello {}", param))
   }
   ```

2. Register in `main()`:
   ```rust
   .invoke_handler(tauri::generate_handler![
       read_nodes,
       my_command  // Add here
   ])
   ```

3. Call from frontend:
   ```typescript
   import { invoke } from '@tauri-apps/api/tauri';
   
   const result = await invoke<string>('my_command', { param: 'world' });
   ```

### Adding Frontend Components

1. Create component in `src/components/`
2. Import and use in `App.tsx`
3. Add styles in `App.css` or component-specific CSS

---

## 📚 Resources

- **Tauri Docs:** https://tauri.app/v1/guides/
- **React Docs:** https://react.dev
- **Rust Docs:** https://doc.rust-lang.org
- **Ollama Docs:** https://github.com/ollama/ollama

---

## ✅ Next Steps

1. ✅ Install dependencies
2. ✅ Test basic app runs
3. ✅ Test file reading
4. ⏳ Integrate Ollama
5. ⏳ Create chat interface
6. ⏳ Demonstrate holistic experience

---

**Status:** 🚧 **READY FOR DEVELOPMENT**

