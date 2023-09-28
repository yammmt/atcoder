// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let vs = s.chars().collect::<Vec<char>>();
    let mut s_cnt = vec![0; 9];
    for c in &vs {
        s_cnt[(*c as u8 - b'1') as usize] += 1;
    }

    if s.len() == 1 {
        if s == "8" {
            println!("Yes");
            return;
        } else {
            println!("No");
            return;
        }
    } else if s.len() == 2 {
        // TODO: 範囲変える以外は下と共通
        let mut eight_now = 16;
        while eight_now < 100 {
            let mut e_cnt = vec![0; 9];
            let mut en = eight_now;
            let mut is_fail = false;
            while en > 0 {
                let ept = en % 10;
                if ept == 0 {
                    is_fail = true;
                    break;
                }
                e_cnt[(ept - 1) as usize] += 1;
                en /= 10;
            }
            if is_fail {
                eight_now += 8;
                continue;
            }

            for i in 0..9 {
                if e_cnt[i] > s_cnt[i] {
                    break;
                } else if i == 8 {
                    // println!("e: {:?}", e_cnt);
                    println!("Yes");
                    return;
                }
            }
            eight_now += 8;
        }

        println!("No");
        return;
    }

    let mut eight_now = 104;
    while eight_now < 1000 {
        let mut e_cnt = vec![0; 9];
        let mut en = eight_now;
        let mut is_fail = false;
        while en > 0 {
            let ept = en % 10;
            if ept == 0 {
                is_fail = true;
                break;
            }
            e_cnt[(ept - 1) as usize] += 1;
            en /= 10;
        }
        if is_fail {
            eight_now += 8;
            continue;
        }

        for i in 0..9 {
            if e_cnt[i] > s_cnt[i] {
                break;
            } else if i == 8 {
                // println!("e: {:?}", e_cnt);
                println!("Yes");
                return;
            }
        }

        eight_now += 8;
    }
    println!("No");
}
