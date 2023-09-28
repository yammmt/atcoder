// 発想一発勝負類?

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    // white: '.' / black: '#'
    let mut ans = vec![vec!['.'; 100]; 100];
    for i in 50..100 {
        for j in 0..100 {
            ans[i][j] = '#';
        }
    }

    let mut cur_b = 1;
    if b != 1 {
        'outer_b: for i in 0..49 {
            if i % 2 != 0 {
                continue;
            }

            for j in 0..100 {
                if i % 2 == j % 2 {
                    ans[i][j] = '#';
                    cur_b += 1;
                    if cur_b == b {
                        break 'outer_b;
                    }
                }
            }
        }
    }
    let mut cur_w = 1;
    if a != 1 {
        'outer_w: for i in 51..100 {
            if i % 2 == 0 {
                continue;
            }

            for j in 0..100 {
                if i % 2 == j % 2 {
                    ans[i][j] = '.';
                    cur_w += 1;
                    if cur_w == a {
                        break 'outer_w;
                    }
                }
            }
        }
    }

    println!("100 100");
    for i in 0..100 {
        for j in 0..100 {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}
