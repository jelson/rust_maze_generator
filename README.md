# Rust Maze Generator

A Rust CLI tool that generates solvable mazes on **rectangular, triangular, hexagonal, or octagonal grids** as SVG files with automatic solution generation.

**[🎮 Try the Interactive Web Version!](https://htmlpreview.github.io/?https://github.com/jelson/rust_maze_generator/blob/main/maze.html)** - Play mazes directly in your browser with all grid types and difficulty levels.

## Features

- **Four grid types**: Rectangular (4 neighbors), Triangular (3 neighbors), Hexagonal (6 neighbors), Octagonal (4-8 neighbors)
- **Two difficulty levels**: Easy (long winding corridors) and Hard (more branching/dead ends)
- **Generic maze implementation**: Uses Rust traits and generics for grid-agnostic algorithms
- **Perfect mazes**: Exactly one path between any two points
- **Automatic solving**: BFS pathfinding with red solution path
- **Debug mode**: Optional cell numbering for debugging
- **SVG output**: Scalable vector graphics viewable in any browser
- **Configurable**: Custom dimensions and tunnel width

## Quick Start

```bash
# Build the project
cargo build --release

# Generate a 50x50 rectangular maze (easy difficulty)
./target/release/maze -W 50 -H 50 -o maze.svg

# Generate a harder maze with more branching
./target/release/maze -W 50 -H 50 -D hard -o maze_hard.svg

# Generate a hexagonal maze
./target/release/maze -W 30 -H 30 -g hexagonal -o hex_maze.svg

# Generate an octagonal maze
./target/release/maze -W 30 -H 30 -g octagonal -o oct_maze.svg

# Generate with debug cell numbers
./target/release/maze -W 10 -H 10 -d -o debug_maze.svg
```

Each run generates two files:
- `maze.svg` - The unsolved maze
- `maze_solution.svg` - The maze with solution path in red

## Example Output

Sample mazes are included in the `examples/` directory (all 20×20 cells). Compare Easy (long corridors) vs Hard (more branching):

### Rectangular Grid

<table>
<tr>
<td><img src="examples/rect_20x20_easy_solution.svg" width="400" alt="Easy rectangular maze"></td>
<td><img src="examples/rect_20x20_hard_solution.svg" width="400" alt="Hard rectangular maze"></td>
</tr>
<tr>
<td align="center">Easy</td>
<td align="center">Hard</td>
</tr>
</table>

### Triangular Grid

<table>
<tr>
<td><img src="examples/tri_20x20_easy_solution.svg" width="400" alt="Easy triangular maze"></td>
<td><img src="examples/tri_20x20_hard_solution.svg" width="400" alt="Hard triangular maze"></td>
</tr>
<tr>
<td align="center">Easy</td>
<td align="center">Hard</td>
</tr>
</table>

### Hexagonal Grid

<table>
<tr>
<td><img src="examples/hex_20x20_easy_solution.svg" width="400" alt="Easy hexagonal maze"></td>
<td><img src="examples/hex_20x20_hard_solution.svg" width="400" alt="Hard hexagonal maze"></td>
</tr>
<tr>
<td align="center">Easy</td>
<td align="center">Hard</td>
</tr>
</table>

### Octagonal Grid (Truncated Square Tiling)

<table>
<tr>
<td><img src="examples/octagonal_20x20_easy_solution.svg" width="400" alt="Easy octagonal maze"></td>
<td><img src="examples/octagonal_20x20_hard_solution.svg" width="400" alt="Hard octagonal maze"></td>
</tr>
<tr>
<td align="center">Easy</td>
<td align="center">Hard</td>
</tr>
</table>

## Command-Line Arguments

| Flag | Short | Description | Required | Default |
|------|-------|-------------|----------|---------|
| `--width` | `-W` | Width of maze in cells | Yes | - |
| `--height` | `-H` | Height of maze in cells | Yes | - |
| `--output` | `-o` | Output SVG file path | Yes | - |
| `--tunnel-width` | `-t` | Width of tunnels in pixels | No | 20 |
| `--grid-type` | `-g` | Grid type: rectangular, triangular, hexagonal, octagonal | No | rectangular |
| `--difficulty` | `-D` | Difficulty: easy (long corridors), hard (more branching) | No | easy |
| `--debug` | `-d` | Enable debug mode (show cell numbers) | No | false |

Note: `-W`, `-H`, and `-D` use capital letters to avoid conflicts with common short flags.

## Architecture

The project uses a **generic trait-based design** that separates grid topology from maze algorithms:

### Core Components

#### 1. `GenericMaze<S: Shape>` (src/genericmaze.rs)
Generic maze structure parameterized by shape type:
- **Fields**:
  - `width`, `height`: Grid dimensions
  - `cells`: Vec of `MazeCell` (each has neighbors and walls)
  - `_shape`: PhantomData marker for the shape type
- **Methods**:
  - `new()`: Creates maze and initializes neighbor relationships
  - `generate(is_hard)`: Frontier-based maze generation with difficulty selection
  - `solve()`: BFS pathfinding from cell 0 to last cell
  - `cell_index()`, `cell_coords()`: Coordinate conversion helpers

#### 2. `Shape` Trait (src/genericmaze.rs)
Defines grid-specific behavior:
- `num_neighbors()`: How many neighbors each cell type has
- `init_neighbors()`: Build neighbor relationships for the grid
- `to_svg()`: Render maze as SVG for this grid type
- `print_debug_info()`: Debug output (optional)

#### 3. `MazeCell` (src/genericmaze.rs)
Individual cell in the maze:
- `neighbors: Vec<Option<usize>>`: Indices of neighboring cells (None for edges)
- `walls: Vec<bool>`: Which walls are present (parallel to neighbors)

#### 4. Shape Implementations
Each in its own file under `src/shapes/`:

**RectShape** (rect_shape.rs):
- 4 neighbors per cell: North, South, East, West
- Neighbors indexed as: 0=N, 1=S, 2=E, 3=W
- Standard rectangular grid rendering

**TriShape** (tri_shape.rs):
- 3 neighbors per cell: Left, Right, Top/Bottom
- Alternating up-pointing (△) and down-pointing (▽) triangles
- Neighbors indexed as: 0=left, 1=right, 2=top/bottom

**HexShape** (hex_shape.rs):
- 6 neighbors per cell: N, S, NE, SE, NW, SW
- Flat-top hexagons with odd columns offset down
- Neighbors indexed as: 0=N, 1=S, 2=NE, 3=SE, 4=NW, 5=SW

**OctShape** (oct_shape.rs):
- Truncated square tiling with octagons and squares
- Octagons (8 neighbors): N, S, E, W, NE, SE, NW, SW
- Squares (4 neighbors): N, S, E, W
- Checkerboard pattern: octagon when (x+y) is even, square when odd
- All edges have equal length for proper tessellation
- Neighbors indexed - Octagons: 0=N, 1=S, 2=E, 3=W, 4=NE, 5=SE, 6=NW, 7=SW; Squares: 0=N, 1=S, 2=E, 3=W

## Algorithms

### Maze Generation: Frontier-Based with Difficulty Levels

Both difficulty levels use the same core algorithm but differ in **frontier management strategy**:

**Core Algorithm**:
```
1. Start at cell 0 with all walls present
2. Mark current cell as visited, add to frontier
3. While frontier is not empty:
   a. Pick cell from frontier (strategy differs by difficulty)
   b. Find all unvisited neighbors
   c. If unvisited neighbors exist:
      - Choose one randomly
      - Remove wall between current and chosen neighbor (bidirectional)
      - Mark chosen as visited, add to frontier
   d. If no unvisited neighbors, remove from frontier
4. Repeat until all cells visited
```

**Difficulty Strategies**:
- **Easy**: Frontier = Stack (LIFO)
  - Always picks the most recently added cell
  - Creates long winding corridors with less branching
  - Classic "recursive backtracking" feel

- **Hard**: Frontier = Set (random selection)
  - Picks a random cell from all active frontiers
  - Creates more uniform complexity throughout
  - Higher branching factor and more dead ends
  - More challenging to solve

**Properties**:
- Creates a **perfect maze** (exactly one path between any two cells)
- No loops or isolated sections
- Always solvable
- Random selection creates varied mazes each run
- Same code, different data structures (strategy pattern)

### Maze Solving: Breadth-First Search

**Algorithm**: BFS from entrance to exit
```
1. Start BFS from cell 0 (entrance)
2. For each cell, explore neighbors through open edges (where walls[i] == false)
3. Track parent pointers to reconstruct path
4. Stop when reaching last cell (exit)
5. Backtrack using parent pointers to build solution path
```

**Properties**:
- Finds the **shortest solution path**
- Guaranteed to find a solution (maze is perfect)
- Returns vector of cell indices forming the path

## SVG Rendering

### Rectangular Grids
- Each cell is a rectangle
- Walls are SVG lines at cell boundaries
- Cell size = tunnel_width + wall_thickness (2px)

### Triangular Grids
- Alternating up/down triangles
- Triangle height = tunnel_width × 0.866 (equilateral triangles)
- Width advances by half the tunnel width per column

### Hexagonal Grids
- Flat-top hexagons
- Odd columns offset down by half a hex height
- Width advances by 3/4 of hex width per column

### Octagonal Grids
- Truncated square tiling (octagons + squares)
- Regular octagons and squares with equal edge lengths
- Center-to-center spacing = edge_length/2 × (2 + √2)
- Checkerboard pattern alternates cell types

### Solution Path
- Drawn as red SVG path (3px stroke width, round endcaps)
- Connects centers of cells in solution sequence
- Format: `<path d="M x1 y1 L x2 y2 L x3 y3 ..." />`

### Debug Mode
- Adds blue text labels showing cell indices
- Prints neighbor relationships to stdout
- Useful for debugging neighbor initialization

## License

This project is released into the public domain. Use it however you'd like.
