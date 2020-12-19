use proconio::input;
// use proconio::marker::Chars;
use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        t: usize,
        nskt: [(usize, usize, usize); t],
    }

    for nsk in &nskt {
        let mut visited = HashSet::new();
        let mut m = 1;
        let mut cnt = 0;
        loop {
            let bunshi = m * nsk.0 - nsk.1;
            let bunbo = nsk.2;
            if bunshi % bunbo == 0 {
                println!("{}", bunshi / bunbo);
                break;
            } else {
                if visited.contains(&(bunshi % bunbo)) {
                    println!("-1");
                    break;
                }
                visited.insert(bunshi % bunbo);
                m += 1;
                cnt += 1;
                // if cnt > 10 {
                //     println!("cnt err");
                //     break;
                // }
            }
        }

        // let mut ans = 1;
        // let mut cnt = 0;
        // loop {
        //     let cur = (nsk.0 * ans - nsk.1) % nsk.2;
        //     println!("cur: {}", cur);
        //     if cur == 0 {
        //         // TODO: ans 周期までに何回移動したかを出す
        //         println!("{}", ans);
        //         break;
        //     } else if visited.contains(&cur) {
        //         println!("-1");
        //         break;
        //     }
        //     visited.insert(cur);
        //     ans += 1;
        //     cnt += 1;
        //     if cnt > 100 {
        //         println!("cnt err");
        //         return;
        //     }
        // }
    }
}
