use proconio::input;

fn main() {
    input! {
        n: usize,
        xyn: [(i32, i32); n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            // why WA
            // if xyn[j].1 - xyn[i].1 <= xyn[j].0 - xyn[i].0
            //     && xyn[j].1 - xyn[i].1 >= xyn[i].0 - xyn[j].0 {
            //     ans += 1;
            // }
            let dy = xyn[i].1 - xyn[j].1;
            let dx = xyn[i].0 - xyn[j].0;
            if dy * dy <= dx * dx {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
