use std::collections::{HashMap, HashSet};
use wordcloud_rs;
use regex;
use regex::Regex;
use wordcloud_rs::{Token, WordCloud};

fn tokenize(text: String, exclude_words: &HashSet<String>) -> Vec<(Token, f32)> {
    let re_token: Regex = Regex::new(r"\w+").unwrap();
    let mut counts: HashMap<String, usize> = HashMap::new();
    for token in re_token.find_iter(&text) {
        let token_str = token.as_str().to_lowercase();
        if !exclude_words.contains(&token_str) {
            *counts.entry(token_str).or_default() += 1;
        }
    }
    counts.into_iter().map(|(k, v)| (Token::Text(k), v as f32)).collect()
}

pub fn generate_wordcloud(headlines: Vec<String>, name: &str) {
    let text = headlines.join(" ");
    let exclude_words: HashSet<String> = [
        "with", "on", "of", "and", "in", "the", "is", "for", "to", "from", "a",
        "Is", "than", "The", "are", "What", "what", "us", "How", "how", "after",
        "A", "at", "At", "have", "an", "not", "This", "this", "has", "Has", "into",
        "as", "As", "when", "do", "be", "or", "can", "after", "put", "It", "it", "Do",
        "no", "No", "Yes", "yes", "new", "old", "before", "too", "so", "by", "more",
        "about", "why", "our", "against", "over", "two", "but"
    ].iter().cloned().map(String::from).collect();
    let tokens = tokenize(text, &exclude_words);
    let wc = WordCloud::new().generate(tokens);
    wc.save(name).unwrap()
}