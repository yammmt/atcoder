use proconio::input;

const ROULETTE_NUM: usize = 37;

fn main() {
    input! {
        n: usize,
    }

    let mut humans = vec![vec![]; ROULETTE_NUM];

    for i in 0..n {
        input! {
            c: usize,
            ac: [usize; c],
        }

        for a in &ac {
            // (掛けた個数, 掛けた人)
            humans[*a].push((c, i + 1));
        }
    }

    input! {
        x: usize,
    }

    let mut candidates = humans[x].clone();
    candidates.sort_unstable();
    let mut ans = vec![];
    for c in &candidates {
        if c.0 > candidates[0].0 {
            break;
        }
        ans.push(c.1);
    }

    println!("{}", ans.len());
    for a in &ans {
        if a == ans.last().unwrap() {
            println!("{a}");
        } else {
            print!("{a} ");
        }
    }
}
