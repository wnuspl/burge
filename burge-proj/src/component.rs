use crate::*;
use element::Element;
use std::rc::Rc;

pub trait Component {
	fn build_element(&self) -> Element;
	fn to_any(&self) -> Rc<dyn std::any::Any>;
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
    pub fn access<T: 'static>(&self, name: &'static str, function: fn(Rc<T>)) {
        if let Some(component) = self.components.get(name) {
            if let Ok(casted) = component.to_any().downcast::<T>() {
                function(casted)
            }
        }
    }
}