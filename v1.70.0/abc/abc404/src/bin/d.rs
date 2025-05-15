use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        cn: [usize; n],
    }
    let mut km = Vec::with_capacity(m);
    let mut amk = Vec::with_capacity(m);
    for _ in 0..m {
        input! {
            k: usize,
            ak: [Usize1; k],
        }
        km.push(k);
        amk.push(ak);
    }

    let mut park_to_animals = vec![vec![]; n];
    for (i, ak) in amk.iter().enumerate() {
        for &a in ak {
            park_to_animals[a].push(i);
        }
    }

    let mut ans = usize::MAX / 3;
    for bit in 0..3usize.pow(n as u32) {
        let mut visited_count = vec![0; m];
        let mut b = bit;
        let mut cost_cur = 0;
        for i in 0..n {
            match b % 3 {
                0 => {}
                1 => {
                    for &a in &park_to_animals[i] {
                        visited_count[a] += 1;
                    }
                    cost_cur += cn[i];
                }
                2 => {
                    for &a in &park_to_animals[i] {
                        visited_count[a] += 2;
                    }
                    cost_cur += 2 * cn[i];
                }
                _ => unreachable!(),
            }
            b /= 3;
        }

        if visited_count.iter().all(|&v| v >= 2) {
            ans = ans.min(cost_cur);
        }
    }

    println!("{ans}");
}
