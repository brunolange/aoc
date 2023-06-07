use serde_json::{Result, Value};

fn parse_json(json_str: &str) -> Result<Value> {
    let parsed_value: Value = serde_json::from_str(json_str)?;
    Ok(parsed_value)
}

fn add(value: Value) -> f64 {
    _add(value, 0.0)
}

fn _add(v: Value, acc: f64) -> f64 {
    match v {
        Value::Null => acc,
        Value::Bool(_) => acc,
        Value::Number(n) => acc + n.as_f64().unwrap(),
        Value::String(_) => acc,
        Value::Array(arr) => acc + arr.into_iter().map(|v| _add(v, 0.0)).sum::<f64>(),
        Value::Object(obj) => {
            let contains_red = obj.iter().any(|(_, v)| match v {
                Value::String(s) => s == "red",
                _ => false,
            });
            let v = if contains_red {
                0.0
            } else {
                obj.into_iter().map(|(_, v)| _add(v, 0.0)).sum::<f64>()
            };
            acc + v
        }
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
    }
}
