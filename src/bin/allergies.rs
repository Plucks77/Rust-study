pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    fn new(score: u32) -> Option<Self> {
        match score {
            1 => Some(Allergen::Eggs),
            2 => Some(Allergen::Peanuts),
            4 => Some(Allergen::Shellfish),
            8 => Some(Allergen::Strawberries),
            16 => Some(Allergen::Tomatoes),
            32 => Some(Allergen::Chocolate),
            64 => Some(Allergen::Pollen),
            128 => Some(Allergen::Cats),
            _ => None,
        }
    }

    fn allergies_by_score(mut score: u32) -> Vec<Allergen> {
        // TODO: This is dumb. If you cast the enum to u8, you get the enum value.
        score = if score > 255 { score - 256 } else { score };
        let mut allergies: Vec<Allergen> = vec![];
        let mut number = 128;
        for _ in 0..8 {
            let re = score % number;
            if re != score {
                if re == 0 {
                    allergies.push(Allergen::new(number).unwrap());
                    break;
                }
                if re > 0 {
                    allergies.push(Allergen::new(number).unwrap());
                    score = score - number;
                    number = number / 2;
                }
            } else {
                number = number / 2;
            }
        }
        allergies
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let direct_match = Allergen::new(self.score);
        match direct_match {
            Some(x) => {
                if &x == allergen {
                    return true;
                } else {
                    false
                }
            }
            None => {
                let aproximate_match = Allergen::allergies_by_score(self.score);
                println!("APP {:?}", aproximate_match);
                for i in aproximate_match {
                    if &i == allergen {
                        return true;
                    }
                }
                false
            }
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::allergies_by_score(self.score)
    }
}

fn main() {
    let x = Allergies::new(12);
    let expected = &[
        Allergen::Eggs,
        Allergen::Peanuts,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];
    let allergies = Allergies::new(253).allergies();
    println!("ALERGIES {:?}", allergies);
    // println!("{:?}", x.is_allergic_to(&Allergen::Strawberries));
}

fn compare_allergy_vectors(expected: &[Allergen], actual: &[Allergen]) {
    for element in expected {
        if !actual.contains(element) {
            panic!(
                "Allergen missing\n  {:?} should be in {:?}",
                element, actual
            );
        }
    }
    if actual.len() != expected.len() {
        panic!(
            "Allergy vectors are of different lengths\n  expected {:?}\n  got {:?}",
            expected, actual
        );
    }
}
#[test]
fn is_not_allergic_to_anything() {
    let allergies = Allergies::new(0);
    assert!(!allergies.is_allergic_to(&Allergen::Peanuts));
    assert!(!allergies.is_allergic_to(&Allergen::Cats));
    assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
}
#[test]

fn is_allergic_to_eggs() {
    assert!(Allergies::new(1).is_allergic_to(&Allergen::Eggs));
}
#[test]

fn is_allergic_to_eggs_and_shellfish_but_not_strawberries() {
    let allergies = Allergies::new(5);
    assert!(allergies.is_allergic_to(&Allergen::Eggs));
    assert!(allergies.is_allergic_to(&Allergen::Shellfish));
    assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
}
#[test]

fn no_allergies_at_all() {
    let expected = &[];
    let allergies = Allergies::new(0).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]

fn allergic_to_just_eggs() {
    let expected = &[Allergen::Eggs];
    let allergies = Allergies::new(1).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]

fn allergic_to_just_peanuts() {
    let expected = &[Allergen::Peanuts];
    let allergies = Allergies::new(2).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]

fn allergic_to_just_strawberries() {
    let expected = &[Allergen::Strawberries];
    let allergies = Allergies::new(8).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]

fn allergic_to_eggs_and_peanuts() {
    let expected = &[Allergen::Eggs, Allergen::Peanuts];
    let allergies = Allergies::new(3).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]

fn allergic_to_eggs_and_shellfish() {
    let expected = &[Allergen::Eggs, Allergen::Shellfish];
    let allergies = Allergies::new(5).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]

fn allergic_to_many_things() {
    let expected = &[
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];
    let allergies = Allergies::new(248).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]

fn allergic_to_everything() {
    let expected = &[
        Allergen::Eggs,
        Allergen::Peanuts,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];
    let allergies = Allergies::new(255).allergies();
    compare_allergy_vectors(expected, &allergies);
}
#[test]

fn scores_over_255_do_not_trigger_false_positives() {
    let expected = &[
        Allergen::Eggs,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];
    let allergies = Allergies::new(509).allergies();
    compare_allergy_vectors(expected, &allergies);
}
