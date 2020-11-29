use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [i64; n],
    }

    let mut s1 = 0;
    let mut s2 = w.iter().sum::<i64>();
    let mut ans = s2;
    for i in &w {
        s1 += *i;
        s2 -= *i;
        ans = ans.min((s1 - s2).abs());
    }
    println!("{}", ans);
}
