use super::*;

pub struct DisplayedSpeaker {
    speaker: Speaker,
    side: Side,
}

pub struct VNScene {
    current_event: usize,
    events: Vec<VNEvent>,
    displayed_speakers: Vec<DisplayedSpeaker>,
}

impl VNScene {
    pub fn new() -> Self {
        Self {
            current_event: 0,
            events: Vec::new(),
            displayed_speakers: Vec::new(),
        }
    }

    pub fn from_events(events: Vec<VNEvent>) -> Self {
        Self {
            current_event: 0,
            events,
            displayed_speakers: Vec::new(),
        }
    }

    pub fn current(&self) -> Option<VNEvent> {
        if self.events.len() == 0 {
            return None;
        }

        Some(self.events[self.current_event].clone())
    }

    pub fn show_speaker(&mut self, speaker: Speaker, side: Side) {
        if let Some(mut existing) = self
            .displayed_speakers
            .iter_mut()
            .find(|s_display| s_display.speaker == speaker)
        {
            existing.side = side;
        } else {
            self.displayed_speakers
                .push(DisplayedSpeaker { speaker, side });
        }
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
