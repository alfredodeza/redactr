use regex::Regex;
use std::collections::HashMap;

// Define a struct to hold rule configuration
struct Rule {
    pattern: String,
    mapping: HashMap<String, String>,
    count: usize,
    placeholder: String,
    comment: String,
}

impl Rule {
    fn new(pattern: String, placeholder: String, comment: String) -> Self {
        Rule {
            pattern,
            mapping: HashMap::new(),
            count: 0,
            placeholder,
            comment,
        }
    }

    fn on_match(&mut self, matched_text: &str) -> String {
        println!("Redacting: {}", matched_text);
        let redacted_match = self
            .mapping
            .entry(matched_text.to_string())
            .or_insert_with(|| {
                self.count += 1;
                format!("{}{}", self.placeholder, self.count)
            });
        println!("Redacted result: {}", redacted_match);
        redacted_match.clone()
    }
}

// Load the rules from a configuration file (JSON)
fn load_rule_configs() -> Vec<Rule> {
    // load the regex-fules.json file to provide configs
    let json = include_str!("../regex-rules.json");

    // use serde to load the json file
    use serde::Deserialize;
    use serde_json::{Result, Value};
    #[derive(Deserialize)]
    struct JSONRuleConfig {
        pattern: String,
        placeholder: String,
        comment: String,
    }

    let loaded_json: Vec<JSONRuleConfig> = serde_json::from_str(json).unwrap();

    let mut rules: Vec<Rule> = Vec::new();
    for rule in loaded_json {
        rules.push(Rule::new(
            format!(r"{}", rule.pattern),
            rule.placeholder,
            rule.comment,
        ));
    }
    rules
}

fn main() {
    let mut rules = load_rule_configs();

    // Input text with proper names
    let input_text = "John Doe works with Adam Smith, and Adam Smith is about to retire. He lives in 22 Main St, Marietta, CA 94101. His phone number is 415-555-1234. His email is john@example.com";

    // Apply the rules sequentially
    let mut redacted_text = input_text.to_string();
    for rule in &mut rules {
        let regex = Regex::new(rule.pattern.as_str()).unwrap();
        for captures in regex.captures_iter(&input_text) {
            let matched_text = captures.get(0).unwrap().as_str();
            println!("Matched text: {} - Comment: {}", matched_text, rule.comment);
            let redacted_match = rule.on_match(matched_text);
            redacted_text = redacted_text.replace(matched_text, &redacted_match);
            println!("Redacted text: {}", redacted_text);
        }
    }

    // Print the redacted text
    println!("Redacted text: {}", redacted_text);
}
