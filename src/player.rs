use crate::{NUM_COLS, NUM_ROWS};
use crate::shot::Shot;
use crate::frame::{Frame, Drawable};
use std::time::Duration;

pub struct Player
{
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }

    pub fn shoot(&mut self) -> bool{
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta_time);
        }
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) 
    {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
}

impl Drawable for Player
{
    fn draw(&self, frame: &mut Frame)
    {
        frame[self.x][self.y] = "A";
        for shot in self.shots.iter()
        {
            shot.draw(frame);
        }
    }
}