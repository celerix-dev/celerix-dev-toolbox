---
outline: deep
---

# Getting Started with Celerix

Welcome to Celerix. This guide will help you set up the toolbox and explain the core architecture that keeps your data safe and your workflow fast.

---

## 🛡️ The Celerix Philosophy: "Local-First"

Most modern tools rely on the cloud for synchronization and processing. Celerix takes a different path.

**What "Local-First" means for you:**
* **No Accounts:** You don't need to sign up or log in to use Celerix.
* **No Telemetry:** We don't track which tools you use or how often you open the app.
* **Offline by Default:** Every core utility (JSON formatting, Git status, Task management) runs entirely on your CPU without an internet connection.

---

## 🚀 Installation

Celerix is built with **Tauri 2.0**, allowing us to provide native performance on all major platforms with a tiny disk footprint.

### Windows
1. Download the latest `.msi` or `setup.exe` from our [Releases](https://github.com/celerix-dev/celerix-dev-toolbox/releases).
2. Run the installer.
3. Launch Celerix from your Start Menu.

### macOS
1. Download the `.dmg` (Universal or Silicon-specific).
2. Drag Celerix to your **Applications** folder.
3. Open Celerix (you may need to right-click and select "Open" for the first launch).

### Linux
1. Download the `.AppImage` or `.deb` package.
2. Grant execution permissions: `chmod +x Celerix.AppImage`.
3. Run the application.

---

## 🛠️ Your First Look

Once launched, you'll see the **Unified Dashboard**. Here are the three pillars of the Celerix experience:

1. **The Toolbox:** A collection of stateless utilities (Encoders, Formatters, Generators).
2. **Project Hub:** Link your local Git repositories to see a high-level overview of your work-in-progress across different folders.
3. **Privacy Kanban:** A simple, markdown-backed task board that stores your "To-Do" list in a local JSON file. No cloud sync, no prying eyes.

---

## 🔒 Security Verification

Because we believe in **Open Sovereignty**, we encourage you to verify our claims:
* **Audit the Code:** All logic is open-source at [github.com/celerix-dev](https://github.com/celerix-dev).
* **Check the Traffic:** Use a tool like Wireshark or Little Snitch while using Celerix. You will see zero outgoing traffic to our servers.
