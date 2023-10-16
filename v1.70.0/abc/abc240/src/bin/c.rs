use proconio::input;

static X_MAX: usize = 10001;

fn main() {
    input! {
        n: usize,
        x: usize,
        abn: [(usize, usize); n],
    }

    let mut could_stay = vec![false; X_MAX];
    could_stay[0] = true;
    for ab in &abn {
        let mut could_stay_next = vec![false; X_MAX];
        for i in 0..X_MAX {
            if !could_stay[i] {
                continue;
            }

            let i_next_a = i + ab.0;
            let i_next_b = i + ab.1;
            if i_next_a < X_MAX {
                could_stay_next[i_next_a] = true;
            }
            if i_next_b < X_MAX {
                could_stay_next[i_next_b] = true;
            }
        }
        could_stay = could_stay_next;
    }

    println!("{}", if could_stay[x] { "Yes" } else { "No" });
}
