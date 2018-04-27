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
#![allow(private_no_mangle_fns)]
#![allow(private_no_mangle_statics)]
extern crate libc;

mod arguments;
mod attributes;
mod cfg;
mod client;
mod common;
mod cmd;
mod cmd_queue;
mod cmd_attach_session;
mod cmd_bind_key;
mod cmd_find;
mod cmd_find_window;
mod cmd_list;
mod colour;
mod compat;
mod environ;
mod grid;
mod hooks;
mod log;
mod notify;
mod options;
mod osdep;
mod proc_;
mod session;
mod server;
mod style;
mod window;
mod xmalloc;

use cfg::set_cfg_file;
use client::{client_main, clients};
use cmd_queue::{global_queue, cmdq_item};
use compat::getprogname::getprogname;
use compat::fdforkpty::getptmfd;
use environ::{environ_create, environ_put};
use hooks::hooks_create;
use log::log_add_level;
use options::{options_create, options_default, options_set_number, options_table_entry};
use osdep::{osdep_event_init};
use proc_::event_base;
use session::sessions;

extern "C" {
    pub type _IO_FILE_plus;
    pub type input_ctx;
    pub type evbuffer;
    pub type format_job_tree;
    pub type options_entry;
    pub type format_tree;
    pub type args_entry;
    pub type bufferevent_ops;
    pub type tty_code;
    pub type screen_titles;
    // LINKME: Variadic
    #[no_mangle]
    fn environ_set(_: *mut environ::environ, _: *const libc::c_char,
                   _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn options_set_string(_: *mut options::options, _: *const libc::c_char,
                          _: libc::c_int, _: *const libc::c_char, ...);

    #[no_mangle]
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, ...) -> libc::c_int;
    #[no_mangle]
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
    #[no_mangle]
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn realpath(__name: *const libc::c_char, __resolved: *mut libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    static mut __tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut __daylight: libc::c_int;
    #[no_mangle]
    static mut __timezone: libc::c_long;
    #[no_mangle]
    static mut tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    fn tzset() -> ();
    #[no_mangle]
    static mut daylight: libc::c_int;
    #[no_mangle]
    static mut timezone: libc::c_long;
    #[no_mangle]
    fn access(__name: *const libc::c_char, __type: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn getuid() -> __uid_t;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    fn err(_: libc::c_int, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn errx(_: libc::c_int, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn strcasestr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> libc::c_ulong;
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
    fn BSDgetopt(_: libc::c_int, _: *const *mut libc::c_char,
                 _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    static grid_default_cell: grid::grid_cell;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    static mut all_window_panes: window::window_pane_tree;
    #[no_mangle]
    static window_buffer_mode: window::window_mode;
    #[no_mangle]
    static window_tree_mode: window::window_mode;
    #[no_mangle]
    static window_clock_mode: window::window_mode;
    #[no_mangle]
    static window_clock_table: [[[libc::c_char; 5]; 5]; 14];
    #[no_mangle]
    static window_client_mode: window::window_mode;
    #[no_mangle]
    static window_copy_mode: window::window_mode;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    static mut session_groups: session_groups;
}

#[no_mangle]
pub static mut shell_command: *const libc::c_char = 0 as *const libc::c_char;

#[no_mangle]
pub static mut ptm_fd: libc::c_int = -1;

#[no_mangle]
pub static mut socket_path: *const libc::c_char = 0 as *const libc::c_char;

#[no_mangle]
pub static mut global_hooks: *mut hooks::hooks = 0 as *mut hooks::hooks;

#[no_mangle]
pub static mut global_options: *mut options::options = 0 as *mut options::options;

#[no_mangle]
pub static mut global_s_options: *mut options::options = 0 as *mut options::options;

#[no_mangle]
pub static mut global_w_options: *mut options::options = 0 as *mut options::options;

#[no_mangle]
pub static mut global_environ: *mut environ::environ = 0 as *mut environ::environ;

#[no_mangle]
pub static mut start_time: timeval = unsafe { timeval{tv_sec: 0, tv_usec: 0,} };

pub const TTY_VT102: unnamed_29 = 2;
pub const _NL_WT_FMT_AMPM: unnamed_18 = 131167;
pub const __MON_THOUSANDS_SEP: unnamed_18 = 262147;
pub const _NL_COLLATE_CODESET: unnamed_18 = 196626;
pub const __INT_N_CS_PRECEDES: unnamed_18 = 262162;
pub const _NL_CTYPE_INDIGITS0_MB: unnamed_18 = 20;
pub const __THOUSANDS_SEP: unnamed_18 = 65537;
pub const _NL_ADDRESS_COUNTRY_AB2: unnamed_18 = 589827;
pub const _NL_ADDRESS_LANG_TERM: unnamed_18 = 589834;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: unnamed_18 = 262180;
pub const _NL_NUMERIC_CODESET: unnamed_18 = 65541;
pub const _NL_WERA_YEAR: unnamed_18 = 131168;
pub const _NL_IDENTIFICATION_TITLE: unnamed_18 = 786432;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const _NL_WAM_STR: unnamed_18 = 131162;
pub const _NL_TELEPHONE_CODESET: unnamed_18 = 655364;
pub const CODESET: unnamed_18 = 14;
pub type unnamed = libc::c_uint;
pub const ERA: unnamed_18 = 131116;
pub const _NL_NUM_LC_MESSAGES: unnamed_18 = 327685;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink {
    pub idx: libc::c_int,
    pub session: *mut session::session,
    pub window: *mut window::window,
    pub status_width: size_t,
    pub status_cell: grid::grid_cell,
    pub status_text: *mut libc::c_char,
    pub flags: libc::c_int,
    pub entry: unnamed_16,
    pub wentry: unnamed_24,
    pub sentry: unnamed_27,
}
pub const __NOEXPR: unnamed_18 = 327681;
pub const _NL_COLLATE_GAP1: unnamed_18 = 196614;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
pub const _DATE_FMT: unnamed_18 = 131180;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: unnamed_18 = 262181;
pub const _NL_CTYPE_OUTDIGIT7_WC: unnamed_18 = 58;
pub const _NL_CTYPE_OUTDIGIT2_WC: unnamed_18 = 53;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const _NL_CTYPE_INDIGITS6_MB: unnamed_18 = 26;
pub const ABMON_3: unnamed_18 = 131088;
pub const JOB_DEAD: unnamed_33 = 1;
pub type __u_int = libc::c_uint;
pub const _NL_CTYPE_INDIGITS1_MB: unnamed_18 = 21;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const _NL_WMON_4: unnamed_18 = 131153;
pub const _NL_CTYPE_OUTDIGIT0_WC: unnamed_18 = 51;
pub const _NL_CTYPE_OUTDIGIT8_MB: unnamed_18 = 49;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: unnamed_18 = 65540;
pub const _NL_NUM_LC_MEASUREMENT: unnamed_18 = 720898;
pub type __syscall_slong_t = libc::c_long;
pub const __P_SEP_BY_SPACE: unnamed_18 = 262154;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqh_first: *mut session::session,
    pub tqh_last: *mut *mut session::session,
}
pub const _NL_MONETARY_DECIMAL_POINT_WC: unnamed_18 = 262187;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    ev_next_with_common_timeout: unnamed_19,
    min_heap_idx: libc::c_int,
}
pub const _NL_IDENTIFICATION_SOURCE: unnamed_18 = 786433;
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut window::window,
    pub tqe_prev: *mut *mut window::window,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const _NL_CTYPE_EXTRA_MAP_10: unnamed_18 = 81;
pub const _NL_CTYPE_EXTRA_MAP_1: unnamed_18 = 72;
pub const _NL_ADDRESS_COUNTRY_AB3: unnamed_18 = 589828;
pub const _NL_IDENTIFICATION_ADDRESS: unnamed_18 = 786434;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const _NL_NUM: unnamed_18 = 786449;
pub const _NL_WALT_DIGITS: unnamed_18 = 131170;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list::cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_38,
}
pub const PM_STR: unnamed_18 = 131111;
pub const _NL_CTYPE_INDIGITS0_WC: unnamed_18 = 31;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const DAY_1: unnamed_18 = 131079;
pub const _NL_WABMON_9: unnamed_18 = 131146;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: unnamed_18 = 262167;
pub const _NL_CTYPE_GAP5: unnamed_18 = 8;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const _NL_CTYPE_INDIGITS4_WC: unnamed_18 = 35;
pub const _NL_COLLATE_INDIRECTWC: unnamed_18 = 196620;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const ABDAY_7: unnamed_18 = 131078;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: unnamed_18 = 262168;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: unnamed_18 = 196621;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: unnamed_18 = 262166;
pub const _NL_IDENTIFICATION_LANGUAGE: unnamed_18 = 786439;
pub type uid_t = __uid_t;
pub const MON_8: unnamed_18 = 131105;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub rbe_left: *mut window::window,
    pub rbe_right: *mut window::window,
    pub rbe_parent: *mut window::window,
    pub rbe_color: libc::c_int,
}
pub const _NL_CTYPE_EXTRA_MAP_6: unnamed_18 = 77;
pub const _NL_WABDAY_1: unnamed_18 = 131124;
pub const _NL_COLLATE_TABLEMB: unnamed_18 = 196610;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const _NL_CTYPE_OUTDIGIT4_WC: unnamed_18 = 55;
pub const _NL_WMON_1: unnamed_18 = 131150;
pub const ABDAY_2: unnamed_18 = 131073;
pub const ABMON_8: unnamed_18 = 131093;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const MON_10: unnamed_18 = 131107;
pub const _NL_CTYPE_INDIGITS5_WC: unnamed_18 = 36;
pub type __blksize_t = libc::c_long;
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
pub const _NL_IDENTIFICATION_CONTACT: unnamed_18 = 786435;
pub const _NL_WDAY_5: unnamed_18 = 131135;
pub const _NL_NAME_CODESET: unnamed_18 = 524294;
pub const _NL_CTYPE_EXTRA_MAP_13: unnamed_18 = 84;
pub const MON_11: unnamed_18 = 131108;
pub const __MON_DECIMAL_POINT: unnamed_18 = 262146;
pub const _NL_CTYPE_EXTRA_MAP_2: unnamed_18 = 73;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub const _NL_CTYPE_TOUPPER: unnamed_18 = 1;
pub const _NL_TIME_FIRST_WORKDAY: unnamed_18 = 131177;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const _NL_WABMON_3: unnamed_18 = 131140;
pub type __u_char = libc::c_uchar;
pub const _NL_NAME_NAME_MISS: unnamed_18 = 524292;
pub const ABMON_6: unnamed_18 = 131091;
pub const _NL_COLLATE_EXTRAWC: unnamed_18 = 196619;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client::client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const _NL_MONETARY_DUO_VALID_FROM: unnamed_18 = 262184;
pub const _NL_CTYPE_EXTRA_MAP_14: unnamed_18 = 85;
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub const D_T_FMT: unnamed_18 = 131112;
pub const _NL_CTYPE_OUTDIGIT7_MB: unnamed_18 = 48;
pub const ABDAY_4: unnamed_18 = 131075;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_8,
}
pub const _NL_CTYPE_TOLOWER: unnamed_18 = 3;
pub const ABDAY_5: unnamed_18 = 131076;
pub const __INT_P_CS_PRECEDES: unnamed_18 = 262160;
pub const __P_SIGN_POSN: unnamed_18 = 262157;
pub const _NL_CTYPE_MAP_TO_NONASCII: unnamed_18 = 70;
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
pub const _NL_CTYPE_TOUPPER32: unnamed_18 = 15;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_37,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid::grid_cell,
}
pub const _NL_CTYPE_OUTDIGIT8_WC: unnamed_18 = 59;
pub type FILE = _IO_FILE;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub ev_io_next: unnamed_17,
    pub ev_timeout: timeval,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const __INT_P_SEP_BY_SPACE: unnamed_18 = 262161;
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
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd::cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const _NL_PAPER_HEIGHT: unnamed_18 = 458752;
pub const _NL_WERA_T_FMT: unnamed_18 = 131172;
pub const LINE_SEL_RIGHT_LEFT: unnamed_37 = 2;
pub const T_FMT: unnamed_18 = 131114;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: unnamed_18 = 61;
pub const _NL_CTYPE_INDIGITS_MB_LEN: unnamed_18 = 19;
pub const LINE_SEL_LEFT_RIGHT: unnamed_37 = 1;
pub const _NL_MONETARY_CODESET: unnamed_18 = 262189;
pub const T_FMT_AMPM: unnamed_18 = 131115;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const _NL_WMON_12: unnamed_18 = 131161;
pub const DAY_3: unnamed_18 = 131081;
pub const _NL_CTYPE_INDIGITS6_WC: unnamed_18 = 37;
pub const MON_9: unnamed_18 = 131106;
pub const __P_CS_PRECEDES: unnamed_18 = 262153;
pub const _NL_COLLATE_SYMB_TABLEMB: unnamed_18 = 196622;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: unnamed_18 = 262170;
pub const _NL_CTYPE_INDIGITS9_WC: unnamed_18 = 40;
pub const _NL_WERA_D_FMT: unnamed_18 = 131169;
pub const _NL_MONETARY_UNO_VALID_TO: unnamed_18 = 262183;
pub const _NL_WMON_8: unnamed_18 = 131157;
pub const _NL_MONETARY_CONVERSION_RATE: unnamed_18 = 262186;
pub type speed_t = libc::c_uint;
pub const _NL_CTYPE_INDIGITS2_WC: unnamed_18 = 33;
pub type __time_t = libc::c_long;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const _NL_NUM_LC_TIME: unnamed_18 = 131183;
pub const _NL_NUM_LC_MONETARY: unnamed_18 = 262190;
pub const ERA_D_FMT: unnamed_18 = 131118;
pub const _NL_CTYPE_OUTDIGIT9_WC: unnamed_18 = 60;
pub const ERA_D_T_FMT: unnamed_18 = 131120;
pub const TTY_UNKNOWN: unnamed_29 = 6;
pub const _NL_WABDAY_4: unnamed_18 = 131127;
pub const TTY_VT420: unnamed_29 = 5;
pub const ABDAY_6: unnamed_18 = 131077;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub ev_signal_next: unnamed_3,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub type uint8_t = libc::c_uchar;
pub const _NL_TIME_WEEK_1STWEEK: unnamed_18 = 131175;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const __N_CS_PRECEDES: unnamed_18 = 262155;
pub const _NL_CTYPE_INDIGITS7_MB: unnamed_18 = 27;
pub const _NL_CTYPE_NONASCII_CASE: unnamed_18 = 71;
pub const _NL_NUM_LC_NAME: unnamed_18 = 524295;
pub const AM_STR: unnamed_18 = 131110;
pub const _NL_NUM_LC_TELEPHONE: unnamed_18 = 655365;
pub const ABMON_5: unnamed_18 = 131090;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub const _NL_CTYPE_CLASS_NAMES: unnamed_18 = 10;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: unnamed_18 = 63;
pub type __mode_t = libc::c_uint;
pub const TTY_VT100: unnamed_29 = 0;
pub const _NL_ADDRESS_COUNTRY_POST: unnamed_18 = 589826;
pub const _NL_WABMON_11: unnamed_18 = 131148;
pub const _NL_CTYPE_OUTDIGIT1_MB: unnamed_18 = 42;
pub type nl_item = libc::c_int;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: unnamed_18 = 67;
pub const __YESEXPR: unnamed_18 = 327680;
pub const _NL_COLLATE_TABLEWC: unnamed_18 = 196617;
pub type __off64_t = libc::c_long;
pub const _NL_WDAY_4: unnamed_18 = 131134;
pub const PROMPT_ENTRY: unnamed = 0;
pub const _NL_CTYPE_EXTRA_MAP_12: unnamed_18 = 83;
pub const _NL_TIME_WEEK_1STDAY: unnamed_18 = 131174;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const _NL_CTYPE_OUTDIGIT0_MB: unnamed_18 = 41;
pub const ABMON_2: unnamed_18 = 131087;
pub const _NL_CTYPE_CLASS32: unnamed_18 = 5;
pub type cmd_find_type = libc::c_uint;
pub const _NL_IDENTIFICATION_DATE: unnamed_18 = 786445;
pub const _NL_WABMON_8: unnamed_18 = 131145;
pub const _NL_CTYPE_WIDTH: unnamed_18 = 12;
pub const _NL_WMON_6: unnamed_18 = 131155;
pub const JOB_CLOSED: unnamed_33 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: unnamed_18 = 262176;
pub const _NL_CTYPE_OUTDIGIT4_MB: unnamed_18 = 45;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: unnamed_18 = 68;
pub const _NL_TIME_ERA_ENTRIES: unnamed_18 = 131123;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub rbe_left: *mut session::session,
    pub rbe_right: *mut session::session,
    pub rbe_parent: *mut session::session,
    pub rbe_color: libc::c_int,
}
pub const _NL_CTYPE_CLASS: unnamed_18 = 0;
pub const __GROUPING: unnamed_18 = 65538;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: unnamed_18 = 262171;
pub const _NL_CTYPE_INDIGITS5_MB: unnamed_18 = 25;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const _NL_CTYPE_TRANSLIT_TO_TBL: unnamed_18 = 65;
pub const _NL_IDENTIFICATION_CODESET: unnamed_18 = 786447;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const _NL_CTYPE_EXTRA_MAP_11: unnamed_18 = 82;
pub type u_short = __u_short;
pub const _NL_ADDRESS_LANG_NAME: unnamed_18 = 589832;
pub type unnamed_18 = libc::c_uint;
pub const _NL_TIME_WEEK_NDAYS: unnamed_18 = 131173;
pub const _NL_COLLATE_NRULES: unnamed_18 = 196608;
pub type cmdq_type = libc::c_uint;
pub const __MON_GROUPING: unnamed_18 = 262148;
pub const ABMON_1: unnamed_18 = 131086;
pub const _NL_W_DATE_FMT: unnamed_18 = 131181;
pub const _NL_WMON_2: unnamed_18 = 131151;
pub const __INT_FRAC_DIGITS: unnamed_18 = 262151;
pub const _NL_WABDAY_3: unnamed_18 = 131126;
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const _NL_CTYPE_OUTDIGIT5_WC: unnamed_18 = 56;
pub const _NL_COLLATE_SYMB_EXTRAMB: unnamed_18 = 196623;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub const _NL_WD_FMT: unnamed_18 = 131165;
pub const _NL_TELEPHONE_INT_PREFIX: unnamed_18 = 655363;
pub type tcflag_t = libc::c_uint;
pub const _NL_MONETARY_DUO_VALID_TO: unnamed_18 = 262185;
pub const _NL_TIME_FIRST_WEEKDAY: unnamed_18 = 131176;
pub const __N_SEP_BY_SPACE: unnamed_18 = 262156;
pub const ABMON_9: unnamed_18 = 131094;
pub const _NL_COLLATE_GAP2: unnamed_18 = 196615;
pub const _NL_COLLATE_RULESETS: unnamed_18 = 196609;
pub const _NL_WABDAY_7: unnamed_18 = 131130;
pub type __off_t = libc::c_long;
pub const DAY_7: unnamed_18 = 131085;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type uint32_t = libc::c_uint;
pub const _NL_PAPER_CODESET: unnamed_18 = 458754;
pub const _NL_WABMON_6: unnamed_18 = 131143;
pub const _NL_IDENTIFICATION_CATEGORY: unnamed_18 = 786446;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: unnamed_18 = 262188;
pub const _NL_CTYPE_INDIGITS7_WC: unnamed_18 = 38;
pub const __NEGATIVE_SIGN: unnamed_18 = 262150;
pub const _NL_WMON_3: unnamed_18 = 131152;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type options_table_scope = libc::c_uint;
pub const _NL_NAME_NAME_FMT: unnamed_18 = 524288;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const _NL_WABMON_4: unnamed_18 = 131141;
pub const _NL_ADDRESS_POSTAL_FMT: unnamed_18 = 589824;
pub const _NL_TIME_TIMEZONE: unnamed_18 = 131179;
pub const _NL_TELEPHONE_TEL_INT_FMT: unnamed_18 = 655360;
pub const _NL_WABDAY_2: unnamed_18 = 131125;
pub const _NL_WDAY_6: unnamed_18 = 131136;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: unnamed_18 = 66;
pub const ABMON_4: unnamed_18 = 131089;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub type __nlink_t = libc::c_ulong;
pub const ABMON_12: unnamed_18 = 131097;
pub const _NL_CTYPE_EXTRA_MAP_3: unnamed_18 = 74;
pub const ALT_DIGITS: unnamed_18 = 131119;
pub const _NL_COLLATE_INDIRECTMB: unnamed_18 = 196613;
pub const DAY_4: unnamed_18 = 131082;
pub const _NL_ADDRESS_COUNTRY_NAME: unnamed_18 = 589825;
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_23 {
    ev_io: unnamed_9,
    ev_signal: unnamed_12,
}
pub const _NL_CTYPE_INDIGITS2_MB: unnamed_18 = 22;
pub const _NL_CTYPE_MAP_NAMES: unnamed_18 = 11;
pub type __uid_t = libc::c_uint;
pub const ABDAY_3: unnamed_18 = 131074;
pub type u_char = __u_char;
pub const _NL_CTYPE_MB_CUR_MAX: unnamed_18 = 13;
pub const _NL_WABMON_1: unnamed_18 = 131138;
pub const CMDQ_COMMAND: cmdq_type = 0;
pub const _NL_WABDAY_6: unnamed_18 = 131129;
pub const _NL_IDENTIFICATION_EMAIL: unnamed_18 = 786436;
pub const _NL_CTYPE_GAP3: unnamed_18 = 6;
pub const _NL_PAPER_WIDTH: unnamed_18 = 458753;
pub const _NL_COLLATE_WEIGHTMB: unnamed_18 = 196611;
pub type __blkcnt_t = libc::c_long;
pub const MON_7: unnamed_18 = 131104;
pub const _NL_NUM_LC_PAPER: unnamed_18 = 458755;
pub const RADIXCHAR: unnamed_18 = 65536;
pub const _NL_CTYPE_OUTDIGIT1_WC: unnamed_18 = 52;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: unnamed_18 = 262174;
pub const DAY_5: unnamed_18 = 131083;
pub const _NL_MEASUREMENT_CODESET: unnamed_18 = 720897;
pub type cc_t = libc::c_uchar;
pub const _NL_CTYPE_INDIGITS8_WC: unnamed_18 = 39;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const _NL_MESSAGES_CODESET: unnamed_18 = 327684;
pub const _NL_CTYPE_INDIGITS8_MB: unnamed_18 = 28;
pub const _NL_CTYPE_OUTDIGIT6_MB: unnamed_18 = 47;
pub const JOB_RUNNING: unnamed_33 = 0;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const MON_12: unnamed_18 = 131109;
pub type u_int = __u_int;
pub const __N_SIGN_POSN: unnamed_18 = 262158;
pub const _NL_CTYPE_GAP4: unnamed_18 = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub const _NL_TIME_ERA_NUM_ENTRIES: unnamed_18 = 131122;
pub const _NL_IDENTIFICATION_TERRITORY: unnamed_18 = 786440;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const _NL_CTYPE_OUTDIGIT2_MB: unnamed_18 = 43;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_28,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_25 {
    offset: u_int,
    data: unnamed_32,
}
pub const _NL_CTYPE_TRANSLIT_TO_IDX: unnamed_18 = 64;
pub const _NL_ADDRESS_CODESET: unnamed_18 = 589836;
pub const MON_6: unnamed_18 = 131103;
pub const _NL_ADDRESS_COUNTRY_CAR: unnamed_18 = 589829;
pub const _NL_CTYPE_INDIGITS9_MB: unnamed_18 = 29;
pub const LINE_SEL_NONE: unnamed_37 = 0;
pub const MON_3: unnamed_18 = 131100;
pub const _NL_WABDAY_5: unnamed_18 = 131128;
pub const THOUSEP: unnamed_18 = 65537;
pub const __FRAC_DIGITS: unnamed_18 = 262152;
pub const _NL_NUM_LC_ADDRESS: unnamed_18 = 589837;
pub const _NL_CTYPE_OUTDIGIT3_MB: unnamed_18 = 44;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_20,
    pub ev_next: unnamed_26,
    pub ev_timeout_pos: unnamed_1,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_23,
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
pub const _NL_MONETARY_DUO_FRAC_DIGITS: unnamed_18 = 262169;
pub const _NL_CTYPE_INDIGITS3_WC: unnamed_18 = 34;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const _NL_COLLATE_EXTRAMB: unnamed_18 = 196612;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: unnamed_18 = 262175;
pub type time_t = __time_t;
pub const _NL_CTYPE_GAP1: unnamed_18 = 2;
pub const _NL_CTYPE_OUTDIGIT5_MB: unnamed_18 = 46;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_0,
    pub entry: unnamed_11,
}
pub type __pid_t = libc::c_int;
pub const DAY_2: unnamed_18 = 131080;
pub const _NL_CTYPE_OUTDIGIT9_MB: unnamed_18 = 50;
pub const _NL_CTYPE_EXTRA_MAP_9: unnamed_18 = 80;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: unnamed_18 = 65539;
pub const _NL_COLLATE_WEIGHTWC: unnamed_18 = 196618;
pub const _NL_CTYPE_GAP6: unnamed_18 = 9;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const MON_2: unnamed_18 = 131099;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen {
    pub title: *mut libc::c_char,
    pub titles: *mut screen_titles,
    pub grid: *mut grid::grid,
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
pub type unnamed_29 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window::window,
}
pub const _NL_NUM_LC_IDENTIFICATION: unnamed_18 = 786448;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: unnamed_18 = 262179;
pub const _NL_CTYPE_INDIGITS1_WC: unnamed_18 = 32;
pub const _NL_CTYPE_TRANSLIT_IGNORE: unnamed_18 = 69;
pub const _NL_WMON_7: unnamed_18 = 131156;
pub const ABMON_10: unnamed_18 = 131095;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const ERA_T_FMT: unnamed_18 = 131121;
pub const _NL_WABMON_7: unnamed_18 = 131144;
pub type cmd_retval = libc::c_int;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: unnamed_18 = 262173;
pub const ABMON_7: unnamed_18 = 131092;
pub const TTY_VT220: unnamed_29 = 3;
pub const _NL_CTYPE_MAP_OFFSET: unnamed_18 = 18;
pub const _NL_TELEPHONE_INT_SELECT: unnamed_18 = 655362;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub type layout_type = libc::c_uint;
pub type __dev_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub const _NL_NAME_NAME_GEN: unnamed_18 = 524289;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_line {
    pub cellused: u_int,
    pub cellsize: u_int,
    pub celldata: *mut grid::grid_cell_entry,
    pub extdsize: u_int,
    pub extddata: *mut grid::grid_cell,
    pub flags: libc::c_int,
}
pub const _NL_WDAY_7: unnamed_18 = 131137;
pub const _NL_CTYPE_EXTRA_MAP_5: unnamed_18 = 76;
pub const _NL_IDENTIFICATION_APPLICATION: unnamed_18 = 786442;
pub const _NL_MONETARY_UNO_VALID_FROM: unnamed_18 = 262182;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub const _NL_WDAY_1: unnamed_18 = 131131;
pub const ABDAY_1: unnamed_18 = 131072;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: unnamed_18 = 262177;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqe_next: *mut session::session,
    pub tqe_prev: *mut *mut session::session,
}
pub const _NL_CTYPE_INDIGITS3_MB: unnamed_18 = 23;
pub const _NL_IDENTIFICATION_REVISION: unnamed_18 = 786444;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut client::client,
    pub tqe_prev: *mut *mut client::client,
}
pub const _NL_MONETARY_CRNCYSTR: unnamed_18 = 262159;
pub type __ino_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const TTY_VT320: unnamed_29 = 4;
pub const __DECIMAL_POINT: unnamed_18 = 65536;
pub const _NL_WABMON_5: unnamed_18 = 131142;
pub type __suseconds_t = libc::c_long;
pub const _NL_CTYPE_EXTRA_MAP_7: unnamed_18 = 78;
pub const _NL_WT_FMT: unnamed_18 = 131166;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const _NL_TIME_CAL_DIRECTION: unnamed_18 = 131178;
pub type unnamed_33 = libc::c_uint;
pub const _NL_CTYPE_OUTDIGIT6_WC: unnamed_18 = 57;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: unnamed_18 = 62;
pub const _NL_COLLATE_GAP3: unnamed_18 = 196616;
pub type __gid_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_36,
}
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
pub const __ERA_YEAR: unnamed_18 = 131117;
pub const _NL_ADDRESS_COUNTRY_ISBN: unnamed_18 = 589831;
pub const __INT_N_SEP_BY_SPACE: unnamed_18 = 262163;
pub const MON_5: unnamed_18 = 131102;
pub const _NL_ADDRESS_COUNTRY_NUM: unnamed_18 = 589830;
pub const _NL_NAME_NAME_MS: unnamed_18 = 524293;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqh_first: *mut cmd::cmd,
    pub tqh_last: *mut *mut cmd::cmd,
}
pub type pid_t = __pid_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut cmd::cmd,
    pub tqe_prev: *mut *mut cmd::cmd,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const _NL_MEASUREMENT_MEASUREMENT: unnamed_18 = 720896;
pub const _NL_WERA_D_T_FMT: unnamed_18 = 131171;
pub const _NL_NAME_NAME_MRS: unnamed_18 = 524291;
pub const _NL_CTYPE_GAP2: unnamed_18 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_36 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const PROMPT_COMMAND: unnamed = 1;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const _NL_CTYPE_OUTDIGIT3_WC: unnamed_18 = 54;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_33,
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
    pub entry: unnamed_5,
}
pub const __INT_N_SIGN_POSN: unnamed_18 = 262165;
pub const _NL_TELEPHONE_TEL_DOM_FMT: unnamed_18 = 655361;
pub const _NL_IDENTIFICATION_ABBREVIATION: unnamed_18 = 786443;
pub const __INT_CURR_SYMBOL: unnamed_18 = 262144;
pub type _IO_lock_t = ();
pub const MON_4: unnamed_18 = 131101;
pub const __CURRENCY_SYMBOL: unnamed_18 = 262145;
pub const _NL_WPM_STR: unnamed_18 = 131163;
pub const _NL_WMON_11: unnamed_18 = 131160;
pub const _NL_IDENTIFICATION_TEL: unnamed_18 = 786437;
pub const DAY_6: unnamed_18 = 131084;
pub const _NL_COLLATE_COLLSEQMB: unnamed_18 = 196624;
pub const _NL_NUM_LC_CTYPE: unnamed_18 = 86;
pub const _NL_COLLATE_COLLSEQWC: unnamed_18 = 196625;
pub const MON_1: unnamed_18 = 131098;
pub const _NL_CTYPE_INDIGITS4_MB: unnamed_18 = 24;
pub const _NL_CTYPE_EXTRA_MAP_4: unnamed_18 = 75;
pub const TTY_VT101: unnamed_29 = 1;
pub const _NL_ADDRESS_LANG_LIB: unnamed_18 = 589835;
pub const _NL_NUM_LC_COLLATE: unnamed_18 = 196627;
pub const __YESSTR: unnamed_18 = 327682;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const _NL_CTYPE_EXTRA_MAP_8: unnamed_18 = 79;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: unnamed_18 = 262178;
pub const _NL_IDENTIFICATION_FAX: unnamed_18 = 786438;
pub const __INT_P_SIGN_POSN: unnamed_18 = 262164;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const __NOSTR: unnamed_18 = 327683;
pub const _NL_WMON_10: unnamed_18 = 131159;
pub const ABMON_11: unnamed_18 = 131096;
pub const _NL_CTYPE_TOLOWER32: unnamed_18 = 16;
pub const _NL_WMON_9: unnamed_18 = 131158;
pub const _NL_CTYPE_CODESET_NAME: unnamed_18 = 14;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const _NL_TIME_CODESET: unnamed_18 = 131182;
pub const _NL_WD_T_FMT: unnamed_18 = 131164;
pub const _NL_CTYPE_CLASS_OFFSET: unnamed_18 = 17;
pub type options_table_type = libc::c_uint;
pub const _NL_WMON_5: unnamed_18 = 131154;
pub const _NL_WABMON_10: unnamed_18 = 131147;
pub const _NL_WDAY_3: unnamed_18 = 131133;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: unnamed_18 = 262172;
pub const __POSITIVE_SIGN: unnamed_18 = 262149;
pub const _NL_ADDRESS_LANG_AB: unnamed_18 = 589833;
pub const _NL_NUM_LC_NUMERIC: unnamed_18 = 65542;
pub const _NL_WDAY_2: unnamed_18 = 131132;
pub const _NL_CTYPE_INDIGITS_WC_LEN: unnamed_18 = 30;
pub const _NL_WABMON_12: unnamed_18 = 131149;
pub const _NL_WABMON_2: unnamed_18 = 131139;
pub const D_FMT: unnamed_18 = 131113;
pub type unnamed_37 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const _NL_NAME_NAME_MR: unnamed_18 = 524290;
pub type size_t = libc::c_ulong;
pub const _NL_IDENTIFICATION_AUDIENCE: unnamed_18 = 786441;
#[no_mangle]
pub unsafe extern "C" fn areshell(mut shell: *const libc::c_char)
 -> libc::c_int {
    let mut progname: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    ptr = strrchr(shell, 47);
    if ptr != 0 as *mut libc::c_void as *const libc::c_char {
        ptr = ptr.offset(1isize)
    } else { ptr = shell }
    progname = getprogname();
    if *progname as libc::c_int == 45 { progname = progname.offset(1isize) }
    if strcmp(ptr, progname) == 0i32 { return 1i32 } else { return 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn setblocking(mut fd: libc::c_int,
                                     mut state: libc::c_int) -> () {
    let mut mode: libc::c_int = 0;
    mode = fcntl(fd, 3i32);
    if mode != 1i32.wrapping_neg() {
        if 0 == state { mode |= 2048i32 } else { mode &= !2048i32 }
        fcntl(fd, 4i32, mode);
    };
}
#[no_mangle]
pub unsafe extern "C" fn find_home() -> *const libc::c_char {
    let mut pw: *mut passwd = 0 as *mut passwd;
    static mut home: *const libc::c_char =
        unsafe { 0 as *const libc::c_char };
    if home != 0 as *mut libc::c_void as *const libc::c_char {
        return home
    } else {
        home = getenv(b"HOME\x00" as *const u8 as *const libc::c_char);
        if home == 0 as *mut libc::c_void as *const libc::c_char ||
               *home as libc::c_int == 0 {
            pw = getpwuid(getuid());
            if pw != 0 as *mut libc::c_void as *mut passwd {
                home = (*pw).pw_dir
            } else { home = 0 as *const libc::c_char }
        }
        return home
    };
}
unsafe extern "C" fn usage() -> ! {
    fprintf(stderr,
            b"usage: %s [-2CluvV] [-c shell-command] [-f file] [-L socket-name]\n            [-S socket-path] [command [flags]]\n\x00"
                as *const u8 as *const libc::c_char, getprogname());
    exit(1i32);
}
unsafe extern "C" fn make_label(mut label: *const libc::c_char,
                                mut cause: *mut *mut libc::c_char)
 -> *mut libc::c_char {
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut resolved: [libc::c_char; 4096] = [0; 4096];
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sb: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut uid: uid_t = 0;
    *cause = 0 as *mut libc::c_char;
    if label == 0 as *mut libc::c_void as *const libc::c_char {
        label = b"default\x00" as *const u8 as *const libc::c_char
    }
    uid = getuid();
    s = getenv(b"TMUX_TMPDIR\x00" as *const u8 as *const libc::c_char);
    if s != 0 as *mut libc::c_void as *mut libc::c_char &&
           *s as libc::c_int != 0 {
        xasprintf(&mut base as *mut *mut libc::c_char,
                  b"%s/tmux-%ld\x00" as *const u8 as *const libc::c_char, s,
                  uid as libc::c_long);
    } else {
        xasprintf(&mut base as *mut *mut libc::c_char,
                  b"%s/tmux-%ld\x00" as *const u8 as *const libc::c_char,
                  b"/tmp/\x00" as *const u8 as *const libc::c_char,
                  uid as libc::c_long);
    }
    if realpath(base, resolved.as_mut_ptr()) ==
           0 as *mut libc::c_void as *mut libc::c_char &&
           strlcpy(resolved.as_mut_ptr(), base,
                   ::std::mem::size_of::<[libc::c_char; 4096]>() as
                       libc::c_ulong) >=
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
       {
        *__errno_location() = 34i32;
        free(base as *mut libc::c_void);
    } else if !(mkdir(resolved.as_mut_ptr(),
                      (256i32 | 128i32 | 64i32) as __mode_t) != 0i32 &&
                    *__errno_location() != 17i32) {
        if !(lstat(resolved.as_mut_ptr(), &mut sb as *mut stat) != 0i32) {
            if 0 !=
                   !(sb.st_mode & 61440i32 as libc::c_uint ==
                         16384i32 as libc::c_uint) as libc::c_int {
                *__errno_location() = 20i32
            } else if sb.st_uid != uid ||
                          sb.st_mode &
                              ((256i32 | 128i32 | 64i32) >> 3i32 >> 3i32) as
                                  libc::c_uint != 0i32 as libc::c_uint {
                *__errno_location() = 13i32
            } else {
                xasprintf(&mut path as *mut *mut libc::c_char,
                          b"%s/%s\x00" as *const u8 as *const libc::c_char,
                          resolved.as_mut_ptr(), label);
                return path
            }
        }
    }
    xasprintf(cause,
              b"error creating %s (%s)\x00" as *const u8 as
                  *const libc::c_char, resolved.as_mut_ptr(),
              strerror(*__errno_location()));
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn getshell() -> *const libc::c_char {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    shell = getenv(b"SHELL\x00" as *const u8 as *const libc::c_char);
    if 0 != checkshell(shell) {
        return shell
    } else {
        pw = getpwuid(getuid());
        if pw != 0 as *mut libc::c_void as *mut passwd &&
               0 != checkshell((*pw).pw_shell) {
            return (*pw).pw_shell
        } else { return b"/bin/sh\x00" as *const u8 as *const libc::c_char }
    };
}
unsafe extern "C" fn checkshell(mut shell: *const libc::c_char)
 -> libc::c_int {
    if shell == 0 as *mut libc::c_void as *const libc::c_char ||
           *shell as libc::c_int != 47 {
        return 0i32
    } else if 0 != areshell(shell) {
        return 0i32
    } else if access(shell, 1i32) != 0i32 {
        return 0i32
    } else { return 1i32 };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    // Initialize self referential global statics when starting
    global_queue.tqh_last = &global_queue.tqh_first as *const *mut cmdq_item as *mut *mut cmdq_item;

    let mut current_block: u64;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut var: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp: [libc::c_char; 4096] = [0; 4096];
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    let mut cwd: *const libc::c_char = 0 as *const libc::c_char;
    let mut opt: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut keys: libc::c_int = 0;
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
    if setlocale(0i32, b"en_US.UTF-8\x00" as *const u8 as *const libc::c_char)
           == 0 as *mut libc::c_void as *mut libc::c_char &&
           setlocale(0i32, b"C.UTF-8\x00" as *const u8 as *const libc::c_char)
               == 0 as *mut libc::c_void as *mut libc::c_char {
        if setlocale(0i32, b"\x00" as *const u8 as *const libc::c_char) ==
               0 as *mut libc::c_void as *mut libc::c_char {
            errx(1i32,
                 b"invalid LC_ALL, LC_CTYPE or LANG\x00" as *const u8 as
                     *const libc::c_char);
        }
        s = nl_langinfo(CODESET as libc::c_int);
        if strcasecmp(s, b"UTF-8\x00" as *const u8 as *const libc::c_char) !=
               0i32 &&
               strcasecmp(s, b"UTF8\x00" as *const u8 as *const libc::c_char)
                   != 0i32 {
            errx(1i32,
                 b"need UTF-8 locale (LC_CTYPE) but have %s\x00" as *const u8
                     as *const libc::c_char, s);
        }
    }
    setlocale(2i32, b"\x00" as *const u8 as *const libc::c_char);
    tzset();
    if **argv as libc::c_int == 45 { flags = 2i32 } else { flags = 0i32 }
    path = 0 as *mut libc::c_char;
    label = path;
    loop  {
        opt =
            BSDgetopt(argc, argv,
                      b"2c:Cdf:lL:qS:uUVv\x00" as *const u8 as
                          *const libc::c_char);
        if opt != 1i32.wrapping_neg() {
            match opt {
                50 => { flags |= 131072i32 }
                99 => { shell_command = BSDoptarg }
                67 => {
                    if 0 != flags & 8192i32 {
                        flags |= 16384i32
                    } else { flags |= 8192i32 }
                }
                86 => {
                    printf(b"%s %s\n\x00" as *const u8 as *const libc::c_char,
                           getprogname(),
                           b"xmaster-rs\x00" as *const u8 as
                               *const libc::c_char);
                    exit(0i32);
                }
                102 => { set_cfg_file(BSDoptarg); }
                108 => { flags |= 2i32 }
                76 => {
                    free(label as *mut libc::c_void);
                    label = xstrdup(BSDoptarg)
                }
                113 => { }
                83 => {
                    free(path as *mut libc::c_void);
                    path = xstrdup(BSDoptarg)
                }
                117 => { flags |= 65536i32 }
                118 => { log_add_level(); }
                _ => { usage(); }
            }
        } else {
            argc -= BSDoptind;
            argv = argv.offset(BSDoptind as isize);
            if shell_command != 0 as *mut libc::c_void as *const libc::c_char
                   && argc != 0i32 {
                current_block = 13586036798005543211;
                break ;
            } else { current_block = 11050875288958768710; break ; }
        }
    }
    match current_block {
        13586036798005543211 => { usage(); }
        _ => {
            ptm_fd = getptmfd();
            if ptm_fd == 1i32.wrapping_neg() {
                err(1i32,
                    b"getptmfd\x00" as *const u8 as *const libc::c_char);
            }
            if 0i32 != 0i32 {
                err(1i32, b"pledge\x00" as *const u8 as *const libc::c_char);
            }
            if getenv(b"TMUX\x00" as *const u8 as *const libc::c_char) !=
                   0 as *mut libc::c_void as *mut libc::c_char {
                flags |= 65536i32
            } else {
                s = getenv(b"LC_ALL\x00" as *const u8 as *const libc::c_char);
                if s == 0 as *mut libc::c_void as *const libc::c_char ||
                       *s as libc::c_int == 0 {
                    s =
                        getenv(b"LC_CTYPE\x00" as *const u8 as
                                   *const libc::c_char)
                }
                if s == 0 as *mut libc::c_void as *const libc::c_char ||
                       *s as libc::c_int == 0 {
                    s =
                        getenv(b"LANG\x00" as *const u8 as
                                   *const libc::c_char)
                }
                if s == 0 as *mut libc::c_void as *const libc::c_char ||
                       *s as libc::c_int == 0 {
                    s = b"\x00" as *const u8 as *const libc::c_char
                }
                if strcasestr(s,
                              b"UTF-8\x00" as *const u8 as
                                  *const libc::c_char) !=
                       0 as *mut libc::c_void as *mut libc::c_char ||
                       strcasestr(s,
                                  b"UTF8\x00" as *const u8 as
                                      *const libc::c_char) !=
                           0 as *mut libc::c_void as *mut libc::c_char {
                    flags |= 65536i32
                }
            }
            global_hooks = hooks_create(0 as *mut hooks::hooks);
            global_environ = environ_create();
            var = environ;
            while *var != 0 as *mut libc::c_void as *mut libc::c_char {
                environ_put(global_environ, *var);
                var = var.offset(1isize)
            }
            cwd = getenv(b"PWD\x00" as *const u8 as *const libc::c_char);
            if cwd == 0 as *mut libc::c_void as *const libc::c_char &&
                   {
                       cwd =
                           getcwd(tmp.as_mut_ptr(),
                                  ::std::mem::size_of::<[libc::c_char; 4096]>()
                                      as libc::c_ulong);
                       cwd != 0 as *mut libc::c_void as *const libc::c_char
                   } {
                environ_set(global_environ,
                            b"PWD\x00" as *const u8 as *const libc::c_char,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            cwd);
            }
            global_options = options_create(0 as *mut options::options);
            global_s_options = options_create(0 as *mut options::options);
            global_w_options = options_create(0 as *mut options::options);
            oe = options_table.as_ptr();
            while (*oe).name != 0 as *mut libc::c_void as *const libc::c_char
                  {
                if (*oe).scope as libc::c_uint ==
                       OPTIONS_TABLE_SERVER as libc::c_int as libc::c_uint {
                    options_default(global_options, oe);
                }
                if (*oe).scope as libc::c_uint ==
                       OPTIONS_TABLE_SESSION as libc::c_int as libc::c_uint {
                    options_default(global_s_options, oe);
                }
                if (*oe).scope as libc::c_uint ==
                       OPTIONS_TABLE_WINDOW as libc::c_int as libc::c_uint {
                    options_default(global_w_options, oe);
                }
                oe = oe.offset(1isize)
            }
            shell = getshell();
            options_set_string(global_s_options,
                               b"default-shell\x00" as *const u8 as
                                   *const libc::c_char, 0i32,
                               b"%s\x00" as *const u8 as *const libc::c_char,
                               shell);
            s = getenv(b"VISUAL\x00" as *const u8 as *const libc::c_char);
            if s != 0 as *mut libc::c_void as *const libc::c_char ||
                   {
                       s =
                           getenv(b"EDITOR\x00" as *const u8 as
                                      *const libc::c_char);
                       s != 0 as *mut libc::c_void as *const libc::c_char
                   } {
                if strrchr(s, 47) !=
                       0 as *mut libc::c_void as *mut libc::c_char {
                    s = strrchr(s, 47).offset(1isize)
                }
                if strstr(s, b"vi\x00" as *const u8 as *const libc::c_char) !=
                       0 as *mut libc::c_void as *mut libc::c_char {
                    keys = 1i32
                } else { keys = 0i32 }
                options_set_number(global_s_options,
                                   b"status-keys\x00" as *const u8 as
                                       *const libc::c_char,
                                   keys as libc::c_longlong);
                options_set_number(global_w_options,
                                   b"mode-keys\x00" as *const u8 as
                                       *const libc::c_char,
                                   keys as libc::c_longlong);
            }
            if path == 0 as *mut libc::c_void as *mut libc::c_char &&
                   label == 0 as *mut libc::c_void as *mut libc::c_char {
                s = getenv(b"TMUX\x00" as *const u8 as *const libc::c_char);
                if s != 0 as *mut libc::c_void as *const libc::c_char &&
                       *s as libc::c_int != 0 && *s as libc::c_int != 44 {
                    path = xstrdup(s);
                    *path.offset(strcspn(path,
                                         b",\x00" as *const u8 as
                                             *const libc::c_char) as isize) =
                        0 as libc::c_char
                }
            }
            if path == 0 as *mut libc::c_void as *mut libc::c_char &&
                   {
                       path =
                           make_label(label,
                                      &mut cause as *mut *mut libc::c_char);
                       path == 0 as *mut libc::c_void as *mut libc::c_char
                   } {
                if cause != 0 as *mut libc::c_void as *mut libc::c_char {
                    fprintf(stderr,
                            b"%s\n\x00" as *const u8 as *const libc::c_char,
                            cause);
                    free(cause as *mut libc::c_void);
                }
                exit(1i32);
            } else {
                socket_path = path;
                free(label as *mut libc::c_void);
                exit(client_main(osdep_event_init(), argc, argv, flags));
            }
        }
    };
}
fn main() -> () {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}

