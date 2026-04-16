use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        abn: [(Usize1, Usize1); n],
    }

    let mut ans = 0;
    for i in 0..n {
        if i > abn[i].0 {
            continue;
        }

        if abn[i].1 == abn[abn[i].0].1 {
            ans += 1;
        }
    }

    println!("{ans}");
}
