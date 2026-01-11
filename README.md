# 🛠 Celerix Dev Toolbox

**A lightweight, local-first developer productivity suite.**

![Release Version](https://img.shields.io/github/v/release/celerix-dev/celerix-dev-toolbox?label=Pre-Alpha&color=orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Platform Support](https://img.shields.io/badge/platform-macOS%20|%20Windows%20|%20Linux-lightgrey)

---

### ⚠️ **PRE-ALPHA**
* **Usage:** This is a **Public Pre-Alpha**. It is free to use but provided "as-is" under a MIT license.
* **Use at your own risk:** Expect bugs, breaking changes, and frequent updates.

---

## 🛠 What is it?
**Celerix** is a high-performance "Swiss Army Knife" for your daily development workflow, It leverages a **Rust** backend via **Tauri 2.0** and a reactive **Vue 3** frontend to deliver a native experience with a minimal footprint.

### Core Philosophy
* **Local-First:** Your data stays on your machine. Import/Export functionality works directly with your local file system.
* **No Bloat:** A native desktop experience that stays out of your way (~35MB RAM usage).
* **Privacy:** No tracking, no cloud-syncing, no accounts required.

<img src="https://raw.githubusercontent.com/celerix-dev/celerix-dev-toolbox/main/public/assets/celerix-dev-toolbox.png" width="400" alt="Celerix Developer Toolbox" />

---

## ✨ Current Features

### 🧩 Mini Applications
Focused tools to keep your project on track:
* **Widget Dashboard:** A customizable space for your daily dev stats.
* **Kanban Board:** Drag-and-drop task management, stored locally in your toolbox.

### ⚙️ Developer Utilities
* **UUID Generator:** Fast, offline ID generation.
* **BASE64 Encode/Decode:** Encode and/or Decode base64 strings in a flash.
* **JWT Decoder:** Decode JWT tokens for quick inspection.
* **Data Portability:** Full support for importing/exporting your settings and data to local files.

### 💾 Data Portability & Compatibility
While **Data Portability** is a core pillar of this project, please be aware that our internal data schemas are evolving rapidly during this Pre-Alpha phase.

* **Compatibility:** Export files created in an older version may not be compatible with newer versions until we reach a stable data format (v1.0).
* **Recommendation:** Keep a backup of your data, but expect to perform a "fresh start" if the data structure undergoes a major shift between updates.

---

## 🛡️ Installation & Security Notes

As this project is in a **Pre-Alpha** stage, the binaries are not yet digitally signed or notarized. Your operating system will likely flag the application as a security risk. This is expected behavior for unsigned software.

### 🍎 For macOS Users
If macOS claims the app is **"Damaged"** or should be moved to the Trash, it is simply because the app is unsigned. To bypass this:

1. Drag the application into your **Applications** folder.
2. Open your **Terminal** and run:
   ```shell
   sudo xattr -rd com.apple.quarantine /Applications/Celerix.app
   ```

### 🪟 For Windows Users
Windows **SmartScreen** will likely prevent the installer from starting with a blue banner stating "Windows protected your PC." This is a standard for apps that are not yet digitally signed.

1. Click the **"More info"** link (this is the small underlined text directly under the main warning message).
2. A new button labeled **"Run anyway"** will appear at the bottom of the window.
3. Click **Run anyway** to start the installation.

### 🐧 For Linux Users
To install the `.deb` or `.rpm`package and ensure all dependencies are correctly handled, use the terminal:

Debian/Ubuntu:
```shell
sudo apt install ./Celerix_0.1.6_amd64.deb
``` 
Fedora/RHEL/OpenSUSE:
```shell
sudo dnf install ./Celerix-0.1.6-1.x86_64.rpm
```

---

## 🗺 Roadmap
- [x] **Phase 1 (Foundation):** Stabilize cross-platform builds and core utilities.
- [ ] **Phase 2 (Features):** Expand the toolset (More widgets, More mini Apps, tools like: Regex Tester).
- [ ] **Phase 3 (Stability):** Bug fixing and UI polish, and finalizing on data structures.
- [ ] **Phase 4 (1.0):** **Stable Release** Data structure stabilization and finalizing on UI/UX.

---

## ⬇️ Installation
Visit the **[Releases](https://github.com/celerix-dev/celerix-dev-toolbox/releases)** page for the latest installers:
* **macOS:** Apple Silicon & Intel versions.
* **Windows:** x64 & ARM64 versions.
* **Linux:** `.deb` package for Debian/Ubuntu and `.rpm` for Fedora/RHEL/OpenSUSE.

---

## 📜 License
See the [LICENSE](./LICENSE) file for the terms.

*Copyright © 2026 Celerix*
