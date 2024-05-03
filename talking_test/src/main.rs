pub fn talking(text: &str) -> &str {
    if text == text.to_uppercase(){
        if text.trim_end().ends_with('!'){
            return "There is no need to yell, calm down!";
        }else if text.trim_end().ends_with('?'){
            return "Quiet, I am thinking!";
        }
    }

    if text != text.to_uppercase(){
        if text.ends_with('?'){
            return "Sure."
        }
    }

    if text.trim().is_empty(){
        return "Just say something!";
    }

    return "Interesting";
}





fn main() {
    println!("{:?}", talking("JUST DO IT!"));
    println!("{:?}", talking("Hello how are you?"));
    println!("{:?}", talking("WHAT'S GOING ON?"));
    println!("{:?}", talking("something"));
    println!("{:?}", talking(""));
}