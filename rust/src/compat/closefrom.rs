extern crate libc;
extern "C" {
    pub type __dirstream;
    pub type _IO_FILE_plus;
    #[no_mangle]
    static _sys_siglist: [*const libc::c_char; 65];
    #[no_mangle]
    static sys_siglist: [*const libc::c_char; 65];
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn sysconf(__name: libc::c_int) -> libc::c_long;
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
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    static mut _sys_nerr: libc::c_int;
    #[no_mangle]
    static _sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn dirfd(__dirp: *mut DIR) -> libc::c_int;
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
pub const _SC_ARG_MAX: unnamed = 0;
pub const _SC_NPROCESSORS_CONF: unnamed = 83;
pub const _SC_XBS5_ILP32_OFFBIG: unnamed = 126;
pub const _SC_REGEX_VERSION: unnamed = 156;
pub const _SC_SCHAR_MAX: unnamed = 111;
pub const _SC_XBS5_ILP32_OFF32: unnamed = 125;
pub const _SC_PII: unnamed = 53;
pub const _SC_TIMER_MAX: unnamed = 35;
pub const _SC_LINE_MAX: unnamed = 43;
pub const _SC_CHAR_MIN: unnamed = 103;
pub const _SC_TIMERS: unnamed = 11;
pub const _SC_CHARCLASS_NAME_MAX: unnamed = 45;
pub const _SC_PII_INTERNET: unnamed = 56;
pub type __ino_t = libc::c_ulong;
pub const _SC_V6_LP64_OFF64: unnamed = 178;
pub const _SC_XOPEN_SHM: unnamed = 94;
pub const _SC_PII_INTERNET_STREAM: unnamed = 61;
pub const _SC_STREAM_MAX: unnamed = 5;
pub const _SC_RTSIG_MAX: unnamed = 31;
pub const _SC_2_PBS_ACCOUNTING: unnamed = 169;
pub const _SC_NL_SETMAX: unnamed = 123;
pub const _SC_THREAD_STACK_MIN: unnamed = 75;
pub const _SC_TRACE_USER_EVENT_MAX: unnamed = 245;
pub const _SC_LEVEL2_CACHE_LINESIZE: unnamed = 193;
pub const _SC_CPUTIME: unnamed = 138;
pub const _SC_MULTI_PROCESS: unnamed = 150;
pub const _SC_ATEXIT_MAX: unnamed = 87;
pub const _SC_HOST_NAME_MAX: unnamed = 180;
pub const _SC_CHILD_MAX: unnamed = 1;
pub const _SC_PIPE: unnamed = 145;
pub const _SC_THREAD_THREADS_MAX: unnamed = 76;
pub const _SC_OPEN_MAX: unnamed = 4;
pub const _SC_SEM_NSEMS_MAX: unnamed = 32;
pub const _SC_2_PBS_CHECKPOINT: unnamed = 175;
pub const _SC_THREAD_SPORADIC_SERVER: unnamed = 161;
pub const _SC_THREAD_PRIORITY_SCHEDULING: unnamed = 79;
pub const _SC_INT_MAX: unnamed = 104;
pub const _SC_NPROCESSORS_ONLN: unnamed = 84;
pub const _SC_TRACE_EVENT_FILTER: unnamed = 182;
pub const _SC_2_PBS_TRACK: unnamed = 172;
pub const _SC_LEVEL1_DCACHE_SIZE: unnamed = 188;
pub const _SC_DEVICE_SPECIFIC_R: unnamed = 142;
pub const _SC_CLOCK_SELECTION: unnamed = 137;
pub const _SC_SIGNALS: unnamed = 158;
pub const _SC_SHARED_MEMORY_OBJECTS: unnamed = 22;
pub const _SC_NL_MSGMAX: unnamed = 121;
pub type size_t = libc::c_ulong;
pub const _SC_RE_DUP_MAX: unnamed = 44;
pub const _SC_ULONG_MAX: unnamed = 117;
pub const _SC_XOPEN_REALTIME: unnamed = 130;
pub const _SC_SAVED_IDS: unnamed = 8;
pub const _SC_2_C_DEV: unnamed = 48;
pub const _SC_MEMLOCK: unnamed = 17;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const _SC_TRACE: unnamed = 181;
pub const _SC_MQ_OPEN_MAX: unnamed = 27;
pub const _SC_NZERO: unnamed = 109;
pub const _SC_CHAR_MAX: unnamed = 102;
pub const _SC_2_LOCALEDEF: unnamed = 52;
pub const _SC_ADVISORY_INFO: unnamed = 132;
pub const _SC_CLK_TCK: unnamed = 2;
pub const _SC_BC_SCALE_MAX: unnamed = 38;
pub const _SC_LEVEL2_CACHE_ASSOC: unnamed = 192;
pub const _SC_TRACE_EVENT_NAME_MAX: unnamed = 242;
pub const _SC_C_LANG_SUPPORT: unnamed = 135;
pub const _SC_LEVEL1_ICACHE_SIZE: unnamed = 185;
pub const _SC_NL_NMAX: unnamed = 122;
pub const _SC_LEVEL4_CACHE_ASSOC: unnamed = 198;
pub const _SC_TRACE_INHERIT: unnamed = 183;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub const _SC_PII_INTERNET_DGRAM: unnamed = 62;
pub const _SC_BC_STRING_MAX: unnamed = 39;
pub const _SC_SELECT: unnamed = 59;
pub const _SC_PII_OSI_M: unnamed = 65;
pub const _SC_MEMLOCK_RANGE: unnamed = 18;
pub const _SC_WORD_BIT: unnamed = 107;
pub const _SC_2_C_VERSION: unnamed = 96;
pub const _SC_V7_LPBIG_OFFBIG: unnamed = 240;
pub const _SC_LEVEL3_CACHE_ASSOC: unnamed = 195;
pub const _SC_VERSION: unnamed = 29;
pub const _SC_UIO_MAXIOV: unnamed = 60;
pub const _SC_SYNCHRONIZED_IO: unnamed = 14;
pub const _SC_LONG_BIT: unnamed = 106;
pub const _SC_MAPPED_FILES: unnamed = 16;
pub const _SC_LOGIN_NAME_MAX: unnamed = 71;
pub type DIR = __dirstream;
pub const _SC_2_PBS_MESSAGE: unnamed = 171;
pub const _SC_EXPR_NEST_MAX: unnamed = 42;
pub const _SC_FILE_LOCKING: unnamed = 147;
pub const _SC_NGROUPS_MAX: unnamed = 3;
pub const _SC_SEM_VALUE_MAX: unnamed = 33;
pub const _SC_MB_LEN_MAX: unnamed = 108;
pub const _SC_V6_LPBIG_OFFBIG: unnamed = 179;
pub const _SC_XOPEN_VERSION: unnamed = 89;
pub const _SC_CHAR_BIT: unnamed = 101;
pub const _SC_XOPEN_UNIX: unnamed = 91;
pub const _SC_PRIORITIZED_IO: unnamed = 13;
pub const _SC_USHRT_MAX: unnamed = 118;
pub const _SC_SPIN_LOCKS: unnamed = 154;
pub const _SC_PII_XTI: unnamed = 54;
pub const _SC_XOPEN_LEGACY: unnamed = 129;
pub const _SC_LEVEL1_DCACHE_LINESIZE: unnamed = 190;
pub const _SC_TIMEOUTS: unnamed = 164;
pub type __pid_t = libc::c_int;
pub const _SC_2_FORT_RUN: unnamed = 50;
pub const _SC_REGEXP: unnamed = 155;
pub const _SC_DEVICE_SPECIFIC: unnamed = 141;
pub const _SC_THREAD_PRIO_INHERIT: unnamed = 80;
pub const _SC_PHYS_PAGES: unnamed = 85;
pub const _SC_LEVEL3_CACHE_SIZE: unnamed = 194;
pub const _SC_AIO_PRIO_DELTA_MAX: unnamed = 25;
pub const _SC_TTY_NAME_MAX: unnamed = 72;
pub const _SC_AVPHYS_PAGES: unnamed = 86;
pub const _SC_LEVEL1_DCACHE_ASSOC: unnamed = 189;
pub const _SC_ASYNCHRONOUS_IO: unnamed = 12;
pub const _SC_STREAMS: unnamed = 174;
pub const _SC_XOPEN_XPG4: unnamed = 100;
pub const _SC_PASS_MAX: unnamed = 88;
pub const _SC_THREAD_KEYS_MAX: unnamed = 74;
pub const _SC_USER_GROUPS_R: unnamed = 167;
pub const _SC_MONOTONIC_CLOCK: unnamed = 149;
pub const _SC_FILE_ATTRIBUTES: unnamed = 146;
pub const _SC_BC_BASE_MAX: unnamed = 36;
pub const _SC_USER_GROUPS: unnamed = 166;
pub const _SC_V6_ILP32_OFF32: unnamed = 176;
pub const _SC_COLL_WEIGHTS_MAX: unnamed = 40;
pub const _SC_NL_TEXTMAX: unnamed = 124;
pub const _SC_TRACE_SYS_MAX: unnamed = 244;
pub const _SC_TRACE_LOG: unnamed = 184;
pub const _SC_DELAYTIMER_MAX: unnamed = 26;
pub const _SC_FILE_SYSTEM: unnamed = 148;
pub const _SC_UCHAR_MAX: unnamed = 115;
pub const _SC_SHRT_MIN: unnamed = 114;
pub const _SC_SYSTEM_DATABASE_R: unnamed = 163;
pub const _SC_XOPEN_XPG2: unnamed = 98;
pub const _SC_BARRIERS: unnamed = 133;
pub const _SC_SCHAR_MIN: unnamed = 112;
pub const _SC_2_PBS: unnamed = 168;
pub const _SC_LEVEL1_ICACHE_LINESIZE: unnamed = 187;
pub const _SC_TZNAME_MAX: unnamed = 6;
pub const _SC_THREADS: unnamed = 67;
pub const _SC_PII_OSI: unnamed = 57;
pub const _SC_BC_DIM_MAX: unnamed = 37;
pub const _SC_JOB_CONTROL: unnamed = 7;
pub const _SC_TRACE_NAME_MAX: unnamed = 243;
pub const _SC_NL_LANGMAX: unnamed = 120;
pub const _SC_V6_ILP32_OFFBIG: unnamed = 177;
pub const _SC_NETWORKING: unnamed = 152;
pub const _SC_MQ_PRIO_MAX: unnamed = 28;
pub const _SC_REALTIME_SIGNALS: unnamed = 9;
pub const _SC_FD_MGMT: unnamed = 143;
pub const _SC_XOPEN_XCU_VERSION: unnamed = 90;
pub const _SC_C_LANG_SUPPORT_R: unnamed = 136;
pub const _SC_XBS5_LP64_OFF64: unnamed = 127;
pub const _SC_SPAWN: unnamed = 159;
pub const _SC_MEMORY_PROTECTION: unnamed = 19;
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
pub const _SC_2_CHAR_TERM: unnamed = 95;
pub const _SC_THREAD_PROCESS_SHARED: unnamed = 82;
pub const _SC_SS_REPL_MAX: unnamed = 241;
pub const _SC_T_IOV_MAX: unnamed = 66;
pub const _SC_LEVEL1_ICACHE_ASSOC: unnamed = 186;
pub const _SC_LEVEL4_CACHE_LINESIZE: unnamed = 199;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: unnamed = 248;
pub const _SC_READER_WRITER_LOCKS: unnamed = 153;
pub type _IO_lock_t = ();
pub const _SC_2_UPE: unnamed = 97;
pub const _SC_THREAD_CPUTIME: unnamed = 139;
pub const _SC_XBS5_LPBIG_OFFBIG: unnamed = 128;
pub const _SC_IPV6: unnamed = 235;
pub const _SC_XOPEN_ENH_I18N: unnamed = 93;
pub const _SC_SSIZE_MAX: unnamed = 110;
pub const _SC_INT_MIN: unnamed = 105;
pub const _SC_FSYNC: unnamed = 15;
pub const _SC_2_PBS_LOCATE: unnamed = 170;
pub const _SC_GETGR_R_SIZE_MAX: unnamed = 69;
pub const _SC_SPORADIC_SERVER: unnamed = 160;
pub const _SC_SHELL: unnamed = 157;
pub const _SC_RAW_SOCKETS: unnamed = 236;
pub const _SC_THREAD_PRIO_PROTECT: unnamed = 81;
pub const _SC_2_FORT_DEV: unnamed = 49;
pub const _SC_PAGESIZE: unnamed = 30;
pub const _SC_PII_OSI_COTS: unnamed = 63;
pub const _SC_SEMAPHORES: unnamed = 21;
pub const _SC_SYSTEM_DATABASE: unnamed = 162;
pub const _SC_2_C_BIND: unnamed = 47;
pub const _SC_GETPW_R_SIZE_MAX: unnamed = 70;
pub const _SC_2_VERSION: unnamed = 46;
pub const _SC_PII_SOCKET: unnamed = 55;
pub const _SC_THREAD_SAFE_FUNCTIONS: unnamed = 68;
pub const _SC_XOPEN_REALTIME_THREADS: unnamed = 131;
pub const _SC_IOV_MAX: unnamed = 60;
pub const _SC_UINT_MAX: unnamed = 116;
pub const _SC_XOPEN_STREAMS: unnamed = 246;
pub const _SC_FIFO: unnamed = 144;
pub const _SC_2_SW_DEV: unnamed = 51;
pub const _SC_V7_ILP32_OFFBIG: unnamed = 238;
pub type __off_t = libc::c_long;
pub const _SC_PII_OSI_CLTS: unnamed = 64;
pub const _SC_V7_ILP32_OFF32: unnamed = 237;
pub type unnamed = libc::c_uint;
pub const _SC_XOPEN_XPG3: unnamed = 99;
pub const _SC_THREAD_ATTR_STACKSIZE: unnamed = 78;
pub const _SC_LEVEL2_CACHE_SIZE: unnamed = 191;
pub const _SC_MESSAGE_PASSING: unnamed = 20;
pub const _SC_V7_LP64_OFF64: unnamed = 239;
pub const _SC_THREAD_ATTR_STACKADDR: unnamed = 77;
pub const _SC_LEVEL3_CACHE_LINESIZE: unnamed = 196;
pub const _SC_LEVEL4_CACHE_SIZE: unnamed = 197;
pub const _SC_SINGLE_PROCESS: unnamed = 151;
pub const _SC_XOPEN_CRYPT: unnamed = 92;
pub const _SC_POLL: unnamed = 58;
pub const _SC_SYMLOOP_MAX: unnamed = 173;
pub const _SC_AIO_LISTIO_MAX: unnamed = 23;
pub const _SC_EQUIV_CLASS_MAX: unnamed = 41;
pub const _SC_SHRT_MAX: unnamed = 113;
pub const _SC_BASE: unnamed = 134;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: unnamed = 247;
pub const _SC_AIO_MAX: unnamed = 24;
pub const _SC_SIGQUEUE_MAX: unnamed = 34;
pub const _SC_PRIORITY_SCHEDULING: unnamed = 10;
pub const _SC_DEVICE_IO: unnamed = 140;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: unnamed = 73;
pub const _SC_NL_ARGMAX: unnamed = 119;
pub const _SC_TYPED_MEMORY_OBJECTS: unnamed = 165;
pub type __off64_t = libc::c_long;
#[no_mangle]
pub unsafe extern "C" fn closefrom(mut lowfd: libc::c_int) -> () {
    let mut fd: libc::c_long = 0;
    let mut maxfd: libc::c_long = 0;
    let mut fdpath: [libc::c_char; 4096] = [0; 4096];
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dent: *mut dirent = 0 as *mut dirent;
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut len: libc::c_int = 0;
    len =
        snprintf(fdpath.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 4096]>() as
                     libc::c_ulong,
                 b"/proc/%ld/fd\x00" as *const u8 as *const libc::c_char,
                 getpid() as libc::c_long);
    if len > 0i32 &&
           len as size_t <=
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
           && { dirp = opendir(fdpath.as_mut_ptr()); !dirp.is_null() } {
        loop  {
            dent = readdir(dirp);
            if !(dent != 0 as *mut libc::c_void as *mut dirent) { break ; }
            fd =
                strtol((*dent).d_name.as_mut_ptr(),
                       &mut endp as *mut *mut libc::c_char, 10i32);
            if !((*dent).d_name.as_mut_ptr() != endp &&
                     *endp as libc::c_int == 0 && fd >= 0i32 as libc::c_long
                     && fd < 2147483647i32 as libc::c_long &&
                     fd >= lowfd as libc::c_long &&
                     fd != dirfd(dirp) as libc::c_long) {
                continue ;
            }
            close(fd as libc::c_int);
        }
        closedir(dirp);
    } else {
        maxfd = sysconf(_SC_OPEN_MAX as libc::c_int);
        if maxfd < 0i32 as libc::c_long { maxfd = 256i32 as libc::c_long }
        fd = lowfd as libc::c_long;
        while fd < maxfd { close(fd as libc::c_int); fd += 1 }
    };
}

