```rust
#[derive(Copy,Clone)]
struct POSettings {
	solid: bool,
	gravity_strength: f32,
	on_collision: fn(&PhysicsObject, &PhysicsObject) -> Vec<PhysicsEvent>,
	on_each: fn(&PhysicsObject, &PhysicsObject) -> Vec<PhysicsEvent>
	//...
}
```

#### Solidity
When `solid` is true, the [[Physics Object Manager (POM)]] will attempt to stop collisions between other `solid` objects.

#### Gravity
Global acceleration and terminal velocity are scaled by `gravity_strength`
- Applied every tick in system

#### Custom Functionality
User implemented methods to add new functionality to physics objects.
Called by [[Physics Object Manager (POM)]] (first parameter is self)
- `on_collision` called when colliders are found intersecting
- `on_each` called for every collider processed in a tick
