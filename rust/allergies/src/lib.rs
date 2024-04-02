use self::Allergen::*;


pub struct Allergies {
    allergens: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs            = 1 << 0,
    Peanuts         = 1 << 1,
    Shellfish       = 1 << 2,
    Strawberries    = 1 << 3,
    Tomatoes        = 1 << 4,
    Chocolate       = 1 << 5,
    Pollen          = 1 << 6,
    Cats            = 1 << 7,
}

const ALLERGENS: [Allergen; 8] = 
    [
        Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats
    ];


impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            allergens: score,
        }
        // todo!("Given the '{score}' score, construct a new Allergies struct.");
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen = *allergen as u32;
        self.allergens & allergen == allergen
        // todo!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
        .iter()
        .filter(|allergen| self.is_allergic_to(allergen))
        .cloned()
        .collect::<Vec<Allergen>>()
        // todo!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }
}
