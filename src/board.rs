//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2017/12/5

use squares::*;

//I will give you a heads up because a few things here weren't in my examples.
//This is a `tuple struct`, meaning that we don't name the fields of the struct, we just throw them in there.
//  In this case there's only one field so we may as well use a tuple struct.
//  Note we still need to make the field public if we want to be able to access it.
//The field is a fixed array of fixed arrays of `Square`s.
pub struct Board(pub [[Square; 3]; 3]);

impl Board {
    /// Sets the value of the `Square` at `row`, `column`.
    ///
    /// #Params
    ///
    /// row --- The row of the `Square` to set.
    /// column --- The column of the `Square` to set.
    /// square --- The value to set.
    ///
    /// #James Note
    ///
    /// The `mut` keyword is needed because everything is `const` by default in Rust.
    /// `mut` means that the value, and its fields, can be modified.
    pub fn set_square(&mut self, row: usize, column: usize, square: Square) {
        self.0[row][column] = square;
    }
    /// Returns the winner of this game of tic-tac-toe:
    /// None --- No winner.
    /// Some(Mark::X) --- `X` is the winner.
    /// Some(Mark::O) --- `O` is the winner.
    pub fn winner(&self) -> Square {
        //This is a nice little macro to let the code compile and just crash at this
        //  point with a little message.
        unimplemented!("Todo message string.");
    }
}

impl Default for Board {
    fn default() -> Self {
        Board([[None; 3]; 3])
    }
}
