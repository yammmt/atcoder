use proconio::fastout;
use proconio::marker::Usize1;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        abn: [(Usize1,Usize1); n],
    }

    // 交点は 2N 個だから必ず各点の対称方向に点がある
    // 例 1 では 1-3 が交点をもつ => 2 から 3 < n < 7 (1) に伸びている
    // 線分の左側/右側の任意の点をつなぐ線が引かれているか？
    // 愚直では TLE するので高速化したいがどうやれば

    // 別解で二分探索やろうにも判定片方向だけにした方がよい
    // ソートしてから片方向を全点回せば結局全部判定取っていることになる
    // thx: https://atcoder.jp/contests/abc338/submissions/49724508

    // 点 2N 個に被りがないから実装で楽できる
    // (push?, id)
    let mut events = vec![(false, 0); 2 * n];

    for (i, ab) in abn.iter().enumerate() {
        let a = ab.0.min(ab.1);
        let b = ab.0.max(ab.1);

        events[a] = (true, i);
        events[b] = (false, i);
    }

    let mut pushed = vec![];
    for e in events {
        if e.0 {
            pushed.push(e.1);
        } else {
            let last_i = pushed.pop().unwrap();
            if last_i != e.1 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
