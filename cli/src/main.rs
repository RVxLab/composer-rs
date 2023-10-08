use composer::lock_file::LockFile;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let lock_file: LockFile = File::open("./cli/src/composer.lock")?.try_into()?;

    dbg!(lock_file);

    Ok(())
}
