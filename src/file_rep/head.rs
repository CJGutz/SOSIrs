use geojson::JsonObject;

pub struct SOSIHead {
    charset: String,
    transpar: JsonObject,
    area: JsonObject,
    version: String,
}

impl SOSIHead {
    pub fn new() -> Self {
        SOSIHead {
            charset: String::new(),
            transpar: JsonObject::new(),
            area: JsonObject::new(),
            version: String::new(),
        }
    }
}
