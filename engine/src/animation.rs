//! Deterministic 2D keyframe animation sampling.
//! No external dependencies, no network, no randomness.

use std::cmp::Ordering;

// ---------------------------------------------------------------------------
// Math types – minimal, self-contained, deterministic
// ---------------------------------------------------------------------------

/// 2D vector with deterministic float operations (no fast-math, no SIMD flags).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        Self {
            x: a.x + (b.x - a.x) * t,
            y: a.y + (b.y - a.y) * t,
        }
    }
}

// ---------------------------------------------------------------------------
// Easing functions (deterministic, pure)
// ---------------------------------------------------------------------------

/// Easing mode applied between a keyframe and the next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Easing {
    /// Transform linear `t` (0..1) into eased `t` using the chosen mode.
    /// All operations are pure float math, no randomness.
    pub fn apply(self, t: f32) -> f32 {
        // Clamp to 0..1 to avoid overshoot during interpolation.
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => t * (2.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    let f = t - 1.0;
                    1.0 - 2.0 * f * f
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Keyframe
// ---------------------------------------------------------------------------

/// A single keyframe holding a value of type `T` at a specific time.
/// The `easing` describes how this keyframe transitions to the next.
#[derive(Debug, Clone)]
pub struct Keyframe<T> {
    pub time: f32,
    pub value: T,
    pub easing: Easing,
}

// ---------------------------------------------------------------------------
// Animation curve – samples a value at any time
// ---------------------------------------------------------------------------

/// A sorted collection of keyframes that can be sampled deterministically.
#[derive(Debug, Clone)]
pub struct Curve<T> {
    keyframes: Vec<Keyframe<T>>,
}

impl<T: Clone + Interpolate> Curve<T> {
    /// Create a curve from a pre-sorted (by time) list of keyframes.
    /// The list must be sorted ascending by `time`; panics in debug if not.
    pub fn new(keyframes: Vec<Keyframe<T>>) -> Self {
        debug_assert!(
            keyframes.windows(2).all(|w| w[0].time <= w[1].time),
            "Keyframes must be sorted by time"
        );
        Self { keyframes }
    }

    /// Sample the curve at time `t`.
    ///
    /// - Before the first keyframe: returns the first value.
    /// - After the last keyframe: returns the last value.
    /// - Inside the range: interpolates between surrounding keyframes
    ///   using the easing of the left keyframe.
    pub fn sample(&self, t: f32) -> T {
        if self.keyframes.is_empty() {
            panic!("Cannot sample an empty curve");
        }

        // Find insertion point via binary search
        let idx = self
            .keyframes
            .binary_search_by(|kf| {
                if kf.time < t {
                    Ordering::Less
                } else if kf.time > t {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .unwrap_or_else(|i| i); // i is the index where t would be inserted

        // Exact match
        if idx < self.keyframes.len() && (self.keyframes[idx].time - t).abs() < f32::EPSILON {
            return self.keyframes[idx].value.clone();
        }

        // Before first keyframe
        if idx == 0 {
            return self.keyframes[0].value.clone();
        }

        // After last keyframe
        if idx == self.keyframes.len() {
            return self.keyframes.last().unwrap().value.clone();
        }

        // Interpolate between keyframes[idx-1] (left) and keyframes[idx] (right)
        let left = &self.keyframes[idx - 1];
        let right = &self.keyframes[idx];
        let duration = right.time - left.time;

        let raw_t = if duration.abs() < f32::EPSILON {
            0.0
        } else {
            (t - left.time) / duration
        };

        let eased_t = left.easing.apply(raw_t);
        T::interpolate(&left.value, &right.value, eased_t)
    }
}

// ---------------------------------------------------------------------------
// Interpolation trait
// ---------------------------------------------------------------------------

/// Trait for types that can be linearly interpolated.
pub trait Interpolate {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self;
}

impl Interpolate for f32 {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        a + (b - a) * t
    }
}

impl Interpolate for Vec2 {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        Vec2::lerp(*a, *b, t)
    }
}

// ---------------------------------------------------------------------------
// Transform – the state we sample
// ---------------------------------------------------------------------------

/// A 2D transform that can be sampled from an AnimationClip.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub translation: Vec2,
    pub rotation: f32,    // angle in radians
    pub scale: Vec2,      // non-uniform scale
}

impl Transform {
    pub const fn identity() -> Self {
        Self {
            translation: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
        }
    }
}

// ---------------------------------------------------------------------------
// AnimationClip – collection of curves that together define a transform
// ---------------------------------------------------------------------------

/// A named animation clip that maps time to a full Transform.
/// All curves are optional: missing curves keep their default value.
#[derive(Debug, Clone)]
pub struct AnimationClip {
    pub name: String,
    pub pos_x: Option<Curve<f32>>,
    pub pos_y: Option<Curve<f32>>,
    pub rotation: Option<Curve<f32>>,
    pub scale_x: Option<Curve<f32>>,
    pub scale_y: Option<Curve<f32>>,
}

impl AnimationClip {
    /// Sample the full transform at a given time.
    /// Defaults are applied for any unassigned curve.
    pub fn sample(&self, t: f32) -> Transform {
        Transform {
            translation: Vec2::new(
                self.pos_x.as_ref().map_or(0.0, |c| c.sample(t)),
                self.pos_y.as_ref().map_or(0.0, |c| c.sample(t)),
            ),
            rotation: self.rotation.as_ref().map_or(0.0, |c| c.sample(t)),
            scale: Vec2::new(
                self.scale_x.as_ref().map_or(1.0, |c| c.sample(t)),
                self.scale_y.as_ref().map_or(1.0, |c| c.sample(t)),
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests – verify determinism and correctness
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_kf<T: Clone>(time: f32, value: T, easing: Easing) -> Keyframe<T> {
        Keyframe { time, value, easing }
    }

    #[test]
    fn linear_interpolation() {
        let curve = Curve::new(vec![
            make_kf(0.0, 0.0f32, Easing::Linear),
            make_kf(1.0, 10.0, Easing::Linear),
        ]);
        assert!((curve.sample(0.5) - 5.0).abs() < 0.001);
    }

    #[test]
    fn ease_in_interpolation() {
        let curve = Curve::new(vec![
            make_kf(0.0, 0.0f32, Easing::EaseIn),
            make_kf(1.0, 10.0, Easing::EaseIn),
        ]);
        let val = curve.sample(0.5);
        // easeIn t=0.5 → 0.25, so value = 2.5
        assert!((val - 2.5).abs() < 0.001);
    }

    #[test]
    fn extrapolation_before() {
        let curve = Curve::new(vec![
            make_kf(2.0, 5.0f32, Easing::Linear),
            make_kf(3.0, 8.0, Easing::Linear),
        ]);
        assert!((curve.sample(0.0) - 5.0).abs() < 0.001);
    }

    #[test]
    fn extrapolation_after() {
        let curve = Curve::new(vec![
            make_kf(2.0, 5.0f32, Easing::Linear),
            make_kf(3.0, 8.0, Easing::Linear),
        ]);
        assert!((curve.sample(10.0) - 8.0).abs() < 0.001);
    }

    #[test]
    fn exact_keyframe_hit() {
        let curve = Curve::new(vec![
            make_kf(1.0, 100.0f32, Easing::Linear),
        ]);
        assert!((curve.sample(1.0) - 100.0).abs() < 0.001);
    }

    #[test]
    fn animation_clip_defaults() {
        let clip = AnimationClip {
            name: "idle".into(),
            pos_x: Some(Curve::new(vec![
                make_kf(0.0, 5.0, Easing::Linear),
                make_kf(1.0, 10.0, Easing::Linear),
            ])),
            pos_y: None,
            rotation: None,
            scale_x: None,
            scale_y: None,
        };
        let t = clip.sample(0.5);
        assert!((t.translation.x - 7.5).abs() < 0.01);
        assert!((t.translation.y - 0.0).abs() < 0.01);
        assert!((t.rotation - 0.0).abs() < 0.01);
        assert!((t.scale.x - 1.0).abs() < 0.01);
        assert!((t.scale.y - 1.0).abs() < 0.01);
    }
}