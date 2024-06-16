use std::path::Path;
use std::{fs, io};

pub(crate) fn copy_directory_recursively(src: &Path, dest: &Path) -> io::Result<()> {
    if src.is_file() {
        fs::copy(src, dest)?;
    } else if src.is_dir() && src.file_name().unwrap() != ".git" {
        fs::create_dir_all(dest)?;
        let entries = fs::read_dir(src)?;
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            let dest_path = dest.join(entry.file_name());

            copy_directory_recursively(&entry_path, &dest_path)?;
        }
    }
    Ok(())
}
