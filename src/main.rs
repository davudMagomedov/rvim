#![allow(dead_code)]

pub mod get_content;
pub mod liner;
pub mod numeric;
pub mod viewer;

use crossterm::cursor::{Hide, Show};
use crossterm::event::*;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use viewer::ViewerCommand;

use std::io::{stdout, Write};

fn main() {
    // stdout().write_all(&[47; 200]);
    // stdout().flush();
    //
    // std::thread::sleep(std::time::Duration::from_secs(3));

    let content = get_content::get_content().expect("Error then getting content");
    let size = size().expect("");
    let mut viewer = viewer::Viewer::new(&content, size.1 as usize, size.0 as usize);

    let _ = execute!(stdout(), Hide);

    viewer.draw();

    loop {
        let _ = enable_raw_mode();
        let event = match read().expect("Read event error!") {
            Event::Key(KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
                ..
            }) => viewer::ViewerCommand::ScrollUp,
            Event::Key(KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
                ..
            }) => viewer::ViewerCommand::ScrollDown,
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch @ ('0'..='9')),
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                let mut to = ch.to_string();
                while let Event::Key(KeyEvent {
                    code: code @ (KeyCode::Char('k' | 'j') | KeyCode::Char('0'..='9')),
                    modifiers: KeyModifiers::NONE,
                    ..
                }) = read().expect("Read event error!")
                {
                    match code {
                        KeyCode::Char(ch @ ('0'..='9')) => to.push(ch),
                        _ => break,
                    }
                }
                ViewerCommand::ScrollDown
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            }) => break,
            _ => continue,
        };
        let _ = disable_raw_mode();

        viewer.update(event);
        viewer.draw();
    }

    let _ = disable_raw_mode();
    let _ = execute!(stdout(), Show);
}
