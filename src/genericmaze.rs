use rand::Rng;
use std::collections::{HashMap, VecDeque};

/// Helper functions for converting between (x, y) coordinates and cell indices
pub fn cell_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

pub fn cell_coords(idx: usize, width: usize) -> (usize, usize) {
    (idx % width, idx / width)
}

trait SliceRandom {
    type Item;
    fn choose<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<&Self::Item>;
}

impl<T> SliceRandom for [T] {
    type Item = T;

    fn choose<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<&Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(&self[rng.gen_range(0..self.len())])
        }
    }
}

/// A cell in the maze with neighbors and walls
#[derive(Clone)]
pub struct MazeCell {
    pub neighbors: Vec<Option<usize>>,
    pub walls: Vec<bool>,
}

impl MazeCell {
    pub fn new(num_neighbors: usize) -> Self {
        MazeCell {
            neighbors: vec![None; num_neighbors],
            walls: vec![true; num_neighbors],
        }
    }
}

/// Trait defining shape-specific behavior for different maze topologies
pub trait Shape {
    /// Number of neighbors for each cell in this shape
    fn num_neighbors() -> usize;

    /// Initialize neighbor relationships for all cells
    fn init_neighbors(width: usize, height: usize, cells: &mut [MazeCell]);

    /// Render the maze as SVG
    fn to_svg(maze: &GenericMaze<Self>, tunnel_width: usize, solution_path: Option<&[usize]>, debug: bool) -> String
    where
        Self: Sized;

    /// Print debug information (optional)
    fn print_debug_info(maze: &GenericMaze<Self>)
    where
        Self: Sized,
    {
        println!("Grid: {}x{}", maze.width, maze.height);
        println!("Total cells: {}", maze.cells.len());
    }
}

/// Generic maze structure parameterized by shape
pub struct GenericMaze<S: Shape> {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<MazeCell>,
    _shape: std::marker::PhantomData<S>,
}

impl<S: Shape> GenericMaze<S> {
    /// Create a new maze with initialized neighbor relationships
    pub fn new(width: usize, height: usize) -> Self {
        let num_cells = width * height;
        let mut cells = vec![MazeCell::new(S::num_neighbors()); num_cells];
        S::init_neighbors(width, height, &mut cells);

        GenericMaze {
            width,
            height,
            cells,
            _shape: std::marker::PhantomData,
        }
    }

    /// Convert (x, y) coordinates to cell index
    pub fn cell_index(&self, x: usize, y: usize) -> usize {
        cell_index(x, y, self.width)
    }

    /// Convert cell index to (x, y) coordinates
    pub fn cell_coords(&self, idx: usize) -> (usize, usize) {
        cell_coords(idx, self.width)
    }

    /// Generate the maze using recursive backtracking
    pub fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        let mut visited = vec![false; self.cells.len()];
        let mut stack = Vec::new();

        stack.push(0);
        visited[0] = true;

        while let Some(current) = stack.last().copied() {
            let mut unvisited = Vec::new();

            for (edge_idx, &neighbor_opt) in self.cells[current].neighbors.iter().enumerate() {
                if let Some(neighbor) = neighbor_opt {
                    if !visited[neighbor] {
                        unvisited.push((neighbor, edge_idx));
                    }
                }
            }

            if unvisited.is_empty() {
                stack.pop();
            } else {
                let &(next, edge_idx) = unvisited.choose(&mut rng).unwrap();

                self.cells[current].walls[edge_idx] = false;

                // Find reverse edge
                for (rev_idx, &neighbor_opt) in self.cells[next].neighbors.iter().enumerate() {
                    if neighbor_opt == Some(current) {
                        self.cells[next].walls[rev_idx] = false;
                        break;
                    }
                }

                visited[next] = true;
                stack.push(next);
            }
        }
    }

    /// Solve the maze using BFS
    pub fn solve(&self) -> Vec<usize> {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.cells.len()];
        let mut parent: HashMap<usize, usize> = HashMap::new();

        let end = self.cells.len() - 1;

        queue.push_back(0);
        visited[0] = true;

        while let Some(current) = queue.pop_front() {
            if current == end {
                break;
            }

            for (edge_idx, &neighbor_opt) in self.cells[current].neighbors.iter().enumerate() {
                if let Some(neighbor) = neighbor_opt {
                    if !self.cells[current].walls[edge_idx] && !visited[neighbor] {
                        visited[neighbor] = true;
                        parent.insert(neighbor, current);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        let mut path = Vec::new();
        let mut current = end;
        path.push(current);

        while current != 0 {
            if let Some(&prev) = parent.get(&current) {
                path.push(prev);
                current = prev;
            } else {
                break;
            }
        }

        path.reverse();
        path
    }
}
