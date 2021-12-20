use std::{fs::File, io};

use lib::{board::Board, square::Square};
use minifb::{Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, Image, Path, PathBuilder, SolidSource, Source};

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

struct SimpleImage {
    data: Vec<u32>,
    width: i32,
    height: i32,
}

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
// Probably by using bit arithmetics, but I don't understand how
fn draw_gameboard(width: i32, height: i32) -> DrawTarget {
    let mut dt = DrawTarget::new(width, height);
    let pawn = load_image("pawn.png").unwrap();
    let square_size = width / 8;
    let color_black = SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff);
    let color_white = SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0);
    let board = Board::default();

    for (index, piece) in board.squares.iter().enumerate() {
        let square = Square(index as u8);
        let (x, y) = square.coordinate();

        let (square_color, _text_color) = if (x + y) % 2 == 0 {
            (color_black, color_white)
        } else {
            (color_white, color_black)
        };

        let sq = draw_square(square_size, x as i32, y as i32);
        dt.fill(&sq, &Source::Solid(square_color), &DrawOptions::new());
        dt.draw_image_at(
            x as f32,
            y as f32,
            &Image {
                data: &pawn.data,
                width: pawn.width,
                height: pawn.height,
            },
            &DrawOptions::new(),
        )
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

fn load_image(path: &str) -> io::Result<SimpleImage> {
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info()?;
    let mut img_data = vec![0; reader.output_buffer_size()];

    let info = reader.next_frame(&mut img_data).unwrap();

    // Convert u8 to u32
    let mut data = Vec::with_capacity(img_data.len());
    for pixel in img_data.chunks(4) {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        let a = pixel[3];

        let pixel = (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32;
        data.push(pixel);
    }

    Ok(SimpleImage {
        width: info.width as i32,
        height: info.height as i32,
        data,
    })
}
