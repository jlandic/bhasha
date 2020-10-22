use std::collections::HashMap;

use crate::math::{power_law, power_law_char, random_char};

pub struct Language {
    pub syllable_types: Vec<String>,
    pub categories: HashMap<char, String>,
    pub syllables: Vec<String>,
}

impl Default for Language {
    fn default() -> Self {
        Self {
            syllable_types: vec![],
            categories: HashMap::new(),
            syllables: vec![],
        }
    }
}

impl Language {
    pub fn new() -> Self {
        Self {
            syllable_types: Vec::new(),
            categories: HashMap::new(),
            syllables: Vec::new(),
        }
    }

    pub fn with_category(&mut self, key: char, phonemes: String) -> &mut Self {
        self.categories.insert(key, phonemes);
        self
    }

    pub fn with_syllable_type(&mut self, description: String) -> &mut Self {
        self.syllable_types.push(description);
        self
    }

    pub fn generate_syllables(
        &mut self,
        dropoff: u32,
        syllable_dropoff: u32,
        amount: u32,
    ) -> &mut Self {
        for _ in 0..amount {
            self.syllables
                .push(self.random_syllable(dropoff, syllable_dropoff));
        }

        self
    }

    pub fn random_syllable(&self, dropoff: u32, syllable_dropoff: u32) -> String {
        let mut syllable = String::new();
        let pattern_index = power_law(self.categories.len() as u32, syllable_dropoff);
        let pattern = &self.syllable_types[pattern_index as usize];

        for c in pattern.chars() {
            let expansion = self.categories.get(&c).unwrap();

            if dropoff == 0 {
                syllable.push(random_char(&expansion));
            } else {
                syllable.push(power_law_char(&expansion, dropoff));
            }
        }

        syllable
    }

    pub fn random_word(&self, dropoff: u32, syllable_dropoff: u32) -> String {
        let mut word = String::new();
        let nw = 1 + power_law(4, 50);

        for _ in 0..nw {
            word.push_str(&self.random_syllable(dropoff, syllable_dropoff));
        }

        word
    }
}
