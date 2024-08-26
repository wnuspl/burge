```rust
#[derive(Copy,Clone)]
struct RoutedEvent<T:Clone>(Uuid, T);

struct Router<T:Clone> {
	//...
}

impl<T:Clone> Router<T> {
	fn new() -> Self {
		//...
	}
	fn use_uuid(&mut self, uuid: Uuid) -> &Self {
		//...
	}
}

impl<T:Clone> EventSender<RoutedEvent<T>> for Router<T> {
	fn send(&self, event: RoutedEvent<T>) {
		//...
	}
	fn new_receiver(&self) {
		//...
	}
}
```

Special subset of [[Event Sender]]

Will save receivers with a [[Uuid]], and will only send events to receiver referenced in Uuid field of `RoutedEvent`

`use_uuid` method to give receiver a specific Uuid
```rust
fn main() {
	let router = Router::new();
	let player_uuid = Uuid::new_v4();
	let player_receiver = router.use_uuid(player_uuid).new_receiver();
}
```