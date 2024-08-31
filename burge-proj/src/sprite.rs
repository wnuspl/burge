use crate::Vertex;

pub struct Sprite {
    pos: [f32;2],
    scale: [f32;2],
    tex_indices: Vec<Vec<usize>>
}

impl Sprite {
    pub fn single(index: usize) -> Sprite {
        Self {
            pos: [0.0,0.0],
            scale: [1.0,1.0],
            tex_indices: vec![vec![index]]
        }
    }
    pub fn with_pos(mut self, pos: [f32;2]) -> Self {
        self.pos = pos;
        self
    }
    pub fn with_scale(mut self, scale: [f32;2]) -> Self {
        self.scale = scale;
        self
    }
}


pub struct SpriteSheet {
    pub tex: Option<glium::Texture2d>,
    pub shape: (usize,usize)
}

impl SpriteSheet {
    pub fn new(shape: (usize,usize)) -> Self {
        Self {
            tex: None,
            shape: shape
        }
    }
    pub fn vertices(&self, sprite: Sprite) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        let tex_unit = (1.0 / self.shape.0 as f32, 1.0 / self.shape.1 as f32);

        for row in 0..sprite.tex_indices.len() {
            for col in 0..sprite.tex_indices[row].len() {
                let this_idx = sprite.tex_indices[row][col];
                let tex_col = (this_idx%self.shape.0) as f32 * tex_unit.0;
                let tex_row = (this_idx/self.shape.0) as f32 * tex_unit.1;


                vertices.append(&mut vec![
                    Vertex { pos: [sprite.pos[0], sprite.pos[1], 1.0], tex_coords: [tex_col, tex_row] },
                    Vertex { pos: [sprite.pos[0] + sprite.scale[0], sprite.pos[1], 1.0], tex_coords: [tex_col+tex_unit.0, tex_row] },
                    Vertex { pos: [sprite.pos[0] + sprite.scale[0], sprite.pos[1] + sprite.scale[1], 1.0], tex_coords: [tex_col+tex_unit.0, tex_row+tex_unit.1] },

                    Vertex { pos: [sprite.pos[0], sprite.pos[1], 1.0], tex_coords: [tex_col, tex_row] },
                    Vertex { pos: [sprite.pos[0], sprite.pos[1] + sprite.scale[1], 1.0], tex_coords: [tex_col, tex_row+tex_unit.1] },
                    Vertex { pos: [sprite.pos[0] + sprite.scale[0], sprite.pos[1] + sprite.scale[1], 1.0], tex_coords: [tex_col+tex_unit.0, tex_row+tex_unit.1] },
                ]);
            }
        }


        vertices
    }
}