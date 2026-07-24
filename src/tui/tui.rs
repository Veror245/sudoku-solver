use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::Rect, style::{Color, Stylize}, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Table, Widget},
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

        Paragraph::new("Board goes here")
            .centered()
            .block(board_block)
            .render(area, buf);
        
    }
}