use rand::Rng;

pub fn random_char(text: &str, rng: &mut impl Rng) -> char {
    let index = rng.gen_range(0..text.chars().count()) as usize;

    match text.chars().nth(index) {
        Some(c) => c,
        None => {
            eprintln!("Text '{}' has no char at index {}", text, index);
            '.'
        }
    }
}

pub fn power_law_char(text: &str, percentage: u32, rng: &mut impl Rng) -> char {
    let index = power_law(text.len() as u32, percentage, rng) as usize;
    text.to_string().chars().nth(index).unwrap()
}

pub fn power_law(max: u32, percentage: u32, rng: &mut impl Rng) -> u32 {
    let mut r: u32 = 0;

    loop {
        if rng.gen_range(0..=100) < percentage {
            return r;
        }

        r = (r + 1) % max;
    }
}
