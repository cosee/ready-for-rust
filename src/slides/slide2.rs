use super::*;

pub fn control_flow() {}

pub fn syntax_if() {
    let x = 8;

    // conditionals
    if x >= 255 {
        println!("x ist groß");
    } else if x < 3 {
        println!("x ist winzig");
    } else {
        println!("x ist irgendwo zwischen winzig und groß");
    }
}

fn sum(from: u8, to: u8) -> u32 {
    let mut sum = 0;
    // 🎀 Schleifen & ranges
    for i in from..to {
        sum += i;
    }
    sum as u32
}

#[cfg(test)]
#[test]
fn test_sum() {
    assert_eq!(sum(1, 4), 6);
    assert_eq!(sum(1, 6), 15);
}

fn busy_wait(file: &str) -> String {
    let ret = loop {
        use std::{fs, thread, time::Duration};
        thread::sleep(Duration::from_millis(100));

        let exit = fs::read_to_string(file);
        if exit.is_err() {
            println!("still waiting for {}", file);
            continue;
        }
        let exit = exit.unwrap();

        break exit.trim().to_string();
    };

    ret
}

#[cfg(test)]
#[test]
fn test_busy_wait() {
    let _ = std::fs::remove_file(".exitcondition");

    // asynchroner Thread legt die Datei an
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let _ = std::fs::File::create(".exitcondition");
    });

    // wartet bis die Datei da ist
    busy_wait(".exitcondition");
    println!("Done.");

    let _ = std::fs::remove_file(".exitcondition"); // cleanup
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

fn next_slide() {
    structs();
}
