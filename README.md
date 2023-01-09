# adventofcode2022
I'm trying to solve Advent of Code 2022 while learning the Rust language.  The goal is not "code golf" or speed.  It is to
learn to use Rust as if I were using it for work or an OSS project: handling errors, using modules and packages, and writing tests.

Each directory `day`_`n`_ is a crate with a library and a binary.
The website examples are used as unit test inputs.
Problem inputs are saved as `day`_`n`_`/input.txt`.
For example:

```
cd day1
cargo test
cargo run prob1 input.txt
cargo run prob2 input.txt
```

### Concepts I Learned, In Brief

#### Day 1
Vectors, and strings.  `String` vs `&str`. 

#### Day 2
Match.  Enums.

#### Day 3
HashSet

#### Day 4

"Classes", including a "constructor". The FromStr trait.
Using `?` and `map_err` to return an Err result.
Defining a custom Error type.

#### Day 5
Peekable iterators.  Converting from vector to iterator and back.  

#### Day 6
Basic use of Cargo. Separating a binary's main from a library.  How to unit test in Rust.  Applied all of those back to previous days solutions.  

HashSet.  Table-driven tests.

#### Day 7
Defining an enum that contains data. Defining an iterator.  Wrestled with lifetime specifiers for a good while.

Handling more complex parsing - using match over if/then seemed hard at first, but resulted in a satisfactory solution with patience.

#### Day 8

Defining a custom error type as an enum to provide details.
`fold()` method on iterators.
Defining an additional module within a crate -  [good reference](https://www.sheshbabu.com/posts/rust-module-system/)).
Working with nested vectors - error handling is tricky inside a double for loop. 
Defining a function that operates on iterators using type bounds on the iterator type and its item type.
Used crate "grid" which lets you iterate over a 2d-like array rowwise or columnwise.  Very cool how the lazy
iterators stack up to let you go in any direction.

#### Day 9
This one was annoying.
Used crate `cgmath`.

#### Day 10
Writing classes, or rather types with methods.

#### Day 11
Learned about when to use newtype vs type aliasing.  Chose type aliasing.

Learned different ways to handle an inner loop that wants
to modify two different indexes of a slice.  


#### Modifying two indexes of an array

One way is to use split_at_mut(), putting the two things
you want to change in different partitions.  For this problem, I knew which ones I wanted at different times, with differet lifetimes of the reference.  So things would have gotten complex.

The other way was to buffer all the needed changes in a temporary list, and then apply them in a different scope, so that I wasn't mutating two elements in the same scope.  This worked well.

### Longer Notes

#### Error Types
The examples for defining Error types that I saw don't include error details in the type ([example doc](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html))
Next time I write a library with multiple points that could return an error, it will help me to 
write it first using `.expect("detailed message")` everywhere, along with `cargo test -- --nocapture`.
Then, when all the test cases pass, change those sites  to  `?` or `map_err`, and then test for error return.

For users of a library, it seems like they may need more error details that a typename can provide.  The two examples of how to do this I could find was in the `std::net` library, which adds a `source()` method to a custom error type ([docs](https://doc.rust-lang.org/std/error/trait.Error.html#provided-methods)), and using an `enum` as the custom error type, where the enum inner type names are descriptive.


#### Invariant Checks in Constructors.
Some types, like the Range type I defined to solve day 4, required that one argument is greater than or equal to the other.  It seemed best to check this in the constructor.  The trick of adding a `_private: ()` member works to force callers to use `new`.  However,
I didn't know how the constructor should handle failure.  This was not covered in the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/ownership/constructors.html).  The [builder pattern](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html) didn't seem appropriate either.  Later I found that `std::num` uses the pattern `fn new(...) -> Option<SelfType>` ([example](https://doc.rust-lang.org/stable/std/num/struct.NonZeroUsize.html)).

#### Multi-binary vs Multi-crate
You can have multiple crates in one git repo.  This was cleaner than what I originally tries: having one crate for all 25 days, with multiple binaries defined in it.


