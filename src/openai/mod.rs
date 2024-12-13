pub fn get_response(input: &str) -> String {
    let client = reqwest::blocking::Client::new();

    let body = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", config_reader::read_value("OPENAIAPIKEY")))
        .header("Content-Type", "application/json")
        .body("{
                \"model\": \"gpt-4o-mini\",
                \"messages\": [
                    {
                        \"role\": \"system\",
                        \"content\": \"You are a tool which can categorize news headlines by their severity on a scale of 1 - 10. 1 represents the least severe headline possible, while 10 represents maximum severity like the outbreak of a war or a pandemic. You return the result in json format and you do not return anything else. The json should include the headlines and keep the same order as the input.\"
                    },
                    {
                        \"role\": \"user\",
                        \"content\": \"Write a haiku that explains the concept of recursion.\"
                    }
                ]
                }"
        );

    let response = body.send().unwrap().text().unwrap().as_str().to_owned();

    response
}