extern crate libc;

use arguments::{args, args_has, args_get};
use common::timeval;
use client::{client, clients};
use cmd::{cmd, unnamed_36 as unnamed_16, cmd_entry, cmd_entry_flag};
use cmd_find::{cmd_find_target, cmd_find_state};
use cmd_queue::{cmdq_item};
use environ::{environ, environ_update};
use session::{session, sessions};
use options::options;
use proc_::proc_send;
use window::{winlink, window, windows, window_pane, window_pane_tree, window_mode};

extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type screen_titles;
    pub type tty_code;
    pub type hooks;
    pub type format_job_tree;
    pub type tmuxproc;
    pub type bufferevent_ops;
    pub type format_tree;
    pub type _IO_FILE_plus;
    pub type input_ctx;
    pub type args_entry;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
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
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
     -> libc::c_int;
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
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn format_single(_: *mut cmdq_item, _: *const libc::c_char,
                     _: *mut client, _: *mut session, _: *mut winlink,
                     _: *mut window_pane) -> *mut libc::c_char;
    #[no_mangle]
    fn notify_client(_: *const libc::c_char, _: *mut client) -> ();
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn cmd_find_from_winlink(_: *mut cmd_find_state, _: *mut winlink,
                             _: libc::c_int) -> ();
    #[no_mangle]
    fn cmd_find_from_winlink_pane(_: *mut cmd_find_state, _: *mut winlink,
                                  _: *mut window_pane, _: libc::c_int) -> ();
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn server_update_socket() -> ();
    #[no_mangle]
    fn alerts_check_session(_: *mut session) -> ();
    #[no_mangle]
    fn recalculate_sizes() -> ();
    #[no_mangle]
    fn server_redraw_client(_: *mut client) -> ();
    #[no_mangle]
    fn session_update_activity(_: *mut session, _: *mut timeval) -> ();
    #[no_mangle]
    fn status_timer_start(_: *mut client) -> ();
    #[no_mangle]
    fn server_client_set_key_table(_: *mut client, _: *const libc::c_char)
     -> ();
    #[no_mangle]
    fn server_client_detach(_: *mut client, _: msgtype) -> ();
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn server_client_open(_: *mut client, _: *mut *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn session_set_current(_: *mut session, _: *mut winlink) -> libc::c_int;
    #[no_mangle]
    fn window_set_active_pane(_: *mut window, _: *mut window_pane)
     -> libc::c_int;
    #[no_mangle]
    fn server_client_check_nested(_: *mut client) -> libc::c_int;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    static grid_default_cell: grid_cell;
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
    static mut session_groups: session_groups;
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type pid_t = __pid_t;
pub type __u_char = libc::c_uchar;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const LINE_SEL_LEFT_RIGHT: unnamed_13 = 1;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub const MSG_IDENTIFY_TTYNAME: msgtype = 102;
pub type __timezone_ptr_t = *mut timezone;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const MSG_VERSION: msgtype = 12;
pub const MSG_COMMAND: msgtype = 200;
pub const MSG_SHELL: msgtype = 209;
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const TTY_VT220: unnamed_26 = 3;
pub const TTY_VT102: unnamed_26 = 2;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_11,
    pub entry: unnamed_4,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const MSG_EXITING: msgtype = 205;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_13,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    offset: u_int,
    data: unnamed_9,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const MSG_IDENTIFY_FLAGS: msgtype = 100;
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_30,
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
    pub entry: unnamed_35,
}
pub const MSG_IDENTIFY_TERM: msgtype = 101;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_3 {
    ev_io: unnamed_38,
    ev_signal: unnamed_27,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type __pid_t = libc::c_int;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_2,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const MSG_RESIZE: msgtype = 208;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const MSG_READY: msgtype = 207;
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
pub const TTY_VT320: unnamed_26 = 4;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub type msgtype = libc::c_uint;
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
pub struct unnamed_9 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const MSG_DETACHKILL: msgtype = 202;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const MSG_EXIT: msgtype = 203;
pub type __u_short = libc::c_ushort;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const MSG_EXITED: msgtype = 204;
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
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_24,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_25,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type unnamed_13 = libc::c_uint;
pub const PROMPT_ENTRY: unnamed_23 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type u_char = __u_char;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type u_int = __u_int;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const MSG_DETACH: msgtype = 201;
pub const MSG_STDERR: msgtype = 211;
pub const MSG_IDENTIFY_CWD: msgtype = 108;
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
pub const TTY_VT101: unnamed_26 = 1;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const JOB_DEAD: unnamed_30 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const MSG_STDIN: msgtype = 212;
pub type options_table_type = libc::c_uint;
pub const MSG_EXEC: msgtype = 217;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const JOB_CLOSED: unnamed_30 = 2;
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
pub const MSG_LOCK: msgtype = 206;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub type unnamed_23 = libc::c_uint;
pub const TTY_VT100: unnamed_26 = 0;
pub type speed_t = libc::c_uint;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const MSG_IDENTIFY_STDIN: msgtype = 104;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed,
}
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub type options_table_scope = libc::c_uint;
pub const MSG_IDENTIFY_DONE: msgtype = 106;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_6,
}
pub const MSG_SHUTDOWN: msgtype = 210;
pub const TTY_VT420: unnamed_26 = 5;
pub const JOB_RUNNING: unnamed_30 = 0;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_25 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const LINE_SEL_NONE: unnamed_13 = 0;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const MSG_IDENTIFY_OLDCWD: msgtype = 103;
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
    pub entry: unnamed_19,
}
pub const MSG_WAKEUP: msgtype = 216;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub type unnamed_26 = libc::c_uint;
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_1,
}
pub const PROMPT_COMMAND: unnamed_23 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub ev_signal_next: unnamed_12,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const LINE_SEL_RIGHT_LEFT: unnamed_13 = 2;
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type time_t = __time_t;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_15,
    pub ev_next: unnamed_17,
    pub ev_timeout_pos: unnamed_32,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_3,
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
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub type unnamed_30 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type tcflag_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub const MSG_UNLOCK: msgtype = 215;
pub const TTY_UNKNOWN: unnamed_26 = 6;
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
    pub term_type: unnamed_26,
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
pub union unnamed_32 {
    ev_next_with_common_timeout: unnamed_31,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub const MSG_IDENTIFY_ENVIRON: msgtype = 105;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const MSG_SUSPEND: msgtype = 214;
pub const MSG_IDENTIFY_CLIENTPID: msgtype = 107;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub type bitstr_t = libc::c_uchar;
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub ev_io_next: unnamed_29,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_7,
}
pub const MSG_STDOUT: msgtype = 213;
pub type cmd_find_type = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn cmd_attach_session(mut item: *mut cmdq_item,
                                            mut tflag: *const libc::c_char,
                                            mut dflag: libc::c_int,
                                            mut rflag: libc::c_int,
                                            mut cflag: *const libc::c_char,
                                            mut Eflag: libc::c_int)
 -> cmd_retval {
    let mut current: *mut cmd_find_state =
        &mut (*(*item).shared).current as *mut cmd_find_state;
    let mut type_0: cmd_find_type = CMD_FIND_PANE;
    let mut flags: libc::c_int = 0;
    let mut c: *mut client = (*item).client;
    let mut c_loop: *mut client = 0 as *mut client;
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*(&mut sessions as *mut sessions)).rbh_root ==
           0 as *mut libc::c_void as *mut session {
        cmdq_error(item,
                   b"no sessions\x00" as *const u8 as *const libc::c_char);
        return CMD_RETURN_ERROR
    } else if c == 0 as *mut libc::c_void as *mut client {
        return CMD_RETURN_NORMAL
    } else if 0 != server_client_check_nested(c) {
        cmdq_error(item,
                   b"sessions should be nested with care, unset $TMUX to force\x00"
                       as *const u8 as *const libc::c_char);
        return CMD_RETURN_ERROR
    } else {
        if tflag != 0 as *mut libc::c_void as *const libc::c_char &&
               *tflag.offset(strcspn(tflag,
                                     b":.\x00" as *const u8 as
                                         *const libc::c_char) as isize) as
                   libc::c_int != 0 {
            type_0 = CMD_FIND_PANE;
            flags = 0i32
        } else { type_0 = CMD_FIND_SESSION; flags = 1i32 }
        if cmd_find_target(&mut (*item).target as *mut cmd_find_state, item,
                           tflag, type_0, flags) != 0i32 {
            return CMD_RETURN_ERROR
        } else {
            s = (*item).target.s;
            wl = (*item).target.wl;
            wp = (*item).target.wp;
            if wl != 0 as *mut libc::c_void as *mut winlink {
                if wp != 0 as *mut libc::c_void as *mut window_pane {
                    window_set_active_pane((*wp).window, wp);
                }
                session_set_current(s, wl);
                if wp != 0 as *mut libc::c_void as *mut window_pane {
                    cmd_find_from_winlink_pane(current, wl, wp, 0i32);
                } else { cmd_find_from_winlink(current, wl, 0i32); }
            }
            if cflag != 0 as *mut libc::c_void as *const libc::c_char {
                free((*s).cwd as *mut libc::c_void);
                (*s).cwd = format_single(item, cflag, c, s, wl, wp)
            }
            if (*c).session != 0 as *mut libc::c_void as *mut session {
                if 0 != dflag {
                    c_loop = (*(&mut clients as *mut clients)).tqh_first;
                    while c_loop != 0 as *mut libc::c_void as *mut client {
                        if !((*c_loop).session != s || c == c_loop) {
                            server_client_detach(c_loop, MSG_DETACH);
                        }
                        c_loop = (*c_loop).entry.tqe_next
                    }
                }
                if 0 == Eflag {
                    environ_update((*s).options, (*c).environ, (*s).environ);
                }
                (*c).session = s;
                if 0 != !(*(*item).shared).flags & 1i32 {
                    server_client_set_key_table(c, 0 as *const libc::c_char);
                }
                status_timer_start(c);
                notify_client(b"client-session-changed\x00" as *const u8 as
                                  *const libc::c_char, c);
                session_update_activity(s, 0 as *mut timeval);
                gettimeofday(&mut (*s).last_attached_time as *mut timeval,
                             0 as *mut timezone);
                server_redraw_client(c);
                (*(*s).curw).flags &= !(1i32 | 2i32 | 4i32)
            } else if server_client_open(c,
                                         &mut cause as *mut *mut libc::c_char)
                          != 0i32 {
                cmdq_error(item,
                           b"open terminal failed: %s\x00" as *const u8 as
                               *const libc::c_char, cause);
                free(cause as *mut libc::c_void);
                return CMD_RETURN_ERROR
            } else {
                if 0 != rflag { (*c).flags |= 2048i32 }
                if 0 != dflag {
                    c_loop = (*(&mut clients as *mut clients)).tqh_first;
                    while c_loop != 0 as *mut libc::c_void as *mut client {
                        if !((*c_loop).session != s || c == c_loop) {
                            server_client_detach(c_loop, MSG_DETACH);
                        }
                        c_loop = (*c_loop).entry.tqe_next
                    }
                }
                if 0 == Eflag {
                    environ_update((*s).options, (*c).environ, (*s).environ);
                }
                (*c).session = s;
                server_client_set_key_table(c, 0 as *const libc::c_char);
                status_timer_start(c);
                notify_client(b"client-session-changed\x00" as *const u8 as
                                  *const libc::c_char, c);
                session_update_activity(s, 0 as *mut timeval);
                gettimeofday(&mut (*s).last_attached_time as *mut timeval,
                             0 as *mut timezone);
                server_redraw_client(c);
                (*(*s).curw).flags &= !(1i32 | 2i32 | 4i32);
                if 0 != !(*c).flags & 8192i32 {
                    proc_send((*c).peer, MSG_READY, 1i32.wrapping_neg(),
                              0 as *const libc::c_void, 0i32 as size_t);
                }
                notify_client(b"client-attached\x00" as *const u8 as
                                  *const libc::c_char, c);
                (*c).flags |= 128i32
            }
            recalculate_sizes();
            alerts_check_session(s);
            server_update_socket();
            return CMD_RETURN_NORMAL
        }
    };
}
unsafe extern "C" fn cmd_attach_session_exec(mut self_0: *mut cmd,
                                             mut item: *mut cmdq_item)
 -> cmd_retval {
    let mut args: *mut args = (*self_0).args;
    return cmd_attach_session(item, args_get(args, 116 as u_char),
                              args_has(args, 100 as u_char),
                              args_has(args, 114 as u_char),
                              args_get(args, 99 as u_char),
                              args_has(args, 69 as u_char));
}
#[no_mangle]
pub static mut cmd_attach_session_entry: cmd_entry =
    unsafe {
        cmd_entry{name:
                      b"attach-session\x00" as *const u8 as
                          *const libc::c_char,
                  alias: b"attach\x00" as *const u8 as *const libc::c_char,
                  args:
                      unnamed_16{template:
                                     b"c:dErt:\x00" as *const u8 as
                                         *const libc::c_char,
                                 lower: 0i32,
                                 upper: 0i32,},
                  usage:
                      b"[-dEr] [-c working-directory] [-t target-session]\x00"
                          as *const u8 as *const libc::c_char,
                  source:
                      cmd_entry_flag{flag: 0,
                                     type_0: CMD_FIND_PANE,
                                     flags: 0,},
                  target:
                      cmd_entry_flag{flag: 0,
                                     type_0: CMD_FIND_PANE,
                                     flags: 0,},
                  flags: 1i32,
                  exec: Some(cmd_attach_session_exec),}
    };

