use super::schema::Response;
use futures_util::StreamExt;
use std::io::Write;
pub(super) async fn stream(response: reqwest::Response) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = response.bytes_stream();
    let mut buffer = Vec::new();

    while let Some(item) = stream.next().await {
        match item {
            Ok(chunk) => {
                buffer.extend_from_slice(&chunk);

                // Convert buffer to string and process lines
                if let Ok(text) = String::from_utf8(buffer.clone()) {
                    if text.contains("[DONE]") {
                        break;
                    }

                    // Process each line in the text
                    for line in text.lines() {
                        if line.starts_with("data: ") {
                            let data = line.replace("data: ", "");
                            if data == "[DONE]" {
                                break;
                            }
                            match serde_json::from_str::<Response>(&data) {
                                Ok(chunk) => {
                                    if let Some(content) = chunk.choices[0].delta.content.as_ref() {
                                        print!("{}", content);
                                        // Flush immediately to show the output
                                        std::io::stdout().flush()?;
                                    }
                                }
                                Err(e) => eprintln!("Error parsing chunk: {}", e),
                            }
                        }
                    }

                    // Clear buffer after processing
                    buffer.clear();
                }
            }
            Err(e) => eprintln!("Stream error: {}", e),
        }
    }

    Ok(())
}
