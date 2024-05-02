extern crate case;

pub use case::CaseExt;



pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut dp = vec![vec![0; target.len() + 1]; source.len() + 1];

    for i in 0..=source.len() {
        dp[i][0] = i;
    }

    for j in 0..=target.len() {
        dp[0][j] = j;
    }

    for (i, source_char) in source.chars().enumerate() {
        for (j, target_char) in target.chars().enumerate() {
            if source_char == target_char {
                dp[i + 1][j + 1] = dp[i][j];
            } else {
                dp[i + 1][j + 1] = dp[i][j].min(dp[i][j + 1].min(dp[i + 1][j])) + 1;
            }
        }
    }

    dp[source.len()][target.len()]
}

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    if !contains_lowercase_and_underscore(compared) && !contains_lowercase_and_underscore(expected) {
        return None;
    }

    let compared_snake_case = compared.to_lowercase().replace(" ", "_");
    let expected_snake_case = expected.to_lowercase().replace(" ", "_");

    let distance = edit_distance(&compared_snake_case, &expected_snake_case);
    let max_length = compared_snake_case.len().max(expected_snake_case.len());

    let similarity = 1.0 - (distance as f64 / max_length as f64);

    if similarity > 0.5 {
        Some(format!("{:.0}% close to it", similarity * 100.0))
    } else {
        None
    }
}


fn contains_lowercase_and_underscore(s: &str) -> bool {
    is_snake(s) || is_camel(s)
}

fn is_snake(s: &str) -> bool {
    // Vérifie si la chaîne contient uniquement des lettres minuscules, des chiffres et des traits de soulignement
    s.chars().all(|c| c.is_ascii_lowercase() || c.is_digit(10) || c == '_') &&
    // Vérifie si la chaîne ne commence ni ne se termine par un trait de soulignement
    !s.starts_with('_') && !s.ends_with('_') &&
    // Vérifie s'il y a au moins un trait de soulignement
    s.contains('_')
}

fn is_camel(s: &str) -> bool {
    // Vérifie si la chaîne commence par une lettre majuscule suivie de lettres minuscules ou des chiffres
    s.chars().next().map_or(false, |c| c.is_ascii_uppercase()) &&
    // Vérifie si la chaîne ne contient pas de traits de soulignement
    !s.contains('_') &&
    // Vérifie s'il y a au moins une lettre minuscule
    s.chars().any(|c| c.is_ascii_lowercase())
}
