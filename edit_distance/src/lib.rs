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