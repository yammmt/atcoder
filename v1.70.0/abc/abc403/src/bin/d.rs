// 実装詰まる...

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    // const A_MAX: usize = 15;
    const A_MAX: usize = 1_000_002;

    input! {
        n: usize,
        d: usize,
        an: [usize; n],
    }

    // ペアはどちらか片方 (or 両方もありうる) を必ず消さねばならない
    // どっちを消すかで二択
    // dp[i][0, 1]: i 組目の pair まで見て [0]: 直前を残した, [1]: 直前を消した
    //              場合の要素削除数の最小
    // A の最大値がそれほどでないので, 全領域舐めてやった方が実装が安全？
    // でも消す消さないを扱う賢い実装が思いつかず, 却下

    // 幅 D の公差数列部分だけを取り出せばよさそう
    // A の値を頭から見ていって, 差が途切れるまで進む
    // 見た数字には skip マークをつける, で高々 A_MAX * 2 回の反復で足りるはず

    let mut num_counts = vec![0; A_MAX];
    for a in an {
        num_counts[a] += 1;
    }
    if d == 0 {
        // 死角
        let mut ans = 0;
        for c in num_counts {
            if c > 1 {
                ans += c - 1;
            }
        }
        println!("{ans}");
        return;
    }

    let mut visited = vec![false; A_MAX];
    let mut ans = 0;
    for i in 0..A_MAX {
        if visited[i] {
            continue;
        }

        let mut j = i;
        let mut dp = vec![0; 2];
        while j < visited.len() && !visited[j] {
            visited[j] = true;
            let mut dp_next = vec![0; 2];
            // 今回は残す
            dp_next[0] = dp[1];
            // 今回は消す
            dp_next[1] = (dp[0].min(dp[1])) + num_counts[j];
            j += d;
            dp = dp_next;
        }
        // println!("i: {i}, dp: {:?}", dp);
        ans += dp[0].min(dp[1]);
    }

    println!("{ans}");
}
