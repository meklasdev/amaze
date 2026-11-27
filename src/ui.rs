use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::maze::{Maze, Cell};
use crate::solver::Solver;

pub fn draw(f: &mut Frame<'_>, maze: &Maze, solver: &Option<Solver>, algorithm_name: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title & Stats
            Constraint::Min(0),    // Maze
            Constraint::Length(3), // Controls
        ].as_ref())
        .split(f.size());

    // Title
    let title = Paragraph::new(format!("🧩 Amaze - Pathfinding Visualizer | Algorithm: {}", algorithm_name))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Controls
    let controls = Paragraph::new("SPACE: Generate New | ENTER: Solve | TAB: Switch Algo (A*/Dijkstra) | Q: Quit")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(controls, chunks[2]);

    // Maze Area
    let maze_area = chunks[1];
    draw_maze(f, maze_area, maze, solver);
}

fn draw_maze(f: &mut Frame<'_>, area: Rect, maze: &Maze, solver: &Option<Solver>) {
    // Calculate cell size
    // We want to fit maze.width in area.width and maze.height in area.height
    // Since characters are roughly 1x2 pixel-wise, usually 2 chars per cell width looks square-ish.
    
    // Simple render: Iterate cells
    // For large mazes, this might need viewport logic, but for now assuming it fits terminal.
    
    // We construct a buffer of styled strings? No, we can render colored blocks directly.
    // Ratatui Buffer manipulation is low-level but fast.
    
    // Let's use logic to map grid cells to terminal cells.
    // To make it look nice, we center it.
    
    let cell_w = 2; // "  " or "██"
    let total_w = maze.width as u16 * cell_w;
    let total_h = maze.height as u16;

    let start_x = area.x + (area.width.saturating_sub(total_w)) / 2;
    let start_y = area.y + (area.height.saturating_sub(total_h)) / 2;

    let buf = f.buffer_mut();

    for y in 0..maze.height {
        for x in 0..maze.width {
            let term_x = start_x + (x as u16 * cell_w);
            let term_y = start_y + y as u16;

            if term_x + cell_w > area.right() || term_y >= area.bottom() {
                continue;
            }

            let cell = maze.grid[y][x];
            let mut color = match cell {
                Cell::Wall => Color::Rgb(40, 40, 40), // Dark Grey
                Cell::Path => Color::Black,
                Cell::Start => Color::Green,
                Cell::End => Color::Red,
                _ => Color::Black,
            };
            
            let mut text = "  ";

            // If solver is active, override colors
            if let Some(s) = solver {
                // Check if this cell is part of the final path
                if s.path_found {
                     // We need to check if cell is in reconstructed path efficiently
                     // But here we are iterating.
                     // Ideally maze cell has state 'PathFound'.
                }
                
                // Visited
                // This naive contains check is slow O(N^2) for rendering, 
                // but for TUI sizes (max 100x100) it's acceptable for demo.
                // Better: Solver updates Maze state directly.
                
                // Let's rely on Maze state for efficiency if we updated it,
                // BUT solver has its own state.
                
                // Check if current head
                if Some((x,y)) == s.current {
                    color = Color::Yellow;
                    text = "oo";
                } else if s.visited.contains(&(x,y)) {
                    // Make visited cells blue gradient? Just blue for now.
                    // To avoid O(N) search, we should map this.
                    // For performance, we'll trust the visualizer speed.
                    if cell != Cell::Start && cell != Cell::End {
                        color = Color::Blue;
                    }
                }
            }
            
            // Check final path in maze grid (if we wrote it back)
            if cell == Cell::PathFound {
                 color = Color::Green;
                 text = "••";
            }

            // Draw block
            if cell == Cell::Wall {
                text = "██";
            }

            // Set style
            for i in 0..cell_w {
                if term_x + i < area.right() {
                    let cell_ref = buf.get_mut(term_x + i, term_y);
                    cell_ref.set_char(text.chars().nth(0).unwrap_or(' '));
                    cell_ref.set_bg(color);
                    cell_ref.set_fg(if cell == Cell::Wall { Color::Rgb(60,60,60) } else { Color::White });
                }
            }
        }
    }
}
