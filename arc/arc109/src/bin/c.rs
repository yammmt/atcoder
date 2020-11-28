use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: String,
    }
    let vorgc = s.chars().collect::<Vec<char>>();
    let mut vc = vorgc.clone();

    let vtwo = [2, 4, 8, 16, 32, 64, 128];
    // 手が参加者に対して多すぎる場合
    while vc.len() > 2u128.pow(k as u32) as usize {
        vc.pop();
    }
    if vc.len() < k {
        // 参加者に対して手が足りない場合
        let mut i = s.len();
        while !vtwo.contains(&vc.len()) {
            vc.push(vorgc[i % n]);
            i += 1;
        }
    }
    // println!("{:?}", vc);

    let mut cur = vc.clone();
    while cur.len() != 1 {
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
            } else {
                if cur[2*i+1] == 'R' {
                    cc.push('R');
                } else {
                    cc.push('S');
                }
            }
        }
        cur = cc;
    }
    println!("{}", cur[0]);

    // println!("{}", winner(0, s.len(), &vc));
}
