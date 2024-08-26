Convey information and functionality about an object
- Shape
- Position
- Functionality: [[PO Settings]]

---
#### Code representation

```rust
#[derive(Copy,Clone)]
struct PhysicsObject {
	pos: [f32;2],
	shape: [f32;2],
	delta: [f32;2],
	settings: POSettings
}
```

