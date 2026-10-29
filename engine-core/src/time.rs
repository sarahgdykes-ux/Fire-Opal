use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct DeltaTime(pub f32);

impl DeltaTime {
    pub fn as_secs_f32(self) -> f32 {
        self.0
    }

    pub fn as_secs(self) -> f64 {
        self.0 as f64
    }

    pub fn as_millis(self) -> u64 {
        (self.0 * 1000.0) as u64
    }
}

#[derive(Debug)]
pub struct Time {
    start_time: Instant,
    last_frame_time: Instant,
    delta_time: f32,
    fixed_time_step: f32,
    accumulator: f32,
    frame_count: u64,
    fps: f32,
    fps_timer: Instant,
    fps_frame_count: u64,
}

impl Time {
    pub fn new(fixed_time_step: f32) -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_frame_time: now,
            delta_time: 0.0,
            fixed_time_step,
            accumulator: 0.0,
            frame_count: 0,
            fps: 0.0,
            fps_timer: now,
            fps_frame_count: 0,
        }
    }

    pub fn tick(&mut self) -> DeltaTime {
        let now = Instant::now();
        let duration = now.duration_since(self.last_frame_time);
        self.delta_time = duration.as_secs_f32();
        self.last_frame_time = now;
        self.accumulator += self.delta_time;
        self.frame_count += 1;
        self.fps_frame_count += 1;

        let fps_duration = now.duration_since(self.fps_timer);
        if fps_duration >= Duration::from_secs(1) {
            self.fps = self.fps_frame_count as f32 / fps_duration.as_secs_f32();
            self.fps_frame_count = 0;
            self.fps_timer = now;
        }

        DeltaTime(self.delta_time)
    }

    pub fn delta_time(&self) -> DeltaTime {
        DeltaTime(self.delta_time)
    }

    pub fn fixed_time_step(&self) -> f32 {
        self.fixed_time_step
    }

    pub fn accumulator(&self) -> f32 {
        self.accumulator
    }

    pub fn consume_fixed_step(&mut self) -> bool {
        if self.accumulator >= self.fixed_time_step {
            self.accumulator -= self.fixed_time_step;
            true
        } else {
            false
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn reset_accumulator(&mut self) {
        self.accumulator = 0.0;
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new(1.0 / 60.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_delta_time() {
        let dt = DeltaTime(0.016);
        assert_eq!(dt.as_secs_f32(), 0.016);
        assert_eq!(dt.as_millis(), 16);
    }

    #[test]
    fn test_time_creation() {
        let time = Time::new(1.0 / 60.0);
        assert_eq!(time.fixed_time_step(), 1.0 / 60.0);
        assert_eq!(time.accumulator(), 0.0);
    }

    #[test]
    fn test_time_tick() {
        let mut time = Time::new(1.0 / 60.0);
        thread::sleep(Duration::from_millis(16));
        let dt = time.tick();
        assert!(dt.as_secs_f32() > 0.015);
        assert!(dt.as_secs_f32() < 0.02);
    }

    #[test]
    fn test_fixed_step_consumption() {
        let mut time = Time::new(1.0 / 60.0);
        time.accumulator = 0.02;

        assert!(time.consume_fixed_step());
        assert!((time.accumulator() - (0.02 - 1.0 / 60.0)).abs() < 1e-6);

        assert!(!time.consume_fixed_step());
    }
}
