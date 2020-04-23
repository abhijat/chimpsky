use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::field_definitions::{FieldDefinition, parse_field_definitions};

#[derive(Debug)]
pub struct ObjectDefinition {
    pub name: String,
    pub kind: String,
    pub required: Option<Vec<String>>,
    pub field_definitions: Option<Vec<FieldDefinition>>,
    pub references: Option<Vec<String>>,
}

impl ObjectDefinition {
    pub fn new(name: &str, v: &Value) -> Self {
        let name = name.to_owned();
        let mut od = ObjectDefinition {
            name,
            kind: "".to_string(),
            required: None,
            field_definitions: None,
            references: None,
        };

        assert!(v.is_object());
        let v = v.as_object().unwrap();
        for (k, v) in v {
            if k == "type" {
                od.kind = Self::parse_kind(v);
            }

            if k == "required" {
                od.required = Some(Self::parse_required(v));
            }

            if k == "properties" {
                od.field_definitions = Some(parse_field_definitions(v));
            }

            if k == "allOf" {
                let (refs, fields) = Self::parse_all_of(v);
                od.field_definitions = match od.field_definitions.take() {
                    None => Some(fields),
                    Some(mut f) => {
                        f.extend(fields);
                        Some(f)
                    }
                };

                od.references = Some(refs);
            }
        }
        od
    }

    pub fn generate_json(&self, reference_map: Option<&HashMap<String, ObjectDefinition>>) -> Option<Value> {
        self.field_definitions.as_ref().map(|field_definitions| {
            let v = field_definitions.iter()
                .map(|field| field.generate_json_elements(reference_map))
                .collect::<Map<String, Value>>();
            Value::Object(v)
        })
    }

    fn parse_all_of(v: &Value) -> (Vec<String>, Vec<FieldDefinition>) {
        assert!(v.is_array(), format!("allOf {} is not an array", v));

        let mut references = vec![];
        let mut field_definitions = vec![];

        for m in v.as_array().unwrap().iter() {
            assert!(m.is_object(), format!("value in allOf {} is not an object", m));

            for (k, v) in m.as_object().unwrap() {
                if k == "$ref" {
                    references.push(v.as_str().unwrap().to_owned());
                }

                if k == "properties" {
                    let mut fds = parse_field_definitions(v);
                    field_definitions.append(&mut fds);
                }
            }
        }

        (references, field_definitions)
    }

    fn parse_required(v: &Value) -> Vec<String> {
        assert!(v.is_array(), format!("required {} is not an array", v));
        v.as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_owned())
            .collect()
    }

    fn parse_kind(v: &Value) -> String {
        assert!(v.is_string(), format!("kind {} is not a string", v));
        v.as_str().unwrap().to_owned()
    }
}

pub fn parse_definitions(v: &Value) -> HashMap<String, ObjectDefinition> {
    assert!(v.is_object());
    let v = v.as_object().unwrap();
    v.iter()
        .map(|(k, v)| (k.to_owned(), ObjectDefinition::new(k, v)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_definition() {
        let v: Value = serde_json::from_str(r#" { "basicmessageformat": {
          "type": "object",
          "properties": {
            "type": { "type": "string" },
            "timestamp": { "type": "string", "format": "date-time" },
            "metadata": { "$ref": "metadata.schema.json#/definitions/metadata" }
          },
          "required": [ "type", "timestamp", "metadata" ]
        } } "#).unwrap();
        let definitions = parse_definitions(&v);
        assert_eq!(definitions.len(), 1);
        let d = &definitions["basicmessageformat"];
        assert_eq!(d.name, "basicmessageformat");
        assert_eq!(d.kind, "object");
        assert_eq!(d.required.as_ref().unwrap().len(), 3);
        assert_eq!(d.field_definitions.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn all_of_parser() {
        let v: Value = serde_json::from_str(r#"{ "complex-message": {
      "type": "object",
      "allOf": [
        { "$ref": "basicmessageformat.schema.json#/definitions/basicmessageformat" },
        { "properties": {
            "st": { "type": [ "string", "null" ] },
            "code": { "$ref": "code.schema.json#/definitions/code" },
            "level": { "type": "string" },
            "classification": { "$ref": "classification.schema.json#/definitions/classification" },
            "data": { "type": "object" },
            "person-id": { "type": "string", "pattern": "^[a-zA-Z0-9]+(-*[a-zA-Z0-9]+)*$" }
          }
        }
      ],
      "required": [ "st", "code", "level", "classification", "data", "person-id" ]
    }
  }"#).unwrap();
        let definitions = parse_definitions(&v);
        assert_eq!(definitions.len(), 1);
        let d = &definitions["complex-message"];
        assert_eq!(d.name, "complex-message");
        assert_eq!(d.kind, "object");
        assert_eq!(d.required.as_ref().unwrap().len(), 6);
        assert_eq!(d.field_definitions.as_ref().unwrap().len(), 6);
        assert_eq!(d.references.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn parse_multiple_object_definitions() {
        let v: Value = serde_json::from_str(r#"{
            "mobile": {
              "type": "object",
              "properties": {
                "mod": { "type": [ "string", "null" ] },
                "meth": { "type": [ "string", "null" ] },
                "lno": { "type": [ "integer", "null" ] }
              }
            },
            "desktop": {
              "type": "object",
              "properties": {
                "name": { "type": "string" },
                "args": { "type": "object" },
                "file": { "type": "string" },
                "lno": { "type": "integer" },
                "trace": { "type": "string" }
              },
              "required": [ "name", "args", "file", "lno" ]
            }
          }
        "#).unwrap();
        let definitions = parse_definitions(&v);
        assert_eq!(definitions.len(), 2);
        let mobile = &definitions["mobile"];
        let desktop = &definitions["desktop"];
        assert_eq!(mobile.field_definitions.as_ref().unwrap().len(), 3);
        assert_eq!(desktop.field_definitions.as_ref().unwrap().len(), 5);
    }

    #[test]
    fn generate_json() {
        let v: Value = serde_json::from_str(r#" { "basicmessageformat": {
          "type": "object",
          "properties": {
            "timestamp": { "type": "string", "format": "date-time" },
            "userid": { "type": "string", "format": "uuid" },
            "weight": { "type": "integer" },
            "is_working": { "type": "boolean" },
            "hobbies": {"type": "array", "items": {"type": "string"}}
          },
          "required": [ "type", "timestamp", "metadata" ]
        } } "#).unwrap();
        let definition = &parse_definitions(&v)["basicmessageformat"];
        let v = definition.generate_json(None).unwrap();
        assert!(v.is_object());
        assert!(v["hobbies"].is_array());
        assert!(v["is_working"].is_boolean());
        assert!(v["weight"].is_number());
    }
}