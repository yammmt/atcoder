// s を二つ並べるとこに気付くか問題なのにそこの解説が

use proconio::input;

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: String,
    }
    let mut cur = s.chars().collect::<Vec<char>>();
    // println!("{:?}", vc);

    for _ in 0..k {
        let mut c = cur.clone();
        cur.append(&mut c);

        let mut cc = vec![];
        for i in 0..cur.len() / 2 {
            if cur[2 * i] == 'R' {
                if cur[2*i+1] == 'P' {
                    cc.push('P');
                } else {
                    cc.push('R');
                }
            } else if cur[2 * i] == 'P' {
                if cur[2*i+1] == 'S' {
                    cc.push('S');
                } else {
                    cc.push('P');
                }
            } else if cur[2*i+1] == 'R' {
                cc.push('R');
            } else {
                cc.push('S');
            }
        }
        cur = cc;
    }
    println!("{}", cur[0]);
}
