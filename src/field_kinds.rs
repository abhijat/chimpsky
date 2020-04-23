use serde_json::Value;

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
}

impl FieldKind {
    pub fn new(v: &Value) -> Self {
        match v {
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
}