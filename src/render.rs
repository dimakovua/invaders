use std::io::Stdout;
use crate::frame::Frame;
use crossterm::QueueableCommand;
use crossterm::style::{SetBackgroundColor, SetForegroundColor, ResetColor, Color};
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::MoveTo;
use std::io::Write;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force: bool)
{
    if force 
    {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    for (x, col) in current_frame.iter().enumerate()
    {
        for (y,s) in col.iter().enumerate()
        {
            if *s != last_frame[x][y] || force
            {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", s);
            }
        }
    }
    stdout.flush().unwrap();
}