# burge

basic uniform rust game engine

---

### Introduction

Create a basic game loop
```rust
use burge::instance;

fn main() {
	let mut instance = instance::Instance::new();

	instance.run();
}
```
