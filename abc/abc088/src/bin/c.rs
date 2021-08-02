use proconio::input;

fn main() {
    input! {
        c33: [[i16; 3]; 3],
    }

    let b3 = [c33[0][0], c33[0][1], c33[0][2]];
    let a3 = [0, c33[1][1] - b3[1], c33[2][2] - b3[2]];
    for i in 0..3 {
        for j in 0..3 {
            if a3[i] + b3[j] != c33[i][j] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
