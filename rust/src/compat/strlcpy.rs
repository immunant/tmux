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
pub type _IO_lock_t = ();
pub type __off64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn strlcpy(mut dst: *mut libc::c_char,
                                 mut src: *const libc::c_char,
                                 mut siz: size_t) -> libc::c_ulong {
    let mut current_block: u64;
    let mut d: *mut libc::c_char = dst;
    let mut s: *const libc::c_char = src;
    let mut n: size_t = siz;
    if n != 0i32 as libc::c_ulong &&
           { n = n.wrapping_sub(1); n != 0i32 as libc::c_ulong } {
        current_block = 4988723283678924448;
    } else { current_block = 11174649648027449784; }
    loop  {
        match current_block {
            4988723283678924448 => {
                let fresh1 = d;
                d = d.offset(1);
                let fresh0 = s;
                s = s.offset(1);
                *fresh1 = *fresh0;
                if *fresh1 as libc::c_int == 0i32 {
                    current_block = 11174649648027449784;
                    continue ;
                }
                n = n.wrapping_sub(1);
                if n != 0i32 as libc::c_ulong {
                    current_block = 4988723283678924448;
                } else { current_block = 11174649648027449784; }
            }
            _ => {
                if !(n == 0i32 as libc::c_ulong) {
                    current_block = 4644295000439058019;
                    break ;
                }
                if siz != 0i32 as libc::c_ulong {
                    current_block = 7502529970979898288;
                    break ;
                } else { current_block = 7095457783677275021; break ; }
            }
        }
    }
    match current_block {
        7502529970979898288 => {
            *d = 0 as libc::c_char;
            current_block = 7095457783677275021;
        }
        _ => { }
    }
    loop  {
        match current_block {
            4644295000439058019 => {
                return (src.offset_to(s).expect("bad offset_to") as
                            libc::c_long - 1i32 as libc::c_long) as
                           libc::c_ulong
            }
            _ => {
                let fresh2 = s;
                s = s.offset(1);
                if 0 != *fresh2 {
                    current_block = 7095457783677275021;
                } else { current_block = 4644295000439058019; }
            }
        }
    };
}

