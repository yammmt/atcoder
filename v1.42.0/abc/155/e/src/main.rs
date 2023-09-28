// -*- coding:utf-8-unix -*-

// https://algo-logic.info/abc155e/

use proconio::input;

fn main() {
    input! {
        n: String,
    }
    let vn = n.chars().map(|a| a.to_digit(10).unwrap() as i64).collect::<Vec<_>>();
    // dp[x][n]: n 桁目まで払った場合の最小枚数
    //           x: 1 であれば次の桁でお釣りをもらう
    let mut dp = vec![vec![0; vn.len() + 1]; 2];
    dp[1][0] = 1;
    for i in 0..vn.len() {
        // println!("{:?}", dp);
        // この桁でお釣りが出ない: お釣りが出ない払い方 + 今の桁で払う枚数
        // あるいはお釣りが出る払い方 + (繰り下がり) 受け取るお釣り
        dp[0][i + 1] = (dp[0][i] + vn[i]).min(dp[1][i] + 10 - vn[i]);
        // この桁でお釣りが出る: お釣りが出ない払い方 + お釣り 1 枚
        // あるいはお釣りが出る払い方 + さらに下の位にもお釣りが出る払い方
        dp[1][i + 1] = (dp[0][i] + vn[i] + 1).min(dp[1][i] + 10 - vn[i] - 1);
    }
    // println!("{:?}", dp);
    println!("{}", dp[0][vn.len()]);
}
