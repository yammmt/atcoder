use proconio::input;
use std::collections::BinaryHeap;

static DUMMY: isize = std::isize::MAX / 4;

fn main() {
    input! {
        n: usize,
        m: usize,
        abcdm: [(usize, usize, isize, isize); m],
    }

    let mut edges = vec![vec![]; n];
    for abcd in &abcdm {
        edges[abcd.0 - 1].push((abcd.1 - 1, (abcd.2, abcd.3)));
        edges[abcd.1 - 1].push((abcd.0 - 1, (abcd.2, abcd.3)));
    }

    let mut dist = vec![DUMMY; n];
    let mut heap = BinaryHeap::new();
    // cost, vertex
    heap.push((0, 0));
    while let Some(cur) = heap.pop() {
        println!("{}", cur.1);
        println!("  cur: {:?}", cur);
        println!("  {:?}", heap);
        if dist[cur.1] != DUMMY {
            println!("  continue");
            continue;
        }

        dist[cur.1] = -cur.0;
        for e in &edges[cur.1] {
            if dist[e.0] != DUMMY {
                continue;
            }

            // 待機時間 (D / (t + 1)) を線形探索したいが間に合う？
            // TLE はさておきサンプルは合ってほしいが
            let mut cost_d = (e.1).1 / (-cur.0 + 1);
            let mut cur_t = -cur.0;
            // println!("cost_d: {}", cost_d);
            while cost_d > 1 {
                let required_time = (e.1).1 / (cost_d - 1) - cur_t;
                // println!("  required_time: {}", required_time);
                if required_time <= 1 {
                    cost_d -= 1;
                    cur_t += required_time;
                } else {
                    break;
                }
            }
            heap.push((-(cost_d + (e.1).0 - cur.0), e.0));
            println!("  {:?}", heap);
        }
    }
    println!("{:?}", dist);

    println!(
        "{}",
        if dist[n - 1] == DUMMY {
            -1
        } else {
            dist[n - 1]
        }
    );
}
