// 16min (1WA: 12min)
// WA: 2/29 の次の日が 2/30 になっていた

use proconio::input;

#[allow(clippy::many_single_char_names)]
fn main() {
    input! {
        s: String,
    }
    let v = s.split('/').collect::<Vec<&str>>();
    let mut y = v[0].parse::<u16>().unwrap();
    let mut m = v[1].parse::<u16>().unwrap();
    let mut d = v[2].parse::<u16>().unwrap();

    loop {
        if y % m == 0 && (y / m) % d == 0 {
            println!("{}/{:02}/{:02}", y, m, d);
            return;
        }

        match d {
            28 => match m {
                2 => match y % 4 == 0 && y % 100 != 0 {
                    true => d += 1,
                    false => {
                        m += 1;
                        d = 1;
                    }
                },
                _ => d += 1,
            },
            29 => match m {
                2 => {
                    m += 1;
                    d = 1;
                }
                _ => d += 1,
            },
            30 => match m {
                4 | 6 | 9 | 11 => {
                    m += 1;
                    d = 1;
                }
                _ => d += 1,
            },
            31 => match m {
                12 => {
                    y += 1;
                    m = 1;
                    d = 1;
                }
                _ => {
                    m += 1;
                    d = 1;
                }
            },
            _ => d += 1,
        }
    }
}
