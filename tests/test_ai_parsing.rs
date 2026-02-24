use architect_linter_pro::ai::extraer_json_flexible;

#[test]
fn test_extraer_json_flexible_with_markdown() {
    let input = "Sure, here is your JSON:\n```json\n{\n  \"pattern\": \"MVC\",\n  \"rules\": []\n}\n```\nHope it helps!";
    let result = extraer_json_flexible(input).unwrap();
    assert!(result.contains("\"pattern\": \"MVC\""));
    assert!(!result.contains("```"));
}

#[test]
fn test_extraer_json_flexible_raw() {
    let input = "{\n  \"pattern\": \"Clean\"\n}";
    let result = extraer_json_flexible(input).unwrap();
    assert_eq!(result, "{\n  \"pattern\": \"Clean\"\n}");
}

#[test]
fn test_extraer_json_flexible_truncated_error() {
    let input = "{\n  \"pattern\": \"MVC\",";
    let result = extraer_json_flexible(input);
    assert!(result.is_err());
}

#[test]
fn test_extraer_json_flexible_with_text_around() {
    let input = "The answer is: {\"options\": []} - thank you.";
    let result = extraer_json_flexible(input).unwrap();
    assert_eq!(result, "{\"options\": []}");
}
