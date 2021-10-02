#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

fn main() {
    let (x, y, z) = parse_line!(f64, f64, f64);
    if x == z {
        println!("{}", y as i32 - 1);
    } else {
        let a = y * z / x;
        let ans = a.ceil() as i32 - 1;
        println!("{}", ans);
    }
}
