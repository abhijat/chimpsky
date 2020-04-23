use serde_json::{Map, Value};

#[derive(Debug)]
pub enum FieldKind {
    Str,
    Int,
    Float,
    Bool,
    Object,
    Null,
    OneOf(Vec<FieldKind>),
    Reference(String),
    ListOf(Vec<FieldKind>),
}

impl FieldKind {
    pub fn new(v: &Value, parent_node: &Map<String, Value>) -> Self {
        match v {
            Value::String(s) if s == "array" => Self::parse_array_definition(parent_node),
            Value::String(s) => FieldKind::match_value(&s),
            Value::Array(v) => {
                let field_names = v.iter()
                    .map(|v| v.as_str().unwrap())
                    .map(FieldKind::match_value)
                    .collect();
                FieldKind::OneOf(field_names)
            }
            _ => panic!("unexpected field-kind")
        }
    }

    fn match_value(s: &str) -> FieldKind {
        match s {
            "string" => FieldKind::Str,
            "integer" => FieldKind::Int,
            "number" => FieldKind::Float,
            "boolean" => FieldKind::Bool,
            "object" => FieldKind::Object,
            "null" => FieldKind::Null,
            _ => panic!(format!("unknown type {}", s))
        }
    }

    fn parse_array_definition(node: &Map<String, Value>) -> FieldKind {
        let items_node = node["items"].as_object().unwrap();
        let mut kinds = vec![];
        for (k, v) in items_node {
            let v = v.as_str().unwrap();
            if k == "type" {
                kinds.push(Self::match_value(v));
            }

            if k == "$ref" {
                kinds.push(FieldKind::Reference(v.to_owned()));
            }
        }

        FieldKind::ListOf(kinds)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn string_def() {
        let t = json!("string");
        let v = json!({"type": t});
        let k = FieldKind::new(&t, v.as_object().unwrap());
        assert!(matches!(k, FieldKind::Str));
    }

    #[test]
    fn bool_def() {
        let t = json!("boolean");
        let v = json!({"type": t});
        let k = FieldKind::new(&t, v.as_object().unwrap());
        assert!(matches!(k, FieldKind::Bool));
    }

    #[test]
    fn list_def() {
        let t = json!("array");
        let v = json!({
            "type": t,
            "items": {
                "type": "integer",
                "$ref": "foobar"
            }
        });
        let k = FieldKind::new(&t, v.as_object().unwrap());

        match k {
            FieldKind::ListOf(kinds) => assert_eq!(kinds.len(), 2),
            _ => panic!()
        }
    }
}