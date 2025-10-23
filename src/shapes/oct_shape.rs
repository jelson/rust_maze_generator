use crate::genericmaze::{cell_index, GenericMaze, MazeCell, Shape};

/// Octagon + Square grid shape (truncated square tiling)
/// Layout: Octagons at main grid points with squares filling the gaps
/// Octagons have 8 neighbors, squares have 4 neighbors
pub struct OctShape;

impl OctShape {
    /// Returns true if the cell at (x, y) is an octagon (vs a square)
    /// Checkerboard pattern: octagon when (x + y) is even
    fn is_octagon(x: usize, y: usize) -> bool {
        (x + y) % 2 == 0
    }
}

impl Shape for OctShape {
    fn num_neighbors() -> usize {
        // Maximum neighbors for any cell type (octagons have 8)
        8
    }

    fn init_neighbors(width: usize, height: usize, cells: &mut [MazeCell]) {
        // Build neighbor relationships for octagon+square tiling
        // Neighbor indices for octagons: 0=N, 1=S, 2=E, 3=W, 4=NE, 5=SE, 6=NW, 7=SW
        // Neighbor indices for squares: 0=N, 1=S, 2=E, 3=W

        for y in 0..height {
            for x in 0..width {
                let idx = cell_index(x, y, width);

                if Self::is_octagon(x, y) {
                    // Octagon: connect to 4 adjacent squares and 4 diagonal octagons

                    // N - square above
                    if y > 0 {
                        cells[idx].neighbors[0] = Some(cell_index(x, y - 1, width));
                    }

                    // S - square below
                    if y < height - 1 {
                        cells[idx].neighbors[1] = Some(cell_index(x, y + 1, width));
                    }

                    // E - square to the right
                    if x < width - 1 {
                        cells[idx].neighbors[2] = Some(cell_index(x + 1, y, width));
                    }

                    // W - square to the left
                    if x > 0 {
                        cells[idx].neighbors[3] = Some(cell_index(x - 1, y, width));
                    }

                    // NE - octagon diagonally up-right
                    if x < width - 1 && y > 0 {
                        cells[idx].neighbors[4] = Some(cell_index(x + 1, y - 1, width));
                    }

                    // SE - octagon diagonally down-right
                    if x < width - 1 && y < height - 1 {
                        cells[idx].neighbors[5] = Some(cell_index(x + 1, y + 1, width));
                    }

                    // NW - octagon diagonally up-left
                    if x > 0 && y > 0 {
                        cells[idx].neighbors[6] = Some(cell_index(x - 1, y - 1, width));
                    }

                    // SW - octagon diagonally down-left
                    if x > 0 && y < height - 1 {
                        cells[idx].neighbors[7] = Some(cell_index(x - 1, y + 1, width));
                    }

                } else {
                    // Square: connect to 4 surrounding octagons (N, S, E, W)

                    // N - octagon above
                    if y > 0 {
                        cells[idx].neighbors[0] = Some(cell_index(x, y - 1, width));
                    }

                    // S - octagon below
                    if y < height - 1 {
                        cells[idx].neighbors[1] = Some(cell_index(x, y + 1, width));
                    }

                    // E - octagon to the right
                    if x < width - 1 {
                        cells[idx].neighbors[2] = Some(cell_index(x + 1, y, width));
                    }

                    // W - octagon to the left
                    if x > 0 {
                        cells[idx].neighbors[3] = Some(cell_index(x - 1, y, width));
                    }
                }
            }
        }
    }

    fn to_svg(maze: &GenericMaze<Self>, tunnel_width: usize, solution_path: Option<&[usize]>, debug: bool) -> String {
        // In truncated square tiling:
        // - tunnel_width is the edge length (all edges are equal length)
        // - Center-to-center spacing = edge_length/2 * (2 + sqrt(2))
        let edge_length = tunnel_width as f64;
        let spacing = edge_length / 2.0 * (2.0 + std::f64::consts::SQRT_2);

        // Margin needs to accommodate the octagon's farthest extent from its center
        // which is edge_length/2 * (1 + sqrt(2)) for the N/S/E/W edges
        let margin = edge_length / 2.0 * (1.0 + std::f64::consts::SQRT_2) + 10.0;

        let svg_width = (maze.width as f64 * spacing + 2.0 * margin).ceil() as usize;
        let svg_height = (maze.height as f64 * spacing + 2.0 * margin).ceil() as usize;

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
  <rect width="{}" height="{}" fill="white"/>
  <g stroke="black" stroke-width="2" stroke-linecap="square" fill="none">
"#,
            svg_width, svg_height, svg_width, svg_height, svg_width, svg_height
        ));

        // Helper to get cell center
        // All cells are on a regular grid with uniform spacing
        let get_center = |x: usize, y: usize| -> (f64, f64) {
            let cx = margin + x as f64 * spacing;
            let cy = margin + y as f64 * spacing;
            (cx, cy)
        };

        // Draw all cells and walls
        for y in 0..maze.height {
            for x in 0..maze.width {
                let idx = maze.cell_index(x, y);
                let (cx, cy) = get_center(x, y);

                if Self::is_octagon(x, y) {
                    // Draw octagon walls
                    // For a regular octagon with edge length a:
                    // - Distance from center to midpoint of N/S/E/W edge = a/2 * (1 + sqrt(2))
                    // - The corners are at distance a/2 horizontally/vertically from the center axis
                    let half_edge = edge_length / 2.0;
                    let radius = half_edge * (1.0 + std::f64::consts::SQRT_2);

                    let points = [
                        (cx - half_edge, cy - radius),     // Top-left
                        (cx + half_edge, cy - radius),     // Top-right
                        (cx + radius, cy - half_edge),     // Right-top
                        (cx + radius, cy + half_edge),     // Right-bottom
                        (cx + half_edge, cy + radius),     // Bottom-right
                        (cx - half_edge, cy + radius),     // Bottom-left
                        (cx - radius, cy + half_edge),     // Left-bottom
                        (cx - radius, cy - half_edge),     // Left-top
                    ];

                    // Draw walls based on neighbor connections
                    // Wall between top-left and top-right (N square)
                    if maze.cells[idx].walls[0] {
                        svg.push_str(&format!("    <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"/>\n",
                            points[0].0, points[0].1, points[1].0, points[1].1));
                    }

                    // Wall between bottom-left and bottom-right (S square)
                    if maze.cells[idx].walls[1] {
                        svg.push_str(&format!("    <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"/>\n",
                            points[4].0, points[4].1, points[5].0, points[5].1));
                    }

                    // Wall between right-top and right-bottom (E square)
                    if maze.cells[idx].walls[2] {
                        svg.push_str(&format!("    <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"/>\n",
                            points[2].0, points[2].1, points[3].0, points[3].1));
                    }

                    // Wall between left-top and left-bottom (W square)
                    if maze.cells[idx].walls[3] {
                        svg.push_str(&format!("    <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"/>\n",
                            points[6].0, points[6].1, points[7].0, points[7].1));
                    }

                    // Diagonal walls (NE octagon)
                    if maze.cells[idx].walls[4] {
                        svg.push_str(&format!("    <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"/>\n",
                            points[1].0, points[1].1, points[2].0, points[2].1));
                    }

                    // SE octagon (skip for last cell - exit)
                    if idx != maze.cells.len() - 1 && maze.cells[idx].walls[5] {
                        svg.push_str(&format!("    <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"/>\n",
                            points[3].0, points[3].1, points[4].0, points[4].1));
                    }

                    // NW octagon (skip for first cell - entry)
                    if idx != 0 && maze.cells[idx].walls[6] {
                        svg.push_str(&format!("    <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"/>\n",
                            points[7].0, points[7].1, points[0].0, points[0].1));
                    }

                    // SW octagon
                    if maze.cells[idx].walls[7] {
                        svg.push_str(&format!("    <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"/>\n",
                            points[5].0, points[5].1, points[6].0, points[6].1));
                    }

                } else {
                    // Draw square walls
                    // Squares have 4 walls based on neighbors: 0=N, 1=S, 2=E, 3=W
                    // Square has side length = edge_length
                    let half_edge = edge_length / 2.0;

                    let points = [
                        (cx - half_edge, cy - half_edge),  // Top-left
                        (cx + half_edge, cy - half_edge),  // Top-right
                        (cx + half_edge, cy + half_edge),  // Bottom-right
                        (cx - half_edge, cy + half_edge),  // Bottom-left
                    ];

                    // N wall
                    if idx != 0 && maze.cells[idx].walls[0] {
                        svg.push_str(&format!("    <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"/>\n",
                            points[0].0, points[0].1, points[1].0, points[1].1));
                    }

                    // S wall
                    if idx != maze.cells.len() - 1 && maze.cells[idx].walls[1] {
                        svg.push_str(&format!("    <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"/>\n",
                            points[2].0, points[2].1, points[3].0, points[3].1));
                    }

                    // E wall
                    if maze.cells[idx].walls[2] {
                        svg.push_str(&format!("    <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"/>\n",
                            points[1].0, points[1].1, points[2].0, points[2].1));
                    }

                    // W wall
                    if idx != 0 && maze.cells[idx].walls[3] {
                        svg.push_str(&format!("    <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"/>\n",
                            points[0].0, points[0].1, points[3].0, points[3].1));
                    }
                }
            }
        }

        svg.push_str("  </g>\n");

        // Draw solution path if provided
        if let Some(path) = solution_path {
            if !path.is_empty() {
                svg.push_str("  <g stroke=\"red\" stroke-width=\"3\" stroke-linecap=\"round\" fill=\"none\">\n");
                svg.push_str("    <path d=\"");

                for (i, &cell_idx) in path.iter().enumerate() {
                    let (x, y) = maze.cell_coords(cell_idx);
                    let (cx, cy) = get_center(x, y);
                    if i == 0 {
                        svg.push_str(&format!("M {:.2} {:.2} ", cx, cy));
                    } else {
                        svg.push_str(&format!("L {:.2} {:.2} ", cx, cy));
                    }
                }

                svg.push_str("\"/>\n");
                svg.push_str("  </g>\n");
            }
        }

        // Debug: cell numbers
        if debug {
            svg.push_str("  <g font-size=\"12\" fill=\"blue\" text-anchor=\"middle\">\n");
            for y in 0..maze.height {
                for x in 0..maze.width {
                    let idx = maze.cell_index(x, y);
                    let (cx, cy) = get_center(x, y);
                    svg.push_str(&format!("    <text x=\"{:.2}\" y=\"{:.2}\">{}</text>\n", cx, cy + 4.0, idx));
                }
            }
            svg.push_str("  </g>\n");
        }

        svg.push_str("</svg>\n");
        svg
    }

    fn print_debug_info(maze: &GenericMaze<Self>) {
        println!("\n=== Octagonal Maze Debug Info ===");
        println!("Grid: {}x{} (width x height)", maze.width, maze.height);
        println!("Total cells: {}", maze.cells.len());
        println!("\nCell types and neighbor relationships:");
        println!("Octagons (indices: 0=N, 1=S, 2=E, 3=W, 4=NE, 5=SE, 6=NW, 7=SW)");
        println!("Squares (indices: 0=N, 1=S, 2=E, 3=W)");

        for y in 0..maze.height {
            for x in 0..maze.width {
                let idx = maze.cell_index(x, y);
                let cell_type = if Self::is_octagon(x, y) { "OCT" } else { "SQR" };
                print!("Cell {:2} (x={}, y={}) [{}]: [", idx, x, y, cell_type);

                for (i, &neighbor) in maze.cells[idx].neighbors.iter().enumerate() {
                    if i > 0 { print!(", "); }
                    match neighbor {
                        Some(n) => print!("{:2}", n),
                        None => print!("--"),
                    }
                }
                println!("]");
            }
        }
    }
}
