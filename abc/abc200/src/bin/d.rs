// :fu: 21-05 DP 経路復元は EDPC になかったか でも Editional は別解法 (故に Diff 下がった？)
// WA: else if の基本的なミス

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut old_an: [usize; n],
    }

    let mut an = vec![];
    for o in &old_an {
        an.push(*o % 200);
    }

    let mut cnt = vec![0; 200];
    for a in &an {
        cnt[*a % 200] += 1;
    }

    // 割った余りが等しい数列を作る
    // N <= 200 であり O(N^3) が通る
    // 愚直は B/C 両方に選ぶ選ばないの選択肢があり 4^N 通り試せば良く TLE
    // つまりは直接経路をもたせて DP 繊維すると TLE してしまう

    // dp[i][j]: i 番目の要素まで見終わった状態で解が j となる組み合わせの数
    // dp[n] の要素に > 1 があれば達成可能
    let mut dp = vec![vec![0; 200]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..200 {
            if dp[i][j] == 0 {
                continue;
            }

            dp[i + 1][j] += dp[i][j];
            dp[i + 1][(j + an[i]) % 200] += 1;
        }
    }

    // 0 はすべて選ばない場合でも 1 になってしまうので注意
    if dp[n].iter().skip(1).all(|&d| d < 2) && dp[n][0] < 3 {
        println!("No");
        return;
    }

    let mut final_mod = 0;
    for i in 0..200 {
        if i == 0 && dp[n][0] > 2 {
            final_mod = 0;
            break;
        } else if i > 0 && dp[n][i] > 1 {
            final_mod = i;
            break;
        }
    }
    // println!(" final_mod: {}", final_mod);

    let mut cur_mod = final_mod;
    let mut ans_b = vec![];
    for i in (1..n + 1).rev() {
        let prev_mod = (cur_mod + 200 - an[i - 1]) % 200;
        // println!("  pm: {}", prev_mod);
        // println!("  dp: {}", dp[i - 1][prev_mod]);
        if dp[i - 1][prev_mod] > 0 {
            // println!("    -= {}", an[i - 1]);
            ans_b.push(i - 1);
            cur_mod = prev_mod;
        }
    }
    ans_b.reverse();

    cur_mod = final_mod;
    let mut ans_c = vec![];
    let mut already_bunki = false;
    for i in (1..n + 1).rev() {
        let prev_mod = (cur_mod + 200 - an[i - 1]) % 200;
        // println!("  pm: {}", prev_mod);
        // println!("  dp[prev]: {}", dp[i - 1][prev_mod]);
        // println!("  dp[cur]: {}", dp[i - 1][cur_mod]);
        if !already_bunki && dp[i - 1][cur_mod] < dp[i][cur_mod] {
            // "選ばない" 方に分岐
            // println!("    already");
            already_bunki = true;
            continue;
        }

        if (already_bunki && dp[i - 1][prev_mod] > 0)
            || (!already_bunki && dp[i - 1][prev_mod] == dp[i][cur_mod])
        {
            // 分岐後あるいは分岐不可
            // println!("    -= {}", an[i - 1]);
            ans_c.push(i - 1);
            cur_mod = prev_mod;
        }
    }
    // println!("{:?}", already_bunki);
    ans_c.reverse();

    println!("Yes");
    print!("{} ", ans_b.len());
    for (i, a) in ans_b.iter().enumerate() {
        print!("{}", a + 1);
        if i == ans_b.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
    print!("{} ", ans_c.len());
    for (i, a) in ans_c.iter().enumerate() {
        print!("{}", a + 1);
        if i == ans_c.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
