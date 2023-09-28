use proconio::input;

fn main() {
    input! {
        s: u64,
        p: u64,
    }

    let mut i = 1;
    while i * i <= p {
        if p % i == 0 {
            let ii = p / i;
            if i + ii == s {
                println!("Yes");
                return;
            }
        }

        i += 1;
    }
    println!("No");
}
