use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }

    if c == 0 {
        println!(
            "{}",
            if a > b {
                "Takahashi"
            } else {
                "Aoki"
            }
        );
    } else {
        println!(
            "{}",
            if b > a {
                "Aoki"
            } else {
                "Takahashi"
            }
        );
    }
}
