use proconio::input;
use proconio::marker::Chars;

// wrong
fn can_create_n(v: &Vec<usize>, n: usize) -> bool {
    if v[n] > 0 {
        true
    } else {
        false
    }
}

fn main() {
    input! {
        n: usize,
        ss: String,
        x: Chars,
    }

    let sss = ss.chars().collect::<Vec<char>>();
    let mut s = vec![];
    for c in &sss {
        s.push(c.to_digit(10).unwrap() as usize);
    }
    // 0 を加える = 10 倍する = 7 の倍数であれば維持される
    // 今が 7n => Takahashi は 0 を足し Aoki は s[i] を足す
    // 今が 7n でない => Takahashi は S[i] を足し Aoki は 0 を足す
    // 今が 7n か判定 => 前の数を 10 倍してなにか足す (mod 7)
    // Takahashi は連続する n 手を見た場合にうまく 7n にできる可能性がある
    // Aoki も連続する n 手を見た場合に Takahashi が 7n にできない手を返せる可能性がある
    // すると互いに連続する分の手を見て mod を取っておけば良い？

    let mut tkhs_src = vec![];
    let mut aoki_src = vec![];
    let mut idx = 0;
    let mut cur = vec![0; 7];
    let mut is_t = x[0] == 'T';
    for i in 0..n {
        // ターン切替時に mod を作れる数のリストを作っておければは後はどうにか
        if is_t && x[i] == 'A' {
            tkhs_src.push(cur);
            is_t = false;
            cur = vec![0; 7];
        } else if !is_t && x[i] == 'T' {
            aoki_src.push(cur);
            is_t = true;
            cur = vec![0; 7];
        }
        cur[s[i] % 7] += 1;
    }

    let mut cur_mod = 0;
    let mut t_idx = 0;
    let mut a_idx = 0;
    let mut is_t = x[0] == 'T';
    while t_idx < tkhs_src.len() || a_idx < aoki_src.len() {
        if is_t {
            // 作れるなら 7n を作る
            t_idx += 1;
        } else {
            // 次の番で高橋が 7n にできない数字を返す
            a_idx += 1;
        }
        is_t = !is_t;
    }

    println!(
        "{}",
        if cur_mod == 0 {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}
