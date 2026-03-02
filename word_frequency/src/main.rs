// this solution is O(n) 
// where n is the number of words in the input text, since we need to iterate through all the words once to count them, 
// and then we find the maximum count in O(n) time as well.
fn most_frequent_word(text: &str) -> (String, usize) {
    use std::collections::HashMap; // we can use a hashmap lol
    // hashmap is good for keeping track of counted words
    
    let mut word_counts = HashMap::new();
    
    for word in text.split_whitespace() {
        *word_counts.entry(word).or_insert(0) += 1;
    }
    
    word_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(word, count)| (word.to_string(), count))
        .unwrap_or_default()
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}