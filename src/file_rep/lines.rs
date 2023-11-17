#[derive(Debug)]
pub struct DefinitionData {
    pub indentation: u8,
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
}


pub enum LineType {
    DefinitionData(DefinitionData),
    Coordinates(Coordinates),
}