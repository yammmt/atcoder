use proconio::input;

fn main() {
    input! {
        a: i16,
        b: i16,
    }

    println!(
        "{}",
        if b >= a {
            b - a + 1
        } else {
            0
        }
    );
}
