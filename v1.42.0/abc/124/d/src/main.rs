// -*- coding:utf-8-unix -*-

// https://drken1215.hatenablog.com/entry/2019/04/14/222900

use proconio::input;

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: String,
    }
    let vc = s.chars().collect::<Vec<char>>();

    let mut vv = vec![];
    if vc[0] == '0' {
        vv.push(0);
    }
    let mut current_str = 0;
    for i in 0..vc.len() {
        // println!("{}", vc[i]);
        if i > 0 && vc[i] != vc[i - 1] {
            vv.push(current_str);
            current_str = 1;
        } else {
            current_str += 1;
        }
        if i == vc.len() - 1 {
            vv.push(current_str);
            if vc[i] == '0' {
                vv.push(0);
            }
        }
    }
    // println!("vv: {:?}", vv);

    let mut csum = 0;
    let mut vsum = vec![0];
    for i in &vv {
        csum += *i;
        vsum.push(csum);
    }
    // println!("vsum: {:?}", vsum);

    let mut ans = 0;
    for left in 0..vsum.len() {
        if left % 2 == 1 {
            continue;
        }

        let right = (left + 2 * k + 1).min(vsum.len() - 1);
        // println!("l: {}, r: {}", left, right);
        ans = ans.max(vsum[right] - vsum[left]);
    }
    println!("{}", ans);
}
