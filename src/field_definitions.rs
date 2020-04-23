use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use serde_json::{json, Value};

use crate::field_kinds::FieldKind;
use crate::object_definitions::ObjectDefinition;
use crate::random_values;

#[derive(Debug)]
pub struct FieldDefinition {
    pub name: String,
    pub format: Option<String>,
    pub pattern: Option<String>,
    pub kind: Option<FieldKind>,
}

impl FieldDefinition {
    pub fn new(name: &str, v: &Value) -> Self {
        let name = name.to_owned();
        let node = v.as_object().unwrap();

        let mut fd = FieldDefinition {
            name,
            format: None,
            pattern: None,
            kind: None,
        };

        for (k, v) in node {
            if k == "type" {
                fd.kind = Some(FieldKind::new(v, node));
            }

            if k == "format" {
                fd.format = Some(v.as_str().unwrap().to_owned());
            }

            if k == "$ref" {
                fd.kind = Some(FieldKind::Reference(v.as_str().unwrap().to_owned()));
            }

            if k == "pattern" {
                fd.pattern = Some(v.as_str().unwrap().to_owned());
            }
        }

        fd
    }

    pub fn generate_json_elements(&self, reference_map: &Option<HashMap<String, ObjectDefinition>>) -> (String, Value) {
        let name = self.name.to_owned();

        if let Some(format) = &self.format {
            return self.generate_by_format(format);
        }

        if let Some(pattern) = &self.pattern {
            return self.generate_by_pattern(pattern);
        }

        if let Some(FieldKind::Reference(reference)) = &self.kind {
            match reference_map {
                None => panic!(format!("cannot resolve reference {} without a reference map", reference)),
                Some(refmap) => {
                    let definition = refmap.get(reference).unwrap();
                    panic!("not supported obj-ref yet")
                }
            }
        }

        let v = match self.kind.as_ref() {
            None => json!(()),
            Some(k) => {
                random_values::value_of_kind(k)
            }
        };

        (name, v)
    }

    fn generate_by_format(&self, format: &str) -> (String, Value) {
        let name = self.name.to_owned();
        match format {
            "uuid" => (name, json!(random_values::uuid4())),
            "date-time" => (name, json!(random_values::datetime())),
            "hex-string" => (name, json!(random_values::string())),
            _ => panic!(format!("unsupported format {}", format))
        }
    }

    fn generate_by_pattern(&self, pattern: &str) -> (String, Value) {
        (self.name.to_owned(), json!(random_values::string_matching_pattern(pattern)))
    }

    fn generate_by_reference(&self) -> (String, Value) {
        (self.name.to_owned(), json!({}))
    }
}

pub fn parse_field_definitions(v: &Value) -> Vec<FieldDefinition> {
    assert!(v.is_object());
    let v = v.as_object().unwrap();
    v.iter().map(|(k, v)| FieldDefinition::new(k, v)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_properties_object() {
        let v: Value = serde_json::from_str(r#"
        {
            "nullable-string": { "type": [ "string", "null" ] },
            "data": { "type": "object" },
            "some-id": { "type": "integer" },
            "some-ref": { "$ref": "another.schema.json#/definitions/something" },
            "pattern-id": { "type": "string", "pattern": "^[a-zA-Z0-9]+(-*[a-zA-Z0-9]+)*$" }
         }
        "#).unwrap();
        let fds = parse_field_definitions(&v);
        assert_eq!(fds.len(), 5);
    }

    #[test]
    fn pattern_is_parsed() {
        let v: Value = serde_json::from_str(r#"
        {
            "pattern-id": { "type": "string", "pattern": "^[a-zA-Z0-9]+(-*[a-zA-Z0-9]+)*$" }
         }
        "#).unwrap();
        let fds = parse_field_definitions(&v);
        assert_eq!(fds.len(), 1);
        assert_eq!(fds[0].pattern.as_ref().unwrap(), "^[a-zA-Z0-9]+(-*[a-zA-Z0-9]+)*$");
    }

    #[test]
    fn reference_is_parsed() {
        let v: Value = serde_json::from_str(r#"
        {
            "some-ref": { "$ref": "another.schema.json#/definitions/something" }
         }
        "#).unwrap();
        let fds = parse_field_definitions(&v);
        assert_eq!(fds.len(), 1);

        let kind = &fds[0].kind.as_ref().unwrap();
        match kind {
            FieldKind::Reference(reference) => {
                assert_eq!(reference, "another.schema.json#/definitions/something");
            }
            _ => panic!()
        }
    }

    #[test]
    fn string_kind_parsing() {
        let v: Value = serde_json::from_str(r#" { "some-id": { "type": "integer" } } "#).unwrap();
        let fds = parse_field_definitions(&v);
        assert_eq!(fds.len(), 1);

        match &fds[0].kind.as_ref().unwrap() {
            FieldKind::Int => {}
            _ => panic!()
        }
    }

    #[test]
    fn list_kind_parsing() {
        let v: Value = serde_json::from_str(r#" { "nullable-string": { "type": [ "string", "null" ] } } "#).unwrap();
        let fds = parse_field_definitions(&v);
        match &fds[0].kind.as_ref().unwrap() {
            FieldKind::OneOf(v) => {
                assert_eq!(v.len(), 2);
            }
            _ => panic!()
        }
    }
}