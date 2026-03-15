# HDL-RS

HDL-RS is a framework for hardware definition in Rust.

## Goals of this framework

- **Statically Checked Rules**: Prevents unwanted latches and checks clock domain crossing at compile time.
- **Type Safety**: Doesn't perform implicit casts between types.
- **Excellent Testability**: Testbenches are simply Rust `#[test]` functions.
- **Structured Module I/O**: Supports `struct`s and `enum`s as module inputs and outputs.

## Workflow

Users implement functions that construct modules, which can call other functions to build composite modules. These modules can then be converted to Verilog or simulated directly.

## Examples

A simple module definition:

```rust
// Module with 2 8-bit inputs `a` and `b`, and one 8-bit output.
fn module<A: InputBus<8>, B: InputBus<8>>(
    a: InputBusWrapper<8, A>,
    b: InputBusWrapper<8, B>,
) -> impl Bus<8> {
    // Bitwise operators are implemented for buses.
    let c = a & b | a ^ !b;

    // Take 3 wires from the bus starting from index 3.
    let left = c.sub_bus::<3, 1>();
    // Take a wire at index 3 and invert it.
    let middle = !a.wire_at::<3>();
    // Create a bus with 4 grounded wires.
    let right = ConstBus::new([LogicalWireState::Zero; 4]);

    // Concatenate all 3 buses.
    concat_buses!(left, middle, right)
}
```

For more examples see the [Examples folder](https://github.com/mertwole/hdl-rs/tree/main/examples/src/bin)

## License

[GNU General Public License](https://github.com/mertwole/hdl-rs/blob/main/LICENSE)