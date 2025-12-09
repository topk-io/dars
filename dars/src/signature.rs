use std::fmt::Debug;

pub trait Signature: Sized + Debug + Send + Sync + 'static {
    type Input: Debug + Send + Sync + 'static;
    type Output: Debug + Send + Sync + 'static;
}
