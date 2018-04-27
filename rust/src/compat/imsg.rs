extern crate libc;

use compat::freezero::freezero;
use compat::imsg_buffer::{msgbuf, msgbuf_init, msgbuf_clear, ibuf, ibuf_add, ibuf_free, ibuf_close, ibuf_dynamic};

extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn __cmsg_nxthdr(__mhdr: *mut msghdr, __cmsg: *mut cmsghdr)
     -> *mut cmsghdr;
    #[no_mangle]
    fn recvmsg(__fd: libc::c_int, __message: *mut msghdr,
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
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
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
    fn getdtablesize() -> libc::c_int;
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
    fn msgbuf_write(_: *mut msgbuf) -> libc::c_int; // FIXME: not being translated
    #[no_mangle]
    fn getdtablecount() -> libc::c_int;
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
pub type __u_char = libc::c_uchar;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ibuf_read {
    pub buf: [u_char; 65535],
    pub rptr: *mut u_char,
    pub wpos: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqh_first: *mut ibuf,
    pub tqh_last: *mut *mut ibuf,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type pid_t = __pid_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqe_next: *mut ibuf,
    pub tqe_prev: *mut *mut ibuf,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct imsgbuf {
    pub fds: unnamed_3,
    pub r: ibuf_read,
    pub w: msgbuf,
    pub fd: libc::c_int,
    pub pid: pid_t,
}
pub const SCM_RIGHTS: unnamed_1 = 1;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct imsg_hdr {
    pub type_0: uint32_t,
    pub len: uint16_t,
    pub flags: uint16_t,
    pub peerid: uint32_t,
    pub pid: uint32_t,
}
pub type u_char = __u_char;
pub type unnamed_1 = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type socklen_t = __socklen_t;
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
pub struct imsg_fd {
    pub entry: unnamed_4,
    pub fd: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct imsg {
    pub hdr: imsg_hdr,
    pub fd: libc::c_int,
    pub data: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: libc::c_int,
    pub cmsg_type: libc::c_int,
    pub __cmsg_data: [libc::c_uchar; 0],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    hdr: cmsghdr,
    buf: [libc::c_char; 24],
}
pub type ssize_t = __ssize_t;
pub type uint16_t = libc::c_ushort;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqh_first: *mut imsg_fd,
    pub tqh_last: *mut *mut imsg_fd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut imsg_fd,
    pub tqe_prev: *mut *mut imsg_fd,
}
pub type __socklen_t = libc::c_uint;
pub type _IO_lock_t = ();
pub type __pid_t = libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn imsg_init(mut ibuf: *mut imsgbuf,
                                   mut fd: libc::c_int) -> () {
    msgbuf_init(&mut (*ibuf).w as *mut msgbuf);
    memset(&mut (*ibuf).r as *mut ibuf_read as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<ibuf_read>() as libc::c_ulong);
    (*ibuf).fd = fd;
    (*ibuf).w.fd = fd;
    (*ibuf).pid = getpid();
    loop  {
        let ref mut fresh0 =
            (*(&mut (*ibuf).fds as *mut unnamed_3)).tqh_first;
        *fresh0 = 0 as *mut imsg_fd;
        let ref mut fresh1 = (*(&mut (*ibuf).fds as *mut unnamed_3)).tqh_last;
        *fresh1 =
            &mut (*(&mut (*ibuf).fds as *mut unnamed_3)).tqh_first as
                *mut *mut imsg_fd;
        if !(0 != 0i32) { break ; }
    };
}
#[no_mangle]
pub static mut imsg_fd_overhead: libc::c_int = unsafe { 0i32 };
#[no_mangle]
pub unsafe extern "C" fn imsg_get(mut ibuf: *mut imsgbuf, mut imsg: *mut imsg)
 -> ssize_t {
    let mut av: size_t = 0;
    let mut left: size_t = 0;
    let mut datalen: size_t = 0;
    av = (*ibuf).r.wpos;
    if ::std::mem::size_of::<imsg_hdr>() as libc::c_ulong > av {
        return 0i32 as ssize_t
    } else {
        memcpy(&mut (*imsg).hdr as *mut imsg_hdr as *mut libc::c_void,
               (*ibuf).r.buf.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<imsg_hdr>() as libc::c_ulong);
        if ((*imsg).hdr.len as libc::c_ulong) <
               ::std::mem::size_of::<imsg_hdr>() as libc::c_ulong ||
               (*imsg).hdr.len as libc::c_int > 16384i32 {
            *__errno_location() = 34i32;
            return 1i32.wrapping_neg() as ssize_t
        } else if (*imsg).hdr.len as libc::c_ulong > av {
            return 0i32 as ssize_t
        } else {
            datalen =
                ((*imsg).hdr.len as
                     libc::c_ulong).wrapping_sub(::std::mem::size_of::<imsg_hdr>()
                                                     as libc::c_ulong);
            (*ibuf).r.rptr =
                (*ibuf).r.buf.as_mut_ptr().offset(::std::mem::size_of::<imsg_hdr>()
                                                      as libc::c_ulong as
                                                      isize);
            if datalen == 0i32 as libc::c_ulong {
                (*imsg).data = 0 as *mut libc::c_void
            } else {
                (*imsg).data = malloc(datalen);
                if (*imsg).data == 0 as *mut libc::c_void {
                    return 1i32.wrapping_neg() as ssize_t
                }
            }
            if 0 != (*imsg).hdr.flags as libc::c_int & 1i32 {
                (*imsg).fd = imsg_get_fd(ibuf)
            } else { (*imsg).fd = 1i32.wrapping_neg() }
            memcpy((*imsg).data, (*ibuf).r.rptr as *const libc::c_void,
                   datalen);
            if ((*imsg).hdr.len as libc::c_ulong) < av {
                left = av.wrapping_sub((*imsg).hdr.len as libc::c_ulong);
                memmove(&mut (*ibuf).r.buf as *mut [u_char; 65535] as
                            *mut libc::c_void,
                        (*ibuf).r.buf.as_mut_ptr().offset((*imsg).hdr.len as
                                                              libc::c_int as
                                                              isize) as
                            *const libc::c_void, left);
                (*ibuf).r.wpos = left
            } else { (*ibuf).r.wpos = 0i32 as size_t }
            return datalen.wrapping_add(::std::mem::size_of::<imsg_hdr>() as
                                            libc::c_ulong) as ssize_t
        }
    };
}
unsafe extern "C" fn imsg_get_fd(mut ibuf: *mut imsgbuf) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut ifd: *mut imsg_fd = 0 as *mut imsg_fd;
    ifd = (*(&mut (*ibuf).fds as *mut unnamed_3)).tqh_first;
    if ifd == 0 as *mut libc::c_void as *mut imsg_fd {
        return 1i32.wrapping_neg()
    } else {
        fd = (*ifd).fd;
        loop  {
            if (*ifd).entry.tqe_next != 0 as *mut libc::c_void as *mut imsg_fd
               {
                (*(*ifd).entry.tqe_next).entry.tqe_prev =
                    (*ifd).entry.tqe_prev
            } else {
                let ref mut fresh2 =
                    (*(&mut (*ibuf).fds as *mut unnamed_3)).tqh_last;
                *fresh2 = (*ifd).entry.tqe_prev
            }
            *(*ifd).entry.tqe_prev = (*ifd).entry.tqe_next;
            if !(0 != 0i32) { break ; }
        }
        free(ifd as *mut libc::c_void);
        return fd
    };
}
#[no_mangle]
pub unsafe extern "C" fn imsg_compose(mut ibuf: *mut imsgbuf,
                                      mut type_0: uint32_t,
                                      mut peerid: uint32_t, mut pid: pid_t,
                                      mut fd: libc::c_int,
                                      mut data: *const libc::c_void,
                                      mut datalen: uint16_t) -> libc::c_int {
    let mut wbuf: *mut ibuf = 0 as *mut ibuf;
    wbuf = imsg_create(ibuf, type_0, peerid, pid, datalen);
    if wbuf == 0 as *mut libc::c_void as *mut ibuf {
        return 1i32.wrapping_neg()
    } else if imsg_add(wbuf, data, datalen) == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg()
    } else { (*wbuf).fd = fd; imsg_close(ibuf, wbuf); return 1i32 };
}
#[no_mangle]
pub unsafe extern "C" fn imsg_close(mut ibuf: *mut imsgbuf,
                                    mut msg: *mut ibuf) -> () {
    let mut hdr: *mut imsg_hdr = 0 as *mut imsg_hdr;
    hdr = (*msg).buf as *mut imsg_hdr;
    (*hdr).flags = ((*hdr).flags as libc::c_int & !1i32) as uint16_t;
    if (*msg).fd != 1i32.wrapping_neg() {
        (*hdr).flags = ((*hdr).flags as libc::c_int | 1i32) as uint16_t
    }
    (*hdr).len = (*msg).wpos as uint16_t;
    ibuf_close(&mut (*ibuf).w as *mut msgbuf, msg);
}
#[no_mangle]
pub unsafe extern "C" fn imsg_add(mut msg: *mut ibuf,
                                  mut data: *const libc::c_void,
                                  mut datalen: uint16_t) -> libc::c_int {
    if 0 != datalen {
        if ibuf_add(msg, data, datalen as size_t) == 1i32.wrapping_neg() {
            ibuf_free(msg);
            return 1i32.wrapping_neg()
        }
    }
    return datalen as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_create(mut ibuf: *mut imsgbuf,
                                     mut type_0: uint32_t,
                                     mut peerid: uint32_t, mut pid: pid_t,
                                     mut datalen: uint16_t) -> *mut ibuf {
    let mut wbuf: *mut ibuf = 0 as *mut ibuf;
    let mut hdr: imsg_hdr =
        imsg_hdr{type_0: 0, len: 0, flags: 0, peerid: 0, pid: 0,};
    datalen =
        (datalen as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<imsg_hdr>() as
                                             libc::c_ulong) as uint16_t as
            uint16_t;
    if datalen as libc::c_int > 16384i32 {
        *__errno_location() = 34i32;
        return 0 as *mut ibuf
    } else {
        hdr.type_0 = type_0;
        hdr.flags = 0i32 as uint16_t;
        hdr.peerid = peerid;
        hdr.pid = pid as uint32_t;
        if hdr.pid == 0i32 as libc::c_uint {
            hdr.pid = (*ibuf).pid as uint32_t
        }
        wbuf = ibuf_dynamic(datalen as size_t, 16384i32 as size_t);
        if wbuf == 0 as *mut libc::c_void as *mut ibuf {
            return 0 as *mut ibuf
        } else if imsg_add(wbuf,
                           &mut hdr as *mut imsg_hdr as *const libc::c_void,
                           ::std::mem::size_of::<imsg_hdr>() as libc::c_ulong
                               as uint16_t) == 1i32.wrapping_neg() {
            return 0 as *mut ibuf
        } else { return wbuf }
    };
}
#[no_mangle]
pub unsafe extern "C" fn imsg_composev(mut ibuf: *mut imsgbuf,
                                       mut type_0: uint32_t,
                                       mut peerid: uint32_t, mut pid: pid_t,
                                       mut fd: libc::c_int,
                                       mut iov: *const iovec,
                                       mut iovcnt: libc::c_int)
 -> libc::c_int {
    let mut wbuf: *mut ibuf = 0 as *mut ibuf;
    let mut i: libc::c_int = 0;
    let mut datalen: libc::c_int = 0i32;
    i = 0i32;
    while i < iovcnt {
        datalen =
            (datalen as
                 libc::c_ulong).wrapping_add((*iov.offset(i as
                                                              isize)).iov_len)
                as libc::c_int as libc::c_int;
        i += 1
    }
    wbuf = imsg_create(ibuf, type_0, peerid, pid, datalen as uint16_t);
    if wbuf == 0 as *mut libc::c_void as *mut ibuf {
        return 1i32.wrapping_neg()
    } else {
        i = 0i32;
        loop  {
            if i < iovcnt {
                if imsg_add(wbuf, (*iov.offset(i as isize)).iov_base,
                            (*iov.offset(i as isize)).iov_len as uint16_t) ==
                       1i32.wrapping_neg() {
                    return 1i32.wrapping_neg()
                } else { i += 1 }
            } else { (*wbuf).fd = fd; imsg_close(ibuf, wbuf); return 1i32 }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn imsg_free(mut imsg: *mut imsg) -> () {
    freezero((*imsg).data,
             ((*imsg).hdr.len as
                  libc::c_ulong).wrapping_sub(::std::mem::size_of::<imsg_hdr>()
                                                  as libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn imsg_flush(mut ibuf: *mut imsgbuf) -> libc::c_int {
    loop  {
        if 0 != (*ibuf).w.queued {
            if !(msgbuf_write(&mut (*ibuf).w as *mut msgbuf) <= 0i32) {
                continue ;
            }
            return 1i32.wrapping_neg()
        } else { return 0i32 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn imsg_clear(mut ibuf: *mut imsgbuf) -> () {
    let mut fd: libc::c_int = 0;
    msgbuf_clear(&mut (*ibuf).w as *mut msgbuf);
    loop  {
        fd = imsg_get_fd(ibuf);
        if !(fd != 1i32.wrapping_neg()) { break ; }
        close(fd);
    };
}

