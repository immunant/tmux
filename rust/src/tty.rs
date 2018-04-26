#![feature ( libc )]
#![feature ( i128_type )]
#![feature ( const_ptr_null )]
#![feature ( offset_to )]
#![feature ( const_ptr_null_mut )]
#![feature ( extern_types )]
#![feature ( asm )]
#![allow ( non_upper_case_globals )]
#![allow ( non_camel_case_types )]
#![allow ( non_snake_case )]
#![allow ( dead_code )]
#![allow ( mutable_transmutes )]
#![allow ( unused_mut )]
extern crate libc;
extern "C" {
    pub type format_tree;
    pub type tmuxpeer;
    pub type environ;
    pub type hooks;
    pub type tmuxproc;
    pub type format_job_tree;
    pub type _IO_FILE_plus;
    pub type args_entry;
    pub type evbuffer;
    pub type tty_code;
    pub type options;
    pub type event_base;
    pub type screen_titles;
    pub type ldat;
    pub type bufferevent_ops;
    pub type input_ctx;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, ...) -> libc::c_int;
    #[no_mangle]
    static in6addr_any: in6_addr;
    #[no_mangle]
    static in6addr_loopback: in6_addr;
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
    static mut acs_map: [chtype; 0];
    #[no_mangle]
    static mut curscr: *mut WINDOW;
    #[no_mangle]
    static mut newscr: *mut WINDOW;
    #[no_mangle]
    static mut stdscr: *mut WINDOW;
    #[no_mangle]
    static mut ttytype: [libc::c_char; 0];
    #[no_mangle]
    static mut COLORS: libc::c_int;
    #[no_mangle]
    static mut COLOR_PAIRS: libc::c_int;
    #[no_mangle]
    static mut COLS: libc::c_int;
    #[no_mangle]
    static mut ESCDELAY: libc::c_int;
    #[no_mangle]
    static mut LINES: libc::c_int;
    #[no_mangle]
    static mut TABSIZE: libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, ...) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, ...)
     -> libc::c_int;
    #[no_mangle]
    static _sys_siglist: [*const libc::c_char; 65];
    #[no_mangle]
    static sys_siglist: [*const libc::c_char; 65];
    #[no_mangle]
    static _ns_flagdata: [_ns_flagdata; 0];
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    #[no_mangle]
    fn tcsetattr(__fd: libc::c_int, __optional_actions: libc::c_int,
                 __termios_p: *const termios) -> libc::c_int;
    #[no_mangle]
    fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
    #[no_mangle]
    fn event_initialized(ev: *const event) -> libc::c_int;
    #[no_mangle]
    fn event_set(_: *mut event, _: libc::c_int, _: libc::c_short,
                 _:
                     Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_short,
                                                 _: *mut libc::c_void) -> ()>,
                 _: *mut libc::c_void) -> ();
    #[no_mangle]
    fn evbuffer_new() -> *mut evbuffer;
    #[no_mangle]
    fn evbuffer_free(buf: *mut evbuffer) -> ();
    #[no_mangle]
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn evbuffer_add(buf: *mut evbuffer, data: *const libc::c_void,
                    datlen: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_write(buffer: *mut evbuffer, fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_read(buffer: *mut evbuffer, fd: libc::c_int,
                     howmuch: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn b64_ntop(_: *const libc::c_char, _: size_t, _: *mut libc::c_char,
                _: size_t) -> libc::c_int;
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
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut global_hooks: *mut hooks;
    #[no_mangle]
    static mut global_options: *mut options;
    #[no_mangle]
    static mut global_s_options: *mut options;
    #[no_mangle]
    static mut global_w_options: *mut options;
    #[no_mangle]
    static mut global_environ: *mut environ;
    #[no_mangle]
    static mut start_time: timeval;
    #[no_mangle]
    static mut socket_path: *const libc::c_char;
    #[no_mangle]
    static mut shell_command: *const libc::c_char;
    #[no_mangle]
    static mut ptm_fd: libc::c_int;
    #[no_mangle]
    fn setblocking(_: libc::c_int, _: libc::c_int) -> ();
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn options_get_string(_: *mut options, _: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    fn options_get_style(_: *mut options, _: *const libc::c_char)
     -> *const grid_cell;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn fatal(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn tty_term_string(_: *mut tty_term, _: tty_code_code)
     -> *const libc::c_char;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn tty_acs_needed(_: *mut tty) -> libc::c_int;
    #[no_mangle]
    fn tty_term_has(_: *mut tty_term, _: tty_code_code) -> libc::c_int;
    #[no_mangle]
    fn tty_term_string1(_: *mut tty_term, _: tty_code_code, _: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_string3(_: *mut tty_term, _: tty_code_code, _: libc::c_int,
                        _: libc::c_int, _: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn colour_split_rgb(_: libc::c_int, _: *mut u_char, _: *mut u_char,
                        _: *mut u_char) -> ();
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    fn grid_cells_equal(_: *const grid_cell, _: *const grid_cell)
     -> libc::c_int;
    #[no_mangle]
    fn tty_term_flag(_: *mut tty_term, _: tty_code_code) -> libc::c_int;
    #[no_mangle]
    fn colour_256to16(_: u_char) -> u_char;
    #[no_mangle]
    fn tty_term_number(_: *mut tty_term, _: tty_code_code) -> libc::c_int;
    #[no_mangle]
    fn colour_find_rgb(_: u_char, _: u_char, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn window_pane_get_palette(_: *const window_pane, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn tty_term_string2(_: *mut tty_term, _: tty_code_code, _: libc::c_int,
                        _: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn tty_acs_get(_: *mut tty, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_ptr1(_: *mut tty_term, _: tty_code_code,
                     _: *const libc::c_void) -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_ptr2(_: *mut tty_term, _: tty_code_code,
                     _: *const libc::c_void, _: *const libc::c_void)
     -> *const libc::c_char;
    #[no_mangle]
    fn screen_select_cell(_: *mut screen, _: *mut grid_cell,
                          _: *const grid_cell) -> ();
    #[no_mangle]
    fn grid_view_get_cell(_: *mut grid, _: u_int, _: u_int, _: *mut grid_cell)
     -> ();
    #[no_mangle]
    fn tty_keys_build(_: *mut tty) -> ();
    #[no_mangle]
    fn tty_keys_next(_: *mut tty) -> key_code;
    #[no_mangle]
    fn server_client_lost(_: *mut client) -> ();
    #[no_mangle]
    fn tty_keys_free(_: *mut tty) -> ();
    #[no_mangle]
    fn tty_term_free(_: *mut tty_term) -> ();
    #[no_mangle]
    fn tty_term_find(_: *mut libc::c_char, _: libc::c_int,
                     _: *mut *mut libc::c_char) -> *mut tty_term;
    #[no_mangle]
    fn status_line_size(_: *mut session) -> u_int;
    #[no_mangle]
    fn status_at_line(_: *mut client) -> libc::c_int;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn window_pane_visible(_: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    static window_buffer_mode: window_mode;
    #[no_mangle]
    static window_tree_mode: window_mode;
    #[no_mangle]
    static window_clock_mode: window_mode;
    #[no_mangle]
    static window_clock_table: [[[libc::c_char; 5]; 5]; 14];
    #[no_mangle]
    static window_client_mode: window_mode;
    #[no_mangle]
    static window_copy_mode: window_mode;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    static mut session_groups: session_groups;
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
pub type speed_t = libc::c_uint;
pub const TTYC_KNXT2: tty_code_code = 154;
pub const JOB_RUNNING: unnamed_36 = 0;
pub const TTYC_KF31: tty_code_code = 92;
pub const TTYC_KCUU1: tty_code_code = 47;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_27,
}
pub const TTYC_KCUD1: tty_code_code = 45;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_2,
}
pub const TTYC_KF51: tty_code_code = 114;
pub type uint8_t = libc::c_uchar;
pub const TTYC_TC: tty_code_code = 202;
pub const TTYC_KF11: tty_code_code = 70;
pub const TTYC_INVIS: tty_code_code = 42;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session {
    pub id: u_int,
    pub name: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub creation_time: timeval,
    pub last_attached_time: timeval,
    pub activity_time: timeval,
    pub last_activity_time: timeval,
    pub lock_timer: event,
    pub sx: u_int,
    pub sy: u_int,
    pub curw: *mut winlink,
    pub lastw: winlink_stack,
    pub windows: winlinks,
    pub statusat: libc::c_int,
    pub hooks: *mut hooks,
    pub options: *mut options,
    pub flags: libc::c_int,
    pub attached: u_int,
    pub tio: *mut termios,
    pub environ: *mut environ,
    pub references: libc::c_int,
    pub gentry: unnamed_12,
    pub entry: unnamed,
}
pub const TTYC_KF8: tty_code_code = 129;
pub type size_t = libc::c_ulong;
pub const TTYC_KF13: tty_code_code = 72;
pub const TTYC_KDN6: tty_code_code = 59;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const TTYC_KF61: tty_code_code = 125;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    ev_io: unnamed_29,
    ev_signal: unnamed_23,
}
pub type uint16_t = libc::c_ushort;
pub const TTYC_KHOM6: tty_code_code = 135;
pub const TTYC_KDN4: tty_code_code = 57;
pub type options_table_scope = libc::c_uint;
pub const TTYC_IL: tty_code_code = 39;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const TTYC_KIC7: tty_code_code = 143;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const TTYC_BLINK: tty_code_code = 4;
pub const TTYC_KF4: tty_code_code = 101;
pub const TTYC_KF28: tty_code_code = 88;
pub const TTYC_KF12: tty_code_code = 71;
pub const TTYC_KDC5: tty_code_code = 51;
pub const TTYC_KF39: tty_code_code = 100;
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const TTYC_KEND5: tty_code_code = 65;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const TTYC_CLEAR: tty_code_code = 7;
pub const TTYC_KUP4: tty_code_code = 176;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub const TTYC_SE: tty_code_code = 188;
pub const TTYC_KEND6: tty_code_code = 66;
pub type unnamed_4 = libc::c_uint;
pub const JOB_DEAD: unnamed_36 = 1;
pub type unnamed_5 = libc::c_uint;
pub const TTYC_KF58: tty_code_code = 121;
pub type __off64_t = libc::c_long;
pub type options_table_type = libc::c_uint;
pub const TTYC_KEND2: tty_code_code = 62;
pub const TTYC_TSL: tty_code_code = 203;
pub const TTYC_KUP6: tty_code_code = 178;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub const TTYC_KDN3: tty_code_code = 56;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const TTYC_KLFT3: tty_code_code = 147;
pub const TTYC_KF60: tty_code_code = 124;
pub const TTYC_FSL: tty_code_code = 34;
pub const TTYC_KF35: tty_code_code = 96;
pub const TTYC_XENL: tty_code_code = 206;
pub const TTYC_KRIT5: tty_code_code = 171;
pub type cc_t = libc::c_uchar;
pub const TTYC_SMSO: tty_code_code = 198;
pub const TTYC_DL: tty_code_code = 26;
pub const TTYC_KF49: tty_code_code = 111;
pub const TTYC_CR: tty_code_code = 10;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct bufferevent {
    pub ev_base: *mut event_base,
    pub be_ops: *const bufferevent_ops,
    pub ev_read: event,
    pub ev_write: event,
    pub input: *mut evbuffer,
    pub output: *mut evbuffer,
    pub wm_read: event_watermark,
    pub wm_write: event_watermark,
    pub readcb: bufferevent_data_cb,
    pub writecb: bufferevent_data_cb,
    pub errorcb: bufferevent_event_cb,
    pub cbarg: *mut libc::c_void,
    pub timeout_read: timeval,
    pub timeout_write: timeval,
    pub enabled: libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_10,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const TTYC_KF46: tty_code_code = 108;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct client {
    pub name: *const libc::c_char,
    pub peer: *mut tmuxpeer,
    pub queue: cmdq_list,
    pub pid: pid_t,
    pub fd: libc::c_int,
    pub event: event,
    pub retval: libc::c_int,
    pub creation_time: timeval,
    pub activity_time: timeval,
    pub environ: *mut environ,
    pub jobs: *mut format_job_tree,
    pub title: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub term: *mut libc::c_char,
    pub ttyname: *mut libc::c_char,
    pub tty: tty,
    pub written: size_t,
    pub discarded: size_t,
    pub redraw: size_t,
    pub stdin_callback: Option<unsafe extern "C" fn(_: *mut client,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_void)
                                   -> ()>,
    pub stdin_callback_data: *mut libc::c_void,
    pub stdin_data: *mut evbuffer,
    pub stdin_closed: libc::c_int,
    pub stdout_data: *mut evbuffer,
    pub stderr_data: *mut evbuffer,
    pub repeat_timer: event,
    pub click_timer: event,
    pub click_button: u_int,
    pub status: status_line,
    pub flags: libc::c_int,
    pub keytable: *mut key_table,
    pub identify_timer: event,
    pub identify_callback: Option<unsafe extern "C" fn(_: *mut client,
                                                       _: *mut window_pane)
                                      -> ()>,
    pub identify_callback_data: *mut libc::c_void,
    pub message_string: *mut libc::c_char,
    pub message_timer: event,
    pub message_next: u_int,
    pub message_log: unnamed_11,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_4,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_19,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub type __time_t = libc::c_long;
pub type __u_short = libc::c_ushort;
pub const TTYC_CUP: tty_code_code = 19;
pub type __pid_t = libc::c_int;
pub const TTYC_KPRV2: tty_code_code = 161;
pub const TTYC_OP: tty_code_code = 181;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_find_state {
    pub flags: libc::c_int,
    pub current: *mut cmd_find_state,
    pub s: *mut session,
    pub wl: *mut winlink,
    pub w: *mut window,
    pub wp: *mut window_pane,
    pub idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub const TTY_VT100: unnamed_31 = 0;
pub const TTYC_COLORS: tty_code_code = 9;
pub const TTYC_CNORM: tty_code_code = 8;
pub const TTYC_KF34: tty_code_code = 95;
pub const TTYC_SITM: tty_code_code = 194;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const TTYC_KDN5: tty_code_code = 58;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const TTYC_KEND7: tty_code_code = 67;
pub const TTYC_KEND3: tty_code_code = 63;
pub const TTYC_KF18: tty_code_code = 77;
pub const TTYC_HPA: tty_code_code = 36;
pub const TTYC_RI: tty_code_code = 184;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const TTYC_KNP: tty_code_code = 153;
pub const LINE_SEL_RIGHT_LEFT: unnamed_5 = 2;
pub const TTYC_KF41: tty_code_code = 103;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const TTYC_KRIT6: tty_code_code = 172;
pub const TTYC_ECH: tty_code_code = 29;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const TTYC_KHOM7: tty_code_code = 136;
pub const TTYC_KIC2: tty_code_code = 138;
pub const TTYC_SETAF: tty_code_code = 190;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub type ssize_t = __ssize_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink {
    pub idx: libc::c_int,
    pub session: *mut session,
    pub window: *mut window,
    pub status_width: size_t,
    pub status_cell: grid_cell,
    pub status_text: *mut libc::c_char,
    pub flags: libc::c_int,
    pub entry: unnamed_8,
    pub wentry: unnamed_7,
    pub sentry: unnamed_26,
}
pub const TTYC_U8: tty_code_code = 204;
pub const TTYC_KDN2: tty_code_code = 55;
pub const TTYC_SMACS: tty_code_code = 195;
pub const TTYC_KNXT6: tty_code_code = 158;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const TTYC_ED: tty_code_code = 30;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_13 {
    offset: u_int,
    data: unnamed_15,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const TTYC_KF23: tty_code_code = 83;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen {
    pub title: *mut libc::c_char,
    pub titles: *mut screen_titles,
    pub grid: *mut grid,
    pub cx: u_int,
    pub cy: u_int,
    pub cstyle: u_int,
    pub ccolour: *mut libc::c_char,
    pub rupper: u_int,
    pub rlower: u_int,
    pub mode: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: screen_sel,
}
pub const TTYC_KPRV5: tty_code_code = 164;
pub const TTYC_SETRGBF: tty_code_code = 192;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_13,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_line {
    pub cellused: u_int,
    pub cellsize: u_int,
    pub celldata: *mut grid_cell_entry,
    pub extdsize: u_int,
    pub extddata: *mut grid_cell,
    pub flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const PROMPT_ENTRY: unnamed_4 = 0;
pub const TTYC_CS: tty_code_code = 11;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type chtype = libc::c_ulong;
pub const TTYC_MS: tty_code_code = 180;
pub const TTY_VT420: unnamed_31 = 5;
pub type cmdq_type = libc::c_uint;
pub const TTYC_KF17: tty_code_code = 76;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub const TTYC_SMKX: tty_code_code = 197;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const TTYC_CUU: tty_code_code = 20;
pub const TTYC_E3: tty_code_code = 28;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const TTYC_KCUB1: tty_code_code = 44;
pub const TTYC_KMOUS: tty_code_code = 152;
pub const TTYC_KF16: tty_code_code = 75;
pub const TTYC_VPA: tty_code_code = 205;
pub const TTYC_CUB1: tty_code_code = 14;
pub type cmd_find_type = libc::c_uint;
pub const TTYC_KF52: tty_code_code = 115;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const TTYC_KF59: tty_code_code = 122;
pub const TTYC_KDC2: tty_code_code = 48;
pub const TTYC_KNXT7: tty_code_code = 159;
pub const TTYC_KF63: tty_code_code = 127;
pub const TTYC_KF24: tty_code_code = 84;
pub const LINE_SEL_LEFT_RIGHT: unnamed_5 = 1;
pub const TTYC_KF47: tty_code_code = 109;
pub const TTYC_KF37: tty_code_code = 98;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const TTYC_KCBT: tty_code_code = 43;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_16 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const TTYC_KF2: tty_code_code = 79;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_17 {
    ev_next_with_common_timeout: unnamed_33,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid {
    pub flags: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub hscrolled: u_int,
    pub hsize: u_int,
    pub hlimit: u_int,
    pub linedata: *mut grid_line,
}
pub const TTYC_KHOM2: tty_code_code = 131;
pub type attr_t = chtype;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_35,
    pub ev_next: unnamed_38,
    pub ev_timeout_pos: unnamed_17,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_1,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_flags: libc::c_short,
    pub ev_pri: uint8_t,
    pub ev_closure: uint8_t,
    pub ev_timeout: timeval,
    pub ev_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_short,
                                                 _: *mut libc::c_void) -> ()>,
    pub ev_arg: *mut libc::c_void,
}
pub const TTYC_DCH1: tty_code_code = 24;
pub const TTYC_KF15: tty_code_code = 74;
pub const TTYC_KF48: tty_code_code = 110;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const TTYC_BEL: tty_code_code = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane {
    pub id: u_int,
    pub active_point: u_int,
    pub window: *mut window,
    pub layout_cell: *mut layout_cell,
    pub saved_layout_cell: *mut layout_cell,
    pub sx: u_int,
    pub sy: u_int,
    pub osx: u_int,
    pub osy: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub flags: libc::c_int,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub shell: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub pid: pid_t,
    pub tty: [libc::c_char; 32],
    pub status: libc::c_int,
    pub fd: libc::c_int,
    pub event: *mut bufferevent,
    pub resize_timer: event,
    pub ictx: *mut input_ctx,
    pub colgc: grid_cell,
    pub palette: *mut libc::c_int,
    pub pipe_fd: libc::c_int,
    pub pipe_event: *mut bufferevent,
    pub pipe_off: size_t,
    pub screen: *mut screen,
    pub base: screen,
    pub status_screen: screen,
    pub status_size: size_t,
    pub saved_cx: u_int,
    pub saved_cy: u_int,
    pub saved_grid: *mut grid,
    pub saved_cell: grid_cell,
    pub mode: *const window_mode,
    pub modedata: *mut libc::c_void,
    pub modetimer: event,
    pub modelast: time_t,
    pub modeprefix: u_int,
    pub searchstr: *mut libc::c_char,
    pub entry: unnamed_18,
    pub tree_entry: unnamed_3,
}
pub const TTYC_KF20: tty_code_code = 80;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_25,
}
pub const TTYC_KLFT6: tty_code_code = 150;
pub const TTYC_RMKX: tty_code_code = 187;
pub const TTYC_BOLD: tty_code_code = 5;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const TTYC_CUB: tty_code_code = 13;
pub const TTYC_KRIT4: tty_code_code = 170;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const TTYC_KF45: tty_code_code = 107;
pub type __off_t = libc::c_long;
pub const TTYC_KPP: tty_code_code = 160;
pub const TTYC_KF43: tty_code_code = 105;
pub const TTYC_KF53: tty_code_code = 116;
pub const TTYC_KF5: tty_code_code = 112;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_36,
    pub flags: libc::c_int,
    pub cmd: *mut libc::c_char,
    pub pid: pid_t,
    pub status: libc::c_int,
    pub fd: libc::c_int,
    pub event: *mut bufferevent,
    pub updatecb: job_update_cb,
    pub completecb: job_complete_cb,
    pub freecb: job_free_cb,
    pub data: *mut libc::c_void,
    pub entry: unnamed_21,
}
pub const TTY_VT320: unnamed_31 = 4;
pub const TTYC_KPRV6: tty_code_code = 165;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub const TTYC_KF55: tty_code_code = 118;
pub const TTYC_KDC6: tty_code_code = 52;
pub const TTYC_KRIT2: tty_code_code = 168;
pub const TTYC_KF10: tty_code_code = 69;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct mouse_event {
    pub valid: libc::c_int,
    pub key: key_code,
    pub statusat: libc::c_int,
    pub x: u_int,
    pub y: u_int,
    pub b: u_int,
    pub lx: u_int,
    pub ly: u_int,
    pub lb: u_int,
    pub s: libc::c_int,
    pub w: libc::c_int,
    pub wp: libc::c_int,
    pub sgr_type: u_int,
    pub sgr_b: u_int,
}
pub const TTYC_KF22: tty_code_code = 82;
pub const TTYC_KRIT3: tty_code_code = 169;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const TTY_VT102: unnamed_31 = 2;
pub const TTYC_KF57: tty_code_code = 120;
pub const TTYC_DIM: tty_code_code = 25;
pub const TTYC_KF25: tty_code_code = 85;
pub const TTYC_KIC5: tty_code_code = 141;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _win_st {
    pub _cury: libc::c_short,
    pub _curx: libc::c_short,
    pub _maxy: libc::c_short,
    pub _maxx: libc::c_short,
    pub _begy: libc::c_short,
    pub _begx: libc::c_short,
    pub _flags: libc::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: libc::c_int,
    pub _line: *mut ldat,
    pub _regtop: libc::c_short,
    pub _regbottom: libc::c_short,
    pub _parx: libc::c_int,
    pub _pary: libc::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: libc::c_short,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const TTYC_KUP2: tty_code_code = 174;
pub const TTY_VT220: unnamed_31 = 3;
pub const TTYC_KUP3: tty_code_code = 175;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_14,
}
pub const TTYC_KPRV3: tty_code_code = 162;
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const TTY_VT101: unnamed_31 = 1;
pub const TTYC_IL1: tty_code_code = 40;
pub const TTYC_KF56: tty_code_code = 119;
pub const TTYC_KICH1: tty_code_code = 144;
pub const JOB_CLOSED: unnamed_36 = 2;
pub const TTYC_DL1: tty_code_code = 27;
pub const TTYC_KNXT5: tty_code_code = 157;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const TTYC_KLFT5: tty_code_code = 149;
pub const TTYC_KCUF1: tty_code_code = 46;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const TTY_UNKNOWN: unnamed_31 = 6;
pub const TTYC_HOME: tty_code_code = 35;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const TTYC_KF6: tty_code_code = 123;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_0,
    pub entry: unnamed_20,
}
pub const TTYC_KF19: tty_code_code = 78;
pub const TTYC_KHOME: tty_code_code = 137;
pub const TTYC_KF54: tty_code_code = 117;
pub type bitstr_t = libc::c_uchar;
pub const CMDQ_CALLBACK: cmdq_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_32,
}
pub const TTYC_SMCUP: tty_code_code = 196;
pub const TTYC_KF1: tty_code_code = 68;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub const TTYC_KIC4: tty_code_code = 140;
pub const TTYC_KF50: tty_code_code = 113;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const TTYC_RGB: tty_code_code = 183;
pub const TTYC_KF62: tty_code_code = 126;
pub const TTYC_XT: tty_code_code = 207;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_6,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const TTYC_KF21: tty_code_code = 81;
pub const TTYC_CUF1: tty_code_code = 18;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub ev_signal_next: unnamed_30,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const TTYC_KLFT2: tty_code_code = 146;
pub const TTYC_KDN7: tty_code_code = 60;
pub const TTYC_REV: tty_code_code = 182;
pub type __useconds_t = libc::c_uint;
pub const TTYC_KEND4: tty_code_code = 64;
pub const TTYC_CUD1: tty_code_code = 16;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _ns_flagdata {
    pub mask: libc::c_int,
    pub shift: libc::c_int,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const TTYC_KF44: tty_code_code = 106;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const TTYC_SETRGBB: tty_code_code = 191;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_item {
    pub name: *const libc::c_char,
    pub queue: *mut cmdq_list,
    pub next: *mut cmdq_item,
    pub client: *mut client,
    pub type_0: cmdq_type,
    pub group: u_int,
    pub number: u_int,
    pub time: time_t,
    pub flags: libc::c_int,
    pub shared: *mut cmdq_shared,
    pub source: cmd_find_state,
    pub target: cmd_find_state,
    pub cmdlist: *mut cmd_list,
    pub cmd: *mut cmd,
    pub cb: cmdq_cb,
    pub data: *mut libc::c_void,
    pub entry: unnamed_37,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_mode {
    pub name: *const libc::c_char,
    pub init: Option<unsafe extern "C" fn(_: *mut window_pane,
                                          _: *mut cmd_find_state,
                                          _: *mut args) -> *mut screen>,
    pub free: Option<unsafe extern "C" fn(_: *mut window_pane) -> ()>,
    pub resize: Option<unsafe extern "C" fn(_: *mut window_pane, _: u_int,
                                            _: u_int) -> ()>,
    pub key: Option<unsafe extern "C" fn(_: *mut window_pane, _: *mut client,
                                         _: *mut session, _: key_code,
                                         _: *mut mouse_event) -> ()>,
    pub key_table: Option<unsafe extern "C" fn(_: *mut window_pane)
                              -> *const libc::c_char>,
    pub command: Option<unsafe extern "C" fn(_: *mut window_pane,
                                             _: *mut client, _: *mut session,
                                             _: *mut args,
                                             _: *mut mouse_event) -> ()>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const TTYC_KPRV4: tty_code_code = 163;
pub const TTYC_KLFT4: tty_code_code = 148;
pub const TTYC_CUD: tty_code_code = 15;
pub const TTYC_KHOM4: tty_code_code = 133;
pub const TTYC_KF27: tty_code_code = 87;
pub const TTYC_SETAB: tty_code_code = 189;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_5,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type __u_char = libc::c_uchar;
pub const TTYC_KDC4: tty_code_code = 50;
pub const TTYC_ACSC: tty_code_code = 1;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty {
    pub client: *mut client,
    pub sx: u_int,
    pub sy: u_int,
    pub cx: u_int,
    pub cy: u_int,
    pub cstyle: u_int,
    pub ccolour: *mut libc::c_char,
    pub mode: libc::c_int,
    pub rlower: u_int,
    pub rupper: u_int,
    pub rleft: u_int,
    pub rright: u_int,
    pub fd: libc::c_int,
    pub event_in: event,
    pub in_0: *mut evbuffer,
    pub event_out: event,
    pub out: *mut evbuffer,
    pub timer: event,
    pub discarded: size_t,
    pub tio: termios,
    pub cell: grid_cell,
    pub last_wp: libc::c_int,
    pub last_cell: grid_cell,
    pub flags: libc::c_int,
    pub term: *mut tty_term,
    pub term_name: *mut libc::c_char,
    pub term_flags: libc::c_int,
    pub term_type: unnamed_31,
    pub mouse: mouse_event,
    pub mouse_drag_flag: libc::c_int,
    pub mouse_drag_update: Option<unsafe extern "C" fn(_: *mut client,
                                                       _: *mut mouse_event)
                                      -> ()>,
    pub mouse_drag_release: Option<unsafe extern "C" fn(_: *mut client,
                                                        _: *mut mouse_event)
                                       -> ()>,
    pub key_timer: event,
    pub key_tree: *mut tty_key,
}
pub const LINE_SEL_NONE: unnamed_5 = 0;
pub const TTYC_KF3: tty_code_code = 90;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window {
    pub id: u_int,
    pub name: *mut libc::c_char,
    pub name_event: event,
    pub name_time: timeval,
    pub alerts_timer: event,
    pub activity_time: timeval,
    pub active: *mut window_pane,
    pub last: *mut window_pane,
    pub panes: window_panes,
    pub lastlayout: libc::c_int,
    pub layout_root: *mut layout_cell,
    pub saved_layout_root: *mut layout_cell,
    pub old_layout: *mut libc::c_char,
    pub sx: u_int,
    pub sy: u_int,
    pub flags: libc::c_int,
    pub alerts_queued: libc::c_int,
    pub alerts_entry: unnamed_9,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_22,
    pub entry: unnamed_28,
}
pub const TTYC_DCH: tty_code_code = 23;
pub type u_int = __u_int;
pub const TTYC_SGR0: tty_code_code = 193;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_ctx {
    pub wp: *mut window_pane,
    pub cell: *const grid_cell,
    pub wrapped: libc::c_int,
    pub num: u_int,
    pub ptr: *mut libc::c_void,
    pub ocx: u_int,
    pub ocy: u_int,
    pub orupper: u_int,
    pub orlower: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub bg: u_int,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const TTYC_SMXX: tty_code_code = 200;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_CIVIS: tty_code_code = 6;
pub const TTYC_KF29: tty_code_code = 89;
pub const TTYC_KF42: tty_code_code = 104;
pub const TTYC_KF9: tty_code_code = 130;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const TTYC_RMACS: tty_code_code = 185;
pub const TTYC_ICH1: tty_code_code = 38;
pub const TTYC_KRI: tty_code_code = 167;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_16,
}
pub const TTYC_KDC7: tty_code_code = 53;
pub const TTYC_KF33: tty_code_code = 94;
pub const TTYC_KF38: tty_code_code = 99;
pub const TTYC_CSR: tty_code_code = 12;
pub type WINDOW = _win_st;
pub const TTYC_EL: tty_code_code = 31;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const TTYC_INDN: tty_code_code = 41;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub type tty_code_code = libc::c_uint;
pub const TTYC_KPRV7: tty_code_code = 166;
pub const TTYC_KHOM3: tty_code_code = 132;
pub const TTYC_KIC3: tty_code_code = 139;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct pdat {
    pub _pad_y: libc::c_short,
    pub _pad_x: libc::c_short,
    pub _pad_top: libc::c_short,
    pub _pad_left: libc::c_short,
    pub _pad_bottom: libc::c_short,
    pub _pad_right: libc::c_short,
}
pub const TTYC_KDC3: tty_code_code = 49;
pub type pid_t = __pid_t;
pub const TTYC_CUU1: tty_code_code = 21;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub ev_io_next: unnamed_24,
    pub ev_timeout: timeval,
}
pub type u_short = __u_short;
pub const TTYC_KF14: tty_code_code = 73;
pub const TTYC_KF30: tty_code_code = 91;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const TTYC_KDCH1: tty_code_code = 54;
pub const TTYC_ICH: tty_code_code = 37;
pub const TTYC_KUP7: tty_code_code = 179;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type tcflag_t = libc::c_uint;
pub type cmd_retval = libc::c_int;
pub const TTYC_BCE: tty_code_code = 2;
pub type __u_int = libc::c_uint;
pub type unnamed_31 = libc::c_uint;
pub const TTYC_KF40: tty_code_code = 102;
pub const PROMPT_COMMAND: unnamed_4 = 1;
pub const TTYC_KF32: tty_code_code = 93;
pub type time_t = __time_t;
pub type __suseconds_t = libc::c_long;
pub const TTYC_SMUL: tty_code_code = 199;
pub const TTYC_EL1: tty_code_code = 32;
pub const TTYC_CUF: tty_code_code = 17;
pub type __ssize_t = libc::c_long;
pub const TTYC_CVVIS: tty_code_code = 22;
pub type u_char = __u_char;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cell {
    pub type_0: layout_type,
    pub parent: *mut layout_cell,
    pub sx: u_int,
    pub sy: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub wp: *mut window_pane,
    pub cells: layout_cells,
    pub entry: unnamed_34,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_KLFT7: tty_code_code = 151;
pub const TTYC_KUP5: tty_code_code = 177;
pub const TTYC_KNXT4: tty_code_code = 156;
pub const TTYC_KIND: tty_code_code = 145;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const TTYC_KRIT7: tty_code_code = 173;
pub const TTYC_ENACS: tty_code_code = 33;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub const TTYC_KEND: tty_code_code = 61;
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub const TTYC_KF7: tty_code_code = 128;
pub const TTYC_KF26: tty_code_code = 86;
pub const TTYC_AX: tty_code_code = 0;
pub const TTYC_KIC6: tty_code_code = 142;
pub type unnamed_36 = libc::c_uint;
pub const TTYC_KNXT3: tty_code_code = 155;
pub const TTYC_SS: tty_code_code = 201;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct options_table_entry {
    pub name: *const libc::c_char,
    pub type_0: options_table_type,
    pub scope: options_table_scope,
    pub minimum: u_int,
    pub maximum: u_int,
    pub choices: *mut *const libc::c_char,
    pub default_str: *const libc::c_char,
    pub default_num: libc::c_longlong,
    pub separator: *const libc::c_char,
    pub style: *const libc::c_char,
}
pub const TTYC_KF36: tty_code_code = 97;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_KHOM5: tty_code_code = 134;
pub const TTYC_RMCUP: tty_code_code = 186;
#[no_mangle]
pub unsafe extern "C" fn tty_create_log() -> () {
    let mut name: [libc::c_char; 64] = [0; 64];
    xsnprintf(name.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
              b"tmux-out-%ld.log\x00" as *const u8 as *const libc::c_char,
              getpid() as libc::c_long);
    tty_log_fd = open(name.as_mut_ptr(), 1i32 | 64i32 | 512i32, 420i32);
    if tty_log_fd != 1i32.wrapping_neg() &&
           fcntl(tty_log_fd, 2i32, 1i32) == 1i32.wrapping_neg() {
        fatal(b"fcntl failed\x00" as *const u8 as *const libc::c_char);
    } else { return; };
}
static mut tty_log_fd: libc::c_int = unsafe { -1i32 };
#[no_mangle]
pub unsafe extern "C" fn tty_raw(mut tty: *mut tty,
                                 mut s: *const libc::c_char) -> () {
    let mut n: ssize_t = 0;
    let mut slen: ssize_t = 0;
    let mut i: u_int = 0;
    slen = strlen(s) as ssize_t;
    i = 0i32 as u_int;
    while i < 5i32 as libc::c_uint {
        n = write((*tty).fd, s as *const libc::c_void, slen as size_t);
        if n >= 0i32 as libc::c_long {
            s = s.offset(n as isize);
            slen -= n;
            if slen == 0i32 as libc::c_long { break ; }
        } else if n == 1i32.wrapping_neg() as libc::c_long &&
                      *__errno_location() != 11i32 {
            break ;
        }
        usleep(100i32 as __useconds_t);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_attributes(mut tty: *mut tty,
                                        mut gc: *const grid_cell,
                                        mut wp: *const window_pane) -> () {
    let mut tc: *mut grid_cell = &mut (*tty).cell as *mut grid_cell;
    let mut gc2: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut changed: libc::c_int = 0;
    if wp != 0 as *mut libc::c_void as *const window_pane &&
           (*wp).id as libc::c_int == (*tty).last_wp &&
           0 != !((*(*wp).window).flags & 32768i32) &&
           (*gc).attr as libc::c_int == (*tty).last_cell.attr as libc::c_int
           && (*gc).fg == (*tty).last_cell.fg &&
           (*gc).bg == (*tty).last_cell.bg {
        return
    } else {
        (*tty).last_wp =
            if wp != 0 as *mut libc::c_void as *const window_pane {
                (*wp).id as libc::c_int
            } else { 1i32.wrapping_neg() };
        memcpy(&mut (*tty).last_cell as *mut grid_cell as *mut libc::c_void,
               gc as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        memcpy(&mut gc2 as *mut grid_cell as *mut libc::c_void,
               gc as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        if wp != 0 as *mut libc::c_void as *const window_pane {
            tty_default_colours(&mut gc2 as *mut grid_cell, wp);
        }
        if 0 == tty_term_has((*tty).term, TTYC_SETAB) {
            if 0 != gc2.attr as libc::c_int & 16i32 {
                if gc2.fg != 7i32 && gc2.fg != 8i32 {
                    gc2.attr = (gc2.attr as libc::c_int & !16i32) as u_short
                }
            } else if gc2.bg != 0i32 && gc2.bg != 8i32 {
                gc2.attr = (gc2.attr as libc::c_int | 16i32) as u_short
            }
        }
        tty_check_fg(tty, wp, &mut gc2 as *mut grid_cell);
        tty_check_bg(tty, wp, &mut gc2 as *mut grid_cell);
        if 0 != (*tc).attr as libc::c_int & !(gc2.attr as libc::c_int) {
            tty_reset(tty);
        }
        tty_colours(tty, &mut gc2 as *mut grid_cell);
        changed = gc2.attr as libc::c_int & !((*tc).attr as libc::c_int);
        (*tc).attr = gc2.attr;
        if 0 != changed & 1i32 { tty_putcode(tty, TTYC_BOLD); }
        if 0 != changed & 2i32 { tty_putcode(tty, TTYC_DIM); }
        if 0 != changed & 64i32 { tty_set_italics(tty); }
        if 0 != changed & 4i32 { tty_putcode(tty, TTYC_SMUL); }
        if 0 != changed & 8i32 { tty_putcode(tty, TTYC_BLINK); }
        if 0 != changed & 16i32 {
            if 0 != tty_term_has((*tty).term, TTYC_REV) {
                tty_putcode(tty, TTYC_REV);
            } else if 0 != tty_term_has((*tty).term, TTYC_SMSO) {
                tty_putcode(tty, TTYC_SMSO);
            }
        }
        if 0 != changed & 32i32 { tty_putcode(tty, TTYC_INVIS); }
        if 0 != changed & 256i32 { tty_putcode(tty, TTYC_SMXX); }
        if 0 != changed & 128i32 && 0 != tty_acs_needed(tty) {
            tty_putcode(tty, TTYC_SMACS);
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode(mut tty: *mut tty,
                                     mut code: tty_code_code) -> () {
    tty_puts(tty, tty_term_string((*tty).term, code));
}
#[no_mangle]
pub unsafe extern "C" fn tty_puts(mut tty: *mut tty,
                                  mut s: *const libc::c_char) -> () {
    if *s as libc::c_int != 0 { tty_add(tty, s, strlen(s)); };
}
unsafe extern "C" fn tty_add(mut tty: *mut tty, mut buf: *const libc::c_char,
                             mut len: size_t) -> () {
    let mut c: *mut client = (*tty).client;
    if 0 != (*tty).flags & 128i32 {
        (*tty).discarded =
            ((*tty).discarded as libc::c_ulong).wrapping_add(len) as size_t as
                size_t;
        return
    } else {
        evbuffer_add((*tty).out, buf as *const libc::c_void, len);
        log_debug(b"%s: %.*s\x00" as *const u8 as *const libc::c_char,
                  (*c).name, len as libc::c_int, buf);
        (*c).written =
            ((*c).written as libc::c_ulong).wrapping_add(len) as size_t as
                size_t;
        if tty_log_fd != 1i32.wrapping_neg() {
            write(tty_log_fd, buf as *const libc::c_void, len);
        }
        if 0 != (*tty).flags & 16i32 {
            event_add(&mut (*tty).event_out as *mut event,
                      0 as *const timeval);
        }
        return;
    };
}
unsafe extern "C" fn tty_set_italics(mut tty: *mut tty) -> () {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if 0 != tty_term_has((*tty).term, TTYC_SITM) {
        s =
            options_get_string(global_options,
                               b"default-terminal\x00" as *const u8 as
                                   *const libc::c_char);
        if strcmp(s, b"screen\x00" as *const u8 as *const libc::c_char) !=
               0i32 &&
               strncmp(s, b"screen-\x00" as *const u8 as *const libc::c_char,
                       7i32 as libc::c_ulong) != 0i32 {
            tty_putcode(tty, TTYC_SITM);
            return
        }
    }
    tty_putcode(tty, TTYC_SMSO);
}
unsafe extern "C" fn tty_colours(mut tty: *mut tty, mut gc: *const grid_cell)
 -> () {
    let mut tc: *mut grid_cell = &mut (*tty).cell as *mut grid_cell;
    let mut have_ax: libc::c_int = 0;
    if (*gc).fg == (*tc).fg && (*gc).bg == (*tc).bg {
        return
    } else {
        if (*gc).fg == 8i32 || (*gc).bg == 8i32 {
            have_ax = tty_term_flag((*tty).term, TTYC_AX);
            if 0 == have_ax && 0 != tty_term_has((*tty).term, TTYC_OP) {
                tty_reset(tty);
            } else {
                if (*gc).fg == 8i32 && (*tc).fg != 8i32 {
                    if 0 != have_ax {
                        tty_puts(tty,
                                 b"\x1b[39m\x00" as *const u8 as
                                     *const libc::c_char);
                    } else if (*tc).fg != 7i32 {
                        tty_putcode1(tty, TTYC_SETAF, 7i32);
                    }
                    (*tc).fg = 8i32
                }
                if (*gc).bg == 8i32 && (*tc).bg != 8i32 {
                    if 0 != have_ax {
                        tty_puts(tty,
                                 b"\x1b[49m\x00" as *const u8 as
                                     *const libc::c_char);
                    } else if (*tc).bg != 0i32 {
                        tty_putcode1(tty, TTYC_SETAB, 0i32);
                    }
                    (*tc).bg = 8i32
                }
            }
        }
        if (*gc).fg != 8i32 && (*gc).fg != (*tc).fg {
            tty_colours_fg(tty, gc);
        }
        if (*gc).bg != 8i32 && (*gc).bg != (*tc).bg {
            tty_colours_bg(tty, gc);
        }
        return;
    };
}
unsafe extern "C" fn tty_colours_bg(mut tty: *mut tty,
                                    mut gc: *const grid_cell) -> () {
    let mut tc: *mut grid_cell = &mut (*tty).cell as *mut grid_cell;
    let mut s: [libc::c_char; 32] = [0; 32];
    if 0 != (*gc).bg & 33554432i32 || 0 != (*gc).bg & 16777216i32 {
        if !(tty_try_colour(tty, (*gc).bg,
                            b"48\x00" as *const u8 as *const libc::c_char) ==
                 0i32) {
            return
        }
    } else if (*gc).bg >= 90i32 && (*gc).bg <= 97i32 {
        xsnprintf(s.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong,
                  b"\x1b[%dm\x00" as *const u8 as *const libc::c_char,
                  (*gc).bg + 10i32);
        tty_puts(tty, s.as_mut_ptr());
    } else { tty_putcode1(tty, TTYC_SETAB, (*gc).bg); }
    (*tc).bg = (*gc).bg;
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode1(mut tty: *mut tty,
                                      mut code: tty_code_code,
                                      mut a: libc::c_int) -> () {
    if a < 0i32 {
        return
    } else { tty_puts(tty, tty_term_string1((*tty).term, code, a)); return; };
}
unsafe extern "C" fn tty_try_colour(mut tty: *mut tty,
                                    mut colour: libc::c_int,
                                    mut type_0: *const libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    let mut s: [libc::c_char; 32] = [0; 32];
    if 0 != colour & 16777216i32 {
        if !(0 != (*tty).term_flags & 1i32 ||
                 0 != tty_term_has((*tty).term, TTYC_RGB)) {
            if 0 != (*(*tty).term).flags & 1i32 {
                if *type_0 as libc::c_int == 51 {
                    if 0 == tty_term_has((*tty).term, TTYC_SETAF) {
                        current_block = 16963945826854080480;
                    } else {
                        tty_putcode1(tty, TTYC_SETAF, colour & 255i32);
                        current_block = 12517898123489920830;
                    }
                } else if 0 == tty_term_has((*tty).term, TTYC_SETAB) {
                    current_block = 16963945826854080480;
                } else {
                    tty_putcode1(tty, TTYC_SETAB, colour & 255i32);
                    current_block = 12517898123489920830;
                }
                match current_block {
                    16963945826854080480 => { }
                    _ => { return 0i32 }
                }
            }
        }
        xsnprintf(s.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong,
                  b"\x1b[%s;5;%dm\x00" as *const u8 as *const libc::c_char,
                  type_0, colour & 255i32);
        log_debug(b"%s: 256 colour fallback: %s\x00" as *const u8 as
                      *const libc::c_char, (*(*tty).client).name,
                  s.as_mut_ptr());
        tty_puts(tty, s.as_mut_ptr());
        return 0i32
    } else if 0 != colour & 33554432i32 {
        if *type_0 as libc::c_int == 51 {
            if 0 == tty_term_has((*tty).term, TTYC_SETRGBF) {
                return 1i32.wrapping_neg()
            } else {
                colour_split_rgb(colour & 16777215i32, &mut r as *mut u_char,
                                 &mut g as *mut u_char,
                                 &mut b as *mut u_char);
                tty_putcode3(tty, TTYC_SETRGBF, r as libc::c_int,
                             g as libc::c_int, b as libc::c_int);
            }
        } else if 0 == tty_term_has((*tty).term, TTYC_SETRGBB) {
            return 1i32.wrapping_neg()
        } else {
            colour_split_rgb(colour & 16777215i32, &mut r as *mut u_char,
                             &mut g as *mut u_char, &mut b as *mut u_char);
            tty_putcode3(tty, TTYC_SETRGBB, r as libc::c_int,
                         g as libc::c_int, b as libc::c_int);
        }
        return 0i32
    } else { return 1i32.wrapping_neg() };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode3(mut tty: *mut tty,
                                      mut code: tty_code_code,
                                      mut a: libc::c_int, mut b: libc::c_int,
                                      mut c: libc::c_int) -> () {
    if a < 0i32 || b < 0i32 || c < 0i32 {
        return
    } else {
        tty_puts(tty, tty_term_string3((*tty).term, code, a, b, c));
        return;
    };
}
unsafe extern "C" fn tty_colours_fg(mut tty: *mut tty,
                                    mut gc: *const grid_cell) -> () {
    let mut tc: *mut grid_cell = &mut (*tty).cell as *mut grid_cell;
    let mut s: [libc::c_char; 32] = [0; 32];
    if 0 != (*gc).fg & 33554432i32 || 0 != (*gc).fg & 16777216i32 {
        if !(tty_try_colour(tty, (*gc).fg,
                            b"38\x00" as *const u8 as *const libc::c_char) ==
                 0i32) {
            return
        }
    } else if (*gc).fg >= 90i32 && (*gc).fg <= 97i32 {
        xsnprintf(s.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong,
                  b"\x1b[%dm\x00" as *const u8 as *const libc::c_char,
                  (*gc).fg);
        tty_puts(tty, s.as_mut_ptr());
    } else { tty_putcode1(tty, TTYC_SETAF, (*gc).fg); }
    (*tc).fg = (*gc).fg;
}
#[no_mangle]
pub unsafe extern "C" fn tty_reset(mut tty: *mut tty) -> () {
    let mut gc: *mut grid_cell = &mut (*tty).cell as *mut grid_cell;
    if 0 == grid_cells_equal(gc, &grid_default_cell as *const grid_cell) {
        if 0 != (*gc).attr as libc::c_int & 128i32 && 0 != tty_acs_needed(tty)
           {
            tty_putcode(tty, TTYC_RMACS);
        }
        tty_putcode(tty, TTYC_SGR0);
        memcpy(gc as *mut libc::c_void,
               &grid_default_cell as *const grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    }
    memcpy(&mut (*tty).last_cell as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    (*tty).last_wp = 1i32.wrapping_neg();
}
unsafe extern "C" fn tty_check_bg(mut tty: *mut tty,
                                  mut wp: *const window_pane,
                                  mut gc: *mut grid_cell) -> () {
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    let mut colours: u_int = 0;
    let mut c: libc::c_int = 0;
    if 0 != !((*gc).flags as libc::c_int) & 32i32 {
        c = window_pane_get_palette(wp, (*gc).bg);
        if c != 1i32.wrapping_neg() { (*gc).bg = c }
    }
    if 0 != (*gc).bg & 33554432i32 {
        if 0 == tty_term_has((*tty).term, TTYC_SETRGBB) {
            colour_split_rgb((*gc).bg, &mut r as *mut u_char,
                             &mut g as *mut u_char, &mut b as *mut u_char);
            (*gc).bg = colour_find_rgb(r, g, b)
        } else { return }
    }
    if 0 != ((*(*tty).term).flags | (*tty).term_flags) & 1i32 {
        colours = 256i32 as u_int
    } else { colours = tty_term_number((*tty).term, TTYC_COLORS) as u_int }
    if 0 != (*gc).bg & 16777216i32 {
        if colours != 256i32 as libc::c_uint {
            (*gc).bg = colour_256to16((*gc).bg as u_char) as libc::c_int;
            if 0 != (*gc).bg & 8i32 {
                (*gc).bg &= 7i32;
                if colours >= 16i32 as libc::c_uint { (*gc).fg += 90i32 }
            }
        }
        return
    } else {
        if (*gc).bg >= 90i32 && (*gc).bg <= 97i32 &&
               colours < 16i32 as libc::c_uint {
            (*gc).bg -= 90i32
        }
        return;
    };
}
unsafe extern "C" fn tty_check_fg(mut tty: *mut tty,
                                  mut wp: *const window_pane,
                                  mut gc: *mut grid_cell) -> () {
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    let mut colours: u_int = 0;
    let mut c: libc::c_int = 0;
    if 0 != !((*gc).flags as libc::c_int) & 32i32 {
        c = (*gc).fg;
        if c < 8i32 && 0 != (*gc).attr as libc::c_int & 1i32 { c += 90i32 }
        c = window_pane_get_palette(wp, c);
        if c != 1i32.wrapping_neg() { (*gc).fg = c }
    }
    if 0 != (*gc).fg & 33554432i32 {
        if 0 == tty_term_has((*tty).term, TTYC_SETRGBF) {
            colour_split_rgb((*gc).fg, &mut r as *mut u_char,
                             &mut g as *mut u_char, &mut b as *mut u_char);
            (*gc).fg = colour_find_rgb(r, g, b)
        } else { return }
    }
    if 0 != ((*(*tty).term).flags | (*tty).term_flags) & 1i32 {
        colours = 256i32 as u_int
    } else { colours = tty_term_number((*tty).term, TTYC_COLORS) as u_int }
    if 0 != (*gc).fg & 16777216i32 {
        if colours != 256i32 as libc::c_uint {
            (*gc).fg = colour_256to16((*gc).fg as u_char) as libc::c_int;
            if 0 != (*gc).fg & 8i32 {
                (*gc).fg &= 7i32;
                if colours >= 16i32 as libc::c_uint {
                    (*gc).fg += 90i32
                } else {
                    (*gc).attr = ((*gc).attr as libc::c_int | 1i32) as u_short
                }
            } else {
                (*gc).attr = ((*gc).attr as libc::c_int & !1i32) as u_short
            }
        }
        return
    } else {
        if (*gc).fg >= 90i32 && (*gc).fg <= 97i32 &&
               colours < 16i32 as libc::c_uint {
            (*gc).fg -= 90i32;
            (*gc).attr = ((*gc).attr as libc::c_int | 1i32) as u_short
        }
        return;
    };
}
unsafe extern "C" fn tty_default_colours(mut gc: *mut grid_cell,
                                         mut wp: *const window_pane) -> () {
    let mut w: *mut window = (*wp).window;
    let mut oo: *mut options = (*w).options;
    let mut agc: *const grid_cell = 0 as *const grid_cell;
    let mut pgc: *const grid_cell = 0 as *const grid_cell;
    let mut wgc: *const grid_cell = 0 as *const grid_cell;
    let mut c: libc::c_int = 0;
    if 0 != (*w).flags & 32768i32 {
        (*w).flags &= !32768i32;
        agc =
            options_get_style(oo,
                              b"window-active-style\x00" as *const u8 as
                                  *const libc::c_char);
        memcpy(&mut (*w).active_style as *mut grid_cell as *mut libc::c_void,
               agc as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        wgc =
            options_get_style(oo,
                              b"window-style\x00" as *const u8 as
                                  *const libc::c_char);
        memcpy(&mut (*w).style as *mut grid_cell as *mut libc::c_void,
               wgc as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    } else {
        agc = &mut (*w).active_style as *mut grid_cell;
        wgc = &mut (*w).style as *mut grid_cell
    }
    pgc = &(*wp).colgc as *const grid_cell;
    if (*gc).fg == 8i32 {
        if (*pgc).fg != 8i32 {
            (*gc).fg = (*pgc).fg
        } else if wp == (*w).active && (*agc).fg != 8i32 {
            (*gc).fg = (*agc).fg
        } else { (*gc).fg = (*wgc).fg }
        if (*gc).fg != 8i32 &&
               {
                   c = window_pane_get_palette(wp, (*gc).fg);
                   c != 1i32.wrapping_neg()
               } {
            (*gc).fg = c
        }
    }
    if (*gc).bg == 8i32 {
        if (*pgc).bg != 8i32 {
            (*gc).bg = (*pgc).bg
        } else if wp == (*w).active && (*agc).bg != 8i32 {
            (*gc).bg = (*agc).bg
        } else { (*gc).bg = (*wgc).bg }
        if (*gc).bg != 8i32 &&
               {
                   c = window_pane_get_palette(wp, (*gc).bg);
                   c != 1i32.wrapping_neg()
               } {
            (*gc).bg = c
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_region_off(mut tty: *mut tty) -> () {
    tty_region(tty, 0i32 as u_int,
               (*tty).sy.wrapping_sub(1i32 as libc::c_uint));
}
unsafe extern "C" fn tty_region(mut tty: *mut tty, mut rupper: u_int,
                                mut rlower: u_int) -> () {
    if (*tty).rlower == rlower && (*tty).rupper == rupper {
        return
    } else if 0 == tty_term_has((*tty).term, TTYC_CSR) {
        return
    } else {
        (*tty).rupper = rupper;
        (*tty).rlower = rlower;
        if (*tty).cx >= (*tty).sx {
            tty_cursor(tty, 0i32 as u_int, (*tty).cy);
        }
        tty_putcode2(tty, TTYC_CSR, (*tty).rupper as libc::c_int,
                     (*tty).rlower as libc::c_int);
        (*tty).cy =
            (2147483647i32 as
                 libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32);
        (*tty).cx = (*tty).cy;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode2(mut tty: *mut tty,
                                      mut code: tty_code_code,
                                      mut a: libc::c_int, mut b: libc::c_int)
 -> () {
    if a < 0i32 || b < 0i32 {
        return
    } else {
        tty_puts(tty, tty_term_string2((*tty).term, code, a, b));
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cursor(mut tty: *mut tty, mut cx: u_int,
                                    mut cy: u_int) -> () {
    let mut current_block: u64;
    let mut term: *mut tty_term = (*tty).term;
    let mut thisx: u_int = 0;
    let mut thisy: u_int = 0;
    let mut change: libc::c_int = 0;
    if cx > (*tty).sx.wrapping_sub(1i32 as libc::c_uint) {
        cx = (*tty).sx.wrapping_sub(1i32 as libc::c_uint)
    }
    thisx = (*tty).cx;
    thisy = (*tty).cy;
    if cx == thisx && cy == thisy {
        return
    } else {
        if thisx > (*tty).sx.wrapping_sub(1i32 as libc::c_uint) {
            current_block = 6407599955816740281;
        } else if cx == 0i32 as libc::c_uint && cy == 0i32 as libc::c_uint &&
                      0 != tty_term_has(term, TTYC_HOME) {
            tty_putcode(tty, TTYC_HOME);
            current_block = 1184183797960978408;
        } else if cx == 0i32 as libc::c_uint &&
                      cy == thisy.wrapping_add(1i32 as libc::c_uint) &&
                      thisy != (*tty).rlower &&
                      (0 !=
                           !((*tty).term_type as libc::c_uint ==
                                 TTY_VT420 as libc::c_int as libc::c_uint) as
                               libc::c_int ||
                           (*tty).rleft == 0i32 as libc::c_uint) {
            tty_putc(tty, 13 as u_char);
            tty_putc(tty, 10 as u_char);
            current_block = 1184183797960978408;
        } else if cy == thisy {
            if cx == 0i32 as libc::c_uint &&
                   (0 !=
                        !((*tty).term_type as libc::c_uint ==
                              TTY_VT420 as libc::c_int as libc::c_uint) as
                            libc::c_int ||
                        (*tty).rleft == 0i32 as libc::c_uint) {
                tty_putc(tty, 13 as u_char);
                current_block = 1184183797960978408;
            } else if cx == thisx.wrapping_sub(1i32 as libc::c_uint) &&
                          0 != tty_term_has(term, TTYC_CUB1) {
                tty_putcode(tty, TTYC_CUB1);
                current_block = 1184183797960978408;
            } else if cx == thisx.wrapping_add(1i32 as libc::c_uint) &&
                          0 != tty_term_has(term, TTYC_CUF1) {
                tty_putcode(tty, TTYC_CUF1);
                current_block = 1184183797960978408;
            } else {
                change = thisx.wrapping_sub(cx) as libc::c_int;
                if abs(change) as u_int > cx &&
                       0 != tty_term_has(term, TTYC_HPA) {
                    tty_putcode1(tty, TTYC_HPA, cx as libc::c_int);
                    current_block = 1184183797960978408;
                } else if change > 0i32 && 0 != tty_term_has(term, TTYC_CUB) {
                    if change == 2i32 && 0 != tty_term_has(term, TTYC_CUB1) {
                        tty_putcode(tty, TTYC_CUB1);
                        tty_putcode(tty, TTYC_CUB1);
                    } else { tty_putcode1(tty, TTYC_CUB, change); }
                    current_block = 1184183797960978408;
                } else if change < 0i32 && 0 != tty_term_has(term, TTYC_CUF) {
                    tty_putcode1(tty, TTYC_CUF, change.wrapping_neg());
                    current_block = 1184183797960978408;
                } else { current_block = 6407599955816740281; }
            }
        } else if cx == thisx {
            if thisy != (*tty).rupper &&
                   cy == thisy.wrapping_sub(1i32 as libc::c_uint) &&
                   0 != tty_term_has(term, TTYC_CUU1) {
                tty_putcode(tty, TTYC_CUU1);
                current_block = 1184183797960978408;
            } else if thisy != (*tty).rlower &&
                          cy == thisy.wrapping_add(1i32 as libc::c_uint) &&
                          0 != tty_term_has(term, TTYC_CUD1) {
                tty_putcode(tty, TTYC_CUD1);
                current_block = 1184183797960978408;
            } else {
                change = thisy.wrapping_sub(cy) as libc::c_int;
                if abs(change) as u_int > cy ||
                       change < 0i32 &&
                           cy.wrapping_sub(change as libc::c_uint) >
                               (*tty).rlower ||
                       change > 0i32 &&
                           cy.wrapping_sub(change as libc::c_uint) <
                               (*tty).rupper {
                    if 0 != tty_term_has(term, TTYC_VPA) {
                        tty_putcode1(tty, TTYC_VPA, cy as libc::c_int);
                        current_block = 1184183797960978408;
                    } else { current_block = 6407599955816740281; }
                } else if change > 0i32 && 0 != tty_term_has(term, TTYC_CUU) {
                    tty_putcode1(tty, TTYC_CUU, change);
                    current_block = 1184183797960978408;
                } else if change < 0i32 && 0 != tty_term_has(term, TTYC_CUD) {
                    tty_putcode1(tty, TTYC_CUD, change.wrapping_neg());
                    current_block = 1184183797960978408;
                } else { current_block = 6407599955816740281; }
            }
        } else { current_block = 6407599955816740281; }
        match current_block {
            6407599955816740281 => {
                tty_putcode2(tty, TTYC_CUP, cy as libc::c_int,
                             cx as libc::c_int);
            }
            _ => { }
        }
        (*tty).cx = cx;
        (*tty).cy = cy;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putc(mut tty: *mut tty, mut ch: u_char) -> () {
    let mut acs: *const libc::c_char = 0 as *const libc::c_char;
    if 0 != (*tty).cell.attr as libc::c_int & 128i32 {
        acs = tty_acs_get(tty, ch);
        if acs != 0 as *mut libc::c_void as *const libc::c_char {
            tty_add(tty, acs, strlen(acs));
        } else {
            tty_add(tty, &mut ch as *mut u_char as *const libc::c_char,
                    1i32 as size_t);
        }
    } else {
        tty_add(tty, &mut ch as *mut u_char as *const libc::c_char,
                1i32 as size_t);
    }
    if ch as libc::c_int >= 32i32 && ch as libc::c_int != 127i32 {
        if (*tty).cx >= (*tty).sx {
            (*tty).cx = 1i32 as u_int;
            if (*tty).cy != (*tty).rlower {
                (*tty).cy = (*tty).cy.wrapping_add(1)
            }
            if 0 != (*(*tty).term).flags & 2i32 {
                tty_putcode2(tty, TTYC_CUP, (*tty).cy as libc::c_int,
                             (*tty).cx as libc::c_int);
            }
        } else { (*tty).cx = (*tty).cx.wrapping_add(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_margin_off(mut tty: *mut tty) -> () {
    tty_margin(tty, 0i32 as u_int,
               (*tty).sx.wrapping_sub(1i32 as libc::c_uint));
}
unsafe extern "C" fn tty_margin(mut tty: *mut tty, mut rleft: u_int,
                                mut rright: u_int) -> () {
    let mut s: [libc::c_char; 64] = [0; 64];
    if 0 !=
           !((*tty).term_type as libc::c_uint ==
                 TTY_VT420 as libc::c_int as libc::c_uint) as libc::c_int {
        return
    } else if (*tty).rleft == rleft && (*tty).rright == rright {
        return
    } else {
        tty_putcode2(tty, TTYC_CSR, (*tty).rupper as libc::c_int,
                     (*tty).rlower as libc::c_int);
        (*tty).rleft = rleft;
        (*tty).rright = rright;
        if rleft == 0i32 as libc::c_uint &&
               rright == (*tty).sx.wrapping_sub(1i32 as libc::c_uint) {
            snprintf(s.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 64]>() as
                         libc::c_ulong,
                     b"\x1b[s\x00" as *const u8 as *const libc::c_char);
        } else {
            snprintf(s.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 64]>() as
                         libc::c_ulong,
                     b"\x1b[%u;%us\x00" as *const u8 as *const libc::c_char,
                     rleft.wrapping_add(1i32 as libc::c_uint),
                     rright.wrapping_add(1i32 as libc::c_uint));
        }
        tty_puts(tty, s.as_mut_ptr());
        (*tty).cy =
            (2147483647i32 as
                 libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32);
        (*tty).cx = (*tty).cy;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode_ptr1(mut tty: *mut tty,
                                          mut code: tty_code_code,
                                          mut a: *const libc::c_void) -> () {
    if a != 0 as *mut libc::c_void {
        tty_puts(tty, tty_term_ptr1((*tty).term, code, a));
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode_ptr2(mut tty: *mut tty,
                                          mut code: tty_code_code,
                                          mut a: *const libc::c_void,
                                          mut b: *const libc::c_void) -> () {
    if a != 0 as *mut libc::c_void && b != 0 as *mut libc::c_void {
        tty_puts(tty, tty_term_ptr2((*tty).term, code, a, b));
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putn(mut tty: *mut tty,
                                  mut buf: *const libc::c_void,
                                  mut len: size_t, mut width: u_int) -> () {
    tty_add(tty, buf as *const libc::c_char, len);
    if (*tty).cx.wrapping_add(width) > (*tty).sx {
        (*tty).cx = (*tty).cx.wrapping_add(width).wrapping_sub((*tty).sx);
        if (*tty).cx <= (*tty).sx {
            (*tty).cy = (*tty).cy.wrapping_add(1)
        } else {
            (*tty).cy =
                (2147483647i32 as
                     libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32);
            (*tty).cx = (*tty).cy
        }
    } else {
        (*tty).cx =
            ((*tty).cx as libc::c_uint).wrapping_add(width) as u_int as u_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_init(mut tty: *mut tty, mut c: *mut client,
                                  mut fd: libc::c_int,
                                  mut term: *mut libc::c_char)
 -> libc::c_int {
    if 0 == isatty(fd) {
        return 1i32.wrapping_neg()
    } else {
        memset(tty as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<tty>() as libc::c_ulong);
        if term == 0 as *mut libc::c_void as *mut libc::c_char ||
               *term as libc::c_int == 0 {
            (*tty).term_name =
                xstrdup(b"unknown\x00" as *const u8 as *const libc::c_char)
        } else { (*tty).term_name = xstrdup(term) }
        (*tty).fd = fd;
        (*tty).client = c;
        (*tty).cstyle = 0i32 as u_int;
        (*tty).ccolour = xstrdup(b"\x00" as *const u8 as *const libc::c_char);
        (*tty).flags = 0i32;
        (*tty).term_flags = 0i32;
        (*tty).term_type = TTY_UNKNOWN;
        return 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_resize(mut tty: *mut tty) -> () {
    let mut c: *mut client = (*tty).client;
    let mut ws: winsize =
        winsize{ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0,};
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    if ioctl((*tty).fd, 21523i32 as libc::c_ulong, &mut ws as *mut winsize) !=
           1i32.wrapping_neg() {
        sx = ws.ws_col as u_int;
        if sx == 0i32 as libc::c_uint { sx = 80i32 as u_int }
        sy = ws.ws_row as u_int;
        if sy == 0i32 as libc::c_uint { sy = 24i32 as u_int }
    } else { sx = 80i32 as u_int; sy = 24i32 as u_int }
    log_debug(b"%s: %s now %ux%u\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"tty_resize\x00")).as_ptr(),
              (*c).name, sx, sy);
    tty_set_size(tty, sx, sy);
    tty_invalidate(tty);
}
unsafe extern "C" fn tty_invalidate(mut tty: *mut tty) -> () {
    memcpy(&mut (*tty).cell as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    memcpy(&mut (*tty).last_cell as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    (*tty).last_wp = 1i32.wrapping_neg();
    (*tty).cy =
        (2147483647i32 as libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32);
    (*tty).cx = (*tty).cy;
    (*tty).rleft =
        (2147483647i32 as libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32);
    (*tty).rupper = (*tty).rleft;
    (*tty).rright =
        (2147483647i32 as libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32);
    (*tty).rlower = (*tty).rright;
    if 0 != (*tty).flags & 16i32 {
        tty_putcode(tty, TTYC_SGR0);
        (*tty).mode = 16777215i32;
        tty_update_mode(tty, 1i32, 0 as *mut screen);
        tty_cursor(tty, 0i32 as u_int, 0i32 as u_int);
        tty_region_off(tty);
        tty_margin_off(tty);
    } else { (*tty).mode = 1i32 };
}
#[no_mangle]
pub unsafe extern "C" fn tty_update_mode(mut tty: *mut tty,
                                         mut mode: libc::c_int,
                                         mut s: *mut screen) -> () {
    let mut changed: libc::c_int = 0;
    if s != 0 as *mut libc::c_void as *mut screen &&
           strcmp((*s).ccolour, (*tty).ccolour) != 0i32 {
        tty_force_cursor_colour(tty, (*s).ccolour);
    }
    if 0 != (*tty).flags & 1i32 { mode &= !1i32 }
    changed = mode ^ (*tty).mode;
    if 0 != changed & 128i32 {
        if 0 != tty_term_has((*tty).term, TTYC_CVVIS) {
            tty_putcode(tty, TTYC_CVVIS);
        } else { tty_putcode(tty, TTYC_CNORM); }
        changed |= 1i32
    }
    if 0 != changed & 1i32 {
        if 0 != mode & 1i32 {
            tty_putcode(tty, TTYC_CNORM);
        } else { tty_putcode(tty, TTYC_CIVIS); }
    }
    if s != 0 as *mut libc::c_void as *mut screen &&
           (*tty).cstyle != (*s).cstyle {
        if 0 != tty_term_has((*tty).term, TTYC_SS) {
            if (*s).cstyle == 0i32 as libc::c_uint &&
                   0 != tty_term_has((*tty).term, TTYC_SE) {
                tty_putcode(tty, TTYC_SE);
            } else { tty_putcode1(tty, TTYC_SS, (*s).cstyle as libc::c_int); }
        }
        (*tty).cstyle = (*s).cstyle
    }
    if 0 != changed & (32i32 | 64i32 | 4096i32) {
        if 0 != mode & (32i32 | 64i32 | 4096i32) {
            tty_puts(tty,
                     b"\x1b[?1006h\x00" as *const u8 as *const libc::c_char);
            if 0 != mode & 4096i32 {
                tty_puts(tty,
                         b"\x1b[?1003h\x00" as *const u8 as
                             *const libc::c_char);
            } else if 0 != mode & 64i32 {
                tty_puts(tty,
                         b"\x1b[?1002h\x00" as *const u8 as
                             *const libc::c_char);
            } else if 0 != mode & 32i32 {
                tty_puts(tty,
                         b"\x1b[?1000h\x00" as *const u8 as
                             *const libc::c_char);
            }
        } else {
            if 0 != (*tty).mode & 4096i32 {
                tty_puts(tty,
                         b"\x1b[?1003l\x00" as *const u8 as
                             *const libc::c_char);
            } else if 0 != (*tty).mode & 64i32 {
                tty_puts(tty,
                         b"\x1b[?1002l\x00" as *const u8 as
                             *const libc::c_char);
            } else if 0 != (*tty).mode & 32i32 {
                tty_puts(tty,
                         b"\x1b[?1000l\x00" as *const u8 as
                             *const libc::c_char);
            }
            tty_puts(tty,
                     b"\x1b[?1006l\x00" as *const u8 as *const libc::c_char);
        }
    }
    if 0 != changed & 1024i32 {
        if 0 != mode & 1024i32 {
            tty_puts(tty,
                     b"\x1b[?2004h\x00" as *const u8 as *const libc::c_char);
        } else {
            tty_puts(tty,
                     b"\x1b[?2004l\x00" as *const u8 as *const libc::c_char);
        }
    }
    (*tty).mode = mode;
}
unsafe extern "C" fn tty_force_cursor_colour(mut tty: *mut tty,
                                             mut ccolour: *const libc::c_char)
 -> () {
    if *ccolour as libc::c_int == 0 {
        tty_putcode(tty, TTYC_CR);
    } else { tty_putcode_ptr1(tty, TTYC_CS, ccolour as *const libc::c_void); }
    free((*tty).ccolour as *mut libc::c_void);
    (*tty).ccolour = xstrdup(ccolour);
}
#[no_mangle]
pub unsafe extern "C" fn tty_set_size(mut tty: *mut tty, mut sx: u_int,
                                      mut sy: u_int) -> () {
    (*tty).sx = sx;
    (*tty).sy = sy;
}
#[no_mangle]
pub unsafe extern "C" fn tty_start_tty(mut tty: *mut tty) -> () {
    let mut c: *mut client = (*tty).client;
    let mut tio: termios =
        termios{c_iflag: 0,
                c_oflag: 0,
                c_cflag: 0,
                c_lflag: 0,
                c_line: 0,
                c_cc: [0; 32],
                c_ispeed: 0,
                c_ospeed: 0,};
    if (*tty).fd != 1i32.wrapping_neg() &&
           tcgetattr((*tty).fd, &mut (*tty).tio as *mut termios) == 0i32 {
        setblocking((*tty).fd, 0i32);
        event_add(&mut (*tty).event_in as *mut event, 0 as *const timeval);
        memcpy(&mut tio as *mut termios as *mut libc::c_void,
               &mut (*tty).tio as *mut termios as *const libc::c_void,
               ::std::mem::size_of::<termios>() as libc::c_ulong);
        tio.c_iflag &=
            !(1024i32 | 4096i32 | 256i32 | 64i32 | 128i32 | 8192i32 | 32i32)
                as libc::c_uint;
        tio.c_iflag |= 1i32 as libc::c_uint;
        tio.c_oflag &= !(1i32 | 4i32 | 8i32 | 32i32) as libc::c_uint;
        tio.c_lflag &=
            !(32768i32 | 2i32 | 8i32 | 16i32 | 64i32 | 512i32 | 1024i32 |
                  2048i32 | 1i32) as libc::c_uint;
        tio.c_cc[6usize] = 1i32 as cc_t;
        tio.c_cc[5usize] = 0i32 as cc_t;
        if tcsetattr((*tty).fd, 0i32, &mut tio as *mut termios) == 0i32 {
            tcflush((*tty).fd, 2i32);
        }
    }
    tty_putcode(tty, TTYC_SMCUP);
    tty_putcode(tty, TTYC_SMKX);
    tty_putcode(tty, TTYC_CLEAR);
    if 0 != tty_acs_needed(tty) {
        log_debug(b"%s: using capabilities for ACS\x00" as *const u8 as
                      *const libc::c_char, (*c).name);
        tty_putcode(tty, TTYC_ENACS);
    } else {
        log_debug(b"%s: using UTF-8 for ACS\x00" as *const u8 as
                      *const libc::c_char, (*c).name);
    }
    tty_putcode(tty, TTYC_CNORM);
    if 0 != tty_term_has((*tty).term, TTYC_KMOUS) {
        tty_puts(tty,
                 b"\x1b[?1000l\x1b[?1002l\x1b[?1006l\x1b[?1005l\x00" as
                     *const u8 as *const libc::c_char);
    }
    if 0 != tty_term_flag((*tty).term, TTYC_XT) {
        if 0 !=
               options_get_number(global_options,
                                  b"focus-events\x00" as *const u8 as
                                      *const libc::c_char) {
            (*tty).flags |= 64i32;
            tty_puts(tty,
                     b"\x1b[?1004h\x00" as *const u8 as *const libc::c_char);
        }
        tty_puts(tty, b"\x1b[c\x00" as *const u8 as *const libc::c_char);
    }
    (*tty).flags |= 16i32;
    tty_invalidate(tty);
    tty_force_cursor_colour(tty, b"\x00" as *const u8 as *const libc::c_char);
    (*tty).mouse_drag_flag = 0i32;
    (*tty).mouse_drag_update = None;
    (*tty).mouse_drag_release = None;
}
#[no_mangle]
pub unsafe extern "C" fn tty_stop_tty(mut tty: *mut tty) -> () {
    let mut ws: winsize =
        winsize{ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0,};
    if 0 == (*tty).flags & 16i32 {
        return
    } else {
        (*tty).flags &= !16i32;
        event_del(&mut (*tty).timer as *mut event);
        (*tty).flags &= !128i32;
        event_del(&mut (*tty).event_in as *mut event);
        event_del(&mut (*tty).event_out as *mut event);
        if ioctl((*tty).fd, 21523i32 as libc::c_ulong,
                 &mut ws as *mut winsize) == 1i32.wrapping_neg() {
            return
        } else if tcsetattr((*tty).fd, 0i32, &mut (*tty).tio as *mut termios)
                      == 1i32.wrapping_neg() {
            return
        } else {
            tty_raw(tty,
                    tty_term_string2((*tty).term, TTYC_CSR, 0i32,
                                     ws.ws_row as libc::c_int - 1i32));
            if 0 != tty_acs_needed(tty) {
                tty_raw(tty, tty_term_string((*tty).term, TTYC_RMACS));
            }
            tty_raw(tty, tty_term_string((*tty).term, TTYC_SGR0));
            tty_raw(tty, tty_term_string((*tty).term, TTYC_RMKX));
            tty_raw(tty, tty_term_string((*tty).term, TTYC_CLEAR));
            if 0 != tty_term_has((*tty).term, TTYC_SS) &&
                   (*tty).cstyle != 0i32 as libc::c_uint {
                if 0 != tty_term_has((*tty).term, TTYC_SE) {
                    tty_raw(tty, tty_term_string((*tty).term, TTYC_SE));
                } else {
                    tty_raw(tty,
                            tty_term_string1((*tty).term, TTYC_SS, 0i32));
                }
            }
            if 0 != (*tty).mode & 1024i32 {
                tty_raw(tty,
                        b"\x1b[?2004l\x00" as *const u8 as
                            *const libc::c_char);
            }
            tty_raw(tty, tty_term_string((*tty).term, TTYC_CR));
            tty_raw(tty, tty_term_string((*tty).term, TTYC_CNORM));
            if 0 != tty_term_has((*tty).term, TTYC_KMOUS) {
                tty_raw(tty,
                        b"\x1b[?1000l\x1b[?1002l\x1b[?1006l\x1b[?1005l\x00" as
                            *const u8 as *const libc::c_char);
            }
            if 0 != tty_term_flag((*tty).term, TTYC_XT) {
                if 0 != (*tty).flags & 64i32 {
                    (*tty).flags &= !64i32;
                    tty_raw(tty,
                            b"\x1b[?1004l\x00" as *const u8 as
                                *const libc::c_char);
                }
            }
            if (*tty).term_type as libc::c_uint ==
                   TTY_VT420 as libc::c_int as libc::c_uint {
                tty_raw(tty,
                        b"\x1b[?69l\x00" as *const u8 as *const libc::c_char);
            }
            tty_raw(tty, tty_term_string((*tty).term, TTYC_RMCUP));
            setblocking((*tty).fd, 1i32);
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_set_title(mut tty: *mut tty,
                                       mut title: *const libc::c_char) -> () {
    if 0 == tty_term_has((*tty).term, TTYC_TSL) ||
           0 == tty_term_has((*tty).term, TTYC_FSL) {
        return
    } else {
        tty_putcode(tty, TTYC_TSL);
        tty_puts(tty, title);
        tty_putcode(tty, TTYC_FSL);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_draw_pane(mut tty: *mut tty,
                                       mut wp: *const window_pane,
                                       mut py: u_int, mut ox: u_int,
                                       mut oy: u_int) -> () {
    tty_draw_line(tty, wp, (*wp).screen, py, ox, oy);
}
#[no_mangle]
pub unsafe extern "C" fn tty_draw_line(mut tty: *mut tty,
                                       mut wp: *const window_pane,
                                       mut s: *mut screen, mut py: u_int,
                                       mut ox: u_int, mut oy: u_int) -> () {
    let mut gd: *mut grid = (*s).grid;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut last: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut gcp: *const grid_cell = 0 as *const grid_cell;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut ux: u_int = 0;
    let mut sx: u_int = 0;
    let mut nx: u_int = 0;
    let mut width: u_int = 0;
    let mut flags: libc::c_int = 0;
    let mut cleared: libc::c_int = 0i32;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut len: size_t = 0;
    let mut old_len: size_t = 0;
    flags = (*tty).flags & 1i32;
    (*tty).flags |= 1i32;
    tty_update_mode(tty, (*tty).mode, s);
    tty_region_off(tty);
    tty_margin_off(tty);
    sx = (*(*s).grid).sx;
    if sx >
           (*(*gd).linedata.offset((*gd).hsize.wrapping_add(py) as
                                       isize)).cellsize {
        sx =
            (*(*gd).linedata.offset((*gd).hsize.wrapping_add(py) as
                                        isize)).cellsize
    }
    if sx > (*tty).sx { sx = (*tty).sx }
    ux = 0i32 as u_int;
    if wp == 0 as *mut libc::c_void as *const window_pane ||
           py == 0i32 as libc::c_uint ||
           0 !=
               !(*(*gd).linedata.offset((*gd).hsize.wrapping_add(py).wrapping_sub(1i32
                                                                                      as
                                                                                      libc::c_uint)
                                            as isize)).flags & 1i32 ||
           ox != 0i32 as libc::c_uint || (*tty).cx < (*tty).sx ||
           (*(*s).grid).sx < (*tty).sx {
        if (*(*s).grid).sx < (*tty).sx && ox == 0i32 as libc::c_uint &&
               sx != (*(*s).grid).sx &&
               0 != tty_term_has((*tty).term, TTYC_EL1) &&
               0 == tty_fake_bce(tty, wp, 8i32 as u_int) {
            tty_default_attributes(tty, wp, 8i32 as u_int);
            tty_cursor(tty,
                       (*(*s).grid).sx.wrapping_sub(1i32 as libc::c_uint),
                       oy.wrapping_add(py));
            tty_putcode(tty, TTYC_EL1);
            cleared = 1i32
        }
        if sx != 0i32 as libc::c_uint {
            tty_cursor(tty, ox, oy.wrapping_add(py));
        }
    } else {
        log_debug(b"%s: wrapped line %u\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"tty_draw_line\x00")).as_ptr(),
                  oy.wrapping_add(py));
    }
    memcpy(&mut last as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    len = 0i32 as size_t;
    width = 0i32 as u_int;
    i = 0i32 as u_int;
    while i < sx {
        grid_view_get_cell(gd, i, py, &mut gc as *mut grid_cell);
        gcp = tty_check_codeset(tty, &mut gc as *mut grid_cell);
        if len != 0i32 as libc::c_ulong &&
               (0 != (*gcp).attr as libc::c_int & 128i32 ||
                    (*gcp).flags as libc::c_int != last.flags as libc::c_int
                    || (*gcp).attr as libc::c_int != last.attr as libc::c_int
                    || (*gcp).fg != last.fg || (*gcp).bg != last.bg ||
                    ux.wrapping_add(width).wrapping_add((*gcp).data.width as
                                                            libc::c_uint) >=
                        (*(*s).grid).sx ||
                    (::std::mem::size_of::<[libc::c_char; 512]>() as
                         libc::c_ulong).wrapping_sub(len) <
                        (*gcp).data.size as libc::c_ulong) {
            tty_attributes(tty, &mut last as *mut grid_cell, wp);
            tty_putn(tty, buf.as_mut_ptr() as *const libc::c_void, len,
                     width);
            ux = (ux as libc::c_uint).wrapping_add(width) as u_int as u_int;
            len = 0i32 as size_t;
            width = 0i32 as u_int
        }
        if 0 != (*gcp).flags as libc::c_int & 16i32 {
            screen_select_cell(s, &mut last as *mut grid_cell, gcp);
        } else {
            memcpy(&mut last as *mut grid_cell as *mut libc::c_void,
                   gcp as *const libc::c_void,
                   ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        }
        if ux.wrapping_add((*gcp).data.width as libc::c_uint) >
               (*(*s).grid).sx {
            tty_attributes(tty, &mut last as *mut grid_cell, wp);
            j = 0i32 as u_int;
            while j < (*gcp).data.width as libc::c_uint {
                if ux.wrapping_add(j) > (*(*s).grid).sx { break ; }
                tty_putc(tty, 32 as u_char);
                ux = ux.wrapping_add(1);
                j = j.wrapping_add(1)
            }
        } else if 0 != (*gcp).attr as libc::c_int & 128i32 {
            tty_attributes(tty, &mut last as *mut grid_cell, wp);
            j = 0i32 as u_int;
            while j < (*gcp).data.size as libc::c_uint {
                tty_putc(tty, (*gcp).data.data[j as usize]);
                j = j.wrapping_add(1)
            }
            ux =
                (ux as
                     libc::c_uint).wrapping_add(gc.data.width as libc::c_uint)
                    as u_int as u_int
        } else {
            memcpy(buf.as_mut_ptr().offset(len as isize) as *mut libc::c_void,
                   (*gcp).data.data.as_ptr() as *const libc::c_void,
                   (*gcp).data.size as libc::c_ulong);
            len =
                (len as
                     libc::c_ulong).wrapping_add((*gcp).data.size as
                                                     libc::c_ulong) as size_t
                    as size_t;
            width =
                (width as
                     libc::c_uint).wrapping_add((*gcp).data.width as
                                                    libc::c_uint) as u_int as
                    u_int
        }
        i = i.wrapping_add(1)
    }
    if len != 0i32 as libc::c_ulong {
        if 0 !=
               grid_cells_equal(&mut last as *mut grid_cell,
                                &grid_default_cell as *const grid_cell) {
            old_len = len;
            while len > 0i32 as libc::c_ulong &&
                      buf[len.wrapping_sub(1i32 as libc::c_ulong) as usize] as
                          libc::c_int == 32 {
                len = len.wrapping_sub(1);
                width = width.wrapping_sub(1)
            }
            log_debug(b"%s: trimmed %zu spaces\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"tty_draw_line\x00")).as_ptr(),
                      old_len.wrapping_sub(len));
        }
        if len != 0i32 as libc::c_ulong {
            tty_attributes(tty, &mut last as *mut grid_cell, wp);
            tty_putn(tty, buf.as_mut_ptr() as *const libc::c_void, len,
                     width);
            ux = (ux as libc::c_uint).wrapping_add(width) as u_int as u_int
        }
    }
    if 0 == cleared && ux < (*(*s).grid).sx {
        nx = (*(*s).grid).sx.wrapping_sub(ux);
        tty_default_attributes(tty, wp, 8i32 as u_int);
        tty_clear_line(tty, wp, oy.wrapping_add(py), ox.wrapping_add(ux), nx,
                       8i32 as u_int);
    }
    (*tty).flags = (*tty).flags & !1i32 | flags;
    tty_update_mode(tty, (*tty).mode, s);
}
unsafe extern "C" fn tty_clear_line(mut tty: *mut tty,
                                    mut wp: *const window_pane, mut py: u_int,
                                    mut px: u_int, mut nx: u_int,
                                    mut bg: u_int) -> () {
    log_debug(b"%s: %u at %u,%u\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"tty_clear_line\x00")).as_ptr(),
              nx, px, py);
    if nx == 0i32 as libc::c_uint {
        return
    } else {
        if 0 == tty_fake_bce(tty, wp, bg) {
            if px.wrapping_add(nx) >= (*tty).sx &&
                   0 != tty_term_has((*tty).term, TTYC_EL) {
                tty_cursor(tty, px, py);
                tty_putcode(tty, TTYC_EL);
                return
            } else if px == 0i32 as libc::c_uint &&
                          0 != tty_term_has((*tty).term, TTYC_EL1) {
                tty_cursor(tty,
                           px.wrapping_add(nx).wrapping_sub(1i32 as
                                                                libc::c_uint),
                           py);
                tty_putcode(tty, TTYC_EL1);
                return
            } else if 0 != tty_term_has((*tty).term, TTYC_ECH) {
                tty_cursor(tty, px, py);
                tty_putcode1(tty, TTYC_ECH, nx as libc::c_int);
                return
            }
        }
        tty_cursor(tty, px, py);
        tty_repeat_space(tty, nx);
        return;
    };
}
unsafe extern "C" fn tty_repeat_space(mut tty: *mut tty, mut n: u_int) -> () {
    static mut s: [libc::c_char; 500] = unsafe { [0; 500] };
    if *s.as_mut_ptr() as libc::c_int != 32 {
        memset(s.as_mut_ptr() as *mut libc::c_void, 32,
               ::std::mem::size_of::<[libc::c_char; 500]>() as libc::c_ulong);
    }
    while n as libc::c_ulong >
              ::std::mem::size_of::<[libc::c_char; 500]>() as libc::c_ulong {
        tty_putn(tty, s.as_mut_ptr() as *const libc::c_void,
                 ::std::mem::size_of::<[libc::c_char; 500]>() as
                     libc::c_ulong,
                 ::std::mem::size_of::<[libc::c_char; 500]>() as libc::c_ulong
                     as u_int);
        n =
            (n as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<[libc::c_char; 500]>()
                                                 as libc::c_ulong) as u_int as
                u_int
    }
    if n != 0i32 as libc::c_uint {
        tty_putn(tty, s.as_mut_ptr() as *const libc::c_void, n as size_t, n);
    };
}
unsafe extern "C" fn tty_fake_bce(mut tty: *const tty,
                                  mut wp: *const window_pane, mut bg: u_int)
 -> libc::c_int {
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    if 0 != tty_term_flag((*tty).term, TTYC_BCE) {
        return 0i32
    } else {
        memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
               &grid_default_cell as *const grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        if wp != 0 as *mut libc::c_void as *const window_pane {
            tty_default_colours(&mut gc as *mut grid_cell, wp);
        }
        if bg != 8i32 as libc::c_uint || gc.bg != 8i32 {
            return 1i32
        } else { return 0i32 }
    };
}
unsafe extern "C" fn tty_default_attributes(mut tty: *mut tty,
                                            mut wp: *const window_pane,
                                            mut bg: u_int) -> () {
    static mut gc: grid_cell =
        unsafe {
            grid_cell{flags: 0,
                      attr: 0,
                      fg: 0,
                      bg: 0,
                      data:
                          utf8_data{data: [0; 9],
                                    have: 0,
                                    size: 0,
                                    width: 0,},}
        };
    memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    gc.bg = bg as libc::c_int;
    tty_attributes(tty, &mut gc as *mut grid_cell, wp);
}
unsafe extern "C" fn tty_check_codeset(mut tty: *mut tty,
                                       mut gc: *const grid_cell)
 -> *const grid_cell {
    static mut new: grid_cell =
        unsafe {
            grid_cell{flags: 0,
                      attr: 0,
                      fg: 0,
                      bg: 0,
                      data:
                          utf8_data{data: [0; 9],
                                    have: 0,
                                    size: 0,
                                    width: 0,},}
        };
    let mut n: u_int = 0;
    if (*gc).data.size as libc::c_int == 1i32 &&
           (*(*gc).data.data.as_ptr() as libc::c_int) < 127i32 {
        return gc
    } else if 0 != (*tty).flags & 8i32 {
        return gc
    } else {
        n = (*gc).data.width as u_int;
        if n > 9i32 as libc::c_uint { n = 9i32 as u_int }
        memcpy(&mut new as *mut grid_cell as *mut libc::c_void,
               gc as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        new.data.size = n as u_char;
        memset(new.data.data.as_mut_ptr() as *mut libc::c_void, 95,
               n as libc::c_ulong);
        return &mut new as *mut grid_cell
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_open(mut tty: *mut tty,
                                  mut cause: *mut *mut libc::c_char)
 -> libc::c_int {
    (*tty).term = tty_term_find((*tty).term_name, (*tty).fd, cause);
    if (*tty).term == 0 as *mut libc::c_void as *mut tty_term {
        tty_close(tty);
        return 1i32.wrapping_neg()
    } else {
        (*tty).flags |= 32i32;
        (*tty).flags &= !(1i32 | 2i32 | 128i32 | 4i32);
        event_set(&mut (*tty).event_in as *mut event, (*tty).fd,
                  (16i32 | 2i32) as libc::c_short, Some(tty_read_callback),
                  tty as *mut libc::c_void);
        (*tty).in_0 = evbuffer_new();
        event_set(&mut (*tty).event_out as *mut event, (*tty).fd,
                  4i32 as libc::c_short, Some(tty_write_callback),
                  tty as *mut libc::c_void);
        (*tty).out = evbuffer_new();
        event_set(&mut (*tty).timer as *mut event, 1i32.wrapping_neg(),
                  0i32 as libc::c_short, Some(tty_timer_callback),
                  tty as *mut libc::c_void);
        tty_start_tty(tty);
        tty_keys_build(tty);
        return 0i32
    };
}
unsafe extern "C" fn tty_timer_callback(mut fd: libc::c_int,
                                        mut events: libc::c_short,
                                        mut data: *mut libc::c_void) -> () {
    let mut tty: *mut tty = data as *mut tty;
    let mut c: *mut client = (*tty).client;
    let mut tv: timeval =
        timeval{tv_sec: 0, tv_usec: 100000i32 as __suseconds_t,};
    log_debug(b"%s: %zu discarded\x00" as *const u8 as *const libc::c_char,
              (*c).name, (*tty).discarded);
    (*c).flags |= 8i32;
    (*c).discarded =
        ((*c).discarded as libc::c_ulong).wrapping_add((*tty).discarded) as
            size_t as size_t;
    if (*tty).discarded <
           (1i32 as
                libc::c_uint).wrapping_add((*tty).sx.wrapping_mul((*tty).sy).wrapping_div(8i32
                                                                                              as
                                                                                              libc::c_uint))
               as libc::c_ulong {
        (*tty).flags &= !128i32;
        tty_invalidate(tty);
        return
    } else {
        (*tty).discarded = 0i32 as size_t;
        event_add(&mut (*tty).timer as *mut event, &mut tv as *mut timeval);
        return;
    };
}
unsafe extern "C" fn tty_write_callback(mut fd: libc::c_int,
                                        mut events: libc::c_short,
                                        mut data: *mut libc::c_void) -> () {
    let mut tty: *mut tty = data as *mut tty;
    let mut c: *mut client = (*tty).client;
    let mut size: size_t = evbuffer_get_length((*tty).out);
    let mut nwrite: libc::c_int = 0;
    nwrite = evbuffer_write((*tty).out, (*tty).fd);
    if nwrite == 1i32.wrapping_neg() {
        return
    } else {
        log_debug(b"%s: wrote %d bytes (of %zu)\x00" as *const u8 as
                      *const libc::c_char, (*c).name, nwrite, size);
        if (*c).redraw > 0i32 as libc::c_ulong {
            if nwrite as size_t >= (*c).redraw {
                (*c).redraw = 0i32 as size_t
            } else {
                (*c).redraw =
                    ((*c).redraw as
                         libc::c_ulong).wrapping_sub(nwrite as libc::c_ulong)
                        as size_t as size_t
            }
            log_debug(b"%s: waiting for redraw, %zu bytes left\x00" as
                          *const u8 as *const libc::c_char, (*c).name,
                      (*c).redraw);
        } else if 0 != tty_block_maybe(tty) { return }
        if evbuffer_get_length((*tty).out) != 0i32 as libc::c_ulong {
            event_add(&mut (*tty).event_out as *mut event,
                      0 as *const timeval);
        }
        return;
    };
}
unsafe extern "C" fn tty_block_maybe(mut tty: *mut tty) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut size: size_t = evbuffer_get_length((*tty).out);
    let mut tv: timeval =
        timeval{tv_sec: 0, tv_usec: 100000i32 as __suseconds_t,};
    if size <
           (1i32 as
                libc::c_uint).wrapping_add((*tty).sx.wrapping_mul((*tty).sy).wrapping_mul(8i32
                                                                                              as
                                                                                              libc::c_uint))
               as libc::c_ulong {
        return 0i32
    } else if 0 != (*tty).flags & 128i32 {
        return 1i32
    } else {
        (*tty).flags |= 128i32;
        log_debug(b"%s: can\'t keep up, %zu discarded\x00" as *const u8 as
                      *const libc::c_char, (*c).name, size);
        evbuffer_drain((*tty).out, size);
        (*c).discarded =
            ((*c).discarded as libc::c_ulong).wrapping_add(size) as size_t as
                size_t;
        (*tty).discarded = 0i32 as size_t;
        event_add(&mut (*tty).timer as *mut event, &mut tv as *mut timeval);
        return 1i32
    };
}
unsafe extern "C" fn tty_read_callback(mut fd: libc::c_int,
                                       mut events: libc::c_short,
                                       mut data: *mut libc::c_void) -> () {
    let mut tty: *mut tty = data as *mut tty;
    let mut c: *mut client = (*tty).client;
    let mut size: size_t = evbuffer_get_length((*tty).in_0);
    let mut nread: libc::c_int = 0;
    nread = evbuffer_read((*tty).in_0, (*tty).fd, 1i32.wrapping_neg());
    if nread == 0i32 || nread == 1i32.wrapping_neg() {
        event_del(&mut (*tty).event_in as *mut event);
        server_client_lost((*tty).client);
        return
    } else {
        log_debug(b"%s: read %d bytes (already %zu)\x00" as *const u8 as
                      *const libc::c_char, (*c).name, nread, size);
        while 0 != tty_keys_next(tty) { }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_close(mut tty: *mut tty) -> () {
    if 0 != event_initialized(&mut (*tty).key_timer as *mut event) {
        event_del(&mut (*tty).key_timer as *mut event);
    }
    tty_stop_tty(tty);
    if 0 != (*tty).flags & 32i32 {
        evbuffer_free((*tty).in_0);
        event_del(&mut (*tty).event_in as *mut event);
        evbuffer_free((*tty).out);
        event_del(&mut (*tty).event_out as *mut event);
        tty_term_free((*tty).term);
        tty_keys_free(tty);
        (*tty).flags &= !32i32
    }
    if (*tty).fd != 1i32.wrapping_neg() {
        close((*tty).fd);
        (*tty).fd = 1i32.wrapping_neg()
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_free(mut tty: *mut tty) -> () {
    tty_close(tty);
    free((*tty).ccolour as *mut libc::c_void);
    free((*tty).term_name as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tty_set_type(mut tty: *mut tty,
                                      mut type_0: libc::c_int) -> () {
    (*tty).term_type = type_0 as unnamed_31;
    if (*tty).term_type as libc::c_uint ==
           TTY_VT420 as libc::c_int as libc::c_uint {
        tty_puts(tty, b"\x1b[?69h\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_write(mut cmdfn:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut tty,
                                                                   _:
                                                                       *const tty_ctx)
                                                  -> ()>,
                                   mut ctx: *mut tty_ctx) -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    let mut c: *mut client = 0 as *mut client;
    if wp == 0 as *mut libc::c_void as *mut window_pane {
        return
    } else if 0 != (*wp).flags & (1i32 | 2i32) || 0 == window_pane_visible(wp)
     {
        return
    } else {
        c = (*(&mut clients as *mut clients)).tqh_first;
        while c != 0 as *mut libc::c_void as *mut client {
            if !(0 == tty_client_ready(c, wp)) {
                (*ctx).xoff = (*wp).xoff;
                (*ctx).yoff = (*wp).yoff;
                if status_at_line(c) == 0i32 {
                    (*ctx).yoff =
                        ((*ctx).yoff as
                             libc::c_uint).wrapping_add(status_line_size((*c).session))
                            as u_int as u_int
                }
                cmdfn.expect("non-null function pointer")(&mut (*c).tty as
                                                              *mut tty, ctx);
            }
            c = (*c).entry.tqe_next
        }
        return;
    };
}
unsafe extern "C" fn tty_client_ready(mut c: *mut client,
                                      mut wp: *mut window_pane)
 -> libc::c_int {
    if (*c).session == 0 as *mut libc::c_void as *mut session ||
           (*c).tty.term == 0 as *mut libc::c_void as *mut tty_term {
        return 0i32
    } else if 0 != (*c).flags & (8i32 | 64i32) {
        return 0i32
    } else if 0 != (*c).tty.flags & 2i32 {
        return 0i32
    } else if (*(*(*c).session).curw).window != (*wp).window {
        return 0i32
    } else { return 1i32 };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_alignmenttest(mut tty: *mut tty,
                                               mut ctx: *const tty_ctx)
 -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    let mut s: *mut screen = (*wp).screen;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    tty_attributes(tty, &grid_default_cell as *const grid_cell, wp);
    tty_region_pane(tty, ctx, 0i32 as u_int,
                    (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint));
    tty_margin_off(tty);
    j = 0i32 as u_int;
    while j < (*(*s).grid).sy {
        tty_cursor_pane(tty, ctx, 0i32 as u_int, j);
        i = 0i32 as u_int;
        while i < (*(*s).grid).sx {
            tty_putc(tty, 69 as u_char);
            i = i.wrapping_add(1)
        }
        j = j.wrapping_add(1)
    };
}
unsafe extern "C" fn tty_cursor_pane(mut tty: *mut tty,
                                     mut ctx: *const tty_ctx, mut cx: u_int,
                                     mut cy: u_int) -> () {
    tty_cursor(tty, (*ctx).xoff.wrapping_add(cx),
               (*ctx).yoff.wrapping_add(cy));
}
unsafe extern "C" fn tty_region_pane(mut tty: *mut tty,
                                     mut ctx: *const tty_ctx,
                                     mut rupper: u_int, mut rlower: u_int)
 -> () {
    tty_region(tty, (*ctx).yoff.wrapping_add(rupper),
               (*ctx).yoff.wrapping_add(rlower));
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_cell(mut tty: *mut tty,
                                      mut ctx: *const tty_ctx) -> () {
    if (*ctx).xoff.wrapping_add((*ctx).ocx) >
           (*tty).sx.wrapping_sub(1i32 as libc::c_uint) &&
           (*ctx).ocy == (*ctx).orlower {
        if (*ctx).xoff == 0i32 as libc::c_uint &&
               (*(*(*(*ctx).wp).screen).grid).sx >= (*tty).sx {
            tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
        } else { tty_margin_off(tty); }
    }
    tty_cursor_pane_unless_wrap(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_cell(tty, (*ctx).cell, (*ctx).wp);
}
unsafe extern "C" fn tty_cell(mut tty: *mut tty, mut gc: *const grid_cell,
                              mut wp: *const window_pane) -> () {
    let mut gcp: *const grid_cell = 0 as *const grid_cell;
    if 0 != (*(*tty).term).flags & 2i32 &&
           (*tty).cy == (*tty).sy.wrapping_sub(1i32 as libc::c_uint) &&
           (*tty).cx == (*tty).sx.wrapping_sub(1i32 as libc::c_uint) {
        return
    } else if 0 != (*gc).flags as libc::c_int & 4i32 {
        return
    } else {
        tty_attributes(tty, gc, wp);
        gcp = tty_check_codeset(tty, gc);
        if (*gcp).data.size as libc::c_int == 1i32 {
            if (*(*gcp).data.data.as_ptr() as libc::c_int) < 32i32 ||
                   *(*gcp).data.data.as_ptr() as libc::c_int == 127i32 {
                return
            } else { tty_putc(tty, *(*gcp).data.data.as_ptr()); return }
        } else {
            tty_putn(tty, (*gcp).data.data.as_ptr() as *const libc::c_void,
                     (*gcp).data.size as size_t, (*gcp).data.width as u_int);
            return;
        }
    };
}
unsafe extern "C" fn tty_cursor_pane_unless_wrap(mut tty: *mut tty,
                                                 mut ctx: *const tty_ctx,
                                                 mut cx: u_int, mut cy: u_int)
 -> () {
    if 0 == (*ctx).wrapped ||
           0 !=
               !((*ctx).xoff == 0i32 as libc::c_uint &&
                     (*(*(*(*ctx).wp).screen).grid).sx >= (*tty).sx) as
                   libc::c_int || 0 != (*(*tty).term).flags & 2i32 ||
           (*ctx).xoff.wrapping_add(cx) != 0i32 as libc::c_uint ||
           (*ctx).yoff.wrapping_add(cy) !=
               (*tty).cy.wrapping_add(1i32 as libc::c_uint) ||
           (*tty).cx < (*tty).sx || (*tty).cy == (*tty).rlower {
        tty_cursor_pane(tty, ctx, cx, cy);
    } else {
        log_debug(b"%s: will wrap at %u,%u\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 28],
                                            &[libc::c_char; 28]>(b"tty_cursor_pane_unless_wrap\x00")).as_ptr(),
                  (*tty).cx, (*tty).cy);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_cells(mut tty: *mut tty,
                                       mut ctx: *const tty_ctx) -> () {
    tty_cursor_pane_unless_wrap(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_attributes(tty, (*ctx).cell, (*ctx).wp);
    tty_putn(tty, (*ctx).ptr, (*ctx).num as size_t, (*ctx).num);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearendofline(mut tty: *mut tty,
                                                mut ctx: *const tty_ctx)
 -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    let mut nx: u_int = 0;
    let mut py: u_int = (*ctx).yoff.wrapping_add((*ctx).ocy);
    tty_default_attributes(tty, wp, (*ctx).bg);
    nx = (*(*(*wp).screen).grid).sx.wrapping_sub((*ctx).ocx);
    tty_clear_line(tty, wp, py, (*ctx).xoff.wrapping_add((*ctx).ocx), nx,
                   (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearendofscreen(mut tty: *mut tty,
                                                  mut ctx: *const tty_ctx)
 -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut nx: u_int = 0;
    let mut ny: u_int = 0;
    tty_default_attributes(tty, wp, (*ctx).bg);
    tty_region_pane(tty, ctx, 0i32 as u_int,
                    (*(*(*wp).screen).grid).sy.wrapping_sub(1i32 as
                                                                libc::c_uint));
    tty_margin_off(tty);
    px = (*ctx).xoff;
    nx = (*(*(*wp).screen).grid).sx;
    py =
        (*ctx).yoff.wrapping_add((*ctx).ocy).wrapping_add(1i32 as
                                                              libc::c_uint);
    ny =
        (*(*(*wp).screen).grid).sy.wrapping_sub((*ctx).ocy).wrapping_sub(1i32
                                                                             as
                                                                             libc::c_uint);
    tty_clear_area(tty, wp, py, ny, px, nx, (*ctx).bg);
    px = (*ctx).xoff.wrapping_add((*ctx).ocx);
    nx = (*(*(*wp).screen).grid).sx.wrapping_sub((*ctx).ocx);
    py = (*ctx).yoff.wrapping_add((*ctx).ocy);
    tty_clear_line(tty, wp, py, px, nx, (*ctx).bg);
}
unsafe extern "C" fn tty_clear_area(mut tty: *mut tty,
                                    mut wp: *const window_pane, mut py: u_int,
                                    mut ny: u_int, mut px: u_int,
                                    mut nx: u_int, mut bg: u_int) -> () {
    let mut yy: u_int = 0;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    log_debug(b"%s: %u,%u at %u,%u\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"tty_clear_area\x00")).as_ptr(),
              nx, ny, px, py);
    if nx == 0i32 as libc::c_uint || ny == 0i32 as libc::c_uint {
        return
    } else {
        if 0 == tty_fake_bce(tty, wp, bg) {
            if px == 0i32 as libc::c_uint && px.wrapping_add(nx) >= (*tty).sx
                   && py.wrapping_add(ny) >= (*tty).sy &&
                   0 != tty_term_has((*tty).term, TTYC_ED) {
                tty_cursor(tty, 0i32 as u_int, py);
                tty_putcode(tty, TTYC_ED);
                return
            } else if (*tty).term_type as libc::c_uint ==
                          TTY_VT420 as libc::c_int as libc::c_uint &&
                          bg != 8i32 as libc::c_uint {
                xsnprintf(tmp.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong,
                          b"\x1b[32;%u;%u;%u;%u$x\x00" as *const u8 as
                              *const libc::c_char,
                          py.wrapping_add(1i32 as libc::c_uint),
                          px.wrapping_add(1i32 as libc::c_uint),
                          py.wrapping_add(ny), px.wrapping_add(nx));
                tty_puts(tty, tmp.as_mut_ptr());
                return
            } else if px == 0i32 as libc::c_uint &&
                          px.wrapping_add(nx) >= (*tty).sx &&
                          ny > 2i32 as libc::c_uint &&
                          0 != tty_term_has((*tty).term, TTYC_CSR) &&
                          0 != tty_term_has((*tty).term, TTYC_INDN) {
                tty_region(tty, py,
                           py.wrapping_add(ny).wrapping_sub(1i32 as
                                                                libc::c_uint));
                tty_margin_off(tty);
                tty_putcode1(tty, TTYC_INDN, ny as libc::c_int);
                return
            } else if nx > 2i32 as libc::c_uint && ny > 2i32 as libc::c_uint
                          && 0 != tty_term_has((*tty).term, TTYC_CSR) &&
                          (*tty).term_type as libc::c_uint ==
                              TTY_VT420 as libc::c_int as libc::c_uint &&
                          0 != tty_term_has((*tty).term, TTYC_INDN) {
                tty_region(tty, py,
                           py.wrapping_add(ny).wrapping_sub(1i32 as
                                                                libc::c_uint));
                tty_margin(tty, px,
                           px.wrapping_add(nx).wrapping_sub(1i32 as
                                                                libc::c_uint));
                tty_putcode1(tty, TTYC_INDN, ny as libc::c_int);
                return
            }
        }
        yy = py;
        while yy < py.wrapping_add(ny) {
            tty_clear_line(tty, wp, yy, px, nx, bg);
            yy = yy.wrapping_add(1)
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearline(mut tty: *mut tty,
                                           mut ctx: *const tty_ctx) -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    let mut nx: u_int = 0;
    let mut py: u_int = (*ctx).yoff.wrapping_add((*ctx).ocy);
    tty_default_attributes(tty, wp, (*ctx).bg);
    nx = (*(*(*wp).screen).grid).sx;
    tty_clear_line(tty, wp, py, (*ctx).xoff, nx, (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearscreen(mut tty: *mut tty,
                                             mut ctx: *const tty_ctx) -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut nx: u_int = 0;
    let mut ny: u_int = 0;
    tty_default_attributes(tty, wp, (*ctx).bg);
    tty_region_pane(tty, ctx, 0i32 as u_int,
                    (*(*(*wp).screen).grid).sy.wrapping_sub(1i32 as
                                                                libc::c_uint));
    tty_margin_off(tty);
    px = (*ctx).xoff;
    nx = (*(*(*wp).screen).grid).sx;
    py = (*ctx).yoff;
    ny = (*(*(*wp).screen).grid).sy;
    tty_clear_area(tty, wp, py, ny, px, nx, (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearstartofline(mut tty: *mut tty,
                                                  mut ctx: *const tty_ctx)
 -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    let mut py: u_int = (*ctx).yoff.wrapping_add((*ctx).ocy);
    tty_default_attributes(tty, wp, (*ctx).bg);
    tty_clear_line(tty, wp, py, (*ctx).xoff,
                   (*ctx).ocx.wrapping_add(1i32 as libc::c_uint), (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearstartofscreen(mut tty: *mut tty,
                                                    mut ctx: *const tty_ctx)
 -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut nx: u_int = 0;
    let mut ny: u_int = 0;
    tty_default_attributes(tty, wp, (*ctx).bg);
    tty_region_pane(tty, ctx, 0i32 as u_int,
                    (*(*(*wp).screen).grid).sy.wrapping_sub(1i32 as
                                                                libc::c_uint));
    tty_margin_off(tty);
    px = (*ctx).xoff;
    nx = (*(*(*wp).screen).grid).sx;
    py = (*ctx).yoff;
    ny = (*ctx).ocy.wrapping_sub(1i32 as libc::c_uint);
    tty_clear_area(tty, wp, py, ny, px, nx, (*ctx).bg);
    px = (*ctx).xoff;
    nx = (*ctx).ocx.wrapping_add(1i32 as libc::c_uint);
    py = (*ctx).yoff.wrapping_add((*ctx).ocy);
    tty_clear_line(tty, wp, py, px, nx, (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_deletecharacter(mut tty: *mut tty,
                                                 mut ctx: *const tty_ctx)
 -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    if 0 !=
           !((*ctx).xoff == 0i32 as libc::c_uint &&
                 (*(*(*(*ctx).wp).screen).grid).sx >= (*tty).sx) as
               libc::c_int || 0 != tty_fake_bce(tty, wp, (*ctx).bg) ||
           0 == tty_term_has((*tty).term, TTYC_DCH) &&
               0 == tty_term_has((*tty).term, TTYC_DCH1) {
        tty_draw_pane(tty, wp, (*ctx).ocy, (*ctx).xoff, (*ctx).yoff);
        return
    } else {
        tty_default_attributes(tty, wp, (*ctx).bg);
        tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
        tty_emulate_repeat(tty, TTYC_DCH, TTYC_DCH1, (*ctx).num);
        return;
    };
}
unsafe extern "C" fn tty_emulate_repeat(mut tty: *mut tty,
                                        mut code: tty_code_code,
                                        mut code1: tty_code_code,
                                        mut n: u_int) -> () {
    if 0 != tty_term_has((*tty).term, code) {
        tty_putcode1(tty, code, n as libc::c_int);
    } else {
        loop  {
            let fresh0 = n;
            n = n.wrapping_sub(1);
            if !(fresh0 > 0i32 as libc::c_uint) { break ; }
            tty_putcode(tty, code1);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearcharacter(mut tty: *mut tty,
                                                mut ctx: *const tty_ctx)
 -> () {
    tty_default_attributes(tty, (*ctx).wp, (*ctx).bg);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    if 0 != tty_term_has((*tty).term, TTYC_ECH) &&
           0 == tty_fake_bce(tty, (*ctx).wp, 8i32 as u_int) {
        tty_putcode1(tty, TTYC_ECH, (*ctx).num as libc::c_int);
    } else { tty_repeat_space(tty, (*ctx).num); };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_deleteline(mut tty: *mut tty,
                                            mut ctx: *const tty_ctx) -> () {
    if 0 !=
           !((*ctx).xoff == 0i32 as libc::c_uint &&
                 (*(*(*(*ctx).wp).screen).grid).sx >= (*tty).sx) as
               libc::c_int || 0 != tty_fake_bce(tty, (*ctx).wp, (*ctx).bg) ||
           0 == tty_term_has((*tty).term, TTYC_CSR) ||
           0 == tty_term_has((*tty).term, TTYC_DL1) {
        tty_redraw_region(tty, ctx);
        return
    } else {
        tty_default_attributes(tty, (*ctx).wp, (*ctx).bg);
        tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
        tty_margin_off(tty);
        tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
        tty_emulate_repeat(tty, TTYC_DL, TTYC_DL1, (*ctx).num);
        (*tty).cy =
            (2147483647i32 as
                 libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32);
        (*tty).cx = (*tty).cy;
        return;
    };
}
unsafe extern "C" fn tty_redraw_region(mut tty: *mut tty,
                                       mut ctx: *const tty_ctx) -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    let mut s: *mut screen = (*wp).screen;
    let mut i: u_int = 0;
    if 0 != tty_large_region(tty, ctx) {
        (*wp).flags |= 1i32;
        return
    } else {
        if (*ctx).ocy < (*ctx).orupper || (*ctx).ocy > (*ctx).orlower {
            i = (*ctx).ocy;
            while i < (*(*s).grid).sy {
                tty_draw_pane(tty, wp, i, (*ctx).xoff, (*ctx).yoff);
                i = i.wrapping_add(1)
            }
        } else {
            i = (*ctx).orupper;
            while i <= (*ctx).orlower {
                tty_draw_pane(tty, wp, i, (*ctx).xoff, (*ctx).yoff);
                i = i.wrapping_add(1)
            }
        }
        return;
    };
}
unsafe extern "C" fn tty_large_region(mut tty: *mut tty,
                                      mut ctx: *const tty_ctx)
 -> libc::c_int {
    let mut wp: *mut window_pane = (*ctx).wp;
    return ((*ctx).orlower.wrapping_sub((*ctx).orupper) >=
                (*(*(*wp).screen).grid).sy.wrapping_div(2i32 as libc::c_uint))
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_insertcharacter(mut tty: *mut tty,
                                                 mut ctx: *const tty_ctx)
 -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    if 0 !=
           !((*ctx).xoff == 0i32 as libc::c_uint &&
                 (*(*(*(*ctx).wp).screen).grid).sx >= (*tty).sx) as
               libc::c_int || 0 != tty_fake_bce(tty, wp, (*ctx).bg) ||
           0 == tty_term_has((*tty).term, TTYC_ICH) &&
               0 == tty_term_has((*tty).term, TTYC_ICH1) {
        tty_draw_pane(tty, wp, (*ctx).ocy, (*ctx).xoff, (*ctx).yoff);
        return
    } else {
        tty_default_attributes(tty, wp, (*ctx).bg);
        tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
        tty_emulate_repeat(tty, TTYC_ICH, TTYC_ICH1, (*ctx).num);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_insertline(mut tty: *mut tty,
                                            mut ctx: *const tty_ctx) -> () {
    if 0 !=
           !((*ctx).xoff == 0i32 as libc::c_uint &&
                 (*(*(*(*ctx).wp).screen).grid).sx >= (*tty).sx) as
               libc::c_int || 0 != tty_fake_bce(tty, (*ctx).wp, (*ctx).bg) ||
           0 == tty_term_has((*tty).term, TTYC_CSR) ||
           0 == tty_term_has((*tty).term, TTYC_IL1) {
        tty_redraw_region(tty, ctx);
        return
    } else {
        tty_default_attributes(tty, (*ctx).wp, (*ctx).bg);
        tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
        tty_margin_off(tty);
        tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
        tty_emulate_repeat(tty, TTYC_IL, TTYC_IL1, (*ctx).num);
        (*tty).cy =
            (2147483647i32 as
                 libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32);
        (*tty).cx = (*tty).cy;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_linefeed(mut tty: *mut tty,
                                          mut ctx: *const tty_ctx) -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    if (*ctx).ocy != (*ctx).orlower {
        return
    } else if 0 !=
                  !((*ctx).xoff == 0i32 as libc::c_uint &&
                        (*(*(*(*ctx).wp).screen).grid).sx >= (*tty).sx) as
                      libc::c_int &&
                  0 !=
                      !((*tty).term_type as libc::c_uint ==
                            TTY_VT420 as libc::c_int as libc::c_uint) as
                          libc::c_int ||
                  0 != tty_fake_bce(tty, wp, 8i32 as u_int) ||
                  0 == tty_term_has((*tty).term, TTYC_CSR) {
        tty_redraw_region(tty, ctx);
        return
    } else {
        tty_default_attributes(tty, wp, (*ctx).bg);
        tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
        tty_margin_pane(tty, ctx);
        if (*ctx).xoff.wrapping_add((*ctx).ocx) > (*tty).rright {
            if 0 !=
                   !((*tty).term_type as libc::c_uint ==
                         TTY_VT420 as libc::c_int as libc::c_uint) as
                       libc::c_int {
                tty_cursor(tty, 0i32 as u_int,
                           (*ctx).yoff.wrapping_add((*ctx).ocy));
            } else {
                tty_cursor(tty, (*tty).rright,
                           (*ctx).yoff.wrapping_add((*ctx).ocy));
            }
        } else { tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy); }
        tty_putc(tty, 10 as u_char);
        return;
    };
}
unsafe extern "C" fn tty_margin_pane(mut tty: *mut tty,
                                     mut ctx: *const tty_ctx) -> () {
    tty_margin(tty, (*ctx).xoff,
               (*ctx).xoff.wrapping_add((*(*ctx).wp).sx).wrapping_sub(1i32 as
                                                                          libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_scrollup(mut tty: *mut tty,
                                          mut ctx: *const tty_ctx) -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    let mut i: u_int = 0;
    if 0 !=
           !((*ctx).xoff == 0i32 as libc::c_uint &&
                 (*(*(*(*ctx).wp).screen).grid).sx >= (*tty).sx) as
               libc::c_int &&
           0 !=
               !((*tty).term_type as libc::c_uint ==
                     TTY_VT420 as libc::c_int as libc::c_uint) as libc::c_int
           || 0 != tty_fake_bce(tty, wp, 8i32 as u_int) ||
           0 == tty_term_has((*tty).term, TTYC_CSR) {
        tty_redraw_region(tty, ctx);
        return
    } else {
        tty_default_attributes(tty, wp, (*ctx).bg);
        tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
        tty_margin_pane(tty, ctx);
        if (*ctx).num == 1i32 as libc::c_uint ||
               0 == tty_term_has((*tty).term, TTYC_INDN) {
            if 0 !=
                   !((*tty).term_type as libc::c_uint ==
                         TTY_VT420 as libc::c_int as libc::c_uint) as
                       libc::c_int {
                tty_cursor(tty, 0i32 as u_int, (*tty).rlower);
            } else { tty_cursor(tty, (*tty).rright, (*tty).rlower); }
            i = 0i32 as u_int;
            while i < (*ctx).num {
                tty_putc(tty, 10 as u_char);
                i = i.wrapping_add(1)
            }
        } else {
            tty_cursor(tty, 0i32 as u_int, (*tty).cy);
            tty_putcode1(tty, TTYC_INDN, (*ctx).num as libc::c_int);
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_reverseindex(mut tty: *mut tty,
                                              mut ctx: *const tty_ctx) -> () {
    let mut wp: *mut window_pane = (*ctx).wp;
    if (*ctx).ocy != (*ctx).orupper {
        return
    } else if 0 !=
                  !((*ctx).xoff == 0i32 as libc::c_uint &&
                        (*(*(*(*ctx).wp).screen).grid).sx >= (*tty).sx) as
                      libc::c_int || 0 != tty_fake_bce(tty, wp, 8i32 as u_int)
                  || 0 == tty_term_has((*tty).term, TTYC_CSR) ||
                  0 == tty_term_has((*tty).term, TTYC_RI) {
        tty_redraw_region(tty, ctx);
        return
    } else {
        tty_default_attributes(tty, wp, (*ctx).bg);
        tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
        tty_margin_off(tty);
        tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).orupper);
        tty_putcode(tty, TTYC_RI);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_setselection(mut tty: *mut tty,
                                              mut ctx: *const tty_ctx) -> () {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut off: size_t = 0;
    if 0 == tty_term_has((*tty).term, TTYC_MS) {
        return
    } else {
        off =
            (4i32 as
                 libc::c_uint).wrapping_mul((*ctx).num.wrapping_add(2i32 as
                                                                        libc::c_uint).wrapping_div(3i32
                                                                                                       as
                                                                                                       libc::c_uint)).wrapping_add(1i32
                                                                                                                                       as
                                                                                                                                       libc::c_uint)
                as size_t;
        buf = xmalloc(off) as *mut libc::c_char;
        b64_ntop((*ctx).ptr as *const libc::c_char, (*ctx).num as size_t, buf,
                 off);
        tty_putcode_ptr2(tty, TTYC_MS,
                         b"\x00" as *const u8 as *const libc::c_char as
                             *const libc::c_void, buf as *const libc::c_void);
        free(buf as *mut libc::c_void);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_rawstring(mut tty: *mut tty,
                                           mut ctx: *const tty_ctx) -> () {
    tty_add(tty, (*ctx).ptr as *const libc::c_char, (*ctx).num as size_t);
    tty_invalidate(tty);
}

