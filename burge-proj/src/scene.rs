use std::collections::HashMap;

use uuid::Uuid;

use crate::{component::ComponentManager, element::Element};

#[derive(Default)]
pub struct Scene {
    camera_uuid: Option<Uuid>,
    pub elements: HashMap<Uuid, Element>,
    pub components: HashMap<Uuid, Box<dyn Component>>
}

impl Scene {
    pub fn init_elements(&mut self) {
        for (uuid, element) in &mut self.elements {
            element.init(*uuid, &ComponentManager::new());
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
    pub fn add_component(&mut self, component: Box<dyn Component>) {
        let uuid = Uuid::new_v4();
        self.elements.insert(uuid, component.build_element());
        self.components.insert(uuid, component);
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
    pub fn create_scene(&self, data: serde_json::Value) -> Scene {
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

                for c in components {
                    let component = load_object!(component_names,c);
                    if let Some(component) = component {
                        scene.add_component(component);
                    }
                }
            }

            if let Some(Value::Array(elements)) = fields.get("elements") {
                for e in elements {
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