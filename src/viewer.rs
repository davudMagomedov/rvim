use crate::liner::Liner;

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::*;

use std::io::{stdout, Write};

#[derive(Debug)]
pub enum ViewerCommand {
    ScrollDown,
    ScrollUp,
    ScrollUpTo(usize),
    ScrollDownTo(usize),
}

#[derive(Debug)]
pub struct Viewer<'a> {
    liner: Liner<'a>,
    width: usize,
}

impl<'a> Viewer<'a> {
    pub fn new(content: &'a str, shift: usize, width: usize) -> Self {
        Self {
            liner: Liner::new(content, shift),
            width,
        }
    }

    pub fn update(&mut self, command: ViewerCommand) {
        match command {
            ViewerCommand::ScrollUp => self.liner.scroll_up(),
            ViewerCommand::ScrollDown => self.liner.scroll_down(),
            ViewerCommand::ScrollUpTo(a) => self.liner.scroll_up_to(a),
            ViewerCommand::ScrollDownTo(a) => self.liner.scroll_down_to(a),
            _ => (),
        };
    }

    pub fn draw(&self) {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))
            .expect("Output error! (clear screen and move cursor)");

        let mut extra_lines = 0;

        self.liner
            .get_current_lines()
            .into_iter()
            .map(|a| a.unwrap_or_else(|| "~"))
            .for_each(|a| {
                if a.len() > self.width {
                    extra_lines += 1
                }
            });

        let content = self
            .liner
            .get_current_lines()
            .into_iter()
            .enumerate()
            .map(|a| {
                a.1.map(|n| format!("{}\t{}", a.0 + self.liner.position, n))
                    .unwrap_or_else(|| "~".to_string())
            })
            .take(self.liner.shift - extra_lines)
            .fold("".to_string(), |l, r| l + "\n\r" + &r);

        stdout()
            .write_all(content.as_bytes())
            .expect("Output error! (write content)");
        stdout().flush();

        stdout()
            .write_all("\0".as_bytes())
            .expect("Output error! (write content)");
        stdout().flush();
    }
}
