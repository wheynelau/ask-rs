/// Helper structs for creating and parsing the response from the API
///
/// Some notes: Response is always in the format Vec of Choices, where each choice has a delta
/// The delta is a message, which has a role and content
///
/// RequestBody is the struct that is used to send the request to the API

#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn test_serde_openai() {
        // sample taken from openai
        let json_response = r#"
            {
                "object": "list",
                "data": [
                    {
                    "id": "model-id-0",
                    "object": "model",
                    "created": 1686935002,
                    "owned_by": "organization-owner"
                    },
                    {
                    "id": "model-id-1",
                    "object": "model",
                    "created": 1686935002,
                    "owned_by": "organization-owner"
                    },
                    {
                    "id": "model-id-2",
                    "object": "model",
                    "created": 1686935002,
                    "owned_by": "openai"
                    }
                ],
                "object": "list"
            }
        "#;
        // Since APIResponse and Model are now in src/services/schema, we need to import them for the test
        use crate::services::schema::APIResponse; // Only APIResponse is directly used here
        let parsed: APIResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(parsed.data.len(), 3);
    }
}
