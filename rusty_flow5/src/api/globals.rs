//! Global functions regarding a flow5 project.

use std::path::Path;

use cxx::let_cxx_string;

#[cxx::bridge(namespace = "globals")]
mod ffi {
    unsafe extern "C++" {
        include!("rusty_flow5/cpp/wrapper.h");

        /// Removes all 2d and 3d objects from the internal arrays and deletes them.
        /// This function __MUST__ be called on exit, otherwise will cause a memory leak.
        fn deleteObjects();

        /// Saves all the data to a .fl5 project file.
        /// Overwrites any existing file.
        ///
        /// Returns true if the save operation was successful.
        fn saveFl5Project(pathname: &CxxString) -> bool;

        /// Appends a message to the log.
        fn pushToLog(msg: &CxxString);

        /// Clears the message stack.
        fn clearLog();

        /// Removes the front message in the queue and returns it.
        #[namespace = "modified::globals"]
        fn poplog() -> UniquePtr<CxxString>;
    }
}

/// Removes all 2d and 3d objects from the internal arrays and deletes them.
/// This function __MUST__ be called on exit, otherwise will cause a memory leak.
///
/// Here, call it when deleting a project.
pub(super) fn delete_objects() {
    ffi::deleteObjects();
}

/// Saves all the data to a .fl5 project file.
/// Overwrites any existing file.
///
/// Returns true if the save operation was successful.
pub fn save_fl5_project(path: &Path) -> bool {
    let_cxx_string!(str = path.as_os_str().as_encoded_bytes());

    ffi::saveFl5Project(&str)
}

/// Appends a message to the log.
pub fn push_to_log(msg: String) {
    let_cxx_string!(str = msg);

    ffi::pushToLog(&str);
}

pub use ffi::clearLog as clear_log;

/// Removes the front message in the queue and returns it.
pub fn poplog() -> String {
    ffi::poplog().to_string()
}
