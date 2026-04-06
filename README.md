# Squan Desktop (Tauri)

**Lightweight, native desktop app for Squan - just like Goose!**

## Why Tauri?

| Feature | Electron (Old) | Tauri (New) |
|---------|---------------|-------------|
| Size | 76MB | ~5-10MB (7-15x smaller!) |
| Memory | ~150MB | ~50MB (3x less!) |
| Install | Required | Optional (portable) |
| Performance | Slower | Much faster |
| Technology | Chromium bundle | System webview |
| Backend | Node.js | Rust |

**Result:** Smaller, faster, more native feel - just like Goose!

---

## Features

### ✅ Actually Usable!
- **Quick connect** to existing Squan server
- **Test connection** before opening
- **Docker setup** instructions built-in
- **Check Docker** installation with one click
- **Open docs** directly from app

### ✅ Native Experience
- System tray integration
- Auto-updates
- Native notifications
- Fast startup
- Low memory usage

---

## Building

### Prerequisites

1. **Rust** (required for Tauri)
   ```bash
   # Windows
   https://www.rust-lang.org/tools/install
   
   # Mac/Linux
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js 20+** (for package management)
   ```bash
   node -v  # Should be 20+
   ```

3. **Build tools**
   
   **Windows:**
   ```bash
   # Install Visual Studio Build Tools
   https://visualstudio.microsoft.com/visual-cpp-build-tools/
   ```
   
   **Mac:**
   ```bash
   xcode-select --install
   ```
   
   **Linux:**
   ```bash
   sudo apt install libwebkit2gtk-4.0-dev \
     build-essential \
     curl \
     wget \
     file \
     libssl-dev \
     libgtk-3-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev
   ```

### Build Commands

```bash
# Install dependencies
npm install

# Development mode (with hot reload)
npm run dev

# Build for your platform
npm run build

# Build Windows
npm run build:win

# Build macOS
npm run build:mac

# Build Linux
npm run build:linux
```

### Output

After building, you'll find:

**Windows:**
- `src-tauri/target/release/squan.exe` (~5-8MB portable!)
- `src-tauri/target/release/bundle/msi/Squan_2.0.0_x64_en-US.msi` (installer)
- `src-tauri/target/release/bundle/nsis/Squan_2.0.0_x64-setup.exe` (NSIS installer)

**macOS:**
- `src-tauri/target/release/bundle/macos/Squan.app` (~8-10MB!)
- `src-tauri/target/release/bundle/dmg/Squan_2.0.0_x64.dmg` (installer)

**Linux:**
- `src-tauri/target/release/squan` (~5-8MB portable!)
- `src-tauri/target/release/bundle/deb/squan_2.0.0_amd64.deb`
- `src-tauri/target/release/bundle/appimage/squan_2.0.0_amd64.AppImage`

---

## First-Time User Experience

When users launch the app, they see:

### Tab 1: Connect to Server
- Input field for server URL (default: http://localhost:3000)
- "Connect" button → Opens server in browser
- "Test Connection" button → Verifies server is running
- Quick actions grid (Dashboard, Agents, Workflows, Cost Tracking)

### Tab 2: Start with Docker
- Complete docker-compose commands (copy-paste ready)
- "Check Docker" button → Verifies Docker is installed
- "View Full Guide" button → Opens documentation
- API key configuration instructions

### Tab 3: Learn More
- What is Squan explanation
- Feature highlights (Multi-Agent, Cost Savings, Speed, Tools)
- Resource links (Docs, GitHub, Website)

### Smart Defaults
- Auto-tests localhost:3000 on startup
- Shows status messages (connected/not connected)
- Provides clear next steps

---

## Development

```bash
# Run in dev mode
npm run dev

# The app will open and hot-reload on changes
```

## Comparison with Electron Version

### Electron (Old)
```
📦 Squan-Setup-2.0.0.exe
├── Size: 76MB
├── Install: Required
├── Memory: ~150MB
├── Startup: 3-4 seconds
└── Bundle: Includes entire Chromium browser
```

### Tauri (New)
```
📦 squan.exe
├── Size: ~5-8MB (15x smaller!)
├── Install: Optional (portable .exe)
├── Memory: ~50MB (3x less!)
├── Startup: <1 second
└── Uses: System webview (Edge on Windows)
```

### File Size Examples

**Electron:**
- Installer: 76MB
- Portable: 76MB
- Total: 152MB

**Tauri:**
- Portable .exe: ~5MB
- NSIS installer: ~6MB
- MSI installer: ~7MB
- Total: ~18MB for all formats!

---

## Troubleshooting

### "cargo not found"
Install Rust: https://www.rust-lang.org/tools/install

### "webkit2gtk not found" (Linux)
```bash
sudo apt install libwebkit2gtk-4.0-dev
```

### Build fails on Windows
Install Visual Studio Build Tools:
https://visualstudio.microsoft.com/visual-cpp-build-tools/

### "Tauri CLI not found"
```bash
cargo install tauri-cli
```

---

## What Users Can Do

### Immediate Actions
1. **Connect to existing server** (if they have one)
2. **Test connection** before committing
3. **Get Docker commands** (copy-paste ready)
4. **Check Docker installation** (one click)
5. **Open documentation** (built-in links)

### No More "What Now?"
- Clear tabs: Connect / Docker / Learn
- Step-by-step instructions
- Working buttons (not just decoration)
- Auto-test connection on startup
- Helpful status messages

---

## Next Steps After Building

1. **Test the portable .exe**
   ```bash
   # Just run it!
   ./src-tauri/target/release/squan.exe
   ```

2. **Upload to GitHub releases**
   ```bash
   gh release upload v2.0.0 \
     src-tauri/target/release/bundle/nsis/Squan_2.0.0_x64-setup.exe \
     src-tauri/target/release/squan.exe
   ```

3. **Update website download links**
   - Much smaller files to download!
   - Faster downloads for users
   - Professional like Goose

---

## Why This is Better

### For Users
- ✅ Smaller download (5MB vs 76MB)
- ✅ No installer needed (just run .exe)
- ✅ Faster startup (<1s vs 3-4s)
- ✅ Less memory (50MB vs 150MB)
- ✅ Actually usable (clear actions)
- ✅ Works offline (after first setup)

### For You
- ✅ Smaller releases (save GitHub bandwidth)
- ✅ Faster to download/upload
- ✅ More professional (like Goose)
- ✅ Easier to maintain (Rust is reliable)
- ✅ Better performance (native)

---

## License

MIT

## Links

- Website: https://squan.dev
- GitHub: https://github.com/tarvitave/squan
- Documentation: https://squan.dev/docs
