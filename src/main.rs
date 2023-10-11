use serde_json::json;
use std::io;
use tokio::main;

fn read_stdin() -> Result<String, Box<dyn std::error::Error>> {
    let mut prompt =
        "Generate a git commit message that describes the changes from the following git diff:\n\n"
            .to_string();

    let mut git_diff = String::new();
    loop {
        let r = io::stdin().read_line(&mut git_diff)?;
        if r == 0 {
            break;
        }
    }

    if git_diff.len() > 0 {
        prompt.push_str(&git_diff);
        Ok(prompt)
    } else {
        Err("No git diff provided".into())
    }
}

async fn query_openai(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    let token = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY env variable not set");
    let auth_header = format!("Bearer {}", token);

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", auth_header)
        .json(&json!({
          "model": "text-davinci-003",
          "prompt": prompt,
          "temperature": 0.0,
          "max_tokens": 300,
          "top_p": 1.0,
          "frequency_penalty": 0.0,
          "presence_penalty": 0.0,
          "stream": false,
          "stop": ["\n\n\n"],
        }))
        .send()
        .await?;

    let json_response: serde_json::Value = response.json().await?;
    let mut openai_response = json_response["choices"][0]["text"]
        .as_str()
        .unwrap_or("")
        .trim();

    let possible_prefixes = vec!["Commit message:", "Commit:", "Message:", "git commit -m \""];
    for prefix in possible_prefixes {
        if openai_response
            .to_uppercase()
            .starts_with(&prefix.to_uppercase())
        {
            openai_response = openai_response[prefix.len()..].trim();
        }
    }

    if openai_response.ends_with("\"")
        || openai_response.ends_with("'")
        || openai_response.ends_with(".")
    {
        openai_response = &openai_response[..openai_response.len() - 1];
    }

    Ok(openai_response.to_string())
}

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prompt = read_stdin()?;

    let openai_response = query_openai(prompt).await;

    match openai_response {
        Ok(generated_commit_message) => {
            if generated_commit_message.len() == 0 {
                Err("Could not generate a description for the provided diff".into())
            } else {
                println!("{}", generated_commit_message);
                Ok(())
            }
        }
        Err(_) => Err("Could not connect to OpenAI".into()),
    }
}
