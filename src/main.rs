mod maze;
mod solver;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io, time::{Duration, Instant}};
use maze::{Maze, Cell};
use solver::Solver;

struct App {
    maze: Maze,
    solver: Option<Solver>,
    algorithm: Algorithm,
    state: AppState,
}

#[derive(PartialEq)]
enum AppState {
    Idle,
    Generating, // Visualization of generation could be added later
    Solving,
    Finished,
}

enum Algorithm {
    AStar,
    Dijkstra,
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithm::AStar => write!(f, "A* (A-Star)"),
            Algorithm::Dijkstra => write!(f, "Dijkstra"),
        }
    }
}

impl App {
    fn new() -> Self {
        // Default size based on typical terminal
        let mut maze = Maze::new(41, 21);
        maze.generate();
        
        App {
            maze,
            solver: None,
            algorithm: Algorithm::AStar,
            state: AppState::Idle,
        }
    }

    fn on_tick(&mut self) {
        if self.state == AppState::Solving {
            if let Some(solver) = &mut self.solver {
                // Perform multiple steps per frame to speed up
                for _ in 0..5 {
                    let running = solver.step(&self.maze, matches!(self.algorithm, Algorithm::AStar));
                    if !running {
                        self.state = AppState::Finished;
                        
                        // Mark path on maze for rendering
                        if solver.path_found {
                            let path = solver.reconstruct_path();
                            for (x, y) in path {
                                if self.maze.grid[y][x] != Cell::Start && self.maze.grid[y][x] != Cell::End {
                                    self.maze.grid[y][x] = Cell::PathFound;
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();
    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            ui::draw(f, &app.maze, &app.solver, &app.algorithm.to_string());
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Space => {
                        // Generate new maze
                        app.maze.generate();
                        app.solver = None;
                        app.state = AppState::Idle;
                    }
                    KeyCode::Enter => {
                        if app.state == AppState::Idle || app.state == AppState::Finished {
                            // Reset maze path visual states
                            for y in 0..app.maze.height {
                                for x in 0..app.maze.width {
                                    if app.maze.grid[y][x] == Cell::PathFound || app.maze.grid[y][x] == Cell::Visited {
                                        app.maze.grid[y][x] = Cell::Path;
                                    }
                                }
                            }
                            
                            app.solver = Some(Solver::new(app.maze.start));
                            app.state = AppState::Solving;
                        }
                    }
                    KeyCode::Tab => {
                        app.algorithm = match app.algorithm {
                            Algorithm::AStar => Algorithm::Dijkstra,
                            Algorithm::Dijkstra => Algorithm::AStar,
                        };
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
