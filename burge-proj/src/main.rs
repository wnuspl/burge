use burge_proj::{component::{Component, ComponentManager}, element::{Element, ElementBase}, event::{Receiver, RoutedEvent, Sender}, physics::{self, POMComponent, PhysEvent, PhysObj, PhysObjManager}, scene::JSONManager};
use serde_json::Value;


#[derive(Clone)]
struct Player;
impl ElementBase for Player {
    fn load(&self, data: &serde_json::Map<String, serde_json::Value>) -> Element {
        Element::Element(Box::new(self.clone()))
    }
}


fn main() {
    let mut json_manager = JSONManager::default();

    json_manager.component_names.insert("pom".to_string(), Box::new(PhysObjManager::new()));
    json_manager.element_names.insert("player".to_string(), Element::Element(Box::new(Player)));

    let data = r#"{
        "components": [
            {"name":"pom"}
        ],
        "elements": [
            {"name":"player"}
        ]
    }"#;
    let data: serde_json::Value = serde_json::from_str(data).unwrap();
    
    let my_scene = json_manager.create_scene(data);

    print!("{}", my_scene.components.len());
    print!("{}", my_scene.elements.len());


    
}
