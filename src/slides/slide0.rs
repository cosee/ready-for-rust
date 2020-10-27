use super::*;
use chrono::{Date, Utc};

pub fn titlepage() {
    let talk = Talk {
        speaker: Person {
            name: String::from("Raik Joachim"),
            occupation: String::from("Backend engineer / Student"),
            workplace: String::from("cosee GmbH / TU-Darmstadt"),
        },
        subject: String::from("Rust"),
        title: String::from("Ready for Rust"),
        text: String::from(
            "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, \
            sed diam nonumy eirmod tempor invidunt ut labore et dolore\
            magna aliquyam erat, sed diam voluptua. At vero eos et \
            accusam et justo duo dolores et ea rebum.",
        ),
        date: Utc::today(),
    };

    println!("{}\n", talk);

    println!("{}\n", talk.present());
}

fn next_slide() {
    syntax();
}
