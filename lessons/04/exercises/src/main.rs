//! You can use this file for experiments.

///  TODO (bonus): write a simple "DSL" (domain-specific language) that can
/// be used to test the ringbuffer in a more visual way.
/// For example, you could create a function that "renders" the ringbuffer
/// to a string like this:
/// ```
/// [1, 2, 3, 1, 3, _, _, _, _, 1, 2, 3, 3, 2]
/// _            ^ end          ^ start
/// ```
/// and then you can rewrite the tests to assert that the ringbuffer
/// has a specific state represented by that string. This would make
/// checking that the ringbuffer behaves as you expect much easier.
/// The render function can assume that the buffer only contains unsigned
/// integers smaller than 10, to simplify the rendering code.

// need a completely different implementation of ring buffer to achieve the wrap around 
// style string representation 

fn main() {
    println!("Hello Rust!");
}
