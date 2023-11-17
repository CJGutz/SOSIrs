mod comments;
mod keys;
mod parse_head;

use std::vec;

use keys::parse_definition_key;

use crate::file_rep::lines::DefinitionData;

use regex;

pub fn parse_sosi_to_geojson(sosi_text: String) -> Result<geojson::GeoJson, &'static str> {
    let delimiter = regex::Regex::new(r"\r\n\.[A-Å]").unwrap();
    let iter = delimiter.find_iter(&sosi_text);

    let mut features: Vec<&str> = vec![];
    let mut first_index = 0;

    for m in iter {
        let found = m.as_str();
        if let Some(i) = found.find('\n') {
            let split_index = m.start() + i;
            let result = &sosi_text[first_index..split_index];
            features.push(result);
            first_index = split_index;
        }
    }

    if features.len() == 0 {
        return Err("No features found");
    }

    let head = features.remove(0).lines().collect::<Vec<&str>>();

    let _file_definitions = head
        .iter()
        .map(|line| parse_definition_key(line))
        .collect::<Vec<Option<DefinitionData>>>();

    Ok(geojson::GeoJson::Feature(geojson::Feature {
        bbox: None,
        geometry: None,
        id: None,
        properties: None,
        foreign_members: None,
    }))
}
