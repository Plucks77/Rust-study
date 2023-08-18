pub fn abbreviate(phrase: &str) -> String {
    let mut acronym: String = "".to_string();

    let split_phrase: Vec<&str> = phrase.split(' ').collect();
    for word in split_phrase {
        let uppers = get_uppers(word);
        acronym.push_str(&uppers.to_string());
    }
    return acronym;
}

fn get_uppers(p: &str) -> String {
    let mut ret: String = "".to_string();

    if p.contains("-") && p.len() > 1 {
        let sploted: Vec<&str> = p.split("-").collect();

        for char in sploted {
            ret.push_str(&char.chars().next().unwrap().to_uppercase().to_string());
        }

        return ret;
    }

    for (index, char) in p.chars().enumerate() {
        if !char.is_ascii_alphabetic() {
            continue;
        }
        if ret.is_empty() {
            ret.push_str(&char.to_uppercase().to_string());
            continue;
        }
        if char.is_uppercase() {
            let prev = p.chars().nth(index - 1);

            if let Some(prev) = prev {
                if !prev.is_uppercase() {
                    ret.push(char);
                }
            }
        }
    }
    return ret;
}

fn main() {
    //     assert_eq!(abbreviate("GNU Image Manipulation Program"), "GIMP");
    // let p = "GNU Image Manipulation Program";
    // let p = "HyperText Markup Language";
    let p = "Something - I made up from thin air";
    // let p = "Complementary metal-oxide semiconductor";

    let abb = abbreviate(p);
    println!("ABB: {}", abb)
}

#[test]
fn empty() {
    assert_eq!(abbreviate(""), "");
}
#[test]
fn basic() {
    assert_eq!(abbreviate("Portable Network Graphics"), "PNG");
}
#[test]
fn lowercase_words() {
    assert_eq!(abbreviate("Ruby on Rails"), "ROR");
}
#[test]
fn camelcase() {
    assert_eq!(abbreviate("HyperText Markup Language"), "HTML");
}
#[test]
fn punctuation() {
    assert_eq!(abbreviate("First In, First Out"), "FIFO");
}
#[test]
fn all_caps_word() {
    assert_eq!(abbreviate("GNU Image Manipulation Program"), "GIMP");
}
#[test]
fn all_caps_word_with_punctuation() {
    assert_eq!(abbreviate("PHP: Hypertext Preprocessor"), "PHP");
}
#[test]
fn punctuation_without_whitespace() {
    assert_eq!(
        abbreviate("Complementary metal-oxide semiconductor"),
        "CMOS"
    );
}
#[test]
fn very_long_abbreviation() {
    assert_eq!(
        abbreviate("Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me"),
        "ROTFLSHTMDCOALM"
    );
}
#[test]
fn consecutive_delimiters() {
    assert_eq!(abbreviate("Something - I made up from thin air"), "SIMUFTA");
}
#[test]
fn apostrophes() {
    assert_eq!(abbreviate("Halley's Comet"), "HC");
}
#[test]
fn underscore_emphasis() {
    assert_eq!(abbreviate("The Road _Not_ Taken"), "TRNT");
}
