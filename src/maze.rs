use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Wall,
    Path,
    Start,
    End,
    Visited, // For solver visualization
    PathFound, // Final path
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Cell>>,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        // Initialize full of walls
        let grid = vec![vec![Cell::Wall; width]; height];
        Maze {
            width,
            height,
            grid,
            start: (1, 1),
            end: (width - 2, height - 2),
        }
    }

    pub fn generate(&mut self) {
        // Reset grid
        self.grid = vec![vec![Cell::Wall; self.width]; self.height];
        
        let mut rng = thread_rng();
        let mut stack = Vec::new();
        
        // Start from (1,1)
        let initial = (1, 1);
        self.grid[initial.1][initial.0] = Cell::Path;
        stack.push(initial);

        while let Some((cx, cy)) = stack.last() {
            let cx = *cx;
            let cy = *cy;
            
            // Find unvisited neighbors
            // We step 2 units because walls are between paths
            let mut neighbors = Vec::new();
            
            let dirs = [(0, -2), (0, 2), (-2, 0), (2, 0)];
            for (dx, dy) in dirs.iter() {
                let nx = cx as i32 + dx;
                let ny = cy as i32 + dy;

                if nx > 0 && ny > 0 && nx < (self.width as i32 - 1) && ny < (self.height as i32 - 1) {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if self.grid[ny][nx] == Cell::Wall {
                        neighbors.push(((nx, ny), (dx / 2, dy / 2)));
                    }
                }
            }

            if !neighbors.is_empty() {
                // Choose random neighbor
                let ((nx, ny), (wx, wy)) = neighbors.choose(&mut rng).unwrap();
                
                // Carve path to neighbor
                self.grid[*ny][*nx] = Cell::Path;
                
                // Carve wall between
                let wall_x = (cx as i32 + wx) as usize;
                let wall_y = (cy as i32 + wy) as usize;
                self.grid[wall_y][wall_x] = Cell::Path;
                
                stack.push((*nx, *ny));
            } else {
                stack.pop();
            }
        }
        
        // Ensure start and end are set
        self.start = (1, 1);
        self.end = (self.width - 2, self.height - 2);
        
        // Make sure end is a path (sometimes recursive backtracker leaves it as wall if odds are weird, 
        // essentially ensure boundaries are correct)
        // With odd dimensions it works best.
        self.grid[self.start.1][self.start.0] = Cell::Start;
        self.grid[self.end.1][self.end.0] = Cell::End;
    }
}
