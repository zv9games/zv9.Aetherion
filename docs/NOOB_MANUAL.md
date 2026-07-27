# Aetherion in one page (Godot n00b edition)

**Goal:** Install tools → build Aetherion → open a Godot project → see Aetherion run.  
**Time:** ~15–30 minutes if Rust and Godot install cleanly.  
**OS:** Windows (paths shown); Linux/macOS same steps with different binary names.

---

## What you are installing

| Piece | What it is | Who provides it |
|-------|------------|-----------------|
| **Godot 4** | The game editor / runtime | You download it (not in this repo) |
| **Aetherion** | Rust code that becomes a **plugin DLL** Godot loads | This git repo |
| **Demo project** | Tiny Godot project that already hooks Aetherion | `examples/godot_demo/` in this repo |

Aetherion is **not** a second editor. It is a **GDExtension**: Godot loads `aetherion.dll` (or `.so` / `.dylib`) like a plugin.

---

## 0. Prerequisites

1. **Git** — [https://git-scm.com](https://git-scm.com)  
2. **Rust** — [https://rustup.rs](https://rustup.rs) (install, then close and reopen the terminal)  
3. **Visual Studio Build Tools** (Windows) — “Desktop development with C++” so Rust can link  
4. **Godot 4.2 or newer** (4.5.x is fine)  
   - Download: [https://godotengine.org/download](https://godotengine.org/download)  
   - Get the **standard** Windows 64-bit build (`.exe`). Unzip anywhere, e.g.  
     `C:\Godot\Godot_v4.5.1-stable_win64.exe`  
   - You do **not** put this file inside the Aetherion repo.

---

## 1. Get Aetherion

```powershell
cd C:\ZV9\lines\games
# or any folder you like:
git clone https://github.com/zv9games/aetherion.git
cd aetherion
```

If you already have the repo: `cd C:\ZV9\lines\games\aetherion`.

---

## 2. Point the CLI at Godot

```powershell
$env:GODOT_BIN = "C:\Godot\Godot_v4.5.1-stable_win64.exe"
# use YOUR real path to the .exe
```

Check:

```powershell
cargo run -p aetherion-cli -- doctor
```

You want lines like `aetherion 0.1.0`, `health=aetherion-ok`, and `godot=C:\...exe` (not “not configured”).

---

## 3. Build the extension + hook it into the demo (one command)

From the **repo root** (`aetherion/`, next to `Cargo.toml`):

```powershell
cargo run -p aetherion-cli -- run
```

That single command:

1. **Builds** the Rust library with Godot support (`aetherion.dll` on Windows)  
2. **Copies** it into `examples/godot_demo/` (this is the “hook in” step)  
3. **Writes** Godot’s extension list so Godot finds `aetherion.gdextension`  
4. **Launches** Godot on `examples/godot_demo`

**Do not** type only `cargo run` — always use `-p aetherion-cli -- …`.

### Headless test (no window)

```powershell
cargo run -p aetherion-cli -- smoke
```

### CPU-only stress (no Godot)

```powershell
cargo run -p aetherion-cli --release -- bench4m
```

---

## 4. What “success” looks like

**In the Godot Output dock** (or console):

```text
[Aetherion] GDExtension Scene init …
[Aetherion] ready — health=aetherion-ok
[Aetherion] bound TileMap / MultiMesh …
… tiles … | apply … cells/instances …
```

**On screen:** status text with version + tile counts; colored tiles / multimesh flood.

If you see `Cannot get class 'AetherionEngine'`: the DLL was not deployed or Godot did not load the extension — run `cargo run -p aetherion-cli -- deploy` then open the project again with `run`.

**Godot version:** Built/tested against **4.5–4.7**. A line like  
`API v4.5.stable … runtime v4.7.1` is normal (godot-rust targets a stable API; newer Godot still loads it).

**MultiMesh errors** (“Instance count must be 0 to change transform format”) on older builds: update to latest `main` and rebuild (`cargo run -p aetherion-cli -- run`).

---

## 5. How the hook works (so you can make your own project later)

Inside `examples/godot_demo/`:

| File | Role |
|------|------|
| `aetherion.gdextension` | Tells Godot: load `aetherion.dll`, entry `gdext_rust_init` |
| `aetherion.dll` | Built by the CLI (gitignored; recreated by `build`/`deploy`) |
| `main.tscn` | Scene with an **`AetherionEngine`** node + TileMap / MultiMesh |
| `main.gd` | Calls `bind_tilemap` / `bind_multimesh` and shows stats |

**Your own project (after the demo works):**

1. Create a new Godot 4 project.  
2. Copy `aetherion.gdextension` into the project root.  
3. From Aetherion repo:  
   `cargo run -p aetherion-cli -- build`  
   then copy `target/release/aetherion.dll` next to that `.gdextension`  
   (or change the CLI deploy path later).  
4. Add a node of type **`AetherionEngine`** (appears after the extension loads).  
5. Optionally add a `TileMap` and call `bind_tilemap` from a script (see `main.gd`).

You must **re-run `build` + `deploy` (or `run`)** after you change Rust code.

---

## 6. Mental model (one diagram)

```text
  YOU                     AETHERION REPO                  GODOT
  ───                     ──────────────                  ─────
  Install Godot.exe  ──►  GODOT_BIN=...
  git clone          ──►  cargo run -p aetherion-cli -- run
                              │
                              ├─ cargo build → aetherion.dll
                              ├─ copy dll → examples/godot_demo/
                              └─ start Godot --path examples/godot_demo
                                                           │
                                                           ▼
                                              loads aetherion.gdextension
                                              creates AetherionEngine node
                                              generates + draws tiles
```

---

## 7. Expectation check

| This **is** | This is **not** (yet) |
|-------------|------------------------|
| A real, testable **v0.1** GDExtension + demo | A finished commercial engine product |
| Something you build from source with a CLI | A double-click installer that bundles Godot |
| Ready for learning + OSS iteration | Guaranteed multi-million **TileMap** FPS on every PC |

---

## Cheat sheet

```powershell
cd C:\ZV9\lines\games\aetherion
$env:GODOT_BIN = "C:\path\to\Godot_v4.x_win64.exe"

cargo run -p aetherion-cli -- doctor    # is Godot found?
cargo run -p aetherion-cli -- smoke     # headless proof
cargo run -p aetherion-cli -- run       # open demo and play/see it
```

More detail: [ARCHITECTURE.md](ARCHITECTURE.md) · [LINEAGE.md](LINEAGE.md) · [CONTRIBUTING.md](../CONTRIBUTING.md)
