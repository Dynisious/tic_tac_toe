//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2017/12/5

/// A `Square` can be either empty (None) or filled (Some(Mark)).
pub type Square = Option<Mark>;

/// `Mark`s are used to fill in squares as either `X` or `O`.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Mark {
    X, O
}
