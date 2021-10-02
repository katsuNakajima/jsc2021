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

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn cross<T>(v: &[T], w: &[T]) -> Vec<(T, T)>
where
    T: Clone,
{
    v.iter()
        .flat_map(|i| w.iter().map(move |j| (i.clone(), j.clone())))
        .collect()
}
fn main() {
    let (a, b) = parse_line!(i32, i32);
    let mut x: Vec<i32> = Vec::new();
    for i in a..b {
        x.push(i);
    }
    let mut y: Vec<i32> = Vec::new();
    for i in a + 1..b + 1 {
        y.push(i);
    }
    let mut ans:Vec<i32> = Vec::new();
    let cross = cross(&x, &y);
    for (i,j) in cross{
        ans.push(gcd(i,j));
    }
    let ans2 = ans.iter().max().unwrap();
    println!("{}",ans2);
}
