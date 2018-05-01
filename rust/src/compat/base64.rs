extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    static in6addr_any: in6_addr;
    #[no_mangle]
    static in6addr_loopback: in6_addr;
    #[no_mangle]
    static _sys_siglist: [*const libc::c_char; 65];
    #[no_mangle]
    static sys_siglist: [*const libc::c_char; 65];
    #[no_mangle]
    static _ns_flagdata: [_ns_flagdata; 0];
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
}
pub type u_char = __u_char;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub const _IScntrl: unnamed = 2;
pub const _ISlower: unnamed = 512;
pub type uint16_t = libc::c_ushort;
pub const _ISalnum: unnamed = 8;
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _ns_flagdata {
    pub mask: libc::c_int,
    pub shift: libc::c_int,
}
pub const _ISprint: unnamed = 16384;
pub type __off_t = libc::c_long;
pub type unnamed = libc::c_uint;
pub type _IO_lock_t = ();
pub const _ISxdigit: unnamed = 4096;
pub const _ISpunct: unnamed = 4;
pub const _ISspace: unnamed = 8192;
pub const _ISdigit: unnamed = 2048;
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
pub const _ISupper: unnamed = 256;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub type uint8_t = libc::c_uchar;
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const _ISblank: unnamed = 1;
pub const _ISalpha: unnamed = 1024;
pub const _ISgraph: unnamed = 32768;
#[no_mangle]
pub unsafe extern "C" fn __b64_ntop(mut src: *const u_char,
                                    mut srclength: size_t,
                                    mut target: *mut libc::c_char,
                                    mut targsize: size_t) -> libc::c_int {
    let mut current_block: u64;
    let mut datalength: size_t = 0i32 as size_t;
    let mut input: [u_char; 3] = [0; 3];
    let mut output: [u_char; 4] = [0; 4];
    let mut i: libc::c_int = 0;
    loop  {
        if (2i32 as libc::c_ulong) < srclength {
            let fresh0 = src;
            src = src.offset(1);
            input[0usize] = *fresh0;
            let fresh1 = src;
            src = src.offset(1);
            input[1usize] = *fresh1;
            let fresh2 = src;
            src = src.offset(1);
            input[2usize] = *fresh2;
            srclength =
                (srclength as
                     libc::c_ulong).wrapping_sub(3i32 as libc::c_ulong) as
                    size_t as size_t;
            output[0usize] = (input[0usize] as libc::c_int >> 2i32) as u_char;
            output[1usize] =
                (((input[0usize] as libc::c_int & 3i32) << 4i32) +
                     (input[1usize] as libc::c_int >> 4i32)) as u_char;
            output[2usize] =
                (((input[1usize] as libc::c_int & 15i32) << 2i32) +
                     (input[2usize] as libc::c_int >> 6i32)) as u_char;
            output[3usize] = (input[2usize] as libc::c_int & 63i32) as u_char;
            if datalength.wrapping_add(4i32 as libc::c_ulong) > targsize {
                return 1i32.wrapping_neg()
            } else {
                let fresh3 = datalength;
                datalength = datalength.wrapping_add(1);
                *target.offset(fresh3 as isize) =
                    Base64[output[0usize] as usize];
                let fresh4 = datalength;
                datalength = datalength.wrapping_add(1);
                *target.offset(fresh4 as isize) =
                    Base64[output[1usize] as usize];
                let fresh5 = datalength;
                datalength = datalength.wrapping_add(1);
                *target.offset(fresh5 as isize) =
                    Base64[output[2usize] as usize];
                let fresh6 = datalength;
                datalength = datalength.wrapping_add(1);
                *target.offset(fresh6 as isize) =
                    Base64[output[3usize] as usize]
            }
        } else if 0i32 as libc::c_ulong != srclength {
            current_block = 6873731126896040597;
            break ;
        } else { current_block = 15240798224410183470; break ; }
    }
    match current_block {
        6873731126896040597 => {
            input[2usize] = 0 as u_char;
            input[1usize] = input[2usize];
            input[0usize] = input[1usize];
            i = 0i32;
            while (i as libc::c_ulong) < srclength {
                let fresh7 = src;
                src = src.offset(1);
                input[i as usize] = *fresh7;
                i += 1
            }
            output[0usize] = (input[0usize] as libc::c_int >> 2i32) as u_char;
            output[1usize] =
                (((input[0usize] as libc::c_int & 3i32) << 4i32) +
                     (input[1usize] as libc::c_int >> 4i32)) as u_char;
            output[2usize] =
                (((input[1usize] as libc::c_int & 15i32) << 2i32) +
                     (input[2usize] as libc::c_int >> 6i32)) as u_char;
            if datalength.wrapping_add(4i32 as libc::c_ulong) > targsize {
                return 1i32.wrapping_neg()
            } else {
                let fresh8 = datalength;
                datalength = datalength.wrapping_add(1);
                *target.offset(fresh8 as isize) =
                    Base64[output[0usize] as usize];
                let fresh9 = datalength;
                datalength = datalength.wrapping_add(1);
                *target.offset(fresh9 as isize) =
                    Base64[output[1usize] as usize];
                if srclength == 1i32 as libc::c_ulong {
                    let fresh10 = datalength;
                    datalength = datalength.wrapping_add(1);
                    *target.offset(fresh10 as isize) = Pad64
                } else {
                    let fresh11 = datalength;
                    datalength = datalength.wrapping_add(1);
                    *target.offset(fresh11 as isize) =
                        Base64[output[2usize] as usize]
                }
                let fresh12 = datalength;
                datalength = datalength.wrapping_add(1);
                *target.offset(fresh12 as isize) = Pad64
            }
        }
        _ => { }
    }
    if datalength >= targsize {
        return 1i32.wrapping_neg()
    } else {
        *target.offset(datalength as isize) = 0 as libc::c_char;
        return datalength as libc::c_int
    };
}
static mut Pad64: libc::c_char = unsafe { 61 as libc::c_char };
static mut Base64: [libc::c_char; 65] =
    unsafe {
        [65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81,
         82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103,
         104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117,
         118, 119, 120, 121, 122, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 43,
         47, 0]
    };
#[no_mangle]
pub unsafe extern "C" fn b64_pton(mut src: *const libc::c_char,
                                    mut target: *mut u_char,
                                    mut targsize: size_t) -> libc::c_int {
    let mut current_block: u64;
    let mut tarindex: libc::c_int = 0;
    let mut state: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut nextbyte: u_char = 0;
    let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
    state = 0i32;
    tarindex = 0i32;
    loop  {
        let fresh13 = src;
        src = src.offset(1);
        ch = *fresh13 as libc::c_uchar as libc::c_int;
        if !(ch != 0) { break ; }
        if 0 !=
               *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int &
                   _ISspace as libc::c_int as libc::c_ushort as libc::c_int {
            continue ;
        }
        if ch == Pad64 as libc::c_int { break ; }
        pos = strchr(Base64.as_ptr(), ch);
        if pos == 0 as *mut libc::c_char {
            return 1i32.wrapping_neg()
        } else {
            match state {
                0 => {
                    if !target.is_null() {
                        if tarindex as libc::c_ulong >= targsize {
                            return 1i32.wrapping_neg()
                        } else {
                            *target.offset(tarindex as isize) =
                                ((Base64.as_ptr().offset_to(pos).expect("bad offset_to")
                                      as libc::c_long) << 2i32) as u_char
                        }
                    }
                    state = 1i32
                }
                1 => {
                    if !target.is_null() {
                        if tarindex as libc::c_ulong >= targsize {
                            return 1i32.wrapping_neg()
                        } else {
                            let ref mut fresh14 =
                                *target.offset(tarindex as isize);
                            *fresh14 =
                                (*fresh14 as libc::c_long |
                                     Base64.as_ptr().offset_to(pos).expect("bad offset_to")
                                         as libc::c_long >> 4i32) as u_char;
                            nextbyte =
                                ((Base64.as_ptr().offset_to(pos).expect("bad offset_to")
                                      as libc::c_long & 15i32 as libc::c_long)
                                     << 4i32) as u_char;
                            if ((tarindex + 1i32) as libc::c_ulong) < targsize
                               {
                                *target.offset((tarindex + 1i32) as isize) =
                                    nextbyte
                            } else if 0 != nextbyte {
                                return 1i32.wrapping_neg()
                            }
                        }
                    }
                    tarindex += 1;
                    state = 2i32
                }
                2 => {
                    if !target.is_null() {
                        if tarindex as libc::c_ulong >= targsize {
                            return 1i32.wrapping_neg()
                        } else {
                            let ref mut fresh15 =
                                *target.offset(tarindex as isize);
                            *fresh15 =
                                (*fresh15 as libc::c_long |
                                     Base64.as_ptr().offset_to(pos).expect("bad offset_to")
                                         as libc::c_long >> 2i32) as u_char;
                            nextbyte =
                                ((Base64.as_ptr().offset_to(pos).expect("bad offset_to")
                                      as libc::c_long & 3i32 as libc::c_long)
                                     << 6i32) as u_char;
                            if ((tarindex + 1i32) as libc::c_ulong) < targsize
                               {
                                *target.offset((tarindex + 1i32) as isize) =
                                    nextbyte
                            } else if 0 != nextbyte {
                                return 1i32.wrapping_neg()
                            }
                        }
                    }
                    tarindex += 1;
                    state = 3i32
                }
                3 => {
                    if !target.is_null() {
                        if tarindex as libc::c_ulong >= targsize {
                            return 1i32.wrapping_neg()
                        } else {
                            let ref mut fresh16 =
                                *target.offset(tarindex as isize);
                            *fresh16 =
                                (*fresh16 as libc::c_long |
                                     Base64.as_ptr().offset_to(pos).expect("bad offset_to")
                                         as libc::c_long) as u_char
                        }
                    }
                    tarindex += 1;
                    state = 0i32
                }
                _ => { }
            }
        }
    }
    if ch == Pad64 as libc::c_int {
        let fresh17 = src;
        src = src.offset(1);
        ch = *fresh17 as libc::c_uchar as libc::c_int;
        match state {
            0 | 1 => {
                current_block = 5611126728812655874;
                match current_block {
                    13797916685926291137 => {
                        while ch != 0 {
                            if 0 ==
                                   *(*__ctype_b_loc()).offset(ch as isize) as
                                       libc::c_int &
                                       _ISspace as libc::c_int as
                                           libc::c_ushort as libc::c_int {
                                break ;
                            }
                            let fresh18 = src;
                            src = src.offset(1);
                            ch = *fresh18 as libc::c_uchar as libc::c_int
                        }
                        if ch != Pad64 as libc::c_int {
                            return 1i32.wrapping_neg()
                        } else {
                            let fresh19 = src;
                            src = src.offset(1);
                            ch = *fresh19 as libc::c_uchar as libc::c_int
                        }
                    }
                    5611126728812655874 => { return 1i32.wrapping_neg() }
                    _ => { }
                }
                loop  {
                    if ch != 0 {
                        if 0 ==
                               *(*__ctype_b_loc()).offset(ch as isize) as
                                   libc::c_int &
                                   _ISspace as libc::c_int as libc::c_ushort
                                       as libc::c_int {
                            return 1i32.wrapping_neg()
                        } else {
                            let fresh20 = src;
                            src = src.offset(1);
                            ch = *fresh20 as libc::c_uchar as libc::c_int
                        }
                    } else if !target.is_null() &&
                                  (tarindex as libc::c_ulong) < targsize &&
                                  *target.offset(tarindex as isize) as
                                      libc::c_int != 0i32 {
                        current_block = 11194104282611034094;
                        break ;
                    } else { current_block = 17833034027772472439; break ; }
                }
                match current_block {
                    17833034027772472439 => { }
                    _ => { return 1i32.wrapping_neg() }
                }
            }
            2 => {
                current_block = 13797916685926291137;
                match current_block {
                    13797916685926291137 => {
                        while ch != 0 {
                            if 0 ==
                                   *(*__ctype_b_loc()).offset(ch as isize) as
                                       libc::c_int &
                                       _ISspace as libc::c_int as
                                           libc::c_ushort as libc::c_int {
                                break ;
                            }
                            let fresh18 = src;
                            src = src.offset(1);
                            ch = *fresh18 as libc::c_uchar as libc::c_int
                        }
                        if ch != Pad64 as libc::c_int {
                            return 1i32.wrapping_neg()
                        } else {
                            let fresh19 = src;
                            src = src.offset(1);
                            ch = *fresh19 as libc::c_uchar as libc::c_int
                        }
                    }
                    5611126728812655874 => { return 1i32.wrapping_neg() }
                    _ => { }
                }
                loop  {
                    if ch != 0 {
                        if 0 ==
                               *(*__ctype_b_loc()).offset(ch as isize) as
                                   libc::c_int &
                                   _ISspace as libc::c_int as libc::c_ushort
                                       as libc::c_int {
                            return 1i32.wrapping_neg()
                        } else {
                            let fresh20 = src;
                            src = src.offset(1);
                            ch = *fresh20 as libc::c_uchar as libc::c_int
                        }
                    } else if !target.is_null() &&
                                  (tarindex as libc::c_ulong) < targsize &&
                                  *target.offset(tarindex as isize) as
                                      libc::c_int != 0i32 {
                        current_block = 11194104282611034094;
                        break ;
                    } else { current_block = 17833034027772472439; break ; }
                }
                match current_block {
                    17833034027772472439 => { }
                    _ => { return 1i32.wrapping_neg() }
                }
            }
            3 => {
                current_block = 18317007320854588510;
                match current_block {
                    13797916685926291137 => {
                        while ch != 0 {
                            if 0 ==
                                   *(*__ctype_b_loc()).offset(ch as isize) as
                                       libc::c_int &
                                       _ISspace as libc::c_int as
                                           libc::c_ushort as libc::c_int {
                                break ;
                            }
                            let fresh18 = src;
                            src = src.offset(1);
                            ch = *fresh18 as libc::c_uchar as libc::c_int
                        }
                        if ch != Pad64 as libc::c_int {
                            return 1i32.wrapping_neg()
                        } else {
                            let fresh19 = src;
                            src = src.offset(1);
                            ch = *fresh19 as libc::c_uchar as libc::c_int
                        }
                    }
                    5611126728812655874 => { return 1i32.wrapping_neg() }
                    _ => { }
                }
                loop  {
                    if ch != 0 {
                        if 0 ==
                               *(*__ctype_b_loc()).offset(ch as isize) as
                                   libc::c_int &
                                   _ISspace as libc::c_int as libc::c_ushort
                                       as libc::c_int {
                            return 1i32.wrapping_neg()
                        } else {
                            let fresh20 = src;
                            src = src.offset(1);
                            ch = *fresh20 as libc::c_uchar as libc::c_int
                        }
                    } else if !target.is_null() &&
                                  (tarindex as libc::c_ulong) < targsize &&
                                  *target.offset(tarindex as isize) as
                                      libc::c_int != 0i32 {
                        current_block = 11194104282611034094;
                        break ;
                    } else { current_block = 17833034027772472439; break ; }
                }
                match current_block {
                    17833034027772472439 => { }
                    _ => { return 1i32.wrapping_neg() }
                }
            }
            _ => { }
        }
    } else if state != 0i32 { return 1i32.wrapping_neg() }
    return tarindex;
}

