use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::Write;

mod genericmaze;
mod shapes;

use genericmaze::{GenericMaze, Shape};
use shapes::{RectShape, TriShape, HexShape, OctShape};

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
enum GridType {
    Rectangular,
    Triangular,
    Hexagonal,
    Octagonal,
}

#[derive(Parser)]
#[command(name = "maze")]
#[command(about = "Generate a maze in SVG format", long_about = None)]
struct Args {
    /// Width of the maze in cells
    #[arg(short = 'W', long)]
    width: usize,

    /// Height of the maze in cells
    #[arg(short = 'H', long)]
    height: usize,

    /// Output SVG file path
    #[arg(short, long)]
    output: String,

    /// Tunnel width in pixels (default: 20)
    #[arg(short, long, default_value = "20")]
    tunnel_width: usize,

    /// Grid type: rectangular, triangular, hexagonal, or octagonal (default: rectangular)
    #[arg(short, long, value_enum, default_value = "rectangular")]
    grid_type: GridType,

    /// Enable debug mode (show cell numbers and print neighbor info)
    #[arg(short, long, default_value = "false")]
    debug: bool,

    /// Render all walls (skip maze generation)
    #[arg(long, default_value = "false")]
    all_walls: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.width == 0 || args.height == 0 {
        eprintln!("Error: Width and height must be greater than 0");
        std::process::exit(1);
    }

    println!("Generating {}x{} {:?} maze...", args.width, args.height, args.grid_type);

    match args.grid_type {
        GridType::Rectangular => {
            process_maze::<RectShape>(&args)?;
        }
        GridType::Triangular => {
            process_maze::<TriShape>(&args)?;
        }
        GridType::Hexagonal => {
            process_maze::<HexShape>(&args)?;
        }
        GridType::Octagonal => {
            process_maze::<OctShape>(&args)?;
        }
    }

    Ok(())
}

fn process_maze<S: Shape>(args: &Args) -> std::io::Result<()> {
    let mut maze = GenericMaze::<S>::new(args.width, args.height);
    if args.debug {
        S::print_debug_info(&maze);
    }

    if !args.all_walls {
        maze.generate();
        let solution = maze.solve();
        let svg_content = S::to_svg(&maze, args.tunnel_width, None, args.debug);
        let svg_solution = S::to_svg(&maze, args.tunnel_width, Some(&solution), args.debug);
        write_output(&args.output, &svg_content, &svg_solution)?;
    } else {
        // Render all walls without generating maze
        let svg_content = S::to_svg(&maze, args.tunnel_width, None, args.debug);
        write_output(&args.output, &svg_content, &svg_content)?;
    }

    Ok(())
}

fn write_output(output_path: &str, svg_content: &str, svg_solution: &str) -> std::io::Result<()> {
    let mut file = File::create(output_path)?;
    file.write_all(svg_content.as_bytes())?;
    println!("Maze saved to {}", output_path);

    let solution_filename = if output_path.ends_with(".svg") {
        output_path.replace(".svg", "_solution.svg")
    } else {
        format!("{}_solution.svg", output_path)
    };

    let mut solution_file = File::create(&solution_filename)?;
    solution_file.write_all(svg_solution.as_bytes())?;
    println!("Solution saved to {}", solution_filename);

    Ok(())
}
