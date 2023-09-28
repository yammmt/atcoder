use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    for i in a..b + 1 {
        if i % c == 0 {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
