use std::mem::transmute;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

pub const ALL_RANKS: [Rank; 8] = [
    Rank::One,
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
];

impl Rank {
    pub fn from_str(s: &str) -> Option<Rank> {
        match s {
            "1" => Some(Rank::One),
            "2" => Some(Rank::Two),
            "3" => Some(Rank::Three),
            "4" => Some(Rank::Four),
            "5" => Some(Rank::Five),
            "6" => Some(Rank::Six),
            "7" => Some(Rank::Seven),
            "8" => Some(Rank::Eight),
            _ => None,
        }
    }

    pub fn from_index(i: usize) -> Rank {
        unsafe { transmute((i as u8) & 7) }
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn to_int(&self) -> i8 {
        ALL_RANKS.iter().position(|&r| r == *self).unwrap() as i8
    }

    pub fn to_str(&self) -> &'static str {
        match *self {
            Rank::One => "1",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
        }
    }
}
