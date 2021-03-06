use std::str::FromStr;
use std::fmt::Debug;

fn read_line<T>() -> Vec<T>
where T: FromStr, <T as FromStr>::Err : Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn main() {
    let vec: Vec<usize> = read_line();
    let k = vec[1];
    let hs: Vec<i32> = read_line();
    let mut dp: Vec<i32> = Vec::new();

    for _ in 0..hs.len() { dp.push(0); }
    dp[1] = (hs[1] - hs[0]).abs();

    for i in 2..hs.len() {
        let start: usize = if (i as i32) - (k as i32) >= 0 { i - k } else { 0 };
        let mut costs: Vec<i32> = Vec::new();

        for j in start..i { costs.push(dp[j] + (hs[i] - hs[j]).abs()); }
        dp[i] = costs.into_iter().min().unwrap();
    }

    println!("{}", dp.last().unwrap());
}