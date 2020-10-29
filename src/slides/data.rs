use super::*;
use chrono::{Date, Utc};
use std::fmt;
use std::io;

#[derive(PartialEq, PartialOrd, Debug, Hash)]
pub struct Talk {
    pub speaker: Person,
    pub subject: String,
    pub title: String,
    pub text: String,
    pub date: Date<Utc>,
}

#[derive(PartialEq, PartialOrd, Debug, Hash)]
pub struct Person {
    pub name: String,
    pub occupation: String,
    pub workplace: String,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{name}, {occupation} bei {workplace}",
            name = self.name,
            occupation = self.occupation,
            workplace = self.workplace,
        )
    }
}

impl fmt::Display for Talk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Wilkommen zum {subject} TechTalk \"{title}\".",
            subject = self.subject,
            title = self.title,
        )?;
        write!(f, "Speaker: {speaker}", speaker = self.speaker,)
    }
}

// Ein eigenes Trait definieren.
// Hier: Mit Supertrait (= Typen, die Present
// implementieren wollen, müssen auch Display
// implementiert haben)
// und mit Default implementation.
pub trait Present: fmt::Display {
    fn present(&self) -> String {
        format!("{}", self)
    }
}

// Eigene Traitimplementation schreiben
impl Present for Talk {
    fn present(&self) -> String {
        format!("{}\n\n{}", self, self.text)
    }
}

// Erlaubt uns
// let talk += new_speaker;
// zu schreiben
impl std::ops::AddAssign<Person> for Talk {
    fn add_assign(&mut self, person: Person) {
        self.speaker = person;
    }
}

// Trait bounds: T muss alle angegebenen Traits implementieren
fn some_fn<U, V, T: From<U> + Into<V>>(arg: T) {}

impl Talk {
    fn ready_for_rust() -> Self {
        Self {
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
        }
    }
}

#[cfg(test)]
#[test]
fn display_impl() {
    let talk = Talk::ready_for_rust();
    let display = format!("{}", talk);
    assert_eq!(false, display.contains("Lorem"));
}

#[cfg(test)]
#[test]
fn present_impl() {
    let talk = Talk::ready_for_rust();
    let present = format!("{}", talk.present());
    assert_eq!(true, present.contains("Lorem"));
}


fn next_slide() {
    closures();
}
