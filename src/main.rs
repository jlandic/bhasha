mod language;
mod math;

use language::Language;

fn main() {
    let mut language = Language::new();
    language
        .with_category('V', "aeiou".to_string())
        .with_category('L', "āēīōū".to_string())
        .with_category('C', "ptcqbdgmnlrhs".to_string())
        .with_category('F', "ie".to_string())
        .with_category('B', "ou".to_string())
        .with_category('S', "ptc".to_string())
        .with_category('Z', "bdg".to_string())
        .with_syllable_type("CVC".to_string())
        .with_syllable_type("CV".to_string())
        .with_syllable_type("CLC".to_string())
        .with_syllable_type("CL".to_string())
        .with_syllable_type("CBC".to_string())
        .with_syllable_type("CB".to_string())
        .with_syllable_type("CFC".to_string())
        .with_syllable_type("CF".to_string())
        .with_syllable_type("ZBF".to_string())
        .with_syllable_type("SBF".to_string())
        .with_syllable_type("ZV".to_string());

    for _i in 0..100 {
        println!("{}", language.random_word(0, 50));
    }
}
