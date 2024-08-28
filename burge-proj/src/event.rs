use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use uuid::Uuid;


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
    pub fn poll(&self) -> Vec<T> {
        self.event_queue.borrow_mut().drain(..).collect::<Vec<T>>()
    }
}






#[derive(Clone)]
pub struct Sender<T:Clone> {
    receivers: Rc<RefCell<Vec<Rc<Receiver<T>>>>>
}

impl<T:Clone> Sender<T> {
    pub fn new() -> Self {
        Self {
            receivers: Rc::new(RefCell::new(Vec::new()))
        }
    }
    pub fn new_receiver(&self) -> Rc<Receiver<T>> {
        let r = Receiver::new();
        self.receivers.borrow_mut().push(r.clone());
        r
    }
    pub fn send(&self, event: T) {
        for r in self.receivers.borrow().iter() {
            r.receive(event.clone())
        }
    }
}




#[derive(Clone)]
pub struct RoutedEvent<T:Clone>(pub Option<Uuid>, pub T);

#[derive(Clone)]
pub struct Router<T:Clone> {
    receivers: Rc<RefCell<HashMap<Uuid, Rc<Receiver<T>>>>>
}


impl<T:Clone> Router<T> {
    pub fn new() -> Self {
		Self {
            receivers: Rc::new(RefCell::new(HashMap::new()))
        }
	}
	pub fn send(&self, event: RoutedEvent<T>) {
        if let RoutedEvent(Some(uuid), ref e) = event {
            if let Some(r) = self.receivers.borrow_mut().get(&uuid) {
                r.receive(e.clone())
            }
        } else {
            let RoutedEvent(_, ref e) = event;
            for r in self.receivers.borrow_mut().values() {
                r.receive(e.clone())
            }
        }
	}
    pub fn new_receiver_uuid(&self, uuid: Uuid) -> (Uuid, Rc<Receiver<T>>) {
        let r = Receiver::new();
        self.receivers.borrow_mut().insert(uuid, r.clone());
        (uuid, r)
    }
	pub fn new_receiver(&self) -> (Uuid, Rc<Receiver<T>>) {
        let uuid = Uuid::new_v4();
        let r = Receiver::new();
        self.receivers.borrow_mut().insert(uuid, r.clone());
        (uuid, r)
	}
}