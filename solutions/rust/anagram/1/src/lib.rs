use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let norm_word = normalize_word(word);
    let mut anagrams: Vec<&'a str> = Vec::new();

    for i in possible_anagrams{
        if i.to_lowercase() == word.to_lowercase() {
            continue;
        }
        let comparison: String = normalize_word(i);
        if norm_word == comparison {
            anagrams.push(i);
        }
    }

    let hashset: HashSet<&'a str> = anagrams.into_iter().collect();
    return hashset;

}

pub fn normalize_word(word: &str) -> String{

    let s = word.to_lowercase();
    let mut s_array: Vec<char> = s.chars().collect();
    s_array.sort();
    let s_string: String = s_array.into_iter().collect();
    return s_string;

}