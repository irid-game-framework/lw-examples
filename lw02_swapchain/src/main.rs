//= USES ===========================================================================================

use wgpu::Color;

use irid::{ApplicationBuilder, Listener};

//= LISTENER =======================================================================================

struct GameListener {}

impl Listener for GameListener {
    fn on_redraw(&self) -> bool {
        true
    }
}

//= MAIN ===========================================================================================

fn main() {
    env_logger::init();

    let listener = GameListener { };

    let application = ApplicationBuilder::new(listener)
        .with_clear_color(Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        })
        .build();
    let _ = application.start();
}
