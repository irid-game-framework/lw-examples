//= USES ===========================================================================================

use wgpu::Color;

use irid::{ApplicationBuilder, Listener};
use irid_assets::ColorVertex;

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

    let shader_paths = vec!["lw04_buffers_indices/assets/shader.wgsl"];

    // We arrange the vertices in counter clockwise order: top, bottom left, bottom right.
    // We do it this way partially out of tradition, but mostly because we specified in the
    // rasterization_state of the render_pipeline that we want the front_face of our triangle
    // to be wgpu::FrontFace::Ccw so that we cull the back face.
    #[rustfmt::skip]
    let vertices: &[ColorVertex] = &[
        ColorVertex { position: [-0.08682410,  0.49240386, 0.0], colors: [0.5, 0.0, 0.5] },
        ColorVertex { position: [-0.49513406,  0.06958647, 0.0], colors: [0.5, 0.0, 0.5] },
        ColorVertex { position: [-0.21918549, -0.44939706, 0.0], colors: [0.5, 0.0, 0.5] },
        ColorVertex { position: [ 0.35966998, -0.34732910, 0.0], colors: [0.5, 0.0, 0.5] },
        ColorVertex { position: [ 0.44147372,  0.23473590, 0.0], colors: [0.5, 0.0, 0.5] },
    ];

    #[rustfmt::skip]
    let indices: &[u16] = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
        /* padding */ 0,
    ];

    let application = ApplicationBuilder::new(listener)
        .with_clear_color(clear_color)
        .with_shader_paths(shader_paths)
        .with_vertices(vertices)
        .with_indices(indices)
        .build();
    let _ = application.start();
}
