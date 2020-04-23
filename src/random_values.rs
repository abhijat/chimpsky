use chrono::{TimeZone, Utc};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::field_kinds::FieldKind;

pub fn string() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(20).collect()
}

pub fn u64() -> u64 {
    thread_rng().gen_range(0, 10000)
}

pub fn float() -> f64 {
    thread_rng().gen_range(0.0, 1000.0)
}

pub fn boolean() -> bool {
    thread_rng().gen()
}

pub fn datetime() -> String {
    let offset = thread_rng().gen_range(0, 10000000);
    let point = Utc.timestamp(offset * 60 * 24, 0);
    point.to_rfc3339()
}

pub fn string_matching_pattern(pattern: &str) -> String {
    let p;
    let need_filter;
    if has_anchors(pattern) {
        p = remove_anchors(pattern);
        need_filter = true;
    } else {
        p = pattern.to_owned();
        need_filter = false;
    }

    let mut parser = regex_syntax::ParserBuilder::new()
        .unicode(false)
        .build();

    let hir = parser.parse(&p).unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 5).unwrap();

    if need_filter {
        let filter_regex = regex::Regex::new(pattern).unwrap();
        thread_rng()
            .sample_iter::<String, _>(&gen)
            .filter(|s| filter_regex.is_match(s))
            .next()
            .unwrap()
    } else {
        thread_rng().sample(&gen)
    }
}

pub fn element_from_collection<T>(v: &Vec<T>) -> &T {
    v.choose(&mut thread_rng()).unwrap()
}

pub fn value_of_kind(k: &FieldKind) -> Value {
    match k {
        FieldKind::Str => json!(string()),
        FieldKind::Int => json!(u64()),
        FieldKind::Float => json!(float()),
        FieldKind::Bool => json!(boolean()),
        FieldKind::Object => json!({}),
        FieldKind::Null => json!(()),
        FieldKind::OneOf(kinds) => {
            let kind = element_from_collection(kinds);
            value_of_kind(kind)
        }
        FieldKind::Reference(refpath) => json!({}),
    }
}

pub fn uuid4() -> String {
    Uuid::new_v4().to_string()
}

fn has_anchors(pattern: &str) -> bool {
    pattern.contains("^") || pattern.contains("$")
}

fn remove_anchors(pattern: &str) -> String {
    pattern.replace("^", "").replace("$", "")
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::*;

    #[test]
    fn generates_random_datetime() {
        for _ in 0..100 {
            let d = datetime();
            let dt = d.parse::<DateTime<Utc>>();
            assert!(dt.is_ok());
        }
    }

    #[test]
    fn regex_with_anchors() {
        let s = r"^[a-zA-Z0-9]+(-*[a-zA-Z0-9]+)*$";
        let expr = regex::Regex::new(s).unwrap();

        for _ in 0..100 {
            let r = string_matching_pattern(s);
            assert!(expr.is_match(&r));
        }
    }

    #[test]
    fn regex_without_anchors() {
        let s = r"[a-zA-Z0-9]+(-*[a-zA-Z0-9]+)*";
        let expr = regex::Regex::new(s).unwrap();

        for _ in 0..100 {
            let r = string_matching_pattern(s);
            assert!(expr.is_match(&r));
        }
    }

    #[test]
    fn anchor_presence() {
        assert!(has_anchors("^"));
        assert!(has_anchors("$"));
    }

    #[test]
    fn choice_from_collection() {
        let v = vec![1, 2, 3, 4, 5, 6];
        for _ in 0..100 {
            let c = element_from_collection(&v);
            assert!(*c >= 1 && *c <= 6);
        }
    }

    #[test]
    fn values_for_kind() {
        let kind = FieldKind::OneOf(vec![FieldKind::Str, FieldKind::Int, FieldKind::Object]);
        for _ in 0..100 {
            let v = value_of_kind(&kind);
            assert!(v.is_string() || v.is_number() || v.is_object());
        }
    }

    #[test]
    fn uuids() {
        println!("{}", uuid4());
    }
}