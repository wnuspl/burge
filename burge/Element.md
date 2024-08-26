Provides essential functionality to every element present in scene.
- Extra functionality through [[Component]]/[[Event Messaging System (EMS)]]

---

#### Code Representation

```rust
trait Element {
	fn init(&mut self, components: &ComponentManager);
	fn local_update(&mut self, td: f32);
	fn post_update(&mut self);
	
	fn save(&self) -> serde_json::Value;
	fn load(&self, data: serde_json::Value) -> Box<dyn Element>;
}
```

`init`: Called in [[Instance#Run]], or on instantiation
- Used to initialize [[Component]]
`local_update`: Called every tick
- updates to self
- reading events
`post_update`: Called every tick, after local update
- sending events