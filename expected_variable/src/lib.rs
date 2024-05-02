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



pub fn is_camel_case(s: &str) -> bool {
    s.chars().any(|c| c.is_uppercase())
}

pub fn is_snake_case(s: &str) -> bool {
    s.chars().any(|c| c == '_')
}


pub fn expected_variable(compared_str: &str, expected_str: &str) -> Option<String> {
    // Check if the compared string is in camel case or snake case
    if is_camel_case(compared_str) || is_snake_case(compared_str) {
        // Calculate edit distance between compared string and expected string
        let distance = edit_distance(compared_str, expected_str);
        let max_distance = (expected_str.len() as f64 * 0.5).ceil() as usize;

        // Check if the distance is less than or equal to 50% of expected string length
        if distance <= max_distance {
            // Calculate percentage similarity
            let similarity = ((expected_str.len() - distance) as f64 / expected_str.len() as f64 * 100.0).round() as usize;
            Some(format!("{}% close to it", similarity))
        } else {
            None
        }
    } else {
        None
    }
}

