//= USES ===========================================================================================

use wgpu::Color;

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

    let clear_color = Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };

    let listener = GameListener {};

    let shader_paths = vec!["lw05_textures_bind_groups/assets/shader.wgsl"];

    let texture_path = "lw05_textures_bind_groups/assets/happy-tree.png";

    #[rustfmt::skip]
    let vertices = &[
        TextCoordsVertex { position: [-0.08682410,  0.49240386, 0.0], tex_coords: [0.4131759000, 0.00759614], },
        TextCoordsVertex { position: [-0.49513406,  0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], },
        TextCoordsVertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.2808145300, 0.94939700], },
        TextCoordsVertex { position: [ 0.35966998, -0.34732910, 0.0], tex_coords: [0.8596700000, 0.84732914], },
        TextCoordsVertex { position: [ 0.44147372,  0.23473590, 0.0], tex_coords: [0.9414737000, 0.26526410], },
    ];

    #[rustfmt::skip]
    let indices = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4_u16,
    ];

    let application = ApplicationBuilder::new(listener)
        .with_clear_color(clear_color)
        .with_shader_paths(shader_paths)
        .with_texture_path(texture_path)
        .with_vertices(vertices)
        .with_indices(indices)
        .build();
    let _ = application.start();
}
