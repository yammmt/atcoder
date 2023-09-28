use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut i = 1;
    loop {
        if i * 1000 >= n {
            println!("{}", i * 1000 - n);
            return;
        }
        i += 1;
    }
}
