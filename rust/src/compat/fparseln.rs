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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn fgetln(_: *mut FILE, _: *mut size_t) -> *mut libc::c_char;
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
pub type FILE = _IO_FILE;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn fparseln(mut fp: *mut FILE, mut size: *mut size_t,
                                  mut lineno: *mut size_t,
                                  mut str: *const libc::c_char,
                                  mut flags: libc::c_int)
 -> *mut libc::c_char {
    static mut dstr: [libc::c_char; 3] =
        unsafe {
            [92 as libc::c_char, 92 as libc::c_char, 35 as libc::c_char]
        };
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut esc: libc::c_char = 0;
    let mut con: libc::c_char = 0;
    let mut nl: libc::c_char = 0;
    let mut com: libc::c_char = 0;
    let mut s: size_t = 0;
    let mut len: size_t = 0i32 as size_t;
    let mut cnt: libc::c_int = 1i32;
    if str == 0 as *mut libc::c_void as *const libc::c_char {
        str = dstr.as_ptr()
    }
    esc = *str.offset(0isize);
    con = *str.offset(1isize);
    com = *str.offset(2isize);
    nl = 10 as libc::c_char;
    while 0 != cnt {
        cnt = 0i32;
        if !lineno.is_null() { *lineno = (*lineno).wrapping_add(1) }
        ptr = fgetln(fp, &mut s as *mut size_t);
        if ptr == 0 as *mut libc::c_void as *mut libc::c_char { break ; }
        if 0 != s && 0 != com as libc::c_int {
            cp = ptr;
            while cp < ptr.offset(s as isize) {
                if *cp as libc::c_int == com as libc::c_int &&
                       0 == isescaped(ptr, cp, esc as libc::c_int) {
                    s =
                        ptr.offset_to(cp).expect("bad offset_to") as
                            libc::c_long as size_t;
                    cnt =
                        (s == 0i32 as libc::c_ulong &&
                             buf ==
                                 0 as *mut libc::c_void as *mut libc::c_char)
                            as libc::c_int;
                    break ;
                } else { cp = cp.offset(1isize) }
            }
        }
        if 0 != s && 0 != nl as libc::c_int {
            cp =
                &mut *ptr.offset(s.wrapping_sub(1i32 as libc::c_ulong) as
                                     isize) as *mut libc::c_char;
            if *cp as libc::c_int == nl as libc::c_int {
                s = s.wrapping_sub(1)
            }
        }
        if 0 != s && 0 != con as libc::c_int {
            cp =
                &mut *ptr.offset(s.wrapping_sub(1i32 as libc::c_ulong) as
                                     isize) as *mut libc::c_char;
            if *cp as libc::c_int == con as libc::c_int &&
                   0 == isescaped(ptr, cp, esc as libc::c_int) {
                s = s.wrapping_sub(1);
                cnt = 1i32
            }
        }
        if s == 0i32 as libc::c_ulong &&
               buf != 0 as *mut libc::c_void as *mut libc::c_char {
            continue ;
        }
        cp =
            realloc(buf as *mut libc::c_void,
                    len.wrapping_add(s).wrapping_add(1i32 as libc::c_ulong))
                as *mut libc::c_char;
        if cp == 0 as *mut libc::c_void as *mut libc::c_char {
            free(buf as *mut libc::c_void);
            return 0 as *mut libc::c_char
        } else {
            buf = cp;
            memcpy(buf.offset(len as isize) as *mut libc::c_void,
                   ptr as *const libc::c_void, s);
            len = (len as libc::c_ulong).wrapping_add(s) as size_t as size_t;
            *buf.offset(len as isize) = 0 as libc::c_char
        }
    }
    if flags & 15i32 != 0i32 && 0 != esc as libc::c_int &&
           buf != 0 as *mut libc::c_void as *mut libc::c_char &&
           strchr(buf, esc as libc::c_int) !=
               0 as *mut libc::c_void as *mut libc::c_char {
        cp = buf;
        ptr = cp;
        while *cp.offset(0isize) as libc::c_int != 0 {
            let mut skipesc: libc::c_int = 0;
            while *cp.offset(0isize) as libc::c_int != 0 &&
                      *cp.offset(0isize) as libc::c_int != esc as libc::c_int
                  {
                let fresh1 = ptr;
                ptr = ptr.offset(1);
                let fresh0 = cp;
                cp = cp.offset(1);
                *fresh1 = *fresh0
            }
            if *cp.offset(0isize) as libc::c_int == 0 ||
                   *cp.offset(1isize) as libc::c_int == 0 {
                break ;
            }
            skipesc = 0i32;
            if *cp.offset(1isize) as libc::c_int == com as libc::c_int {
                skipesc += flags & 4i32
            }
            if *cp.offset(1isize) as libc::c_int == con as libc::c_int {
                skipesc += flags & 2i32
            }
            if *cp.offset(1isize) as libc::c_int == esc as libc::c_int {
                skipesc += flags & 1i32
            }
            if *cp.offset(1isize) as libc::c_int != com as libc::c_int &&
                   *cp.offset(1isize) as libc::c_int != con as libc::c_int &&
                   *cp.offset(1isize) as libc::c_int != esc as libc::c_int {
                skipesc = flags & 8i32
            }
            if 0 != skipesc {
                cp = cp.offset(1isize)
            } else {
                let fresh3 = ptr;
                ptr = ptr.offset(1);
                let fresh2 = cp;
                cp = cp.offset(1);
                *fresh3 = *fresh2
            }
            let fresh5 = ptr;
            ptr = ptr.offset(1);
            let fresh4 = cp;
            cp = cp.offset(1);
            *fresh5 = *fresh4
        }
        *ptr = 0 as libc::c_char;
        len = strlen(buf)
    }
    if !size.is_null() { *size = len }
    return buf;
}
unsafe extern "C" fn isescaped(mut sp: *const libc::c_char,
                               mut p: *const libc::c_char,
                               mut esc: libc::c_int) -> libc::c_int {
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut ne: size_t = 0;
    if esc == 0 {
        return 1i32
    } else {
        ne = 0i32 as size_t;
        cp = p;
        loop  {
            cp = cp.offset(-1isize);
            if !(cp >= sp && *cp as libc::c_int == esc) { break ; }
            ne = ne.wrapping_add(1)
        }
        return (ne & 1i32 as libc::c_ulong != 0i32 as libc::c_ulong) as
                   libc::c_int
    };
}

