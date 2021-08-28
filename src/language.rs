use std::collections::HashMap;
use rand::rngs::SmallRng;
use rand_seeder::Seeder;

use crate::math::{power_law, power_law_char, random_char};

type RewriteRule<'a> = (&'a str, &'a str);

const DEFAULT_SEED: &str = "default seed";

pub struct Language<'a> {
    pub syllable_types: Vec<&'a str>,
    pub categories: HashMap<char, &'a str>,
    pub syllables: Vec<String>,
    pub rewrite_rules: Vec<RewriteRule<'a>>,
    pub rng: SmallRng,
}

impl<'a> Default for Language<'a> {
    fn default() -> Self {
        Self {
            syllable_types: vec![],
            categories: HashMap::new(),
            syllables: vec![],
            rng: Seeder::from(DEFAULT_SEED).make_rng(),
            rewrite_rules: vec![],
        }
    }
}

impl<'a> Language<'a> {
    pub fn new(seed: &str) -> Self {
        Self {
            syllable_types: vec![],
            categories: HashMap::new(),
            syllables:vec![],
            rng: Seeder::from(seed).make_rng(),
            rewrite_rules: vec![],
        }
    }

    pub fn with_category(&mut self, key: char, phonemes: &'a str) -> &mut Self {
        self.categories.insert(key, phonemes);
        self
    }

    pub fn with_syllable_type(&mut self, description: &'a str) -> &mut Self {
        self.syllable_types.push(description);
        self
    }

    pub fn with_rewrite_rules(&mut self, rule: RewriteRule<'a>) -> &mut Self {
        self.rewrite_rules.push(rule);
        self
    }

    #[allow(dead_code)]
    pub fn generate_syllables(
        &mut self,
        dropoff: u32,
        syllable_dropoff: u32,
        amount: u32,
    ) -> &mut Self {
        for _ in 0..amount {
            let syllable = self.random_syllable(dropoff, syllable_dropoff);
            self.syllables
                .push(syllable);
        }

        self
    }

    pub fn random_syllable(&mut self, dropoff: u32, syllable_dropoff: u32) -> String {
        let mut syllable = String::new();
        let pattern_index = power_law(self.categories.len() as u32, syllable_dropoff, &mut self.rng);
        let pattern = &self.syllable_types[pattern_index as usize];

        for c in pattern.chars() {
            let expansion = self.categories.get(&c).unwrap();

            if dropoff == 0 {
                syllable.push(random_char(&expansion, &mut self.rng));
            } else {
                syllable.push(power_law_char(&expansion, dropoff, &mut self.rng));
            }
        }

        syllable
    }

    pub fn random_word(&mut self, dropoff: u32, syllable_dropoff: u32) -> String {
        let mut word = String::new();
        let nw = 1 + power_law(4, 50, &mut self.rng);

        for _ in 0..nw {
            word.push_str(&self.random_syllable(dropoff, syllable_dropoff));
        }

        self.rewrite_rules.iter().for_each(|(from, to)| {
            word = word.replace(from, to);
        });

        word
    }
}
