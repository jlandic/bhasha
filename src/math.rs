use rand::Rng;

pub fn random_char(text: &str) -> char {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, text.chars().count()) as usize;

    match text.chars().nth(index) {
        Some(c) => c,
        None => {
            eprintln!("Text '{}' has no char at index {}", text, index);
            '.'
        }
    }
}

pub fn power_law_char(text: &str, percentage: u32) -> char {
    let index = power_law(text.len() as u32, percentage) as usize;
    text.to_string().chars().nth(index).unwrap()
}

pub fn power_law(max: u32, percentage: u32) -> u32 {
    let mut r: u32 = 0;
    let mut rng = rand::thread_rng();

    loop {
        if rng.gen_range(0, 100) < percentage {
            return r;
        }

        r = (r + 1) % max;
    }
}
