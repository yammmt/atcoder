use proconio::input;

fn main() {
    input! {
        n: usize,
        mut pn: [usize; n],
    }

    let mut cpos = vec![0; n + 1];
    for i in 0..n {
        cpos[pn[i]] = i;
    }

    let mut ans = vec![];
    let mut is_moved = vec![false; n];
    for i in 0..n {
        if pn[i] != i + 1 {
            // println!("{:?}", pn);
            // println!("cpos: {}", cpos[i + 1]);
            for j in (i..cpos[i + 1]).rev() {
                // println!("cpos: {:?}", cpos);
                // println!("{:?}", is_moved);
                if is_moved[j] {
                    println!("-1");
                    return;
                }

                cpos.swap(j, j + 1);
                pn.swap(j, j + 1);

                // println!("moved {}", j + 1);
                ans.push(j + 1);
                is_moved[j] = true;
            }
        }
    }
    if is_moved.iter().take(n - 1).any(|b| !b) {
        println!("-1");
        return;
    }

    for a in &ans {
        println!("{}", a);
    }
}
