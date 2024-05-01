pub fn talking(text: &str) -> &str {
    // Check if the text is empty
    if text.trim().is_empty() {
        return "Just say something!";
    }

    if text == text.to_uppercase() {
        // Check if the text is a question (ends with a question mark)
        if text.ends_with('?') {
            return "Quiet, I am thinking!";
        } else {
            return "There is no need to yell, calm down!";
        }
    }

    // Check if the text is a question
    if text.ends_with('?') {
        return "Sure.";
    }

    // Default response
    "Interesting"
}







// "There is no need to yell, calm down!"
// "Sure."
// "Quiet, I am thinking!"
// "Interesting"
// "Just say something!"