use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        _n: usize,
        s5: [Chars; 5],
    }

    let num_ptrn = vec![
        [
            "###", ".#.", "###", "###", "#.#", "###", "###", "###", "###", "###",
        ],
        [
            "#.#", "##.", "..#", "..#", "#.#", "#..", "#..", "..#", "#.#", "#.#",
        ],
        [
            "#.#", ".#.", "###", "###", "###", "###", "###", "..#", "###", "###",
        ],
        [
            "#.#", ".#.", "#..", "..#", "..#", "..#", "#.#", "..#", "#.#", "..#",
        ],
        [
            "###", "###", "###", "###", "..#", "###", "###", "..#", "###", "###",
        ],
    ];

    let mut i = 1;
    while i + 2 < s5[0].len() {
        // println!("i: {i}");
        let cur0 = s5[0].iter().skip(i).take(3).collect::<String>();
        let cur1 = s5[1].iter().skip(i).take(3).collect::<String>();
        let cur2 = s5[2].iter().skip(i).take(3).collect::<String>();
        let cur3 = s5[3].iter().skip(i).take(3).collect::<String>();
        let cur4 = s5[4].iter().skip(i).take(3).collect::<String>();

        for j in 0..=9 {
            if cur0 == num_ptrn[0][j]
                && cur1 == num_ptrn[1][j]
                && cur2 == num_ptrn[2][j]
                && cur3 == num_ptrn[3][j]
                && cur4 == num_ptrn[4][j]
            {
                print!("{j}");
                break;
            }
        }

        i += 4;
    }
}
