//! Flow5 main API.
//!
//! The API is designed to work with projects, only one project open at a time.
//! In the C++ library, this is up to the programmer to manage, but in Rust, this is enforced by the [Project] struct.

use std::{
    sync::{Mutex, MutexGuard},
    thread::{self, ThreadId},
};

pub mod globals;

static PROJECT_LOCK: (Mutex<()>, Mutex<Option<ThreadId>>) = (Mutex::new(()), Mutex::new(None));

/// Struct that you must have in order to manipulate a project.
/// It does not hold any data, it acts as a key.
///
/// Only one instance may exist at once, because flow5 can only handle one project at once.
#[derive(Debug)]
pub struct Project {
    _guard: MutexGuard<'static, ()>,
}

/// Error type returned if the creation of a [Project] failed.
#[derive(Debug, Clone, Copy)]
pub enum AquisitionError {
    /// A [Project] already exists and is held by current thread. Waiting would create a deadlock.
    SelfOwned,
}

impl Project {
    /// Attempts to create a new [Project] struct.
    ///
    /// If a project already exists, waits for it to be dropped if owned by other thread,
    /// and returns an error if owned by current thread.
    pub fn new() -> Result<Self, AquisitionError> {
        let (lock, id) = &PROJECT_LOCK;

        let this_id = thread::current().id();

        let mut id_guard = id
            .lock()
            .expect("Mutex for project lock thread id got poisoned.");

        if id_guard
            .map(|blocker_id| blocker_id == this_id)
            .unwrap_or(false)
        {
            return Err(AquisitionError::SelfOwned);
        }

        *id_guard = Some(this_id);

        let guard = lock
            .lock()
            .expect("Mutex for project lock got poisoned. An opened project must have panicked.");

        Ok(Project { _guard: guard })
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        // Cleanup
        globals::delete_objects();

        // Unlock
        let (_, id) = &PROJECT_LOCK;

        *id.lock()
            .expect("Mutex for project lock thread id got poisoned.") = None;
    }
}
