# 🧙‍♂️ Instantiate ChangeOver
var changeover = ChangeOver.new()

# ⚙️ Configuration
changeover.set_grid_width(width: int)
changeover.set_grid_height(height: int)
changeover.set_chunk_size(size: int)
changeover.set_seed(seed: int)
changeover.set_can_dissolve(enabled: bool)

# 🧱 Build Chunk on TileMap Layer
changeover.build_chunk(tilemap: TileMap, layer: int)

# 🔄 Flip Logic
var should_flip = changeover.should_flip(delta: float)

# 🎲 Flip Random Tiles
var flips: Array = changeover.flip_random_tiles(count: int)

# 🕳️ Fill Grid with Black Tiles
var blackout: Array = changeover.fill_black()

# 🧬 Build Initial Matrix Frame
var matrix: Array = changeover.build_matrix(scale: float)

# 🫧 Dissolve Tiles from Matrix
var dissolve_frames: Array = changeover.dissolve_matrix(scale: float, tiles: Array[Vector2i])
