use std::collections::HashMap;
use std::hash::Hash;

use uuid::Uuid;

use crate::{component::ComponentManager, element::Element};

#[derive(Default)]
pub struct Scene {
    camera_uuid: Option<Uuid>,
    pub elements: HashMap<Uuid, Element>,
    pub component_manager: ComponentManager
}

impl Scene {
    pub fn init_elements(&mut self) {
        for (uuid, element) in &mut self.elements {
            element.init(*uuid, &self.component_manager);
        }
    }
    pub fn update_elements(&mut self, td: f32) {
        for (_, element) in &mut self.elements {
            element.local_update(td);
        }
        for (_, element) in &mut self.elements {
            element.post_update();
        }
    }
    pub fn add_component(&mut self, name: &'static str, component: Box<dyn Component>) {
        self.add_element(component.build_element());
        self.component_manager.add(name, component);
    }
    pub fn add_element(&mut self, element: Element) {
        let uuid = Uuid::new_v4();
        self.elements.insert(uuid, element);
    }
}




use crate::component::Component;

#[derive(Default)]
pub struct JSONManager {
    pub element_names: HashMap<String, Element>,
    pub component_names: HashMap<String, Box<dyn Component>>
}


impl JSONManager {
    pub fn create_scene(&self, data: serde_json::Value, mut default_components: Vec<serde_json::Value>, mut default_elements: Vec<serde_json::Value>) -> Scene {
        macro_rules! load_object {
            ($hashmap:ident, $c:tt) => {
                if let Value::Object(fields) = $c {
                    if let Some(Value::String(name)) = fields.get("name") {
                        if let Some(default) = self.$hashmap.get(name) {
                            Some(default.load(&fields))
                        } else { None }
                    } else { None }
                } else { None }
            };
        }

        use serde_json::Value;
        let mut scene = Scene::default();
        if let Value::Object(fields) = data {

            if let Some(Value::Array(components)) = fields.get("components") {
                let mut both = components.clone();
                both.append(&mut default_components);
                for c in both {
                    let component = load_object!(component_names,c);
                    if let Some(component) = component {
                        scene.add_component(component.name(), component);
                    }
                }
            }

            
            if let Some(Value::Array(elements)) = fields.get("elements") {
                let mut both = elements.clone();
                both.append(&mut default_elements);
                for e in both {
                    let element = load_object!(element_names,e);
                    if let Some(element) = element {
                        scene.add_element(element);
                    }
                }
            }
        }
        
        scene
    }
}



/*
[
	{
		"name": "main",
		"components": [],
		"elements": []
	}
]
*/






pub struct SceneManager {
    pub scenes: HashMap<String, Scene>,
    pub json_manager: JSONManager,
    default_components: Vec<serde_json::Value>,
    default_elements: Vec<serde_json::Value>
}

impl SceneManager {
    pub fn new() -> Self {

        let mut dc = vec![serde_json::from_str(r#"{"name":"pom"}"#).unwrap()];
        Self {
            scenes: HashMap::new(),
            json_manager: JSONManager::default(),
            default_components: dc,
            default_elements: Vec::new()
        }
    }
    pub fn add_scene(&mut self, data: serde_json::Value) {
        let mut scene_name = "".to_string();
        if let serde_json::Value::Object(ref fields) = data {
            if let Some(serde_json::Value::String(name)) = fields.get("name") {
                scene_name = name.clone();
            }
        }
        self.scenes.insert(scene_name, self.json_manager.create_scene(data, self.default_components.clone(), self.default_elements.clone()));
    }
}