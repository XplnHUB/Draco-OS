# Draco OS Daemon – Build Instructions

This guide explains how to properly build the **Draco OS Daemon (`draco_core`)**, which acts as the central intelligence engine of the system.

---

## 🧠 Overview

`draco_core` is a background service responsible for:

- Handling user input (via CLI or API)
- Communicating with LLM (Ollama)
- Injecting system context
- Managing memory
- Routing safe system commands

It runs as a **daemon (background service)** using `systemd`.

---

## ⚙️ Requirements

- Linux (Ubuntu/Debian recommended)
- Rust (latest stable)
- Cargo
- Ollama (running locally)

---

## 🔧 Step 1 – Install Dependencies

### Install Rust

```bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```

### Install Ollama

```bash
curl -fsSL https://ollama.com/install.sh | sh
ollama pull llama3
```

---

## 📁 Step 2 – Project Structure

Ensure your structure looks like:

```
Draco-OS/
│
├── draco/
│   ├── draco_core/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── brain/
│   │   │   ├── system/
│   │   │   ├── memory/
│   │   │   ├── security/
│   │   │   ├── automation/
│   │   │   └── ipc/
│   │   │
│   │   └── Cargo.toml
```

---

## 🏗️ Step 3 – Add Dependencies

Inside `draco/draco_core`:

```bash
cargo add tokio --features full
cargo add axum
cargo add serde serde_json
cargo add reqwest --features json
cargo add sysinfo
cargo add anyhow
```

---

## 🚀 Step 4 – Build the Daemon

```bash
cd draco/draco_core
cargo build --release
```

Binary will be available at:

```
target/release/draco_core
```

---

## ▶️ Step 5 – Run the Daemon (Development)

```bash
cargo run
```

Expected output:

```
Draco Core starting...
Listening on 127.0.0.1:8080
```

---

## 🔌 Step 6 – Verify API

Test with curl:

```bash
curl -X POST http://127.0.0.1:8080/input \
-H "Content-Type: application/json" \
-d '{"message":"hello"}'
```

---

## 🧩 Step 7 – Install as System Binary

```bash
sudo cp target/release/draco_core /usr/local/bin/
```

---

## ⚙️ Step 8 – Create systemd Service

Create file:

```bash
sudo nano /etc/systemd/system/draco.service
```

Add:

```ini
[Unit]
Description=Draco OS Core Daemon
After=network.target

[Service]
ExecStart=/usr/local/bin/draco_core
Restart=always
User=root
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

---

## 🔄 Step 9 – Enable & Start Service

```bash
sudo systemctl daemon-reexec
sudo systemctl daemon-reload
sudo systemctl enable draco
sudo systemctl start draco
```

---

## 📜 Step 10 – Check Logs

```bash
journalctl -u draco -f
```

---

## 🧠 Next Steps

After daemon is running:

* Implement LLM integration (Ollama API)
* Add system metrics collection
* Add intent classification
* Add safe command execution layer
* Add memory persistence

---

## 🔐 Notes

* Do NOT allow raw shell execution
* Always validate commands before execution
* Log every action
* Keep AI decision separate from execution

---

## ✅ Result

You now have a running:

* Background AI daemon
* Local API server
* System-integrated service

This is the foundation of Draco OS.
