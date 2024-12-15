use vader_sentiment;

pub fn get_sentiment_value_multiple(input: Vec<String>) -> Vec<f64> {
    let vader = vader_sentiment::SentimentIntensityAnalyzer::new();
    let mut results = Vec::new();

    for sentence in input {
        results.push((*vader.polarity_scores(sentence.as_str()).get("compound").unwrap() * 100.0).round() / 100.0)
    }

    results
}

pub fn get_sentiment_value_single(input: &String) -> f64 {
    let vader = vader_sentiment::SentimentIntensityAnalyzer::new();
    (*vader.polarity_scores(input.as_str()).get("compound").unwrap() * 100.0).round() / 100.0
}