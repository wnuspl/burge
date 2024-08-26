use std::rc::Rc;
use std::cell::RefCell;

pub trait Sender<T:Clone> {
    fn new_receiver(&self) -> Rc<Receiver<T>>;
    fn send(&self, event: T); // 
}

#[derive(Clone)]
pub struct Receiver<T:Clone> {
    event_queue: RefCell<Vec<T>>
}

impl<T:Clone> Receiver<T> {
    pub fn new() -> Rc<Receiver<T>> {
        Rc::new(Self{
            event_queue: RefCell::new(Vec::new())
        })
    }
    pub fn receive(&self, event: T) {
        self.event_queue.borrow_mut().push(event)
    }
}







pub struct EventSender<T:Clone> {
    receivers: Rc<RefCell<Vec<Rc<Receiver<T>>>>>
}

impl<T:Clone> Sender<T> for EventSender<T> {
    fn new_receiver(&self) -> Rc<Receiver<T>> {
        let r = Receiver::new();
        self.receivers.borrow_mut().push(r.clone());
        r
    }
    fn send(&self, event: T) {
        for r in self.receivers.borrow().iter() {
            r.receive(event.clone())
        }
    }
}