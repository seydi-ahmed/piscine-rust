#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String);
    fn append_number(&mut self, nb_to_append: f64);
    fn remove_punctuation_marks(&mut self);
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append: String) -> Self {
        self.value.push_str(&str_to_append);
    }

    fn append_number(&mut self, nb_to_append: f64) -> Self {
        self.value.push_str(&nb_to_append.to_string());
    }

    fn remove_punctuation_marks(&mut self) -> Self {
        let punctuation_marks = ['.', ',', '?', '!'];
        self.value.retain(|c| !punctuation_marks.contains(&c));
    }
}