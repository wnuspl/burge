use crate::*;
use component::ComponentManager;

pub trait ElementBase {
	fn init(&mut self, uuid: uuid::Uuid, components: &ComponentManager) {}
	fn local_update(&mut self, td: f32) {}
	fn post_update(&mut self) {}
	
	fn save(&self) -> serde_json::Value { serde_json::Value::Null}
	fn load(&self, data: &serde_json::Map<String, serde_json::Value>) -> Element { Element::Null }
}

pub enum Element {
	Element(Box<dyn ElementBase>),
	Null
}



impl Element {
	pub fn init(&mut self, uuid: uuid::Uuid, components: &ComponentManager) {
		match self {
			Element::Element(element) => element.init(uuid, components),
			_ => ()
		}
	}
	pub fn local_update(&mut self, td: f32) {
		match self {
			Element::Element(element) => element.local_update(td),
			_ => ()
		}
	}
	pub fn post_update(&mut self) {
		match self {
			Element::Element(element) => element.post_update(),
			_ => ()
		}
	}

	pub fn load(&self, data: &serde_json::Map<String, serde_json::Value>) -> Element {
		match self {
			Element::Element(element) => element.load(data),
			_ => Element::Null
		}
	}
}