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

// Helfermethode
fn get_from_predefined_map(key: String) -> Option<String> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("Schuld und Sühne".to_string(), "Dostojewski".to_string());
    map.insert("Die Verwandlung".to_string(), "Kafka".to_string());
    map.insert("Der Schneesturm".to_string(), "Sorokin".to_string());

    match map.get(&key) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

fn ok_or_abort(maybe_value: Option<String>) -> String {
    // Hole das Objekt aus der Some Variante, ansonsten beende
    // das Programm.
    let ret = match maybe_value {
        Some(a) => a,
        None => panic!("Tried to assign None"),
    };

    ret
}

#[cfg(test)]
#[test]
fn test_ok_or_abort() {
    assert_eq!(ok_or_abort(Some("Hello".to_string())), "Hello".to_string())
}

#[cfg(test)]
#[test]
#[should_panic]
fn test_panic() {
    ok_or_abort(None);
}

#[cfg(test)]
#[test]
fn test_showcase_unwrap() {
    let kafka = "Kafka".to_string();
    let item = get_from_predefined_map("Die Verwandlung".to_string());
    assert_eq!(item, Some(kafka.clone()));
    assert_eq!(ok_or_abort(item.clone()), kafka.clone());
    assert_eq!(item.unwrap(), kafka.clone());   // äquivalent: unwrap

    let item = get_from_predefined_map("Twilight".to_string());
    assert_eq!(item, None);
}

fn ok_or_default(maybe_value: Option<String>, default: String) -> String {
    // Hole das Objekt aus der Some Variante, ansonsten befülle
    // die Variable mit einem default Wert.
    let ret = match maybe_value {
        Some(s) => s,
        None => default,
    };

    ret
}

#[cfg(test)]
#[test]
fn test_ok_or_default() {
    assert_eq!(ok_or_default(Some("Hello".to_string()), "Default".to_string()), "Hello".to_string());
    assert_eq!(ok_or_default(None, "Default".to_string()), "Default".to_string());
}

#[cfg(test)]
#[test]
fn test_showcase_unwrap_or() {
    let unknown = "Unknown".to_string();

    let item = get_from_predefined_map("Die Verwandlung".to_string());
    assert_eq!(item, Some("Kafka".to_string()));
    assert_eq!(ok_or_default(item.clone(), unknown.clone()), "Kafka".to_string());
    assert_eq!(item.unwrap_or(unknown.clone()), "Kafka".to_string());   // äquivalent: unwrap_or

    let item = get_from_predefined_map("Twilight".to_string());
    assert_eq!(item, None);
    assert_eq!(ok_or_default(item.clone(), unknown.clone()), unknown.clone());
}

pub fn try_operator() -> Option<String> {
    let item = get_from_predefined_map("Die Verwandlung".to_string());
    let mut author_match = "Unknown".to_string();

    // Hole das Objekt aus der Some Variante, ansonsten gib None
    // an unseren Aufrufer zurück (early out).
    author_match = match item {
        Some(a) => a,
        None => return None,
    }
        // Wenn wir hier ankommen haben wir garantiert einen String.
        .to_uppercase();

    // äquivalent: Der try-operator

    let item = get_from_predefined_map("Die Verwandlung".to_string());
    let mut author_try = "Unknown".to_string();

    author_try = item?.to_uppercase();

    assert_eq!(author_match, author_try);

    if author_try.len() > 12 {
        Some("Der Author hat einen langen Namen".to_string())
    } else {
        Some("Der Author hat einen kurzen Namen".to_string())
    }
}

fn get_first_word_from_file(filename: &impl ToString) -> std::io::Result<String> {
    // try-operator (=?) kann gechained werden
    use std::fs::File;
    use std::io::Read;
    let mut content = String::new();

    // Versuche die Datei zu öffenen, wenn erfolgreich versuche ihren
    // Inhalt in content zu lesen. read_to_string gibt die Anzahl an
    // gelesenen Bytes zurück. Diese verwerfen wir hier (_).
    // Sollte open oder read_to_string fehlschlagen, würde diese Funktion
    // an der Stelle early returnen, und den Fehler zurückgeben.
    let _ = File::open(filename.to_string())?.read_to_string(&mut content)?;
    let mut words: Vec<String> = content
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect();
    let first_word = words.remove(0);

    Ok(first_word)
}

#[cfg(test)]
#[test]
fn test_get_content_length() {
    let filename = "filename.txt";
    let result = get_first_word_from_file(&filename);
    // get_first_word_from_file könnte einen Fehler zurückgegeben haben.
    // Darauf überprüfen wir hier.
    match result {
        Ok(s) => {
            println!("We read the file \"{}\". It says \"{}\"", filename, s);
            assert_eq!("a", s);
        }
        Err(e) => println!("The file \"{}\"could not be read!", filename),
    }
}

fn next_slide() {
    traits();
}
