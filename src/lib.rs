mod encoding_utils;

use std::fs::File;
use std::path::Path;

use anyhow::{bail, Result};
use regex::{self, Regex};
use srtlib::Subtitles;

pub const DEFAULT_PATTERN: &str = r"\[.*?\]";

pub fn deccify_file(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    pattern: Option<String>,
) -> Result<()> {
    let pattern = Regex::new(pattern.unwrap_or_else(|| DEFAULT_PATTERN.into()).as_str())?;
    let extension = input.as_ref().extension().and_then(|ostr| ostr.to_str());
    match extension {
        Some("srt") => deccify_srt_file(input, output, &pattern)?,
        _ => bail!("Unrecognized file type!"),
    }
    Ok(())
}

fn deccify_srt_file(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    pattern: &Regex,
) -> Result<()> {
    let subtitles = Subtitles::parse_from_str(encoding_utils::read_file_with_encoding(
        &mut File::open(input)?,
        None,
        false,
    )?)?;
    let subtitles = remove_pattern_from_subs(subtitles, pattern);
    subtitles.write_to_file(output, None)?;
    Ok(())
}

fn remove_pattern_from_subs(subs: Subtitles, pattern: &Regex) -> Subtitles {
    let mut new_subs = Subtitles::new();
    for mut sub in subs {
        sub.text = pattern.replace_all(&sub.text, "").to_string();
        if !sub.text.is_empty() {
            new_subs.push(sub)
        }
    }
    new_subs
}

#[cfg(test)]
mod test {
    use super::*;
    use srtlib::{Subtitle, Timestamp};
    use tempfile::NamedTempFile;

    fn craft_subtitles(lines: &'static str) -> Subtitles {
        let lines = lines.to_string();
        let lines = lines.split("\n");
        let mut subs = Subtitles::new();
        for line in lines {
            subs.push(Subtitle::new(
                0,
                Timestamp::new(0, 0, 0, 0),
                Timestamp::new(0, 0, 0, 0),
                line.into(),
            ));
        }
        subs
    }

    fn compare_srt_files(file1: impl AsRef<Path>, file2: impl AsRef<Path>) -> bool {
        let subs1 = Subtitles::parse_from_file(file1, None).unwrap();
        let subs2 = Subtitles::parse_from_file(file2, None).unwrap();
        subs1 == subs2
    }

    #[test]
    fn test_removing_pattern() {
        let re = Regex::new(DEFAULT_PATTERN).unwrap();
        let test_vectors: Vec<(Subtitles, Subtitles)> = vec![
            ("Hello[unpleasant screeching] World!", "Hello World!"),
            ("I [don't]like [apple]pie", "I like pie"),
            (
                "Hello[fudge] world\nHow[pie] are[poop] you?",
                "Hello world\nHow are you?",
            ),
            (
                "[Well well well]\nWhat have we here[SANDY CLAWS?]?",
                "What have we here?",
            ),
        ]
        .into_iter()
        .map(|(a, b)| (craft_subtitles(a), craft_subtitles(b)))
        .collect();

        for (raw, expected) in test_vectors.into_iter() {
            let actual = remove_pattern_from_subs(raw, &re);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn fix_up_srt_file() {
        let input = "assets/asset1.srt";
        let output = NamedTempFile::new().unwrap();

        deccify_file(input, output.path(), None).unwrap();
        assert!(compare_srt_files(output.path(), "assets/asset1_fixed.srt"));
    }
}
