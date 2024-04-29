use chrono::{NaiveDate, Utc};

// Error type for form validation
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

// Structure to handle form data
#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
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
            let err = FormError::new(String::from("first_name"), String::from(""), String::from("No user name"));
            return Err(err);
        } else {
            errors.push("Valid first name");
        }

        // Validate password
        if self.password.len() < 8 {
            let err = FormError::new(String::from("password"), self.password.clone(), String::from("At least 8 characters"));
            return Err(err);
        } else if !self.password.chars().any(|c| c.is_numeric())
            || !self.password.chars().any(|c| c.is_alphabetic())
            || !self.password.chars().any(|c| !c.is_alphanumeric())
        {
            let err = FormError::new(
                String::from("password"),
                self.password.clone(),
                String::from("Combination of different ASCII character types (numbers, letters and non-alphanumeric characters)"),
            );
            return Err(err);
        } else {
            errors.push("Valid password");
        }

        Ok(errors)
    }
}

// Helper function to create NaiveDate from string
pub fn create_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
}



// Form { first_name: "Lee", last_name: "Silva", birth: 2015-09-05, birth_location: "Africa", password: "qwqwsa1dty_" }
// ["Valid first name", "Valid password"]
// FormError { form_values: ("first_name", ""), date: "2022-10-17 12:09:25", err: "No user name" }
// FormError { form_values: ("password", "dty_1"), date: "2022-10-17 12:09:25", err: "At least 8 characters" }
// FormError { form_values: ("password", "asdasASd(_"), date: "2022-10-17 12:09:25", err: "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)" }
// FormError { form_values: ("password", "asdasASd123SA"), date: "2022-10-17 12:09:25", err: "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)" }