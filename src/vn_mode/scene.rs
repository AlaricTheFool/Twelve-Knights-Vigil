use super::*;

pub struct VNScene {
    current_event: usize,
    events: Vec<VNEvent>,
}

impl VNScene {
    pub fn new() -> Self {
        Self {
            current_event: 0,
            events: Vec::new(),
        }
    }

    pub fn from_events(events: Vec<VNEvent>) -> Self {
        Self {
            current_event: 0,
            events,
        }
    }

    pub fn current(&self) -> Option<VNEvent> {
        if self.events.len() == 0 {
            return None;
        }

        Some(self.events[self.current_event].clone())
    }
}

impl Iterator for VNScene {
    type Item = VNEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_event == self.events.len() - 1 {
            return None;
        }

        self.current_event += 1;
        Some(self.events[self.current_event].clone())
    }
}
