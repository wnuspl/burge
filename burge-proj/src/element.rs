use crate::*;
use component::ComponentManager;

pub trait ElementBase {
	fn init(&mut self, components: &ComponentManager);
	fn local_update(&mut self, td: f32);
	fn post_update(&mut self);
	
	fn save(&self) -> serde_json::Value;
	fn load(&self, data: serde_json::Value) -> Element;
}

pub enum Element {
	Element(Box<dyn ElementBase>),
	Null
}