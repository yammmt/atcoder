// :fu: 21-05 Editional は別解法 (故に Diff 下がった？)
// DP 経路復元はともかく嘘解法で通ってしまった挙げ句ミスも複数箇所
// WA: else if の基本的なミス + その他諸々だがテストケースの穴を付いて通ってしまった

// "2 200 1" や "4 197 97 3 5" あたりが嵌り

use proconio::input;

fn main() {
    input! {
        mut n: usize,
        old_an: [usize; n],
    }

    let mut an = vec![];
    for o in &old_an {
        an.push(*o % 200);
    }

    // DP 中にオーバーフローすることに気付いたため応急処置 (気付いた時点で bit 全探索に移るべき)
    an = an.iter().take(10.min(n)).copied().collect::<Vec<usize>>();
    n = an.len();

    // 割った余りが等しい数列を作る
    // N <= 200 であり O(N^3) が通る
    // 愚直は B/C 両方に選ぶ選ばないの選択肢があり 4^N 通り試せば良く TLE
    // つまりは直接経路をもたせて DP 繊維すると TLE してしまう

    // A_i が 200 要素かつすべて 200 だった場合に dp の値が 2^200 になるので死ぬ
    // dp[i][j]: i 番目の要素まで見終わった状態で解が j となる組み合わせの数
    let mut dp = vec![vec![0usize; 200]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..200 {
            if dp[i][j] == 0 {
                continue;
            }

            // 選ばない
            dp[i + 1][j] += dp[i][j];
            // 選ぶ
            dp[i + 1][(j + an[i]) % 200] += dp[i][j];
        }
    }

    // dp[n] の要素に > 1 があれば達成可能
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
        } else if i != 0 && dp[n][i] > 1 {
            final_mod = i;
            break;
        }
    }

    // DP 経路復元: こっちは貪欲で良い
    let mut cur_mod = final_mod;
    let mut ans_b = vec![];
    for i in (1..n + 1).rev() {
        let prev_mod = (cur_mod + 200 - an[i - 1]) % 200;
        if dp[i - 1][prev_mod] > 0 {
            ans_b.push(i);
            cur_mod = prev_mod;
        }
    }
    ans_b.reverse();

    cur_mod = final_mod;
    let mut ans_c = vec![];
    let mut already_bunki = false;
    for i in (1..n + 1).rev() {
        let prev_mod = (cur_mod + 200 - an[i - 1]) % 200;
        // 何も選ばない場合の 0 に注意
        if !already_bunki && dp[i - 1][cur_mod] < dp[i][cur_mod]
            && ((i > 1 && ((dp[i - 1][cur_mod] > 0 && cur_mod != 0) || (dp[i - 1][cur_mod] > 1 && cur_mod == 0)))
            || (i == 1 && dp[i - 1][cur_mod] > 0))
        {
            // 未分岐 && 選んでも選ばなくても自身に来るパスがある
            // "選ばない" 方に分岐
            already_bunki = true;
            continue;
        }

        if dp[i - 1][prev_mod] > 0 {
            // 分岐後あるいは分岐不可
            ans_c.push(i);
            cur_mod = prev_mod;
        }
    }
    ans_c.reverse();

    println!("Yes");
    print!("{} ", ans_b.len());
    for (i, a) in ans_b.iter().enumerate() {
        print!("{}", a);
        if i == ans_b.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
    print!("{} ", ans_c.len());
    for (i, a) in ans_c.iter().enumerate() {
        print!("{}", a);
        if i == ans_c.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
