//= USES ===========================================================================================

use irid::{ApplicationBuilder, Listener};
use irid_assets::TextCoordsVertex;

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

    let shader_paths = vec!["lw09_model_loading/assets/shader.wgsl"];

    let texture_path = "lw09_model_loading/assets/happy-tree.png";

    #[rustfmt::skip]
    let vertices = &[irid::assets::ModelVertex] = &[];

    #[rustfmt::skip]
    let indices = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4_u16,
    ];

    let application = ApplicationBuilder::new(listener)
        .with_clear_color_rgb(0.1, 0.2, 0.3)
        .with_shader_paths(shader_paths)
        .with_texture_path(texture_path)
        .with_vertices(vertices)
        .with_indices(indices)
        .build();
    let _ = application.start();
}
