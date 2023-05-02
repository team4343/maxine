use async_trait::async_trait;

use crate::scheduler::Schedulable;

/// A data structure that reads external information and updates input state
/// based off of it.
///
/// TODO: Potentially come up with a better name for this so it is not confused
///       with the std::io::Read trait.
/// TODO: Make this require the "executing" trait, or whatever we call it.
/// TODO: Make the State associated type require something that ensures it can
///       be stored properly.
#[async_trait]
pub trait Reader: Schedulable {
    type State;
}
