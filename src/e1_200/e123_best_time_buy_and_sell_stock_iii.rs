fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut dp = vec![0; n];
    let mut min_i = 0;
    let mut res = 0;
    for i in 0..n {
        if prices[i] <= prices[min_i] {
            min_i = i;
        }

        res = res.max(prices[i] - prices[min_i] + if min_i>0 { dp[min_i-1] } else { 0 });
        dp[i] = prices[i]-prices[min_i];
        if i>0 {
            dp[i] = dp[i].max(dp[i-1]);
        }
    }

    let mut max_i = n-1;
    for i in (0..n).rev() {
        if prices[i] >= prices[max_i] {
            max_i = i;
        }

        res = res.max(prices[max_i] - prices[i] + if i>0 { dp[i-1] } else { 0 });
    }

    res
}

pub fn main() {
    let prices = [1,2,4,2,5,7,2,4,9,0].to_vec();
    println!("{}", max_profit(prices));
}
