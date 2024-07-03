use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

fn a_win(a: char, b: char) -> bool {
    !((b == 'R' && a == 'S') || (b == 'P' && a == 'R') || (b == 'S' && a == 'P'))
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    // 参加者が最大で 2^100 人も発生する
    // 試合数は初戦で 2^(k-1) 回 = 2^99 回あるので愚直だと無事に TLE
    // s を二度繰り返してやれば試合の勝者を繰り返しで表せられるようになる
    // 先頭分だけくくりゃあいいのか, 枝刈りもしなくてよさそう

    let mut hands = vec![];
    for i in 0..2 * n {
        hands.push(s[i % n]);
    }

    for _ in 0..k {
        let mut next = vec![];
        for i in 0..2 * n {
            if i % 2 == 1 {
                continue;
            }

            next.push(if a_win(hands[i], hands[i + 1]) {
                hands[i]
            } else {
                hands[i + 1]
            });
        }

        for i in 0..n {
            next.push(next[i]);
        }

        hands = next;
    }

    println!("{}", hands[0]);
}
