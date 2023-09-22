mod file_rep;
mod parser;

fn main() {
    let sosi_text = std::fs::read_to_string("tests/sample_datasets/wikipedia_sample.sos").unwrap();
    let geojson = parser::parse_sosi_to_geojson(sosi_text).unwrap();
    println!("{}", geojson);
}
