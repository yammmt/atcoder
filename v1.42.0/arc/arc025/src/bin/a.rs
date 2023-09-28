use proconio::input;

fn main() {
    input! {
        d7: [u16; 7],
        j7: [u16; 7],
    }

    let mut ans = 0;
    for i in 0..7 {
        ans += d7[i].max(j7[i]);
    }

    println!("{}", ans);
}
