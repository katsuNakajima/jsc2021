use std::collections::HashSet;

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let _ = parse_vec!(usize);
    let a = parse_vec!(i32);
    let b = parse_vec!(i32);
    let a_set: HashSet<i32> = a.into_iter().collect();
    let b_set: HashSet<i32> = b.into_iter().collect();
    let mut  ans = a_set.symmetric_difference(&b_set).collect::<Vec<&i32>>();
    ans.sort();
    for x in ans{
        println!("{}", x);
    }
}
