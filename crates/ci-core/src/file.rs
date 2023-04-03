use core::mem::transmute;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

pub const ALL_FILES: [File; 8] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
];

impl File {
    pub fn from_str(s: &str) -> Option<File> {
        match s.to_lowercase().as_ref() {
            "a" => Some(File::A),
            "b" => Some(File::B),
            "c" => Some(File::C),
            "d" => Some(File::D),
            "e" => Some(File::E),
            "f" => Some(File::F),
            "g" => Some(File::G),
            "h" => Some(File::H),
            _ => None,
        }
    }

    pub fn from_index(i: usize) -> File {
        unsafe { transmute((i as u8) & 7) }
    }

    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn to_int(&self) -> i8 {
        ALL_FILES.iter().position(|&x| x == *self).unwrap() as i8
    }

    pub fn to_str(&self) -> &'static str {
        match *self {
            File::A => "a",
            File::B => "b",
            File::C => "c",
            File::D => "d",
            File::E => "e",
            File::F => "f",
            File::G => "g",
            File::H => "h",
        }
    }
}
