/// Deterministic, frame-accurate timeline for offline 2D animation playback.
///
/// This module provides the core time-control mechanism for the engine.  
/// It does not perform any rendering or I/O; it only tracks a frame index  
/// driven by wall‑clock deltas and a constant frame rate.  
///
/// The timeline is completely offline and never interacts with any network.
/// All state changes are initiated by the human through the editor UI.

#[derive(Debug, Clone)]
pub struct Timeline {
    /// Frames per second. Must be > 0.
    frame_rate: u32,
    /// Total number of frames in the current animation segment.
    /// 0 means an empty timeline (no content).
    total_frames: u32,
    /// Current playback position (0‑based frame index).
    /// Guaranteed to be in `[0, total_frames)` when `total_frames > 0`,
    /// otherwise always 0.
    current_frame: u32,
    /// Whether the timeline is advancing on each tick.
    playing: bool,
    /// When true, playback wraps from the last frame back to frame 0.
    looping: bool,
    /// Fractional time accumulated since the last whole frame step.
    time_accumulator: f64,
    /// Pre‑computed duration of a single frame in seconds.
    frame_duration: f64,
}

impl Timeline {
    /// Creates a new timeline with the given frame rate and total frames.
    ///
    /// # Panics
    /// Panics if `frame_rate` is 0.
    pub fn new(frame_rate: u32, total_frames: u32) -> Self {
        assert!(frame_rate > 0, "frame_rate must be greater than 0");
        let frame_duration = 1.0 / frame_rate as f64;
        Self {
            frame_rate,
            total_frames,
            current_frame: 0,
            playing: false,
            looping: false,
            time_accumulator: 0.0,
            frame_duration,
        }
    }

    // ------------------------------------------------------------------
    //  Query methods
    // ------------------------------------------------------------------

    /// Returns the current frame index (0‑based).
    #[inline]
    pub fn current_frame(&self) -> u32 {
        self.current_frame
    }

    /// Returns the total number of frames in the timeline.
    #[inline]
    pub fn total_frames(&self) -> u32 {
        self.total_frames
    }

    /// Returns the playback frame rate.
    #[inline]
    pub fn frame_rate(&self) -> u32 {
        self.frame_rate
    }

    /// Returns `true` if the timeline is actively playing.
    #[inline]
    pub fn is_playing(&self) -> bool {
        self.playing
    }

    /// Returns `true` if playback wraps around.
    #[inline]
    pub fn is_looping(&self) -> bool {
        self.looping
    }

    /// Returns the current playback time in seconds.
    #[inline]
    pub fn current_time_seconds(&self) -> f64 {
        self.current_frame as f64 * self.frame_duration
    }

    /// Returns `true` if the timeline is non‑empty and `current_frame`
    /// has reached the last frame. Useful for determining when a
    /// non‑looping playback has completed.
    #[inline]
    pub fn is_at_end(&self) -> bool {
        self.total_frames > 0 && self.current_frame == self.total_frames - 1
    }

    // ------------------------------------------------------------------
    //  Modification methods
    // ------------------------------------------------------------------

    /// Begins or resumes playback.
    ///
    /// If the timeline is at the end of a non‑looping animation, calling
    /// `play()` will restart from frame 0.
    pub fn play(&mut self) {
        if !self.looping && self.is_at_end() {
            self.current_frame = 0;
            self.time_accumulator = 0.0;
        }
        self.playing = true;
    }

    /// Pauses playback without changing the current position.
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// Stops playback, resets the current frame to 0, and clears the
    /// time accumulator.
    pub fn stop(&mut self) {
        self.playing = false;
        self.current_frame = 0;
        self.time_accumulator = 0.0;
    }

    /// Jumps to an absolute frame index.
    ///
    /// The value is clamped to the valid range `[0, total_frames - 1]`.
    /// If `total_frames == 0`, the frame is forced to 0.
    pub fn seek(&mut self, frame: u32) {
        if self.total_frames == 0 {
            self.current_frame = 0;
        } else {
            self.current_frame = frame.min(self.total_frames - 1);
        }
        self.time_accumulator = 0.0;
    }

    /// Advances the timeline by `delta_seconds` if playing.
    ///
    /// This method implements deterministic, frame‑accurate stepping:
    /// it accumulates the elapsed time and moves forward by one frame
    /// for every full frame duration that has passed.
    ///
    /// If looping is enabled, the timeline wraps around seamlessly.
    /// If looping is disabled and the last frame is reached, playback
    /// stops automatically.
    ///
    /// The function returns the number of frame steps that occurred,
    /// which may be zero if insufficient time has passed or if the
    /// timeline is not playing.
    pub fn advance_by_time(&mut self, delta_seconds: f64) -> u32 {
        if !self.playing || self.total_frames == 0 || delta_seconds <= 0.0 {
            return 0;
        }

        self.time_accumulator += delta_seconds;
        let mut steps = 0u32;

        // Process whole frames as long as enough time has accumulated.
        while self.time_accumulator >= self.frame_duration {
            self.time_accumulator -= self.frame_duration;
            steps += 1;

            // Move forward or wrap.
            if self.current_frame + 1 < self.total_frames {
                self.current_frame += 1;
            } else if self.looping {
                self.current_frame = 0;
            } else {
                // Non‑looping: stay on last frame and stop playback.
                self.current_frame = self.total_frames - 1;
                self.playing = false;
                // Discard any remaining accumulated time so that
                // future playback starts cleanly.
                self.time_accumulator = 0.0;
                break;
            }
        }

        steps
    }

    /// Sets the frame rate. Existing time accumulation is preserved
    /// (the absolute playback time does not jump), but the conversion
    /// from time to frames will use the new rate on subsequent ticks.
    ///
    /// # Panics
    /// Panics if `frame_rate` is 0.
    pub fn set_frame_rate(&mut self, fps: u32) {
        assert!(fps > 0, "frame_rate must be greater than 0");
        self.frame_rate = fps;
        self.frame_duration = 1.0 / fps as f64;
    }

    /// Changes the total number of frames in the timeline.
    ///
    /// If the current frame is now outside the valid range, it is clamped.
    /// If `total_frames` is set to 0, `current_frame` is forced to 0.
    pub fn set_total_frames(&mut self, total: u32) {
        self.total_frames = total;
        if total == 0 {
            self.current_frame = 0;
        } else if self.current_frame >= total {
            self.current_frame = total - 1;
        }
        // Time accumulator is unaffected so that a subsequent play
        // will continue from the clamped position with correct timing.
    }

    /// Enables or disables looping.
    pub fn set_looping(&mut self, looping: bool) {
        self.looping = looping;
        // If we are currently at the end and looping is turned on,
        // playback may resume when advance is called; no additional logic needed.
    }

    /// Resets the timeline to its initial state (stopped, frame 0,
    /// accumulator zero). Equivalent to calling `stop()`.
    pub fn reset(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: run several ticks with a fixed delta.
    fn tick_many(tl: &mut Timeline, delta: f64, count: usize) {
        for _ in 0..count {
            tl.advance_by_time(delta);
        }
    }

    #[test]
    fn new_timeline_is_stopped_at_zero() {
        let tl = Timeline::new(24, 100);
        assert_eq!(tl.current_frame(), 0);
        assert!(!tl.is_playing());
    }

    #[test]
    fn play_advances_frames() {
        let mut tl = Timeline::new(10, 50);
        tl.play();
        // With 10 fps, one frame lasts 0.1s.
        tl.advance_by_time(0.1);
        assert_eq!(tl.current_frame(), 1);
        assert!(tl.is_playing());
    }

    #[test]
    fn fractional_time_accumulates_correctly() {
        let mut tl = Timeline::new(10, 50);
        tl.play();
        // Give 0.05 s – not enough for a frame.
        tl.advance_by_time(0.05);
        assert_eq!(tl.current_frame(), 0);
        // Another 0.05 s should complete the frame.
        tl.advance_by_time(0.05);
        assert_eq!(tl.current_frame(), 1);
    }

    #[test]
    fn non_looping_stops_at_end() {
        let mut tl = Timeline::new(10, 3); // frames 0,1,2
        tl.play();
        tick_many(&mut tl, 0.1, 3); // three frame steps
        assert_eq!(tl.current_frame(), 2);
        assert!(!tl.is_playing()); // stopped automatically
    }

    #[test]
    fn looping_wraps_around() {
        let mut tl = Timeline::new(10, 3);
        tl.set_looping(true);
        tl.play();
        tick_many(&mut tl, 0.1, 3); // 0->1, 1->2, 2->0
        assert_eq!(tl.current_frame(), 0);
        assert!(tl.is_playing());
    }

    #[test]
    fn seek_clamps_correctly() {
        let mut tl = Timeline::new(24, 10);
        tl.seek(15); // exceeds total-1
        assert_eq!(tl.current_frame(), 9);
        tl.seek(5);
        assert_eq!(tl.current_frame(), 5);
    }

    #[test]
    fn stop_resets_to_zero() {
        let mut tl = Timeline::new(30, 60);
        tl.play();
        tick_many(&mut tl, 1.0 / 30.0, 10);
        assert!(tl.current_frame() > 0);
        tl.stop();
        assert_eq!(tl.current_frame(), 0);
        assert!(!tl.is_playing());
    }

    #[test]
    fn set_total_frames_clamps_current() {
        let mut tl = Timeline::new(10, 100);
        tl.seek(50);
        tl.set_total_frames(30);
        assert_eq!(tl.current_frame(), 29);
        tl.set_total_frames(0);
        assert_eq!(tl.current_frame(), 0);
    }

    #[test]
    fn determinism_same_input_same_output() {
        let mut a = Timeline::new(15, 300);
        let mut b = Timeline::new(15, 300);
        a.play();
        b.play();
        let deltas = [0.016, 0.02, 0.013, 0.1];
        for &d in &deltas {
            a.advance_by_time(d);
            b.advance_by_time(d);
        }
        assert_eq!(a.current_frame(), b.current_frame());
        assert_eq!(a.time_accumulator, b.time_accumulator);
    }

    #[test]
    fn large_delta_moves_multiple_frames() {
        let mut tl = Timeline::new(10, 100);
        tl.play();
        // 0.5 s -> 5 frames
        tl.advance_by_time(0.5);
        assert_eq!(tl.current_frame(), 5);
    }

    #[test]
    fn play_after_end_non_looping_restarts() {
        let mut tl = Timeline::new(10, 5);
        tl.play();
        tick_many(&mut tl, 0.1, 5); // reached end, stopped
        assert!(!tl.is_playing());
        tl.play(); // should restart from 0
        assert_eq!(tl.current_frame(), 0);
        assert!(tl.is_playing());
    }
}