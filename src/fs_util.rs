use std::fs;
use std::path::Path;

pub(crate) fn copy_directory_recursively<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dest: Q,
) -> std::io::Result<()> {
    let src = src.as_ref();
    let dest = dest.as_ref();

    if src.is_file() {
        fs::copy(src, dest)?;
    } else if src.is_dir() {
        if !dest.exists() {
            fs::create_dir_all(dest)?;
        }

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
