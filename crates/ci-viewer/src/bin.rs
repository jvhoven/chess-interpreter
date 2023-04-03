use core::fmt;
use std::{collections::HashMap, str::FromStr};

use ci_core::{
    board::{Board, Color},
    piece::Piece,
    square::Square,
};
use eframe::egui;
use egui::{ComboBox, Id, Rect};
use egui_extras::RetainedImage;
use lazy_static::lazy_static;

#[derive(Hash, PartialEq, Eq)]
enum ChessPiece {
    WhiteQueen,
    WhiteKing,
    WhiteBishop,
    WhiteKnight,
    WhiteRook,
    WhitePawn,
    BlackQueen,
    BlackKing,
    BlackBishop,
    BlackKnight,
    BlackRook,
    BlackPawn,
}

impl FromStr for ChessPiece {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "King, White" => Ok(ChessPiece::WhiteKing),
            "Rook, White" => Ok(ChessPiece::WhiteRook),
            "Bishop, White" => Ok(ChessPiece::WhiteBishop),
            "Queen, White" => Ok(ChessPiece::WhiteQueen),
            "Knight, White" => Ok(ChessPiece::WhiteKnight),
            "Pawn, White" => Ok(ChessPiece::WhitePawn),
            "King, Black" => Ok(ChessPiece::BlackKing),
            "Rook, Black" => Ok(ChessPiece::BlackRook),
            "Bishop, Black" => Ok(ChessPiece::BlackBishop),
            "Queen, Black" => Ok(ChessPiece::BlackQueen),
            "Knight, Black" => Ok(ChessPiece::BlackKnight),
            "Pawn, Black" => Ok(ChessPiece::BlackPawn),
            _ => Err(String::from("Unknown piece")),
        }
    }
}

struct Image {
    image: RetainedImage,
}

impl Image {
    pub fn new(name: impl Into<String>, buffer: &[u8]) -> Self {
        Self {
            image: egui_extras::RetainedImage::from_svg_bytes_with_size(
                name,
                buffer,
                egui_extras::image::FitTo::Original,
            )
            .expect("Could not find image"),
        }
    }

    fn draw(&self, ui: &mut egui::Ui, context: &egui::Context, rect: Rect) {
        egui::Image::new(self.image.texture_id(context), self.image.size_vec2()).paint_at(ui, rect);
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 500.0)),
        ..Default::default()
    };
    eframe::run_native(
        "svg example",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    chessboard: Chessboard,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            chessboard: Chessboard::new(),
        }
    }
}

lazy_static! {
    static ref CHESS_PIECES: HashMap<ChessPiece, Image> = HashMap::from([
        (
            ChessPiece::WhiteQueen,
            Image::new(
                "white queen",
                include_bytes!("./assets/pieces/white_queen.svg"),
            ),
        ),
        (
            ChessPiece::BlackQueen,
            Image::new(
                "black queen",
                include_bytes!("./assets/pieces/black_queen.svg"),
            ),
        ),
        (
            ChessPiece::WhiteKing,
            Image::new(
                "white king",
                include_bytes!("./assets/pieces/white_king.svg"),
            ),
        ),
        (
            ChessPiece::BlackKing,
            Image::new(
                "black king",
                include_bytes!("./assets/pieces/black_king.svg"),
            ),
        ),
        (
            ChessPiece::WhiteBishop,
            Image::new(
                "white bishop",
                include_bytes!("./assets/pieces/white_bishop.svg"),
            ),
        ),
        (
            ChessPiece::BlackBishop,
            Image::new(
                "black bishop",
                include_bytes!("./assets/pieces/black_bishop.svg"),
            ),
        ),
        (
            ChessPiece::WhiteKnight,
            Image::new(
                "white knight",
                include_bytes!("./assets/pieces/white_knight.svg"),
            ),
        ),
        (
            ChessPiece::BlackKnight,
            Image::new(
                "black knight",
                include_bytes!("./assets/pieces/black_knight.svg"),
            ),
        ),
        (
            ChessPiece::WhiteRook,
            Image::new(
                "white rook",
                include_bytes!("./assets/pieces/white_rook.svg"),
            ),
        ),
        (
            ChessPiece::BlackRook,
            Image::new(
                "black rook",
                include_bytes!("./assets/pieces/black_rook.svg"),
            ),
        ),
        (
            ChessPiece::WhitePawn,
            Image::new(
                "white pawn",
                include_bytes!("./assets/pieces/white_pawn.svg"),
            ),
        ),
        (
            ChessPiece::BlackPawn,
            Image::new(
                "black pawn",
                include_bytes!("./assets/pieces/black_pawn.svg"),
            ),
        ),
    ]);
}

#[derive(PartialEq)]
enum Perspective {
    White,
    Black,
}

impl fmt::Display for Perspective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self == &Perspective::Black {
            f.write_str("black")?;
        } else {
            f.write_str("white")?;
        }

        Ok(())
    }
}

struct Chessboard {
    pub drawn: bool,
    state: Board,
    perspective: Perspective,
}

impl Chessboard {
    pub fn new() -> Self {
        Self {
            drawn: false,
            state: Board::default(),
            perspective: Perspective::White,
        }
    }
}

static SQUARE_SIZE: f32 = 50.0;

struct ChessboardSquare<'a> {
    piece: &'a Option<(Piece, Color)>,
    coords: (i8, i8),
    bounds: Rect,
}

impl<'a> ChessboardSquare<'a> {
    pub fn new(index: u8, piece: &'a Option<(Piece, Color)>) -> Self {
        let (x, y) = Square(index).coordinate();

        Self {
            piece,
            coords: (x, y),
            bounds: Rect {
                min: egui::Pos2::new(x as f32 * SQUARE_SIZE, y as f32 * SQUARE_SIZE),
                max: egui::Pos2::new(
                    (x as f32 + 1.0) * SQUARE_SIZE,
                    (y as f32 + 1.0) * SQUARE_SIZE,
                ),
            },
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        let (x, y) = self.coords;
        let bg_color = if (x + y) % 2 == 0 {
            egui::Color32::WHITE
        } else {
            egui::Color32::BROWN
        };

        let painter = ui.painter_at(self.bounds);
        ui.painter()
            .rect_filled(painter.clip_rect(), egui::Rounding::default(), bg_color);

        // Optionally draw a chess piece if the square contains it
        if let Some((piece, color)) = self.piece {
            self.with_piece(ui, piece, color);
        }
    }

    fn with_piece(&self, ui: &mut egui::Ui, piece: &Piece, color: &Color) {
        let piece_color = format!("{:?}, {:?}", piece, color);
        if let Ok(cp) = ChessPiece::from_str(piece_color.as_str()) {
            let img = CHESS_PIECES
                .get(&cp)
                .expect(format!("Could not find image for {}", piece_color).as_str());

            // TODO: the chess pieces don't fit nicely in their squares
            img.draw(ui, &ui.ctx().to_owned(), self.bounds);
        }
    }
}

impl Chessboard {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.set_max_size(egui::Vec2::new(500.0, 500.0));

        for (index, piece) in self.state.squares.iter().enumerate() {
            let idx = if self.perspective == Perspective::White {
                63 - index
            } else {
                index
            };

            let square = ChessboardSquare::new(idx as u8, piece);

            square.draw(ui);
            self.drawn = true;
        }
    }

    pub fn move_piece(&mut self) {
        let from = Square::new(ci_core::rank::Rank::Two, ci_core::file::File::A);
        let to = Square::new(ci_core::rank::Rank::Three, ci_core::file::File::A);

        let mut current_square: &Option<(Piece, Color)> = self
            .state
            .squares
            .get(from.to_index())
            .expect("Square should exist");

        let mut to_square = self
            .state
            .squares
            .get(to.to_index())
            .expect("Square should exist");

        let binding = current_square.to_owned();
        to_square = &binding;
        current_square = &None;

        self.state.squares[from.to_index()] = *current_square;
        self.state.squares[to.to_index()] = *to_square;
    }

    pub fn reset(&mut self) {
        self.state = Board::default();
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.chessboard.ui(ui);
        });

        egui::SidePanel::new(egui::panel::Side::Right, Id::new("something"))
            .max_width(200.0)
            .resizable(false)
            .show_separator_line(true)
            .show(ctx, |ui| {
                ui.label("Perspective");
                ComboBox::from_label("")
                    .wrap(true)
                    .selected_text(format!("{:?}", self.chessboard.perspective.to_string()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.chessboard.perspective,
                            Perspective::White,
                            "White",
                        );
                        ui.selectable_value(
                            &mut self.chessboard.perspective,
                            Perspective::Black,
                            "Black",
                        );
                    });

                if (ui.button("Play move")).clicked() {
                    self.chessboard.move_piece();
                }

                if (ui.button("Reset")).clicked() {
                    self.chessboard.reset();
                }
            });
    }
}

pub fn coordinates_to_rec_min_max(x: f32, y: f32, size: f32) -> egui::Rect {
    Rect {
        min: egui::Pos2::new(x * size, y * size),
        max: egui::Pos2::new((x + 1.0) * size, (y + 1.0) * size),
    }
}
