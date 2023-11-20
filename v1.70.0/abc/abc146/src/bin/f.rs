use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: Chars,
    }
    s.reverse();

    // m + 1 マス進む場合の最短は [1, m]
    // m + 2 マス進む場合の最短は [2, m]
    // 辞書順最小のためには桁数を下げたい
    // 前から貪欲は怪しいが, 後ろからなら貪欲に進めるだけ進むと桁が下がりそう
    // 障害物がない場合に貪欲で最適解出せない場合はある？サンプルにはない
    // 移動先を線形探索すると書き方次第で TLE だが O(N) にできそう, M は無視できる

    let mut ans = vec![];
    let mut cur = 0;
    let mut seen = 0;
    while cur < n {
        let mut move_to = None;
        while seen <= cur + m && seen <= n {
            if s[seen] == '0' {
                move_to = Some(seen);
            }
            seen += 1;
        }

        if let Some(mt) = move_to {
            ans.push(mt - cur);
            cur = mt;
        } else {
            println!("-1");
            return;
        }
    }

    ans.reverse();
    for (i, a) in ans.iter().enumerate() {
        print!("{a}");
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
