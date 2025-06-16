use super::request::RequestBodyBuilder;
use serde_json::json;

pub(super) fn gemini_config(mut builder: RequestBodyBuilder) -> RequestBodyBuilder {
    if !builder.show_reasoning {
        return builder;
    }

    let thinking_budget: u16 = match builder.reasoning_effort.as_deref() {
        Some("high") => 24_576,
        Some("medium") => 8_192,
        Some("low") => 512,
        _ => 0,
    };

    builder.reasoning_effort = None; // Reset reasoning effort for gemini

    if thinking_budget != 0 {
        builder.extra_body = Some(json!({
            "google": {
                "thinking_config": {
                    "thinkingBudget": thinking_budget,
                    "include_thoughts": true
                }
            }
        }));
    }

    builder
}
