use proconio::input;

fn main() {
    input! {
        n: usize,
        xyn: [(i64, i64); n],
    }

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let dxj = xyn[j].0 - xyn[i].0;
                let dyj = xyn[j].1 - xyn[i].1;
                let dxk = xyn[k].0 - xyn[i].0;
                let dyk = xyn[k].1 - xyn[i].1;
                if dxj * dyk == dxk * dyj {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
