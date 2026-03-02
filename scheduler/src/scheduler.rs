use crate::{
    component::{Phase, SimComponent},
    context::SimContext,
};

pub trait SimBackend {
    fn eval(&mut self);
}

pub struct Scheduler {
    components: Vec<Box<dyn SimComponent>>,
    ctx: SimContext,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            ctx: SimContext::new(),
        }
    }

    pub fn add<C>(&mut self, component: C)
    where
        C: SimComponent + 'static,
    {
        self.components.push(Box::new(component));
    }

    pub fn run<B>(&mut self, backend: &mut B, cycles: u64)
    where
        B: SimBackend,
    {
        for _ in 0..cycles {
            self.step(backend);
        }
    }

    pub fn step<B>(&mut self, backend: &mut B)
    where
        B: SimBackend,
    {
        // DRIVE
        for comp in &mut self.components {
            comp.on_phase(Phase::Drive, &mut self.ctx);
        }

        // Hardware step
        backend.eval();

        // SAMPLE
        for comp in &mut self.components {
            comp.on_phase(Phase::Sample, &mut self.ctx);
        }

        self.ctx.advance();
    }
}
