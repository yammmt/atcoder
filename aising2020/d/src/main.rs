// -*- coding:utf-8-unix -*-

// DOES NOT WORK

use proconio::input;

fn popcount(n: u64) -> u64 {
    let mut num_of_one = 0;
    for i in 0..n {
        let shifted = n >> i;
        if shifted == 0 {
            break;
        }

        if shifted & 1 == 1 {
            num_of_one += 1;
        }
    }
    num_of_one
}

#[test]
fn test_popcount() {
    assert_eq!(popcount(3), 2);
    assert_eq!(popcount(7), 3);
    assert_eq!(popcount(0), 0);
}

fn main() {
    input! {
        n: u64,
        s: String,
    }

    let x: Vec<char> = s.chars().collect();
    let mut ans = Vec::with_capacity((n + 1) as usize);
    for _i in 0..n + 1 {
        ans.push(-1);
    }

    for i in 1..n + 1 {
        if ans[i as usize] >= 0 {
            continue;
        }

        let mut x_now = x.clone();
        if x_now[(i - 1) as usize] == '0' {
            x_now[(i - 1) as usize] = '1';
        } else {
            x_now[(i - 1) as usize] = '0';
        }
        if x_now.iter().all(|c| *c == '0') {
            ans[i as usize] = 0;
            continue;
        }

        let mut calc_num = 0;
        // while !x_now.iter().all(|c| *c == '0') {
        loop {
            let x_num = u64::from_str_radix(&x_now.iter().collect::<String>(), 2).unwrap();
            let x_num_after = x_num % popcount(x_num);
            calc_num += 1;
            // if ans[x_num_after as usize] >= 0 {
            //     ans[i as usize] = calc_num + ans[x_num_after as usize];
            //     break;
            // } else if x_num_after == 0 {
            //     ans[i as usize] = calc_num;
            //     break;
            // }
            if x_num_after == 0 {
                ans[i as usize] = calc_num;
                break;
            }

            let s_tmp = format!("{:b}", x_num_after);
            x_now = s_tmp.chars().collect();
        }
    }

    for i in 1..n + 1 {
        println!("{}", ans[i as usize]);
    }
}
