use std::rc::Rc;

use uuid::Uuid;

use crate::{component::Component, element::{Element, ElementBase}, event::{Sender, Receiver, RoutedEvent, Router}};

#[derive(Copy,Clone)]
pub struct PhysObj {
	pub pos: [f32;2],
	pub shape: [f32;2],
	pub delta: [f32;2],
	pub settings: POSettings
}

impl PhysObj {
    pub fn intersects(&self, other: &PhysObj) -> bool {
        let this_left = self.pos[0];
        let this_right = self.pos[0]+self.shape[0];
        let this_bottom = self.pos[1];
        let this_top = self.pos[1]+self.shape[1];

        let other_left = other.pos[0];
        let other_right = other.pos[0]+other.shape[0];
        let other_bottom = other.pos[1];
        let other_top = other.pos[1]+other.shape[1];

        !(this_left > other_right || this_right < other_left || this_bottom > other_top || this_top < other_bottom)
    }
    pub fn exclusive_delta(&self, other: &Self) -> [f32;2] {
        let left = other.pos[0] - (self.pos[0]+self.shape[0]);
        let right = (other.pos[0] + other.shape[0]) - self.pos[0];
        let down = other.pos[1] - (self.pos[1]+self.shape[1]);
        let up = (other.pos[1] + other.shape[1]) - self.pos[1];

        let horizontal = if left.abs() > right.abs() {
            right
        } else {
            left
        };

        let vertical = if down.abs() > up.abs() {
            up
        } else {
            down
        };

        if horizontal > vertical {
            [horizontal, 0.0]
        } else {
            [0.0, vertical]
        }
    }
}




pub fn box_collider(pos: [f32;2], shape: [f32;2]) -> PhysObj {
    PhysObj {
        pos: pos,
        shape: shape,
        delta: [0.0,0.0],
        settings: POSettings::default()
    }
}


#[derive(Copy,Clone)]
pub struct POSettings {
	pub solid: bool,
	pub gravity_strength: f32,
    pub terminal_velocity: f32,
	pub on_collision: fn(&PhysObj, &PhysObj) -> Vec<PhysEvent>,
	pub on_each: fn(&PhysObj, &PhysObj) -> Vec<PhysEvent>
	//...
}

impl std::default::Default for POSettings {
    fn default() -> Self {
        Self {
            solid: true,
            gravity_strength: 1.0,
            terminal_velocity: -10.0,
            on_collision: |_,_| { Vec::new() },
            on_each: |_,_| { Vec::new() }
        }
    }
}

#[derive(Clone,Copy)]
pub enum PhysEvent {
    StaticPO(Uuid, PhysObj),
    DynamicPO(Uuid, PhysObj),

    RemoveStaticPO(Uuid),


    Collision(Uuid, PhysObj),

    PosDeltaRequest([f32;2]),
    VelocityDeltaRequest([f32;2])
}


#[derive(Clone)]
pub struct PhysObjManager {
    statics: Vec<(Uuid, PhysObj)>,
    receiver: Rc<Receiver<PhysEvent>>,
    priv_sender: Sender<PhysEvent>,

    router: Router<PhysEvent>,
    
    send_queue: Vec<RoutedEvent<PhysEvent>>,

    component: POMComponent
}

impl PhysObjManager {
    pub fn new() -> Self {
        let ps = Sender::new();
        let router = Router::new();

        Self {
            statics: Vec::new(),
            receiver: ps.new_receiver(),
            priv_sender: ps.clone(),

            router: router.clone(),

            send_queue: Vec::new(),

            component: POMComponent {
                priv_sender: ps,
                router: router
            }
        }
    }
}

#[derive(Clone)]
pub struct POMComponent {
    pub priv_sender: Sender<PhysEvent>,

    pub router: Router<PhysEvent>,
}
impl POMComponent {
    pub fn new_receiver(&self) -> (Uuid, Rc<Receiver<PhysEvent>>) {
        self.router.new_receiver()
    }
    pub fn new_sender(&self) -> Sender<PhysEvent> {
        self.priv_sender.clone()
    }
    pub fn new_receiver_uuid(&self, uuid: Uuid) -> (Uuid, Rc<Receiver<PhysEvent>>) {
        self.router.new_receiver_uuid(uuid)
    }
}

impl Component for PhysObjManager {
    fn name(&self) -> &'static str {
        "pom"
    }
    fn to_any(&self) -> &dyn std::any::Any {
        &self.component
    }
    fn build_element(&self) -> Element {
        Element::Element(Box::new(self.clone()))
    }
    fn load(&self, data: &serde_json::Map<String, serde_json::Value>) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}


impl ElementBase for PhysObjManager {
    fn init(&mut self, uuid: uuid::Uuid, components: &crate::component::ComponentManager) {
    }
    fn local_update(&mut self, td: f32) {
        
        let mut dynamics = Vec::new();
        for e in self.receiver.poll() {
            
            match e {
                PhysEvent::DynamicPO(uuid, po) => {
                    dynamics.push((uuid,po));
                },
                PhysEvent::StaticPO(uuid, po) => {
                    self.statics.push((uuid,po));
                },
                _ => ()
            }
        }

        for (uuid, d) in dynamics {
            self.send_queue.append(&mut
                self.individual((&uuid, &d), td)
            );
            for (s_uuid, s) in &self.statics {
                self.send_queue.append(&mut
                    self.interaction((&uuid, &d), (s_uuid, s), td)
                );
                
            }
        }
    }
    fn post_update(&mut self) {
        for e in self.send_queue.drain(..) {
            self.router.send(e);
        }
    }
}




impl PhysObjManager {
    fn individual(&self, this: (&Uuid, &PhysObj), td: f32) -> Vec<RoutedEvent<PhysEvent>> {
        let mut queue = Vec::new();

        let (uuid, this) = this;

        if this.settings.gravity_strength != 0.0 && this.delta[1] > this.settings.terminal_velocity  {
            queue.push(RoutedEvent(Some(*uuid), PhysEvent::VelocityDeltaRequest([0.0, GRAVITY*this.settings.gravity_strength*td])));
        }



        queue
    }
    fn interaction(&self, this: (&Uuid, &PhysObj), other: (&Uuid, &PhysObj), td: f32) -> Vec<RoutedEvent<PhysEvent>> {
        let mut queue = Vec::new();

        let (this_uuid, this) = this;
        let (other_uuid, other) = other;


        for e in (this.settings.on_each)(this,other) {
            queue.push(RoutedEvent(Some(*this_uuid), e))
        }
        
        if this.intersects(other) {
            queue.push(RoutedEvent(Some(*this_uuid), PhysEvent::Collision(*other_uuid, *other)));
            for e in (this.settings.on_collision)(this,other) {
                queue.push(RoutedEvent(Some(*this_uuid), e))
            }



            if this.settings.solid && other.settings.solid {
                let delta = this.exclusive_delta(other);

                
                queue.push(RoutedEvent(Some(*this_uuid), PhysEvent::PosDeltaRequest(delta)));
            }

            
            
        }

        



        queue
    }
}

const GRAVITY: f32 = -0.05;