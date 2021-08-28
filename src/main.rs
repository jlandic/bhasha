mod language;
mod math;

use language::Language;

fn main() {
    let mut language = Language::new("seed");
    language
        .with_category('V', "aeiou")
        .with_category('L', "āēīōū")
        .with_category('C', "ptcqbdglrhs")
        .with_category('N', "mn")
        .with_category('F', "ie")
        .with_category('B', "ou")
        .with_category('S', "ptc")
        .with_category('Z', "bdg")
        .with_syllable_type("CVC")
        .with_syllable_type("NV")
        .with_syllable_type("NL")
        .with_syllable_type("NB")
        .with_syllable_type("CV")
        .with_syllable_type("CLC")
        .with_syllable_type("CL")
        .with_syllable_type("CBC")
        .with_syllable_type("CB")
        .with_syllable_type("CFC")
        .with_syllable_type("CF")
        .with_syllable_type("ZBF")
        .with_syllable_type("SBF")
        .with_syllable_type("ZV")
        .with_rewrite_rules(("pi", "bi"))
        .with_rewrite_rules(("pn", "nn"))
        .with_rewrite_rules(("pm", "mm"));

    for _ in 0..100 {
        println!("{}", language.random_word(0, 50));
    }
}
