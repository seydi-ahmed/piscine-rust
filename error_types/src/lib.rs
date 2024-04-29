use chrono::{Utc, NaiveDate};

#[derive(Debug, Eq, PartialEq, Clone)] // Ajoutez `Clone` ici
pub struct FormError {
    form_values: (String, String),
    date: String,
    err: String,
}

impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Utc::now().to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    first_name: String,
    last_name: String,
    birth: NaiveDate,
    birth_location: String,
    password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }
    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        let mut errors = Vec::new();

        // Validate first name
        if self.first_name.is_empty() {
            errors.push(FormError::new("first_name".to_string(), self.first_name.clone(), "No user name".to_string()));
        } else {
            errors.push(FormError::new("first_name".to_string(), self.first_name.clone(), "Valid first name".to_string()));
        }

        // Validate password
        if self.password.len() < 8 {
            errors.push(FormError::new("password".to_string(), self.password.clone(), "At least 8 characters".to_string()));
        } else if !self.password.chars().any(char::is_numeric) || !self.password.chars().any(char::is_alphabetic) || self.password.chars().all(char::is_alphanumeric) {
            errors.push(FormError::new("password".to_string(), self.password.clone(), "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string()));
        } else {
            errors.push(FormError::new("password".to_string(), self.password.clone(), "Valid password".to_string()));
        }

        if errors.is_empty() {
            Ok(errors.into_iter().map(|s| s.err.as_str()).collect())
        } else {
            Err(errors[0].clone())
        }
    }
}
