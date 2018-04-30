extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    pub type stat;
    pub type dirent;
    #[no_mangle]
    fn glob(__pattern: *const libc::c_char, __flags: libc::c_int,
            __errfunc:
                Option<unsafe extern "C" fn(_: *const libc::c_char,
                                            _: libc::c_int) -> libc::c_int>,
            __pglob: *mut glob_t) -> libc::c_int;
    #[no_mangle]
    fn globfree(__pglob: *mut glob_t) -> ();
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
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
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    static mut _sys_nerr: libc::c_int;
    #[no_mangle]
    static _sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn fatal(_: *const libc::c_char, ...) -> ();
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub gl_readdir: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                               -> *mut dirent>,
    pub gl_opendir: Option<unsafe extern "C" fn(_: *const libc::c_char)
                               -> *mut libc::c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *mut stat) -> libc::c_int>,
    pub gl_stat: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                             _: *mut stat) -> libc::c_int>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type __size_t = libc::c_ulong;
pub type _IO_lock_t = ();
pub type __pid_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type __off64_t = libc::c_long;
#[no_mangle]
pub unsafe extern "C" fn getdtablecount() -> libc::c_int {
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut g: glob_t =
        glob_t{gl_pathc: 0,
               gl_pathv: 0 as *mut *mut libc::c_char,
               gl_offs: 0,
               gl_flags: 0,
               gl_closedir: None,
               gl_readdir: None,
               gl_opendir: None,
               gl_lstat: None,
               gl_stat: None,};
    let mut n: libc::c_int = 0i32;
    if snprintf(path.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as
                    libc::c_ulong,
                b"/proc/%ld/fd/*\x00" as *const u8 as *const libc::c_char,
                getpid() as libc::c_long) < 0i32 {
        fatal(b"snprintf overflow\x00" as *const u8 as *const libc::c_char);
    }
    if glob(path.as_mut_ptr(), 0i32, None, &mut g as *mut glob_t) == 0i32 {
        n = g.gl_pathc as libc::c_int
    }
    globfree(&mut g as *mut glob_t);
    return n;
}

