# ClipKeeper — Cross-Platform System Clipboard Manager

**ClipKeeper** is an ultra-lightweight, production-ready system clipboard manager built with **Tauri v2** and **Rust**, available for **Windows**, **macOS**, and **Linux** (X11 and Wayland).

![ClipKeeper Release](https://img.shields.io/github/v/release/awaismirza/clipkeeper-linux?color=blue&label=version)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-orange)
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

Grab the installer for your OS from [Releases](https://github.com/awaismirza/clipkeeper-linux/releases/latest):

### Windows (`.msi`)

Download `ClipKeeper_<version>_x64_en-US.msi` and run it.

### macOS (`.dmg`)

Download `ClipKeeper_<version>_<arch>.dmg`, open it, and drag ClipKeeper into Applications.
On first launch you may need to right-click → Open, since the build isn't notarized.

### Debian / Ubuntu (`.deb`)

```bash
wget https://github.com/awaismirza/clipkeeper-linux/releases/latest/download/ClipKeeper_<version>_amd64.deb
sudo dpkg -i ClipKeeper_<version>_amd64.deb
```

### Fedora / RHEL (`.rpm`)

```bash
wget https://github.com/awaismirza/clipkeeper-linux/releases/latest/download/ClipKeeper-<version>-1.x86_64.rpm
sudo rpm -i ClipKeeper-<version>-1.x86_64.rpm
```

### Standalone AppImage

```bash
wget https://github.com/awaismirza/clipkeeper-linux/releases/latest/download/ClipKeeper_<version>_amd64.AppImage
chmod +x ClipKeeper_<version>_amd64.AppImage
./ClipKeeper_<version>_amd64.AppImage
```

---

## Keyboard Shortcuts

| Shortcut | Description |
|---|---|
| `Alt + Shift + V` | Open / Toggle ClipKeeper floating palette |
| `Super + Shift + V` | Alternate global shortcut |
| `Ctrl + Shift + V` | Alternate global shortcut |
| `Arrow Keys` / `Tab` | Navigate search items |
| `Enter` | Copy selected item and auto-paste (`Ctrl+V`) into target application |
| `Shift + Enter` | Copy selected item to clipboard without auto-pasting |
| `Esc` | Close search palette |

---

## Build from Source

### Prerequisites

- Node.js & npm
- Rust (`cargo`)
- Linux only: `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`
- Windows only: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (WebView2 ships with Windows 10/11)
- macOS only: Xcode Command Line Tools (`xcode-select --install`)

### Development Setup

```bash
# Clone the repository
git clone https://github.com/awaismirza/clipkeeper-linux.git
cd clipkeeper-linux

# Install dependencies
npm install

# Run development server
npm run tauri dev
```

### Build Release

```bash
npm run tauri build
```

Compiled binaries and package bundles are generated inside `src-tauri/target/release/bundle/`:
`.msi`/`.exe` on Windows, `.dmg`/`.app` on macOS, `.deb`/`.rpm`/`.AppImage` on Linux.

---

## License

MIT License. See [LICENSE](LICENSE) for details.
