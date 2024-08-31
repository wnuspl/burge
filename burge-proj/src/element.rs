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
	Generic(Box<dyn ElementBase>),
	Camera(Box<dyn Camera>),
	Entity(Box<dyn Entity>),
	Null
}



impl Element {
	pub fn init(&mut self, uuid: uuid::Uuid, components: &ComponentManager) {
		match self {
			Element::Generic(element) => element.init(uuid, components),
			Element::Entity(element) => element.init(uuid, components),
			Element::Camera(element) => element.init(uuid, components),
			_ => ()
		}
	}
	pub fn local_update(&mut self, td: f32) {
		match self {
			Element::Generic(element) => element.local_update(td),
			Element::Entity(element) => element.local_update(td),
			Element::Camera(element) => element.local_update(td),
			_ => ()
		}
	}
	pub fn post_update(&mut self) {
		match self {
			Element::Generic(element) => element.post_update(),
			Element::Entity(element) => element.post_update(),
			Element::Camera(element) => element.post_update(),
			_ => ()
		}
	}

	pub fn load(&self, data: &serde_json::Map<String, serde_json::Value>) -> Element {
		match self {
			Element::Generic(element) => element.load(data),
			Element::Entity(element) => element.load(data),
			Element::Camera(element) => element.load(data),
			_ => Element::Null
		}
	}
}









pub trait Entity: ElementBase {
	fn sprite(&self) -> sprite::Sprite;
}

pub trait Camera: ElementBase {
	fn clip_matrix(&self, window_size: [u32;2]) -> [[f32;3];3];
	fn offset(&self) -> [f32;2];
}


#[derive(Clone)]
pub struct DefaultCamera {
	pos: [f32;2],
	scale: f32,
	aspect: f32
}

impl DefaultCamera {
	pub fn new() -> Self {
		Self {
			pos: [0.0,0.0],
			scale: 12.0,
			aspect: 16./9.
		}
	}
}

impl ElementBase for DefaultCamera {
	fn load(&self, data: &serde_json::Map<String, serde_json::Value>) -> Element {
		Element::Camera(Box::new(self.clone()))
	}
}
impl Camera for DefaultCamera {
	fn clip_matrix(&self, window_size: [u32;2]) -> [[f32;3];3] {
		let aspect_ratio = window_size[0] as f32/window_size[1] as f32;
		let mut width = self.scale;
		let mut height = self.scale/self.aspect;
		if aspect_ratio > self.aspect { // more width
			width = height * aspect_ratio;
		} else if aspect_ratio < self.aspect {
			height = width / aspect_ratio;
		}
		[[2.0/width, 0.0, 0.0],
		[0.0, 2.0/height, 0.0],
		[0.0, 0.0, 1.0f32]]
	}
	fn offset(&self) -> [f32;2] {
		self.pos
	}
}