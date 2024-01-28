use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xm: [Usize1; m],
    }

    // 長さの最小値を答える => 元の長さを極力維持する
    // 元の最短ルートで橋を通る回数はいもす法で出せるが, 橋を落とされた時のペナルティがわからない
    // 一つ一つ実際に落として数えると O(NM) で TLE
    // 関係が連続的でないので二分探索ではなさそう
    // ペナルティをいもす法でもてる？もてそう

    // 最後を除いて島 i と i+1 をつなぐ
    let mut imos = vec![0isize; n];
    let mut shortest = 0;
    for i in 1..m {
        let bigger = xm[i].max(xm[i - 1]);
        let smaller = xm[i].min(xm[i - 1]);
        let to_big = bigger - smaller;
        let to_small = smaller + n - bigger;
        shortest += to_big.min(to_small);
        if to_big < to_small {
            // e.g. 1 -> 3 (N = 10)
            let pena = (to_small - to_big) as isize;
            imos[smaller] += pena;
            imos[bigger] -= pena;
        } else {
            let pena = (to_big - to_small) as isize;
            // 始点から最後まで
            imos[bigger] += pena;
            // 最初から終点まで
            imos[0] += pena;
            imos[smaller] -= pena;
        }
    }

    let mut ans = isize::MAX / 2;
    let mut cusum = shortest as isize;
    for im in &imos {
        cusum += im;
        ans = ans.min(cusum);
    }

    println!("{}", ans);
}
