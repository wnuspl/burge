```rust
trait EventSender<T> {
	fn new_receiver(&self) -> Rc<Receiver<T>>;
	fn send(&self, event: T);
}

struct Sender<T:Clone> {
	//...
}

impl<T:Clone> EventSender<T> for Sender<T> {}
```

`EventSender`: trait specifying functions of a sender
`Sender`: default sender