use once_cell::sync::Lazy;
use serde::Deserialize;
use std::io::Cursor;
use unicode_segmentation::{self, UnicodeSegmentation};
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct ProxiedEmojiRecord {
    expected: String,
    actual: String,
}

#[derive(Deserialize)]
struct EmojiRecord {
    emoji: String,
}

struct Entry {
    actual: String,
    ideal: String,
}

impl From<ProxiedEmojiRecord> for Entry {
    fn from(ProxiedEmojiRecord { expected, actual }: ProxiedEmojiRecord) -> Self {
        Entry {
            actual: actual,
            ideal: expected,
        }
    }
}

impl From<EmojiRecord> for Entry {
    fn from(EmojiRecord { emoji }: EmojiRecord) -> Self {
        Entry {
            actual: emoji.clone(),
            ideal: emoji,
        }
    }
}

#[wasm_bindgen]
pub fn fix(mut broken: &str) -> String {
    static EMOJI_MAP: Lazy<Vec<Entry>> = Lazy::new(emoji_map);

    let mut fixed = String::new();

    'outer: while broken.len() > 0 {
        for Entry { ideal, actual } in EMOJI_MAP.iter() {
            if broken.starts_with(actual) {
                fixed.push_str(&ideal);
                broken = &broken[actual.len()..];
                continue 'outer;
            }
        }
        let grapheme = broken.graphemes(true).next().unwrap();
        fixed.push_str(grapheme);
        broken = &broken[grapheme.len()..];
    }

    fixed
}

fn proxied_emojis() -> Vec<Entry> {
    let data = include_bytes!("../../data/proxied_emoji_clean.csv");
    let reader = Cursor::new(data);
    let mut reader = csv::Reader::from_reader(reader);
    let records: Vec<Entry> = reader
        .deserialize::<ProxiedEmojiRecord>()
        .filter_map(Result::ok)
        .map(Into::into)
        .collect();
    records
}

fn emojis() -> Vec<Entry> {
    let data = include_bytes!("../../data/emoji.csv");
    let reader = Cursor::new(data);
    let mut reader = csv::Reader::from_reader(reader);
    let records: Vec<Entry> = reader
        .deserialize::<EmojiRecord>()
        .filter_map(Result::ok)
        .map(Into::into)
        .collect();
    records
}

fn emoji_map() -> Vec<Entry> {
    let mut entries = Vec::new();
    entries.append(&mut proxied_emojis());
    entries.append(&mut emojis());
    entries.sort_by_key(|r| -(r.actual.len() as i64));
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_emojis() {
        assert_eq!("", fix(""));
        assert_eq!("sjdfejkfjiwef", fix("sjdfejkfjiwef"));
        assert_eq!("schön wűh", fix("schön wűh"));
    }

    #[test]
    fn ok_emojis() {
        // two valid forms of rainbow flag listed in the reference
        assert_eq!(
            "\u{1F3F3}\u{200D}\u{1F308}",
            fix("\u{1F3F3}\u{200D}\u{1F308}")
        );
        assert_eq!(
            "\u{1F3F3}\u{fe0f}\u{200D}\u{1F308}",
            fix("\u{1F3F3}\u{fe0f}\u{200D}\u{1F308}")
        );
    }

    #[test]
    fn broken_emojis() {
        assert_eq!(
            "\u{1F635}\u{200D}\u{1F4AB}",
            fix("\u{1F635}\u{1F4AB}")
        ); // 😵💫 -> 😵‍💫
        assert_eq!(
            "\u{1F3F3}\u{FE0F}\u{200D}\u{1F308}",
            fix("\u{1F3F3}\u{1F308}")
        ) // 🏳🌈 -> 🏳️‍🌈
    }
}
