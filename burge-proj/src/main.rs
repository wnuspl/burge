use burge_proj::{component::{Component, ComponentManager}, element::{Element, ElementBase}, event::{Receiver, RoutedEvent, Sender}, physics::{self, box_collider, POMComponent, PhysEvent, PhysObj, PhysObjManager}, scene::{JSONManager, SceneManager}};
use serde_json::Value;
use std::rc::Rc;


#[derive(Clone)]
struct Player {
    phys_receiver: Rc<Receiver<PhysEvent>>,
    phys_sender: Sender<PhysEvent>,

    uuid: uuid::Uuid,
    
    pos: [f32;2],
    velocity: [f32;2]
}

impl Player {
    pub fn new(pos: [f32;2]) -> Self {
        Self {
            phys_receiver: Receiver::new(),
            phys_sender: Sender::new(),

            uuid: uuid::Uuid::nil(),
            
            pos: pos,
            velocity: [0.0,0.0]
        }
    }
}

impl ElementBase for Player {
    fn init(&mut self, uuid: uuid::Uuid, components: &ComponentManager) {
        self.uuid = uuid;
        components.access("pom", |pom: &POMComponent| {
            println!("found pom");
            self.phys_sender = pom.new_sender();
            (_, self.phys_receiver) = pom.new_receiver_uuid(uuid);
        });


        self.phys_sender.send(PhysEvent::StaticPO(uuid::Uuid::new_v4(), box_collider([-0.0,-3.0], [3.0,1.0])));
    }
    fn load(&self, data: &serde_json::Map<String, serde_json::Value>) -> Element {
        Element::Element(Box::new(self.clone()))
    }
    fn local_update(&mut self, td: f32) {
        println!("x: {}, y: {}", self.pos[0], self.pos[1]);

        for e in self.phys_receiver.poll() {
            match e {
                PhysEvent::Collision(uuid, hb) => (),
                PhysEvent::PosDeltaRequest(delta) => {
                    if delta[0] > delta[1] {
                        self.velocity[0] = 0.0;
                    } else {
                        self.velocity[1] = 0.0;
                    }
                    self.pos[0] += delta[0];
                    self.pos[1] += delta[1];
                },
                PhysEvent::VelocityDeltaRequest(delta) => {
                    self.velocity[0] += delta[0];
                    self.velocity[1] += delta[1];
                },
                _ => ()
            }
        }
        self.pos[0] += self.velocity[0]*td;
        self.pos[1] += self.velocity[1]*td;
    }
    fn post_update(&mut self) {
        let mut hitbox = box_collider(self.pos, [1.0,1.0]);
        hitbox.delta = self.velocity;
        self.phys_sender.send(PhysEvent::DynamicPO(self.uuid, hitbox));
    }
}


fn main() {
    let mut scene_manager = SceneManager::new();


    let mut json_manager = JSONManager::default();

    json_manager.component_names.insert("pom".to_string(), Box::new(PhysObjManager::new()));
    json_manager.element_names.insert("player".to_string(), Element::Element(Box::new(Player::new([0.0,0.0]))));

    scene_manager.json_manager = json_manager;

    let data = r#"{
        "name": "main",
        "components": [],
        "elements": [
            {"name":"player"}
        ]
    }"#;
    let data: serde_json::Value = serde_json::from_str(data).unwrap();

    scene_manager.add_scene(data);

    scene_manager.scenes.get_mut("main").unwrap().init_elements();

    for i in 0..300 {
        scene_manager.scenes.get_mut("main").unwrap().update_elements(0.05);
    }
    print!("done");


    
}
