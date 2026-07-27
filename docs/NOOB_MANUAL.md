# Aetherion n00b bible

**One job:** install tools → get source → compile → drop Godot in the folder → run the CLI → press **g** → make a game.

**Time:** ~20–40 minutes the first time (Rust + VS tools dominate).  
**OS:** Windows steps below. Linux/macOS are the same ideas with different binary names.

---

## What the pieces are

| Piece | What it is |
|-------|------------|
| **Godot 4** | Game editor. You download it. Drop the `.exe` into the Aetherion repo root. |
| **Aetherion** | Rust library that becomes `aetherion.dll` — a **plugin** Godot loads. |
| **aetherion-cli** | Small menu program (`aetherion-cli.exe`) that builds, installs, and launches. |
| **Your game** | A Godot project folder. Menu **g** can create one for you. |

Aetherion is **not** a second editor. It is a **GDExtension**.  
Godot loads the DLL. You place an **`AetherionEngine`** node (menu **g** already does this).

```text
  YOU                         AETHERION FOLDER
  ───                         ────────────────
  install Rust + Git
  git clone            ──►    source code
  cargo build          ──►    target\debug\aetherion-cli.exe
  download Godot       ──►    Godot_v*-stable_*.exe  (repo root)
  run CLI → press g    ──►    godot_save\… + plugin + editor open
                                      │
                                      ▼
                              make a game
```

---

## Step 0 — Install tools

### Git

[https://git-scm.com](https://git-scm.com)  
Install with defaults. Open a **new** PowerShell after install.

### Rust

[https://rustup.rs](https://rustup.rs)

```powershell
# After rustup finishes:
rustc --version
cargo --version
```

If those fail, close the terminal completely and open a new one.

This repo pins a toolchain in `rust-toolchain.toml` (currently **1.87**).  
First build may download that channel automatically.

### Windows C++ build tools

Rust on Windows needs a linker:

1. Install **Visual Studio Build Tools** (or full VS).  
2. Workload: **Desktop development with C++**.  
3. New terminal again.

### Godot 4 stable

1. Open [https://godotengine.org/download](https://godotengine.org/download)  
2. Download **standard** Godot 4 **Windows 64-bit** (not .NET unless you know you want it).  
3. Unzip — you want a file like:

```text
Godot_v4.7.1-stable_win64.exe
```

(Version numbers change. Any **4.2+** stable is fine; **4.5–4.7** are tested.)

---

## Step 1 — Download Aetherion source

```powershell
cd C:\ZV9\lines\games
# or wherever you keep game code:

git clone https://github.com/zv9games/aetherion.git
cd aetherion
```

You should see `Cargo.toml` in the current folder.

---

## Step 2 — Copy Godot into the Aetherion root

Copy / move the Godot `.exe` **into the aetherion folder** (same place as `Cargo.toml`):

```text
aetherion\
  Cargo.toml
  README.md
  Godot_v4.7.1-stable_win64.exe     ★ here
  crates\
  docs\
  examples\
  target\                           (appears after you build)
```

The CLI looks for any `Godot*.exe` in this folder automatically.

**Optional** (if you refuse to copy Godot here):

```powershell
$env:GODOT_BIN = "C:\path\to\Godot_v4.x_win64.exe"
```

**Do not commit Godot** — git ignores `Godot_*.exe` on purpose (huge file, not ours to redistribute).

---

## Step 3 — Compile the binary

From the **repo root** (`aetherion\`):

```powershell
cargo build -p aetherion-cli
```

First compile can take a while. When it finishes:

```text
Finished `dev` profile …
```

### Where is the .exe?

```text
target\debug\aetherion-cli.exe
```

| Profile | Command | Binary |
|---------|---------|--------|
| debug (default, fine for learning) | `cargo build -p aetherion-cli` | `target\debug\aetherion-cli.exe` |
| release (faster, longer compile) | `cargo build -p aetherion-cli --release` | `target\release\aetherion-cli.exe` |

Check Godot was found:

```powershell
.\target\debug\aetherion-cli.exe doctor
```

You want:

```text
health=aetherion-ok
godot=C:\...\Godot_v....exe
```

If `godot=(not configured)`, the `.exe` is not in the repo root and `GODOT_BIN` is unset — fix Step 2.

---

## Step 4 — Run the menu, press **g**, make a game

```powershell
.\target\debug\aetherion-cli.exe
```

(or `cargo run -p aetherion-cli`)

```text
  Aetherion
  C:\...\aetherion
  godot: C:\...\Godot_v4.x...exe

  1  d    doctor
  2  b    build
  3  l    launch demo
  4  g    launch godot + aetherion  ★ make a game
  5  s    smoke
  6  f    bench 4M
  7  t    bench 10M
  0  q    quit
     h    help

aetherion>
```

### Make a game (the gift)

1. Type **`g`** (or **`4`**) → Enter  
2. Project path prompt → press **Enter** for the default  
   (`godot_save\aetherion-game`)  
   or paste any empty folder path  
3. Wait for build + install  
4. Godot **editor** opens  

What the CLI just did for you:

- Created a Godot project if the folder was empty  
- Built `aetherion.dll` with Godot support  
- Wrote `aetherion.gdextension`  
- Installed **`aetherion_engine_api.gd`** and attached it to **AetherionEngine**  
- Opened the **editor** (not “run game”)

In Godot:

- Select **AetherionEngine** in the scene tree — script: `aetherion_engine_api.gd`  
- That script **is** the API tutorial (`extends AetherionEngine`, every callable)  
- Press **F5** / Play: HUD + Output walk `bind_*`, `generate_region`, benches…  
- Keys: **1** light tour · **2** +`flood_million` · **3** `flood_10m` · **Space** skip hold  

Repo template (re-copied on each **g** / install): `templates/gift/aetherion_engine_api.gd`

**After you change Rust code:** run the CLI again → **`g`** on the same path.

### Repo demo (menu **l**)

Type **`l`** for the bigger staged showcase under `examples/godot_demo/` (separate from your gift game).

### Other keys

| Key | Action |
|-----|--------|
| `d` | doctor — health + paths |
| `b` | build plugin only |
| `s` | smoke — headless proof |
| `f` / `t` | CPU benches ~4M / ~10M tiles |
| `q` | quit |
| `h` | reprint menu |

---

## Success looks like

**CLI / Output dock:**

```text
[Aetherion] GDExtension Scene init …
[Aetherion] ready — health=aetherion-ok
```

**Demo (`l`):** large window, dark background, stage timer HUD, floods of tiles.

**Your game (`g`):** editor open on `godot_save\…` (or your path), `AetherionEngine` in the scene tree.

### If something’s wrong

| Symptom | Fix |
|---------|-----|
| `godot=(not configured)` | Put `Godot_v*.exe` in repo root or set `GODOT_BIN` |
| `Cannot get class 'AetherionEngine'` | Run menu **g** again on that project (reinstall plugin); reload project |
| `Can't run project: no main scene` | Use a **current** CLI — menu **g** opens with `--editor` and scaffolds `main.tscn` |
| Scene tree search finds nothing | Use **Create New Node** (Ctrl+A), not the scene-tree filter |
| Cargo / link errors on Windows | Install VS Build Tools “Desktop development with C++”, new terminal |
| First build forever | Normal — dependencies + Rust download |

Godot may print `API v4.5 … runtime v4.7` — that is normal (godot-rust targets a stable API).

---

## Mental model (keep this)

```text
  target\debug\aetherion-cli.exe     operator menu
           │
           ├─ g  →  install plugin into YOUR project + open editor
           ├─ l  →  run the Aetherion showcase demo
           └─ b  →  rebuild aetherion.dll after Rust edits

  aetherion.dll + aetherion.gdextension   what Godot actually loads
  AetherionEngine                         the node you add / already have
```

---

## Cheat sheet

```powershell
cd C:\path\to\aetherion

# Godot.exe sits in this folder (or set GODOT_BIN)

cargo build -p aetherion-cli
.\target\debug\aetherion-cli.exe doctor
.\target\debug\aetherion-cli.exe
# → g   make a game
# → l   launch demo
# → q   quit
```

Shell equivalents (no menu):

```powershell
cargo run -p aetherion-cli -- launch
cargo run -p aetherion-cli -- install "C:\path\to\game" --open
cargo run -p aetherion-cli -- smoke
cargo run -p aetherion-cli --release -- bench10m
```

---

## Expectation check

| This **is** | This is **not** (yet) |
|-------------|------------------------|
| Real **v0.2** **2D** GDExtension + CLI + gift API bible | A double-click store installer |
| Source you compile; Godot you download | Godot or DLLs bundled in git |
| Gift path + tracked `examples/gift_game` | 3D mesh engine (fork for that) |
| crates.io: `aetherion` / `aetherion-cli` | Guaranteed 10M-tile FPS on every laptop |

More: [ARCHITECTURE.md](ARCHITECTURE.md) · [LINEAGE.md](LINEAGE.md) · [BENCHMARKS.md](BENCHMARKS.md) · [CONTRIBUTING.md](../CONTRIBUTING.md)
