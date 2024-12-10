use serde_json::{Value, Map};

pub struct FlattenContext<'a> {
    pub array_separator: String,
    pub object_separator: String,
    pub json: &'a Value,
    pub prefix: String,
    pub flat_map: &'a mut Map<String, Value>,
}