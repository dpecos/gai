use serde_json::json;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut prompt = String::from(
        "Generate a git commit message that describes the changes from the following git diff:\n\n",
    );

    let mut git_diff = String::new();
    loop {
        let r = io::stdin().read_line(&mut git_diff)?;
        if r == 0 {
            break;
        }
    }

    if git_diff.len() > 0 {
        prompt.push_str(&git_diff);

        let token = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
        let auth_header = format!("Bearer {}", token);

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/completions")
            .header("Authorization", auth_header)
            .json(&json!({
                "model": "text-davinci-003",
                "prompt": prompt,
                "temperature": 0.0,
                "max_tokens": 150,
                "top_p": 1.0,
                "frequency_penalty": 0.0,
                "presence_penalty": 0.0,
                "stream": false,
            }))
            .send()
            .await?;

        let json_response: serde_json::Value = response.json().await?;
        let mut generated_commit_message =
            json_response["choices"][0]["text"].as_str().unwrap_or("");

        let possible_prefixes = vec!["Commit message:", "Commit:", "Commit message:"];
        for prefix in possible_prefixes {
            if generated_commit_message
                .to_uppercase()
                .starts_with(&prefix.to_uppercase())
            {
                generated_commit_message = &generated_commit_message[prefix.len()..].trim();
            }
        }

        println!("{}", generated_commit_message);

        return Ok(());
    } else {
        return Err("No git diff provided".into());
    }
}
