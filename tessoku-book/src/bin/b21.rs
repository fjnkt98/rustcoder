use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String
    };

    // dp[i][j]: S[l..r]から作れる最長回文の長さ
    let mut dp = vec![vec![0; n + 1]; n + 1];
    // 1文字は長さ1の回分とみなす
    for i in 0..n {
        dp[i][i + 1] = 1;
    }

    let s = s.chars().collect::<Vec<char>>();

    // 区間DP
    for l in 2..=n {
        for i in 0..n - l + 1 {
            let j = i + l;

            if s[i] == s[j - 1] {
                // 現在見ている区間[i, j)の両端の文字が等しいならば、区間[l + 1, r - 1)にその両端を追加して回文を作れる
                // 区間[l + 1, r), [l, r - 1)の方が長い可能性もあるのでそれらのMaxを取る
                dp[i][j] = std::cmp::max(
                    dp[i + 1][j],
                    std::cmp::max(dp[i][j - 1], dp[i + 1][j - 1] + 2),
                );
            } else {
                // 両端の文字が等しくないなら、それらの文字は使えないので単に区間[l + 1, r), [l, r - 1)のMaxを取る
                dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j - 1]);
            }
        }
    }

    println!("{}", dp[0][n]);
}
