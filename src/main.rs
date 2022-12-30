use std::fs::File;

use ev3dev_lang_rust::{Ev3Result};
use player::Player;

mod player;

fn main() -> Ev3Result<()>{
    let file = File::open("./frames.bin")?;
    let player = Player::new(file);
    player.play()?;

    Ok(())
}
