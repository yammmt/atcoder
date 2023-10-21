use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut appears = vec![false; k];
    let mut seven = 7;
    let mut ans = 1;
    loop {
        let remainder = seven % k;
        if remainder == 0 {
            println!("{ans}");
            return;
        }

        if appears[remainder] {
            break;
        }

        appears[remainder] = true;
        seven = remainder * 10 + 7;
        ans += 1;
    }

    println!("-1");
}
