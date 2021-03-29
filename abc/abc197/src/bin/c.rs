use proconio::input;

fn bit_rows(n: u32) -> Vec<Vec<usize>> {
    let mut ret = vec![];
    for b in 0..2u64.pow(n) {
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
        an: [u64; n],
    }

    // 区切りを入れる入れないで全探索
    let mut ans = std::u64::MAX / 2;
    let brs = bit_rows(n as u32);
    for br in brs {
        if br.is_empty() {
            let mut cur_or = 0;
            for a in &an {
                cur_or |= *a;
            }
            ans = ans.min(cur_or);

            continue;
        }

        let mut cur_xor = 0;
        let mut cur_or = 0;
        let mut br_i = 0;
        for i in 0..n {
            cur_or |= an[i];
            if br_i < br.len() && i == br[br_i] {
                cur_xor ^= cur_or;
                br_i += 1;
                cur_or = 0;
            }
        }
        cur_xor ^= cur_or;
        ans = ans.min(cur_xor);
    }

    println!("{}", ans);
}
