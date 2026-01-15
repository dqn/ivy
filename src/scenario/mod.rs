pub mod easing;
pub mod parser;
pub mod types;
pub mod validator;

#[allow(unused_imports)]
pub use parser::{load_scenario, parse_scenario};
#[allow(unused_imports)]
pub use types::{
    CameraFocus, CharAnimation, CharAnimationType, CharIdleAnimation, CharIdleType, CharPosition,
    Choice, Easing, Input, ModularCharDef, Scenario, Shake, ShakeType, TransitionDirection,
    TransitionType,
};
#[allow(unused_imports)]
pub use validator::{
    ValidationIssue, ValidationResult, Severity, detect_circular_paths, validate_scenario,
};
