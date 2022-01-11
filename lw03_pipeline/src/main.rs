//= USES ===========================================================================================

use wgpu::Color;

use irid::{Application, ApplicationBuilder, ColorVertex, Listener};

//= GAME LOGIC =====================================================================================

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

    let clear_color = Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };

    let shader_paths = vec!["lw03_pipeline/assets/shader.wgsl"];

    let application: Application<'_, _, _, _, &str, ColorVertex, u16> =
        ApplicationBuilder::new(listener)
            .with_clear_color(clear_color)
            .with_shader_paths(shader_paths)
            .build();
    let _ = application.start();
}
