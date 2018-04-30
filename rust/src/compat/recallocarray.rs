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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    fn getpagesize() -> libc::c_int;
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
    #[no_mangle]
    fn explicit_bzero(_: *mut libc::c_void, _: size_t) -> ();
}
pub type size_t = libc::c_ulong;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type __off64_t = libc::c_long;
#[no_mangle]
pub unsafe extern "C" fn recallocarray(mut ptr: *mut libc::c_void,
                                       mut oldnmemb: size_t,
                                       mut newnmemb: size_t, mut size: size_t)
 -> *mut libc::c_void {
    let mut oldsize: size_t = 0;
    let mut newsize: size_t = 0;
    let mut newptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if ptr == 0 as *mut libc::c_void {
        return calloc(newnmemb, size)
    } else if (newnmemb >=
                   (1i32 as size_t) <<
                       (::std::mem::size_of::<size_t>() as
                            libc::c_ulong).wrapping_mul(4i32 as libc::c_ulong)
                   ||
                   size >=
                       (1i32 as size_t) <<
                           (::std::mem::size_of::<size_t>() as
                                libc::c_ulong).wrapping_mul(4i32 as
                                                                libc::c_ulong))
                  && newnmemb > 0i32 as libc::c_ulong &&
                  18446744073709551615u64.wrapping_div(newnmemb) < size {
        *__errno_location() = 12i32;
        return 0 as *mut libc::c_void
    } else {
        newsize = newnmemb.wrapping_mul(size);
        if (oldnmemb >=
                (1i32 as size_t) <<
                    (::std::mem::size_of::<size_t>() as
                         libc::c_ulong).wrapping_mul(4i32 as libc::c_ulong) ||
                size >=
                    (1i32 as size_t) <<
                        (::std::mem::size_of::<size_t>() as
                             libc::c_ulong).wrapping_mul(4i32 as
                                                             libc::c_ulong))
               && oldnmemb > 0i32 as libc::c_ulong &&
               18446744073709551615u64.wrapping_div(oldnmemb) < size {
            *__errno_location() = 22i32;
            return 0 as *mut libc::c_void
        } else {
            oldsize = oldnmemb.wrapping_mul(size);
            if newsize <= oldsize {
                let mut d: size_t = oldsize.wrapping_sub(newsize);
                if d < oldsize.wrapping_div(2i32 as libc::c_ulong) &&
                       d < getpagesize() as size_t {
                    memset((ptr as *mut libc::c_char).offset(newsize as isize)
                               as *mut libc::c_void, 0i32, d);
                    return ptr
                }
            }
            newptr = malloc(newsize);
            if newptr == 0 as *mut libc::c_void {
                return 0 as *mut libc::c_void
            } else {
                if newsize > oldsize {
                    memcpy(newptr, ptr, oldsize);
                    memset((newptr as
                                *mut libc::c_char).offset(oldsize as isize) as
                               *mut libc::c_void, 0i32,
                           newsize.wrapping_sub(oldsize));
                } else { memcpy(newptr, ptr, newsize); }
                explicit_bzero(ptr, oldsize);
                free(ptr);
                return newptr
            }
        }
    };
}
