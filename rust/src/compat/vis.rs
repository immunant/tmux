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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
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
pub const _ISxdigit: unnamed = 4096;
pub const _ISpunct: unnamed = 4;
pub type u_char = __u_char;
pub const _ISspace: unnamed = 8192;
pub const _ISblank: unnamed = 1;
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
pub type u_int = __u_int;
pub const _IScntrl: unnamed = 2;
pub type __u_int = libc::c_uint;
pub type __u_char = libc::c_uchar;
pub const _ISdigit: unnamed = 2048;
pub const _ISupper: unnamed = 256;
pub type _IO_lock_t = ();
pub type unnamed = libc::c_uint;
pub const _ISlower: unnamed = 512;
pub const _ISprint: unnamed = 16384;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const _ISalpha: unnamed = 1024;
pub const _ISalnum: unnamed = 8;
pub type __off64_t = libc::c_long;
pub const _ISgraph: unnamed = 32768;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn vis(mut dst: *mut libc::c_char, mut c: libc::c_int,
                             mut flag: libc::c_int, mut nextc: libc::c_int)
 -> *mut libc::c_char {
    let mut current_block: u64;
    if (c == 92 || flag & 1024i32 == 0i32) &&
           (c as u_int <= (127i32 * 2i32 + 1i32) as libc::c_uint &&
                c as u_char as libc::c_int & !127i32 == 0i32 &&
                (c != 42 && c != 63 && c != 91 && c != 35 ||
                     flag & 256i32 == 0i32) &&
                0 !=
                    *(*__ctype_b_loc()).offset(c as u_char as libc::c_int as
                                                   isize) as libc::c_int &
                        _ISgraph as libc::c_int as libc::c_ushort as
                            libc::c_int || flag & 4i32 == 0i32 && c == 32 ||
                flag & 8i32 == 0i32 && c == 9 ||
                flag & 16i32 == 0i32 && c == 10 ||
                0 != flag & 32i32 &&
                    (c == 8 || c == 7 || c == 13 ||
                         0 !=
                             *(*__ctype_b_loc()).offset(c as u_char as
                                                            libc::c_int as
                                                            isize) as
                                 libc::c_int &
                                 _ISgraph as libc::c_int as libc::c_ushort as
                                     libc::c_int)) {
        if c == 34 && flag & 512i32 != 0i32 || c == 92 && flag & 64i32 == 0i32
           {
            let fresh0 = dst;
            dst = dst.offset(1);
            *fresh0 = 92 as libc::c_char
        }
        let fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = c as libc::c_char;
        *dst = 0 as libc::c_char;
        return dst
    } else {
        if 0 != flag & 2i32 {
            match c {
                10 => {
                    current_block = 8744843025385043909;
                    match current_block {
                        15750721572495859450 => {
                            let fresh10 = dst;
                            dst = dst.offset(1);
                            *fresh10 = 92 as libc::c_char;
                            let fresh11 = dst;
                            dst = dst.offset(1);
                            *fresh11 = 118 as libc::c_char
                        }
                        15054042581426528366 => {
                            let fresh12 = dst;
                            dst = dst.offset(1);
                            *fresh12 = 92 as libc::c_char;
                            let fresh13 = dst;
                            dst = dst.offset(1);
                            *fresh13 = 116 as libc::c_char
                        }
                        1063080514726413358 => {
                            let fresh6 = dst;
                            dst = dst.offset(1);
                            *fresh6 = 92 as libc::c_char;
                            let fresh7 = dst;
                            dst = dst.offset(1);
                            *fresh7 = 98 as libc::c_char
                        }
                        8744843025385043909 => {
                            let fresh2 = dst;
                            dst = dst.offset(1);
                            *fresh2 = 92 as libc::c_char;
                            let fresh3 = dst;
                            dst = dst.offset(1);
                            *fresh3 = 110 as libc::c_char
                        }
                        11653228657089272161 => {
                            let fresh18 = dst;
                            dst = dst.offset(1);
                            *fresh18 = 92 as libc::c_char;
                            let fresh19 = dst;
                            dst = dst.offset(1);
                            *fresh19 = 48 as libc::c_char;
                            if nextc as u_char as libc::c_int >= 48 &&
                                   nextc as u_char as libc::c_int <= 55 {
                                let fresh20 = dst;
                                dst = dst.offset(1);
                                *fresh20 = 48 as libc::c_char;
                                let fresh21 = dst;
                                dst = dst.offset(1);
                                *fresh21 = 48 as libc::c_char
                            }
                        }
                        13382527123874078044 => {
                            let fresh14 = dst;
                            dst = dst.offset(1);
                            *fresh14 = 92 as libc::c_char;
                            let fresh15 = dst;
                            dst = dst.offset(1);
                            *fresh15 = 102 as libc::c_char
                        }
                        16190698728104149563 => {
                            let fresh4 = dst;
                            dst = dst.offset(1);
                            *fresh4 = 92 as libc::c_char;
                            let fresh5 = dst;
                            dst = dst.offset(1);
                            *fresh5 = 114 as libc::c_char
                        }
                        16067806742815681068 => {
                            let fresh16 = dst;
                            dst = dst.offset(1);
                            *fresh16 = 92 as libc::c_char;
                            let fresh17 = dst;
                            dst = dst.offset(1);
                            *fresh17 = 115 as libc::c_char
                        }
                        _ => {
                            let fresh8 = dst;
                            dst = dst.offset(1);
                            *fresh8 = 92 as libc::c_char;
                            let fresh9 = dst;
                            dst = dst.offset(1);
                            *fresh9 = 97 as libc::c_char
                        }
                    }
                    current_block = 15979760618723914544;
                }
                13 => {
                    current_block = 16190698728104149563;
                    match current_block {
                        15750721572495859450 => {
                            let fresh10 = dst;
                            dst = dst.offset(1);
                            *fresh10 = 92 as libc::c_char;
                            let fresh11 = dst;
                            dst = dst.offset(1);
                            *fresh11 = 118 as libc::c_char
                        }
                        15054042581426528366 => {
                            let fresh12 = dst;
                            dst = dst.offset(1);
                            *fresh12 = 92 as libc::c_char;
                            let fresh13 = dst;
                            dst = dst.offset(1);
                            *fresh13 = 116 as libc::c_char
                        }
                        1063080514726413358 => {
                            let fresh6 = dst;
                            dst = dst.offset(1);
                            *fresh6 = 92 as libc::c_char;
                            let fresh7 = dst;
                            dst = dst.offset(1);
                            *fresh7 = 98 as libc::c_char
                        }
                        8744843025385043909 => {
                            let fresh2 = dst;
                            dst = dst.offset(1);
                            *fresh2 = 92 as libc::c_char;
                            let fresh3 = dst;
                            dst = dst.offset(1);
                            *fresh3 = 110 as libc::c_char
                        }
                        11653228657089272161 => {
                            let fresh18 = dst;
                            dst = dst.offset(1);
                            *fresh18 = 92 as libc::c_char;
                            let fresh19 = dst;
                            dst = dst.offset(1);
                            *fresh19 = 48 as libc::c_char;
                            if nextc as u_char as libc::c_int >= 48 &&
                                   nextc as u_char as libc::c_int <= 55 {
                                let fresh20 = dst;
                                dst = dst.offset(1);
                                *fresh20 = 48 as libc::c_char;
                                let fresh21 = dst;
                                dst = dst.offset(1);
                                *fresh21 = 48 as libc::c_char
                            }
                        }
                        13382527123874078044 => {
                            let fresh14 = dst;
                            dst = dst.offset(1);
                            *fresh14 = 92 as libc::c_char;
                            let fresh15 = dst;
                            dst = dst.offset(1);
                            *fresh15 = 102 as libc::c_char
                        }
                        16190698728104149563 => {
                            let fresh4 = dst;
                            dst = dst.offset(1);
                            *fresh4 = 92 as libc::c_char;
                            let fresh5 = dst;
                            dst = dst.offset(1);
                            *fresh5 = 114 as libc::c_char
                        }
                        16067806742815681068 => {
                            let fresh16 = dst;
                            dst = dst.offset(1);
                            *fresh16 = 92 as libc::c_char;
                            let fresh17 = dst;
                            dst = dst.offset(1);
                            *fresh17 = 115 as libc::c_char
                        }
                        _ => {
                            let fresh8 = dst;
                            dst = dst.offset(1);
                            *fresh8 = 92 as libc::c_char;
                            let fresh9 = dst;
                            dst = dst.offset(1);
                            *fresh9 = 97 as libc::c_char
                        }
                    }
                    current_block = 15979760618723914544;
                }
                8 => {
                    current_block = 1063080514726413358;
                    match current_block {
                        15750721572495859450 => {
                            let fresh10 = dst;
                            dst = dst.offset(1);
                            *fresh10 = 92 as libc::c_char;
                            let fresh11 = dst;
                            dst = dst.offset(1);
                            *fresh11 = 118 as libc::c_char
                        }
                        15054042581426528366 => {
                            let fresh12 = dst;
                            dst = dst.offset(1);
                            *fresh12 = 92 as libc::c_char;
                            let fresh13 = dst;
                            dst = dst.offset(1);
                            *fresh13 = 116 as libc::c_char
                        }
                        1063080514726413358 => {
                            let fresh6 = dst;
                            dst = dst.offset(1);
                            *fresh6 = 92 as libc::c_char;
                            let fresh7 = dst;
                            dst = dst.offset(1);
                            *fresh7 = 98 as libc::c_char
                        }
                        8744843025385043909 => {
                            let fresh2 = dst;
                            dst = dst.offset(1);
                            *fresh2 = 92 as libc::c_char;
                            let fresh3 = dst;
                            dst = dst.offset(1);
                            *fresh3 = 110 as libc::c_char
                        }
                        11653228657089272161 => {
                            let fresh18 = dst;
                            dst = dst.offset(1);
                            *fresh18 = 92 as libc::c_char;
                            let fresh19 = dst;
                            dst = dst.offset(1);
                            *fresh19 = 48 as libc::c_char;
                            if nextc as u_char as libc::c_int >= 48 &&
                                   nextc as u_char as libc::c_int <= 55 {
                                let fresh20 = dst;
                                dst = dst.offset(1);
                                *fresh20 = 48 as libc::c_char;
                                let fresh21 = dst;
                                dst = dst.offset(1);
                                *fresh21 = 48 as libc::c_char
                            }
                        }
                        13382527123874078044 => {
                            let fresh14 = dst;
                            dst = dst.offset(1);
                            *fresh14 = 92 as libc::c_char;
                            let fresh15 = dst;
                            dst = dst.offset(1);
                            *fresh15 = 102 as libc::c_char
                        }
                        16190698728104149563 => {
                            let fresh4 = dst;
                            dst = dst.offset(1);
                            *fresh4 = 92 as libc::c_char;
                            let fresh5 = dst;
                            dst = dst.offset(1);
                            *fresh5 = 114 as libc::c_char
                        }
                        16067806742815681068 => {
                            let fresh16 = dst;
                            dst = dst.offset(1);
                            *fresh16 = 92 as libc::c_char;
                            let fresh17 = dst;
                            dst = dst.offset(1);
                            *fresh17 = 115 as libc::c_char
                        }
                        _ => {
                            let fresh8 = dst;
                            dst = dst.offset(1);
                            *fresh8 = 92 as libc::c_char;
                            let fresh9 = dst;
                            dst = dst.offset(1);
                            *fresh9 = 97 as libc::c_char
                        }
                    }
                    current_block = 15979760618723914544;
                }
                7 => {
                    current_block = 3889539373966549683;
                    match current_block {
                        15750721572495859450 => {
                            let fresh10 = dst;
                            dst = dst.offset(1);
                            *fresh10 = 92 as libc::c_char;
                            let fresh11 = dst;
                            dst = dst.offset(1);
                            *fresh11 = 118 as libc::c_char
                        }
                        15054042581426528366 => {
                            let fresh12 = dst;
                            dst = dst.offset(1);
                            *fresh12 = 92 as libc::c_char;
                            let fresh13 = dst;
                            dst = dst.offset(1);
                            *fresh13 = 116 as libc::c_char
                        }
                        1063080514726413358 => {
                            let fresh6 = dst;
                            dst = dst.offset(1);
                            *fresh6 = 92 as libc::c_char;
                            let fresh7 = dst;
                            dst = dst.offset(1);
                            *fresh7 = 98 as libc::c_char
                        }
                        8744843025385043909 => {
                            let fresh2 = dst;
                            dst = dst.offset(1);
                            *fresh2 = 92 as libc::c_char;
                            let fresh3 = dst;
                            dst = dst.offset(1);
                            *fresh3 = 110 as libc::c_char
                        }
                        11653228657089272161 => {
                            let fresh18 = dst;
                            dst = dst.offset(1);
                            *fresh18 = 92 as libc::c_char;
                            let fresh19 = dst;
                            dst = dst.offset(1);
                            *fresh19 = 48 as libc::c_char;
                            if nextc as u_char as libc::c_int >= 48 &&
                                   nextc as u_char as libc::c_int <= 55 {
                                let fresh20 = dst;
                                dst = dst.offset(1);
                                *fresh20 = 48 as libc::c_char;
                                let fresh21 = dst;
                                dst = dst.offset(1);
                                *fresh21 = 48 as libc::c_char
                            }
                        }
                        13382527123874078044 => {
                            let fresh14 = dst;
                            dst = dst.offset(1);
                            *fresh14 = 92 as libc::c_char;
                            let fresh15 = dst;
                            dst = dst.offset(1);
                            *fresh15 = 102 as libc::c_char
                        }
                        16190698728104149563 => {
                            let fresh4 = dst;
                            dst = dst.offset(1);
                            *fresh4 = 92 as libc::c_char;
                            let fresh5 = dst;
                            dst = dst.offset(1);
                            *fresh5 = 114 as libc::c_char
                        }
                        16067806742815681068 => {
                            let fresh16 = dst;
                            dst = dst.offset(1);
                            *fresh16 = 92 as libc::c_char;
                            let fresh17 = dst;
                            dst = dst.offset(1);
                            *fresh17 = 115 as libc::c_char
                        }
                        _ => {
                            let fresh8 = dst;
                            dst = dst.offset(1);
                            *fresh8 = 92 as libc::c_char;
                            let fresh9 = dst;
                            dst = dst.offset(1);
                            *fresh9 = 97 as libc::c_char
                        }
                    }
                    current_block = 15979760618723914544;
                }
                11 => {
                    current_block = 15750721572495859450;
                    match current_block {
                        15750721572495859450 => {
                            let fresh10 = dst;
                            dst = dst.offset(1);
                            *fresh10 = 92 as libc::c_char;
                            let fresh11 = dst;
                            dst = dst.offset(1);
                            *fresh11 = 118 as libc::c_char
                        }
                        15054042581426528366 => {
                            let fresh12 = dst;
                            dst = dst.offset(1);
                            *fresh12 = 92 as libc::c_char;
                            let fresh13 = dst;
                            dst = dst.offset(1);
                            *fresh13 = 116 as libc::c_char
                        }
                        1063080514726413358 => {
                            let fresh6 = dst;
                            dst = dst.offset(1);
                            *fresh6 = 92 as libc::c_char;
                            let fresh7 = dst;
                            dst = dst.offset(1);
                            *fresh7 = 98 as libc::c_char
                        }
                        8744843025385043909 => {
                            let fresh2 = dst;
                            dst = dst.offset(1);
                            *fresh2 = 92 as libc::c_char;
                            let fresh3 = dst;
                            dst = dst.offset(1);
                            *fresh3 = 110 as libc::c_char
                        }
                        11653228657089272161 => {
                            let fresh18 = dst;
                            dst = dst.offset(1);
                            *fresh18 = 92 as libc::c_char;
                            let fresh19 = dst;
                            dst = dst.offset(1);
                            *fresh19 = 48 as libc::c_char;
                            if nextc as u_char as libc::c_int >= 48 &&
                                   nextc as u_char as libc::c_int <= 55 {
                                let fresh20 = dst;
                                dst = dst.offset(1);
                                *fresh20 = 48 as libc::c_char;
                                let fresh21 = dst;
                                dst = dst.offset(1);
                                *fresh21 = 48 as libc::c_char
                            }
                        }
                        13382527123874078044 => {
                            let fresh14 = dst;
                            dst = dst.offset(1);
                            *fresh14 = 92 as libc::c_char;
                            let fresh15 = dst;
                            dst = dst.offset(1);
                            *fresh15 = 102 as libc::c_char
                        }
                        16190698728104149563 => {
                            let fresh4 = dst;
                            dst = dst.offset(1);
                            *fresh4 = 92 as libc::c_char;
                            let fresh5 = dst;
                            dst = dst.offset(1);
                            *fresh5 = 114 as libc::c_char
                        }
                        16067806742815681068 => {
                            let fresh16 = dst;
                            dst = dst.offset(1);
                            *fresh16 = 92 as libc::c_char;
                            let fresh17 = dst;
                            dst = dst.offset(1);
                            *fresh17 = 115 as libc::c_char
                        }
                        _ => {
                            let fresh8 = dst;
                            dst = dst.offset(1);
                            *fresh8 = 92 as libc::c_char;
                            let fresh9 = dst;
                            dst = dst.offset(1);
                            *fresh9 = 97 as libc::c_char
                        }
                    }
                    current_block = 15979760618723914544;
                }
                9 => {
                    current_block = 15054042581426528366;
                    match current_block {
                        15750721572495859450 => {
                            let fresh10 = dst;
                            dst = dst.offset(1);
                            *fresh10 = 92 as libc::c_char;
                            let fresh11 = dst;
                            dst = dst.offset(1);
                            *fresh11 = 118 as libc::c_char
                        }
                        15054042581426528366 => {
                            let fresh12 = dst;
                            dst = dst.offset(1);
                            *fresh12 = 92 as libc::c_char;
                            let fresh13 = dst;
                            dst = dst.offset(1);
                            *fresh13 = 116 as libc::c_char
                        }
                        1063080514726413358 => {
                            let fresh6 = dst;
                            dst = dst.offset(1);
                            *fresh6 = 92 as libc::c_char;
                            let fresh7 = dst;
                            dst = dst.offset(1);
                            *fresh7 = 98 as libc::c_char
                        }
                        8744843025385043909 => {
                            let fresh2 = dst;
                            dst = dst.offset(1);
                            *fresh2 = 92 as libc::c_char;
                            let fresh3 = dst;
                            dst = dst.offset(1);
                            *fresh3 = 110 as libc::c_char
                        }
                        11653228657089272161 => {
                            let fresh18 = dst;
                            dst = dst.offset(1);
                            *fresh18 = 92 as libc::c_char;
                            let fresh19 = dst;
                            dst = dst.offset(1);
                            *fresh19 = 48 as libc::c_char;
                            if nextc as u_char as libc::c_int >= 48 &&
                                   nextc as u_char as libc::c_int <= 55 {
                                let fresh20 = dst;
                                dst = dst.offset(1);
                                *fresh20 = 48 as libc::c_char;
                                let fresh21 = dst;
                                dst = dst.offset(1);
                                *fresh21 = 48 as libc::c_char
                            }
                        }
                        13382527123874078044 => {
                            let fresh14 = dst;
                            dst = dst.offset(1);
                            *fresh14 = 92 as libc::c_char;
                            let fresh15 = dst;
                            dst = dst.offset(1);
                            *fresh15 = 102 as libc::c_char
                        }
                        16190698728104149563 => {
                            let fresh4 = dst;
                            dst = dst.offset(1);
                            *fresh4 = 92 as libc::c_char;
                            let fresh5 = dst;
                            dst = dst.offset(1);
                            *fresh5 = 114 as libc::c_char
                        }
                        16067806742815681068 => {
                            let fresh16 = dst;
                            dst = dst.offset(1);
                            *fresh16 = 92 as libc::c_char;
                            let fresh17 = dst;
                            dst = dst.offset(1);
                            *fresh17 = 115 as libc::c_char
                        }
                        _ => {
                            let fresh8 = dst;
                            dst = dst.offset(1);
                            *fresh8 = 92 as libc::c_char;
                            let fresh9 = dst;
                            dst = dst.offset(1);
                            *fresh9 = 97 as libc::c_char
                        }
                    }
                    current_block = 15979760618723914544;
                }
                12 => {
                    current_block = 13382527123874078044;
                    match current_block {
                        15750721572495859450 => {
                            let fresh10 = dst;
                            dst = dst.offset(1);
                            *fresh10 = 92 as libc::c_char;
                            let fresh11 = dst;
                            dst = dst.offset(1);
                            *fresh11 = 118 as libc::c_char
                        }
                        15054042581426528366 => {
                            let fresh12 = dst;
                            dst = dst.offset(1);
                            *fresh12 = 92 as libc::c_char;
                            let fresh13 = dst;
                            dst = dst.offset(1);
                            *fresh13 = 116 as libc::c_char
                        }
                        1063080514726413358 => {
                            let fresh6 = dst;
                            dst = dst.offset(1);
                            *fresh6 = 92 as libc::c_char;
                            let fresh7 = dst;
                            dst = dst.offset(1);
                            *fresh7 = 98 as libc::c_char
                        }
                        8744843025385043909 => {
                            let fresh2 = dst;
                            dst = dst.offset(1);
                            *fresh2 = 92 as libc::c_char;
                            let fresh3 = dst;
                            dst = dst.offset(1);
                            *fresh3 = 110 as libc::c_char
                        }
                        11653228657089272161 => {
                            let fresh18 = dst;
                            dst = dst.offset(1);
                            *fresh18 = 92 as libc::c_char;
                            let fresh19 = dst;
                            dst = dst.offset(1);
                            *fresh19 = 48 as libc::c_char;
                            if nextc as u_char as libc::c_int >= 48 &&
                                   nextc as u_char as libc::c_int <= 55 {
                                let fresh20 = dst;
                                dst = dst.offset(1);
                                *fresh20 = 48 as libc::c_char;
                                let fresh21 = dst;
                                dst = dst.offset(1);
                                *fresh21 = 48 as libc::c_char
                            }
                        }
                        13382527123874078044 => {
                            let fresh14 = dst;
                            dst = dst.offset(1);
                            *fresh14 = 92 as libc::c_char;
                            let fresh15 = dst;
                            dst = dst.offset(1);
                            *fresh15 = 102 as libc::c_char
                        }
                        16190698728104149563 => {
                            let fresh4 = dst;
                            dst = dst.offset(1);
                            *fresh4 = 92 as libc::c_char;
                            let fresh5 = dst;
                            dst = dst.offset(1);
                            *fresh5 = 114 as libc::c_char
                        }
                        16067806742815681068 => {
                            let fresh16 = dst;
                            dst = dst.offset(1);
                            *fresh16 = 92 as libc::c_char;
                            let fresh17 = dst;
                            dst = dst.offset(1);
                            *fresh17 = 115 as libc::c_char
                        }
                        _ => {
                            let fresh8 = dst;
                            dst = dst.offset(1);
                            *fresh8 = 92 as libc::c_char;
                            let fresh9 = dst;
                            dst = dst.offset(1);
                            *fresh9 = 97 as libc::c_char
                        }
                    }
                    current_block = 15979760618723914544;
                }
                32 => {
                    current_block = 16067806742815681068;
                    match current_block {
                        15750721572495859450 => {
                            let fresh10 = dst;
                            dst = dst.offset(1);
                            *fresh10 = 92 as libc::c_char;
                            let fresh11 = dst;
                            dst = dst.offset(1);
                            *fresh11 = 118 as libc::c_char
                        }
                        15054042581426528366 => {
                            let fresh12 = dst;
                            dst = dst.offset(1);
                            *fresh12 = 92 as libc::c_char;
                            let fresh13 = dst;
                            dst = dst.offset(1);
                            *fresh13 = 116 as libc::c_char
                        }
                        1063080514726413358 => {
                            let fresh6 = dst;
                            dst = dst.offset(1);
                            *fresh6 = 92 as libc::c_char;
                            let fresh7 = dst;
                            dst = dst.offset(1);
                            *fresh7 = 98 as libc::c_char
                        }
                        8744843025385043909 => {
                            let fresh2 = dst;
                            dst = dst.offset(1);
                            *fresh2 = 92 as libc::c_char;
                            let fresh3 = dst;
                            dst = dst.offset(1);
                            *fresh3 = 110 as libc::c_char
                        }
                        11653228657089272161 => {
                            let fresh18 = dst;
                            dst = dst.offset(1);
                            *fresh18 = 92 as libc::c_char;
                            let fresh19 = dst;
                            dst = dst.offset(1);
                            *fresh19 = 48 as libc::c_char;
                            if nextc as u_char as libc::c_int >= 48 &&
                                   nextc as u_char as libc::c_int <= 55 {
                                let fresh20 = dst;
                                dst = dst.offset(1);
                                *fresh20 = 48 as libc::c_char;
                                let fresh21 = dst;
                                dst = dst.offset(1);
                                *fresh21 = 48 as libc::c_char
                            }
                        }
                        13382527123874078044 => {
                            let fresh14 = dst;
                            dst = dst.offset(1);
                            *fresh14 = 92 as libc::c_char;
                            let fresh15 = dst;
                            dst = dst.offset(1);
                            *fresh15 = 102 as libc::c_char
                        }
                        16190698728104149563 => {
                            let fresh4 = dst;
                            dst = dst.offset(1);
                            *fresh4 = 92 as libc::c_char;
                            let fresh5 = dst;
                            dst = dst.offset(1);
                            *fresh5 = 114 as libc::c_char
                        }
                        16067806742815681068 => {
                            let fresh16 = dst;
                            dst = dst.offset(1);
                            *fresh16 = 92 as libc::c_char;
                            let fresh17 = dst;
                            dst = dst.offset(1);
                            *fresh17 = 115 as libc::c_char
                        }
                        _ => {
                            let fresh8 = dst;
                            dst = dst.offset(1);
                            *fresh8 = 92 as libc::c_char;
                            let fresh9 = dst;
                            dst = dst.offset(1);
                            *fresh9 = 97 as libc::c_char
                        }
                    }
                    current_block = 15979760618723914544;
                }
                0 => {
                    current_block = 11653228657089272161;
                    match current_block {
                        15750721572495859450 => {
                            let fresh10 = dst;
                            dst = dst.offset(1);
                            *fresh10 = 92 as libc::c_char;
                            let fresh11 = dst;
                            dst = dst.offset(1);
                            *fresh11 = 118 as libc::c_char
                        }
                        15054042581426528366 => {
                            let fresh12 = dst;
                            dst = dst.offset(1);
                            *fresh12 = 92 as libc::c_char;
                            let fresh13 = dst;
                            dst = dst.offset(1);
                            *fresh13 = 116 as libc::c_char
                        }
                        1063080514726413358 => {
                            let fresh6 = dst;
                            dst = dst.offset(1);
                            *fresh6 = 92 as libc::c_char;
                            let fresh7 = dst;
                            dst = dst.offset(1);
                            *fresh7 = 98 as libc::c_char
                        }
                        8744843025385043909 => {
                            let fresh2 = dst;
                            dst = dst.offset(1);
                            *fresh2 = 92 as libc::c_char;
                            let fresh3 = dst;
                            dst = dst.offset(1);
                            *fresh3 = 110 as libc::c_char
                        }
                        11653228657089272161 => {
                            let fresh18 = dst;
                            dst = dst.offset(1);
                            *fresh18 = 92 as libc::c_char;
                            let fresh19 = dst;
                            dst = dst.offset(1);
                            *fresh19 = 48 as libc::c_char;
                            if nextc as u_char as libc::c_int >= 48 &&
                                   nextc as u_char as libc::c_int <= 55 {
                                let fresh20 = dst;
                                dst = dst.offset(1);
                                *fresh20 = 48 as libc::c_char;
                                let fresh21 = dst;
                                dst = dst.offset(1);
                                *fresh21 = 48 as libc::c_char
                            }
                        }
                        13382527123874078044 => {
                            let fresh14 = dst;
                            dst = dst.offset(1);
                            *fresh14 = 92 as libc::c_char;
                            let fresh15 = dst;
                            dst = dst.offset(1);
                            *fresh15 = 102 as libc::c_char
                        }
                        16190698728104149563 => {
                            let fresh4 = dst;
                            dst = dst.offset(1);
                            *fresh4 = 92 as libc::c_char;
                            let fresh5 = dst;
                            dst = dst.offset(1);
                            *fresh5 = 114 as libc::c_char
                        }
                        16067806742815681068 => {
                            let fresh16 = dst;
                            dst = dst.offset(1);
                            *fresh16 = 92 as libc::c_char;
                            let fresh17 = dst;
                            dst = dst.offset(1);
                            *fresh17 = 115 as libc::c_char
                        }
                        _ => {
                            let fresh8 = dst;
                            dst = dst.offset(1);
                            *fresh8 = 92 as libc::c_char;
                            let fresh9 = dst;
                            dst = dst.offset(1);
                            *fresh9 = 97 as libc::c_char
                        }
                    }
                    current_block = 15979760618723914544;
                }
                _ => { current_block = 4988723283678924448; }
            }
        } else { current_block = 4988723283678924448; }
        match current_block {
            4988723283678924448 => {
                if c & 127i32 == 32 || 0 != flag & 1i32 ||
                       0 != flag & 256i32 &&
                           (c == 42 || c == 63 || c == 91 || c == 35) {
                    let fresh22 = dst;
                    dst = dst.offset(1);
                    *fresh22 = 92 as libc::c_char;
                    let fresh23 = dst;
                    dst = dst.offset(1);
                    *fresh23 =
                        ((c as u_char as libc::c_int >> 6i32 & 7i32) + 48) as
                            libc::c_char;
                    let fresh24 = dst;
                    dst = dst.offset(1);
                    *fresh24 =
                        ((c as u_char as libc::c_int >> 3i32 & 7i32) + 48) as
                            libc::c_char;
                    let fresh25 = dst;
                    dst = dst.offset(1);
                    *fresh25 =
                        ((c as u_char as libc::c_int & 7i32) + 48) as
                            libc::c_char
                } else {
                    if flag & 64i32 == 0i32 {
                        let fresh26 = dst;
                        dst = dst.offset(1);
                        *fresh26 = 92 as libc::c_char
                    }
                    if 0 != c & 128i32 {
                        c &= 127i32;
                        let fresh27 = dst;
                        dst = dst.offset(1);
                        *fresh27 = 77 as libc::c_char
                    }
                    if 0 !=
                           *(*__ctype_b_loc()).offset(c as u_char as
                                                          libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _IScntrl as libc::c_int as libc::c_ushort as
                                   libc::c_int {
                        let fresh28 = dst;
                        dst = dst.offset(1);
                        *fresh28 = 94 as libc::c_char;
                        if c == 127i32 {
                            let fresh29 = dst;
                            dst = dst.offset(1);
                            *fresh29 = 63 as libc::c_char
                        } else {
                            let fresh30 = dst;
                            dst = dst.offset(1);
                            *fresh30 = (c + 64) as libc::c_char
                        }
                    } else {
                        let fresh31 = dst;
                        dst = dst.offset(1);
                        *fresh31 = 45 as libc::c_char;
                        let fresh32 = dst;
                        dst = dst.offset(1);
                        *fresh32 = c as libc::c_char
                    }
                }
            }
            _ => { }
        }
        *dst = 0 as libc::c_char;
        return dst
    };
}
#[no_mangle]
pub unsafe extern "C" fn strvis(mut dst: *mut libc::c_char,
                                mut src: *const libc::c_char,
                                mut flag: libc::c_int) -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    start = dst;
    loop  {
        c = *src;
        if !(0 != c) { break ; }
        src = src.offset(1isize);
        dst = vis(dst, c as libc::c_int, flag, *src as libc::c_int)
    }
    *dst = 0 as libc::c_char;
    return start.offset_to(dst).expect("bad offset_to") as libc::c_long as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stravis(mut outp: *mut *mut libc::c_char,
                                 mut src: *const libc::c_char,
                                 mut flag: libc::c_int) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut serrno: libc::c_int = 0;
    buf =
        calloc(4i32 as libc::c_ulong,
               strlen(src).wrapping_add(1i32 as libc::c_ulong)) as
            *mut libc::c_char;
    if buf == 0 as *mut libc::c_void as *mut libc::c_char {
        return 1i32.wrapping_neg()
    } else {
        len = strvis(buf, src, flag);
        serrno = *__errno_location();
        *outp =
            realloc(buf as *mut libc::c_void, (len + 1i32) as libc::c_ulong)
                as *mut libc::c_char;
        if *outp == 0 as *mut libc::c_void as *mut libc::c_char {
            *outp = buf;
            *__errno_location() = serrno
        }
        return len
    };
}
#[no_mangle]
pub unsafe extern "C" fn strnvis(mut dst: *mut libc::c_char,
                                 mut src: *const libc::c_char,
                                 mut siz: size_t, mut flag: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tbuf: [libc::c_char; 5] = [0; 5];
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    i = 0i32;
    start = dst;
    end = start.offset(siz as isize).offset(-1isize);
    loop  {
        c = *src as libc::c_int;
        if !(0 != c && dst < end) { break ; }
        if (c == 92 || flag & 1024i32 == 0i32) &&
               (c as u_int <= (127i32 * 2i32 + 1i32) as libc::c_uint &&
                    c as u_char as libc::c_int & !127i32 == 0i32 &&
                    (c != 42 && c != 63 && c != 91 && c != 35 ||
                         flag & 256i32 == 0i32) &&
                    0 !=
                        *(*__ctype_b_loc()).offset(c as u_char as libc::c_int
                                                       as isize) as
                            libc::c_int &
                            _ISgraph as libc::c_int as libc::c_ushort as
                                libc::c_int || flag & 4i32 == 0i32 && c == 32
                    || flag & 8i32 == 0i32 && c == 9 ||
                    flag & 16i32 == 0i32 && c == 10 ||
                    0 != flag & 32i32 &&
                        (c == 8 || c == 7 || c == 13 ||
                             0 !=
                                 *(*__ctype_b_loc()).offset(c as u_char as
                                                                libc::c_int as
                                                                isize) as
                                     libc::c_int &
                                     _ISgraph as libc::c_int as libc::c_ushort
                                         as libc::c_int)) {
            if c == 34 && flag & 512i32 != 0i32 ||
                   c == 92 && flag & 64i32 == 0i32 {
                if dst.offset(1isize) >= end {
                    i = 2i32;
                    break ;
                } else {
                    let fresh33 = dst;
                    dst = dst.offset(1);
                    *fresh33 = 92 as libc::c_char
                }
            }
            i = 1i32;
            let fresh34 = dst;
            dst = dst.offset(1);
            *fresh34 = c as libc::c_char;
            src = src.offset(1isize)
        } else {
            src = src.offset(1isize);
            i =
                tbuf.as_mut_ptr().offset_to(vis(tbuf.as_mut_ptr(), c, flag,
                                                *src as
                                                    libc::c_int)).expect("bad offset_to")
                    as libc::c_long as libc::c_int;
            if dst.offset(i as isize) <= end {
                memcpy(dst as *mut libc::c_void,
                       tbuf.as_mut_ptr() as *const libc::c_void,
                       i as libc::c_ulong);
                dst = dst.offset(i as isize)
            } else { src = src.offset(-1isize); break ; }
        }
    }
    if siz > 0i32 as libc::c_ulong { *dst = 0 as libc::c_char }
    if dst.offset(i as isize) > end {
        current_block = 17965632435239708295;
    } else { current_block = 13109137661213826276; }
    loop  {
        match current_block {
            13109137661213826276 => {
                return start.offset_to(dst).expect("bad offset_to") as
                           libc::c_long as libc::c_int
            }
            _ => {
                c = *src as libc::c_int;
                if !(0 != c) {
                    current_block = 13109137661213826276;
                    continue ;
                }
                src = src.offset(1isize);
                dst =
                    dst.offset(tbuf.as_mut_ptr().offset_to(vis(tbuf.as_mut_ptr(),
                                                               c, flag,
                                                               *src as
                                                                   libc::c_int)).expect("bad offset_to")
                                   as libc::c_long as isize);
                current_block = 17965632435239708295;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn strvisx(mut dst: *mut libc::c_char,
                                 mut src: *const libc::c_char,
                                 mut len: size_t, mut flag: libc::c_int)
 -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    start = dst;
    while len > 1i32 as libc::c_ulong {
        c = *src;
        src = src.offset(1isize);
        dst = vis(dst, c as libc::c_int, flag, *src as libc::c_int);
        len = len.wrapping_sub(1)
    }
    if 0 != len { dst = vis(dst, *src as libc::c_int, flag, 0) }
    *dst = 0 as libc::c_char;
    return start.offset_to(dst).expect("bad offset_to") as libc::c_long as
               libc::c_int;
}

