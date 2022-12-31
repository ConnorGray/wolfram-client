# wolfram-client

Wolfram client library for Rust.

## Examples

Perform an evaluation by launching a Wolfram Kernel and entering an input:

```rust
use wolfram_client::{Packet, WolframSession};

let mut kernel = WolframSession::launch_default_kernel().unwrap();

// In[1]:=
let Packet::InputName(_) = kernel.packets().next().unwrap() else { panic!() };

// Compute the first 10 prime numbers.
kernel.enter_text("Prime[Range[10]]");

// Out[1]=
let Packet::OutputName(_) = kernel.packets().next().unwrap() else { panic!() };

let Packet::ReturnText(result) = kernel.packets().next().unwrap() else { panic!() };

assert_eq!(result, "{2, 3, 5, 7, 11, 13, 17, 19, 23, 29}");
```