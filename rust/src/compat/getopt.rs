extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn getprogname() -> *const libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type __uint32_t = libc::c_uint;
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
pub type __uint16_t = libc::c_ushort;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
    return __x;
}
unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
    return __x;
}
unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
    return __x;
}
#[no_mangle]
pub static mut BSDopterr: libc::c_int = unsafe { 1i32 };
#[no_mangle]
pub static mut BSDoptind: libc::c_int = unsafe { 1i32 };
#[no_mangle]
pub static mut BSDoptopt: libc::c_int = unsafe { 0 };
#[no_mangle]
pub static mut BSDoptreset: libc::c_int = unsafe { 0 };
#[no_mangle]
pub static mut BSDoptarg: *mut libc::c_char =
    unsafe { 0 as *const libc::c_char as *mut libc::c_char };
#[no_mangle]
pub unsafe extern "C" fn BSDgetopt(mut nargc: libc::c_int,
                                   mut nargv: *const *mut libc::c_char,
                                   mut ostr: *const libc::c_char)
 -> libc::c_int {
    static mut place: *const libc::c_char =
        unsafe { b"\x00" as *const u8 as *const libc::c_char };
    let mut oli: *mut libc::c_char = 0 as *mut libc::c_char;
    if ostr == 0 as *mut libc::c_void as *const libc::c_char {
        return 1i32.wrapping_neg()
    } else {
        if 0 != BSDoptreset || 0 == *place {
            BSDoptreset = 0i32;
            if BSDoptind >= nargc ||
                   {
                       place = *nargv.offset(BSDoptind as isize);
                       *place as libc::c_int != 45
                   } {
                place = b"\x00" as *const u8 as *const libc::c_char;
                return 1i32.wrapping_neg()
            } else if 0 != *place.offset(1isize) as libc::c_int &&
                          {
                              place = place.offset(1isize);
                              *place as libc::c_int == 45
                          } {
                if 0 != *place.offset(1isize) {
                    return 63
                } else {
                    BSDoptind += 1;
                    place = b"\x00" as *const u8 as *const libc::c_char;
                    return 1i32.wrapping_neg()
                }
            }
        }
        let fresh0 = place;
        place = place.offset(1);
        BSDoptopt = *fresh0 as libc::c_int;
        if BSDoptopt == 58 ||
               {
                   oli = strchr(ostr, BSDoptopt);
                   0 != oli.is_null() as libc::c_int
               } {
            if BSDoptopt == 45 {
                return 1i32.wrapping_neg()
            } else {
                if 0 == *place { BSDoptind += 1 }
                if 0 != BSDopterr && *ostr as libc::c_int != 58 {
                    fprintf(stderr,
                            b"%s: unknown option -- %c\n\x00" as *const u8 as
                                *const libc::c_char, getprogname(),
                            BSDoptopt);
                }
                return 63
            }
        } else {
            oli = oli.offset(1isize);
            if *oli as libc::c_int != 58 {
                BSDoptarg = 0 as *mut libc::c_char;
                if 0 == *place { BSDoptind += 1 }
            } else {
                if 0 != *place {
                    BSDoptarg = place as *mut libc::c_char
                } else {
                    BSDoptind += 1;
                    if nargc <= BSDoptind {
                        place = b"\x00" as *const u8 as *const libc::c_char;
                        if *ostr as libc::c_int == 58 {
                            return 58
                        } else {
                            if 0 != BSDopterr {
                                fprintf(stderr,
                                        b"%s: option requires an argument -- %c\n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        getprogname(), BSDoptopt);
                            }
                            return 63
                        }
                    } else { BSDoptarg = *nargv.offset(BSDoptind as isize) }
                }
                place = b"\x00" as *const u8 as *const libc::c_char;
                BSDoptind += 1
            }
            return BSDoptopt
        }
    };
}
