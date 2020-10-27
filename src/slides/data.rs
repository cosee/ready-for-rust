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
// Hier: Mit Supertrait & Default implementation
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

fn next_slide() {
    closures();
}
