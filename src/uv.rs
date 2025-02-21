use std::io;
use std::process::Command;

use log::info;

pub fn sync() -> Result<(), io::Error> {
    // We need to sync before we can lock
    info!("Syncing uv");
    let sync_cmd = Command::new("uv").arg("sync").output()?;
    if !sync_cmd.status.success() {
        panic!(
            "Problem syncing uv: {:?}",
            String::from_utf8(sync_cmd.stderr)
        );
    }

    Ok(())
}

pub fn lock() -> Result<(), io::Error> {
    info!("Writting uv lock");
    let lock_cmd = Command::new("uv").arg("lock").output()?;
    if !lock_cmd.status.success() {
        panic!(
            "Problem locking dependencies: {:?}",
            String::from_utf8(lock_cmd.stderr)
        );
    }
    Ok(())
}
