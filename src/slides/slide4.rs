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
        use Tag::*;
        match self {
            Backend => String::from("Backend"),
            Frontend => String::from("Frontend"),
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
        match self {
            TagWithAssociatedData::Backend {
                language: _,
                framework: f,
            } => format!("Dies ist (k)ein Talk über {}", f),
            TagWithAssociatedData::Frontend {
                language: _,
                framework: f,
            } => format!("Dies ist (k)ein Talk über {}", f),
            TagWithAssociatedData::Ops => format!("Dies ist ein Ops Talk."),
            TagWithAssociatedData::LanguageIntroduction { language: lang } => {
                format!("Wir stellen {} vor", lang)
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
        "Dies ist (k)ein Talk über Spring".to_string()
    );
}

fn next_slide() {
    result_and_option();
}
