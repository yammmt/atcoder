use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut km = Vec::with_capacity(m);
    let mut amkm = Vec::with_capacity(m);
    for _ in 0..m {
        input! {
            kk: usize,
            a: [Usize1; kk],
        }
        km.push(kk);
        amkm.push(a);
    }
    input! {
        bn: [Usize1; n],
    }

    // 操作逆, すべての食材を克服したところからだめにしていく
    // 全日程をかけて全食材を克服する

    let mut ans = vec![];
    let mut could_eat = (0..m).collect::<HashSet<usize>>();
    let mut ingredient_to_food = vec![vec![]; n];
    for (i, ak) in amkm.iter().enumerate() {
        for &a in ak {
            ingredient_to_food[a].push(i);
        }
    }

    for &b in bn.iter().rev() {
        ans.push(could_eat.len());
        // println!("could_eat: {:?}", could_eat);
        for f in &ingredient_to_food[b] {
            // println!("rm: {f}");
            could_eat.remove(f);
        }
    }

    for a in ans.iter().rev() {
        println!("{a}");
    }
}
