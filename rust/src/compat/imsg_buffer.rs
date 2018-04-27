extern crate libc;

const IOV_MAX: u64 = 1024;

extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int)
     -> ssize_t;
    #[no_mangle]
    fn sendmsg(__fd: libc::c_int, __message: *const msghdr,
               __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
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
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn recallocarray(_: *mut libc::c_void, _: size_t, _: size_t, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn freezero(_: *mut libc::c_void, _: size_t) -> ();
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
pub struct unnamed {
    pub tqh_first: *mut ibuf,
    pub tqh_last: *mut *mut ibuf,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type __off64_t = libc::c_long;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: libc::c_int,
    pub cmsg_type: libc::c_int,
    pub __cmsg_data: [libc::c_uchar; 0],
}
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct msgbuf {
    pub bufs: unnamed,
    pub queued: uint32_t,
    pub fd: libc::c_int,
}
pub type _IO_lock_t = ();
pub type ssize_t = __ssize_t;
pub type __caddr_t = *mut libc::c_char;
pub const SCM_RIGHTS: unnamed_1 = 1;
pub type __ssize_t = libc::c_long;
pub type caddr_t = __caddr_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ibuf {
    pub entry: unnamed_0,
    pub buf: *mut u_char,
    pub size: size_t,
    pub max: size_t,
    pub wpos: size_t,
    pub rpos: size_t,
    pub fd: libc::c_int,
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqe_next: *mut ibuf,
    pub tqe_prev: *mut *mut ibuf,
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
pub type socklen_t = __socklen_t;
pub type unnamed_1 = libc::c_uint;
pub type __socklen_t = libc::c_uint;
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: libc::c_int,
}
pub type u_char = __u_char;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    hdr: cmsghdr,
    buf: [libc::c_char; 24],
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_open(mut len: size_t) -> *mut ibuf {
    let mut buf: *mut ibuf = 0 as *mut ibuf;
    buf =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<ibuf>() as libc::c_ulong) as *mut ibuf;
    if buf == 0 as *mut libc::c_void as *mut ibuf {
        return 0 as *mut ibuf
    } else {
        (*buf).buf = malloc(len) as *mut u_char;
        if (*buf).buf == 0 as *mut libc::c_void as *mut u_char {
            free(buf as *mut libc::c_void);
            return 0 as *mut ibuf
        } else {
            (*buf).max = len;
            (*buf).size = (*buf).max;
            (*buf).fd = 1i32.wrapping_neg();
            return buf
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_dynamic(mut len: size_t, mut max: size_t)
 -> *mut ibuf {
    let mut buf: *mut ibuf = 0 as *mut ibuf;
    if max < len {
        return 0 as *mut ibuf
    } else {
        buf = ibuf_open(len);
        if buf == 0 as *mut libc::c_void as *mut ibuf {
            return 0 as *mut ibuf
        } else {
            if max > 0i32 as libc::c_ulong { (*buf).max = max }
            return buf
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_add(mut buf: *mut ibuf,
                                  mut data: *const libc::c_void,
                                  mut len: size_t) -> libc::c_int {
    if (*buf).wpos.wrapping_add(len) > (*buf).size {
        if ibuf_realloc(buf, len) == 1i32.wrapping_neg() {
            return 1i32.wrapping_neg()
        }
    }
    memcpy((*buf).buf.offset((*buf).wpos as isize) as *mut libc::c_void, data,
           len);
    (*buf).wpos =
        ((*buf).wpos as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    return 0i32;
}
unsafe extern "C" fn ibuf_realloc(mut buf: *mut ibuf, mut len: size_t)
 -> libc::c_int {
    let mut b: *mut u_char = 0 as *mut u_char;
    if (*buf).wpos.wrapping_add(len) > (*buf).max {
        *__errno_location() = 34i32;
        return 1i32.wrapping_neg()
    } else {
        b =
            recallocarray((*buf).buf as *mut libc::c_void, (*buf).size,
                          (*buf).wpos.wrapping_add(len), 1i32 as size_t) as
                *mut u_char;
        if b == 0 as *mut libc::c_void as *mut u_char {
            return 1i32.wrapping_neg()
        } else {
            (*buf).buf = b;
            (*buf).size = (*buf).wpos.wrapping_add(len);
            return 0i32
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_reserve(mut buf: *mut ibuf, mut len: size_t)
 -> *mut libc::c_void {
    let mut b: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*buf).wpos.wrapping_add(len) > (*buf).size {
        if ibuf_realloc(buf, len) == 1i32.wrapping_neg() {
            return 0 as *mut libc::c_void
        }
    }
    b = (*buf).buf.offset((*buf).wpos as isize) as *mut libc::c_void;
    (*buf).wpos =
        ((*buf).wpos as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_seek(mut buf: *mut ibuf, mut pos: size_t,
                                   mut len: size_t) -> *mut libc::c_void {
    if pos.wrapping_add(len) > (*buf).wpos {
        return 0 as *mut libc::c_void
    } else { return (*buf).buf.offset(pos as isize) as *mut libc::c_void };
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_size(mut buf: *mut ibuf) -> size_t {
    return (*buf).wpos;
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_left(mut buf: *mut ibuf) -> size_t {
    return (*buf).max.wrapping_sub((*buf).wpos);
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_close(mut msgbuf: *mut msgbuf,
                                    mut buf: *mut ibuf) -> () {
    ibuf_enqueue(msgbuf, buf);
}
unsafe extern "C" fn ibuf_enqueue(mut msgbuf: *mut msgbuf, mut buf: *mut ibuf)
 -> () {
    loop  {
        (*buf).entry.tqe_next = 0 as *mut ibuf;
        (*buf).entry.tqe_prev =
            (*(&mut (*msgbuf).bufs as *mut unnamed)).tqh_last;
        let ref mut fresh0 =
            *(*(&mut (*msgbuf).bufs as *mut unnamed)).tqh_last;
        *fresh0 = buf;
        let ref mut fresh1 =
            (*(&mut (*msgbuf).bufs as *mut unnamed)).tqh_last;
        *fresh1 = &mut (*buf).entry.tqe_next as *mut *mut ibuf;
        if !(0 != 0i32) { break ; }
    }
    (*msgbuf).queued = (*msgbuf).queued.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_write(mut msgbuf: *mut msgbuf) -> libc::c_int {
    let mut current_block: u64;
    let mut iov: [iovec; 1024] =
        [iovec{iov_base: 0 as *mut libc::c_void, iov_len: 0,}; 1024];
    let mut buf: *mut ibuf = 0 as *mut ibuf;
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    let mut n: ssize_t = 0;
    memset(&mut iov as *mut [iovec; 1024] as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[iovec; 1024]>() as libc::c_ulong);
    buf = (*(&mut (*msgbuf).bufs as *mut unnamed)).tqh_first;
    while buf != 0 as *mut libc::c_void as *mut ibuf {
        if i >= 1024i32 as libc::c_uint { break ; }
        iov[i as usize].iov_base =
            (*buf).buf.offset((*buf).rpos as isize) as *mut libc::c_void;
        iov[i as usize].iov_len = (*buf).wpos.wrapping_sub((*buf).rpos);
        i = i.wrapping_add(1);
        buf = (*buf).entry.tqe_next
    }
    loop  {
        n = writev((*msgbuf).fd, iov.as_mut_ptr(), i as libc::c_int);
        if n == 1i32.wrapping_neg() as libc::c_long {
            if *__errno_location() == 4i32 { continue ; }
            if *__errno_location() == 105i32 {
                current_block = 6873731126896040597;
                break ;
            } else { current_block = 15427931788582360902; break ; }
        } else if n == 0i32 as libc::c_long {
            current_block = 4906268039856690917;
            break ;
        } else { current_block = 6483416627284290920; break ; }
    }
    match current_block {
        6873731126896040597 => { *__errno_location() = 11i32 }
        4906268039856690917 => { *__errno_location() = 0i32; return 0i32 }
        6483416627284290920 => {
            msgbuf_drain(msgbuf, n as size_t);
            return 1i32
        }
        _ => { }
    }
    return 1i32.wrapping_neg();
}
#[no_mangle]
pub unsafe extern "C" fn msgbuf_drain(mut msgbuf: *mut msgbuf, mut n: size_t)
 -> () {
    let mut buf: *mut ibuf = 0 as *mut ibuf;
    let mut next: *mut ibuf = 0 as *mut ibuf;
    buf = (*(&mut (*msgbuf).bufs as *mut unnamed)).tqh_first;
    while buf != 0 as *mut libc::c_void as *mut ibuf &&
              n > 0i32 as libc::c_ulong {
        next = (*buf).entry.tqe_next;
        if (*buf).rpos.wrapping_add(n) >= (*buf).wpos {
            n =
                (n as
                     libc::c_ulong).wrapping_sub((*buf).wpos.wrapping_sub((*buf).rpos))
                    as size_t as size_t;
            ibuf_dequeue(msgbuf, buf);
        } else {
            (*buf).rpos =
                ((*buf).rpos as libc::c_ulong).wrapping_add(n) as size_t as
                    size_t;
            n = 0i32 as size_t
        }
        buf = next
    };
}
unsafe extern "C" fn ibuf_dequeue(mut msgbuf: *mut msgbuf, mut buf: *mut ibuf)
 -> () {
    loop  {
        if (*buf).entry.tqe_next != 0 as *mut libc::c_void as *mut ibuf {
            (*(*buf).entry.tqe_next).entry.tqe_prev = (*buf).entry.tqe_prev
        } else {
            let ref mut fresh2 =
                (*(&mut (*msgbuf).bufs as *mut unnamed)).tqh_last;
            *fresh2 = (*buf).entry.tqe_prev
        }
        *(*buf).entry.tqe_prev = (*buf).entry.tqe_next;
        if !(0 != 0i32) { break ; }
    }
    if (*buf).fd != 1i32.wrapping_neg() { close((*buf).fd); }
    (*msgbuf).queued = (*msgbuf).queued.wrapping_sub(1);
    ibuf_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_free(mut buf: *mut ibuf) -> () {
    if buf == 0 as *mut libc::c_void as *mut ibuf {
        return
    } else {
        freezero((*buf).buf as *mut libc::c_void, (*buf).size);
        free(buf as *mut libc::c_void);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn msgbuf_init(mut msgbuf: *mut msgbuf) -> () {
    (*msgbuf).queued = 0i32 as uint32_t;
    (*msgbuf).fd = 1i32.wrapping_neg();
    loop  {
        let ref mut fresh3 =
            (*(&mut (*msgbuf).bufs as *mut unnamed)).tqh_first;
        *fresh3 = 0 as *mut ibuf;
        let ref mut fresh4 =
            (*(&mut (*msgbuf).bufs as *mut unnamed)).tqh_last;
        *fresh4 =
            &mut (*(&mut (*msgbuf).bufs as *mut unnamed)).tqh_first as
                *mut *mut ibuf;
        if !(0 != 0i32) { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn msgbuf_clear(mut msgbuf: *mut msgbuf) -> () {
    let mut buf: *mut ibuf = 0 as *mut ibuf;
    loop  {
        buf = (*(&mut (*msgbuf).bufs as *mut unnamed)).tqh_first;
        if !(buf != 0 as *mut libc::c_void as *mut ibuf) { break ; }
        ibuf_dequeue(msgbuf, buf);
    };
}

