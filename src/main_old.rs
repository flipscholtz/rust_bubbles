extern crate termion;
use termion::{clear, color, style, cursor};
use std::io::{self, Read, Write};
use std::thread;
use std::time;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use std::process;
use std::env;

fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    
  //  let stdin = stdin.lock();

    write!(stdout, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1)).unwrap();
    println!("");
    loop {
        let stdin = io::stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('a') => {
                 blank(&mut stdout);
                 x -= 1;
                },
                Key::Char('d') => {
                 blank(&mut stdout);
                 x += 1;
                }
                Key::Char('w') => {
                 blank(&mut stdout);
                 y -= 1;
                }
                Key::Char('s') => {
                 blank(&mut stdout);
                 y += 1;
                }
                _ => ()
             }
        }
       
        write!(stdout, "{}", cursor::Goto(x, y));
        write!(stdout, "a");
        stdout.flush().unwrap();

    }
}

fn blank(stdout: &mut RawTerminal<std::io::Stdout>) {
    write!(stdout, " ");
}
