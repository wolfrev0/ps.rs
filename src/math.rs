use std::ops::Mul;

pub fn sq<T: Copy + Mul<Output = T>> (x:T) -> T { x*x }