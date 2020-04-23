use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde_json::{json, Map, Value};

use crate::object_definitions::{ObjectDefinition, parse_definitions};

#[derive(Debug)]
pub struct Schema {
    pub definitions: HashMap<String, ObjectDefinition>,
    pub all_of: Option<Vec<String>>,
}

impl Schema {
    pub fn new(v: &Value) -> Self {
        assert!(v.is_object(), format!("schema {} not an object", v));
        let v = v.as_object().unwrap();

        if !v.contains_key("definitions") {
            Self::parse_schema_with_single_definition(v)
        } else {
            Self::parse_schema_with_embedded_definitions(v)
        }
    }

    fn parse_schema_with_single_definition(v: &Map<String, Value>) -> Self {
        let temp = json!({"temp": v});
        let definition = parse_definitions(&temp);
        Schema { definitions: definition, all_of: None }
    }

    fn parse_schema_with_embedded_definitions(v: &Map<String, Value>) -> Self {
        let mut definitions = None;
        let mut all_of = None;

        for (k, v) in v {
            if k == "definitions" {
                definitions = Some(parse_definitions(v));
            }

            if k == "allOf" {
                all_of = Some(Self::parse_references_in_AllOf_field(v));
            }
        }

        Schema { definitions: definitions.unwrap(), all_of }
    }

    fn parse_references_in_AllOf_field(v: &Value) -> Vec<String> {
        assert!(v.is_array());
        let v = v.as_array().unwrap();
        v.iter()
            .map(|v| {
                assert!(v.is_object());
                let v = v.as_object().unwrap();
                v.iter()
                    .filter(|(k, _)| *k == "$ref")
                    .map(|(_, v)| v.as_str().unwrap().to_owned())
                    .collect()
            })
            .collect()
    }

    pub fn from_file(filepath: &Path) -> Option<Self> {
        fs::read_to_string(filepath)
            .ok()
            .and_then(|data|
                serde_json::from_str::<Value>(&data)
                    .ok()
                    .map(|v| Self::new(&v)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_schema() {
        let v: Value = serde_json::from_str(r###" { "definitions": {
            "basemessage": {
              "type": "object",
              "properties": {
                "type": { "type": "string" },
                "timestamp": { "type": "string", "format": "date-time" },
                "metadata": { "$ref": "metadata.schema.json#/definitions/metadata" }
              },
              "required": [ "type", "timestamp", "metadata" ]
            }
          },
          "allOf": [ { "$ref": "#/definitions/basemessage" } ]
          }"###).unwrap();
        let schema = Schema::new(&v);
        assert_eq!(schema.definitions.len(), 1);
        assert_eq!(schema.all_of.as_ref().unwrap().len(), 1);
        assert_eq!(schema.all_of.as_ref().unwrap()[0], "#/definitions/basemessage");
        let def = &schema.definitions["basemessage"];
        assert_eq!(def.field_definitions.as_ref().unwrap().len(), 3);
        assert_eq!(def.kind, "object");
    }

    #[test]
    fn parse_single_schema() {
        let v: Value = serde_json::from_str(r###" { "type": "object",
              "allOf": [
                { "$ref": "basemessage.schema.json#/definitions/basemessage" },
                { "properties": {
                    "subtype": { "type": [ "string", "null" ] },
                    "name": { "type": "string" },
                    "action": { "type": "string" },
                    "data": { "type": "object" },
                    "more_data": { "type": "object" },
                    "o_id": { "type": "integer" },
                    "person_id": { "type": "string", "pattern": "^[a-zA-Z0-9]+(-*[a-zA-Z0-9]+)*$" } } }
              ],
              "required": [ "subtype", "name", "action", "more_data", "o_id", "person_id" ] } "###)
            .unwrap();
        let schema = Schema::new(&v);
        assert_eq!(schema.definitions.len(), 1);
        let def = &schema.definitions["temp"];
        assert_eq!(def.kind, "object");
        assert_eq!(def.field_definitions.as_ref().unwrap().len(), 7);
    }
}

