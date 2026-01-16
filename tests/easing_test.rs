use ivy::scenario::Easing;

#[test]
fn test_easing_bounds() {
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
        Easing::EaseOutBounce,
    ];

    for easing in easings {
        assert!(
            (easing.apply(0.0) - 0.0).abs() < 0.01,
            "{:?} should start at 0",
            easing
        );
        assert!(
            (easing.apply(1.0) - 1.0).abs() < 0.01,
            "{:?} should end at 1",
            easing
        );
    }
}

#[test]
fn test_back_easing_overshoots() {
    // Back easings can go below 0 or above 1 during animation
    let ease_in_back = Easing::EaseInBack;
    let ease_out_back = Easing::EaseOutBack;

    // EaseInBack goes slightly negative near t=0
    assert!(ease_in_back.apply(0.2) < 0.0);

    // EaseOutBack goes slightly above 1 near t=1
    assert!(ease_out_back.apply(0.8) > 1.0);
}

#[test]
fn test_clamp_input() {
    let easing = Easing::Linear;
    assert_eq!(easing.apply(-0.5), 0.0);
    assert_eq!(easing.apply(1.5), 1.0);
}

#[test]
fn test_linear_is_identity() {
    let easing = Easing::Linear;
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        assert!((easing.apply(t) - t).abs() < 0.0001);
    }
}

#[test]
fn test_ease_in_starts_slow() {
    let easing = Easing::EaseIn;
    // At t=0.25, ease in should be less than linear
    assert!(easing.apply(0.25) < 0.25);
}

#[test]
fn test_ease_out_starts_fast() {
    let easing = Easing::EaseOut;
    // At t=0.25, ease out should be greater than linear
    assert!(easing.apply(0.25) > 0.25);
}

#[test]
fn test_ease_in_out_symmetric() {
    let easing = Easing::EaseInOut;
    // At t=0.5, ease in out should be exactly 0.5
    assert!((easing.apply(0.5) - 0.5).abs() < 0.01);
}

#[test]
fn test_all_14_easings_exist() {
    // Ensure all 14 easing types are accessible and work
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
    assert_eq!(easings.len(), 14);

    // All should produce valid output for mid-point
    for easing in &easings {
        let mid = easing.apply(0.5);
        assert!(mid >= -0.5 && mid <= 1.5, "{:?} mid value {} out of range", easing, mid);
    }
}

#[test]
fn test_quadratic_vs_cubic() {
    // Cubic should have more "extreme" curves than quadratic
    let quad_in = Easing::EaseInQuad;
    let cubic_in = Easing::EaseInCubic;

    // At t=0.25, cubic should be slower (smaller value)
    assert!(cubic_in.apply(0.25) < quad_in.apply(0.25));

    // At t=0.75, cubic should be faster (larger value for ease in)
    let quad_out = Easing::EaseOutQuad;
    let cubic_out = Easing::EaseOutCubic;
    // EaseOut is faster at start, so at 0.25, cubic out > quad out
    assert!(cubic_out.apply(0.25) > quad_out.apply(0.25));
}

#[test]
fn test_ease_in_out_quad_symmetric() {
    let easing = Easing::EaseInOutQuad;
    assert!((easing.apply(0.5) - 0.5).abs() < 0.01);
}

#[test]
fn test_ease_in_out_cubic_symmetric() {
    let easing = Easing::EaseInOutCubic;
    assert!((easing.apply(0.5) - 0.5).abs() < 0.01);
}

#[test]
fn test_ease_in_out_back_symmetric() {
    let easing = Easing::EaseInOutBack;
    // Back easings can overshoot, but should be roughly symmetric
    let at_quarter = easing.apply(0.25);
    let at_three_quarter = easing.apply(0.75);
    // at_quarter + at_three_quarter should be roughly 1 for symmetric functions
    assert!((at_quarter + at_three_quarter - 1.0).abs() < 0.2);
}

#[test]
fn test_bounce_easing_properties() {
    let bounce = Easing::EaseOutBounce;

    // Bounce should have characteristic "bounces"
    // It should reach high values early
    assert!(bounce.apply(0.4) > 0.7);

    // Mid-range has characteristic dip due to bounce effect
    // bounce(0.6) ≈ 0.77 (in the "dip" between bounces)
    assert!(bounce.apply(0.6) > 0.7);

    // Later values stay very high
    assert!(bounce.apply(0.8) > 0.9);
}

#[test]
fn test_easing_monotonic_for_standard_easings() {
    // Standard easings (without Back) should be monotonically increasing
    let standard_easings = [
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
    ];

    for easing in &standard_easings {
        let mut prev = easing.apply(0.0);
        for i in 1..=100 {
            let t = i as f32 / 100.0;
            let curr = easing.apply(t);
            assert!(
                curr >= prev - 0.001,
                "{:?} is not monotonic at t={}: {} < {}",
                easing,
                t,
                curr,
                prev
            );
            prev = curr;
        }
    }
}

#[test]
fn test_easing_default() {
    // Default should be Linear
    let default_easing = Easing::default();
    assert_eq!(default_easing, Easing::Linear);
}

#[test]
fn test_easing_continuity() {
    // All easings should be continuous (no sudden jumps)
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
        let mut prev = easing.apply(0.0);
        for i in 1..=1000 {
            let t = i as f32 / 1000.0;
            let curr = easing.apply(t);
            // Change between consecutive values should be small
            let delta = (curr - prev).abs();
            assert!(
                delta < 0.1,
                "{:?} has discontinuity at t={}: delta={}",
                easing,
                t,
                delta
            );
            prev = curr;
        }
    }
}
