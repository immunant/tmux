extern crate libc;

use client::{client};
use cmd::{mouse_event};
use cmd_find::{cmd_find_state};
use hooks::hooks;
use proc_::evbuffer;
use session::{session, session_group, session_groups};
use window::{window, winlink, window_pane};

extern "C" {
    pub type args_entry;
    pub type format_job_tree;
    pub type event_base;
    pub type format_tree;
    pub type environ;
    pub type tmuxpeer;
    pub type input_ctx;
    pub type _IO_FILE_plus;
    pub type bufferevent_ops;
    pub type tmuxproc;
    pub type screen_titles;
    pub type options;
    pub type tty_code;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    static mut __tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut __daylight: libc::c_int;
    #[no_mangle]
    static mut __timezone: libc::c_long;
    #[no_mangle]
    static mut tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut daylight: libc::c_int;
    #[no_mangle]
    static mut timezone: libc::c_long;
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
    fn evbuffer_add_printf(buf: *mut evbuffer, fmt: *const libc::c_char, ...)
     -> libc::c_int;
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
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, ...)
     -> libc::c_int;
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
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn format_free(_: *mut format_tree) -> ();
    #[no_mangle]
    fn hooks_insert(_: *mut hooks, _: *mut cmdq_item, _: *mut cmd_find_state,
                    _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_find_target(_: *mut cmd_find_state, _: *mut cmdq_item,
                       _: *const libc::c_char, _: cmd_find_type,
                       _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_client(_: *mut cmdq_item, _: *const libc::c_char,
                       _: libc::c_int) -> *mut client;
    #[no_mangle]
    fn cmd_find_clear_state(_: *mut cmd_find_state, _: libc::c_int) -> ();
    #[no_mangle]
    fn cmd_find_valid_state(_: *mut cmd_find_state) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_copy_state(_: *mut cmd_find_state, _: *mut cmd_find_state)
     -> ();
    #[no_mangle]
    fn cmd_find_from_client(_: *mut cmd_find_state, _: *mut client,
                            _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn cmd_list_free(_: *mut cmd_list) -> ();
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn server_client_unref(_: *mut client) -> ();
    #[no_mangle]
    fn server_client_push_stdout(_: *mut client) -> ();
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut clients: clients;
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
    static mut sessions: sessions;
    #[no_mangle]
    static mut session_groups: session_groups;
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub type cmd_find_type = libc::c_uint;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type time_t = __time_t;
pub type __suseconds_t = libc::c_long;
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
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_5,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub type u_int = __u_int;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
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
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub type unnamed_4 = libc::c_uint;
pub const TTY_VT320: unnamed_23 = 4;
pub const LINE_SEL_NONE: unnamed_13 = 0;
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
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const JOB_RUNNING: unnamed_4 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_9,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const JOB_DEAD: unnamed_4 = 1;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub type __u_int = libc::c_uint;
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
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_29,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_19,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_12 {
    ev_next_with_common_timeout: unnamed_6,
    min_heap_idx: libc::c_int,
}
pub type __off_t = libc::c_long;
pub type uint16_t = libc::c_ushort;
pub type cmd_retval = libc::c_int;
pub type unnamed_13 = libc::c_uint;
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
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_22,
    pub ev_next: unnamed_38,
    pub ev_timeout_pos: unnamed_12,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_24,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub type pid_t = __pid_t;
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_11,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_13 = 2;
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type u_char = __u_char;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_19 {
    offset: u_int,
    data: unnamed_14,
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
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub ev_signal_next: unnamed,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub type options_table_type = libc::c_uint;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
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
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const TTY_VT100: unnamed_23 = 0;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub type unnamed_23 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_15,
}
pub type cmdq_type = libc::c_uint;
pub const JOB_CLOSED: unnamed_4 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_24 {
    ev_io: unnamed_32,
    ev_signal: unnamed_20,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_13 = 1;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type u_short = __u_short;
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
    pub term_type: unnamed_23,
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
pub struct unnamed_25 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
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
    pub entry: unnamed_16,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_36,
}
pub type uint8_t = libc::c_uchar;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type unnamed_26 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTY_UNKNOWN: unnamed_23 = 6;
pub const TTY_VT101: unnamed_23 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_4,
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
    pub entry: unnamed_1,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const TTY_VT220: unnamed_23 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub type __pid_t = libc::c_int;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
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
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub ev_io_next: unnamed_27,
    pub ev_timeout: timeval,
}
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_34,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
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
pub const TTY_VT420: unnamed_23 = 5;
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
    pub entry: unnamed_2,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_34 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const TTY_VT102: unnamed_23 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const PROMPT_COMMAND: unnamed_26 = 1;
pub type tcflag_t = libc::c_uint;
pub const PROMPT_ENTRY: unnamed_26 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_8,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub type __u_char = libc::c_uchar;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_command(mut cmdlist: *mut cmd_list,
                                          mut current: *mut cmd_find_state,
                                          mut m: *mut mouse_event,
                                          mut flags: libc::c_int)
 -> *mut cmdq_item {
    let mut item: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut first: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut last: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut cmd: *mut cmd = 0 as *mut cmd;
    let mut group: u_int = cmdq_next_group();
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shared: *mut cmdq_shared = 0 as *mut cmdq_shared;
    shared =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<cmdq_shared>() as libc::c_ulong) as
            *mut cmdq_shared;
    if current != 0 as *mut libc::c_void as *mut cmd_find_state {
        cmd_find_copy_state(&mut (*shared).current as *mut cmd_find_state,
                            current);
    } else {
        cmd_find_clear_state(&mut (*shared).current as *mut cmd_find_state,
                             0i32);
    }
    if m != 0 as *mut libc::c_void as *mut mouse_event {
        memcpy(&mut (*shared).mouse as *mut mouse_event as *mut libc::c_void,
               m as *const libc::c_void,
               ::std::mem::size_of::<mouse_event>() as libc::c_ulong);
    }
    cmd = (*(&mut (*cmdlist).list as *mut unnamed_29)).tqh_first;
    while cmd != 0 as *mut libc::c_void as *mut cmd {
        xasprintf(&mut tmp as *mut *mut libc::c_char,
                  b"command[%s]\x00" as *const u8 as *const libc::c_char,
                  (*(*cmd).entry).name);
        item =
            xcalloc(1i32 as size_t,
                    ::std::mem::size_of::<cmdq_item>() as libc::c_ulong) as
                *mut cmdq_item;
        (*item).name = tmp;
        (*item).type_0 = CMDQ_COMMAND;
        (*item).group = group;
        (*item).flags = flags;
        (*item).shared = shared;
        (*item).cmdlist = cmdlist;
        (*item).cmd = cmd;
        (*shared).references += 1;
        (*cmdlist).references += 1;
        if first == 0 as *mut libc::c_void as *mut cmdq_item { first = item }
        if last != 0 as *mut libc::c_void as *mut cmdq_item {
            (*last).next = item
        }
        last = item;
        cmd = (*cmd).qentry.tqe_next
    }
    return first;
}
unsafe extern "C" fn cmdq_next_group() -> u_int {
    static mut group: u_int = unsafe { 0 };
    group = group.wrapping_add(1);
    return group;
}
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_callback1(mut name: *const libc::c_char,
                                            mut cb: cmdq_cb,
                                            mut data: *mut libc::c_void)
 -> *mut cmdq_item {
    let mut item: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    xasprintf(&mut tmp as *mut *mut libc::c_char,
              b"callback[%s]\x00" as *const u8 as *const libc::c_char, name);
    item =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<cmdq_item>() as libc::c_ulong) as
            *mut cmdq_item;
    (*item).name = tmp;
    (*item).type_0 = CMDQ_CALLBACK;
    (*item).group = 0i32 as u_int;
    (*item).flags = 0i32;
    (*item).cb = cb;
    (*item).data = data;
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn cmdq_insert_after(mut after: *mut cmdq_item,
                                           mut item: *mut cmdq_item) -> () {
    let mut current_block: u64;
    let mut c: *mut client = (*after).client;
    let mut queue: *mut cmdq_list = (*after).queue;
    let mut next: *mut cmdq_item = 0 as *mut cmdq_item;
    loop  {
        next = (*item).next;
        (*item).next = 0 as *mut cmdq_item;
        if c != 0 as *mut libc::c_void as *mut client { (*c).references += 1 }
        (*item).client = c;
        (*item).queue = queue;
        if (*after).next != 0 as *mut libc::c_void as *mut cmdq_item {
            current_block = 735147466149431745;
        } else { current_block = 4906268039856690917; }
        loop  {
            match current_block {
                4906268039856690917 => {
                    (*item).entry.tqe_next = (*after).entry.tqe_next;
                    if (*item).entry.tqe_next !=
                           0 as *mut libc::c_void as *mut cmdq_item {
                        (*(*item).entry.tqe_next).entry.tqe_prev =
                            &mut (*item).entry.tqe_next as *mut *mut cmdq_item
                    } else {
                        (*queue).tqh_last =
                            &mut (*item).entry.tqe_next as *mut *mut cmdq_item
                    }
                    (*after).entry.tqe_next = item;
                    (*item).entry.tqe_prev =
                        &mut (*after).entry.tqe_next as *mut *mut cmdq_item;
                    if 0 != 0i32 {
                        current_block = 4906268039856690917;
                    } else { break ; }
                }
                _ => {
                    (*item).entry.tqe_next = (*(*after).next).entry.tqe_next;
                    if (*item).entry.tqe_next !=
                           0 as *mut libc::c_void as *mut cmdq_item {
                        (*(*item).entry.tqe_next).entry.tqe_prev =
                            &mut (*item).entry.tqe_next as *mut *mut cmdq_item
                    } else {
                        (*queue).tqh_last =
                            &mut (*item).entry.tqe_next as *mut *mut cmdq_item
                    }
                    (*(*after).next).entry.tqe_next = item;
                    (*item).entry.tqe_prev =
                        &mut (*(*after).next).entry.tqe_next as
                            *mut *mut cmdq_item;
                    if 0 != 0i32 {
                        current_block = 735147466149431745;
                    } else { break ; }
                }
            }
        }
        (*after).next = item;
        item = next;
        if !(item != 0 as *mut libc::c_void as *mut cmdq_item) { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmdq_append(mut c: *mut client,
                                     mut item: *mut cmdq_item) -> () {
    let mut queue: *mut cmdq_list = cmdq_get(c);
    let mut next: *mut cmdq_item = 0 as *mut cmdq_item;
    loop  {
        next = (*item).next;
        (*item).next = 0 as *mut cmdq_item;
        if c != 0 as *mut libc::c_void as *mut client { (*c).references += 1 }
        (*item).client = c;
        (*item).queue = queue;
        loop  {
            (*item).entry.tqe_next = 0 as *mut cmdq_item;
            (*item).entry.tqe_prev = (*queue).tqh_last;
            *(*queue).tqh_last = item;
            (*queue).tqh_last =
                &mut (*item).entry.tqe_next as *mut *mut cmdq_item;
            if !(0 != 0i32) { break ; }
        }
        item = next;
        if !(item != 0 as *mut libc::c_void as *mut cmdq_item) { break ; }
    };
}
unsafe extern "C" fn cmdq_get(mut c: *mut client) -> *mut cmdq_list {
    if c.is_null() {
        return &mut global_queue as *mut cmdq_list
    } else { return &mut (*c).queue as *mut cmdq_list };
}
// Original:
// static mut global_queue: cmdq_list =
//     unsafe {
//         cmdq_list{
//             tqh_first: 0 as *const cmdq_item as *mut cmdq_item,
//             tqh_last: &global_queue.tqh_first as *const *mut cmdq_item as *mut *mut cmdq_item,
//         }
//     };
pub static mut global_queue: cmdq_list =
    unsafe {
        cmdq_list{
            tqh_first: 0 as *const cmdq_item as *mut cmdq_item,
            tqh_last: 0 as *const *mut cmdq_item as *mut *mut cmdq_item, // Initializing in main
        }
    };
#[no_mangle]
pub unsafe extern "C" fn cmdq_next(mut c: *mut client) -> u_int {
    let mut queue: *mut cmdq_list = cmdq_get(c);
    let mut name: *const libc::c_char = cmdq_name(c);
    let mut item: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut retval: cmd_retval = CMD_RETURN_NORMAL;
    let mut items: u_int = 0i32 as u_int;
    static mut number: u_int = unsafe { 0 };
    if (*queue).tqh_first == 0 as *mut libc::c_void as *mut cmdq_item {
        log_debug(b"%s %s: empty\x00" as *const u8 as *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 10],
                                            &[libc::c_char; 10]>(b"cmdq_next\x00")).as_ptr(),
                  name);
        return 0i32 as u_int
    } else if 0 != (*(*queue).tqh_first).flags & 2i32 {
        log_debug(b"%s %s: waiting\x00" as *const u8 as *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 10],
                                            &[libc::c_char; 10]>(b"cmdq_next\x00")).as_ptr(),
                  name);
        return 0i32 as u_int
    } else {
        log_debug(b"%s %s: enter\x00" as *const u8 as *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 10],
                                            &[libc::c_char; 10]>(b"cmdq_next\x00")).as_ptr(),
                  name);
        loop  {
            item = (*queue).tqh_first;
            if item == 0 as *mut libc::c_void as *mut cmdq_item {
                log_debug(b"%s %s: exit (empty)\x00" as *const u8 as
                              *const libc::c_char,
                          (*::std::mem::transmute::<&[u8; 10],
                                                    &[libc::c_char; 10]>(b"cmdq_next\x00")).as_ptr(),
                          name);
                return items
            } else {
                log_debug(b"%s %s: %s (%d), flags %x\x00" as *const u8 as
                              *const libc::c_char,
                          (*::std::mem::transmute::<&[u8; 10],
                                                    &[libc::c_char; 10]>(b"cmdq_next\x00")).as_ptr(),
                          name, (*item).name, (*item).type_0 as libc::c_uint,
                          (*item).flags);
                if 0 != (*item).flags & 2i32 { break ; }
                if 0 != !(*item).flags & 1i32 {
                    (*item).time = time(0 as *mut time_t);
                    number = number.wrapping_add(1);
                    (*item).number = number;
                    match (*item).type_0 as libc::c_uint {
                        0 => {
                            retval = cmdq_fire_command(item);
                            if retval as libc::c_int ==
                                   CMD_RETURN_ERROR as libc::c_int {
                                cmdq_remove_group(item);
                            }
                        }
                        1 => { retval = cmdq_fire_callback(item) }
                        _ => { retval = CMD_RETURN_ERROR }
                    }
                    (*item).flags |= 1i32;
                    if retval as libc::c_int == CMD_RETURN_WAIT as libc::c_int
                       {
                        (*item).flags |= 2i32;
                        break ;
                    } else { items = items.wrapping_add(1) }
                }
                cmdq_remove(item);
            }
        }
        log_debug(b"%s %s: exit (wait)\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 10],
                                            &[libc::c_char; 10]>(b"cmdq_next\x00")).as_ptr(),
                  name);
        return items
    };
}
unsafe extern "C" fn cmdq_name(mut c: *mut client) -> *const libc::c_char {
    static mut s: [libc::c_char; 32] = unsafe { [0; 32] };
    if c == 0 as *mut libc::c_void as *mut client {
        return b"<global>\x00" as *const u8 as *const libc::c_char
    } else {
        xsnprintf(s.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong,
                  b"<%p>\x00" as *const u8 as *const libc::c_char, c);
        return s.as_mut_ptr()
    };
}
unsafe extern "C" fn cmdq_remove(mut item: *mut cmdq_item) -> () {
    if (*item).shared != 0 as *mut libc::c_void as *mut cmdq_shared &&
           {
               (*(*item).shared).references -= 1;
               (*(*item).shared).references == 0i32
           } {
        if (*(*item).shared).formats !=
               0 as *mut libc::c_void as *mut format_tree {
            format_free((*(*item).shared).formats);
        }
        free((*item).shared as *mut libc::c_void);
    }
    if (*item).client != 0 as *mut libc::c_void as *mut client {
        server_client_unref((*item).client);
    }
    if (*item).type_0 as libc::c_uint ==
           CMDQ_COMMAND as libc::c_int as libc::c_uint {
        cmd_list_free((*item).cmdlist);
    }
    loop  {
        if (*item).entry.tqe_next != 0 as *mut libc::c_void as *mut cmdq_item
           {
            (*(*item).entry.tqe_next).entry.tqe_prev = (*item).entry.tqe_prev
        } else { (*(*item).queue).tqh_last = (*item).entry.tqe_prev }
        *(*item).entry.tqe_prev = (*item).entry.tqe_next;
        if !(0 != 0i32) { break ; }
    }
    free((*item).name as *mut libc::c_void);
    free(item as *mut libc::c_void);
}
unsafe extern "C" fn cmdq_fire_callback(mut item: *mut cmdq_item)
 -> cmd_retval {
    return (*item).cb.expect("non-null function pointer")(item, (*item).data);
}
unsafe extern "C" fn cmdq_remove_group(mut item: *mut cmdq_item) -> () {
    let mut this: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut next: *mut cmdq_item = 0 as *mut cmdq_item;
    this = (*item).entry.tqe_next;
    while this != 0 as *mut libc::c_void as *mut cmdq_item {
        next = (*this).entry.tqe_next;
        if (*this).group == (*item).group { cmdq_remove(this); }
        this = next
    };
}
unsafe extern "C" fn cmdq_fire_command(mut item: *mut cmdq_item)
 -> cmd_retval {
    let mut current_block: u64;
    let mut c: *mut client = (*item).client;
    let mut cmd: *mut cmd = (*item).cmd;
    let mut entry: *const cmd_entry = (*cmd).entry;
    let mut retval: cmd_retval = CMD_RETURN_NORMAL;
    let mut fsp: *mut cmd_find_state = 0 as *mut cmd_find_state;
    let mut fs: cmd_find_state =
        cmd_find_state{flags: 0,
                       current: 0 as *mut cmd_find_state,
                       s: 0 as *mut session,
                       wl: 0 as *mut winlink,
                       w: 0 as *mut window,
                       wp: 0 as *mut window_pane,
                       idx: 0,};
    let mut flags: libc::c_int = 0;
    flags = !(0 == (*cmd).flags & 1i32) as libc::c_int;
    cmdq_guard(item, b"begin\x00" as *const u8 as *const libc::c_char, flags);
    if (*item).client == 0 as *mut libc::c_void as *mut client {
        (*item).client = cmd_find_client(item, 0 as *const libc::c_char, 1i32)
    }
    retval =
        cmdq_find_flag(item, &mut (*item).source as *mut cmd_find_state,
                       &(*entry).source as *const cmd_entry_flag);
    if !(retval as libc::c_int == CMD_RETURN_ERROR as libc::c_int) {
        retval =
            cmdq_find_flag(item, &mut (*item).target as *mut cmd_find_state,
                           &(*entry).target as *const cmd_entry_flag);
        if !(retval as libc::c_int == CMD_RETURN_ERROR as libc::c_int) {
            retval =
                (*entry).exec.expect("non-null function pointer")(cmd, item);
            if !(retval as libc::c_int == CMD_RETURN_ERROR as libc::c_int) {
                if 0 != (*entry).flags & 4i32 {
                    if 0 !=
                           cmd_find_valid_state(&mut (*item).target as
                                                    *mut cmd_find_state) {
                        fsp = &mut (*item).target as *mut cmd_find_state;
                        current_block = 6873731126896040597;
                    } else if 0 !=
                                  cmd_find_valid_state(&mut (*(*item).shared).current
                                                           as
                                                           *mut cmd_find_state)
                     {
                        fsp =
                            &mut (*(*item).shared).current as
                                *mut cmd_find_state;
                        current_block = 6873731126896040597;
                    } else if cmd_find_from_client(&mut fs as
                                                       *mut cmd_find_state,
                                                   (*item).client, 0i32) ==
                                  0i32 {
                        fsp = &mut fs as *mut cmd_find_state;
                        current_block = 6873731126896040597;
                    } else { current_block = 4163364439964534091; }
                    match current_block {
                        4163364439964534091 => { }
                        _ => {
                            hooks_insert((*(*fsp).s).hooks, item, fsp,
                                         b"after-%s\x00" as *const u8 as
                                             *const libc::c_char,
                                         (*entry).name);
                        }
                    }
                }
            }
        }
    }
    (*item).client = c;
    if retval as libc::c_int == CMD_RETURN_ERROR as libc::c_int {
        cmdq_guard(item, b"error\x00" as *const u8 as *const libc::c_char,
                   flags);
    } else {
        cmdq_guard(item, b"end\x00" as *const u8 as *const libc::c_char,
                   flags);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn cmdq_guard(mut item: *mut cmdq_item,
                                    mut guard: *const libc::c_char,
                                    mut flags: libc::c_int) -> () {
    let mut c: *mut client = (*item).client;
    if c == 0 as *mut libc::c_void as *mut client || 0 == (*c).flags & 8192i32
       {
        return
    } else {
        evbuffer_add_printf((*c).stdout_data,
                            b"%%%s %ld %u %d\n\x00" as *const u8 as
                                *const libc::c_char, guard, (*item).time,
                            (*item).number, flags);
        server_client_push_stdout(c);
        return;
    };
}
unsafe extern "C" fn cmdq_find_flag(mut item: *mut cmdq_item,
                                    mut fs: *mut cmd_find_state,
                                    mut flag: *const cmd_entry_flag)
 -> cmd_retval {
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    if (*flag).flag as libc::c_int == 0i32 {
        cmd_find_clear_state(fs, 0i32);
        return CMD_RETURN_NORMAL
    } else {
        value = args_get((*(*item).cmd).args, (*flag).flag as u_char);
        if cmd_find_target(fs, item, value, (*flag).type_0, (*flag).flags) !=
               0i32 {
            cmd_find_clear_state(fs, 0i32);
            return CMD_RETURN_ERROR
        } else { return CMD_RETURN_NORMAL }
    };
}

