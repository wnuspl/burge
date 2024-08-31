use burge_proj::{component::{Component, ComponentManager}, element::{DefaultCamera, Element, ElementBase, Entity}, event::{Receiver, RoutedEvent, Sender}, instance::{InputEvent, Instance}, physics::{self, box_collider, POMComponent, PhysEvent, PhysObj, PhysObjManager}, scene::{JSONManager, SceneManager}, sprite::{Sprite, SpriteSheet}, try_vec2, Vertex};
use serde_json::Value;
use uuid::Uuid;
use std::{default, rc::Rc};


#[derive(Clone)]
struct Player {
    phys_receiver: Rc<Receiver<PhysEvent>>,
    phys_sender: Sender<PhysEvent>,

    input_receiver: Rc<Receiver<InputEvent>>,

    uuid: uuid::Uuid,
    
    hb: PhysObj
}

impl Player {
    pub fn new(pos: [f32;2]) -> Self {
        Self {
            phys_receiver: Receiver::new(),
            phys_sender: Sender::new(),

            input_receiver: Receiver::new(),

            uuid: uuid::Uuid::nil(),
            
            
            hb: box_collider(pos, [1.0,1.0])
        }
    }
}

impl ElementBase for Player {
    fn init(&mut self, uuid: uuid::Uuid, components: &ComponentManager) {
        self.uuid = uuid;
        components.access("pom", |pom: &POMComponent| {
            self.phys_sender = pom.new_sender();
            (_, self.phys_receiver) = pom.new_receiver_uuid(uuid);
        });

        components.access("input", |input: &Sender<InputEvent>| {
            self.input_receiver = input.new_receiver();
        });

        self.hb.settings.gravity_strength = 0.0;
        self.hb.settings.on_collision = |a, b| {
            vec![]
        };
    }
    fn load(&self, data: &serde_json::Map<String, serde_json::Value>) -> Element {
        let mut default = self.clone();
        if let Some(pos) = data.get("pos") {
            default.hb.pos = try_vec2!(pos).unwrap();
        }
        Element::Entity(Box::new(default))
    }
    fn local_update(&mut self, td: f32) {
        for e in self.phys_receiver.poll() {
            match e {
                PhysEvent::ModPos(p) => {
                    self.hb.pos[0] += p[0];
                    self.hb.pos[1] += p[1];
                }
                PhysEvent::ModVelocity(d) => {
                    self.hb.delta[0] += d[0];
                    self.hb.delta[1] += d[1];
                }
                _ => ()
            }
        }


        for e in self.input_receiver.poll() {
            match e {
                InputEvent::KeyDown(keycode) => match keycode {
                    32 => self.hb.delta[0] = 0.05,
                    30 => self.hb.delta[0] = -0.05,
                    17 => self.hb.delta[1] = 0.05,
                    31 => self.hb.delta[1] = -0.05,
                    _ => ()
                },
                _ => ()
            }
        }

        self.hb.pos[0] += self.hb.delta[0]*td;
        self.hb.pos[1] += self.hb.delta[1]*td;
    }
    fn post_update(&mut self) {
        self.phys_sender.send(PhysEvent::DynamicPO(self.uuid, self.hb));
    }
}


impl Entity for Player {
    fn sprite(&self) -> Sprite {
        Sprite::single(0).with_pos(self.hb.pos)
    }
}






#[derive(Clone,Default)]
pub struct Block {
    pos: [f32;2],
    shape: [f32;2],

    phys_sender: Sender<PhysEvent>,
}

impl ElementBase for Block {
    fn load(&self, data: &serde_json::Map<String, serde_json::Value>) -> Element {
        let mut default = self.clone();
        if let Some(pos) = data.get("pos") {
            default.pos = try_vec2!(pos).unwrap();
        }
        if let Some(shape) = data.get("shape") {
            default.shape = try_vec2!(shape).unwrap();
        }
        Element::Entity(Box::new(default))
    }
    fn init(&mut self, uuid: uuid::Uuid, components: &ComponentManager) {
        components.access("pom", |pom: &POMComponent| {
            self.phys_sender = pom.new_sender();
        });


        self.phys_sender.send(PhysEvent::StaticPO(Uuid::nil(), box_collider(self.pos, self.shape)))
    }
}
impl Entity for Block {
    fn sprite(&self) -> Sprite {
        Sprite::single(16).with_pos(self.pos).with_scale(self.shape)
    }
}


fn main() {
    let mut instance = Instance::new();
    instance.ss_path = "./src/spritesheet.png";
    let mut json_manager = JSONManager::default();

    json_manager.component_names.insert("pom".to_string(), Box::new(PhysObjManager::new()));
    json_manager.component_names.insert("input".to_string(), Box::new(instance.input()));
    json_manager.element_names.insert("player".to_string(), Element::Entity(Box::new(Player::new([0.0,0.0]))));
    json_manager.element_names.insert("block".to_string(), Element::Entity(Box::new(Block::default())));
    json_manager.element_names.insert("default_camera".to_string(), Element::Camera(Box::new(DefaultCamera::new())));
    

    instance.scene_manager().json_manager = json_manager;

    let data = r#"{
        "name": "main",
        "components": [
            {"name": "input"}
        ],
        "elements": [
            {"name":"player", "pos": [0,1.5]},
            {"name":"block", "pos": [-2,-2], "shape": [3,1]}
        ]
    }"#;
    
    let data: serde_json::Value = serde_json::from_str(data).unwrap();

    instance.scene_manager().add_scene(data);
    instance.scene_manager().set_scene("main");


    instance.start();


    
}
