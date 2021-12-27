use std::io::Stdin;
use crate::screen::Screen;

extern crate termion;

use termion::event::{Event, Key};
use termion::input::TermRead;

const BLANK_CHAR: char = '_';

pub struct Terminal {
    pub screen: Screen,
    stdin: Stdin,
}

impl Terminal {
    pub fn new(rows: usize, cols: usize) -> Self
    {
        Self {
            screen: Screen::init(rows, cols),
            stdin: std::io::stdin(),
        }
    }

    pub fn input_field(&mut self, y: u32, x: u32, length: usize) -> String
    {
        let mut input = String::new();

        for i in 0..length {
            self.screen.overlay.putc(y, x + i as u32, BLANK_CHAR);
        }

        self.screen.draw();
        self.screen.move_cursor(y, x);
        self.screen.show_cursor();
        self.screen.refresh();

        for e in self.stdin.lock().events() {
            match e.unwrap() {
                Event::Key(Key::Char('\n')) => break,
                Event::Key(Key::Char(c)) => {
                    if c.is_alphanumeric() || c.is_ascii_punctuation() || c == ' ' {
                        if input.len() < length - 1 {
                            self.screen.overlay.putc(self.screen.cursor_y, self.screen.cursor_x, c);
                            self.screen.advance_cursor(1);
                            input.push(c);
                        }
                    }
                }
                Event::Key(Key::Backspace) => {
                    if !input.is_empty() {
                        if self.screen.cursor_x != x {
                            self.screen.overlay.putc(self.screen.cursor_y, self.screen.cursor_x - 1, BLANK_CHAR);
                            self.screen.advance_cursor(-1);
                        }
                        input.pop();
                    }
                },
                _ => ()
            }
            self.screen.draw();
            self.screen.refresh();
        }

        self.screen.hide_cursor();

        input
    }
}
