// :fu: :fu: 21-03

use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut n_keta = 0;
    let mut n_tmp = n;
    while n_tmp > 0 {
        n_keta += 1;
        n_tmp /= 10;
    }
    // println!("{}", n_keta);

    let mut ans = 0u64;
    let mut cur_keta = 1;
    while n_keta >= 2 * cur_keta {
        // println!("cur_keta: {}", cur_keta);
        if n_keta > 2 * cur_keta {
            ans += 9 * 10u64.pow(cur_keta - 1);
        } else {
            let mut front = n;
            let mut back = 0;
            let mut ten = 1;
            for _ in 0..cur_keta {
                back += (front % 10) * ten;
                front /= 10;
                ten *= 10;
            }
            ans += front - 10u64.pow(cur_keta - 1);
            if front <= back {
                ans += 1;
            }
        }
        // println!("{}", ans);

        cur_keta += 1;
    }

    println!("{}", ans);
}
