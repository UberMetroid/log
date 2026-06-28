use std::path::Path as StdPath;

pub fn is_path_within_data_dir(path: &StdPath, data_dir: &StdPath) -> bool {
    let canonical_data = match data_dir.canonicalize() {
        Ok(p) => p,
        Err(_) => return false,
    };
    let canonical_path = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            if let (Some(parent), Some(file_name)) = (path.parent(), path.file_name())
                && let Ok(cp) = parent.canonicalize()
            {
                return cp.join(file_name).starts_with(&canonical_data);
            }
            return false;
        }
    };
    canonical_path.starts_with(&canonical_data)
}
