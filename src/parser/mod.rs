mod keys;
mod parse_head;

use keys::value_for_key;

use crate::file_rep::lines::DefinitionData;

pub fn parse_sosi_to_geojson(sosi_text: String) -> Result<geojson::GeoJson, ()> {
    let lines = sosi_text.lines().collect::<Vec<&str>>();

    let value_keys = lines
        .iter()
        .map(|line| value_for_key(line))
        .collect::<Vec<Option<DefinitionData>>>();

    println!("{:?}", value_keys);

    Ok(geojson::GeoJson::Feature(geojson::Feature {
        bbox: None,
        geometry: None,
        id: None,
        properties: None,
        foreign_members: None,
    }))
}
