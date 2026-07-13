use anyhow::Result;
use keyfarts::{AudioPlayer, keyboard_input};

fn main() -> Result<()> {
    keyboard_input(AudioPlayer::new()?)?;
    Ok(())
}
