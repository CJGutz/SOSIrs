mod file_rep;
mod parser;
use encoding::{DecoderTrap, Encoding};
use std::fs::File;
use std::io::{self, BufReader, Read};

fn read_file(file_path: &str, encoding: &'static dyn Encoding) -> Result<String, io::Error> {
    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader to efficiently read from the file
    let mut reader = BufReader::new(file);

    // Read the entire file content into a Vec<u8>
    let mut content_bytes = Vec::new();
    reader.read_to_end(&mut content_bytes)?;

    // Decode the bytes using the specified encoding
    let content = encoding
        .decode(&content_bytes, DecoderTrap::Replace)
        .unwrap();

    Ok(content.to_string())
}

fn main() {
    let sosi_text = read_file(
        "../Dreneringslinjer_1505_Kristiansund_åpne_stikkrenner.sos",
        encoding::all::ISO_8859_1,
    )
    .unwrap();
    let geojson = parser::parse_sosi_to_geojson(sosi_text).unwrap();
    println!("{}", geojson);
}
