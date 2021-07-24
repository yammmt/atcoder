use proconio::input;
use std::cmp::Ordering;
use std::collections::VecDeque;

static DUMMY: usize = std::usize::MAX / 4;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
    }
    let d = 1_000_000_007u64;

    let mut edges = vec![vec![]; n];
    for ab in &abm {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }

    let mut shortest_length = vec![DUMMY; n];
    shortest_length[0] = 0;
    let mut shortest_path_num = vec![0; n];
    shortest_path_num[0] = 1;
    let mut vdq = VecDeque::new();
    // 探索元の点
    vdq.push_back(0);
    while let Some(cur) = vdq.pop_front() {
        // println!("cur: {:?}", cur);
        for e in &edges[cur] {
            match shortest_length[*e].cmp(&(shortest_length[cur] + 1)) {
                Ordering::Less => {}
                Ordering::Equal => {
                    shortest_path_num[*e] = (shortest_path_num[*e] + shortest_path_num[cur]) % d
                }
                Ordering::Greater => {
                    shortest_path_num[*e] = shortest_path_num[cur];
                    shortest_length[*e] = shortest_length[cur] + 1;
                    vdq.push_back(*e);
                }
            }
        }
    }
    // println!("{:?}", shortest_length);
    // println!("{:?}", shortest_path_num);

    println!("{}", shortest_path_num[n - 1]);
}
