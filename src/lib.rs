//! A port of Harfbuzz to Rust.

#![deny(missing_docs)]

/// hb_tag_t
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Tag(u32);

/// HB_TAG_NONE
pub const TAG_NONE: Tag = Tag(0);

impl Tag {
    /// HB_TAG
    pub fn from_bytes(c1: u8, c2: u8, c3: u8, c4: u8) -> Tag {
        let c1 = c1 as u32;
        let c2 = c2 as u32;
        let c3 = c3 as u32;
        let c4 = c4 as u32;
        Tag((c1 << 24) | (c2 << 16) | (c3 << 8) | c4)
    }

    /// hb_tag_from_string
    pub fn from_string(s: &[u8]) -> Tag {
        if s.is_empty() {
            return TAG_NONE;
        }
        let mut tag = [b' '; 4];
        for (i, &byte) in s.iter().take(4).enumerate() {
            tag[i] = byte;
        }
        Tag::from_bytes(tag[0], tag[1], tag[2], tag[3])
    }

    /// hb_tag_to_string
    pub fn to_string(self) -> [u8; 4] {
        let tag = self.0;
        [(tag >> 24) as u8, (tag >> 16) as u8, (tag >> 8) as u8, (tag >> 0) as u8]
    }
}

/// test_types_tag
#[test]
fn types_tag() {
    assert_eq!(TAG_NONE, Tag(0));

    assert_eq!(Tag::from_bytes(b'a', b'B', b'c', b'D'),
               Tag(0x61426344));

    assert_eq!(Tag::from_string(b"aBcDe"), Tag(0x61426344));
    assert_eq!(Tag::from_string(b"aBcD"), Tag(0x61426344));
    assert_eq!(Tag::from_string(b"aBc"), Tag(0x61426320));
    assert_eq!(Tag::from_string(b"aB"), Tag(0x61422020));
    assert_eq!(Tag::from_string(b"a"), Tag(0x61202020));

    assert_eq!(Tag::from_string(b""), TAG_NONE);
}


/// hb_direction_t
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    /// HB_DIRECTION_LTR
    Ltr,
    /// HB_DIRECTION_RTL
    Rtl,
    /// HB_DIRECTION_TTB
    Ttb,
    /// HB_DIRECTION_BTT
    Btt,
}

impl Direction {
    /// hb_direction_from_string
    pub fn from_string(s: &str) -> Result<Direction, ()> {
        // Lets match loosely: just match the first letter, such that
        // all of "ltr", "left-to-right", etc work!
        match s.chars().nth(0) {
            Some('l') | Some('L') => Ok(Direction::Ltr),
            Some('r') | Some('R') => Ok(Direction::Rtl),
            Some('t') | Some('T') => Ok(Direction::Ttb),
            Some('b') | Some('B') => Ok(Direction::Btt),
            _ => Err(()),
        }
    }

    /// hb_direction_to_string
    pub fn to_string(self) -> &'static str {
        match self {
            Direction::Ltr => "ltr",
            Direction::Rtl => "rtl",
            Direction::Ttb => "ttb",
            Direction::Btt => "btt",
        }
    }

    /// HB_DIRECTION_IS_HORIZONTAL
    pub fn is_horizontal(self) -> bool {
        match self {
            Direction::Ltr | Direction::Rtl => true,
            Direction::Ttb | Direction::Btt => false,
        }
    }

    /// HB_DIRECTION_IS_VERTICAL
    pub fn is_vertical(self) -> bool {
        !self.is_horizontal()
    }

    /// HB_DIRECTION_IS_FORWARD
    pub fn is_forward(self) -> bool {
        match self {
            Direction::Ltr | Direction::Ttb => true,
            Direction::Rtl | Direction::Btt => false,
        }
    }

    /// HB_DIRECTION_IS_BACKWARD
    pub fn is_backward(self) -> bool {
        !self.is_forward()
    }

    /// HB_DIRECTION_REVERSE
    pub fn reverse(self) -> Direction {
        match self {
            Direction::Ltr => Direction::Rtl,
            Direction::Rtl => Direction::Ltr,
            Direction::Ttb => Direction::Btt,
            Direction::Btt => Direction::Ttb,
        }
    }
}

/// test_types_direction
#[test]
fn types_direction() {
    assert!(Direction::Ltr.is_horizontal());
    assert!(Direction::Rtl.is_horizontal());
    assert!(!Direction::Ttb.is_horizontal());
    assert!(!Direction::Btt.is_horizontal());

    assert!(!Direction::Ltr.is_vertical());
    assert!(!Direction::Rtl.is_vertical());
    assert!(Direction::Ttb.is_vertical());
    assert!(Direction::Btt.is_vertical());

    assert!(Direction::Ltr.is_forward());
    assert!(!Direction::Rtl.is_forward());
    assert!(Direction::Ttb.is_forward());
    assert!(!Direction::Btt.is_forward());

    assert!(!Direction::Ltr.is_backward());
    assert!(Direction::Rtl.is_backward());
    assert!(!Direction::Ttb.is_backward());
    assert!(Direction::Btt.is_backward());

    assert_eq!(Direction::Ltr.reverse(), Direction::Rtl);
    assert_eq!(Direction::Rtl.reverse(), Direction::Ltr);
    assert_eq!(Direction::Ttb.reverse(), Direction::Btt);
    assert_eq!(Direction::Btt.reverse(), Direction::Ttb);

    assert_eq!(Direction::from_string(""), Err(()));
    assert_eq!(Direction::from_string("x"), Err(()));
    assert_eq!(Direction::from_string("r"), Ok(Direction::Rtl));
    assert_eq!(Direction::from_string("rtl"), Ok(Direction::Rtl));
    assert_eq!(Direction::from_string("RtL"), Ok(Direction::Rtl));
    assert_eq!(Direction::from_string("right-to-left"),
               Ok(Direction::Rtl));
    assert_eq!(Direction::from_string("ttb"), Ok(Direction::Ttb));

    assert_eq!(Direction::Ltr.to_string(), "ltr");
    assert_eq!(Direction::Rtl.to_string(), "rtl");
    assert_eq!(Direction::Ttb.to_string(), "ttb");
    assert_eq!(Direction::Btt.to_string(), "btt");
}
