use proconio::input;

fn main() {
    input! {
        mut a: [u32; 6],
    }
    a.sort_unstable();
    println!("{}", a[3]);
}
