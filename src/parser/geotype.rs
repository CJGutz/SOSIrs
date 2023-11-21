use geojson::{
    Geometry,
    Value::{LineString, MultiPoint, Point, Polygon},
};

pub fn match_geometry(sosi_type: &str, coords: Vec<(f64, f64)>) -> Geometry {
    assert!(coords.len() > 0, "Coordinates must be greater than 0");

    let coordinates = coords.iter().map(|(x, y)| vec![*x, *y]).collect();

    let value = match sosi_type {
        "KURVE" => LineString(coordinates),
        "FLATE" => Polygon(vec![coordinates]),
        "PUNKT" => {
            let (x, y) = coords.first().unwrap();
            Point(vec![*x, *y])
        }
        "SVERM" => MultiPoint(coordinates),
        _ => panic!("Unknown geotype"),
    };
    return Geometry::new(value);
}
