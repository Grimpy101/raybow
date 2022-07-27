use super::renderable::Renderable;

pub struct Node {
    renderable: Option<Box<dyn Renderable>>
}

impl Node {
    pub fn new() -> Self {
        Node {
            renderable: None
        }
    }

    pub fn set_renderable(&mut self, r: Box<dyn Renderable>) {
        self.renderable = Some(r);
    }

    pub fn renderable(&self) -> &Option<Box<dyn Renderable>> {
        &self.renderable
    }
}