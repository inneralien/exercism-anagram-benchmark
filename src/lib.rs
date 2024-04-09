use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // eprintln!("word to check: {word}");
    // Strategy
    // A word is an anagram of another word if it contains exactly all of the
    // letters in the other word.
    // We can sort the word by characters, then do the same for each word in
    // the possibilities and compare them directly.

    let lower_word = word.to_lowercase();
    // Create a vector of lowercase characters in the word
    let mut vec: Vec<char> = lower_word.chars().collect();
    // Sort that vector
    vec.sort_unstable();

    let mut anagrams: HashSet<&str> = HashSet::new();

    for &anagram in possible_anagrams {
        // Reject words that are the same as the input word, but after
        // ignoring case
        let lower_anagram = anagram.to_lowercase();
        if lower_anagram == lower_word {
            continue;
        }
        // Vectorize the potential anagram word
        let mut temp: Vec<char> = lower_anagram.chars().collect();
        // Sort the word
        temp.sort_unstable();
        // println!("after sorting: {temp:?}");

        // Compare 2 vectors?
        // println!("{temp:?} == {vec:?}: {}", temp == vec);
        if temp == vec {
            anagrams.insert(anagram);
        }
    }

    // println!("anagrams: {anagrams:?}");
    anagrams
}
