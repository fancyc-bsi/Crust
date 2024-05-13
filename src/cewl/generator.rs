use std::collections::HashSet;

pub fn generate_wordlist(words: Vec<String>, min_length: usize) -> HashSet<String> {
    words.into_iter()
        .filter(|word| word.len() >= min_length)
        .collect()
}

