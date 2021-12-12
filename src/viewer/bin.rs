use minifb::{Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, Path, PathBuilder, SolidSource, Source};
use lib::game::Piece;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn main() {
    let mut window = Window::new(
        "Chess PGN Viewer",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let (width, height) = window.get_size();
    let gameboard = draw_gameboard(400, 400);

    loop {
        window
            .update_with_buffer(gameboard.get_data(), width, height)
            .unwrap();
    }
}

fn draw_gameboard(width: i32, height: i32) -> DrawTarget {
    let mut dt = DrawTarget::new(width, height);
    let square_size = width / 8;
    let color_black = SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff);
    let color_white = SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0);

    for x in 0..8 {
        for y in 0..8 {
            let src = if (x + y) % 2 == 0 {
                Source::Solid(color_black)
            } else {
                Source::Solid(color_white)
            };

            let square = draw_square(square_size, x, y, None);
            dt.fill(&square, &src, &DrawOptions::new());
        }
    }

    dt
}

fn draw_square(size: i32, x: i32, y: i32, contents: Option<Piece>) -> Path {
    let mut pb = PathBuilder::new();

    pb.rect(
        x as f32 * size as f32,
        y as f32 * size as f32,
        size as f32,
        size as f32,
    );

    pb.finish()
}
