use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u16,
        an: [u16; n],
    }

    let mut ans = 0;
    for a in &an {
        if *a < p {
            ans += 1;
        }
    }

    println!("{}", ans);
}
