use proconio::input;

fn main() {
    input! {
        mut a3: [i64; 3],
    }
    a3.sort();
    println!(
        "{}",
        if a3[1] - a3[0] == a3[2] - a3[1] {
            "Yes"
        } else {
            "No"
        }
    );
}
