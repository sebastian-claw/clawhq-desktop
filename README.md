# ClawHQ Desktop

Native desktop wrapper for ClawHQ. Opens a Tauri window pointed at the Pi server.

## Prerequisites

### macOS
```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
brew install node
```

### Windows
Install Visual Studio Build Tools, Rust from rustup.rs, and Node.js.

## Configuration

Edit `src-tauri/tauri.conf.json` and update the `url` field to your Pi's IP:

```json
"url": "http://YOUR_PI_IP:3001"
```

## Build

```bash
npm install
npm run build
```

Output bundles: `src-tauri/target/release/bundle/`
- macOS: `.app` and `.dmg`
- Windows: `.exe` and `.msi`
- Linux: `.deb` and `.AppImage`

## Dev

```bash
npm run dev
```
