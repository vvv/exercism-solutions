#[derive(PartialEq, Debug)]
pub enum Allergen {
    Eggs,         //   1 = 2^0
    Peanuts,      //   2 = 2^1
    Shellfish,    //   4 = 2^2
    Strawberries, //   8 = 2^3
    Tomatoes,     //  16 = 2^4
    Chocolate,    //  32 = 2^5
    Pollen,       //  64 = 2^6
    Cats,         // 128 = 2^7
}

fn score(a: &Allergen) -> u32 {
    let p2 = match a {
        &Allergen::Eggs => 0,
        &Allergen::Peanuts => 1,
        &Allergen::Shellfish => 2,
        &Allergen::Strawberries => 3,
        &Allergen::Tomatoes => 4,
        &Allergen::Chocolate => 5,
        &Allergen::Pollen => 6,
        &Allergen::Cats => 7,
    };
    1 << p2
}

pub struct Allergies(pub u32);

impl Allergies {
    pub fn is_allergic_to(&self, a: &Allergen) -> bool {
        self.0 & score(a) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        vec![]
    }
}
