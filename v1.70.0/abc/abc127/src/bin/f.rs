use proconio::fastout;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DUMMY: isize = isize::MAX / 2;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    // 最小の x は
    // - 絶対値項が奇数個のときは中央値っぽい
    // - 偶数個のときは中央値候補のうち小さい方っぽい
    // 中央値を都度取得する方法なんてあったかしら, set?
    // => ChatGPT に投げると heap 二つでええやろとのこと, 私より賢い

    // 絶対値項をまとめることを考える. N 個の入力を受けたとして
    // - 小さい側は 中央値 x N/2 - 小さい側総和
    // - 大きい側は 大きい側総和 - 中央値 x N/2
    // あとはこれをちゃんと実装できればよさそう

    let mut heap_lower = BinaryHeap::new();
    let mut heap_upper = BinaryHeap::new();
    let mut lower_sum = 0;
    let mut upper_sum = 0;

    // 初回は 1 クエリ固定, 先に場合分けした方が実装が安全そう
    input! {
        _qq: usize,
        a: isize,
        b: isize,
    }
    // 要素数偶数個時は DUMMY とする
    let mut a_mid = a;
    let mut b_sum = b;

    for _ in 0..q - 1 {
        input! {
            qq: usize,
        }
        match qq {
            1 => {
                // 更新
                input! {
                    a: isize,
                    b: isize,
                }

                if a_mid == DUMMY {
                    // 更新クエリ奇数回目
                    let h_l = heap_lower.pop().unwrap();
                    lower_sum -= h_l;
                    let Reverse(h_u) = heap_upper.pop().unwrap();
                    upper_sum -= h_u;
                    let mut v = [h_l, h_u, a];
                    v.sort_unstable();
                    lower_sum += v[0];
                    a_mid = v[1];
                    upper_sum += v[2];
                    heap_lower.push(v[0]);
                    heap_upper.push(Reverse(v[2]));
                } else {
                    // 更新クエリ偶数回目, 片方空である可能性があるが取得クエリないので無視できる
                    let l = a_mid.min(a);
                    let h = a_mid.max(a);
                    lower_sum += l;
                    heap_lower.push(l);
                    upper_sum += h;
                    heap_upper.push(Reverse(h));
                    a_mid = DUMMY;
                }
                b_sum += b;
            }
            2 => {
                // 求値
                let x = if a_mid == DUMMY {
                    *heap_lower.peek().unwrap()
                } else {
                    a_mid
                };
                let sum_front = x * heap_lower.len() as isize - lower_sum;
                let sum_back = upper_sum - x * heap_upper.len() as isize;
                let ans = sum_front + sum_back + b_sum;
                println!("{x} {ans}");
            }
            _ => unreachable!(),
        }
    }
}
