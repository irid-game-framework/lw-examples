//= USES ===========================================================================================

use std::collections::HashMap;
use std::fs::read_to_string;

use wgpu::Color;

use irid::{ApplicationBuilder, Listener};

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

    let listener = GameListener { };

    const SHADER_WGSL_FILENAME: &str = "shader.wgsl";
    const SHADER_WGSL_FILEPATH: &str = "D:/_BLACK_ABYSS_DUNGEON/_BAD/shaded_sun/lw_examples/lw03_pipeline/assets";

    // TODO give path or filename only
    let mut shaders = HashMap::new();
    let frag_wgsl = match read_to_string(SHADER_WGSL_FILEPATH) {
        Ok(file) => file.clone(),  // TODO try to remove the clone
        Err(why) => panic!("couldn't open {} file: {}", SHADER_WGSL_FILENAME, why),
    };
    shaders.insert(SHADER_WGSL_FILENAME.to_string(), frag_wgsl);

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
