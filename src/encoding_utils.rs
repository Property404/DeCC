// If the world were perfect, every text file would be utf-8 (or 7-bit ASCII)
use anyhow::{bail, Result};
use encoding_rs::Encoding;
use std::io::{Read, Seek};

pub fn read_file_with_encoding(
    file: &mut (impl Read + Seek),
    encoding: Option<&str>,
    force: bool,
) -> Result<String> {
    let gave_encoding = encoding.is_some();
    let encoding = encoding
        .and_then(|encoding| Encoding::for_label(encoding.as_bytes()))
        .unwrap_or(encoding_rs::UTF_8);

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let (cow, used_encoding, malformed_sequences) = encoding.decode(buffer.as_slice());
    if !force {
        // If the USER gave the encoding, it should be the same as the one used. Otherwise they
        // made a mistake.
        // If the USER didn't give the encoding, we used the default, so whatever.
        if gave_encoding && used_encoding != encoding {
            bail!(
                "Attempted to use encoding '{}', but detected '{}'",
                encoding.name(),
                used_encoding.name()
            );
        }
        if malformed_sequences {
            bail!("Malformed sequences in '{}' data", encoding.name());
        }
    }
    Ok(cow[..].to_string())
}
