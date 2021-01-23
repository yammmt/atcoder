use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            println!("NO");
            return;
        }
        i += 1;
    }

    println!("YES");
}
