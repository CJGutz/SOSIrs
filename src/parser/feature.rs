use geojson::Feature;

use crate::file_rep::lines::DefinitionData;

use super::keys::parse_definition_key;

pub fn sosi_feature_to_geojson(text: &str) -> Option<Feature> {
    let found = regex::Regex::new(r"\n\d+").unwrap().find(text);
    println!("{}", found.is_none());
    dbg!(text);
    if found.is_none() {
        return None;
    }
    let coords_index = found.unwrap().start();
    let coords = &text[coords_index..];

    let properties = &text[..coords_index]
        .lines()
        .map(|t| parse_definition_key(t))
        .filter(|d| d.is_some())
        .map(|d| d.unwrap())
        .collect::<Vec<DefinitionData>>();

    dbg!(properties);
    dbg!(coords);

    return None;
}
