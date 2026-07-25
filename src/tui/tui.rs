use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::Rect, macros::ratatui_core, style::{Color, Stylize}, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Table, Widget},
};

use crate::better_solver::Solver;

#[derive(Debug, Default)]
pub struct App {
    sudoku: Solver,
    exit: bool,
}

// pub fn run() -> color_eyre::Result<()> {
//     color_eyre::install()?;
//     ratatui::run(App)?;
//     Ok(())
// }

// pub fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
//     loop {
//         terminal.draw(render)?;
//         if crossterm::event::read()?.is_key_press() {
//             break Ok(());
//         }
//     }
// }

// fn render(frame: &mut Frame) {
//     frame.render_widget("hello world", frame.area());
// }

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame|self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            if key.code == KeyCode::Char('q') {
                self.exit = true;
            }
        }
    }

    Ok(())
    }

    pub fn draw_board(buf: &mut Buffer, grid_x: u16, grid_y: u16, board: &[u8; 81]) {
        for row in 0..9 {
            for col in 0..9 {
                let idx = row * 9 + col;

                let ch = if board[idx] == 0 {
                    '.'
                } else {
                    char::from_digit(board[idx] as u32, 10).unwrap()
                };

                let x = grid_x + 2 + col as u16 * 2 + (col / 3) as u16;
                let y = grid_y + 1 + row as u16 + (row / 3) as u16;

                buf[(x, y)].set_char(ch);

                // buf[(x + col as u16 * 2, y + row as u16)]
                //     .set_char(ch);
            }
        }
    }

    pub fn draw_grid(area: Rect, buf: &mut Buffer) -> (u16, u16) {
        const GRID: [&str; 13] = [
            "┏━━━━━━━┯━━━━━━━┯━━━━━━━┓",
            "┃       │       │       ┃",
            "┃       │       │       ┃",
            "┃       │       │       ┃",
            "┠───────┼───────┼───────┨",
            "┃       │       │       ┃",
            "┃       │       │       ┃",
            "┃       │       │       ┃",
            "┠───────┼───────┼───────┨",
            "┃       │       │       ┃",
            "┃       │       │       ┃",
            "┃       │       │       ┃",
            "┗━━━━━━━┷━━━━━━━┷━━━━━━━┛",
        ];

        let grid_width = GRID[0].chars().count() as u16;
        let grid_height = GRID.len() as u16;

        let start_x = area.x + (area.width.saturating_sub(grid_width)) / 2;
        let start_y = area.y + (area.height.saturating_sub(grid_height)) / 2;

        for (dy, line) in GRID.iter().enumerate() {
            for (dx, ch) in line.chars().enumerate() {
                buf[(start_x + dx as u16, start_y + dy as u16)]
                    .set_char(ch);
            }
        }

        (start_x, start_y)
    }
    
}


impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let title = Line::from("Sudoku");
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        Paragraph::new("Hello")
            .centered()
            .block(block)
            .render(area, buf);

        let board_block = Block::bordered()
            .title("Sudoku Board")
            .border_set(border::HEAVY_DOUBLE_DASHED)
            .bg(Color::Red);

        let board = [
            1,0,0,0,0,7,0,9,0,
            0,3,0,0,2,0,0,0,8,
            0,0,9,6,0,0,5,0,0,
            0,0,5,3,0,0,9,0,0,
            0,1,0,0,8,0,0,0,2,
            6,0,0,0,0,4,0,0,0,
            3,0,0,0,0,0,0,1,0,
            0,4,0,0,0,0,0,0,7,
            0,0,7,0,0,0,3,0,0,
        ];

        

        let (grid_x, grid_y) = App::draw_grid(area, buf);
        App::draw_board(buf, grid_x, grid_y, &board);
    }
}