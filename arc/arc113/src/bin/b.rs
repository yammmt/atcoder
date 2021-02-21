// :fu: :fu: 数問 21-02

use proconio::input;

// n^p mod m (繰り返し二乗法)
fn repeat_square(n: u64, p: u64, m: u64) -> u64 {
    if p == 0 {
        1
    } else if p == 1 {
        n % m
    } else if p % 2 == 0 {
        repeat_square(n, p / 2, m).pow(2) % m
    } else {
        (n * repeat_square(n, p - 1, m)) % m
    }
}

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    if a % 10 == 1 {
        println!("1");
        return;
    }

    let mut cycle = vec![999; 11];
    let mut cur = 1;
    let mut cycle_len = 1;
    loop {
        cur = (cur * a) % 10;
        // println!("{}", cur);
        if cycle[cur as usize] != 999 {
            break;
        }

        cycle[cur as usize] = cycle_len;
        cycle_len += 1;
    }
    cycle_len -= 1;
    // println!("{:?}", cycle);
    // println!("{}", cycle_len);

    let bcmod = repeat_square(b, c, cycle_len);
    // println!("{:?}", bcmod);

    for i in 1..11 {
        if cycle[i] != 999 && cycle[i] % cycle_len == bcmod {
            println!("{}", i);
            return;
        }
    }
}
