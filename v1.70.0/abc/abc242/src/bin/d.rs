use proconio::input;
use proconio::marker::Chars;

fn st_k(s: &Vec<char>, t: usize, k: usize) -> char {
    let vc = ['A', 'B', 'C'];
    if k == 0 {
        let i = match s[0] {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => unreachable!(),
        };
        vc[(i + (t % 3)) % 3]
    } else if t == 0 {
        s[k]
    } else {
        let prev = st_k(s, t - 1, k / 2);
        let i = match prev {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => unreachable!(),
        };
        if k % 2 == 0 {
            vc[(i + 1) % 3]
        } else {
            vc[(i + 2) % 3]
        }
    }
}

fn main() {
    input! {
        s: Chars,
        q: usize,
        tkq: [(usize, usize); q],
    }

    for tk in tkq {
        let t = tk.0;
        let k = tk.1 - 1;
        println!("{}", st_k(&s, t, k));
    }
}
