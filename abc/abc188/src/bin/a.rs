use proconio::input;

fn main() {
    input! {
        mut xy: [u16; 2],
    }
    xy.sort_unstable();
    println!(
        "{}",
        if xy[0] + 3 > xy[1] {
            "Yes"
        } else {
            "No"
        }
    );
}
