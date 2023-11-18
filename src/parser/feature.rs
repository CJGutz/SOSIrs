use std::vec;

use geojson::Feature;

use crate::file_rep::lines::DefinitionData;
use crate::parser::geotype::match_geometry;

use super::keys::parse_definition_key;

pub fn str_to_coords(text: &str) -> Result<(f64, f64), ()> {
    let mut split = text.split(" ");
    let x = split.next();
    let y = split.next();

    if x.is_none() || y.is_none() {
        return Err(());
    }

    let x = x.unwrap().parse::<f64>();
    let y = y.unwrap().parse::<f64>();

    if x.is_err() || y.is_err() {
        return Err(());
    }

    return Ok((x.unwrap(), y.unwrap()));
}

pub fn sosi_feature_to_geojson(text: &str) -> Option<Feature> {
    let found = regex::Regex::new(r"\n\d+").unwrap().find(text);
    if found.is_none() {
        return None;
    }
    let coords_index = found.unwrap().start();
    let coords = text[coords_index..]
        .trim()
        .lines()
        .map(|l| str_to_coords(l))
        .filter(|c| c.is_ok())
        .map(|c| c.unwrap())
        .collect::<Vec<(f64, f64)>>();

    let properties = text[..coords_index]
        .lines()
        .map(|t| parse_definition_key(t))
        .filter(|d| d.is_some())
        .map(|d| d.unwrap())
        .collect::<Vec<DefinitionData>>();

    let geotype = properties.first();
    if geotype.is_none() {
        return None;
    }

    let geometry = match_geometry(geotype.unwrap().key.as_str(), coords);

    Some(Feature {
        bbox: None,
        geometry: Some(geometry),
        id: None,
        properties: None,
        foreign_members: None,
    })
}
