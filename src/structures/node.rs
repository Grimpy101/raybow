use super::renderable::Renderable;

pub struct Node {
    renderable: Option<Box<dyn Renderable + Send + Sync>>
}

impl Node {
    pub fn new() -> Self {
        Node {
            renderable: None
        }
    }

    pub fn set_renderable(&mut self, r: Box<dyn Renderable + Send + Sync>) {
        self.renderable = Some(r);
    }

    pub fn renderable(&self) -> &Option<Box<dyn Renderable + Send + Sync>> {
        &self.renderable
    }
}