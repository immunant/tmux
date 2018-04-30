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
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __u_char = libc::c_uchar;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type u_char = __u_char;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type ssize_t = __ssize_t;
#[no_mangle]
pub unsafe extern "C" fn strunvis(mut dst: *mut libc::c_char,
                                  mut src: *const libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut c: libc::c_char = 0;
    let mut start: *mut libc::c_char = dst;
    let mut state: libc::c_int = 0i32;
    loop  {
        let fresh0 = src;
        src = src.offset(1);
        c = *fresh0;
        if 0 != c {
            loop  {
                match unvis(dst, c, &mut state as *mut libc::c_int, 0i32) {
                    1 => { dst = dst.offset(1isize); break ; }
                    2 => { dst = dst.offset(1isize) }
                    0 | 3 => { break ; }
                    _ => {
                        *dst = 0 as libc::c_char;
                        return 1i32.wrapping_neg()
                    }
                }
            }
        } else if unvis(dst, c, &mut state as *mut libc::c_int, 1i32) == 1i32
         {
            current_block = 735147466149431745;
            break ;
        } else { current_block = 715039052867723359; break ; }
    }
    match current_block {
        735147466149431745 => { dst = dst.offset(1isize) }
        _ => { }
    }
    *dst = 0 as libc::c_char;
    return start.offset_to(dst).expect("bad offset_to") as libc::c_long as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn unvis(mut cp: *mut libc::c_char, mut c: libc::c_char,
                               mut astate: *mut libc::c_int,
                               mut flag: libc::c_int) -> libc::c_int {
    if 0 != flag & 1i32 {
        if *astate == 5i32 || *astate == 6i32 {
            *astate = 0i32;
            return 1i32
        } else {
            return if *astate == 0i32 { 3i32 } else { 1i32.wrapping_neg() }
        }
    } else {
        match *astate {
            0 => {
                *cp = 0i32 as libc::c_char;
                if c as libc::c_int == 92 {
                    *astate = 1i32;
                    return 0i32
                } else { *cp = c; return 1i32 }
            }
            1 => {
                match c as libc::c_int {
                    92 => { *cp = c; *astate = 0i32; return 1i32 }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                        *cp = (c as libc::c_int - 48) as libc::c_char;
                        *astate = 5i32;
                        return 0i32
                    }
                    77 => {
                        *cp = 128i32 as libc::c_char;
                        *astate = 2i32;
                        return 0i32
                    }
                    94 => { *astate = 4i32; return 0i32 }
                    110 => {
                        *cp = 10 as libc::c_char;
                        *astate = 0i32;
                        return 1i32
                    }
                    114 => {
                        *cp = 13 as libc::c_char;
                        *astate = 0i32;
                        return 1i32
                    }
                    98 => {
                        *cp = 8 as libc::c_char;
                        *astate = 0i32;
                        return 1i32
                    }
                    97 => {
                        *cp = 7 as libc::c_char;
                        *astate = 0i32;
                        return 1i32
                    }
                    118 => {
                        *cp = 11 as libc::c_char;
                        *astate = 0i32;
                        return 1i32
                    }
                    116 => {
                        *cp = 9 as libc::c_char;
                        *astate = 0i32;
                        return 1i32
                    }
                    102 => {
                        *cp = 12 as libc::c_char;
                        *astate = 0i32;
                        return 1i32
                    }
                    115 => {
                        *cp = 32 as libc::c_char;
                        *astate = 0i32;
                        return 1i32
                    }
                    69 => {
                        *cp = 27 as libc::c_char;
                        *astate = 0i32;
                        return 1i32
                    }
                    10 => { *astate = 0i32; return 3i32 }
                    36 => { *astate = 0i32; return 3i32 }
                    _ => { *astate = 0i32; return 1i32.wrapping_neg() }
                }
            }
            2 => {
                if c as libc::c_int == 45 {
                    *astate = 3i32
                } else if c as libc::c_int == 94 {
                    *astate = 4i32
                } else { *astate = 0i32; return 1i32.wrapping_neg() }
                return 0i32
            }
            3 => {
                *astate = 0i32;
                *cp = (*cp as libc::c_int | c as libc::c_int) as libc::c_char;
                return 1i32
            }
            4 => {
                if c as libc::c_int == 63 {
                    *cp = (*cp as libc::c_int | 127i32) as libc::c_char
                } else {
                    *cp =
                        (*cp as libc::c_int | c as libc::c_int & 31i32) as
                            libc::c_char
                }
                *astate = 0i32;
                return 1i32
            }
            5 => {
                if c as u_char as libc::c_int >= 48 &&
                       c as u_char as libc::c_int <= 55 {
                    *cp =
                        (((*cp as libc::c_int) << 3i32) +
                             (c as libc::c_int - 48)) as libc::c_char;
                    *astate = 6i32;
                    return 0i32
                } else { *astate = 0i32; return 2i32 }
            }
            6 => {
                *astate = 0i32;
                if c as u_char as libc::c_int >= 48 &&
                       c as u_char as libc::c_int <= 55 {
                    *cp =
                        (((*cp as libc::c_int) << 3i32) +
                             (c as libc::c_int - 48)) as libc::c_char;
                    return 1i32
                } else { return 2i32 }
            }
            _ => { *astate = 0i32; return 1i32.wrapping_neg() }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn strnunvis(mut dst: *mut libc::c_char,
                                   mut src: *const libc::c_char,
                                   mut sz: size_t) -> ssize_t {
    let mut current_block: u64;
    let mut c: libc::c_char = 0;
    let mut p: libc::c_char = 0;
    let mut start: *mut libc::c_char = dst;
    let mut end: *mut libc::c_char = dst.offset(sz as isize).offset(-1isize);
    let mut state: libc::c_int = 0i32;
    if sz > 0i32 as libc::c_ulong { *end = 0 as libc::c_char }
    's_9:
        loop  {
            let fresh1 = src;
            src = src.offset(1);
            c = *fresh1;
            if 0 != c {
                loop  {
                    match unvis(&mut p as *mut libc::c_char, c,
                                &mut state as *mut libc::c_int, 0i32) {
                        1 => {
                            if dst < end {
                                current_block = 10680521327981672866;
                                break ;
                            } else {
                                current_block = 16658872821858055392;
                                break ;
                            }
                        }
                        2 => {
                            if dst < end { *dst = p }
                            dst = dst.offset(1isize)
                        }
                        0 | 3 => { continue 's_9 ; }
                        _ => {
                            if dst <= end {
                                current_block = 6873731126896040597;
                                break 's_9 ;
                            } else {
                                current_block = 15427931788582360902;
                                break 's_9 ;
                            }
                        }
                    }
                }
                match current_block {
                    10680521327981672866 => { *dst = p }
                    _ => { }
                }
                dst = dst.offset(1isize)
            } else if unvis(&mut p as *mut libc::c_char, c,
                            &mut state as *mut libc::c_int, 1i32) == 1i32 {
                current_block = 10879442775620481940;
                break ;
            } else { current_block = 2473556513754201174; break ; }
        }
    match current_block {
        10879442775620481940 => {
            if dst < end { *dst = p }
            dst = dst.offset(1isize);
            current_block = 2473556513754201174;
        }
        6873731126896040597 => {
            *dst = 0 as libc::c_char;
            current_block = 15427931788582360902;
        }
        _ => { }
    }
    match current_block {
        15427931788582360902 => { return 1i32.wrapping_neg() as ssize_t }
        _ => {
            if dst <= end { *dst = 0 as libc::c_char }
            return start.offset_to(dst).expect("bad offset_to") as
                       libc::c_long
        }
    };
}

