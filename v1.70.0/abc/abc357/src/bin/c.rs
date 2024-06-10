use proconio::fastout;
use proconio::input;

// x, y: 始点
fn f(m: &mut Vec<Vec<char>>, x: usize, y: usize, level: usize) {
    if level == 0 {
        m[x][y] = '#';
        return;
    }

    let three_pow = 3usize.pow(level as u32 - 1);
    // 中央
    for i in 0..three_pow {
        for j in 0..three_pow {
            m[x + three_pow + i][y + three_pow + j] = '.';
        }
    }

    // それ以外
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }

            f(m, x + i * three_pow, y + j * three_pow, level - 1);
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    // とても厳しい, C で 40 行も書くのか
    // 点数も diff もほんと謎, Super Ryuma より圧倒的に嫌だが

    let n3 = 3usize.pow(n as u32);
    let mut ans = vec![vec!['Z'; n3]; n3];
    f(&mut ans, 0, 0, n);

    for a in &ans {
        for aa in a {
            print!("{aa}");
        }
        println!();
    }
}
