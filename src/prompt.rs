pub fn format_prompt(
    system_prompt: &str,
    stdin_content: Option<&str>,
    user_question: &str,
) -> String {
    let mut prompt = String::with_capacity(
        system_prompt.len()
            + stdin_content.map(|s| s.len()).unwrap_or(0)
            + user_question.len()
            + 50,
    );

    // System prompt
    prompt.push_str(system_prompt);
    prompt.push_str("\n\n");

    // STDIN section if exists
    if let Some(stdin) = stdin_content {
        if !stdin.trim().is_empty() {
            prompt.push_str("# STDIN\n");
            prompt.push_str(stdin);
            prompt.push_str("\n\n");
        }
    }

    // Question section
    prompt.push_str("# Question\n");
    prompt.push_str(user_question);

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_prompt_with_stdin() {
        let system = "System prompt";
        let stdin = "Stdin content";
        let question = "User question";

        let result = format_prompt(system, Some(stdin), question);
        assert!(result.contains(system));
        assert!(result.contains("# STDIN"));
        assert!(result.contains(stdin));
        assert!(result.contains("# Question"));
        assert!(result.contains(question));
    }

    #[test]
    fn formats_prompt_without_stdin() {
        let system = "System prompt";
        let question = "User question";

        let result = format_prompt(system, None, question);
        assert!(result.contains(system));
        assert!(!result.contains("# STDIN"));
        assert!(result.contains("# Question"));
        assert!(result.contains(question));
    }
}
