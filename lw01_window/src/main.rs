//= USES ===========================================================================================

use irid::{Application, ApplicationConfig, Listener};

//= LISTENER =======================================================================================

struct GameListener {}

impl Listener for GameListener {
    fn on_redraw(&self) -> bool {
        true
    }
}

//= MAIN ===========================================================================================

fn main() {
    log::set_max_level(log::LevelFilter::Debug);
    env_logger::init();

    let listener = GameListener {};

    let application: Application<'_, _, _> = ApplicationConfig::new(listener).build();

    let _ = application.start();
}
