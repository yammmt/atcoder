// 21min

use proconio::input;

fn main() {
    input! {
        aa: u64,
        bb: u64,
    }

    let a = aa.min(bb);
    let b = aa.max(bb);

    let mut aprimes = vec![];
    let mut d = 1;
    while d * d <= a {
        if a % d == 0 {
            aprimes.push(d);
            if d != a / d {
                aprimes.push(a / d);
            }
        }
        d += 1;
    }

    let mut abprimes = aprimes.into_iter().filter(|i| b % i == 0).collect::<Vec<u64>>();
    abprimes.sort_unstable();

    let mut is_passed = vec![true; abprimes.len()];
    for i in 0..abprimes.len() {
        if !is_passed[i] || abprimes[i] == 1 {
            continue;
        }

        for j in i + 1..abprimes.len() {
            if abprimes[j] % abprimes[i] == 0 {
                is_passed[j] = false;
            }
        }
    }
    println!("{}", is_passed.into_iter().filter(|&a| a).count());
}
