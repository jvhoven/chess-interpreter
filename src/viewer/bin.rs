use font_kit::{family_name::FamilyName, properties::Properties, source::SystemSource};
use lib::{board::Board, square::Square};
use minifb::{Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, Path, PathBuilder, Point, SolidSource, Source};

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

// TODO: Introduce perspective, default as white
fn draw_gameboard(width: i32, height: i32) -> DrawTarget {
    let mut dt = DrawTarget::new(width, height);
    let square_size = width / 8;
    let color_black = SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff);
    let color_white = SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0);
    let board = Board::default();
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();

    for (index, piece) in board.squares.iter().enumerate() {
        let square = Square(index as u8);
        let (x, y) = square.coordinate();
        println!("X: {} Y: {}, Index: {}", x, y, index);

        let (square_color, text_color) = if (x + y) % 2 == 0 {
            (color_black, color_white)
        } else {
            (color_white, color_black)
        };

        let sq = draw_square(square_size, x as i32, y as i32);
        dt.fill(&sq, &Source::Solid(square_color), &DrawOptions::new());
        dt.draw_text(
            &font,
            11.,
            &format!("{} {}", square.rank().to_str(), square.file().to_str()),
            Point::new(
                square_size as f32 * (x + 1) as f32 - 30.0,
                square_size as f32 * (y + 1) as f32 - 22.5,
            ),
            &Source::Solid(text_color),
            &DrawOptions::new(),
        );
    }

    dt
}

fn draw_square(size: i32, x: i32, y: i32) -> Path {
    let mut pb = PathBuilder::new();

    pb.rect(
        x as f32 * size as f32,
        y as f32 * size as f32,
        size as f32,
        size as f32,
    );

    pb.finish()
}
