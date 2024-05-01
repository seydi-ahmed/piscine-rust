pub fn talking(text: &str) -> &str {
    // Check if the text is empty or contains only whitespace
    if text.trim().is_empty() {
        return "Just say something!";
    }

    // Check if the text is all uppercase
    let is_yelling = text.chars().all(|c| c.is_uppercase());

    // Check if the text ends with a question mark
    let is_question = text.trim().ends_with('?');

    // Return the appropriate response based on the conditions
    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        _ => "Interesting",
    }
}
