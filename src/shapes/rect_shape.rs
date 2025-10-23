use crate::genericmaze::{cell_index, GenericMaze, MazeCell, Shape};

/// Rectangular grid shape (4 neighbors: N, S, E, W)
pub struct RectShape;

impl Shape for RectShape {
    fn num_neighbors() -> usize {
        4
    }

    fn init_neighbors(width: usize, height: usize, cells: &mut [MazeCell]) {
        // Neighbor indices: 0=N, 1=S, 2=E, 3=W
        for y in 0..height {
            for x in 0..width {
                let idx = cell_index(x, y, width);

                // North
                if y > 0 {
                    cells[idx].neighbors[0] = Some(cell_index(x, y - 1, width));
                }

                // South
                if y < height - 1 {
                    cells[idx].neighbors[1] = Some(cell_index(x, y + 1, width));
                }

                // East
                if x < width - 1 {
                    cells[idx].neighbors[2] = Some(cell_index(x + 1, y, width));
                }

                // West
                if x > 0 {
                    cells[idx].neighbors[3] = Some(cell_index(x - 1, y, width));
                }
            }
        }
    }

    fn to_svg(maze: &GenericMaze<Self>, tunnel_width: usize, solution_path: Option<&[usize]>, debug: bool) -> String {
        let wall_thickness = 2;
        let cell_size = tunnel_width + wall_thickness;
        let svg_width = maze.width * cell_size + wall_thickness;
        let svg_height = maze.height * cell_size + wall_thickness;

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
  <rect width="{}" height="{}" fill="white"/>
  <g stroke="black" stroke-width="{}" stroke-linecap="square">
"#,
            svg_width, svg_height, svg_width, svg_height, svg_width, svg_height, wall_thickness
        ));

        for y in 0..maze.height {
            for x in 0..maze.width {
                let idx = maze.cell_index(x, y);
                let cell_x = x * cell_size + wall_thickness;
                let cell_y = y * cell_size + wall_thickness;

                // North wall (index 0)
                if maze.cells[idx].walls[0] {
                    if idx != 0 {  // Entrance is north wall of cell 0
                        svg.push_str(&format!(
                            "    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n",
                            cell_x,
                            cell_y,
                            cell_x + cell_size,
                            cell_y
                        ));
                    }
                }

                // South wall (index 1)
                if maze.cells[idx].walls[1] {
                    if idx != maze.cells.len() - 1 {  // Exit is south wall of last cell
                        svg.push_str(&format!(
                            "    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n",
                            cell_x,
                            cell_y + cell_size,
                            cell_x + cell_size,
                            cell_y + cell_size
                        ));
                    }
                }

                // East wall (index 2)
                if maze.cells[idx].walls[2] {
                    svg.push_str(&format!(
                        "    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n",
                        cell_x + cell_size,
                        cell_y,
                        cell_x + cell_size,
                        cell_y + cell_size
                    ));
                }

                // West wall (index 3)
                if maze.cells[idx].walls[3] {
                    if idx != 0 {  // Entrance is west wall of cell 0
                        svg.push_str(&format!(
                            "    <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n",
                            cell_x,
                            cell_y,
                            cell_x,
                            cell_y + cell_size
                        ));
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
                    let center_x = x * cell_size + wall_thickness + cell_size / 2;
                    let center_y = y * cell_size + wall_thickness + cell_size / 2;
                    svg.push_str(&format!("    <text x=\"{}\" y=\"{}\">{}</text>\n", center_x, center_y + 4, idx));
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
                    let center_x = x * cell_size + wall_thickness + cell_size / 2;
                    let center_y = y * cell_size + wall_thickness + cell_size / 2;

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
        println!("\n=== Rectangular Maze Debug Info ===");
        println!("Grid: {}x{} (width x height)", maze.width, maze.height);
        println!("Total cells: {}", maze.cells.len());
        println!("\nNeighbor relationships (indices: 0=N, 1=S, 2=E, 3=W):");

        for y in 0..maze.height {
            for x in 0..maze.width {
                let idx = maze.cell_index(x, y);
                print!("Cell {:2} (x={}, y={}): [", idx, x, y);

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
