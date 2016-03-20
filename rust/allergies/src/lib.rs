#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Allergen {
    Eggs         = 1 << 0,
    Peanuts      = 1 << 1,
    Shellfish    = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes     = 1 << 4,
    Chocolate    = 1 << 5,
    Pollen       = 1 << 6,
    Cats         = 1 << 7,
}

pub struct Allergies(pub u32);

impl Allergies {
    pub fn is_allergic_to(&self, a: &Allergen) -> bool {
        self.0 & (*a as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        vec![]
    }
}
