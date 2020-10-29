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

fn sum_vec(vector: Vec<u8>) -> u32 {
    // Es kann mit for in über alles iteriert werden,
    // das ein bestimmtes Trait (~=Interface) implementert.
    // Mehr zu Traits bald™
    let mut sum = 0;
    for i in vector {
        sum += i;
    }
    sum as u32
}

#[cfg(test)]
#[test]
fn test_sum() {
    assert_eq!(sum(1, 4), 6);
    assert_eq!(sum(1, 6), 15);
    // Auch Methoden wie fold und map sind auf allen
    // Typen die dieses Trait implementieren verfügbar.
    assert_eq!((1..6).fold(0, |acc, v| acc + v), 15);
    let vec = vec![1, 2, 3, 4, 5];
    assert_eq!(sum_vec(vec), 15);
}

fn busy_wait(file: &str) -> String {
    // loop erstellt eine Endlosschleife, ähnlich wie while(true)
    // in manchen anderen Sprachen.
    // Wenn man eine Schleife mit break abbricht, kann diese auch
    // einen Wert zurückgeben.
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

fn next_slide() {
    structs();
}
