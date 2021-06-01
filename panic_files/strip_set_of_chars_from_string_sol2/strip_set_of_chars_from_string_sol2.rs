fn strip_characters(original : &str, to_strip : &str) -> String {
    original.chars().filter(|&c| !to_strip.contains(c)).collect()
}

fn main() {
    println!("{}", strip_characters("She was a soul stripper. She took my heart!", "aei"));
}