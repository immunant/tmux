extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut program_invocation_name: *mut libc::c_char;
    #[no_mangle]
    static mut program_invocation_short_name: *mut libc::c_char;
    #[no_mangle]
    fn strtoll(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
               __base: libc::c_int) -> libc::c_longlong;
    #[no_mangle]
    static mut _IO_2_1_stdin_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stdout_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stderr_: _IO_FILE_plus;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    static mut _sys_nerr: libc::c_int;
    #[no_mangle]
    static _sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    static mut BSDopterr: libc::c_int;
    #[no_mangle]
    static mut BSDoptind: libc::c_int;
    #[no_mangle]
    static mut BSDoptopt: libc::c_int;
    #[no_mangle]
    static mut BSDoptreset: libc::c_int;
    #[no_mangle]
    static mut BSDoptarg: *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct errval {
    pub errstr: *const libc::c_char,
    pub err: libc::c_int,
}
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
#[no_mangle]
pub unsafe extern "C" fn strtonum(mut numstr: *const libc::c_char,
                                  mut minval: libc::c_longlong,
                                  mut maxval: libc::c_longlong,
                                  mut errstrp: *mut *const libc::c_char)
 -> libc::c_longlong {
    let mut ll: libc::c_longlong = 0i32 as libc::c_longlong;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut error: libc::c_int = 0i32;
    let mut ev: [errval; 4] =
        [errval{errstr: 0 as *const libc::c_char, err: 0i32,},
         errval{errstr: b"invalid\x00" as *const u8 as *const libc::c_char,
                err: 22i32,},
         errval{errstr: b"too small\x00" as *const u8 as *const libc::c_char,
                err: 34i32,},
         errval{errstr: b"too large\x00" as *const u8 as *const libc::c_char,
                err: 34i32,}];
    ev[0usize].err = *__errno_location();
    *__errno_location() = 0i32;
    if minval > maxval {
        error = 1i32
    } else {
        ll = strtoll(numstr, &mut ep as *mut *mut libc::c_char, 10i32);
        if numstr == ep || *ep as libc::c_int != 0 {
            error = 1i32
        } else if ll == 9223372036854775807i64.wrapping_neg() - 1i64 &&
                      *__errno_location() == 34i32 || ll < minval {
            error = 2i32
        } else if ll == 9223372036854775807i64 && *__errno_location() == 34i32
                      || ll > maxval {
            error = 3i32
        }
    }
    if errstrp != 0 as *mut libc::c_void as *mut *const libc::c_char {
        *errstrp = ev[error as usize].errstr
    }
    *__errno_location() = ev[error as usize].err;
    if 0 != error { ll = 0i32 as libc::c_longlong }
    return ll;
}

