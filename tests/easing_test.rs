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
