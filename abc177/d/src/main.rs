// -*- coding:utf-8-unix -*-

use proconio::input;

// NOT WORK

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut grp = 0;
    let mut vn = vec![0; n + 1];
    let mut vnactual = vec![];
    for i in 0..n + 1 {
        vnactual.push(i);
    }
    let mut vgrpnum = vec![0; n + 1];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        // println!("vn: {:?}", vn);
        // println!("vgrpnum: {:?}", vgrpnum);
        // println!("a, b: {} {}", a, b);
        if vn[a] == 0 && vn[b] == 0 {
            // new
            grp += 1;
            vn[a] = grp;
            vn[b] = grp;
            vgrpnum[grp as usize] = 2;
        } else if vn[a] != 0 && vn[b] != 0 && vn[a] != vn[b] {
            // merge
            vgrpnum[vn[a] as usize] += vgrpnum[vnactual[vn[b]]];
            vgrpnum[vnactual[vn[b]] as usize] = 0;
            // update grpnum
            vnactual[vn[b]] = vn[a];
        } else if vn[a] == 0 {
            vn[a] = vn[b];
            vgrpnum[vn[b] as usize] += 1;
        } else if vn[b] == 0 {
            vn[b] = vn[a];
            vgrpnum[vn[a] as usize] += 1;
        }
        // println!("vn: {:?}", vn);
        // println!("vgrpnum: {:?}\n", vgrpnum);
    }
    // println!("vn: {:?}", vn);
    // println!("vgrpnum: {:?}", vgrpnum);

    let mut ans = 0;
    for i in 0..vgrpnum.len() {
        ans = ans.max(vgrpnum[i]);
    }
    println!("{}", ans);
}
