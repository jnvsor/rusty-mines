use minefield::Minefield;
use termion::color;
use termion::clear;
use termion::event::{Key, Event, MouseEvent, MouseButton};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::cursor::Goto;
use std::io::{Stdout, stdout, stdin, Write};

pub struct Game {
    cursor: (u16, u16),
    field: Minefield,
    stdout: Option<MouseTerminal<RawTerminal<Stdout>>>,
}

impl Game {
    pub fn create(width: u16, height: u16, mines: u32) -> Result<Game, &'static str> {
        Ok(Game {
            cursor: (0, 0),
            field: Minefield::create(width, height, mines)?,
            stdout: None,
        })
    }

    pub fn run(&mut self) {
        let stdin = &mut stdin();

        if self.stdout.is_none() {
            self.stdout = Some(MouseTerminal::from(stdout().into_raw_mode().unwrap()));
        }

        write!(self.stdout.as_mut().unwrap(), "{}", clear::All).unwrap();

        self.render();

        // Get input
        for c in stdin.events() {
            match c.unwrap() {
                Event::Key(Key::Char('q')) => {
                    self.endrender();
                    return;
                },
                Event::Key(Key::Char(' ')) => {
                    if self.field.reveal(self.cursor.0, self.cursor.1).unwrap().is_mine() {
                        self.field.finish();
                        self.render();
                        self.endrender();
                        return;
                    }
                },
                Event::Key(Key::Char('x')) => {
                    self.field.flag(self.cursor.0, self.cursor.1).unwrap();
                },
                Event::Key(Key::Left) => {
                    if self.cursor.0 != 0 {
                        self.cursor.0 -= 1;
                    }
                },
                Event::Key(Key::Right) => {
                    if self.cursor.0 != self.field.get_width() - 1 {
                        self.cursor.0 += 1;
                    }
                },
                Event::Key(Key::Up) => {
                    if self.cursor.1 != 0 {
                        self.cursor.1 -= 1;
                    }
                },
                Event::Key(Key::Down) => {
                    if self.cursor.1 != self.field.get_height() - 1 {
                        self.cursor.1 += 1;
                    }
                },
                Event::Mouse(MouseEvent::Press(button, x, y)) => {
                    if self.field.get_square(x - 1, y - 1).is_ok() {
                        self.cursor.0 = x - 1;
                        self.cursor.1 = y - 1;

                        match button {
                            MouseButton::Left => {
                                if self.field.reveal(self.cursor.0, self.cursor.1).unwrap().is_mine() {
                                    self.field.finish();
                                    self.render();
                                    self.endrender();
                                    return;
                                }
                            },
                            MouseButton::Right => {
                                self.field.flag(self.cursor.0, self.cursor.1).unwrap();
                            },
                            _ => {},
                        }
                    }
                },
                _ => {}
            }

            self.render();

            if self.field.get_size() - self.field.get_mines() as usize - self.field.get_revealed() as usize == 0 {
                self.endrender();
                return;
            }
        }
    }

    fn render(&mut self) {
        let out = self.stdout.as_mut().unwrap();

        for y in 0..self.field.get_height() {
            for x in 0..self.field.get_width() {
                let square = self.field.get_square(x, y).unwrap();
                let active = self.cursor == (x, y);

                write!(out, "{}{}{}", Goto(x + 1, y + 1), color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();

                if square.is_flagged() {
                    if active {
                        write!(out, "{} ", color::Bg(color::LightYellow)).unwrap();
                    } else {
                        write!(out, "{} ", color::Bg(color::Yellow)).unwrap();
                    }
                } else if !square.is_revealed() {
                    if active {
                        write!(out, "{} ", color::Bg(color::LightBlue)).unwrap();
                    } else {
                        write!(out, "{} ", color::Bg(color::Blue)).unwrap();
                    }
                } else if square.is_mine() {
                    if active {
                        write!(out, "{} ", color::Bg(color::LightRed)).unwrap();
                    } else {
                        write!(out, "{} ", color::Bg(color::Red)).unwrap();
                    }
                } else if square.get_number() == Some(0) {
                    if active {
                        write!(out, "{} ", color::Bg(color::White)).unwrap();
                    } else {
                        write!(out, "{} ", color::Bg(color::Reset)).unwrap();
                    }
                } else {
                    let num = square.get_number().unwrap().to_string();

                    if active {
                        write!(out, "{}{}{}", color::Bg(color::White), color::Fg(color::Black), num).unwrap();
                    } else {
                        write!(out, "{}{}", color::Bg(color::Reset), num).unwrap();
                    }
                }
            }
        }

        write!(
            out,
            "{}{}{}{}{}/{} safe squares revealed ({:.0}%)",
            Goto(1, self.field.get_height() + 1),
            color::Bg(color::Reset),
            color::Fg(color::Reset),
            clear::CurrentLine,
            self.field.get_revealed(),
            self.field.get_size() - self.field.get_mines() as usize,
            self.field.get_revealed() as f64 * 100f64 / (self.field.get_size() as f64 - self.field.get_mines() as f64)
        ).unwrap();

        if self.field.get_flagged() != 0 {
            write!(out, " {} tiles flagged.", self.field.get_flagged()).unwrap();
        }

        write!(out, " {} mines on board.", self.field.get_mines()).unwrap();

        out.flush().unwrap();
    }

    fn endrender(&mut self) {
        let stdout = &mut self.stdout.as_mut().unwrap();
        write!(stdout, "{}{}{}\n", color::Bg(color::Reset), color::Fg(color::Reset), Goto(1, self.field.get_height() + 1)).unwrap();
        stdout.flush().unwrap();
    }
}
