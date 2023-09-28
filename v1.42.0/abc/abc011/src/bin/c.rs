// 10min

use proconio::input;

fn main() {
    input! {
        n: usize,
        ng3: [usize; 3],
    }

    if ng3.iter().any(|&a| a == n) {
        println!("NO");
        return;
    }

    let mut cur_n = n;
    let mut cnt = 0;
    while cnt < 100 && cur_n != 0 {
        if cur_n > 2 && ng3.iter().all(|&a| a != cur_n - 3) {
            cur_n -= 3;
        } else if cur_n > 1 && ng3.iter().all(|&a| a != cur_n - 2) {
            cur_n -= 2;
        } else if ng3.iter().all(|&a| a != cur_n - 1) {
            cur_n -= 1;
        } else {
            println!("NO");
            return;
        }

        cnt += 1;
    }
    // println!("cur_n: {}", cur_n);
    // println!("cnt: {}", cnt);

    println!("{}", if cur_n == 0 { "YES" } else { "NO" });
}
