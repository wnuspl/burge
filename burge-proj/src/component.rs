use crate::*;
use element::Element;
use std::rc::Rc;

pub trait Component {
	fn build_element(&self) -> Element {
        Element::Null
    }
	fn to_any(&self) -> &dyn std::any::Any;
    fn load(&self, data: &serde_json::Map<String, serde_json::Value>) -> Box<dyn Component>;
}



use std::collections::HashMap;
pub struct ComponentManager {
    components: HashMap<String, Box<dyn Component>>
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: HashMap::new()
        }
    }
    pub fn add(&mut self, name: &'static str, component: Box<dyn Component>) -> Element {
        let element = component.build_element();
        self.components.insert(name.to_string(), component);
        element
    }
    pub fn access<T: 'static>(&self, name: &'static str, mut function: impl FnMut(&T)) {
        if let Some(component) = self.components.get(name) {
            if let Some(casted) = component.to_any().downcast_ref::<T>() {
                function(casted)
            }
        }
    }
}