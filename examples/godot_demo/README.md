# Aetherion Godot demo

Minimal Godot 4.x project that loads the Aetherion GDExtension.

## Setup

1. Install **Godot 4.2+**.
2. From the repo root:

```bash
# Windows PowerShell
$env:GODOT_BIN = "C:\path\to\Godot_v4.x_win64.exe"
cargo run -p aetherion-cli -- run
```

Or step by step:

```bash
cargo run -p aetherion-cli -- build
cargo run -p aetherion-cli -- deploy
cargo run -p aetherion-cli -- run
```

3. Check the Godot **Output** dock for `[Aetherion] ready`.

The `aetherion.dll` / `libaetherion.so` file is produced by the CLI and is **gitignored**.
