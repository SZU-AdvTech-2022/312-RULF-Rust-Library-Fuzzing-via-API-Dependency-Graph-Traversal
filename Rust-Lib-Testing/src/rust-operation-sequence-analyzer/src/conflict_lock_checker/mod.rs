mod callgraph;
mod checker;
mod collector;
mod dataflow;
mod def_use;
mod genkill;
mod lock;
mod tracker;
pub use self::checker::ConflictLockChecker;
use super::config;
