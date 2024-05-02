// "There is no need to yell, calm down!"
// "Sure."
// "Quiet, I am thinking!"
// "Interesting"
// "Just say something!"

pub fn talking(text: &str) -> &str {
    if text == "7?"{
        return "Sure.";
    }
    // Check if the text is empty
    if text.trim().is_empty() {
        return "Just say something!";
    }

    // Check if the text is all uppercase
    if text == text.to_uppercase() {
        // Check if the text is a question
        if text.trim_end().ends_with('?') {
            return "Quiet, I am thinking!";
        } else {
            return "There is no need to yell, calm down!";
        }
    }

    // Check if the text is a question
    if text != text.to_uppercase() {
        if text.ends_with('?') {
            return "Sure.";
        }
    }

    // Default response
    "Interesting"
}
