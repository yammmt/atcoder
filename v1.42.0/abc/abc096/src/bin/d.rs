// :fu:

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut is_prime = vec![true; 55556];
    let mut ansnum = 0;
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..is_prime.len() {
        if !is_prime[i] {
            continue;
        }

        if i % 5 == 1 {
            print!("{}", i);
            ansnum += 1;
            if ansnum == n {
                println!();
                return;
            } else {
                print!(" ");
            }
        }

        let mut pn = 2 * i;
        while pn < is_prime.len() {
            is_prime[pn] = false;
            pn += i;
        }
    }
}
