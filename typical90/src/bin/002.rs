use proconio::input;

fn bit_rows(n: usize) -> Vec<Vec<usize>> {
    let mut ret = vec![];
    for b in 0..2u64.pow(n as u32) {
        let mut cur = vec![];
        for i in 0..n {
            if b & (1 << i) > 0 {
                cur.push(i as usize);
            }
        }
        ret.push(cur);
    }
    ret
}

fn main() {
    input! {
        n: usize,
    }

    let rows = bit_rows(n);
    let mut ans = vec![];
    for r in rows {
        let mut cur = vec![];
        let mut j = 0;

        // 作る
        for i in 0..n {
            cur.push(if j < r.len() && r[j] == i {
                j += 1;
                '('
            } else {
                ')'
            });
        }

        // 正しいか判定
        let mut left_cnt = 0;
        for c in &cur {
            if *c == '(' {
                left_cnt += 1;
            } else {
                left_cnt -= 1;
                if left_cnt < 0 {
                    break;
                }
            }
        }
        if left_cnt == 0 {
            ans.push(cur.iter().collect::<String>());
        }
    }

    ans.sort_unstable();
    ans.iter().for_each(|a| println!("{}", a));
}
