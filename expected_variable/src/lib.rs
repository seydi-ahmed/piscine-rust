extern crate case;

use case::CaseExt;



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



pub fn is_camel_case(s: &str) -> bool {
    s.chars().any(|c| c.is_uppercase())
}

pub fn is_snake_case(s: &str) -> bool {
    s.chars().any(|c| c == '_')
}


pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    if !contains_lowercase_and_underscore(compared) && !contains_lowercase_and_underscore(expected) {
        return None;
    }

    let compared_snake_case = compared.to_lowercase().replace(" ", "_");
    let expected_snake_case = expected.to_lowercase().replace(" ", "_");

    let max_length = compared_snake_case.len().max(expected_snake_case.len());

    let distance = edit_distance(&compared_snake_case, &expected_snake_case);
    let alikeness = 1.0 - (distance as f64 / max_length as f64);
    
    if alikeness > 0.5 {
        Some(format!("{:.0}% close to it", alikeness * 100.0))
    } else {
        Some("None".to_string())
    }
}

fn contains_lowercase_and_underscore(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_lowercase()) && s.contains('_')
}
