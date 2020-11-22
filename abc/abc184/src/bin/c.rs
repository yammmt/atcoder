use proconio::input;

fn main() {
    input! {
        rc1: (i64, i64),
        rc2: (i64, i64),
    }

    if rc1 == rc2 {
        println!("0");
    } else if rc1.0 + rc1.1 == rc2.0 + rc2.1 || rc1.0 - rc1.1 == rc2.0 - rc2.1 || (rc1.0 - rc2.0).abs() + (rc1.1 - rc2.1).abs() <= 3 {
        println!("1");
    } else{
        let xdiff = (rc1.0 - rc2.0).abs();
        let ydiff = (rc1.1 - rc2.1).abs();
        let mandiff = xdiff + ydiff;
        // println!("{}", xdiff);
        // println!("{}", ydiff);
        if xdiff < ydiff {
            if ydiff - xdiff <= 3 || mandiff % 2 == 0 {
                println!("2");
            } else {
                println!("3");
            }
        } else {
            if xdiff - ydiff <= 3 || mandiff % 2 == 0 {
                println!("2");
            } else {
                println!("3");
            }
        }

        // if mandiff % 2 == 0 {
        //     println!("2");
        // } else {
        //     println!("3");
        // }
    }
}
