use super::*;

pub fn structs() {}

// Structs deklarieren
#[derive(Debug, PartialEq)]
struct Talk {
    speaker: String,
    subject: String,
    title: String,
    tags: Vec<Tag>,
}

// Implementing methods on structs
impl Talk {
    pub fn new(speaker: String, subject: String, title: String, tags: Vec<Tag>) -> Self {
        Self {
            speaker,
            subject,
            title,
            tags,
        }
    }

    pub fn lernen_in_der_cloud() -> Self {
        let speaker = String::from("Tobias Morauf");
        let subject = String::from("");
        let title = String::from("Maschinelles Lernen in der Cloud");
        let tags = vec![Tag::Cloud, Tag::MachineLearning];

        Self {
            speaker,
            subject,
            title,
            tags,
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }
}

#[cfg(test)]
#[test]
fn test_talk_constructor() {
    let talk_by_hand = Talk {
        speaker: String::from("Tobias Morauf"),
        subject: String::from(""),
        title: String::from("Maschinelles Lernen in der Cloud"),
        tags: vec![Tag::Cloud, Tag::MachineLearning],
    };

    let talk_by_constructor = Talk::new(
        String::from("Tobias Morauf"),
        String::from(""),
        String::from("Maschinelles Lernen in der Cloud"),
        vec![Tag::Cloud, Tag::MachineLearning],
    );

    assert_eq!(talk_by_hand, talk_by_constructor);
}

fn get_speaker(talk: Talk) -> String {
    // Auf Felder eines Structs zugreifen
    let speaker = talk.speaker;
    speaker
}

#[cfg(test)]
#[test]
fn test_get_speaker() {
    let talk = Talk::lernen_in_der_cloud();
    assert_eq!(get_speaker(talk), "Tobias Morauf".to_string());
}

fn get_speaker_and_subject(talk: Talk) -> (String, String) {
    // Structs können auch dekonstruiert werden
    let Talk {
        speaker, subject, ..
    } = talk;
    (speaker, subject)
}

#[cfg(test)]
#[test]
fn test_get_speaker_and_subject() {
    let talk = Talk::lernen_in_der_cloud();
    assert_eq!(
        get_speaker_and_subject(talk),
        ("Tobias Morauf".to_string(), "".to_string())
    );
}

#[cfg(test)]
#[test]
fn test_calling_functions() {
    let talk = Talk::lernen_in_der_cloud();
    let exp = &"Maschinelles Lernen in der Cloud".to_string();

    assert_eq!(talk.get_title(), exp);
    assert_eq!(Talk::get_title(&talk), exp);
}

fn next_slide() {
    enums();
}
