use crate::genericmaze::{cell_index, GenericMaze, MazeCell, Shape};

/// Hexagonal grid shape (6 neighbors: N, S, NE, SE, NW, SW)
pub struct HexShape;

impl Shape for HexShape {
    fn num_neighbors() -> usize {
        6
    }

    fn init_neighbors(width: usize, height: usize, cells: &mut [MazeCell]) {
        // Build neighbor relationships for flat-top hexagons with odd columns offset down
        // Neighbor indices: 0=N, 1=S, 2=NE, 3=SE, 4=NW, 5=SW
        for y in 0..height {
            for x in 0..width {
                let idx = cell_index(x, y, width);
                let is_odd_col = x % 2 == 1;

                // N (always up one row)
                if y > 0 {
                    cells[idx].neighbors[0] = Some(cell_index(x, y - 1, width));
                }

                // S (always down one row)
                if y < height - 1 {
                    cells[idx].neighbors[1] = Some(cell_index(x, y + 1, width));
                }

                if is_odd_col {
                    // Odd column: offset down, so NE/SE go up-right/same-row-right, NW/SW go up-left/same-row-left
                    // NE (up-right)
                    if x < width - 1 {
                        cells[idx].neighbors[2] = Some(cell_index(x + 1, y, width));
                    }
                    // SE (down-right)
                    if y < height - 1 && x < width - 1 {
                        cells[idx].neighbors[3] = Some(cell_index(x + 1, y + 1, width));
                    }
                    // NW (up-left)
                    if x > 0 {
                        cells[idx].neighbors[4] = Some(cell_index(x - 1, y, width));
                    }
                    // SW (down-left)
                    if y < height - 1 && x > 0 {
                        cells[idx].neighbors[5] = Some(cell_index(x - 1, y + 1, width));
                    }
                } else {
                    // Even column: NE/SE go up-right/down-right, NW/SW go up-left/down-left
                    // NE (up-right)
                    if y > 0 && x < width - 1 {
                        cells[idx].neighbors[2] = Some(cell_index(x + 1, y - 1, width));
                    }
                    // SE (down-right)
                    if x < width - 1 {
                        cells[idx].neighbors[3] = Some(cell_index(x + 1, y, width));
                    }
                    // NW (up-left)
                    if y > 0 && x > 0 {
                        cells[idx].neighbors[4] = Some(cell_index(x - 1, y - 1, width));
                    }
                    // SW (down-left)
                    if x > 0 {
                        cells[idx].neighbors[5] = Some(cell_index(x - 1, y, width));
                    }
                }
            }
        }
    }

    fn to_svg(maze: &GenericMaze<Self>, tunnel_width: usize, solution_path: Option<&[usize]>, debug: bool) -> String {
        let hex_width = tunnel_width;
        let hex_height = (tunnel_width as f64 * 0.866).round() as usize;
        let svg_width = maze.width * hex_width * 3 / 4 + hex_width / 4 + 10;
        let svg_height = maze.height * hex_height + hex_height / 2 + 10;

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
  <rect width="{}" height="{}" fill="white"/>
  <g stroke="black" stroke-width="2" stroke-linecap="square" fill="none">
"#,
            svg_width, svg_height, svg_width, svg_height, svg_width, svg_height
        ));

        let hex_center = |x: usize, y: usize| -> (usize, usize) {
            let cx = x * hex_width * 3 / 4 + hex_width / 2;
            let cy = y * hex_height + if x % 2 == 1 { hex_height / 2 } else { 0 } + hex_height / 2;
            (cx, cy)
        };

        // Draw hexagons and walls
        for y in 0..maze.height {
            for x in 0..maze.width {
                let idx = maze.cell_index(x, y);
                let (cx, cy) = hex_center(x, y);

                let w = hex_width / 2;
                let h = hex_height / 2;

                let points = [
                    (cx - w/2, cy - h),     // 0: top-left (NW corner)
                    (cx + w/2, cy - h),     // 1: top-right (NE corner)
                    (cx + w, cy),           // 2: right (E corner)
                    (cx + w/2, cy + h),     // 3: bottom-right (SE corner)
                    (cx - w/2, cy + h),     // 4: bottom-left (SW corner)
                    (cx - w, cy),           // 5: left (W corner)
                ];

                // Draw each edge if wall exists (matching neighbor indices: N, S, NE, SE, NW, SW)
                let edges = [
                    (0, 1), // 0: N edge (top)
                    (4, 3), // 1: S edge (bottom)
                    (1, 2), // 2: NE edge
                    (2, 3), // 3: SE edge
                    (5, 0), // 4: NW edge
                    (4, 5), // 5: SW edge
                ];

                for (wall_idx, &(p1, p2)) in edges.iter().enumerate() {
                    if maze.cells[idx].walls[wall_idx] {
                        // Skip entrance (NW edge of cell 0) and exit (SE edge of last cell)
                        let is_entrance = idx == 0 && wall_idx == 4; // NW edge of cell 0
                        let is_exit = idx == maze.cells.len() - 1 && wall_idx == 3; // SE edge of last cell

                        if !is_entrance && !is_exit {
                            svg.push_str(&format!("    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n",
                                points[p1].0, points[p1].1, points[p2].0, points[p2].1));
                        }
                    }
                }
            }
        }

        svg.push_str("  </g>\n");

        // Add cell index labels for debugging
        if debug {
            svg.push_str("  <g font-family=\"monospace\" font-size=\"12\" text-anchor=\"middle\" fill=\"blue\">\n");
            for y in 0..maze.height {
                for x in 0..maze.width {
                    let idx = maze.cell_index(x, y);
                    let (cx, cy) = hex_center(x, y);
                    svg.push_str(&format!("    <text x=\"{}\" y=\"{}\">{}</text>\n", cx, cy + 4, idx));
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
                    let (cx, cy) = hex_center(x, y);

                    if i == 0 {
                        svg.push_str(&format!("M {} {} ", cx, cy));
                    } else {
                        svg.push_str(&format!("L {} {} ", cx, cy));
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
        println!("\n=== Hexagonal Maze Debug Info ===");
        println!("Grid: {}x{} (width x height)", maze.width, maze.height);
        println!("Total cells: {}", maze.cells.len());
        println!("\nNeighbor relationships (indices: N, S, NE, SE, NW, SW):");

        for y in 0..maze.height {
            for x in 0..maze.width {
                let idx = maze.cell_index(x, y);
                let is_odd_col = x % 2 == 1;
                print!("Cell {:2} (x={}, y={}, {}): [",
                    idx, x, y, if is_odd_col { "odd " } else { "even" });

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
