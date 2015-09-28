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
