use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut cnt = vec![0; 9];
    for a in an {
        cnt[a] += 1;
    }
    let mut ans = 999999;
    for c in cnt.iter().skip(1) {
        ans = ans.min(*c);
    }
    println!("{ans}");
}
