use proconio::input;

const POS_MAX: usize = 101;

fn main() {
    input! {
        n: usize,
        abcdn: [(usize, usize, usize, usize); n],
    }

    let mut is_covered = [[false; POS_MAX]; POS_MAX];
    for abcd in &abcdn {
        let a = abcd.0;
        let b = abcd.1;
        let c = abcd.2;
        let d = abcd.3;
        for x in a..b {
            for y in c..d {
                is_covered[x][y] = true;
            }
        }
    }

    let mut ans = 0;
    for x in 0..POS_MAX {
        for y in 0..POS_MAX {
            if is_covered[x][y] {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}
