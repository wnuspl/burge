pub mod element;
pub mod component;

pub mod event;

pub mod scene;

pub mod physics;
pub mod macros;
pub mod sprite;

pub mod instance;


#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: [f32;3],
    pub tex_coords: [f32;2]
}


glium::implement_vertex!(Vertex, pos, tex_coords);