use std::ffi::{CString, CStr};
use std::ptr;
use libc;

fn execl(path: &str, args: &[&str]) -> ! {
    let c_path = CString::new(path).expect("CString::new failed");
    let mut c_args: Vec<CString> = args.iter().map(|&s| CString::new(s).unwrap()).collect();
    let mut raw_args: Vec<*const libc::c_char> = c_args.iter().map(|s| s.as_ptr()).collect();
    raw_args.push(ptr::null()); // Null terminate the argument list

    unsafe {
        libc::execv(c_path.as_ptr(), raw_args.as_ptr());
        libc::perror(CString::new("execv failed").unwrap().as_ptr());
        libc::_exit(1);
    }
}

fn execle(path: &str, args: &[&str], env: &[(&str, &str)]) -> ! {
    let c_path = CString::new(path).expect("CString::new failed");
    let mut c_args: Vec<CString> = args.iter().map(|&s| CString::new(s).unwrap()).collect();
    let mut raw_args: Vec<*const libc::c_char> = c_args.iter().map(|s| s.as_ptr()).collect();
    raw_args.push(ptr::null());

    let c_env: Vec<CString> = env.iter()
        .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
        .collect();
    let mut raw_env: Vec<*const libc::c_char> = c_env.iter().map(|s| s.as_ptr()).collect();
    raw_env.push(ptr::null());

    unsafe {
        libc::execve(c_path.as_ptr(), raw_args.as_ptr(), raw_env.as_ptr());
        libc::perror(CString::new("execve failed").unwrap().as_ptr());
        libc::_exit(1);
    }
}

fn execlp(file: &str, args: &[&str]) -> ! {
    let c_file = CString::new(file).expect("CString::new failed");
    let mut c_args: Vec<CString> = args.iter().map(|&s| CString::new(s).unwrap()).collect();
    let mut raw_args: Vec<*const libc::c_char> = c_args.iter().map(|s| s.as_ptr()).collect();
    raw_args.push(ptr::null());

    unsafe {
        libc::execvp(c_file.as_ptr(), raw_args.as_ptr());
        libc::perror(CString::new("execvp failed").unwrap().as_ptr());
        libc::_exit(1);
    }
}
