use core::iter::zip;
use std::collections::HashMap;
use enum_iterator::{all, Sequence}; // Allows iteration over the enumeration of Allergens

#[derive(Debug, PartialEq, Eq, Sequence, Hash, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

pub struct Allergies {
    allergy_list: Vec<Allergen>,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergies = Allergies{allergy_list: Vec::<Allergen>::new()};
        let allergy_map: HashMap<Allergen, u32> = HashMap::from_iter(zip(all::<Allergen>(), (0..).map(|i| 0x1 << i)));
        // println!("{:?}", allergy_map);
        allergies.allergy_list = all::<Allergen>().filter(|k| score & (allergy_map[k]) > 0).collect();
        /*for (key,value) in &allergy_map {
            if (score & value) > 0 {
                allergies.allergy_list.push(key.clone());
            }
            // println!("Key {:?} Value: {:?} Score: {:#04x}", key, value, score);
        }*/
        // println!("{:?}", allergies.allergy_list);
        allergies
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergy_list.iter().any(|&i| i == allergen.clone())
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergy_list.iter().map(|x| x.clone()).collect()
    }
}
