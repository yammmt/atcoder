// -*- coding:utf-8-unix -*-

// WA
// サンプルあるものの長すぎてデバッグ苦しい
// 落ちたサンプルでデバッグビルド実行してもオーバーフロー/アンダーフロー (panic) は発生しない
// AtCoder では isize は 64bit, 入力文字列も最高で 10^5 でありキャストバグることもないはず
// まったくわからない

// 愚直なシミュレーションは aaaa...z から zzz... を作ると TLE

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let vs = s.chars().collect::<Vec<char>>();
    let vt = t.chars().collect::<Vec<char>>();
    let mut vsappear = vec![vec![]; 26];
    assert_eq!(s, vs.iter().collect::<String>());
    for i in 0..vs.len() {
        // println!("[{}] char: {}, insert to: {}", i, vs[i], (vs[i] as u8 - b'a') as usize);
        vsappear[(vs[i] as u8 - b'a') as usize].push(i);
    }
    for c in &vt {
        if vsappear[(*c as u8 - b'a') as usize].is_empty() {
            // 冗長な判定だが O(vt.len())
            println!("-1");
            return;
        }
    }

    // [a, z] の入力は拾えている
    let mut total_len = 0;
    for i in 0..26 {
        total_len += vsappear[i].len();
        // println!("{}: {:?}", i, vsappear[i]);
    }
    assert_eq!(total_len, vs.len());
    // return;

    let mut ans = 0u64;
    let mut vnextidx = vec![0usize; 26];
    // ex. in/out: a/a
    let mut last_idx = -1;
    let mut resultstr = String::new();
    for c in &vt {
        let c_as_usize = (*c as u8 - b'a') as usize;
        // println!("{}", c);
        // println!("c_as_usize: {}", c_as_usize);
        // vsappear[][] 最大値は 10^5 であり isize へのキャストは問題ない
        if vsappear[c_as_usize][vnextidx[c_as_usize]] as isize > last_idx {
            // ループする必要なし
            // last_idx は初回に限り -1, 残りはすべて [0, 10^5 - 1] であり
            // 上の条件と合わせて u64 へのキャストは失敗しない
            ans += (vsappear[c_as_usize][vnextidx[c_as_usize]] as isize - last_idx) as u64;
            // 配列要素外アクセスであれば RE になる
            last_idx = vsappear[c_as_usize][vnextidx[c_as_usize]] as isize;
            vnextidx[c_as_usize] = (vnextidx[c_as_usize] + 1) % vsappear[c_as_usize].len();
            resultstr.push(*c);
        } else {
            // last_idx より後ろにその字が存在しないならループする
            let mut is_loop = true;
            // vnextidx[c] は vsappear[c][] のインデックスであり範囲は誤っていない
            for i in vnextidx[c_as_usize]..vsappear[c_as_usize].len() {
                if vsappear[c_as_usize][i] as isize > last_idx {
                    is_loop = false;
                    vnextidx[c_as_usize] = i;
                    break;
                }
            }
            if is_loop {
                // 先頭に戻るまでに要した分を足しておく
                // println!("vsappear[c_as_usize]: {:?}", vsappear[c_as_usize]);
                ans += (vs.len() as isize - last_idx) as u64;
                vnextidx[c_as_usize] = 0;
                last_idx = 0;
            }
            ans += (vsappear[c_as_usize][vnextidx[c_as_usize]] as isize - last_idx) as u64;
            last_idx = vsappear[c_as_usize][vnextidx[c_as_usize]] as isize;
            vnextidx[c_as_usize] = (vnextidx[c_as_usize] + 1) % vsappear[c_as_usize].len();
            resultstr.push(*c);
        }
    }
    assert_eq!(resultstr, t);
    println!("{}", ans);
}
