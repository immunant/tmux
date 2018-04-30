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
    fn __uflow(_: *mut _IO_FILE_0) -> libc::c_int;
    #[no_mangle]
    fn __overflow(_: *mut _IO_FILE_0, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn _IO_getc(__fp: *mut _IO_FILE_0) -> libc::c_int;
    #[no_mangle]
    fn _IO_putc(__c: libc::c_int, __fp: *mut _IO_FILE_0) -> libc::c_int;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: *mut __va_list_tag)
     -> libc::c_int;
    #[no_mangle]
    fn __builtin_expect(_: libc::c_long, _: libc::c_long) -> libc::c_long;
    #[no_mangle]
    fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
                  __delimiter: libc::c_int, __stream: *mut FILE) -> __ssize_t;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    static mut _sys_nerr: libc::c_int;
    #[no_mangle]
    static _sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn strtod(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char)
     -> libc::c_double;
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn strtoll(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
               __base: libc::c_int) -> libc::c_longlong;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut program_invocation_name: *mut libc::c_char;
    #[no_mangle]
    static mut program_invocation_short_name: *mut libc::c_char;
    #[no_mangle]
    fn __btowc_alias(__c: libc::c_int) -> wint_t;
    #[no_mangle]
    fn __wctob_alias(__c: wint_t) -> libc::c_int;
    #[no_mangle]
    fn mbrtowc(__pwc: *mut wchar_t, __s: *const libc::c_char, __n: size_t,
               __p: *mut mbstate_t) -> size_t;
    #[no_mangle]
    fn __mbrlen(__s: *const libc::c_char, __n: size_t, __ps: *mut mbstate_t)
     -> size_t;
    #[no_mangle]
    fn reallocarray(_: *mut libc::c_void, _: size_t, _: size_t)
     -> *mut libc::c_void;
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
pub union unnamed {
    __wch: libc::c_uint,
    __wchb: [libc::c_char; 4],
}
pub type wchar_t = libc::c_int;
pub type __ssize_t = libc::c_long;
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
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type _IO_FILE_0 = _IO_FILE;
pub type __off_t = libc::c_long;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: unnamed,
}
pub type wint_t = libc::c_uint;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type mbstate_t = __mbstate_t;
pub type __off64_t = libc::c_long;
pub type FILE = _IO_FILE;
unsafe extern "C" fn vprintf(mut __fmt: *const libc::c_char,
                             mut __arg: *mut __va_list_tag) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg);
}
unsafe extern "C" fn getchar() -> libc::c_int { return _IO_getc(stdin); }
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if 0 !=
                  Some(__builtin_expect).expect("non-null function pointer")(((*__fp)._IO_read_ptr
                                                                                  >=
                                                                                  (*__fp)._IO_read_end)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_long,
                                                                             0i32
                                                                                 as
                                                                                 libc::c_long)
              {
               __uflow(__fp)
           } else {
               let fresh0 = (*__fp)._IO_read_ptr;
               (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
               *(fresh0 as *mut libc::c_uchar) as libc::c_int
           };
}
unsafe extern "C" fn getchar_unlocked() -> libc::c_int {
    return if 0 !=
                  Some(__builtin_expect).expect("non-null function pointer")(((*stdin)._IO_read_ptr
                                                                                  >=
                                                                                  (*stdin)._IO_read_end)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_long,
                                                                             0i32
                                                                                 as
                                                                                 libc::c_long)
              {
               __uflow(stdin)
           } else {
               let fresh1 = (*stdin)._IO_read_ptr;
               (*stdin)._IO_read_ptr = (*stdin)._IO_read_ptr.offset(1);
               *(fresh1 as *mut libc::c_uchar) as libc::c_int
           };
}
unsafe extern "C" fn fgetc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if 0 !=
                  Some(__builtin_expect).expect("non-null function pointer")(((*__fp)._IO_read_ptr
                                                                                  >=
                                                                                  (*__fp)._IO_read_end)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_long,
                                                                             0i32
                                                                                 as
                                                                                 libc::c_long)
              {
               __uflow(__fp)
           } else {
               let fresh2 = (*__fp)._IO_read_ptr;
               (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
               *(fresh2 as *mut libc::c_uchar) as libc::c_int
           };
}
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return _IO_putc(__c, stdout);
}
unsafe extern "C" fn fputc_unlocked(mut __c: libc::c_int,
                                    mut __stream: *mut FILE) -> libc::c_int {
    return if 0 !=
                  Some(__builtin_expect).expect("non-null function pointer")(((*__stream)._IO_write_ptr
                                                                                  >=
                                                                                  (*__stream)._IO_write_end)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_long,
                                                                             0i32
                                                                                 as
                                                                                 libc::c_long)
              {
               __overflow(__stream, __c as libc::c_uchar as libc::c_int)
           } else {
               let fresh3 = (*__stream)._IO_write_ptr;
               (*__stream)._IO_write_ptr =
                   (*__stream)._IO_write_ptr.offset(1);
               *fresh3 = __c as libc::c_char;
               *fresh3 as libc::c_uchar as libc::c_int
           };
}
unsafe extern "C" fn putc_unlocked(mut __c: libc::c_int,
                                   mut __stream: *mut FILE) -> libc::c_int {
    return if 0 !=
                  Some(__builtin_expect).expect("non-null function pointer")(((*__stream)._IO_write_ptr
                                                                                  >=
                                                                                  (*__stream)._IO_write_end)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_long,
                                                                             0i32
                                                                                 as
                                                                                 libc::c_long)
              {
               __overflow(__stream, __c as libc::c_uchar as libc::c_int)
           } else {
               let fresh4 = (*__stream)._IO_write_ptr;
               (*__stream)._IO_write_ptr =
                   (*__stream)._IO_write_ptr.offset(1);
               *fresh4 = __c as libc::c_char;
               *fresh4 as libc::c_uchar as libc::c_int
           };
}
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if 0 !=
                  Some(__builtin_expect).expect("non-null function pointer")(((*stdout)._IO_write_ptr
                                                                                  >=
                                                                                  (*stdout)._IO_write_end)
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_long,
                                                                             0i32
                                                                                 as
                                                                                 libc::c_long)
              {
               __overflow(stdout, __c as libc::c_uchar as libc::c_int)
           } else {
               let fresh5 = (*stdout)._IO_write_ptr;
               (*stdout)._IO_write_ptr = (*stdout)._IO_write_ptr.offset(1);
               *fresh5 = __c as libc::c_char;
               *fresh5 as libc::c_uchar as libc::c_int
           };
}
unsafe extern "C" fn getline(mut __lineptr: *mut *mut libc::c_char,
                             mut __n: *mut size_t, mut __stream: *mut FILE)
 -> __ssize_t {
    return __getdelim(__lineptr, __n, 10, __stream);
}
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 16i32 != 0i32) as libc::c_int;
}
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 32i32 != 0i32) as libc::c_int;
}
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10i32) as libc::c_int;
}
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10i32);
}
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char)
 -> libc::c_longlong {
    return strtoll(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                   10i32);
}
unsafe extern "C" fn gnu_dev_major(mut __dev: libc::c_ulonglong)
 -> libc::c_uint {
    return (__dev >> 8i32 & 4095i32 as libc::c_ulonglong |
                ((__dev >> 32i32) as libc::c_uint & !4095i32 as libc::c_uint)
                    as libc::c_ulonglong) as libc::c_uint;
}
unsafe extern "C" fn gnu_dev_minor(mut __dev: libc::c_ulonglong)
 -> libc::c_uint {
    return (__dev & 255i32 as libc::c_ulonglong |
                ((__dev >> 12i32) as libc::c_uint & !255i32 as libc::c_uint)
                    as libc::c_ulonglong) as libc::c_uint;
}
unsafe extern "C" fn gnu_dev_makedev(mut __major: libc::c_uint,
                                     mut __minor: libc::c_uint)
 -> libc::c_ulonglong {
    return (__minor & 255i32 as libc::c_uint |
                (__major & 4095i32 as libc::c_uint) << 8i32) as
               libc::c_ulonglong |
               ((__minor & !255i32 as libc::c_uint) as libc::c_ulonglong) <<
                   12i32 |
               ((__major & !4095i32 as libc::c_uint) as libc::c_ulonglong) <<
                   32i32;
}
unsafe extern "C" fn bsearch(mut __key: *const libc::c_void,
                             mut __base: *const libc::c_void,
                             mut __nmemb: size_t, mut __size: size_t,
                             mut __compar: __compar_fn_t)
 -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0i32 as size_t;
    __u = __nmemb;
    loop  {
        if __l < __u {
            __idx = __l.wrapping_add(__u).wrapping_div(2i32 as libc::c_ulong);
            __p =
                (__base as
                     *const libc::c_char).offset(__idx.wrapping_mul(__size) as
                                                     isize) as
                    *mut libc::c_void;
            __comparison =
                __compar.expect("non-null function pointer")(__key, __p);
            if __comparison < 0i32 {
                __u = __idx
            } else if __comparison > 0i32 {
                __l = __idx.wrapping_add(1i32 as libc::c_ulong)
            } else { return __p as *mut libc::c_void }
        } else { return 0 as *mut libc::c_void }
    };
}
unsafe extern "C" fn mbrlen(mut __s: *const libc::c_char, mut __n: size_t,
                            mut __ps: *mut mbstate_t) -> size_t {
    return if __ps != 0 as *mut libc::c_void as *mut mbstate_t {
               mbrtowc(0 as *mut wchar_t, __s, __n, __ps)
           } else { __mbrlen(__s, __n, 0 as *mut mbstate_t) };
}
#[no_mangle]
pub unsafe extern "C" fn fgetln(mut fp: *mut FILE, mut len: *mut size_t)
 -> *mut libc::c_char {
    static mut buf: *mut libc::c_char =
        unsafe { 0 as *const libc::c_char as *mut libc::c_char };
    static mut bufsz: size_t = unsafe { 0i32 as size_t };
    let mut r: size_t = 0i32 as size_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    if 0 != fp.is_null() as libc::c_int || 0 != len.is_null() as libc::c_int {
        *__errno_location() = 22i32;
        return 0 as *mut libc::c_char
    } else {
        if buf.is_null() {
            buf =
                calloc(1i32 as libc::c_ulong, 8192i32 as libc::c_ulong) as
                    *mut libc::c_char;
            if buf.is_null() {
                return 0 as *mut libc::c_char
            } else { bufsz = 8192i32 as size_t }
        }
        loop  {
            c = _IO_getc(fp);
            if !(c != 1i32.wrapping_neg()) { break ; }
            let fresh6 = r;
            r = r.wrapping_add(1);
            *buf.offset(fresh6 as isize) = c as libc::c_char;
            if r == bufsz {
                p =
                    reallocarray(buf as *mut libc::c_void, 2i32 as size_t,
                                 bufsz) as *mut libc::c_char;
                if p.is_null() {
                    e = *__errno_location();
                    free(buf as *mut libc::c_void);
                    *__errno_location() = e;
                    buf = 0 as *mut libc::c_char;
                    bufsz = 0i32 as size_t;
                    return 0 as *mut libc::c_char
                } else {
                    buf = p;
                    bufsz = (2i32 as libc::c_ulong).wrapping_mul(bufsz)
                }
            }
            if c == 10 { break ; }
        }
        *len = r;
        return if 0 != *len { buf } else { 0 as *mut libc::c_char }
    };
}

