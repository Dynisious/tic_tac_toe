
///This `Foo` type is a definition of a new struct.
pub struct Foo {
    /// This is the `foo` field of this struct, it's also public meaning it can be accessed like:
    ///
    /// #Examples
    ///
    /// ```
    /// let foo = Foo::new(0, Bar::default());
    ///
    /// println!("{}", foo.foo);
    /// ```
    pub foo: i16,
    bar: Bar
}

impl Foo {
    /// This is an example of a common convention in Rust.
    /// Even though you can constructor structs as shown in this constructor, its a
    /// convention to provide a constructor function called new like this. You can have
    /// as many constructor functions as you want of course for different parameters.
    pub fn new(foo: i16, temp: Bar) -> Self {
        //This shows the in-built constructor for a struct.
        Self {
            //Note that `foo` can just be written in but `bar` needs to be specified
            //since its value was named `temp`.
            foo,
            bar: temp
        }
    }
    pub fn get_bar(&self) -> &Bar {
        //Note here that `return` isn't being called to return a value.
        //A strange but infinetly useful feature of Rust is that, if the last line of a
        //code block has no semicolon that is the return value of the code block.
        &self.bar
    }
    pub fn set_bar(&mut self, bar: &Bar) {
        self.bar = bar.clone();
    }
    pub fn same_bar(a: &Self, b: Self) -> bool {
        //When you scroll down to the `Bar` struct later, notice that it derives `Eq`,
        //that defines that it can be compared with the `==` operator.
        a.bar == b.bar
    }
}

//This demonstrates the way to derive traits directly.
//traits define functions which you need to implement but some also provide default
//definitions that you can overwrite if you like.
//
//This trait itself however is very useful:
//  let foo = Foo::new(0, Bar::default());
//  let bar: &Bar = &foo;
//This trait lets this code compile, because if the compiler sees `&Foo` but needs `&Bar`
//it knows it can use this function to convert to what it needs.
impl AsRef<Bar> for Foo {
    fn as_ref(&self) -> &Bar {
        &self.bar
    }
}

//This demonstrates the way to derive traits implicitly.
//`PartialEq` defines the `partial_eq` function as a partial-order relation.
//`Eq` defines that this structs `partial_eq` function is actually a total-order relation
//  and can therefore be used by `==`.
//`Clone` defines a deep cloning function `clone` for making copies of this type.
#[derive(PartialEq, Eq, Clone)]
pub struct Bar {
    //Note the `'static` in this reference, this is a lifetime parameter which defines
    //how long a reference is allowed to stay alive for. Normally this all gets handled
    //under the hood for you, but sometimes necessary to specify. `'static` means that
    //the value of this reference is `static` and therefore this reference can live forever.
    pub bar: &'static str,
    foo: Bool
}

impl Bar {
    pub fn new(bar: &'static str, foo: Option<Bool>) -> Option<Self> {
        //This examplifies the strange `if let` in Rust.
        //We go over `match` later but in `match` you need to cover all cases, sometimes
        //you only care about one case. `if let` does exactly that by unpacking one enum
        //varient and getting its value.
        if let Some(foo) = foo {
            //Note that we don't call return from this `if` block to return this value.
            //Remember before when I mentioned that the last line of a code block can be
            //used to return a value? That's what is happening here!!! We haven't put a
            //semicolon at the end of this line, so the `if` block is returning our `Some`!!!
            Some(Self {
                bar,
                foo
            })
        } else {
            //This is just to demonstrate that there is a return call which will return a
            //value imediatly and break out of the function since we haven't used it yet.
            //We could return `None` from this else block and let that return from the
            //function because there is no semicolon after the `if/else` block.
            return None;
        }
    }
}

impl AsRef<str> for Bar {
    fn as_ref(&self) -> &str {
        &self.bar
    }
}

//This is another useful and common trait for defining a default value for a struct.
impl Default for Bar {
    fn default() -> Self {
        Self {
            bar: "default bar",
            foo: Bool::Blah(true)
        }
    }
}

//Note the new trait, Copy here. This specifies that although you can create a deep copy
//of this type, a shallow copy is also valid and safe. There isn't a `copy` function but
//it means you can pass a `Bool` object byval instead of byref without calling `clone`
//like in `set_bar` earlier.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Bool {
    True,
    False,
    Blah(bool)
}

impl Bool {
    fn boolify(self) -> bool {
        match self {
            True => true,
            False => false,
            _ => false
        }
    }
}

//`#[cfg(test)]` means that this code only compiles when you are testing the code.
//A module named `tests` gets run as tests when you run "cargo test".
#[cfg(test)]
mod tests {
    //This is useful to import all of the things from the `temp` module to be used in the tests.
    use super::*;
    
    fn not_a_test() {}

    //This function is a test which will be run because it starts with `#[test]`.
    #[test]
    fn it_works() {
        let bar: Bar = Bar::default();
        let mut foo = Foo::new(0, bar);
        foo.set_bar(&Bar::default());
        
        //The `assert` macro is used for unit testing, its first parameter tells whether the test passes.
        //Note that it gets called as `assert!` not `assert`. This i the same for all macros to tell them apart from functions.
        assert!(true, "This test passes.");
        
        assert!(false, "This test fails.");
    }
}
