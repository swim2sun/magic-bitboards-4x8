use super::*;

macro_rules! simple_enum {
    ($(
        pub enum $name:ident {
            $($variant:ident),*
        }
    )*) => {$(
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub const NUM: usize = [$(Self::$variant),*].len();
            pub const ALL: [Self; Self::NUM] = [$(Self::$variant),*];

            pub fn try_index(index: usize) -> Option<Self> {
                $(#[allow(non_upper_case_globals, unused)]
                const $variant: usize = $name::$variant as usize;)*
                #[allow(non_upper_case_globals)]
                match index {
                    $($variant => Option::Some(Self::$variant),)*
                    _ => Option::None
                }
            }

            pub fn index(index: usize) -> Self {
                Self::try_index(index).unwrap_or_else(|| panic!("Index {} is out of range.", index))
            }
        }
    )*};
}

simple_enum! {
    pub enum File {
        A,
        B,
        C,
        D
    }

    pub enum Rank {
        First,
        Second,
        Third,
        Fourth,
        Fifth,
        Sixth,
        Seventh,
        Eighth
    }

    pub enum Square {
        A1, B1, C1, D1,
        A2, B2, C2, D2,
        A3, B3, C3, D3,
        A4, B4, C4, D4,
        A5, B5, C5, D5,
        A6, B6, C6, D6,
        A7, B7, C7, D7,
        A8, B8, C8, D8
    }
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Self {
        Self::index(((rank as usize) << 2) | file as usize)
    }

    pub fn file(self) -> File {
        File::index(self as usize & 0b000011)
    }

    pub fn rank(self) -> Rank {
        Rank::index(self as usize >> 2)
    }

    pub fn bitboard(self) -> BitBoard {
        BitBoard(1 << self as u8)
    }

    pub fn try_offset(self, file_offset: i8, rank_offset: i8) -> Option<Square> {
        Some(Square::new(
            File::try_index((self.file() as i8 + file_offset).try_into().ok()?)?,
            Rank::try_index((self.rank() as i8 + rank_offset).try_into().ok()?)?,
        ))
    }
}
