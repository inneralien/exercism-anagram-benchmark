use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // eprintln!("word to check: {word}");
    // Strategy
    // A word is an anagram of another word if it contains exactly all of the
    // letters in the other word.
    // We can sort the word by characters, then do the same for each word in
    // the possibilities and compare them directly.

    // Benchmark
    // Anagram 6 words         time:   [835.56 ns 837.52 ns 839.40 ns]
    // Found 4 outliers among 100 measurements (4.00%)
    //   3 (3.00%) high mild
    //   1 (1.00%) high severe

    // let lower_word = word.to_lowercase();
    // // Create a vector of lowercase characters in the word
    // let mut vec: Vec<char> = lower_word.chars().collect();
    // // Sort that vector
    // vec.sort_unstable();
    //
    // let mut anagrams: HashSet<&str> = HashSet::new();
    //
    // for &anagram in possible_anagrams {
    //     // Reject words that are the same as the input word, but after
    //     // ignoring case
    //     let lower_anagram = anagram.to_lowercase();
    //     if lower_anagram == lower_word {
    //         continue;
    //     }
    //     // Vectorize the potential anagram word
    //     let mut temp: Vec<char> = lower_anagram.chars().collect();
    //     // Sort the word
    //     temp.sort_unstable();
    //     // println!("after sorting: {temp:?}");
    //
    //     // Compare 2 vectors?
    //     // println!("{temp:?} == {vec:?}: {}", temp == vec);
    //     if temp == vec {
    //         anagrams.insert(anagram);
    //     }
    // }
    //
    // anagrams

    // Refactor the above to use filters to see if benchmark numbers improve.
    // This solution is takin from one of the community solutions on Exercism.
    //
    // Benchmark
    // Anagram 6 words         time:   [678.21 ns 680.52 ns 682.85 ns]
    //                         change: [-18.998% -18.658% -18.299%] (p = 0.00 < 0.05)
    //                         Performance has improved.
    // Found 6 outliers among 100 measurements (6.00%)
    //   6 (6.00%) high mild
    let word_lower = word.to_lowercase();
    let word_sorted = get_sorted(&word_lower);
    possible_anagrams
        .iter()
        .filter(|candidate| {
            let candidate_lower = candidate.to_lowercase();
            candidate_lower.len() == word_lower.len()
                && candidate_lower != word_lower
                && get_sorted(&candidate_lower) == word_sorted
        })
        .copied()
        .collect()
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}
