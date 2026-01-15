use serde::{Deserialize, Serialize};

/// Easing functions for smooth animations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "editor-types", derive(ts_rs::TS))]
#[serde(rename_all = "snake_case")]
pub enum Easing {
    /// Linear interpolation (no easing).
    #[default]
    Linear,
    /// Ease in (slow start).
    EaseIn,
    /// Ease out (slow end).
    EaseOut,
    /// Ease in and out (slow start and end).
    EaseInOut,
    /// Quadratic ease in.
    EaseInQuad,
    /// Quadratic ease out.
    EaseOutQuad,
    /// Quadratic ease in and out.
    EaseInOutQuad,
    /// Cubic ease in.
    EaseInCubic,
    /// Cubic ease out.
    EaseOutCubic,
    /// Cubic ease in and out.
    EaseInOutCubic,
    /// Back ease in (slight overshoot at start).
    EaseInBack,
    /// Back ease out (slight overshoot at end).
    EaseOutBack,
    /// Back ease in and out.
    EaseInOutBack,
    /// Bounce ease out.
    EaseOutBounce,
}

impl Easing {
    /// Apply the easing function to a value t in the range [0, 1].
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t * t,
            Easing::EaseOut => 1.0 - (1.0 - t).powi(3),
            Easing::EaseInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Easing::EaseInQuad => t * t,
            Easing::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Easing::EaseInCubic => t * t * t,
            Easing::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Easing::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Easing::EaseInBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            Easing::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            Easing::EaseInOutBack => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if t < 0.5 {
                    ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
                }
            }
            Easing::EaseOutBounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        let easing = Easing::Linear;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.5), 0.5);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_ease_out_starts_fast() {
        let easing = Easing::EaseOut;
        // EaseOut should progress faster at the start
        assert!(easing.apply(0.25) > 0.25);
    }

    #[test]
    fn test_ease_in_starts_slow() {
        let easing = Easing::EaseIn;
        // EaseIn should progress slower at the start
        assert!(easing.apply(0.25) < 0.25);
    }

    #[test]
    fn test_clamp_bounds() {
        let easing = Easing::Linear;
        // Values outside [0, 1] should be clamped
        assert_eq!(easing.apply(-0.5), 0.0);
        assert_eq!(easing.apply(1.5), 1.0);
    }

    #[test]
    fn test_all_easings_reach_endpoints() {
        let easings = [
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
            Easing::EaseInQuad,
            Easing::EaseOutQuad,
            Easing::EaseInOutQuad,
            Easing::EaseInCubic,
            Easing::EaseOutCubic,
            Easing::EaseInOutCubic,
            Easing::EaseInBack,
            Easing::EaseOutBack,
            Easing::EaseInOutBack,
            Easing::EaseOutBounce,
        ];

        for easing in &easings {
            assert!(
                (easing.apply(0.0) - 0.0).abs() < 0.01,
                "{:?} should start at ~0",
                easing
            );
            assert!(
                (easing.apply(1.0) - 1.0).abs() < 0.01,
                "{:?} should end at ~1",
                easing
            );
        }
    }
}
