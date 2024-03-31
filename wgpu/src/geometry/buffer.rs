use crate::graphics::mesh;
use lyon::tessellation;

pub struct Stack {
    solids: Vec<Solid>,
    gradients: Vec<Gradient>,
    current_solid: Option<Solid>,
    current_gradient: Option<Gradient>,
}

pub type Solid = tessellation::VertexBuffers<mesh::SolidVertex2D, u32>;
pub type Gradient = tessellation::VertexBuffers<mesh::GradientVertex2D, u32>;

impl Stack {
    pub fn new() -> Self {
        Self {
            solids: Vec::new(),
            gradients: Vec::new(),
            current_solid: None,
            current_gradient: None,
        }
    }

    pub fn solid_mut(&mut self) -> (&mut Solid, Option<Gradient>) {
        let gradient = self.current_gradient.take();

        if self.current_solid.is_none() {
            self.current_solid = Some(
                self.solids
                    .pop()
                    .unwrap_or_else(tessellation::VertexBuffers::new),
            );
        }

        (self.current_solid.as_mut().unwrap(), gradient)
    }

    pub fn gradient_mut(&mut self) -> (&mut Gradient, Option<Solid>) {
        let gradient = self.current_solid.take();

        if self.current_gradient.is_none() {
            self.current_gradient = Some(
                self.gradients
                    .pop()
                    .unwrap_or_else(tessellation::VertexBuffers::new),
            );
        }

        (self.current_gradient.as_mut().unwrap(), gradient)
    }

    pub fn finish(self) -> (Option<Solid>, Option<Gradient>) {
        (self.current_solid, self.current_gradient)
    }
}
