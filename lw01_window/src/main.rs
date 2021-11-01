//= USES ===========================================================================================

use winit::dpi::PhysicalSize;

use irid::app::{Application, ApplicationBuilder, Listener};

//= GAME LOGIC =====================================================================================

struct GameListener {}

impl Listener for GameListener {
    fn on_suspend(&self) -> bool {
        true
    }

    fn on_resume(&self) -> bool {
        true
    }

    fn on_redraw(&self) -> bool {
        true
    }

    fn on_destroy(&self) -> bool {
        true
    }

    fn on_window_resize(&self, _new_size: PhysicalSize<u32>) -> bool {
        true
    }
}

//= MAIN ===========================================================================================

fn main() {
    env_logger::init();

    let listener = &GameListener { };

    let application = ApplicationBuilder::default().build();
    application.start(listener);
}
