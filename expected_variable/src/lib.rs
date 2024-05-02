pub use std::cmp::min;

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    // Simplified case detection
    let is_camel_case = compared.chars().all(|c: char| c.is_uppercase()) &&!compared.starts_with(|c: char| c.is_uppercase());
    let is_snake_case = compared.chars().all(|c: char| c.is_lowercase()) &&!compared.starts_with(|c: char| c.is_lowercase());

    if!is_camel_case &&!is_snake_case {
        return None;
    }

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

    let distance = edit_distance(compared, expected);
    let total_length = min(compared.len(), expected.len());
    let similarity = (total_length - distance) as f64 / total_length as f64 * 100.0;

    if similarity > 50.0 {
        Some(format!("{}%", similarity))
    } else {
        None
    }
}
