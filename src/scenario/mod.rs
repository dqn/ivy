pub mod parser;
pub mod types;

pub use parser::load_scenario;
pub use types::{
    CharAnimation, CharAnimationType, CharPosition, Choice, IfCondition, Scenario, SetVar, Shake,
    ShakeType, Transition, TransitionType,
};
