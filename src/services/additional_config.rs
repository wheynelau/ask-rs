use super::schema::RequestBodyBuilder;
use serde_json::json;

pub(super) fn gemini_config(mut builder: RequestBodyBuilder) -> RequestBodyBuilder {
    if builder.show_reasoning {
        let thinking_budget = match builder.reasoning_effort.as_deref() {
            Some("high") => 1024,
            Some("medium") => 512,
            Some("low") => 256,
            _ => 0,
        };
        builder.reasoning_effort = None; // Reset reasoning effort for gemini
        builder.extra_body = Some(json!({
            "google": {
                "thinking_config": {
                    "thinkingBudget" : thinking_budget,
                    "include_thoughts": true
                }
            }
        }));
        builder
    } else {
        builder
    }
}
