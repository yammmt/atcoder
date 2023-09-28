use proconio::input;

const DUMMY: usize = std::usize::MAX / 3;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        hn: [usize; n],
        ck: [usize; k],
        abm: [(usize, usize); m],
    }

    // 高いところから順に探索？
    // 低い避難所から登っていける限り避難可能にしていく方が計算量的に楽そう

    // 下から上がって行くので問題文とは逆
    let mut edges = vec![vec![]; n];
    for ab in &abm {
        if hn[ab.0 - 1] < hn[ab.1 - 1] {
            edges[ab.0 - 1].push(ab.1 - 1);
        } else {
            edges[ab.1 - 1].push(ab.0 - 1);
        }
    }

    let mut is_goal = vec![false; n];
    ck.iter().for_each(|c| is_goal[c - 1] = true);

    let mut vh = vec![];
    for (i, h) in hn.iter().enumerate() {
        vh.push((*h, i));
    }
    vh.sort_unstable();

    // TODO: Option 使うとかっこよさそう
    let mut cost = vec![DUMMY; n];
    for hh in &vh {
        if is_goal[hh.1] {
            cost[hh.1] = 0;
        }

        if cost[hh.1] == DUMMY {
            continue;
        }

        for &v in &edges[hh.1] {
            cost[v] = cost[v].min(cost[hh.1] + 1);
        }
    }

    for i in 0..n {
        println!(
            "{}",
            if cost[i] != DUMMY {
                cost[i] as isize
            } else {
                -1
            }
        );
    }
}
