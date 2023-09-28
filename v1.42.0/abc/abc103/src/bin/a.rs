use proconio::input;

fn main() {
    input! {
        mut a3: [i64; 3],
    }
    a3.sort();
    println!("{}", a3[2] - a3[0]);
}
