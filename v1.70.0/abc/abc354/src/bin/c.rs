use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        acn: [(usize, usize); n],
    }

    // コスト小順に見ていくなら, 自分より強いカードが既出であれば捨てる

    let mut acin = vec![];
    for (i, &ac) in acn.iter().enumerate() {
        // strength, cost, i
        acin.push((ac.0, ac.1, i + 1));
    }
    // コスト小 > 強さ小順
    acin.sort_unstable_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut ans = vec![];
    let mut strength_max = 0;
    for (a, _c, i) in acin {
        if strength_max > a {
            continue;
        }

        ans.push(i);
        strength_max = strength_max.max(a);
    }
    ans.sort_unstable();

    println!("{}", ans.len());
    for (i, a) in ans.iter().enumerate() {
        print!("{a}");
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
