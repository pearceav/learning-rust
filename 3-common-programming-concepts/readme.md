## Variables and mutability

- variables are immutalbe by default - helps you write code in a way that takes advantage of the safety and easy concurrency that Rust offers.

## Data Types

- Integer overflow: occurs if you try to change or assign a variable to a value outside the range of values that the variable type can hold (u8 can hold between 0-255). It will _panic_ at runtime when compiling in degub mode. When compiling in release mode, Rust performs _two's compliment wrapping_, so values greater than the max value will wrap over and start at 0 (so 257 will be come 1)
    - to handle overflow possibility: wrap all modes with the `wrapping_*` methods, such as `wrapping_add`
    - Use `checked_*` to return `None` if there is overflow
    - Use `overflowing_*` to return the value and a boolean indicating whether there was overflow
    - Use `saturating_*` to saturate at the value's min or max values
- Floating point default is f64
- `f32` type is a single-precision float, `f64` has double precision



