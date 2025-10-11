use asr::{emulator::{ps2::Emulator}, future::next_tick};

asr::async_main!(stable);

// custom logic
mod mc3;
use mc3::Game;

async fn main() {
    loop {
        let process = Emulator::wait_attach().await;
        process
            .until_closes(async {
                // the game is initialized here
                let mut gaem = Game::new(&process);
                loop {
                    // the game is updated here
                    gaem.update();
                    next_tick().await;
                }
            })
            .await;
    }
}