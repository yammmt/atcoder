use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h3: [isize; 3],
        w3: [isize; 3],
    }

    let mut ans = 0;
    for a in 1..=30 {
        for b in 1..=30 {
            for c in 1..=30 {
                for d in 1..=30 {
                    let mut cur = vec![vec![0; 3]; 3];
                    cur[0][0] = a as isize;
                    cur[0][1] = b as isize;
                    cur[1][0] = c as isize;
                    cur[1][1] = d as isize;

                    let mut vinserted = vec![];
                    for i in 0..2 {
                        let v = h3[i] - cur[i][0] - cur[i][1];
                        cur[i][2] = v;
                        vinserted.push(v);
                    }
                    for i in 0..3 {
                        let v = w3[i] - cur[0][i] - cur[1][i];
                        cur[2][i] = v;
                        vinserted.push(v);
                    }
                    let cur_by_w = cur[2][0] + cur[2][1] + cur[2][2];
                    if cur_by_w == h3[2] && vinserted.iter().all(|v| *v > 0) {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{ans}");
}
