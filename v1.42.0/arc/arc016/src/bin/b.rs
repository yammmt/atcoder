use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        xn9: [Chars; n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..9 {
            match xn9[i][j] {
                'x' => ans += 1,
                'o' => {
                    if i == 0 || xn9[i - 1][j] != 'o' {
                        ans += 1;
                    }
                }
                '.' => {}
                _ => unreachable!(),
            }
        }
    }

    println!("{}", ans);
}
