// 55min 2WA (22.5/43.5)
// WA: 貪欲に文字を選ぶと例えば残り使用回数が [4, 6] で二文字消費するパターンが一つ、
// 四文字消費するパターンが二つ残っていた場合に先頭の 4 から 2 を引いてしまって No 判定

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [Chars; h],
    }

    let mut ccnt = vec![0; 26];
    for i in 0..h {
        for j in 0..w {
            ccnt[(ahw[i][j] as u8 - b'a') as usize] += 1;
        }
    }

    let mut required_one = if h % 2 == 1 && w % 2 == 1 {
        1
    } else {
        0
    };
    let mut required_two = if h % 2 == 0 && w % 2 == 0 {
        0
    } else if h % 2 == 1 && w % 2 == 1 {
        h / 2 + w / 2
    } else if h % 2 == 1 {
        w / 2
    } else {
        //  w % 2 == 1
        h / 2
    };
    let mut required_four = if h % 2 == 0 && w % 2 == 0 {
        h / 2 * w / 2
    } else if h % 2 == 1 && w % 2 == 1 {
        (h - 1) / 2 * (w - 1) / 2
    } else if h % 2 == 1 {
        (h - 1) / 2 * w / 2
    } else {
        //  w % 2 == 1
        h / 2 * (w - 1) / 2
    };
    // println!("{:?}", ccnt);
    // println!("one: {}",required_one);
    // println!("two: {}",required_two);
    // println!("four: {}",required_four);

    let mut pass = true;
    if required_one > 0 {
        for i in 0..ccnt.len() {
            if ccnt[i] % 2 == 1 {
                ccnt[i] -= 1;
                required_one -= 1;
                break;
            }
        }
        if required_one != 0 {
            pass = false;
        }
    }
    if ccnt.iter().any(|a| *a % 2 == 1) {
        println!("No");
        return;
    }
    // println!("one: {:?}", ccnt);
    let mut f_i = 0;
    if pass && required_four > 0 {
        while required_four > 0 && f_i != ccnt.len() {
            if ccnt[f_i] >= 4 {
                ccnt[f_i] -= 4;
                required_four -= 1;
            } else {
                f_i += 1;
            }
        }
        if required_four != 0 {
            pass = false;
        }
    }
    // println!("four: {:?}", ccnt);
    let mut t_i = 0;
    if pass && required_two > 0 {
        while required_two > 0 && t_i != ccnt.len() {
            if ccnt[t_i] >= 2 {
                ccnt[t_i] -= 2;
                required_two -= 1;
            } else {
                t_i += 1;
            }
        }
        if required_two != 0 {
            pass = false;
        }
    }
    // println!("two: {:?}", ccnt);

    println!(
        "{}",
        match pass {
            true => "Yes",
            false => "No",
        }
    );
}
