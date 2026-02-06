use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::maze::{Maze, Cell};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solver {
    open_set: BinaryHeap<State>,
    came_from: HashMap<(usize, usize), (usize, usize)>,
    g_score: HashMap<(usize, usize), usize>,
    
    pub current: Option<(usize, usize)>,
    pub finished: bool,
    pub path_found: bool,
    pub visited: Vec<(usize, usize)>, // For drawing blue cells
}

impl Solver {
    pub fn new(start: (usize, usize)) -> Self {
        let mut open_set = BinaryHeap::new();
        let mut g_score = HashMap::new();
        
        g_score.insert(start, 0);
        open_set.push(State { cost: 0, position: start });

        Solver {
            open_set,
            came_from: HashMap::new(),
            g_score,
            current: None,
            finished: false,
            path_found: false,
            visited: Vec::new(),
        }
    }

    // Returns true if step was taken, false if finished
    pub fn step(&mut self, maze: &Maze, use_heuristic: bool) -> bool {
        if self.open_set.is_empty() {
            self.finished = true;
            return false;
        }

        if let Some(State { cost: _, position }) = self.open_set.pop() {
            self.current = Some(position);
            self.visited.push(position);

            if position == maze.end {
                self.finished = true;
                self.path_found = true;
                return false; // Found path
            }

            let (cx, cy) = position;
            let current_g = *self.g_score.get(&position).unwrap_or(&usize::MAX);

            // Neighbors
            let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            for (dx, dy) in dirs.iter() {
                let nx = cx as i32 + dx;
                let ny = cy as i32 + dy;

                if nx >= 0 && ny >= 0 && nx < maze.width as i32 && ny < maze.height as i32 {
                    let neighbor = (nx as usize, ny as usize);
                    
                    if maze.grid[neighbor.1][neighbor.0] == Cell::Wall {
                        continue;
                    }

                    // Distance between neighbors is always 1
                    let tentative_g = current_g + 1;

                    if tentative_g < *self.g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                        self.came_from.insert(neighbor, position);
                        self.g_score.insert(neighbor, tentative_g);
                        
                        let f_score = if use_heuristic {
                            tentative_g + self.heuristic(neighbor, maze.end)
                        } else {
                            tentative_g // Dijkstra behavior
                        };

                        self.open_set.push(State {
                            cost: f_score,
                            position: neighbor,
                        });
                    }
                }
            }
            return true;
        } else {
            self.finished = true; // No path found
            return false;
        }
    }

    fn heuristic(&self, a: (usize, usize), b: (usize, usize)) -> usize {
        // Manhattan distance
        ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as usize
    }

    pub fn reconstruct_path(&self) -> Vec<(usize, usize)> {
        let mut path = Vec::new();
        if !self.path_found {
            return path;
        }

        if let Some(mut curr) = self.current {
            path.push(curr);
            while let Some(&prev) = self.came_from.get(&curr) {
                path.push(prev);
                curr = prev;
            }
        }
        path.reverse();
        path
    }
}
