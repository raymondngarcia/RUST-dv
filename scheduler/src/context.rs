use crate::time::SimTime;

pub struct SimContext {
    time: SimTime,
}

impl SimContext {
    pub(crate) fn new() -> Self {
        Self { time: SimTime(0) }
    }

    pub fn time(&self) -> SimTime {
        self.time
    }

    pub(crate) fn advance(&mut self) {
        self.time.0 += 1;
    }
}
