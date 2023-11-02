use std::io;

fn main() {
    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("fail");
    let n: u32 = n.trim().parse().expect("fail");
    let a: u32 = n / 5;
    for i in (0..a+1).rev() {
        if (n - (5 * i)) % 3 == 0 {
            print!("{}", i + ((n - (i * 5)) / 3));
            return();
        }
    }
    print!("-1");
}