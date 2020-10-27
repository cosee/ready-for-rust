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

    author_try = item?;

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
    let _ = File::open(filename.to_string())?.read_to_string(&mut content)?;

    Ok(content.len())
}

#[cfg(test)]
#[test]
fn test_get_content_length() {
    let filename = "filename.txt";
    let result = get_content_length(&filename);
    match result {
        Ok(s) => println!("We read {} characters from the \"{}\"", s, filename),
        Err(e) => println!("The file \"{}\"could not be read!", filename),
    }
}

fn next_slide() {
    traits();
}
