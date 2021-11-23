//= USES ===========================================================================================

use std::collections::HashMap;
use std::fs::read_to_string;

use wgpu::Color;
use winit::dpi::PhysicalSize;

use irid::{ApplicationBuilder, ApplicationConfig, AppConfigBuilder, Listener};

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
    log::set_max_level(log::LevelFilter::Error);
    env_logger::init();

    let config = AppConfigBuilder::new()
        .with_clear_color(Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        })
        .build();

    let listener = &GameListener { };

    const SHADER_WGSL_FILENAME: &str = "shader.wgsl";
    const SHADER_WGSL_FILEPATH: &str = "D:/_BLACK_ABYSS_DUNGEON/_BAD/shaded_sun/lw_examples/lw03_pipeline/assets";

    // TODO give path or filename only
    let mut shaders = HashMap::new();
    let frag_wgsl = match read_to_string(SHADER_WGSL_FILEPATH) {
        Ok(file) => file.clone(),  // TODO try to remove the clone
        Err(why) => panic!("couldn't open {} file: {}", SHADER_WGSL_FILENAME, why),
    };
    shaders.insert(SHADER_WGSL_FILENAME.to_string(), frag_wgsl);

    let application = ApplicationBuilder::new_with_config(config)
        .with_shaders(shaders)
        .build();
    application.start(listener);
}
