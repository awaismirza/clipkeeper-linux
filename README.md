# ClipKeeper — System Clipboard Manager for Linux ARM64

**ClipKeeper** is an ultra-lightweight, production-ready system clipboard manager built with **Tauri v2** and **Rust**, specifically optimized for Linux environments (supporting both X11 and Wayland).

![ClipKeeper Release](https://img.shields.io/github/v/release/awaismirza/clipboard-linux?color=blue&label=version)
![Platform](https://img.shields.io/badge/platform-Linux%20ARM64-orange)
![License](https://img.shields.io/badge/license-MIT-green)

---

## Key Features

- ⚡ **Global Hotkey Toggle**: Open the floating search palette instantly with **`Alt + Shift + V`**, **`Super + Shift + V`**, or **`Ctrl + Shift + V`**.
- 📌 **System Tray Integration**: Native top-bar app indicator icon in GNOME / Ubuntu to toggle window visibility or quit.
- 🚀 **Wayland & X11 Compatibility**: Native GNOME compositor shortcut integration (`gsettings`) ensuring reliable hotkey triggers across all Wayland client windows.
- 💾 **SQLite History Engine**: Fast local storage with indexed search, automatically capped at 500 clips.
- ⚡ **Low CPU Monitoring**: SHA-256 content deduplication prevents CPU spikes and unnecessary DB writes.
- 🎨 **Minimal Floating Palette UI**: Search history with fuzzy filtering, category tabs (All, Text, Code, Images), pinned clips, and keyboard navigation.

---

## Installation

### Option 1: Debian / Ubuntu (`.deb`)

Download the latest `.deb` package from [Releases](https://github.com/awaismirza/clipboard-linux/releases/latest):

```bash
wget https://github.com/awaismirza/clipboard-linux/releases/download/v0.1.0/ClipKeeper_0.1.0_arm64.deb
sudo dpkg -i ClipKeeper_0.1.0_arm64.deb
```

### Option 2: Fedora / RHEL (`.rpm`)

```bash
wget https://github.com/awaismirza/clipboard-linux/releases/download/v0.1.0/ClipKeeper-0.1.0-1.aarch64.rpm
sudo rpm -i ClipKeeper-0.1.0-1.aarch64.rpm
```

### Option 3: Standalone AppImage

```bash
wget https://github.com/awaismirza/clipboard-linux/releases/download/v0.1.0/ClipKeeper_0.1.0_aarch64.AppImage
chmod +x ClipKeeper_0.1.0_aarch64.AppImage
./ClipKeeper_0.1.0_aarch64.AppImage
```

---

## Keyboard Shortcuts

| Shortcut | Description |
|---|---|
| `Alt + Shift + V` | Open / Toggle ClipKeeper floating palette |
| `Super + Shift + V` | Alternate global shortcut |
| `Ctrl + Shift + V` | Alternate global shortcut |
| `Arrow Keys` / `Tab` | Navigate search items |
| `Enter` | Copy selected item to clipboard |
| `Esc` | Close search palette |

---

## Build from Source

### Prerequisites

- Node.js & npm
- Rust (`cargo`)
- System libraries: `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`

### Development Setup

```bash
# Clone the repository
git clone https://github.com/awaismirza/clipboard-linux.git
cd clipboard-linux

# Install dependencies
npm install

# Run development server
npm run tauri dev
```

### Build Release

```bash
npm run tauri build
```

Compiled binaries and package bundles (`.deb`, `.rpm`, `.AppImage`) will be generated inside `src-tauri/target/release/bundle/`.

---

## License

MIT License. See [LICENSE](LICENSE) for details.
