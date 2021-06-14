use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }

    println!(
        "{}",
        if a == b || (a == -b && c % 2 == 0 ) {
            "="
        } else if c % 2 == 0 {
            if a.abs() > b.abs() {
                ">"
            } else {
                "<"
            }
        } else {
            if a > b {
                ">"
            } else {
                "<"
            }
        }
    );
}
