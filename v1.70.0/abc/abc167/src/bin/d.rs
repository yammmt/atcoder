use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

const DUMMY: usize = usize::MAX / 2;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        an: [Usize1; n],
    }

    let mut visited_nth = vec![DUMMY; n];
    let mut teleport_cnt = 0;
    let mut cur_town = 0;
    while teleport_cnt < k {
        if visited_nth[cur_town] != DUMMY {
            break;
        }

        visited_nth[cur_town] = teleport_cnt;
        teleport_cnt += 1;
        cur_town = an[cur_town];
    }

    if teleport_cnt == k {
        println!("{}", cur_town + 1);
        return;
    }

    let cycle_len = teleport_cnt - visited_nth[cur_town];
    let cycle_cnt = (k - visited_nth[cur_town]) / cycle_len;
    let teleport_remainder = k - visited_nth[cur_town] - cycle_len * cycle_cnt;
    for _ in 0..teleport_remainder {
        cur_town = an[cur_town];
    }
    println!("{}", cur_town + 1);
}
