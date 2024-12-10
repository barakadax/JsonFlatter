pub mod flatten_context;
pub mod flattener;

use serde_json::{Value, Map};
use crate::flatten_context::FlattenContext;
use crate::flattener::flatten_json;

#[tokio::main]
async fn main() {
    let json_str = r#"
    {
        "name": "Alice",
        "age": 30,
        "address": {
            "city": "Wonderland",
            "zip": 12345
        },
        "hobbies": ["reading", "chess", { "something": true }],
        "settings": {
            "theme": "dark",
            "notifications": {
                "email": true,
                "sms": false
            },
            "other-policies": [],
            "other-info": {}
        }
    }
    "#;

    let json: Value = serde_json::from_str(json_str).unwrap();
    let mut flat_map = Map::new();
    let mut ctx = FlattenContext {
        array_separator: "_".to_string(),
        object_separator: ".".to_string(),
        json: &json,
        prefix: "".to_string(),
        flat_map: &mut flat_map,
    };

    flatten_json(&mut ctx);

    //println!("array_separator: {}", ctx.array_separator);
    //println!("object_separator: {}", ctx.object_separator);
    //println!("Prefix: {}", ctx.prefix);
    //println!("JSON: {}", serde_json::to_string_pretty(ctx.json).unwrap());
    println!("Flat Map: {}", serde_json::to_string_pretty(ctx.flat_map).unwrap());
}
