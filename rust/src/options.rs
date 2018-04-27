extern crate libc;

use attributes::attributes_tostring;
use colour::colour_tostring;
use grid::{grid_cell, grid_default_cell};
use style::{style_parse, style_tostring};
use utf8::utf8_data;
use xmalloc::{xstrdup, xcalloc, xreallocarray};

extern "C" {
    pub type evbuffer;
    pub type input_ctx;
    pub type format_tree;
    pub type environ;
    pub type _IO_FILE_plus;
    pub type screen_titles;
    pub type args_entry;
    pub type tmuxpeer;
    pub type tty_code;
    pub type tmuxproc;
    pub type bufferevent_ops;
    pub type event_base;
    pub type format_job_tree;
    pub type hooks;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char)
     -> *mut libc::c_char;
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
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, ...)
     -> libc::c_int;
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
    fn key_string_lookup_key(_: key_code) -> *const libc::c_char;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut clients: clients;
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
pub struct unnamed {
    pub ev_io_next: unnamed_2,
    pub ev_timeout: timeval,
}
pub type unnamed_0 = libc::c_uint;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub type pid_t = __pid_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_8 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_38,
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
    pub entry: unnamed_17,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub type cc_t = libc::c_uchar;
pub const PROMPT_COMMAND: unnamed_0 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_15,
    pub entry: unnamed_7,
}
pub type u_short = __u_short;
pub type unnamed_3 = libc::c_uint;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
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
    pub message_log: unnamed_28,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_0,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_41,
}
pub type tcflag_t = libc::c_uint;
pub type __u_char = libc::c_uchar;
pub const LINE_SEL_NONE: unnamed_8 = 0;
pub const CMD_RETURN_STOP: cmd_retval = 2;
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
pub struct unnamed_4 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub type cmd_find_type = libc::c_uint;
pub const _ISlower: unnamed_3 = 512;
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_36,
    pub ev_next: unnamed_42,
    pub ev_timeout_pos: unnamed_35,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_19,
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
pub struct in6_addr {
    pub __in6_u: unnamed_11,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
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
    pub alerts_entry: unnamed_33,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_39,
    pub entry: unnamed_23,
}
pub const TTY_VT102: unnamed_14 = 2;
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_10,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type unnamed_8 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_32,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
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
    pub entry: unnamed_25,
    pub tree_entry: unnamed_31,
}
pub const TTY_UNKNOWN: unnamed_14 = 6;
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
pub struct options_entry {
    pub owner: *mut options,
    pub name: *const libc::c_char,
    pub tableentry: *const options_table_entry,
    pub unnamed: unnamed_20,
    pub entry: unnamed_13,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_8,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub const _ISalnum: unnamed_3 = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const JOB_RUNNING: unnamed_38 = 0;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_11 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
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
pub struct unnamed_12 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub type u_int = __u_int;
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub rbe_left: *mut options_entry,
    pub rbe_right: *mut options_entry,
    pub rbe_parent: *mut options_entry,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
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
pub const _ISprint: unnamed_3 = 16384;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub type unnamed_14 = libc::c_uint;
pub type __u_short = libc::c_ushort;
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
    pub entry: unnamed_21,
    pub wentry: unnamed_12,
    pub sentry: unnamed_1,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub array: *mut *const libc::c_char,
    pub arraysize: u_int,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct options {
    pub tree: options_tree,
    pub parent: *mut options,
}
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_6,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const JOB_DEAD: unnamed_38 = 1;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_40,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_19 {
    ev_io: unnamed,
    ev_signal: unnamed_27,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_20 {
    string: *mut libc::c_char,
    number: libc::c_longlong,
    style: grid_cell,
    unnamed: unnamed_16,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
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
    pub gentry: unnamed_37,
    pub entry: unnamed_29,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const TTY_VT420: unnamed_14 = 5;
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub type __off_t = libc::c_long;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
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
pub struct unnamed_23 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct options_tree {
    pub rbh_root: *mut options_entry,
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
    pub entry: unnamed_22,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
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
pub const TTY_VT100: unnamed_14 = 0;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const _ISupper: unnamed_3 = 256;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub ev_signal_next: unnamed_24,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
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
    pub entry: unnamed_9,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const TTY_VT220: unnamed_14 = 3;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_30,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub const PROMPT_ENTRY: unnamed_0 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub type key_code = libc::c_ulonglong;
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
pub const _ISpunct: unnamed_3 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_5,
}
pub const JOB_CLOSED: unnamed_38 = 2;
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
pub const _IScntrl: unnamed_3 = 2;
pub type size_t = libc::c_ulong;
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
    pub term_type: unnamed_14,
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
pub struct sessions {
    pub rbh_root: *mut session,
}
pub type speed_t = libc::c_uint;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_30 {
    offset: u_int,
    data: unnamed_26,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub type options_table_type = libc::c_uint;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_34,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const TTY_VT101: unnamed_14 = 1;
pub const _ISgraph: unnamed_3 = 32768;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_35 {
    ev_next_with_common_timeout: unnamed_18,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub type unnamed_38 = libc::c_uint;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const LINE_SEL_RIGHT_LEFT: unnamed_8 = 2;
pub const _ISspace: unnamed_3 = 8192;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub const _ISblank: unnamed_3 = 1;
pub type u_char = __u_char;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_40 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const _ISxdigit: unnamed_3 = 4096;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_41 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const _ISdigit: unnamed_3 = 2048;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type uint16_t = libc::c_ushort;
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
pub struct unnamed_42 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type uint8_t = libc::c_uchar;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const TTY_VT320: unnamed_14 = 4;
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
pub type uint32_t = libc::c_uint;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const _ISalpha: unnamed_3 = 1024;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_4,
}
#[no_mangle]
pub unsafe extern "C" fn options_create(mut parent: *mut options)
 -> *mut options {
    let mut oo: *mut options = 0 as *mut options;
    oo =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<options>() as libc::c_ulong) as
            *mut options;
    loop  {
        let ref mut fresh0 =
            (*(&mut (*oo).tree as *mut options_tree)).rbh_root;
        *fresh0 = 0 as *mut options_entry;
        if !(0 != 0i32) { break ; }
    }
    (*oo).parent = parent;
    return oo;
}
#[no_mangle]
pub unsafe extern "C" fn options_free(mut oo: *mut options) -> () {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut tmp: *mut options_entry = 0 as *mut options_entry;
    o =
        options_tree_RB_MINMAX(&mut (*oo).tree as *mut options_tree,
                               1i32.wrapping_neg());
    while o != 0 as *mut libc::c_void as *mut options_entry &&
              { tmp = options_tree_RB_NEXT(o); 0 != 1i32 } {
        options_remove(o);
        o = tmp
    }
    free(oo as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn options_remove(mut o: *mut options_entry) -> () {
    let mut oo: *mut options = (*o).owner;
    let mut i: u_int = 0;
    if (*o).tableentry == 0 as *mut libc::c_void as *const options_table_entry
           ||
           (*(*o).tableentry).type_0 as libc::c_uint ==
               OPTIONS_TABLE_STRING as libc::c_int as libc::c_uint {
        free((*o).unnamed.string as *mut libc::c_void);
    } else if (*o).tableentry !=
                  0 as *mut libc::c_void as *const options_table_entry &&
                  (*(*o).tableentry).type_0 as libc::c_uint ==
                      OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint {
        i = 0i32 as u_int;
        while i < (*o).unnamed.unnamed.arraysize {
            free(*(*o).unnamed.unnamed.array.offset(i as isize) as
                     *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        free((*o).unnamed.unnamed.array as *mut libc::c_void);
    }
    options_tree_RB_REMOVE(&mut (*oo).tree as *mut options_tree, o);
    free(o as *mut libc::c_void);
}
unsafe extern "C" fn options_tree_RB_REMOVE(mut head: *mut options_tree,
                                            mut elm: *mut options_entry)
 -> *mut options_entry {
    let mut current_block: u64;
    let mut child: *mut options_entry = 0 as *mut options_entry;
    let mut parent: *mut options_entry = 0 as *mut options_entry;
    let mut old: *mut options_entry = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left == 0 as *mut libc::c_void as *mut options_entry {
        child = (*elm).entry.rbe_right;
        current_block = 9386390421034826751;
    } else if (*elm).entry.rbe_right ==
                  0 as *mut libc::c_void as *mut options_entry {
        child = (*elm).entry.rbe_left;
        current_block = 9386390421034826751;
    } else {
        let mut left: *mut options_entry = 0 as *mut options_entry;
        elm = (*elm).entry.rbe_right;
        loop  {
            left = (*elm).entry.rbe_left;
            if left.is_null() { break ; }
            elm = left
        }
        child = (*elm).entry.rbe_right;
        parent = (*elm).entry.rbe_parent;
        color = (*elm).entry.rbe_color;
        if !child.is_null() { (*child).entry.rbe_parent = parent }
        if !parent.is_null() {
            if (*parent).entry.rbe_left == elm {
                (*parent).entry.rbe_left = child
            } else { (*parent).entry.rbe_right = child }
            while 0 != 0i32 { }
        } else { (*head).rbh_root = child }
        if (*elm).entry.rbe_parent == old { parent = elm }
        (*elm).entry = (*old).entry;
        if !(*old).entry.rbe_parent.is_null() {
            if (*(*old).entry.rbe_parent).entry.rbe_left == old {
                (*(*old).entry.rbe_parent).entry.rbe_left = elm
            } else { (*(*old).entry.rbe_parent).entry.rbe_right = elm }
            while 0 != 0i32 { }
        } else { (*head).rbh_root = elm }
        (*(*old).entry.rbe_left).entry.rbe_parent = elm;
        if !(*old).entry.rbe_right.is_null() {
            (*(*old).entry.rbe_right).entry.rbe_parent = elm
        }
        if !parent.is_null() {
            left = parent;
            loop  {
                if 0 != 0i32 { continue ; }
                left = (*left).entry.rbe_parent;
                if left.is_null() { break ; }
            }
            current_block = 3071463015564287687;
        } else { current_block = 3071463015564287687; }
    }
    match current_block {
        9386390421034826751 => {
            parent = (*elm).entry.rbe_parent;
            color = (*elm).entry.rbe_color;
            if !child.is_null() { (*child).entry.rbe_parent = parent }
            if !parent.is_null() {
                if (*parent).entry.rbe_left == elm {
                    (*parent).entry.rbe_left = child
                } else { (*parent).entry.rbe_right = child }
                while 0 != 0i32 { }
            } else { (*head).rbh_root = child }
        }
        _ => { }
    }
    if color == 0i32 { options_tree_RB_REMOVE_COLOR(head, parent, child); }
    return old;
}
unsafe extern "C" fn options_tree_RB_REMOVE_COLOR(mut head: *mut options_tree,
                                                  mut parent:
                                                      *mut options_entry,
                                                  mut elm: *mut options_entry)
 -> () {
    let mut current_block: u64;
    let mut tmp: *mut options_entry = 0 as *mut options_entry;
    's_4:
        loop  {
            if !((elm == 0 as *mut libc::c_void as *mut options_entry ||
                      (*elm).entry.rbe_color == 0i32) &&
                     elm != (*head).rbh_root) {
                current_block = 11174649648027449784;
                break ;
            }
            if (*parent).entry.rbe_left == elm {
                tmp = (*parent).entry.rbe_right;
                if (*tmp).entry.rbe_color == 1i32 {
                    current_block = 17179679302217393232;
                } else { current_block = 14155750587950065367; }
                loop  {
                    match current_block {
                        17179679302217393232 => {
                            (*tmp).entry.rbe_color = 0i32;
                            (*parent).entry.rbe_color = 1i32;
                            if 0 != 0i32 {
                                current_block = 17179679302217393232;
                                continue ;
                            }
                            's_30:
                                loop  {
                                    tmp = (*parent).entry.rbe_right;
                                    (*parent).entry.rbe_right =
                                        (*tmp).entry.rbe_left;
                                    if !(*parent).entry.rbe_right.is_null() {
                                        (*(*tmp).entry.rbe_left).entry.rbe_parent
                                            = parent
                                    }
                                    while 0 != 0i32 { }
                                    (*tmp).entry.rbe_parent =
                                        (*parent).entry.rbe_parent;
                                    if !(*tmp).entry.rbe_parent.is_null() {
                                        if parent ==
                                               (*(*parent).entry.rbe_parent).entry.rbe_left
                                           {
                                            (*(*parent).entry.rbe_parent).entry.rbe_left
                                                = tmp
                                        } else {
                                            (*(*parent).entry.rbe_parent).entry.rbe_right
                                                = tmp
                                        }
                                    } else { (*head).rbh_root = tmp }
                                    (*tmp).entry.rbe_left = parent;
                                    (*parent).entry.rbe_parent = tmp;
                                    while 0 != 0i32 { }
                                    if !(*tmp).entry.rbe_parent.is_null() {
                                        current_block = 11050875288958768710;
                                    } else {
                                        current_block = 15240798224410183470;
                                    }
                                    loop  {
                                        match current_block {
                                            11050875288958768710 => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        11050875288958768710;
                                                } else {
                                                    current_block =
                                                        15240798224410183470;
                                                }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_30 ; }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).entry.rbe_right;
                            current_block = 14155750587950065367;
                        }
                        _ => {
                            if ((*tmp).entry.rbe_left ==
                                    0 as *mut libc::c_void as
                                        *mut options_entry ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut options_entry ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_right ==
                                          0 as *mut libc::c_void as
                                              *mut options_entry ||
                                          (*(*tmp).entry.rbe_right).entry.rbe_color
                                              == 0i32 {
                                current_block = 15976848397966268834;
                                break 's_4 ;
                            } else {
                                current_block = 7149356873433890176;
                                break 's_4 ;
                            }
                        }
                    }
                }
            } else {
                tmp = (*parent).entry.rbe_left;
                if (*tmp).entry.rbe_color == 1i32 {
                    current_block = 6450597802325118133;
                } else { current_block = 7746103178988627676; }
                loop  {
                    match current_block {
                        7746103178988627676 => {
                            if ((*tmp).entry.rbe_left ==
                                    0 as *mut libc::c_void as
                                        *mut options_entry ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut options_entry ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_left ==
                                          0 as *mut libc::c_void as
                                              *mut options_entry ||
                                          (*(*tmp).entry.rbe_left).entry.rbe_color
                                              == 0i32 {
                                current_block = 13826291924415791078;
                                break 's_4 ;
                            } else {
                                current_block = 5892776923941496671;
                                break 's_4 ;
                            }
                        }
                        _ => {
                            (*tmp).entry.rbe_color = 0i32;
                            (*parent).entry.rbe_color = 1i32;
                            if 0 != 0i32 {
                                current_block = 6450597802325118133;
                                continue ;
                            }
                            's_210:
                                loop  {
                                    tmp = (*parent).entry.rbe_left;
                                    (*parent).entry.rbe_left =
                                        (*tmp).entry.rbe_right;
                                    if !(*parent).entry.rbe_left.is_null() {
                                        (*(*tmp).entry.rbe_right).entry.rbe_parent
                                            = parent
                                    }
                                    while 0 != 0i32 { }
                                    (*tmp).entry.rbe_parent =
                                        (*parent).entry.rbe_parent;
                                    if !(*tmp).entry.rbe_parent.is_null() {
                                        if parent ==
                                               (*(*parent).entry.rbe_parent).entry.rbe_left
                                           {
                                            (*(*parent).entry.rbe_parent).entry.rbe_left
                                                = tmp
                                        } else {
                                            (*(*parent).entry.rbe_parent).entry.rbe_right
                                                = tmp
                                        }
                                    } else { (*head).rbh_root = tmp }
                                    (*tmp).entry.rbe_right = parent;
                                    (*parent).entry.rbe_parent = tmp;
                                    while 0 != 0i32 { }
                                    if !(*tmp).entry.rbe_parent.is_null() {
                                        current_block = 16738040538446813684;
                                    } else {
                                        current_block = 17784502470059252271;
                                    }
                                    loop  {
                                        match current_block {
                                            16738040538446813684 => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        16738040538446813684;
                                                } else {
                                                    current_block =
                                                        17784502470059252271;
                                                }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_210 ; }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).entry.rbe_left;
                            current_block = 7746103178988627676;
                        }
                    }
                }
            }
        }
    match current_block {
        13826291924415791078 => {
            let mut oright: *mut options_entry = 0 as *mut options_entry;
            oright = (*tmp).entry.rbe_right;
            if !oright.is_null() { (*oright).entry.rbe_color = 0i32 }
            (*tmp).entry.rbe_color = 1i32;
            's_276:
                loop  {
                    oright = (*tmp).entry.rbe_right;
                    (*tmp).entry.rbe_right = (*oright).entry.rbe_left;
                    if !(*tmp).entry.rbe_right.is_null() {
                        (*(*oright).entry.rbe_left).entry.rbe_parent = tmp
                    }
                    while 0 != 0i32 { }
                    (*oright).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oright).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oright
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right =
                                oright
                        }
                    } else { (*head).rbh_root = oright }
                    (*oright).entry.rbe_left = tmp;
                    (*tmp).entry.rbe_parent = oright;
                    while 0 != 0i32 { }
                    if !(*oright).entry.rbe_parent.is_null() {
                        current_block = 3392087639489470149;
                    } else { current_block = 1854459640724737493; }
                    loop  {
                        match current_block {
                            1854459640724737493 => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_276 ; }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    current_block = 3392087639489470149;
                                } else {
                                    current_block = 1854459640724737493;
                                }
                            }
                        }
                    }
                }
            tmp = (*parent).entry.rbe_left;
            current_block = 5892776923941496671;
        }
        15976848397966268834 => {
            let mut oleft: *mut options_entry = 0 as *mut options_entry;
            oleft = (*tmp).entry.rbe_left;
            if !oleft.is_null() { (*oleft).entry.rbe_color = 0i32 }
            (*tmp).entry.rbe_color = 1i32;
            's_96:
                loop  {
                    oleft = (*tmp).entry.rbe_left;
                    (*tmp).entry.rbe_left = (*oleft).entry.rbe_right;
                    if !(*tmp).entry.rbe_left.is_null() {
                        (*(*oleft).entry.rbe_right).entry.rbe_parent = tmp
                    }
                    while 0 != 0i32 { }
                    (*oleft).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oleft).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oleft
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oleft
                        }
                    } else { (*head).rbh_root = oleft }
                    (*oleft).entry.rbe_right = tmp;
                    (*tmp).entry.rbe_parent = oleft;
                    while 0 != 0i32 { }
                    if !(*oleft).entry.rbe_parent.is_null() {
                        current_block = 2232869372362427478;
                    } else { current_block = 15904375183555213903; }
                    loop  {
                        match current_block {
                            15904375183555213903 => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_96 ; }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    current_block = 2232869372362427478;
                                } else {
                                    current_block = 15904375183555213903;
                                }
                            }
                        }
                    }
                }
            tmp = (*parent).entry.rbe_right;
            current_block = 7149356873433890176;
        }
        _ => { }
    }
    match current_block {
        7149356873433890176 => {
            (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
            (*parent).entry.rbe_color = 0i32;
            if !(*tmp).entry.rbe_right.is_null() {
                (*(*tmp).entry.rbe_right).entry.rbe_color = 0i32
            }
            's_148:
                loop  {
                    tmp = (*parent).entry.rbe_right;
                    (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                    if !(*parent).entry.rbe_right.is_null() {
                        (*(*tmp).entry.rbe_left).entry.rbe_parent = parent
                    }
                    while 0 != 0i32 { }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent ==
                               (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right =
                                tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).entry.rbe_left = parent;
                    (*parent).entry.rbe_parent = tmp;
                    while 0 != 0i32 { }
                    if !(*tmp).entry.rbe_parent.is_null() {
                        current_block = 6450636197030046351;
                    } else { current_block = 16924917904204750491; }
                    loop  {
                        match current_block {
                            16924917904204750491 => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_148 ; }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    current_block = 6450636197030046351;
                                } else {
                                    current_block = 16924917904204750491;
                                }
                            }
                        }
                    }
                }
            elm = (*head).rbh_root
        }
        5892776923941496671 => {
            (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
            (*parent).entry.rbe_color = 0i32;
            if !(*tmp).entry.rbe_left.is_null() {
                (*(*tmp).entry.rbe_left).entry.rbe_color = 0i32
            }
            's_328:
                loop  {
                    tmp = (*parent).entry.rbe_left;
                    (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                    if !(*parent).entry.rbe_left.is_null() {
                        (*(*tmp).entry.rbe_right).entry.rbe_parent = parent
                    }
                    while 0 != 0i32 { }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent ==
                               (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right =
                                tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).entry.rbe_right = parent;
                    (*parent).entry.rbe_parent = tmp;
                    while 0 != 0i32 { }
                    if !(*tmp).entry.rbe_parent.is_null() {
                        current_block = 13910774313357589740;
                    } else { current_block = 13707613154239713890; }
                    loop  {
                        match current_block {
                            13707613154239713890 => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_328 ; }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    current_block = 13910774313357589740;
                                } else {
                                    current_block = 13707613154239713890;
                                }
                            }
                        }
                    }
                }
            elm = (*head).rbh_root
        }
        _ => { }
    }
    if !elm.is_null() { (*elm).entry.rbe_color = 0i32 };
}
unsafe extern "C" fn options_tree_RB_NEXT(mut elm: *mut options_entry)
 -> *mut options_entry {
    if !(*elm).entry.rbe_right.is_null() {
        elm = (*elm).entry.rbe_right;
        while !(*elm).entry.rbe_left.is_null() { elm = (*elm).entry.rbe_left }
    } else if !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_left {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_right {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
unsafe extern "C" fn options_tree_RB_MINMAX(mut head: *mut options_tree,
                                            mut val: libc::c_int)
 -> *mut options_entry {
    let mut tmp: *mut options_entry = (*head).rbh_root;
    let mut parent: *mut options_entry = 0 as *mut options_entry;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else { tmp = (*tmp).entry.rbe_right }
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn options_first(mut oo: *mut options)
 -> *mut options_entry {
    return options_tree_RB_MINMAX(&mut (*oo).tree as *mut options_tree,
                                  1i32.wrapping_neg());
}
#[no_mangle]
pub unsafe extern "C" fn options_next(mut o: *mut options_entry)
 -> *mut options_entry {
    return options_tree_RB_NEXT(o);
}
#[no_mangle]
pub unsafe extern "C" fn options_empty(mut oo: *mut options,
                                       mut oe: *const options_table_entry)
 -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_add(oo, (*oe).name);
    (*o).tableentry = oe;
    return o;
}
unsafe extern "C" fn options_add(mut oo: *mut options,
                                 mut name: *const libc::c_char)
 -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get_only(oo, name);
    if o != 0 as *mut libc::c_void as *mut options_entry {
        options_remove(o);
    }
    o =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<options_entry>() as libc::c_ulong) as
            *mut options_entry;
    (*o).owner = oo;
    (*o).name = xstrdup(name);
    options_tree_RB_INSERT(&mut (*oo).tree as *mut options_tree, o);
    return o;
}
unsafe extern "C" fn options_tree_RB_INSERT(mut head: *mut options_tree,
                                            mut elm: *mut options_entry)
 -> *mut options_entry {
    let mut tmp: *mut options_entry = 0 as *mut options_entry;
    let mut parent: *mut options_entry = 0 as *mut options_entry;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = options_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    loop  {
        (*elm).entry.rbe_parent = parent;
        (*elm).entry.rbe_right = 0 as *mut options_entry;
        (*elm).entry.rbe_left = (*elm).entry.rbe_right;
        (*elm).entry.rbe_color = 1i32;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut options_entry {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else { (*parent).entry.rbe_right = elm }
        while 0 != 0i32 { }
    } else { (*head).rbh_root = elm }
    options_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut options_entry;
}
unsafe extern "C" fn options_tree_RB_INSERT_COLOR(mut head: *mut options_tree,
                                                  mut elm: *mut options_entry)
 -> () {
    let mut current_block: u64;
    let mut parent: *mut options_entry = 0 as *mut options_entry;
    let mut gparent: *mut options_entry = 0 as *mut options_entry;
    let mut tmp: *mut options_entry = 0 as *mut options_entry;
    loop  {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() && (*parent).entry.rbe_color == 1i32) {
            break ;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                loop  {
                    (*parent).entry.rbe_color = 0i32;
                    (*gparent).entry.rbe_color = 1i32;
                    if !(0 != 0i32) { break ; }
                }
                elm = gparent
            } else {
                if (*parent).entry.rbe_right == elm {
                    current_block = 7351195479953500246;
                } else { current_block = 4956146061682418353; }
                's_87:
                    loop  {
                        match current_block {
                            4956146061682418353 => {
                                (*parent).entry.rbe_color = 0i32;
                                (*gparent).entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4956146061682418353;
                                } else { break ; }
                            }
                            _ => {
                                tmp = (*parent).entry.rbe_right;
                                (*parent).entry.rbe_right =
                                    (*tmp).entry.rbe_left;
                                if !(*parent).entry.rbe_right.is_null() {
                                    (*(*tmp).entry.rbe_left).entry.rbe_parent
                                        = parent
                                }
                                while 0 != 0i32 { }
                                (*tmp).entry.rbe_parent =
                                    (*parent).entry.rbe_parent;
                                if !(*tmp).entry.rbe_parent.is_null() {
                                    if parent ==
                                           (*(*parent).entry.rbe_parent).entry.rbe_left
                                       {
                                        (*(*parent).entry.rbe_parent).entry.rbe_left
                                            = tmp
                                    } else {
                                        (*(*parent).entry.rbe_parent).entry.rbe_right
                                            = tmp
                                    }
                                } else { (*head).rbh_root = tmp }
                                (*tmp).entry.rbe_left = parent;
                                (*parent).entry.rbe_parent = tmp;
                                while 0 != 0i32 { }
                                if !(*tmp).entry.rbe_parent.is_null() {
                                    current_block = 10048703153582371463;
                                } else {
                                    current_block = 10879442775620481940;
                                }
                                loop  {
                                    match current_block {
                                        10879442775620481940 => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    7351195479953500246;
                                                continue 's_87 ;
                                            } else { break ; }
                                        }
                                        _ => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    10048703153582371463;
                                            } else {
                                                current_block =
                                                    10879442775620481940;
                                            }
                                        }
                                    }
                                }
                                tmp = parent;
                                parent = elm;
                                elm = tmp;
                                current_block = 4956146061682418353;
                            }
                        }
                    }
                's_95:
                    loop  {
                        tmp = (*gparent).entry.rbe_left;
                        (*gparent).entry.rbe_left = (*tmp).entry.rbe_right;
                        if !(*gparent).entry.rbe_left.is_null() {
                            (*(*tmp).entry.rbe_right).entry.rbe_parent =
                                gparent
                        }
                        while 0 != 0i32 { }
                        (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                        if !(*tmp).entry.rbe_parent.is_null() {
                            if gparent ==
                                   (*(*gparent).entry.rbe_parent).entry.rbe_left
                               {
                                (*(*gparent).entry.rbe_parent).entry.rbe_left
                                    = tmp
                            } else {
                                (*(*gparent).entry.rbe_parent).entry.rbe_right
                                    = tmp
                            }
                        } else { (*head).rbh_root = tmp }
                        (*tmp).entry.rbe_right = gparent;
                        (*gparent).entry.rbe_parent = tmp;
                        while 0 != 0i32 { }
                        if !(*tmp).entry.rbe_parent.is_null() {
                            current_block = 6669252993407410313;
                        } else { current_block = 5948590327928692120; }
                        loop  {
                            match current_block {
                                5948590327928692120 => {
                                    if 0 != 0i32 {
                                        break ;
                                    } else { break 's_95 ; }
                                }
                                _ => {
                                    if 0 != 0i32 {
                                        current_block = 6669252993407410313;
                                    } else {
                                        current_block = 5948590327928692120;
                                    }
                                }
                            }
                        }
                    }
            }
        } else {
            tmp = (*gparent).entry.rbe_left;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                loop  {
                    (*parent).entry.rbe_color = 0i32;
                    (*gparent).entry.rbe_color = 1i32;
                    if !(0 != 0i32) { break ; }
                }
                elm = gparent
            } else {
                if (*parent).entry.rbe_left == elm {
                    current_block = 652864300344834934;
                } else { current_block = 4567019141635105728; }
                's_211:
                    loop  {
                        match current_block {
                            652864300344834934 => {
                                tmp = (*parent).entry.rbe_left;
                                (*parent).entry.rbe_left =
                                    (*tmp).entry.rbe_right;
                                if !(*parent).entry.rbe_left.is_null() {
                                    (*(*tmp).entry.rbe_right).entry.rbe_parent
                                        = parent
                                }
                                while 0 != 0i32 { }
                                (*tmp).entry.rbe_parent =
                                    (*parent).entry.rbe_parent;
                                if !(*tmp).entry.rbe_parent.is_null() {
                                    if parent ==
                                           (*(*parent).entry.rbe_parent).entry.rbe_left
                                       {
                                        (*(*parent).entry.rbe_parent).entry.rbe_left
                                            = tmp
                                    } else {
                                        (*(*parent).entry.rbe_parent).entry.rbe_right
                                            = tmp
                                    }
                                } else { (*head).rbh_root = tmp }
                                (*tmp).entry.rbe_right = parent;
                                (*parent).entry.rbe_parent = tmp;
                                while 0 != 0i32 { }
                                if !(*tmp).entry.rbe_parent.is_null() {
                                    current_block = 980989089337379490;
                                } else {
                                    current_block = 5494826135382683477;
                                }
                                loop  {
                                    match current_block {
                                        980989089337379490 => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    980989089337379490;
                                            } else {
                                                current_block =
                                                    5494826135382683477;
                                            }
                                        }
                                        _ => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    652864300344834934;
                                                continue 's_211 ;
                                            } else { break ; }
                                        }
                                    }
                                }
                                tmp = parent;
                                parent = elm;
                                elm = tmp;
                                current_block = 4567019141635105728;
                            }
                            _ => {
                                (*parent).entry.rbe_color = 0i32;
                                (*gparent).entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4567019141635105728;
                                } else { break ; }
                            }
                        }
                    }
                's_219:
                    loop  {
                        tmp = (*gparent).entry.rbe_right;
                        (*gparent).entry.rbe_right = (*tmp).entry.rbe_left;
                        if !(*gparent).entry.rbe_right.is_null() {
                            (*(*tmp).entry.rbe_left).entry.rbe_parent =
                                gparent
                        }
                        while 0 != 0i32 { }
                        (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                        if !(*tmp).entry.rbe_parent.is_null() {
                            if gparent ==
                                   (*(*gparent).entry.rbe_parent).entry.rbe_left
                               {
                                (*(*gparent).entry.rbe_parent).entry.rbe_left
                                    = tmp
                            } else {
                                (*(*gparent).entry.rbe_parent).entry.rbe_right
                                    = tmp
                            }
                        } else { (*head).rbh_root = tmp }
                        (*tmp).entry.rbe_left = gparent;
                        (*gparent).entry.rbe_parent = tmp;
                        while 0 != 0i32 { }
                        if !(*tmp).entry.rbe_parent.is_null() {
                            current_block = 11793792312832361944;
                        } else { current_block = 2543120759711851213; }
                        loop  {
                            match current_block {
                                11793792312832361944 => {
                                    if 0 != 0i32 {
                                        current_block = 11793792312832361944;
                                    } else {
                                        current_block = 2543120759711851213;
                                    }
                                }
                                _ => {
                                    if 0 != 0i32 {
                                        break ;
                                    } else { break 's_219 ; }
                                }
                            }
                        }
                    }
            }
        }
    }
    (*(*head).rbh_root).entry.rbe_color = 0i32;
}
unsafe extern "C" fn options_cmp(mut lhs: *mut options_entry,
                                 mut rhs: *mut options_entry) -> libc::c_int {
    return strcmp((*lhs).name, (*rhs).name);
}
#[no_mangle]
pub unsafe extern "C" fn options_get_only(mut oo: *mut options,
                                          mut name: *const libc::c_char)
 -> *mut options_entry {
    let mut o: options_entry =
        options_entry{owner: 0 as *mut options,
                      name: 0 as *const libc::c_char,
                      tableentry: 0 as *const options_table_entry,
                      unnamed: unnamed_20{string: 0 as *mut libc::c_char,},
                      entry:
                          unnamed_13{rbe_left: 0 as *mut options_entry,
                                     rbe_right: 0 as *mut options_entry,
                                     rbe_parent: 0 as *mut options_entry,
                                     rbe_color: 0,},};
    o.name = name;
    return options_tree_RB_FIND(&mut (*oo).tree as *mut options_tree,
                                &mut o as *mut options_entry);
}
unsafe extern "C" fn options_tree_RB_FIND(mut head: *mut options_tree,
                                          mut elm: *mut options_entry)
 -> *mut options_entry {
    let mut tmp: *mut options_entry = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = options_cmp(elm, tmp);
            if comp < 0i32 {
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return 0 as *mut options_entry }
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_default(mut oo: *mut options,
                                         mut oe: *const options_table_entry)
 -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_empty(oo, oe);
    if (*oe).type_0 as libc::c_uint ==
           OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint {
        options_array_assign(o, (*oe).default_str);
    } else if (*oe).type_0 as libc::c_uint ==
                  OPTIONS_TABLE_STRING as libc::c_int as libc::c_uint {
        (*o).unnamed.string = xstrdup((*oe).default_str)
    } else if (*oe).type_0 as libc::c_uint ==
                  OPTIONS_TABLE_STYLE as libc::c_int as libc::c_uint {
        memcpy(&mut (*o).unnamed.style as *mut grid_cell as *mut libc::c_void,
               &grid_default_cell as *const grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        style_parse(&grid_default_cell as *const grid_cell,
                    &mut (*o).unnamed.style as *mut grid_cell,
                    (*oe).default_str);
    } else { (*o).unnamed.number = (*oe).default_num }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_array_assign(mut o: *mut options_entry,
                                              mut s: *const libc::c_char)
 -> () {
    let mut separator: *const libc::c_char = 0 as *const libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: u_int = 0;
    separator = (*(*o).tableentry).separator;
    if separator == 0 as *mut libc::c_void as *const libc::c_char {
        separator = b" ,\x00" as *const u8 as *const libc::c_char
    }
    string = xstrdup(s);
    copy = string;
    loop  {
        next = strsep(&mut string as *mut *mut libc::c_char, separator);
        if !(next != 0 as *mut libc::c_void as *mut libc::c_char) { break ; }
        if *next as libc::c_int == 0 { continue ; }
        i = 0i32 as u_int;
        while i < 1000i32 as libc::c_uint {
            if i >= (*o).unnamed.unnamed.arraysize ||
                   *(*o).unnamed.unnamed.array.offset(i as isize) ==
                       0 as *mut libc::c_void as *const libc::c_char {
                break ;
            }
            i = i.wrapping_add(1)
        }
        if i == 1000i32 as libc::c_uint { break ; }
        options_array_set(o, i, next, 0i32);
    }
    free(copy as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn options_array_set(mut o: *mut options_entry,
                                           mut idx: u_int,
                                           mut value: *const libc::c_char,
                                           mut append: libc::c_int)
 -> libc::c_int {
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: u_int = 0;
    if 0 !=
           !((*o).tableentry !=
                 0 as *mut libc::c_void as *const options_table_entry &&
                 (*(*o).tableentry).type_0 as libc::c_uint ==
                     OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint) as
               libc::c_int {
        return 1i32.wrapping_neg()
    } else if idx >= 1000i32 as libc::c_uint {
        return 1i32.wrapping_neg()
    } else {
        if idx >= (*o).unnamed.unnamed.arraysize {
            (*o).unnamed.unnamed.array =
                xreallocarray((*o).unnamed.unnamed.array as *mut libc::c_void,
                              idx.wrapping_add(1i32 as libc::c_uint) as
                                  size_t,
                              ::std::mem::size_of::<*const libc::c_char>() as
                                  libc::c_ulong) as *mut *const libc::c_char;
            i = (*o).unnamed.unnamed.arraysize;
            while i < idx.wrapping_add(1i32 as libc::c_uint) {
                let ref mut fresh1 =
                    *(*o).unnamed.unnamed.array.offset(i as isize);
                *fresh1 = 0 as *const libc::c_char;
                i = i.wrapping_add(1)
            }
            (*o).unnamed.unnamed.arraysize =
                idx.wrapping_add(1i32 as libc::c_uint)
        }
        new = 0 as *mut libc::c_char;
        if value != 0 as *mut libc::c_void as *const libc::c_char {
            if *(*o).unnamed.unnamed.array.offset(idx as isize) !=
                   0 as *mut libc::c_void as *const libc::c_char &&
                   0 != append {
                xasprintf(&mut new as *mut *mut libc::c_char,
                          b"%s%s\x00" as *const u8 as *const libc::c_char,
                          *(*o).unnamed.unnamed.array.offset(idx as isize),
                          value);
            } else { new = xstrdup(value) }
        }
        free(*(*o).unnamed.unnamed.array.offset(idx as isize) as
                 *mut libc::c_void);
        let ref mut fresh2 = *(*o).unnamed.unnamed.array.offset(idx as isize);
        *fresh2 = new;
        return 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_name(mut o: *mut options_entry)
 -> *const libc::c_char {
    return (*o).name;
}
#[no_mangle]
pub unsafe extern "C" fn options_table_entry(mut o: *mut options_entry)
 -> *const options_table_entry {
    return (*o).tableentry;
}
#[no_mangle]
pub unsafe extern "C" fn options_get(mut oo: *mut options,
                                     mut name: *const libc::c_char)
 -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get_only(oo, name);
    while o == 0 as *mut libc::c_void as *mut options_entry {
        oo = (*oo).parent;
        if oo == 0 as *mut libc::c_void as *mut options { break ; }
        o = options_get_only(oo, name)
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_array_clear(mut o: *mut options_entry)
 -> () {
    if (*o).tableentry != 0 as *mut libc::c_void as *const options_table_entry
           &&
           (*(*o).tableentry).type_0 as libc::c_uint ==
               OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint {
        (*o).unnamed.unnamed.arraysize = 0i32 as u_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_array_get(mut o: *mut options_entry,
                                           mut idx: u_int)
 -> *const libc::c_char {
    if 0 !=
           !((*o).tableentry !=
                 0 as *mut libc::c_void as *const options_table_entry &&
                 (*(*o).tableentry).type_0 as libc::c_uint ==
                     OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint) as
               libc::c_int {
        return 0 as *const libc::c_char
    } else if idx >= (*o).unnamed.unnamed.arraysize {
        return 0 as *const libc::c_char
    } else { return *(*o).unnamed.unnamed.array.offset(idx as isize) };
}
#[no_mangle]
pub unsafe extern "C" fn options_array_size(mut o: *mut options_entry,
                                            mut size: *mut u_int)
 -> libc::c_int {
    if 0 !=
           !((*o).tableentry !=
                 0 as *mut libc::c_void as *const options_table_entry &&
                 (*(*o).tableentry).type_0 as libc::c_uint ==
                     OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint) as
               libc::c_int {
        return 1i32.wrapping_neg()
    } else {
        if size != 0 as *mut libc::c_void as *mut u_int {
            *size = (*o).unnamed.unnamed.arraysize
        }
        return 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_isstring(mut o: *mut options_entry)
 -> libc::c_int {
    if (*o).tableentry == 0 as *mut libc::c_void as *const options_table_entry
       {
        return 1i32
    } else {
        return ((*o).tableentry ==
                    0 as *mut libc::c_void as *const options_table_entry ||
                    (*(*o).tableentry).type_0 as libc::c_uint ==
                        OPTIONS_TABLE_STRING as libc::c_int as libc::c_uint ||
                    (*o).tableentry !=
                        0 as *mut libc::c_void as *const options_table_entry
                        &&
                        (*(*o).tableentry).type_0 as libc::c_uint ==
                            OPTIONS_TABLE_ARRAY as libc::c_int as
                                libc::c_uint) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_tostring(mut o: *mut options_entry,
                                          mut idx: libc::c_int,
                                          mut numeric: libc::c_int)
 -> *const libc::c_char {
    static mut s: [libc::c_char; 1024] = unsafe { [0; 1024] };
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    if (*o).tableentry != 0 as *mut libc::c_void as *const options_table_entry
           &&
           (*(*o).tableentry).type_0 as libc::c_uint ==
               OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint {
        if idx == 1i32.wrapping_neg() {
            return 0 as *const libc::c_char
        } else if idx as u_int >= (*o).unnamed.unnamed.arraysize ||
                      *(*o).unnamed.unnamed.array.offset(idx as isize) ==
                          0 as *mut libc::c_void as *const libc::c_char {
            return b"\x00" as *const u8 as *const libc::c_char
        } else { return *(*o).unnamed.unnamed.array.offset(idx as isize) }
    } else if (*o).tableentry !=
                  0 as *mut libc::c_void as *const options_table_entry &&
                  (*(*o).tableentry).type_0 as libc::c_uint ==
                      OPTIONS_TABLE_STYLE as libc::c_int as libc::c_uint {
        return style_tostring(&mut (*o).unnamed.style as *mut grid_cell)
    } else if (*o).tableentry !=
                  0 as *mut libc::c_void as *const options_table_entry &&
                  ((*(*o).tableentry).type_0 as libc::c_uint ==
                       OPTIONS_TABLE_NUMBER as libc::c_int as libc::c_uint ||
                       (*(*o).tableentry).type_0 as libc::c_uint ==
                           OPTIONS_TABLE_KEY as libc::c_int as libc::c_uint ||
                       (*(*o).tableentry).type_0 as libc::c_uint ==
                           OPTIONS_TABLE_COLOUR as libc::c_int as libc::c_uint
                       ||
                       (*(*o).tableentry).type_0 as libc::c_uint ==
                           OPTIONS_TABLE_ATTRIBUTES as libc::c_int as
                               libc::c_uint ||
                       (*(*o).tableentry).type_0 as libc::c_uint ==
                           OPTIONS_TABLE_FLAG as libc::c_int as libc::c_uint
                       ||
                       (*(*o).tableentry).type_0 as libc::c_uint ==
                           OPTIONS_TABLE_CHOICE as libc::c_int as
                               libc::c_uint) {
        tmp = 0 as *const libc::c_char;
        match (*(*o).tableentry).type_0 as libc::c_uint {
            1 => {
                xsnprintf(s.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 1024]>() as
                              libc::c_ulong,
                          b"%lld\x00" as *const u8 as *const libc::c_char,
                          (*o).unnamed.number);
            }
            2 => {
                tmp = key_string_lookup_key((*o).unnamed.number as key_code)
            }
            3 => { tmp = colour_tostring((*o).unnamed.number as libc::c_int) }
            4 => {
                tmp = attributes_tostring((*o).unnamed.number as libc::c_int)
            }
            5 => {
                if 0 != numeric {
                    xsnprintf(s.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                  libc::c_ulong,
                              b"%lld\x00" as *const u8 as *const libc::c_char,
                              (*o).unnamed.number);
                } else {
                    tmp =
                        if 0 != (*o).unnamed.number {
                            b"on\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"off\x00" as *const u8 as *const libc::c_char
                        }
                }
            }
            6 => {
                tmp =
                    *(*(*o).tableentry).choices.offset((*o).unnamed.number as
                                                           isize)
            }
            0 | 7 | 8 | _ => { }
        }
        if tmp != 0 as *mut libc::c_void as *const libc::c_char {
            xsnprintf(s.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 1024]>() as
                          libc::c_ulong,
                      b"%s\x00" as *const u8 as *const libc::c_char, tmp);
        }
        return s.as_mut_ptr()
    } else if (*o).tableentry ==
                  0 as *mut libc::c_void as *const options_table_entry ||
                  (*(*o).tableentry).type_0 as libc::c_uint ==
                      OPTIONS_TABLE_STRING as libc::c_int as libc::c_uint {
        return (*o).unnamed.string
    } else { return 0 as *const libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn options_parse(mut name: *const libc::c_char,
                                       mut idx: *mut libc::c_int)
 -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if *name as libc::c_int == 0 {
        return 0 as *mut libc::c_char
    } else {
        copy = xstrdup(name);
        cp = strchr(copy, 91);
        if cp == 0 as *mut libc::c_void as *mut libc::c_char {
            *idx = 1i32.wrapping_neg();
            return copy
        } else {
            end = strchr(cp.offset(1isize), 93);
            if end == 0 as *mut libc::c_void as *mut libc::c_char ||
                   *end.offset(1isize) as libc::c_int != 0 ||
                   0 ==
                       *(*__ctype_b_loc()).offset(*end.offset(1i32.wrapping_neg()
                                                                  as isize) as
                                                      u_char as libc::c_int as
                                                      isize) as libc::c_int &
                           _ISdigit as libc::c_int as libc::c_ushort as
                               libc::c_int {
                free(copy as *mut libc::c_void);
                return 0 as *mut libc::c_char
            } else if sscanf(cp,
                             b"[%d]\x00" as *const u8 as *const libc::c_char,
                             idx) != 1i32 || *idx < 0i32 {
                free(copy as *mut libc::c_void);
                return 0 as *mut libc::c_char
            } else { *cp = 0 as libc::c_char; return copy }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_parse_get(mut oo: *mut options,
                                           mut s: *const libc::c_char,
                                           mut idx: *mut libc::c_int,
                                           mut only: libc::c_int)
 -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = options_parse(s, idx);
    if name == 0 as *mut libc::c_void as *mut libc::c_char {
        return 0 as *mut options_entry
    } else {
        if 0 != only {
            o = options_get_only(oo, name)
        } else { o = options_get(oo, name) }
        free(name as *mut libc::c_void);
        return o
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_match(mut s: *const libc::c_char,
                                       mut idx: *mut libc::c_int,
                                       mut ambiguous: *mut libc::c_int)
 -> *mut libc::c_char {
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
    let mut found: *const options_table_entry =
        0 as *const options_table_entry;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut namelen: size_t = 0;
    name = options_parse(s, idx);
    if name == 0 as *mut libc::c_void as *mut libc::c_char {
        return 0 as *mut libc::c_char
    } else {
        namelen = strlen(name);
        if *name as libc::c_int == 64 {
            *ambiguous = 0i32;
            return name
        } else {
            found = 0 as *const options_table_entry;
            oe = options_table.as_ptr();
            while (*oe).name != 0 as *mut libc::c_void as *const libc::c_char
                  {
                if strcmp((*oe).name, name) == 0i32 {
                    found = oe;
                    break ;
                } else {
                    if strncmp((*oe).name, name, namelen) == 0i32 {
                        if found !=
                               0 as *mut libc::c_void as
                                   *const options_table_entry {
                            *ambiguous = 1i32;
                            free(name as *mut libc::c_void);
                            return 0 as *mut libc::c_char
                        } else { found = oe }
                    }
                    oe = oe.offset(1isize)
                }
            }
            free(name as *mut libc::c_void);
            if found == 0 as *mut libc::c_void as *const options_table_entry {
                *ambiguous = 0i32;
                return 0 as *mut libc::c_char
            } else { return xstrdup((*found).name) }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_match_get(mut oo: *mut options,
                                           mut s: *const libc::c_char,
                                           mut idx: *mut libc::c_int,
                                           mut only: libc::c_int,
                                           mut ambiguous: *mut libc::c_int)
 -> *mut options_entry {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    name = options_match(s, idx, ambiguous);
    if name == 0 as *mut libc::c_void as *mut libc::c_char {
        return 0 as *mut options_entry
    } else {
        *ambiguous = 0i32;
        if 0 != only {
            o = options_get_only(oo, name)
        } else { o = options_get(oo, name) }
        free(name as *mut libc::c_void);
        return o
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_get_string(mut oo: *mut options,
                                            mut name: *const libc::c_char)
 -> *const libc::c_char {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get(oo, name);
    if o == 0 as *mut libc::c_void as *mut options_entry {
        fatalx(b"missing option %s\x00" as *const u8 as *const libc::c_char,
               name);
    } else if 0 !=
                  !((*o).tableentry ==
                        0 as *mut libc::c_void as *const options_table_entry
                        ||
                        (*(*o).tableentry).type_0 as libc::c_uint ==
                            OPTIONS_TABLE_STRING as libc::c_int as
                                libc::c_uint) as libc::c_int {
        fatalx(b"option %s is not a string\x00" as *const u8 as
                   *const libc::c_char, name);
    } else { return (*o).unnamed.string };
}
#[no_mangle]
pub unsafe extern "C" fn options_get_number(mut oo: *mut options,
                                            mut name: *const libc::c_char)
 -> libc::c_longlong {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get(oo, name);
    if o == 0 as *mut libc::c_void as *mut options_entry {
        fatalx(b"missing option %s\x00" as *const u8 as *const libc::c_char,
               name);
    } else if 0 !=
                  !((*o).tableentry !=
                        0 as *mut libc::c_void as *const options_table_entry
                        &&
                        ((*(*o).tableentry).type_0 as libc::c_uint ==
                             OPTIONS_TABLE_NUMBER as libc::c_int as
                                 libc::c_uint ||
                             (*(*o).tableentry).type_0 as libc::c_uint ==
                                 OPTIONS_TABLE_KEY as libc::c_int as
                                     libc::c_uint ||
                             (*(*o).tableentry).type_0 as libc::c_uint ==
                                 OPTIONS_TABLE_COLOUR as libc::c_int as
                                     libc::c_uint ||
                             (*(*o).tableentry).type_0 as libc::c_uint ==
                                 OPTIONS_TABLE_ATTRIBUTES as libc::c_int as
                                     libc::c_uint ||
                             (*(*o).tableentry).type_0 as libc::c_uint ==
                                 OPTIONS_TABLE_FLAG as libc::c_int as
                                     libc::c_uint ||
                             (*(*o).tableentry).type_0 as libc::c_uint ==
                                 OPTIONS_TABLE_CHOICE as libc::c_int as
                                     libc::c_uint)) as libc::c_int {
        fatalx(b"option %s is not a number\x00" as *const u8 as
                   *const libc::c_char, name);
    } else { return (*o).unnamed.number };
}
#[no_mangle]
pub unsafe extern "C" fn options_get_style(mut oo: *mut options,
                                           mut name: *const libc::c_char)
 -> *const grid_cell {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get(oo, name);
    if o == 0 as *mut libc::c_void as *mut options_entry {
        fatalx(b"missing option %s\x00" as *const u8 as *const libc::c_char,
               name);
    } else if 0 !=
                  !((*o).tableentry !=
                        0 as *mut libc::c_void as *const options_table_entry
                        &&
                        (*(*o).tableentry).type_0 as libc::c_uint ==
                            OPTIONS_TABLE_STYLE as libc::c_int as
                                libc::c_uint) as libc::c_int {
        fatalx(b"option %s is not a style\x00" as *const u8 as
                   *const libc::c_char, name);
    } else { return &mut (*o).unnamed.style as *mut grid_cell };
}
#[no_mangle]
pub unsafe extern "C" fn options_set_number(mut oo: *mut options,
                                            mut name: *const libc::c_char,
                                            mut value: libc::c_longlong)
 -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    if *name as libc::c_int == 64 {
        fatalx(b"user option %s must be a string\x00" as *const u8 as
                   *const libc::c_char, name);
    } else {
        o = options_get_only(oo, name);
        if o == 0 as *mut libc::c_void as *mut options_entry {
            o = options_default(oo, options_parent_table_entry(oo, name));
            if o == 0 as *mut libc::c_void as *mut options_entry {
                return 0 as *mut options_entry
            }
        }
        if 0 !=
               !((*o).tableentry !=
                     0 as *mut libc::c_void as *const options_table_entry &&
                     ((*(*o).tableentry).type_0 as libc::c_uint ==
                          OPTIONS_TABLE_NUMBER as libc::c_int as libc::c_uint
                          ||
                          (*(*o).tableentry).type_0 as libc::c_uint ==
                              OPTIONS_TABLE_KEY as libc::c_int as libc::c_uint
                          ||
                          (*(*o).tableentry).type_0 as libc::c_uint ==
                              OPTIONS_TABLE_COLOUR as libc::c_int as
                                  libc::c_uint ||
                          (*(*o).tableentry).type_0 as libc::c_uint ==
                              OPTIONS_TABLE_ATTRIBUTES as libc::c_int as
                                  libc::c_uint ||
                          (*(*o).tableentry).type_0 as libc::c_uint ==
                              OPTIONS_TABLE_FLAG as libc::c_int as
                                  libc::c_uint ||
                          (*(*o).tableentry).type_0 as libc::c_uint ==
                              OPTIONS_TABLE_CHOICE as libc::c_int as
                                  libc::c_uint)) as libc::c_int {
            fatalx(b"option %s is not a number\x00" as *const u8 as
                       *const libc::c_char, name);
        } else { (*o).unnamed.number = value; return o }
    };
}
unsafe extern "C" fn options_parent_table_entry(mut oo: *mut options,
                                                mut s: *const libc::c_char)
 -> *const options_table_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    if (*oo).parent == 0 as *mut libc::c_void as *mut options {
        fatalx(b"no parent options for %s\x00" as *const u8 as
                   *const libc::c_char, s);
    } else {
        o = options_get_only((*oo).parent, s);
        if o == 0 as *mut libc::c_void as *mut options_entry {
            fatalx(b"%s not in parent options\x00" as *const u8 as
                       *const libc::c_char, s);
        } else { return (*o).tableentry }
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_set_style(mut oo: *mut options,
                                           mut name: *const libc::c_char,
                                           mut append: libc::c_int,
                                           mut value: *const libc::c_char)
 -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    if *name as libc::c_int == 64 {
        fatalx(b"user option %s must be a string\x00" as *const u8 as
                   *const libc::c_char, name);
    } else {
        o = options_get_only(oo, name);
        if o != 0 as *mut libc::c_void as *mut options_entry && 0 != append &&
               ((*o).tableentry !=
                    0 as *mut libc::c_void as *const options_table_entry &&
                    (*(*o).tableentry).type_0 as libc::c_uint ==
                        OPTIONS_TABLE_STYLE as libc::c_int as libc::c_uint) {
            memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
                   &mut (*o).unnamed.style as *mut grid_cell as
                       *const libc::c_void,
                   ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        } else {
            memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
                   &grid_default_cell as *const grid_cell as
                       *const libc::c_void,
                   ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        }
        if style_parse(&grid_default_cell as *const grid_cell,
                       &mut gc as *mut grid_cell, value) ==
               1i32.wrapping_neg() {
            return 0 as *mut options_entry
        } else {
            if o == 0 as *mut libc::c_void as *mut options_entry {
                o = options_default(oo, options_parent_table_entry(oo, name));
                if o == 0 as *mut libc::c_void as *mut options_entry {
                    return 0 as *mut options_entry
                }
            }
            if 0 !=
                   !((*o).tableentry !=
                         0 as *mut libc::c_void as *const options_table_entry
                         &&
                         (*(*o).tableentry).type_0 as libc::c_uint ==
                             OPTIONS_TABLE_STYLE as libc::c_int as
                                 libc::c_uint) as libc::c_int {
                fatalx(b"option %s is not a style\x00" as *const u8 as
                           *const libc::c_char, name);
            } else {
                memcpy(&mut (*o).unnamed.style as *mut grid_cell as
                           *mut libc::c_void,
                       &mut gc as *mut grid_cell as *const libc::c_void,
                       ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
                return o
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_scope_from_flags(mut args: *mut args,
                                                  mut window: libc::c_int,
                                                  mut fs: *mut cmd_find_state,
                                                  mut oo: *mut *mut options,
                                                  mut cause:
                                                      *mut *mut libc::c_char)
 -> options_table_scope {
    let mut s: *mut session = (*fs).s;
    let mut wl: *mut winlink = (*fs).wl;
    let mut target: *const libc::c_char = args_get(args, 116 as u_char);
    if 0 != args_has(args, 115 as u_char) {
        *oo = global_options;
        return OPTIONS_TABLE_SERVER
    } else if 0 != window || 0 != args_has(args, 119 as u_char) {
        if 0 != args_has(args, 103 as u_char) {
            *oo = global_w_options;
            return OPTIONS_TABLE_WINDOW
        } else if wl == 0 as *mut libc::c_void as *mut winlink {
            if target != 0 as *mut libc::c_void as *const libc::c_char {
                xasprintf(cause,
                          b"no such window: %s\x00" as *const u8 as
                              *const libc::c_char, target);
            } else {
                xasprintf(cause,
                          b"no current window\x00" as *const u8 as
                              *const libc::c_char);
            }
            return OPTIONS_TABLE_NONE
        } else { *oo = (*(*wl).window).options; return OPTIONS_TABLE_WINDOW }
    } else if 0 != args_has(args, 103 as u_char) {
        *oo = global_s_options;
        return OPTIONS_TABLE_SESSION
    } else if s == 0 as *mut libc::c_void as *mut session {
        if target != 0 as *mut libc::c_void as *const libc::c_char {
            xasprintf(cause,
                      b"no such session: %s\x00" as *const u8 as
                          *const libc::c_char, target);
        } else {
            xasprintf(cause,
                      b"no current session\x00" as *const u8 as
                          *const libc::c_char);
        }
        return OPTIONS_TABLE_NONE
    } else { *oo = (*s).options; return OPTIONS_TABLE_SESSION };
}
#[no_mangle]
pub unsafe extern "C" fn options_style_update_new(mut oo: *mut options,
                                                  mut o: *mut options_entry)
 -> () {
    let mut newname: *const libc::c_char = (*(*o).tableentry).style;
    let mut new: *mut options_entry = 0 as *mut options_entry;
    if newname == 0 as *mut libc::c_void as *const libc::c_char {
        return
    } else {
        new = options_get_only(oo, newname);
        if new == 0 as *mut libc::c_void as *mut options_entry {
            new =
                options_set_style(oo, newname, 0i32,
                                  b"default\x00" as *const u8 as
                                      *const libc::c_char)
        }
        if strstr((*o).name, b"-bg\x00" as *const u8 as *const libc::c_char)
               != 0 as *mut libc::c_void as *mut libc::c_char {
            (*new).unnamed.style.bg = (*o).unnamed.number as libc::c_int
        } else if strstr((*o).name,
                         b"-fg\x00" as *const u8 as *const libc::c_char) !=
                      0 as *mut libc::c_void as *mut libc::c_char {
            (*new).unnamed.style.fg = (*o).unnamed.number as libc::c_int
        } else if strstr((*o).name,
                         b"-attr\x00" as *const u8 as *const libc::c_char) !=
                      0 as *mut libc::c_void as *mut libc::c_char {
            (*new).unnamed.style.attr = (*o).unnamed.number as u_short
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_style_update_old(mut oo: *mut options,
                                                  mut o: *mut options_entry)
 -> () {
    let mut newname: [libc::c_char; 128] = [0; 128];
    let mut size: libc::c_int = 0;
    size =
        (*o).name.offset_to(strrchr((*o).name, 45)).expect("bad offset_to") as
            libc::c_long as libc::c_int;
    xsnprintf(newname.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
              b"%.*s-bg\x00" as *const u8 as *const libc::c_char, size,
              (*o).name);
    if options_get(oo, newname.as_mut_ptr()) !=
           0 as *mut libc::c_void as *mut options_entry {
        options_set_number(oo, newname.as_mut_ptr(),
                           (*o).unnamed.style.bg as libc::c_longlong);
    }
    xsnprintf(newname.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
              b"%.*s-fg\x00" as *const u8 as *const libc::c_char, size,
              (*o).name);
    if options_get(oo, newname.as_mut_ptr()) !=
           0 as *mut libc::c_void as *mut options_entry {
        options_set_number(oo, newname.as_mut_ptr(),
                           (*o).unnamed.style.fg as libc::c_longlong);
    }
    xsnprintf(newname.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
              b"%.*s-attr\x00" as *const u8 as *const libc::c_char, size,
              (*o).name);
    if options_get(oo, newname.as_mut_ptr()) !=
           0 as *mut libc::c_void as *mut options_entry {
        options_set_number(oo, newname.as_mut_ptr(),
                           (*o).unnamed.style.attr as libc::c_longlong);
    };
}
unsafe extern "C" fn options_tree_RB_NFIND(mut head: *mut options_tree,
                                           mut elm: *mut options_entry)
 -> *mut options_entry {
    let mut tmp: *mut options_entry = (*head).rbh_root;
    let mut res: *mut options_entry = 0 as *mut options_entry;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = options_cmp(elm, tmp);
            if comp < 0i32 {
                res = tmp;
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return res }
    };
}
unsafe extern "C" fn options_tree_RB_PREV(mut elm: *mut options_entry)
 -> *mut options_entry {
    if !(*elm).entry.rbe_left.is_null() {
        elm = (*elm).entry.rbe_left;
        while !(*elm).entry.rbe_right.is_null() {
            elm = (*elm).entry.rbe_right
        }
    } else if !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_right {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_left {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}

