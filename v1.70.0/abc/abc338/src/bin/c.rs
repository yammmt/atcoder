use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        qn: [usize; n],
        an: [usize; n],
        bn: [usize; n],
    }

    // 再帰 DFS しようにも 10^6 通りの選び方が 10 回降ってくる
    // 料理は二種類しかないので片方の回数を決め打ちすればよい？

    let mut ans = 0;
    let mut i = 0;
    loop {
        let mut a_by_i = vec![0; n];
        let mut could_continue = true;
        for j in 0..n {
            a_by_i[j] = an[j] * i;
            if a_by_i[j] > qn[j] {
                could_continue = false;
                break;
            }
        }
        if !could_continue && i != 0 {
            break;
        }

        let mut q_rest = qn.clone();
        for j in 0..n {
            q_rest[j] -= a_by_i[j];
        }

        let mut b_max = usize::MAX / 2;
        for j in 0..n {
            if bn[j] == 0 {
                continue;
            }

            b_max = b_max.min(q_rest[j] / bn[j]);
        }
        ans = ans.max(i + b_max);
        i += 1;
    }

    println!("{ans}");
}
