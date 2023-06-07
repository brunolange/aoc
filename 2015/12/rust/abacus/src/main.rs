use serde_json::{Result, Value};

// TODO: iterator for numbers. then do it.sum()

fn parse_json(json_str: &str) -> Result<Value> {
    let parsed_value: Value = serde_json::from_str(json_str)?;
    Ok(parsed_value)
}

fn add(value: Value) -> f64 {
    _add(value, 0.0)
}

fn _add(v: Value, acc: f64) -> f64 {
    acc + match v {
        Value::Number(n) => n.as_f64().unwrap(),
        Value::Array(arr) => arr.into_iter().map(|v| _add(v, 0.0)).sum::<f64>(),
        Value::Object(obj) => {
            let contains_red = obj.iter().any(|(_, v)| match v {
                Value::String(s) => s == "red",
                _ => false,
            });
            match contains_red {
                true => 0.0,
                false => obj.into_iter().map(|(_, v)| _add(v, 0.0)).sum::<f64>(),
            }
        }
        _ => 0.0,
        // Value::Null => 0.0,
        // Value::Bool(_) => 0.0,
        // Value::String(_) => 0.0,
    }
}

mod io;

fn main() {
    let data = parse_json(&io::input()).expect("invalid json");
    println!("{}", add(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abacus() {
        assert_eq!(add(parse_json("null").unwrap()), 0.0);
        assert_eq!(add(parse_json("\"hello\"").unwrap()), 0.0);
        assert_eq!(add(parse_json("42").unwrap()), 42.0);
        assert_eq!(add(parse_json("3.14").unwrap()), 3.14);
        assert_eq!(add(parse_json("false").unwrap()), 0.0);
        assert_eq!(add(parse_json("[1,2,3,4]").unwrap()), 10.0);
        assert_eq!(add(parse_json(r#"{"foo": 1, "bar": 2}"#).unwrap()), 3.0);
        assert_eq!(
            add(parse_json(r#"{"foo": 1, "bar": [2,3,4,5]}"#).unwrap()),
            15.0
        );
        assert_eq!(
            add(parse_json(r#"{"foo": 1, "bar": [2,3,4,{"baz":20}]}"#).unwrap()),
            30.0
        );
        assert_eq!(
            add(parse_json(r#"{"foo": "red", "bar": [2,3,4,{"baz":20}]}"#).unwrap()),
            0.0
        );
        assert_eq!(
            add(parse_json(r#"[{"foo": "red", "v": 1},{"foo": "blue", "v": 2}]"#).unwrap()),
            2.0
        );
    }
}
