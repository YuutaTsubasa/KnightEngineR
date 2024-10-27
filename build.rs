use std::fs;
use std::path::Path;

fn copy_files_to_destination(source: &Path, destination: &Path) {
    if !source.exists() {
        return;
    }

    fs::create_dir_all(&destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        if entry.file_type().unwrap().is_dir() {
            copy_files_to_destination(
                &entry.path(),
                &destination.join(&file_name)
            );
            continue;
        }

        let dest_file = destination.join(file_name);
        fs::copy(entry.path(), dest_file).unwrap();
    }
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("resources");
    let source = Path::new("resources");

    copy_files_to_destination(source, &dest);
}