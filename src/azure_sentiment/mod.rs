use config_reader::read_value;
use serde::Deserialize;
use serde_json::json;

pub fn get_sentiment_scores(input: Vec<String>) -> Vec<f32> {
    let url = "https://missingsemester.cognitiveservices.azure.com/language/:analyze-text?api-version=2023-04-01";

    let documents = json!({
       "id": "documentId",
        "text": input.join(". "),
        "language": "de"
    });

    let analysis_input = json!({
        "documents": vec![documents]
    });

    let parameters = json!({
        "opinionMining": "false"
    });

    let item = json!({
        "kind": "SentimentAnalysis",
        "analysisInput": analysis_input,
        "parameters": parameters
    });

    dbg!(item.to_string());

    let client = reqwest::blocking::Client::new();

    let body = client.post(url)
        .header("Ocp-Apim-Subscription-Key", read_value("AZUREKEYONE"))
        .header("Content-Type", "application/json")
        .body(item.to_string());

    let response = body.send().unwrap().text().unwrap().as_str().to_owned();
    dbg!(&response);

    let sentiment_response: SentimentResponse = serde_json::from_str(response.as_str()).unwrap();
    let mut sentiment_values = Vec::new();

    for document in sentiment_response.results.documents {
        for sentence in document.sentences {
            sentiment_values.push((
                sentence.confidenceScores.positive,
                sentence.confidenceScores.neutral,
                sentence.confidenceScores.negative,
                convert_sentiment_values(sentence.confidenceScores.positive, sentence.confidenceScores.negative, sentence.confidenceScores.neutral),
                sentence.text
            ));
        }
    }

    let converted_values: Vec<f32> = sentiment_values.iter().map(|x| {convert_sentiment_values(x.0, x.2, x.1)}).collect();

    converted_values
}

fn split_and_combine_vector(sentences: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_string = String::new();

    for sentence in sentences {
        if current_string.len() + sentence.len() + 2 > 5110 {
            result.push(current_string);
            current_string = sentence;
        } else {
            if !current_string.is_empty() {
                current_string.push_str(". ");
            }
            current_string.push_str(&sentence);
        }
    }

    if !current_string.is_empty() {
        result.push(current_string);
    }

    result
}

fn convert_sentiment_values(pos: f32, neg: f32, neu: f32) -> f32 {
    let combined_score = (pos - neg) / (pos + neg + neu);

    combined_score
}

#[derive(Deserialize, Debug)]
struct SentimentResponse {
    results: Results,
}

#[derive(Deserialize, Debug)]
struct Results {
    documents: Vec<Document>,
}


#[derive(Deserialize, Debug)]
struct Document {
    id: String,
    sentiment: String,
    confidenceScores: ConfidenceScores,
    sentences: Vec<Sentence>,
}

#[derive(Deserialize, Debug)]
struct ConfidenceScores {
    positive: f32,
    neutral: f32,
    negative: f32,
}

#[derive(Deserialize, Debug)]
struct Sentence {
    sentiment: String,
    confidenceScores: ConfidenceScores,
    text: String,
}