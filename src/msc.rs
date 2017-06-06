use std::path::Path;
use std::ffi::CString;
use status;
use status::Status;

pub fn path_to_c_str(path: &Path) -> Status<CString> {
    #[cfg(unix)]
    use std::os::unix::ffi::OsStrExt;

    let path_bytes;

    #[cfg(unix)]
    {
        path_bytes = path.as_os_str().as_bytes();
    }
    #[cfg(not(unix))]
    {
        path_bytes = path.to_str().unwrap().as_bytes()
    }

    match CString::new(path_bytes) {
        Ok(s) => Ok(s),
        Err(_) => Err(status::Error::BadArgument),
    }
}
