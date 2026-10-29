// GH: rebuildup | AtCoder: rebuild | Data use: unrestricted
// github.com/rebuildup/atcoder-kit/tree/dbe1a0f4bc7a

use std::io::*;
fn read<T:std::str::FromStr>()->T{
stdin().lock().bytes().map(|b|b.unwrap() as char).skip_while(|c|c.is_whitespace()).take_while(|c|!c.is_whitespace()).collect::<String>().parse().ok().unwrap()
}
fn main(){}

// Notes / refs:
// -
