use crate::context::SimContext;

#[derive(Clone, Copy, Debug)]
pub enum Phase {
    Drive,
    Sample,
}

pub trait SimComponent {
    fn on_phase(&mut self, phase: Phase, ctx: &mut SimContext);
}
