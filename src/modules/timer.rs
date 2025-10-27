use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerState {
    Idle,
    Running,
    Paused,
    Completed,
}

#[derive(Debug, Clone)]
pub struct Timer {
    state: TimerState,
    duration_seconds: u64,
    elapsed_seconds: u64,
    start_time: Option<Instant>,
    paused_at: Option<u64>,
    task_tag: Option<String>,
}

impl Timer {
    pub fn new(duration_minutes: u64) -> Self {
        Timer {
            state: TimerState::Idle,
            duration_seconds: duration_minutes * 60,
            elapsed_seconds: 0,
            start_time: None,
            paused_at: None,
            task_tag: None,
        }
    }
    
    pub fn start(&mut self) {
        match self.state {
            TimerState::Idle => {
                self.start_time = Some(Instant::now());
                self.state = TimerState::Running;
            }
            TimerState::Paused => {
                self.start_time = Some(Instant::now());
                self.state = TimerState::Running;
            }
            _ => {}
        }
    }
    
    pub fn pause(&mut self) {
        if self.state == TimerState::Running {
            if let Some(start) = self.start_time {
                self.elapsed_seconds += start.elapsed().as_secs();
                self.paused_at = Some(self.elapsed_seconds);
                self.state = TimerState::Paused;
            }
        }
    }
    
    pub fn reset(&mut self) {
        self.state = TimerState::Idle;
        self.elapsed_seconds = 0;
        self.start_time = None;
        self.paused_at = None;
        self.task_tag = None;
    }
    
    pub fn update(&mut self) -> bool {
        let mut completed = false;
        
        if self.state == TimerState::Running {
            if let Some(start) = self.start_time {
                let total_elapsed = self.elapsed_seconds + start.elapsed().as_secs();
                
                if total_elapsed >= self.duration_seconds {
                    completed = true;
                    self.state = TimerState::Completed;
                }
            }
        }
        
        completed
    }
    
    pub fn remaining_seconds(&self) -> u64 {
        let current_elapsed = match self.state {
            TimerState::Running => {
                if let Some(start) = self.start_time {
                    self.elapsed_seconds + start.elapsed().as_secs()
                } else {
                    self.elapsed_seconds
                }
            }
            TimerState::Paused => self.paused_at.unwrap_or(0),
            _ => 0,
        };
        
        if current_elapsed >= self.duration_seconds {
            0
        } else {
            self.duration_seconds - current_elapsed
        }
    }
    
    pub fn state(&self) -> TimerState {
        self.state
    }
    
    pub fn set_tag(&mut self, tag: String) {
        self.task_tag = Some(tag);
    }
    
    pub fn get_tag(&self) -> &Option<String> {
        &self.task_tag
    }
    
    pub fn duration_minutes(&self) -> u64 {
        self.duration_seconds / 60
    }
}
