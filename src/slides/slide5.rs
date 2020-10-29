use super::*;
use chrono::prelude::*;

pub fn result_and_option() {}

mod definitions {
    // Dieses Enum ist so in std vorhanden.
    // Es, oder Variatonen davon, sind der
    // Standardweg zur Fehlerbehandlung in
    // Rust.
    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // In Rust hantiert man nicht mit null
    // oder null pointern.
    // Stattdessen wird dieses, auch so in
    // std vorhandene enum genutzt.
    pub enum Option<T> {
        Some(T),
        None,
    }
}

fn double(input: Option<usize>) -> Option<usize> {
    // Dekonstruieren & if
    if let Some(number) = input {
        return Some(number * 2);
    } else {
        return None;
    };
}

#[cfg(test)]
#[test]
fn test_double() {
    assert_eq!(double(Some(5)), Some(10));
    assert_eq!(double(None), None);

    assert_eq!(Some(5).map(|x| x * 2), Some(10));
    let no_five: Option<u32> = None;
    assert_eq!(no_five.map(|x| x * 2), None);
}

fn maybe_count(start: Option<usize>, end: Option<usize>) -> Option<String> {
    let end = end?;
    // Dekonstruieren & while
    let mut counter = start;
    while let Some(number) = counter {
        counter = Some(number + 1);
        if number > end {
            counter = None;
        }
    }

    Some(String::from("Finished!"))
}

#[cfg(test)]
#[test]
fn test_maybe_count() {
    assert_eq!(maybe_count(Some(3), None), None);
    assert_eq!(maybe_count(Some(3), Some(5)), Some("Finished!".into()));
}

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
fn showcase_unwrap() {
    assert_eq!(
        ok_or_abort(Some("Kafka".to_string())),
        Some("Kafka".to_string()).unwrap()
    );
}

#[cfg(test)]
#[test]
#[should_panic]
fn unwrap_none_panics() {
    let not_kafka: Option<String> = None;
    not_kafka.unwrap();
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
    assert_eq!(
        ok_or_default(Some("Hello".to_string()), "Default".to_string()),
        "Hello".to_string()
    );
    assert_eq!(
        ok_or_default(None, "Default".to_string()),
        "Default".to_string()
    );
}

#[cfg(test)]
#[test]
fn showcase_unwrap_or() {
    let unknown = "Unknown".to_string();

    let item = get_from_predefined_map("Die Verwandlung".to_string());
    assert_eq!(item, Some("Kafka".to_string()));
    assert_eq!(
        ok_or_default(item.clone(), unknown.clone()),
        "Kafka".to_string()
    );
    assert_eq!(item.unwrap_or(unknown.clone()), "Kafka".to_string()); // äquivalent: unwrap_or

    let item = get_from_predefined_map("Twilight".to_string());
    assert_eq!(item, None);
    assert_eq!(
        ok_or_default(item.clone(), unknown.clone()),
        unknown.clone()
    );
}

pub fn try_selfmade(name: Option<String>) -> Option<String> {
    // Hole das Objekt aus der Some Variante, ansonsten gib None
    // an unseren Aufrufer zurück (early out).
    let to_upper = match name {
        Some(a) => a,
        None => return None,
    }
    .to_uppercase(); // Wenn wir hier ankommen haben wir garantiert einen String.

    return Some(to_upper);
}

pub fn try_operator(name: Option<String>) -> Option<String> {
    let to_upper = name?.to_uppercase();
    return Some(to_upper);
}

#[cfg(test)]
#[test]
fn test_try() {
    assert_eq!(
        try_selfmade(Some("Kafka".to_string())),
        Some("KAFKA".to_string())
    );
    assert_eq!(try_selfmade(None), None);

    assert_eq!(
        try_operator(Some("Kafka".to_string())),
        Some("KAFKA".to_string())
    );
    assert_eq!(try_operator(None), None);
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
        Err(e) => assert!(false, "The file could not be read!"),
    }
}

fn next_slide() {
    traits();
}
