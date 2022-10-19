// If the world were perfect, every text file would be utf-8 (or 7-bit ASCII)
use anyhow::{bail, Result};
use encoding_rs::Encoding;
use std::io::{Read, Seek};

// UTF32 has the largest BOM: 4 bytes.
const MAX_BOM_SIZE: usize = 4;

// Note to self: insert joke here about BOM sniffing dogs.
fn sniff_bom(file: &mut (impl Read + Seek)) -> Result<Option<&'static Encoding>> {
    let mut bom: [u8; MAX_BOM_SIZE] = [0; MAX_BOM_SIZE];
    let bom_size = file.read(&mut bom)?;
    let encoding = Encoding::for_bom(&bom[0..bom_size]);
    file.rewind()?;
    Ok(encoding.map(|encoding| encoding.0))
}

pub fn read_file_with_encoding(
    file: &mut (impl Read + Seek),
    encoding: Option<&str>,
    force: bool,
) -> Result<String> {
    let encoding = encoding
        .and_then(|encoding| Encoding::for_label(encoding.as_bytes()))
        .map(Ok)
        .or_else(|| sniff_bom(file).transpose())
        .transpose()?
        .unwrap_or(encoding_rs::UTF_8);

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let (cow, used_encoding, malformed_sequences) = encoding.decode(buffer.as_slice());
    if !force {
        // This should only be the case if an encodeding is given in the arguments, since we detect the
        // BOM otherwise.
        if used_encoding != encoding {
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
