use super::*;
use chrono::prelude::*;

pub fn result_and_option() {}

mod definitions {
    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    pub enum Option<T> {
        Some(T),
        None,
    }
}

fn get_from_predefined_map(key: &String) -> Option<String> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("Schuld und Sühne".to_string(), "Dostojewski".to_string());
    map.insert("Die Verwandlung".to_string(), "Kafka".to_string());
    map.insert("Der Schneesturm".to_string(), "Sorokin".to_string());

    match map.get(key) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

#[cfg(test)]
#[test]
fn test_map_showcase_unwrap() {
    let item = get_from_predefined_map(&"Die Verwandlung".to_string());
    let mut author_match = "Unknown".to_string();

    // Geht nicht. Wir bekommen ein Option zurück, kein String.
    // Man den Fall, dass kein Element gefunden wurde handhaben.
    //   author = item;

    // Hole das Objekt aus der Some Variante, ansonsten beende
    // das Programm.
    author_match = match item {
        Some(a) => a,
        None => panic!("Tried to assign None"),
    };

    // äquivalent: unwrap

    let item = get_from_predefined_map(&"Die Verwandlung".to_string());
    let mut author_unwrap = "Unknown".to_string();

    author_unwrap = item.unwrap();
}

#[cfg(test)]
#[test]
fn test_map_showcase_unwrap_or() {
    let item = get_from_predefined_map(&"Die Verwandlung".to_string());
    let mut author_match = "Unknown".to_string();

    // Hole das Objekt aus der Some Variante, ansonsten befülle
    // die Variable mit einem default Wert.
    author_match = match item {
        Some(a) => a,
        None => "Unbekannt".to_string(),
    };

    // äquivalent: unwrap_or

    let item = get_from_predefined_map(&"Die Verwandlung".to_string());
    let mut author_unwrap = "Unknown".to_string();

    author_unwrap = item.unwrap_or("Unbekannt".to_string());

    assert_eq!(author_match, author_unwrap);
}

pub fn try_operator() -> Option<String> {
    let item = get_from_predefined_map(&"Die Verwandlung".to_string());
    let mut author_match = "Unknown".to_string();

    // Hole das Objekt aus der Some Variante, ansonsten gib None
    // an unseren Aufrufer zurück.
    author_match = match item {
        Some(a) => a,
        None => return None,
    };

    // äquivalent: Der try-operator

    let item = get_from_predefined_map(&"Die Verwandlung".to_string());
    let mut author_try = "Unknown".to_string();

    author_try = item?; // todo: this looks like the assignment is only performed if 'item' is filled ...

    if author_try.len() > 12 {
        Some("Der Author hat einen langen Namen".to_string())
    } else {
        Some("Der Author hat einen kurzen Namen".to_string())
    }
}

fn get_content_length(filename: &impl ToString) -> std::io::Result<usize> {
    // try-operator (=?) kann gechained werden
    use std::fs::File;
    use std::io::Read;
    let mut content = String::new();
    let result = File::open(filename.to_string())?.read_to_string(&mut content);
    match result {
        Ok(_) => Ok(content.len()), // todo: it's kind of redundant now
        Err(e) => Err(e),
    }
}

#[cfg(test)]
#[test]
fn assert_results() {
    let result: std::result::Result<usize, &str> = Ok(42);
    assert_eq!(result, Ok(42));

    let result: std::result::Result<usize, &str> = Err("fail");
    assert_eq!(result, Err("fail"));

    let result: std::result::Result<usize, std::io::Error> = Ok(42);
    // assert_eq!(result, Ok(42));  // todo: this does not work, I assume that 'Error' does not implement some kind of equals
    assert_eq!(result.map_err(|e| "foo"), Ok(42));
}

#[cfg(test)]
#[test]
fn test_get_content_length() {
    let ok = get_content_length(&"filename.txt");
    assert_eq!(ok.unwrap(), 1);

    // assert!(matches!(get_content_length(&"filename.txt"), Ok(1))); // todo: this works but cannot report helpful error if assert fails
    let ok = get_content_length(&"filename.txt").map_err(|e| e.kind()); // Error is mapped
    assert_eq!(ok, Ok(1));

    let fail = get_content_length(&"does-not-exist.txt").map_err(|e| e.kind());
    assert_eq!(fail, Err(std::io::ErrorKind::NotFound));
}

fn next_slide() {
    traits();
}
