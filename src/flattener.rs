use serde_json::Value;
use crate::flatten_context::FlattenContext;

pub fn flatten_json(ctx: &mut FlattenContext) {
    match ctx.json {
        Value::Object(map) => {
            if map.is_empty() {
                ctx.flat_map.insert(format!("{}{}", ctx.prefix, ctx.object_separator), Value::Null);
            } else {
                for (key, value) in map {
                    let new_key = if ctx.prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}{}{}", ctx.prefix, ctx.object_separator, key)
                    };
                    let mut new_ctx = FlattenContext {
                        array_separator: ctx.array_separator.to_string(),
                        object_separator: ctx.object_separator.to_string(),
                        json: value,
                        prefix: new_key,
                        flat_map: ctx.flat_map,
                    };
                    flatten_json(&mut new_ctx);
                }
            }
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                ctx.flat_map.insert(format!("{}{}", ctx.prefix, ctx.array_separator), Value::Null);
            } else {
                for (index, value) in arr.iter().enumerate() {
                    let new_key = format!("{}{}{}", ctx.prefix, ctx.array_separator, index);
                    let mut new_ctx = FlattenContext {
                        array_separator: ctx.array_separator.to_string(),
                        object_separator: ctx.object_separator.to_string(),
                        json: value,
                        prefix: new_key,
                        flat_map: ctx.flat_map,
                    };
                    flatten_json(&mut new_ctx);
                }
            }
        }
        _ => {
            ctx.flat_map.insert(ctx.prefix.clone(), ctx.json.clone());
        }
    }
}

//////////////////
/// Unit Tests ///
//////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Map};

    #[test]
    fn test_flatten_empty_object() {
        let mut flat_map = Map::new();
        let json = json!({});
        let mut ctx = FlattenContext {
            array_separator: "_".to_string(),
            object_separator: ".".to_string(),
            json: &json,
            prefix: "".to_string(),
            flat_map: &mut flat_map,
        };

        flatten_json(&mut ctx);

        assert_eq!(flat_map.len(), 1);
        assert_eq!(flat_map.get(".").unwrap(), &Value::Null);
    }

    #[test]
    fn test_flatten_empty_array() {
        let mut flat_map = Map::new();
        let json = json!([]);
        let mut ctx = FlattenContext {
            array_separator: "_".to_string(),
            object_separator: ".".to_string(),
            json: &json,
            prefix: "".to_string(),
            flat_map: &mut flat_map,
        };

        flatten_json(&mut ctx);

        assert_eq!(flat_map.len(), 1);
        assert_eq!(flat_map.get("_").unwrap(), &Value::Null);
    }

    #[test]
    fn test_flatten_object_with_single_key() {
        let mut flat_map = Map::new();
        let json = json!({"key": "value"});
        let mut ctx = FlattenContext {
            array_separator: ".".to_string(),
            object_separator: ".".to_string(),
            json: &json,
            prefix: "".to_string(),
            flat_map: &mut flat_map,
        };

        flatten_json(&mut ctx);

        assert_eq!(flat_map.len(), 1);
        assert_eq!(flat_map.get("key").unwrap(), &json!("value"));
    }

    #[test]
    fn test_flatten_nested_object() {
        let mut flat_map = Map::new();
        let json = json!({
            "outer": {
                "inner": "value"
            }
        });
        let mut ctx = FlattenContext {
            array_separator: ".".to_string(),
            object_separator: ".".to_string(),
            json: &json,
            prefix: "".to_string(),
            flat_map: &mut flat_map,
        };

        flatten_json(&mut ctx);

        assert_eq!(flat_map.len(), 1);
        assert_eq!(flat_map.get("outer.inner").unwrap(), &json!("value"));
    }

    #[test]
    fn test_flatten_array_with_values() {
        let mut flat_map = Map::new();
        let json = json!([1, 2, 3]);
        let mut ctx = FlattenContext {
            array_separator: ".".to_string(),
            object_separator: ".".to_string(),
            json: &json,
            prefix: "".to_string(),
            flat_map: &mut flat_map,
        };

        flatten_json(&mut ctx);

        assert_eq!(flat_map.len(), 3);
        assert_eq!(flat_map.get(".0").unwrap(), &json!(1));
        assert_eq!(flat_map.get(".1").unwrap(), &json!(2));
        assert_eq!(flat_map.get(".2").unwrap(), &json!(3));
    }

    #[test]
    fn test_flatten_mixed_object_and_array() {
        let mut flat_map = Map::new();
        let json = json!({
            "array": [1, 2],
            "object": {
                "key1": "value1",
                "key2": "value2"
            }
        });
        let mut ctx = FlattenContext {
            array_separator: "_".to_string(),
            object_separator: ".".to_string(),
            json: &json,
            prefix: "".to_string(),
            flat_map: &mut flat_map,
        };

        flatten_json(&mut ctx);

        assert_eq!(flat_map.len(), 4);
        assert_eq!(flat_map.get("array_0").unwrap(), &json!(1));
        assert_eq!(flat_map.get("array_1").unwrap(), &json!(2));
        assert_eq!(flat_map.get("object.key1").unwrap(), &json!("value1"));
        assert_eq!(flat_map.get("object.key2").unwrap(), &json!("value2"));
    }

    #[test]
    fn test_flatten_object_with_null() {
        let mut flat_map = Map::new();
        let json = json!({
            "key": null
        });
        let mut ctx = FlattenContext {
            array_separator: ".".to_string(),
            object_separator: ".".to_string(),
            json: &json,
            prefix: "".to_string(),
            flat_map: &mut flat_map,
        };

        flatten_json(&mut ctx);

        assert_eq!(flat_map.len(), 1);
        assert_eq!(flat_map.get("key").unwrap(), &Value::Null);
    }

    #[test]
    fn test_flatten_array_with_null() {
        let mut flat_map = Map::new();
        let json = json!([null, 2, null]);
        let mut ctx = FlattenContext {
            array_separator: ".".to_string(),
            object_separator: ".".to_string(),
            json: &json,
            prefix: "".to_string(),
            flat_map: &mut flat_map,
        };

        flatten_json(&mut ctx);

        assert_eq!(flat_map.len(), 3);
        assert_eq!(flat_map.get(".0").unwrap(), &Value::Null);
        assert_eq!(flat_map.get(".1").unwrap(), &json!(2));
        assert_eq!(flat_map.get(".2").unwrap(), &Value::Null);
    }

    #[test]
    fn test_flatten_complex_json() {
        let mut flat_map = Map::new();
        let json = json!({
            "a": {
                "b": [1, 2, 3],
                "c": {"d": "value"}
            },
            "e": "hello"
        });
        let mut ctx = FlattenContext {
            array_separator: "_".to_string(),
            object_separator: ".".to_string(),
            json: &json,
            prefix: "".to_string(),
            flat_map: &mut flat_map,
        };

        flatten_json(&mut ctx);

        assert_eq!(flat_map.len(), 5);
        assert_eq!(flat_map.get("a.b_0").unwrap(), &json!(1));
        assert_eq!(flat_map.get("a.b_1").unwrap(), &json!(2));
        assert_eq!(flat_map.get("a.b_2").unwrap(), &json!(3));
        assert_eq!(flat_map.get("a.c.d").unwrap(), &json!("value"));
        assert_eq!(flat_map.get("e").unwrap(), &json!("hello"));
    }
}
