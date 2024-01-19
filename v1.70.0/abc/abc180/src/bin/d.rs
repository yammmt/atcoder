use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        x: u64,
        y: u64,
        a: u64,
        b: u64,
    }

    // 強さが倍になるルートは高々 64 回しか踏まない
    // 一回の操作で得られる経験値はどっちも 1 だから貪欲に小さい方を選ぶとよさそう
    // だが愚直には B=1 のときに 10^18 回判定しようとして TLE
    // B だけ選択し続けた場合の経験値を取っておく？
    // そもそも経験値を多く得たいなら A を選択できるのは序盤に限定されそう
    // すると最初の 0, 1, ..., 64 回 A を選ぶとして全探索できる

    let mut ans = 0u64;
    let mut power_after_a = x;
    for power_by_a in 0..=64 {
        // -1: y 以上とならない
        let power_by_b = (y - power_after_a - 1) / b;
        let score = power_by_a + power_by_b;
        ans = ans.max(score);

        // y 以上とならない, だから = が必要
        if power_after_a >= y / a {
            break;
        }
        power_after_a *= a;
    }

    println!("{ans}");
}
