use super::*;

pub fn enums() {}

#[derive(Debug, PartialEq)]
pub enum Tag {
    Backend,
    Frontend,
    Ops,
    Scrum,
    LanguageIntroduction,
    Cloud,
    MachineLearning,
}

// You can implement functions on Enums!
impl Tag {
    pub fn backend() -> Tag {
        Tag::Backend
    }

    pub fn string(&self) -> String {
        match self {
            Tag::Backend => String::from("Backend"),
            Tag::Frontend => String::from("Frontend"),
            _ => String::from("unknown"),
        }
    }
}

#[cfg(test)]
#[test]
fn test_stringification() {
    assert_eq!(Tag::Backend.string(), "Backend".to_string());
    assert_eq!(Tag::backend().string(), "Backend".to_string());
}

#[derive(Debug, PartialEq)]
enum CloudProvider {
    AWS,
    Azure,
    OTC,
}

impl CloudProvider {
    fn string(&self) -> String {
        match self {
            Self::AWS => "AWS".to_string(),
            Self::Azure => "Azure".to_string(),
            Self::OTC => "OTC".to_string(),
        }
    }
}


#[derive(Debug, PartialEq)]
enum TagWithAssociatedData {
    Backend { language: String, framework: String },
    Frontend { language: String, framework: String },
    Ops,
    Scrum,
    LanguageIntroduction { language: String },
    Cloud(CloudProvider),
    MachineLearning,
}

impl TagWithAssociatedData {
    pub fn java_spring_backend() -> Self {
        TagWithAssociatedData::Backend {
            language: "Java".to_string(),
            framework: "Spring".to_string(),
        }
    }
}

#[cfg(test)]
#[test]
fn test_tag_constructor_java_spring() {
    let constructor_generated = TagWithAssociatedData::java_spring_backend();

    let hand_generated = TagWithAssociatedData::Backend {
        language: String::from("Java"),
        framework: String::from("Spring"),
    };

    assert_eq!(constructor_generated, hand_generated);
}

impl TagWithAssociatedData {
    pub fn string(&self) -> String {
        // Hier wird auf self gematched, und es
        // dabei destrukturiert. Wie bei anderem
        // Destrukturieren können Felder auch an
        // Variablen gebunden werden.
        // Diese sind dann im Body des match Armes
        // gültig.
        use TagWithAssociatedData::*;
        match self {
            Backend {
                language: _,
                framework: f,
            } => format!("Dies ist ein Talk über {}", f),
            Frontend {
                language: _,
                framework: f,
            } => format!("Dies ist ein Talk über {}", f),
            Ops => format!("Dies ist ein Ops Talk."),
            LanguageIntroduction { language: lang } => {
                format!("Wir stellen {} vor", lang)
            }
            Cloud(provider) => {
                format!("Dies ist ein Talk über {}", provider.string())
            }
            _ => format!("unknown"),
        }
    }
}

#[cfg(test)]
#[test]
fn test_stringify_with_data() {
    let variant_with_data = TagWithAssociatedData::java_spring_backend();
    assert_eq!(
        variant_with_data.string(),
        "Dies ist ein Talk über Spring".to_string()
    );
}

fn next_slide() {
    result_and_option();
}
