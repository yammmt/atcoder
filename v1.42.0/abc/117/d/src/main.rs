// :fu: 21-04
// データの持ち方に不慣れ 実装が下手

use proconio::input;

fn n_to_bit_vec(mut n: u64) -> Vec<u64> {
    if n ==0 {
        return vec![0];
    }

    let mut ret = vec![];
    while n > 0 {
        ret.push(n % 2);
        n /= 2;
    }
    ret
}

fn main() {
    input! {
        n: usize,
        k: u64,
        an: [u64; n],
    }

    let mut kbits = n_to_bit_vec(k);
    for _ in 0..40 - kbits.len() {
        kbits.push(0);
    }
    kbits.reverse();
    // println!("k: {:?}", kbits);

    let mut bitcnt = vec![0; 40];
    for a in &an {
        let vb = n_to_bit_vec(*a);
        for i in 0..vb.len() {
            bitcnt[i] += vb[i];
        }
    }
    bitcnt.reverse();
    // println!("b: {:?}", bitcnt);

    let mut ans = 0;
    for i in 0..bitcnt.len() {
        // 上から i 桁目で k 以下と確定する
        // i 桁目が 0 ならスルー
        if kbits[i] == 0 {
            continue;
        }

        // println!("i: {}", i);
        let mut x = 0;
        for j in 0..bitcnt.len() {
            x *= 2;
            // println!("  j: {}, bitcnt: {}", j, bitcnt[j]);
            x += if j < i {
                kbits[j]
            } else if j != i && bitcnt[j] <= n as u64 / 2 {
                1
            } else {
                0
            };
        }
        // println!("  x: {}", x);

        if x > k {
            unreachable!();
        }

        let mut cur = 0;
        for a in &an {
            cur += x ^ *a;
        }
        ans = ans.max(cur);
    }
    // dirty
    let mut cur = 0;
    for a in &an {
        cur += k ^ *a;
    }
    ans = ans.max(cur);

    println!("{}", ans);
}
