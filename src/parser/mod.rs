mod comments;
mod feature;
mod geotype;
mod keys;
mod parse_head;

use feature::sosi_feature_to_geojson;
use keys::parse_definition_key;

use crate::file_rep::lines::DefinitionData;
use geojson::{self, Feature};

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

    let file_definitions = head
        .iter()
        .map(|line| parse_definition_key(line))
        .collect::<Vec<Option<DefinitionData>>>();

    let geojson_features = features.iter().map(|f| sosi_feature_to_geojson(f));
    let filtered_features = geojson_features
        .filter(|f| f.is_some())
        .map(|f| f.unwrap())
        .collect::<Vec<Feature>>();

    return Ok(geojson::GeoJson::FeatureCollection(
        geojson::FeatureCollection {
            bbox: None,
            features: filtered_features,
            foreign_members: None,
        },
    ));
}
