use std::error::Error;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use invaders::{frame, render, player};
use frame::Drawable;
use frame::new_frame;
use rusty_audio::Audio;


fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode","explode.wav");
    audio.add("lose","lose.wav");
    audio.add("move","move.wav");
    audio.add("pew","pew.wav");
    audio.add("startup","startup.wav");
    audio.add("win","win.wav");

    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop 
        {
            let curr_frame = match render_rx.recv()
            {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game loop
    let mut player = player::Player::new();
    let mut instatnt = Instant::now();


    'gameloop: loop {
        let mut curr_frame = new_frame();
        let delta = instatnt.elapsed();
        instatnt = Instant::now();

        // Input
        while event::poll(Duration::default())?
        {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code
                {
                    KeyCode::Esc | KeyCode::Char('q') => 
                    {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    _ => break 'gameloop,
                }
            }
        }

        player.update(delta);

        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }


    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();

    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
