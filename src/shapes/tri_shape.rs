use crate::genericmaze::{cell_index, GenericMaze, MazeCell, Shape};

/// Triangular grid shape (3 neighbors: left, right, top/bottom)
pub struct TriShape;

impl Shape for TriShape {
    fn num_neighbors() -> usize {
        3
    }

    fn init_neighbors(width: usize, height: usize, cells: &mut [MazeCell]) {
        // Neighbor indices: 0=left, 1=right, 2=top (for down triangles) or bottom (for up triangles)
        for y in 0..height {
            for x in 0..width {
                let idx = cell_index(x, y, width);
                let is_up = (x + y) % 2 == 0;

                if is_up {
                    // Up-pointing triangle: 0=left, 1=right, 2=bottom
                    // Left neighbor (down-pointing triangle to the left)
                    if x > 0 {
                        cells[idx].neighbors[0] = Some(cell_index(x - 1, y, width));
                    }
                    // Right neighbor (down-pointing triangle to the right)
                    if x < width - 1 {
                        cells[idx].neighbors[1] = Some(cell_index(x + 1, y, width));
                    }
                    // Bottom neighbor (down-pointing triangle below)
                    if y < height - 1 {
                        cells[idx].neighbors[2] = Some(cell_index(x, y + 1, width));
                    }
                } else {
                    // Down-pointing triangle: 0=left, 1=right, 2=top
                    // Left neighbor (up-pointing triangle to the left)
                    if x > 0 {
                        cells[idx].neighbors[0] = Some(cell_index(x - 1, y, width));
                    }
                    // Right neighbor (up-pointing triangle to the right)
                    if x < width - 1 {
                        cells[idx].neighbors[1] = Some(cell_index(x + 1, y, width));
                    }
                    // Top neighbor (up-pointing triangle above)
                    if y > 0 {
                        cells[idx].neighbors[2] = Some(cell_index(x, y - 1, width));
                    }
                }
            }
        }
    }

    fn to_svg(maze: &GenericMaze<Self>, tunnel_width: usize, solution_path: Option<&[usize]>, debug: bool) -> String {
        let tri_height = (tunnel_width as f64 * 0.866).round() as usize;
        let svg_width = maze.width * tunnel_width / 2 + tunnel_width / 2;
        let svg_height = maze.height * tri_height + tri_height;

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
  <rect width="{}" height="{}" fill="white"/>
  <g stroke="black" stroke-width="2" stroke-linecap="square" fill="none">
"#,
            svg_width, svg_height, svg_width, svg_height, svg_width, svg_height
        ));

        for y in 0..maze.height {
            for x in 0..maze.width {
                let idx = maze.cell_index(x, y);
                let is_up = (x + y) % 2 == 0;

                let base_x = x * tunnel_width / 2;
                let base_y = y * tri_height;

                if is_up {
                    // Up-pointing triangle: vertices at bottom-left, top, bottom-right
                    let x1 = base_x;                      // bottom-left
                    let y1 = base_y + tri_height;
                    let x2 = base_x + tunnel_width / 2;  // top
                    let y2 = base_y;
                    let x3 = base_x + tunnel_width;      // bottom-right
                    let y3 = base_y + tri_height;

                    // Draw walls (0=left edge, 1=right edge, 2=bottom edge)
                    // Skip entrance (left edge of cell 0)
                    if maze.cells[idx].walls[0] && !(idx == 0) {
                        svg.push_str(&format!("    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n", x1, y1, x2, y2));
                    }
                    // Skip exit (right edge of last cell)
                    if maze.cells[idx].walls[1] && !(idx == maze.cells.len() - 1) {
                        svg.push_str(&format!("    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n", x2, y2, x3, y3));
                    }
                    if maze.cells[idx].walls[2] {
                        svg.push_str(&format!("    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n", x1, y1, x3, y3));
                    }
                } else {
                    // Down-pointing triangle: vertices at top-left, bottom, top-right
                    let x1 = base_x;                      // top-left
                    let y1 = base_y;
                    let x2 = base_x + tunnel_width / 2;  // bottom
                    let y2 = base_y + tri_height;
                    let x3 = base_x + tunnel_width;      // top-right
                    let y3 = base_y;

                    // Draw walls (0=left edge, 1=right edge, 2=top edge)
                    if maze.cells[idx].walls[0] {
                        svg.push_str(&format!("    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n", x1, y1, x2, y2));
                    }
                    // Skip exit (right edge of last cell)
                    if maze.cells[idx].walls[1] && !(idx == maze.cells.len() - 1) {
                        svg.push_str(&format!("    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n", x2, y2, x3, y3));
                    }
                    if maze.cells[idx].walls[2] {
                        svg.push_str(&format!("    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n", x1, y1, x3, y3));
                    }
                }
            }
        }

        svg.push_str("  </g>\n");

        // Add cell index labels for debugging
        if debug {
            svg.push_str("  <g font-family=\"monospace\" font-size=\"10\" text-anchor=\"middle\" fill=\"blue\">\n");
            for y in 0..maze.height {
                for x in 0..maze.width {
                    let idx = maze.cell_index(x, y);
                    let is_up = (x + y) % 2 == 0;
                    let base_x = x * tunnel_width / 2;
                    let base_y = y * tri_height;
                    let (center_x, center_y) = if is_up {
                        (base_x + tunnel_width / 2, base_y + tri_height * 2 / 3)
                    } else {
                        (base_x + tunnel_width / 2, base_y + tri_height / 3)
                    };
                    svg.push_str(&format!("    <text x=\"{}\" y=\"{}\">{}</text>\n", center_x, center_y + 3, idx));
                }
            }
            svg.push_str("  </g>\n");
        }

        if let Some(path) = solution_path {
            if !path.is_empty() {
                svg.push_str("  <g stroke=\"red\" stroke-width=\"3\" stroke-linecap=\"round\" fill=\"none\">\n");
                svg.push_str("    <path class=\"solution-path\" d=\"");

                for (i, &idx) in path.iter().enumerate() {
                    let (x, y) = maze.cell_coords(idx);
                    let is_up = (x + y) % 2 == 0;

                    let base_x = x * tunnel_width / 2;
                    let base_y = y * tri_height;

                    let (center_x, center_y) = if is_up {
                        (base_x + tunnel_width / 2, base_y + tri_height * 2 / 3)
                    } else {
                        (base_x + tunnel_width / 2, base_y + tri_height / 3)
                    };

                    if i == 0 {
                        svg.push_str(&format!("M {} {} ", center_x, center_y));
                    } else {
                        svg.push_str(&format!("L {} {} ", center_x, center_y));
                    }
                }

                svg.push_str("\"/>\n");
                svg.push_str("  </g>\n");
            }
        }

        svg.push_str("</svg>");
        svg
    }

    fn print_debug_info(maze: &GenericMaze<Self>) {
        println!("\n=== Triangular Maze Debug Info ===");
        println!("Grid: {}x{} (width x height)", maze.width, maze.height);
        println!("Total cells: {}", maze.cells.len());
        println!("\nNeighbor relationships (indices: 0=left, 1=right, 2=top/bottom):");

        for y in 0..maze.height {
            for x in 0..maze.width {
                let idx = maze.cell_index(x, y);
                let is_up = (x + y) % 2 == 0;
                print!("Cell {:2} (x={}, y={}, {}): [",
                    idx, x, y, if is_up { "up  " } else { "down" });

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
