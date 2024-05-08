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
    fn append_str(&mut self, str_to_append: String) {
        self.value.push_str(&str_to_append);
        // return self.clone();
    }

    fn append_number(&mut self, nb_to_append: f64) {
        let mut new_nb : String = nb_to_append.clone().to_string();

        if nb_to_append.to_string().starts_with('-'){
            new_nb.remove(0);
        }

        self.value.push_str(&new_nb.to_string());
        // return self.clone();
    }

    fn remove_punctuation_marks(&mut self) {
        let punctuation_marks = ['.', ',', '?', '!'];
        self.value.retain(|c| !punctuation_marks.contains(&c));
        // return self.clone();
    }
}