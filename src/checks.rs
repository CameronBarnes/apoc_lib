use std::process::Command;

use crate::IS_WINDOWS;

#[must_use] 
pub fn check_for_rsync() -> bool {
    if IS_WINDOWS {
        return false;
    }
    let result = Command::new("which").arg("rsync").output();

    if let Ok(output) = result {
        output.status.success()
    } else {
        false
    }
}
