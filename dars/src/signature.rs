use std::fmt::Debug;

pub trait Signature: Sized + Debug {
    type Input: std::fmt::Debug;
    type Output;
}
