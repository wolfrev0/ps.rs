unsafe fn solve(){
}
fn main(){
    let t:i32=read1();
    for i in 1..t+1{
        print!("Case #{}: ",i);
        unsafe{ solve() }
    }
}
fn read1<T>()->T where T:FromStr, <T as FromStr>::Err: fmt::Debug{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}
use std::fmt;
use std::io;
use std::cmp::*;
use std::ops::*;
use std::str::FromStr;