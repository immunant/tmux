extern crate libc;

use common::timeval;
use client::client;
use cmd::{cmd, mouse_event};
use cmd_queue::{cmdq_item};
use environ::environ;
use proc_::event_base;
use grid::grid_cell;
use session::session;
use window::{winlink, winlink_stack, winlinks, window, windows, window_pane, window_pane_tree, window_mode};

extern "C" {
    pub type tmuxproc;
    pub type screen_titles;
    pub type format_job_tree;
    pub type evbuffer;
    pub type tty_code;
    pub type options;
    pub type args_entry;
    pub type tmuxpeer;
    pub type format_tree;
    pub type bufferevent_ops;
    pub type _IO_FILE_plus;
    pub type input_ctx;
    pub type hooks;
    #[no_mangle]
    fn fnmatch(__pattern: *const libc::c_char, __name: *const libc::c_char,
               __flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn strtonum(_: *const libc::c_char, _: libc::c_longlong,
                _: libc::c_longlong, _: *mut *const libc::c_char)
     -> libc::c_longlong;
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
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
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
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn environ_find(_: *mut environ, _: *const libc::c_char)
     -> *mut environ_entry;
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn session_has(_: *mut session, _: *mut window) -> libc::c_int;
    #[no_mangle]
    fn window_pane_tree_RB_NEXT(_: *mut window_pane) -> *mut window_pane;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn window_pane_tree_RB_MINMAX(_: *mut window_pane_tree, _: libc::c_int)
     -> *mut window_pane;
    #[no_mangle]
    fn session_find(_: *const libc::c_char) -> *mut session;
    #[no_mangle]
    fn session_find_by_id_str(_: *const libc::c_char) -> *mut session;
    #[no_mangle]
    fn winlink_find_by_index(_: *mut winlinks, _: libc::c_int)
     -> *mut winlink;
    #[no_mangle]
    fn winlink_previous_by_number(_: *mut winlink, _: *mut session,
                                  _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn winlink_next_by_number(_: *mut winlink, _: *mut session,
                              _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn window_find_by_id_str(_: *const libc::c_char) -> *mut window;
    #[no_mangle]
    fn window_find_string(_: *mut window, _: *const libc::c_char)
     -> *mut window_pane;
    #[no_mangle]
    fn window_pane_at_index(_: *mut window, _: u_int) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_previous_by_number(_: *mut window, _: *mut window_pane,
                                      _: u_int) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_next_by_number(_: *mut window, _: *mut window_pane,
                                  _: u_int) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_find_right(_: *mut window_pane) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_find_left(_: *mut window_pane) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_find_down(_: *mut window_pane) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_find_up(_: *mut window_pane) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_find_by_id_str(_: *const libc::c_char) -> *mut window_pane;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    fn server_check_marked() -> libc::c_int;
    #[no_mangle]
    fn cmd_mouse_window(_: *mut mouse_event, _: *mut *mut session)
     -> *mut winlink;
    #[no_mangle]
    fn cmd_mouse_pane(_: *mut mouse_event, _: *mut *mut session,
                      _: *mut *mut winlink) -> *mut window_pane;
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn window_has_pane(_: *mut window, _: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn session_alive(_: *mut session) -> libc::c_int;
    #[no_mangle]
    fn session_find_by_id(_: u_int) -> *mut session;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    static mut windows: windows;
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
pub type u_char = __u_char;
pub const JOB_CLOSED: unnamed_15 = 2;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_11,
}
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const JOB_RUNNING: unnamed_15 = 0;
pub type tcflag_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
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
pub struct unnamed_0 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_17,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
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
pub struct unnamed_2 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub type __u_short = libc::c_ushort;
pub const TTY_VT220: unnamed_5 = 3;
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
pub struct unnamed_4 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub type u_int = __u_int;
pub type unnamed_5 = libc::c_uint;
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
}
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub ev_io_next: unnamed_23,
    pub ev_timeout: timeval,
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
pub const TTY_UNKNOWN: unnamed_5 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type size_t = libc::c_ulong;
pub type layout_type = libc::c_uint;
pub type __u_int = libc::c_uint;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_35,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_9 {
    ev_io: unnamed_7,
    ev_signal: unnamed_34,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_11 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub type unnamed_15 = libc::c_uint;
pub type __off64_t = libc::c_long;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
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
pub struct unnamed_16 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type cmdq_type = libc::c_uint;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const TTY_VT101: unnamed_5 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_18 {
    ev_next_with_common_timeout: unnamed_37,
    min_heap_idx: libc::c_int,
}
pub const TTY_VT420: unnamed_5 = 5;
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
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_29,
}
pub type unnamed_19 = libc::c_uint;
pub const JOB_DEAD: unnamed_15 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_19 = 2;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub type uint32_t = libc::c_uint;
pub type options_table_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_10,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTY_VT102: unnamed_5 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const PROMPT_COMMAND: unnamed_38 = 1;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_21,
    pub entry: unnamed_24,
}
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
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
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_14,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct environ_entry {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub entry: unnamed_6,
}
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
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
    pub term_type: unnamed_5,
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
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub type key_code = libc::c_ulonglong;
pub type cmd_find_type = libc::c_uint;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_19 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_20,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const TTY_VT100: unnamed_5 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
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
    pub entry: unnamed_0,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type time_t = __time_t;
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
pub struct job {
    pub state: unnamed_15,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
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
pub struct unnamed_33 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
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
pub const LINE_SEL_NONE: unnamed_19 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub ev_signal_next: unnamed_12,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub type __pid_t = libc::c_int;
pub const TTY_VT320: unnamed_5 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_19,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_35 {
    offset: u_int,
    data: unnamed_13,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
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
pub struct event {
    pub ev_active_next: unnamed_25,
    pub ev_next: unnamed_2,
    pub ev_timeout_pos: unnamed_18,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_9,
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
pub struct unnamed_37 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub const PROMPT_ENTRY: unnamed_38 = 0;
pub type unnamed_38 = libc::c_uint;
pub type __time_t = libc::c_long;
pub type pid_t = __pid_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub type __suseconds_t = libc::c_long;
#[no_mangle]
pub unsafe extern "C" fn cmd_find_target(mut fs: *mut cmd_find_state,
                                         mut item: *mut cmdq_item,
                                         mut target: *const libc::c_char,
                                         mut type_0: cmd_find_type,
                                         mut flags: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut m: *mut mouse_event = 0 as *mut mouse_event;
    let mut current: cmd_find_state =
        cmd_find_state{flags: 0,
                       current: 0 as *mut cmd_find_state,
                       s: 0 as *mut session,
                       wl: 0 as *mut winlink,
                       w: 0 as *mut window,
                       wp: 0 as *mut window_pane,
                       idx: 0,};
    let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut period: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut session: *const libc::c_char = 0 as *const libc::c_char;
    let mut window: *const libc::c_char = 0 as *const libc::c_char;
    let mut pane: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut window_only: libc::c_int = 0i32;
    let mut pane_only: libc::c_int = 0i32;
    if 0 != flags & 64i32 { flags |= 2i32 }
    if type_0 as libc::c_uint == CMD_FIND_PANE as libc::c_int as libc::c_uint
       {
        s = b"pane\x00" as *const u8 as *const libc::c_char
    } else if type_0 as libc::c_uint ==
                  CMD_FIND_WINDOW as libc::c_int as libc::c_uint {
        s = b"window\x00" as *const u8 as *const libc::c_char
    } else if type_0 as libc::c_uint ==
                  CMD_FIND_SESSION as libc::c_int as libc::c_uint {
        s = b"session\x00" as *const u8 as *const libc::c_char
    } else { s = b"unknown\x00" as *const u8 as *const libc::c_char }
    if target == 0 as *mut libc::c_void as *const libc::c_char {
        log_debug(b"%s: target none, type %s\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"cmd_find_target\x00")).as_ptr(),
                  s);
    } else {
        log_debug(b"%s: target %s, type %s\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"cmd_find_target\x00")).as_ptr(),
                  target, s);
    }
    log_debug(b"%s: item %p, flags %#x\x00" as *const u8 as
                  *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"cmd_find_target\x00")).as_ptr(),
              item, flags);
    cmd_find_clear_state(fs, flags);
    if 0 != server_check_marked() && 0 != flags & 8i32 {
        (*fs).current = &mut marked_pane as *mut cmd_find_state;
        log_debug(b"%s: current is marked pane\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"cmd_find_target\x00")).as_ptr());
        current_block = 7095457783677275021;
    } else if 0 !=
                  cmd_find_valid_state(&mut (*(*item).shared).current as
                                           *mut cmd_find_state) {
        (*fs).current = &mut (*(*item).shared).current as *mut cmd_find_state;
        log_debug(b"%s: current is from queue\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"cmd_find_target\x00")).as_ptr());
        current_block = 7095457783677275021;
    } else if cmd_find_from_client(&mut current as *mut cmd_find_state,
                                   (*item).client, flags) == 0i32 {
        (*fs).current = &mut current as *mut cmd_find_state;
        log_debug(b"%s: current is from client\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"cmd_find_target\x00")).as_ptr());
        current_block = 7095457783677275021;
    } else if 0 != !flags & 2i32 {
        cmdq_error(item,
                   b"no current target\x00" as *const u8 as
                       *const libc::c_char);
        current_block = 7631833770802127844;
    } else { current_block = 7631833770802127844; }
    match current_block {
        7095457783677275021 => {
            if 0 == cmd_find_valid_state((*fs).current) {
                fatalx(b"invalid current find state\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                if target == 0 as *mut libc::c_void as *const libc::c_char ||
                       *target as libc::c_int == 0 {
                    current_block = 16547183354321368147;
                } else if strcmp(target,
                                 b"=\x00" as *const u8 as *const libc::c_char)
                              == 0i32 ||
                              strcmp(target,
                                     b"{mouse}\x00" as *const u8 as
                                         *const libc::c_char) == 0i32 {
                    m = &mut (*(*item).shared).mouse as *mut mouse_event;
                    match type_0 as libc::c_uint {
                        0 => {
                            (*fs).wp =
                                cmd_mouse_pane(m,
                                               &mut (*fs).s as
                                                   *mut *mut session,
                                               &mut (*fs).wl as
                                                   *mut *mut winlink);
                            if (*fs).wp !=
                                   0 as *mut libc::c_void as *mut window_pane
                               {
                                (*fs).w = (*(*fs).wl).window
                            }
                        }
                        1 | 2 => {
                            (*fs).wl =
                                cmd_mouse_window(m,
                                                 &mut (*fs).s as
                                                     *mut *mut session);
                            if (*fs).wl !=
                                   0 as *mut libc::c_void as *mut winlink {
                                (*fs).w = (*(*fs).wl).window;
                                (*fs).wp = (*(*fs).w).active
                            }
                        }
                        _ => { }
                    }
                    if (*fs).wp == 0 as *mut libc::c_void as *mut window_pane
                       {
                        if 0 != !flags & 2i32 {
                            cmdq_error(item,
                                       b"no mouse target\x00" as *const u8 as
                                           *const libc::c_char);
                            current_block = 7631833770802127844;
                        } else { current_block = 7631833770802127844; }
                    } else { current_block = 6366362408155432276; }
                } else if strcmp(target,
                                 b"~\x00" as *const u8 as *const libc::c_char)
                              == 0i32 ||
                              strcmp(target,
                                     b"{marked}\x00" as *const u8 as
                                         *const libc::c_char) == 0i32 {
                    if 0 == server_check_marked() {
                        if 0 != !flags & 2i32 {
                            cmdq_error(item,
                                       b"no marked target\x00" as *const u8 as
                                           *const libc::c_char);
                            current_block = 7631833770802127844;
                        } else { current_block = 7631833770802127844; }
                    } else {
                        cmd_find_copy_state(fs,
                                            &mut marked_pane as
                                                *mut cmd_find_state);
                        current_block = 6366362408155432276;
                    }
                } else {
                    copy = xstrdup(target);
                    colon = strchr(copy, 58);
                    if colon != 0 as *mut libc::c_void as *mut libc::c_char {
                        let fresh0 = colon;
                        colon = colon.offset(1);
                        *fresh0 = 0 as libc::c_char
                    }
                    if colon == 0 as *mut libc::c_void as *mut libc::c_char {
                        period = strchr(copy, 46)
                    } else { period = strchr(colon, 46) }
                    if period != 0 as *mut libc::c_void as *mut libc::c_char {
                        let fresh1 = period;
                        period = period.offset(1);
                        *fresh1 = 0 as libc::c_char
                    }
                    pane = 0 as *const libc::c_char;
                    window = pane;
                    session = window;
                    if colon != 0 as *mut libc::c_void as *mut libc::c_char &&
                           period !=
                               0 as *mut libc::c_void as *mut libc::c_char {
                        session = copy;
                        window = colon;
                        window_only = 1i32;
                        pane = period;
                        pane_only = 1i32
                    } else if colon !=
                                  0 as *mut libc::c_void as *mut libc::c_char
                                  &&
                                  period ==
                                      0 as *mut libc::c_void as
                                          *mut libc::c_char {
                        session = copy;
                        window = colon;
                        window_only = 1i32
                    } else if colon ==
                                  0 as *mut libc::c_void as *mut libc::c_char
                                  &&
                                  period !=
                                      0 as *mut libc::c_void as
                                          *mut libc::c_char {
                        window = copy;
                        pane = period;
                        pane_only = 1i32
                    } else if *copy as libc::c_int == 36 {
                        session = copy
                    } else if *copy as libc::c_int == 64 {
                        window = copy
                    } else if *copy as libc::c_int == 37 {
                        pane = copy
                    } else {
                        match type_0 as libc::c_uint {
                            2 => {
                                current_block = 11448667071180660676;
                                match current_block {
                                    17720461952361946060 => { pane = copy }
                                    11448667071180660676 => { session = copy }
                                    _ => { window = copy }
                                }
                            }
                            1 => {
                                current_block = 13390047745021094676;
                                match current_block {
                                    17720461952361946060 => { pane = copy }
                                    11448667071180660676 => { session = copy }
                                    _ => { window = copy }
                                }
                            }
                            0 => {
                                current_block = 17720461952361946060;
                                match current_block {
                                    17720461952361946060 => { pane = copy }
                                    11448667071180660676 => { session = copy }
                                    _ => { window = copy }
                                }
                            }
                            _ => { }
                        }
                    }
                    if session !=
                           0 as *mut libc::c_void as *const libc::c_char &&
                           *session as libc::c_int == 61 {
                        session = session.offset(1isize);
                        (*fs).flags |= 16i32
                    }
                    if window != 0 as *mut libc::c_void as *const libc::c_char
                           && *window as libc::c_int == 61 {
                        window = window.offset(1isize);
                        (*fs).flags |= 32i32
                    }
                    if session !=
                           0 as *mut libc::c_void as *const libc::c_char &&
                           *session as libc::c_int == 0 {
                        session = 0 as *const libc::c_char
                    }
                    if window != 0 as *mut libc::c_void as *const libc::c_char
                           && *window as libc::c_int == 0 {
                        window = 0 as *const libc::c_char
                    }
                    if pane != 0 as *mut libc::c_void as *const libc::c_char
                           && *pane as libc::c_int == 0 {
                        pane = 0 as *const libc::c_char
                    }
                    if session !=
                           0 as *mut libc::c_void as *const libc::c_char {
                        session =
                            cmd_find_map_table(cmd_find_session_table.as_mut_ptr(),
                                               session)
                    }
                    if window != 0 as *mut libc::c_void as *const libc::c_char
                       {
                        window =
                            cmd_find_map_table(cmd_find_window_table.as_mut_ptr(),
                                               window)
                    }
                    if pane != 0 as *mut libc::c_void as *const libc::c_char {
                        pane =
                            cmd_find_map_table(cmd_find_pane_table.as_mut_ptr(),
                                               pane)
                    }
                    log_debug(b"%s: target %s (flags %#x): session=%s, window=%s, pane=%s\x00"
                                  as *const u8 as *const libc::c_char,
                              (*::std::mem::transmute::<&[u8; 16],
                                                        &[libc::c_char; 16]>(b"cmd_find_target\x00")).as_ptr(),
                              target, flags,
                              if session ==
                                     0 as *mut libc::c_void as
                                         *const libc::c_char {
                                  b"none\x00" as *const u8 as
                                      *const libc::c_char
                              } else { session },
                              if window ==
                                     0 as *mut libc::c_void as
                                         *const libc::c_char {
                                  b"none\x00" as *const u8 as
                                      *const libc::c_char
                              } else { window },
                              if pane ==
                                     0 as *mut libc::c_void as
                                         *const libc::c_char {
                                  b"none\x00" as *const u8 as
                                      *const libc::c_char
                              } else { pane });
                    if pane != 0 as *mut libc::c_void as *const libc::c_char
                           && 0 != flags & 4i32 {
                        if 0 != !flags & 2i32 {
                            cmdq_error(item,
                                       b"can\'t specify pane here\x00" as
                                           *const u8 as *const libc::c_char);
                            current_block = 7631833770802127844;
                        } else { current_block = 7631833770802127844; }
                    } else {
                        if session !=
                               0 as *mut libc::c_void as *const libc::c_char {
                            if cmd_find_get_session(fs, session) != 0i32 {
                                if 0 != !flags & 2i32 {
                                    cmdq_error(item,
                                               b"can\'t find session %s\x00"
                                                   as *const u8 as
                                                   *const libc::c_char,
                                               session);
                                    current_block = 7631833770802127844;
                                } else {
                                    current_block = 7631833770802127844;
                                }
                            } else if window ==
                                          0 as *mut libc::c_void as
                                              *const libc::c_char &&
                                          pane ==
                                              0 as *mut libc::c_void as
                                                  *const libc::c_char {
                                (*fs).wl = (*(*fs).s).curw;
                                (*fs).idx = 1i32.wrapping_neg();
                                (*fs).w = (*(*fs).wl).window;
                                (*fs).wp = (*(*fs).w).active;
                                current_block = 6366362408155432276;
                            } else if window !=
                                          0 as *mut libc::c_void as
                                              *const libc::c_char &&
                                          pane ==
                                              0 as *mut libc::c_void as
                                                  *const libc::c_char {
                                if cmd_find_get_window_with_session(fs,
                                                                    window) !=
                                       0i32 {
                                    current_block = 2196860260205631074;
                                } else {
                                    (*fs).wp = (*(*(*fs).wl).window).active;
                                    current_block = 6366362408155432276;
                                }
                            } else if window ==
                                          0 as *mut libc::c_void as
                                              *const libc::c_char &&
                                          pane !=
                                              0 as *mut libc::c_void as
                                                  *const libc::c_char {
                                if cmd_find_get_pane_with_session(fs, pane) !=
                                       0i32 {
                                    current_block = 6298902569995320835;
                                } else {
                                    current_block = 6366362408155432276;
                                }
                            } else if cmd_find_get_window_with_session(fs,
                                                                       window)
                                          != 0i32 {
                                current_block = 2196860260205631074;
                            } else if cmd_find_get_pane_with_window(fs, pane)
                                          != 0i32 {
                                current_block = 6298902569995320835;
                            } else { current_block = 6366362408155432276; }
                        } else if window !=
                                      0 as *mut libc::c_void as
                                          *const libc::c_char &&
                                      pane !=
                                          0 as *mut libc::c_void as
                                              *const libc::c_char {
                            if cmd_find_get_window(fs, window, window_only) !=
                                   0i32 {
                                current_block = 2196860260205631074;
                            } else if cmd_find_get_pane_with_window(fs, pane)
                                          != 0i32 {
                                current_block = 6298902569995320835;
                            } else { current_block = 6366362408155432276; }
                        } else if window !=
                                      0 as *mut libc::c_void as
                                          *const libc::c_char &&
                                      pane ==
                                          0 as *mut libc::c_void as
                                              *const libc::c_char {
                            if cmd_find_get_window(fs, window, window_only) !=
                                   0i32 {
                                current_block = 2196860260205631074;
                            } else {
                                (*fs).wp = (*(*(*fs).wl).window).active;
                                current_block = 6366362408155432276;
                            }
                        } else if window ==
                                      0 as *mut libc::c_void as
                                          *const libc::c_char &&
                                      pane !=
                                          0 as *mut libc::c_void as
                                              *const libc::c_char {
                            if cmd_find_get_pane(fs, pane, pane_only) != 0i32
                               {
                                current_block = 6298902569995320835;
                            } else { current_block = 6366362408155432276; }
                        } else { current_block = 16547183354321368147; }
                        match current_block {
                            16547183354321368147 => { }
                            6366362408155432276 => { }
                            7631833770802127844 => { }
                            _ => {
                                match current_block {
                                    6298902569995320835 => {
                                        if 0 != !flags & 2i32 {
                                            cmdq_error(item,
                                                       b"can\'t find pane %s\x00"
                                                           as *const u8 as
                                                           *const libc::c_char,
                                                       pane);
                                        }
                                    }
                                    _ => {
                                        if 0 != !flags & 2i32 {
                                            cmdq_error(item,
                                                       b"can\'t find window %s\x00"
                                                           as *const u8 as
                                                           *const libc::c_char,
                                                       window);
                                        }
                                    }
                                }
                                current_block = 7631833770802127844;
                            }
                        }
                    }
                }
                match current_block {
                    7631833770802127844 => { }
                    _ => {
                        match current_block {
                            16547183354321368147 => {
                                cmd_find_copy_state(fs, (*fs).current);
                                if 0 != flags & 4i32 {
                                    (*fs).idx = 1i32.wrapping_neg()
                                }
                            }
                            _ => { }
                        }
                        (*fs).current = 0 as *mut cmd_find_state;
                        cmd_find_log_state((*::std::mem::transmute::<&[u8; 16],
                                                                     &[libc::c_char; 16]>(b"cmd_find_target\x00")).as_ptr(),
                                           fs);
                        free(copy as *mut libc::c_void);
                        return 0i32
                    }
                }
            }
        }
        _ => { }
    }
    (*fs).current = 0 as *mut cmd_find_state;
    log_debug(b"%s: error\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"cmd_find_target\x00")).as_ptr());
    free(copy as *mut libc::c_void);
    if 0 != flags & 64i32 { return 0i32 } else { return 1i32.wrapping_neg() };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_log_state(mut prefix: *const libc::c_char,
                                            mut fs: *mut cmd_find_state)
 -> () {
    if (*fs).s != 0 as *mut libc::c_void as *mut session {
        log_debug(b"%s: s=$%u\x00" as *const u8 as *const libc::c_char,
                  prefix, (*(*fs).s).id);
    } else {
        log_debug(b"%s: s=none\x00" as *const u8 as *const libc::c_char,
                  prefix);
    }
    if (*fs).wl != 0 as *mut libc::c_void as *mut winlink {
        log_debug(b"%s: wl=%u %d w=@%u %s\x00" as *const u8 as
                      *const libc::c_char, prefix, (*(*fs).wl).idx,
                  ((*(*fs).wl).window == (*fs).w) as libc::c_int,
                  (*(*fs).w).id, (*(*fs).w).name);
    } else {
        log_debug(b"%s: wl=none\x00" as *const u8 as *const libc::c_char,
                  prefix);
    }
    if (*fs).wp != 0 as *mut libc::c_void as *mut window_pane {
        log_debug(b"%s: wp=%%%u\x00" as *const u8 as *const libc::c_char,
                  prefix, (*(*fs).wp).id);
    } else {
        log_debug(b"%s: wp=none\x00" as *const u8 as *const libc::c_char,
                  prefix);
    }
    if (*fs).idx != 1i32.wrapping_neg() {
        log_debug(b"%s: idx=%d\x00" as *const u8 as *const libc::c_char,
                  prefix, (*fs).idx);
    } else {
        log_debug(b"%s: idx=none\x00" as *const u8 as *const libc::c_char,
                  prefix);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_copy_state(mut dst: *mut cmd_find_state,
                                             mut src: *mut cmd_find_state)
 -> () {
    (*dst).s = (*src).s;
    (*dst).wl = (*src).wl;
    (*dst).idx = (*src).idx;
    (*dst).w = (*src).w;
    (*dst).wp = (*src).wp;
}
unsafe extern "C" fn cmd_find_get_pane(mut fs: *mut cmd_find_state,
                                       mut pane: *const libc::c_char,
                                       mut only: libc::c_int) -> libc::c_int {
    log_debug(b"%s: %s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"cmd_find_get_pane\x00")).as_ptr(),
              pane);
    if *pane as libc::c_int == 37 {
        (*fs).wp = window_pane_find_by_id_str(pane);
        if (*fs).wp == 0 as *mut libc::c_void as *mut window_pane {
            return 1i32.wrapping_neg()
        } else {
            (*fs).w = (*(*fs).wp).window;
            return cmd_find_best_session_with_window(fs)
        }
    } else {
        (*fs).s = (*(*fs).current).s;
        (*fs).wl = (*(*fs).current).wl;
        (*fs).idx = (*(*fs).current).idx;
        (*fs).w = (*(*fs).current).w;
        if cmd_find_get_pane_with_window(fs, pane) == 0i32 {
            return 0i32
        } else if 0 == only && cmd_find_get_window(fs, pane, 0i32) == 0i32 {
            (*fs).wp = (*(*fs).w).active;
            return 0i32
        } else { return 1i32.wrapping_neg() }
    };
}
unsafe extern "C" fn cmd_find_get_window(mut fs: *mut cmd_find_state,
                                         mut window: *const libc::c_char,
                                         mut only: libc::c_int)
 -> libc::c_int {
    log_debug(b"%s: %s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"cmd_find_get_window\x00")).as_ptr(),
              window);
    if *window as libc::c_int == 64 {
        (*fs).w = window_find_by_id_str(window);
        if (*fs).w == 0 as *mut libc::c_void as *mut window {
            return 1i32.wrapping_neg()
        } else { return cmd_find_best_session_with_window(fs) }
    } else {
        (*fs).s = (*(*fs).current).s;
        if cmd_find_get_window_with_session(fs, window) == 0i32 {
            return 0i32
        } else if 0 == only && cmd_find_get_session(fs, window) == 0i32 {
            (*fs).wl = (*(*fs).s).curw;
            (*fs).w = (*(*fs).wl).window;
            if 0 != !(*fs).flags & 4i32 { (*fs).idx = (*(*fs).wl).idx }
            return 0i32
        } else { return 1i32.wrapping_neg() }
    };
}
unsafe extern "C" fn cmd_find_get_session(mut fs: *mut cmd_find_state,
                                          mut session: *const libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut s: *mut session = 0 as *mut session;
    let mut s_loop: *mut session = 0 as *mut session;
    let mut c: *mut client = 0 as *mut client;
    log_debug(b"%s: %s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"cmd_find_get_session\x00")).as_ptr(),
              session);
    if *session as libc::c_int == 36 {
        (*fs).s = session_find_by_id_str(session);
        if (*fs).s == 0 as *mut libc::c_void as *mut session {
            return 1i32.wrapping_neg()
        } else { return 0i32 }
    } else {
        (*fs).s = session_find(session);
        if (*fs).s != 0 as *mut libc::c_void as *mut session {
            return 0i32
        } else {
            c = cmd_find_client(0 as *mut cmdq_item, session, 1i32);
            if c != 0 as *mut libc::c_void as *mut client &&
                   (*c).session != 0 as *mut libc::c_void as *mut session {
                (*fs).s = (*c).session;
                return 0i32
            } else if 0 != (*fs).flags & 16i32 {
                return 1i32.wrapping_neg()
            } else {
                s = 0 as *mut session;
                s_loop =
                    sessions_RB_MINMAX(&mut sessions as *mut sessions,
                                       1i32.wrapping_neg());
                loop  {
                    if s_loop != 0 as *mut libc::c_void as *mut session {
                        if strncmp(session, (*s_loop).name, strlen(session))
                               == 0i32 {
                            if s != 0 as *mut libc::c_void as *mut session {
                                return 1i32.wrapping_neg()
                            } else { s = s_loop }
                        }
                        s_loop = sessions_RB_NEXT(s_loop)
                    } else if s != 0 as *mut libc::c_void as *mut session {
                        current_block = 1394248824506584008;
                        break ;
                    } else { current_block = 3276175668257526147; break ; }
                }
                match current_block {
                    3276175668257526147 => {
                        s = 0 as *mut session;
                        s_loop =
                            sessions_RB_MINMAX(&mut sessions as *mut sessions,
                                               1i32.wrapping_neg());
                        loop  {
                            if s_loop !=
                                   0 as *mut libc::c_void as *mut session {
                                if fnmatch(session, (*s_loop).name, 0i32) ==
                                       0i32 {
                                    if s !=
                                           0 as *mut libc::c_void as
                                               *mut session {
                                        return 1i32.wrapping_neg()
                                    } else { s = s_loop }
                                }
                                s_loop = sessions_RB_NEXT(s_loop)
                            } else if s !=
                                          0 as *mut libc::c_void as
                                              *mut session {
                                current_block = 12349973810996921269;
                                break ;
                            } else {
                                current_block = 3512920355445576850;
                                break ;
                            }
                        }
                        match current_block {
                            3512920355445576850 => {
                                return 1i32.wrapping_neg()
                            }
                            _ => { (*fs).s = s; return 0i32 }
                        }
                    }
                    _ => { (*fs).s = s; return 0i32 }
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_client(mut item: *mut cmdq_item,
                                         mut target: *const libc::c_char,
                                         mut quiet: libc::c_int)
 -> *mut client {
    let mut c: *mut client = 0 as *mut client;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    if target == 0 as *mut libc::c_void as *const libc::c_char {
        return cmd_find_current_client(item, quiet)
    } else {
        copy = xstrdup(target);
        size = strlen(copy);
        if size != 0i32 as libc::c_ulong &&
               *copy.offset(size.wrapping_sub(1i32 as libc::c_ulong) as isize)
                   as libc::c_int == 58 {
            *copy.offset(size.wrapping_sub(1i32 as libc::c_ulong) as isize) =
                0 as libc::c_char
        }
        c = (*(&mut clients as *mut clients)).tqh_first;
        while c != 0 as *mut libc::c_void as *mut client {
            if !((*c).session == 0 as *mut libc::c_void as *mut session) {
                if strcmp(copy, (*c).name) == 0i32 { break ; }
                if !(*(*c).ttyname as libc::c_int == 0) {
                    if strcmp(copy, (*c).ttyname) == 0i32 { break ; }
                    if !(strncmp((*c).ttyname,
                                 b"/dev/\x00" as *const u8 as
                                     *const libc::c_char,
                                 (::std::mem::size_of::<[libc::c_char; 6]>()
                                      as
                                      libc::c_ulong).wrapping_sub(1i32 as
                                                                      libc::c_ulong))
                             != 0i32) {
                        if strcmp(copy,
                                  (*c).ttyname.offset(::std::mem::size_of::<[libc::c_char; 6]>()
                                                          as libc::c_ulong as
                                                          isize).offset(-1isize))
                               == 0i32 {
                            break ;
                        }
                    }
                }
            }
            c = (*c).entry.tqe_next
        }
        if c == 0 as *mut libc::c_void as *mut client && 0 == quiet {
            cmdq_error(item,
                       b"can\'t find client %s\x00" as *const u8 as
                           *const libc::c_char, copy);
        }
        free(copy as *mut libc::c_void);
        log_debug(b"%s: target %s, return %p\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"cmd_find_client\x00")).as_ptr(),
                  target, c);
        return c
    };
}
unsafe extern "C" fn cmd_find_current_client(mut item: *mut cmdq_item,
                                             mut quiet: libc::c_int)
 -> *mut client {
    let mut c: *mut client = 0 as *mut client;
    let mut s: *mut session = 0 as *mut session;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut fs: cmd_find_state =
        cmd_find_state{flags: 0,
                       current: 0 as *mut cmd_find_state,
                       s: 0 as *mut session,
                       wl: 0 as *mut winlink,
                       w: 0 as *mut window,
                       wp: 0 as *mut window_pane,
                       idx: 0,};
    if (*item).client != 0 as *mut libc::c_void as *mut client &&
           (*(*item).client).session != 0 as *mut libc::c_void as *mut session
       {
        return (*item).client
    } else {
        c = 0 as *mut client;
        wp = cmd_find_inside_pane((*item).client);
        if wp != 0 as *mut libc::c_void as *mut window_pane {
            cmd_find_clear_state(&mut fs as *mut cmd_find_state, 2i32);
            fs.w = (*wp).window;
            if cmd_find_best_session_with_window(&mut fs as
                                                     *mut cmd_find_state) ==
                   0i32 {
                c = cmd_find_best_client(fs.s)
            }
        } else {
            s =
                cmd_find_best_session(0 as *mut *mut session, 0i32 as u_int,
                                      2i32);
            if s != 0 as *mut libc::c_void as *mut session {
                c = cmd_find_best_client(s)
            }
        }
        if c == 0 as *mut libc::c_void as *mut client && 0 == quiet {
            cmdq_error(item,
                       b"no current client\x00" as *const u8 as
                           *const libc::c_char);
        }
        log_debug(b"%s: no target, return %p\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"cmd_find_current_client\x00")).as_ptr(),
                  c);
        return c
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_best_client(mut s: *mut session)
 -> *mut client {
    let mut c_loop: *mut client = 0 as *mut client;
    let mut c: *mut client = 0 as *mut client;
    if 0 != (*s).flags & 1i32 { s = 0 as *mut session }
    c = 0 as *mut client;
    c_loop = (*(&mut clients as *mut clients)).tqh_first;
    while c_loop != 0 as *mut libc::c_void as *mut client {
        if !((*c_loop).session == 0 as *mut libc::c_void as *mut session) {
            if !(s != 0 as *mut libc::c_void as *mut session &&
                     (*c_loop).session != s) {
                if 0 != cmd_find_client_better(c_loop, c) { c = c_loop }
            }
        }
        c_loop = (*c_loop).entry.tqe_next
    }
    return c;
}
unsafe extern "C" fn cmd_find_client_better(mut c: *mut client,
                                            mut than: *mut client)
 -> libc::c_int {
    if than == 0 as *mut libc::c_void as *mut client {
        return 1i32
    } else {
        return if (*(&mut (*c).activity_time as *mut timeval)).tv_sec ==
                      (*(&mut (*than).activity_time as *mut timeval)).tv_sec {
                   ((*(&mut (*c).activity_time as *mut timeval)).tv_usec >
                        (*(&mut (*than).activity_time as
                               *mut timeval)).tv_usec) as libc::c_int
               } else {
                   ((*(&mut (*c).activity_time as *mut timeval)).tv_sec >
                        (*(&mut (*than).activity_time as
                               *mut timeval)).tv_sec) as libc::c_int
               }
    };
}
unsafe extern "C" fn cmd_find_best_session(mut slist: *mut *mut session,
                                           mut ssize: u_int,
                                           mut flags: libc::c_int)
 -> *mut session {
    let mut s_loop: *mut session = 0 as *mut session;
    let mut s: *mut session = 0 as *mut session;
    let mut i: u_int = 0;
    s = 0 as *mut session;
    if slist != 0 as *mut libc::c_void as *mut *mut session {
        i = 0i32 as u_int;
        while i < ssize {
            if 0 !=
                   cmd_find_session_better(*slist.offset(i as isize), s,
                                           flags) {
                s = *slist.offset(i as isize)
            }
            i = i.wrapping_add(1)
        }
    } else {
        s_loop =
            sessions_RB_MINMAX(&mut sessions as *mut sessions,
                               1i32.wrapping_neg());
        while s_loop != 0 as *mut libc::c_void as *mut session {
            if 0 != cmd_find_session_better(s_loop, s, flags) { s = s_loop }
            s_loop = sessions_RB_NEXT(s_loop)
        }
    }
    return s;
}
unsafe extern "C" fn cmd_find_session_better(mut s: *mut session,
                                             mut than: *mut session,
                                             mut flags: libc::c_int)
 -> libc::c_int {
    let mut attached: libc::c_int = 0;
    if than == 0 as *mut libc::c_void as *mut session {
        return 1i32
    } else {
        if 0 != flags & 1i32 {
            attached = !(*than).flags & 1i32;
            if 0 != attached && 0 != (*s).flags & 1i32 {
                return 1i32
            } else if 0 == attached && 0 != !(*s).flags & 1i32 { return 0i32 }
        }
        return if (*(&mut (*s).activity_time as *mut timeval)).tv_sec ==
                      (*(&mut (*than).activity_time as *mut timeval)).tv_sec {
                   ((*(&mut (*s).activity_time as *mut timeval)).tv_usec >
                        (*(&mut (*than).activity_time as
                               *mut timeval)).tv_usec) as libc::c_int
               } else {
                   ((*(&mut (*s).activity_time as *mut timeval)).tv_sec >
                        (*(&mut (*than).activity_time as
                               *mut timeval)).tv_sec) as libc::c_int
               }
    };
}
unsafe extern "C" fn cmd_find_best_session_with_window(mut fs:
                                                           *mut cmd_find_state)
 -> libc::c_int {
    let mut slist: *mut *mut session = 0 as *mut *mut session;
    let mut ssize: u_int = 0;
    let mut s: *mut session = 0 as *mut session;
    ssize = 0i32 as u_int;
    s =
        sessions_RB_MINMAX(&mut sessions as *mut sessions,
                           1i32.wrapping_neg());
    while s != 0 as *mut libc::c_void as *mut session {
        if !(0 == session_has(s, (*fs).w)) {
            slist =
                xreallocarray(slist as *mut libc::c_void,
                              ssize.wrapping_add(1i32 as libc::c_uint) as
                                  size_t,
                              ::std::mem::size_of::<*mut session>() as
                                  libc::c_ulong) as *mut *mut session;
            let fresh2 = ssize;
            ssize = ssize.wrapping_add(1);
            let ref mut fresh3 = *slist.offset(fresh2 as isize);
            *fresh3 = s
        }
        s = sessions_RB_NEXT(s)
    }
    if !(ssize == 0i32 as libc::c_uint) {
        (*fs).s = cmd_find_best_session(slist, ssize, (*fs).flags);
        if !((*fs).s == 0 as *mut libc::c_void as *mut session) {
            free(slist as *mut libc::c_void);
            return cmd_find_best_winlink_with_window(fs)
        }
    }
    free(slist as *mut libc::c_void);
    return 1i32.wrapping_neg();
}
unsafe extern "C" fn cmd_find_best_winlink_with_window(mut fs:
                                                           *mut cmd_find_state)
 -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wl_loop: *mut winlink = 0 as *mut winlink;
    wl = 0 as *mut winlink;
    if (*(*fs).s).curw != 0 as *mut libc::c_void as *mut winlink &&
           (*(*(*fs).s).curw).window == (*fs).w {
        wl = (*(*fs).s).curw
    } else {
        wl_loop =
            winlinks_RB_MINMAX(&mut (*(*fs).s).windows as *mut winlinks,
                               1i32.wrapping_neg());
        while wl_loop != 0 as *mut libc::c_void as *mut winlink {
            if (*wl_loop).window == (*fs).w {
                wl = wl_loop;
                break ;
            } else { wl_loop = winlinks_RB_NEXT(wl_loop) }
        }
    }
    if wl == 0 as *mut libc::c_void as *mut winlink {
        return 1i32.wrapping_neg()
    } else { (*fs).wl = wl; (*fs).idx = (*(*fs).wl).idx; return 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_clear_state(mut fs: *mut cmd_find_state,
                                              mut flags: libc::c_int) -> () {
    memset(fs as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<cmd_find_state>() as libc::c_ulong);
    (*fs).flags = flags;
    (*fs).idx = 1i32.wrapping_neg();
}
unsafe extern "C" fn cmd_find_inside_pane(mut c: *mut client)
 -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if c == 0 as *mut libc::c_void as *mut client {
        return 0 as *mut window_pane
    } else {
        wp =
            window_pane_tree_RB_MINMAX(&mut all_window_panes as
                                           *mut window_pane_tree,
                                       1i32.wrapping_neg());
        while wp != 0 as *mut libc::c_void as *mut window_pane {
            if strcmp((*wp).tty.as_mut_ptr(), (*c).ttyname) == 0i32 {
                break ;
            }
            wp = window_pane_tree_RB_NEXT(wp)
        }
        return wp
    };
}
unsafe extern "C" fn cmd_find_get_window_with_session(mut fs:
                                                          *mut cmd_find_state,
                                                      mut window:
                                                          *const libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut idx: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut exact: libc::c_int = 0;
    let mut s: *mut session = 0 as *mut session;
    log_debug(b"%s: %s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 33],
                                        &[libc::c_char; 33]>(b"cmd_find_get_window_with_session\x00")).as_ptr(),
              window);
    exact = (*fs).flags & 32i32;
    (*fs).wl = (*(*fs).s).curw;
    (*fs).w = (*(*fs).wl).window;
    if *window as libc::c_int == 64 {
        (*fs).w = window_find_by_id_str(window);
        if (*fs).w == 0 as *mut libc::c_void as *mut window ||
               0 == session_has((*fs).s, (*fs).w) {
            return 1i32.wrapping_neg()
        } else { return cmd_find_best_winlink_with_window(fs) }
    } else {
        if 0 == exact &&
               (*window.offset(0isize) as libc::c_int == 43 ||
                    *window.offset(0isize) as libc::c_int == 45) {
            if *window.offset(1isize) as libc::c_int != 0 {
                n =
                    strtonum(window.offset(1isize), 1i32 as libc::c_longlong,
                             2147483647i32 as libc::c_longlong,
                             0 as *mut *const libc::c_char) as libc::c_int
            } else { n = 1i32 }
            s = (*fs).s;
            if 0 != (*fs).flags & 4i32 {
                if *window.offset(0isize) as libc::c_int == 43 {
                    if 2147483647i32 - (*(*s).curw).idx < n {
                        return 1i32.wrapping_neg()
                    } else { (*fs).idx = (*(*s).curw).idx + n }
                } else if n > (*(*s).curw).idx {
                    return 1i32.wrapping_neg()
                } else { (*fs).idx = (*(*s).curw).idx - n }
                return 0i32
            } else {
                if *window.offset(0isize) as libc::c_int == 43 {
                    (*fs).wl = winlink_next_by_number((*s).curw, s, n)
                } else {
                    (*fs).wl = winlink_previous_by_number((*s).curw, s, n)
                }
                if (*fs).wl != 0 as *mut libc::c_void as *mut winlink {
                    (*fs).idx = (*(*fs).wl).idx;
                    (*fs).w = (*(*fs).wl).window;
                    return 0i32
                }
            }
        }
        if 0 == exact {
            if strcmp(window, b"!\x00" as *const u8 as *const libc::c_char) ==
                   0i32 {
                (*fs).wl =
                    (*(&mut (*(*fs).s).lastw as
                           *mut winlink_stack)).tqh_first;
                if (*fs).wl == 0 as *mut libc::c_void as *mut winlink {
                    return 1i32.wrapping_neg()
                } else {
                    (*fs).idx = (*(*fs).wl).idx;
                    (*fs).w = (*(*fs).wl).window;
                    return 0i32
                }
            } else if strcmp(window,
                             b"^\x00" as *const u8 as *const libc::c_char) ==
                          0i32 {
                (*fs).wl =
                    winlinks_RB_MINMAX(&mut (*(*fs).s).windows as
                                           *mut winlinks,
                                       1i32.wrapping_neg());
                if (*fs).wl == 0 as *mut libc::c_void as *mut winlink {
                    return 1i32.wrapping_neg()
                } else {
                    (*fs).idx = (*(*fs).wl).idx;
                    (*fs).w = (*(*fs).wl).window;
                    return 0i32
                }
            } else if strcmp(window,
                             b"$\x00" as *const u8 as *const libc::c_char) ==
                          0i32 {
                (*fs).wl =
                    winlinks_RB_MINMAX(&mut (*(*fs).s).windows as
                                           *mut winlinks, 1i32);
                if (*fs).wl == 0 as *mut libc::c_void as *mut winlink {
                    return 1i32.wrapping_neg()
                } else {
                    (*fs).idx = (*(*fs).wl).idx;
                    (*fs).w = (*(*fs).wl).window;
                    return 0i32
                }
            }
        }
        if *window.offset(0isize) as libc::c_int != 43 &&
               *window.offset(0isize) as libc::c_int != 45 {
            idx =
                strtonum(window, 0i32 as libc::c_longlong,
                         2147483647i32 as libc::c_longlong,
                         &mut errstr as *mut *const libc::c_char) as
                    libc::c_int;
            if errstr == 0 as *mut libc::c_void as *const libc::c_char {
                if 0 != (*fs).flags & 4i32 {
                    (*fs).idx = idx;
                    return 0i32
                } else {
                    (*fs).wl =
                        winlink_find_by_index(&mut (*(*fs).s).windows as
                                                  *mut winlinks, idx);
                    if (*fs).wl != 0 as *mut libc::c_void as *mut winlink {
                        (*fs).w = (*(*fs).wl).window;
                        return 0i32
                    }
                }
            }
        }
        (*fs).wl = 0 as *mut winlink;
        wl =
            winlinks_RB_MINMAX(&mut (*(*fs).s).windows as *mut winlinks,
                               1i32.wrapping_neg());
        loop  {
            if wl != 0 as *mut libc::c_void as *mut winlink {
                if strcmp(window, (*(*wl).window).name) == 0i32 {
                    if (*fs).wl != 0 as *mut libc::c_void as *mut winlink {
                        return 1i32.wrapping_neg()
                    } else { (*fs).wl = wl }
                }
                wl = winlinks_RB_NEXT(wl)
            } else if (*fs).wl != 0 as *mut libc::c_void as *mut winlink {
                current_block = 1608152415753874203;
                break ;
            } else { current_block = 14648156034262866959; break ; }
        }
        match current_block {
            1608152415753874203 => {
                (*fs).idx = (*(*fs).wl).idx;
                (*fs).w = (*(*fs).wl).window;
                return 0i32
            }
            _ => {
                if 0 != exact {
                    return 1i32.wrapping_neg()
                } else {
                    (*fs).wl = 0 as *mut winlink;
                    wl =
                        winlinks_RB_MINMAX(&mut (*(*fs).s).windows as
                                               *mut winlinks,
                                           1i32.wrapping_neg());
                    loop  {
                        if wl != 0 as *mut libc::c_void as *mut winlink {
                            if strncmp(window, (*(*wl).window).name,
                                       strlen(window)) == 0i32 {
                                if (*fs).wl !=
                                       0 as *mut libc::c_void as *mut winlink
                                   {
                                    return 1i32.wrapping_neg()
                                } else { (*fs).wl = wl }
                            }
                            wl = winlinks_RB_NEXT(wl)
                        } else if (*fs).wl !=
                                      0 as *mut libc::c_void as *mut winlink {
                            current_block = 18153031941552419006;
                            break ;
                        } else {
                            current_block = 1118134448028020070;
                            break ;
                        }
                    }
                    match current_block {
                        18153031941552419006 => {
                            (*fs).idx = (*(*fs).wl).idx;
                            (*fs).w = (*(*fs).wl).window;
                            return 0i32
                        }
                        _ => {
                            (*fs).wl = 0 as *mut winlink;
                            wl =
                                winlinks_RB_MINMAX(&mut (*(*fs).s).windows as
                                                       *mut winlinks,
                                                   1i32.wrapping_neg());
                            loop  {
                                if wl !=
                                       0 as *mut libc::c_void as *mut winlink
                                   {
                                    if fnmatch(window, (*(*wl).window).name,
                                               0i32) == 0i32 {
                                        if (*fs).wl !=
                                               0 as *mut libc::c_void as
                                                   *mut winlink {
                                            return 1i32.wrapping_neg()
                                        } else { (*fs).wl = wl }
                                    }
                                    wl = winlinks_RB_NEXT(wl)
                                } else if (*fs).wl !=
                                              0 as *mut libc::c_void as
                                                  *mut winlink {
                                    current_block = 13131896068329595644;
                                    break ;
                                } else {
                                    current_block = 7746103178988627676;
                                    break ;
                                }
                            }
                            match current_block {
                                7746103178988627676 => {
                                    return 1i32.wrapping_neg()
                                }
                                _ => {
                                    (*fs).idx = (*(*fs).wl).idx;
                                    (*fs).w = (*(*fs).wl).window;
                                    return 0i32
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn cmd_find_get_pane_with_window(mut fs:
                                                       *mut cmd_find_state,
                                                   mut pane:
                                                       *const libc::c_char)
 -> libc::c_int {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut idx: libc::c_int = 0;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut n: u_int = 0;
    log_debug(b"%s: %s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 30],
                                        &[libc::c_char; 30]>(b"cmd_find_get_pane_with_window\x00")).as_ptr(),
              pane);
    if *pane as libc::c_int == 37 {
        (*fs).wp = window_pane_find_by_id_str(pane);
        if (*fs).wp == 0 as *mut libc::c_void as *mut window_pane {
            return 1i32.wrapping_neg()
        } else if (*(*fs).wp).window != (*fs).w {
            return 1i32.wrapping_neg()
        } else { return 0i32 }
    } else if strcmp(pane, b"!\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        (*fs).wp = (*(*fs).w).last;
        if (*fs).wp == 0 as *mut libc::c_void as *mut window_pane {
            return 1i32.wrapping_neg()
        } else { return 0i32 }
    } else if strcmp(pane, b"{up-of}\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        (*fs).wp = window_pane_find_up((*(*fs).w).active);
        if (*fs).wp == 0 as *mut libc::c_void as *mut window_pane {
            return 1i32.wrapping_neg()
        } else { return 0i32 }
    } else if strcmp(pane,
                     b"{down-of}\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        (*fs).wp = window_pane_find_down((*(*fs).w).active);
        if (*fs).wp == 0 as *mut libc::c_void as *mut window_pane {
            return 1i32.wrapping_neg()
        } else { return 0i32 }
    } else if strcmp(pane,
                     b"{left-of}\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        (*fs).wp = window_pane_find_left((*(*fs).w).active);
        if (*fs).wp == 0 as *mut libc::c_void as *mut window_pane {
            return 1i32.wrapping_neg()
        } else { return 0i32 }
    } else if strcmp(pane,
                     b"{right-of}\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        (*fs).wp = window_pane_find_right((*(*fs).w).active);
        if (*fs).wp == 0 as *mut libc::c_void as *mut window_pane {
            return 1i32.wrapping_neg()
        } else { return 0i32 }
    } else {
        if *pane.offset(0isize) as libc::c_int == 43 ||
               *pane.offset(0isize) as libc::c_int == 45 {
            if *pane.offset(1isize) as libc::c_int != 0 {
                n =
                    strtonum(pane.offset(1isize), 1i32 as libc::c_longlong,
                             2147483647i32 as libc::c_longlong,
                             0 as *mut *const libc::c_char) as u_int
            } else { n = 1i32 as u_int }
            wp = (*(*fs).w).active;
            if *pane.offset(0isize) as libc::c_int == 43 {
                (*fs).wp = window_pane_next_by_number((*fs).w, wp, n)
            } else {
                (*fs).wp = window_pane_previous_by_number((*fs).w, wp, n)
            }
            if (*fs).wp != 0 as *mut libc::c_void as *mut window_pane {
                return 0i32
            }
        }
        idx =
            strtonum(pane, 0i32 as libc::c_longlong,
                     2147483647i32 as libc::c_longlong,
                     &mut errstr as *mut *const libc::c_char) as libc::c_int;
        if errstr == 0 as *mut libc::c_void as *const libc::c_char {
            (*fs).wp = window_pane_at_index((*fs).w, idx as u_int);
            if (*fs).wp != 0 as *mut libc::c_void as *mut window_pane {
                return 0i32
            }
        }
        (*fs).wp = window_find_string((*fs).w, pane);
        if (*fs).wp != 0 as *mut libc::c_void as *mut window_pane {
            return 0i32
        } else { return 1i32.wrapping_neg() }
    };
}
unsafe extern "C" fn cmd_find_get_pane_with_session(mut fs:
                                                        *mut cmd_find_state,
                                                    mut pane:
                                                        *const libc::c_char)
 -> libc::c_int {
    log_debug(b"%s: %s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 31],
                                        &[libc::c_char; 31]>(b"cmd_find_get_pane_with_session\x00")).as_ptr(),
              pane);
    if *pane as libc::c_int == 37 {
        (*fs).wp = window_pane_find_by_id_str(pane);
        if (*fs).wp == 0 as *mut libc::c_void as *mut window_pane {
            return 1i32.wrapping_neg()
        } else {
            (*fs).w = (*(*fs).wp).window;
            return cmd_find_best_winlink_with_window(fs)
        }
    } else {
        (*fs).wl = (*(*fs).s).curw;
        (*fs).idx = (*(*fs).wl).idx;
        (*fs).w = (*(*fs).wl).window;
        return cmd_find_get_pane_with_window(fs, pane)
    };
}
static mut cmd_find_pane_table: [[*const libc::c_char; 2]; 16] =
    unsafe {
        [[b"{last}\x00" as *const u8 as *const libc::c_char,
          b"!\x00" as *const u8 as *const libc::c_char],
         [b"{next}\x00" as *const u8 as *const libc::c_char,
          b"+\x00" as *const u8 as *const libc::c_char],
         [b"{previous}\x00" as *const u8 as *const libc::c_char,
          b"-\x00" as *const u8 as *const libc::c_char],
         [b"{top}\x00" as *const u8 as *const libc::c_char,
          b"top\x00" as *const u8 as *const libc::c_char],
         [b"{bottom}\x00" as *const u8 as *const libc::c_char,
          b"bottom\x00" as *const u8 as *const libc::c_char],
         [b"{left}\x00" as *const u8 as *const libc::c_char,
          b"left\x00" as *const u8 as *const libc::c_char],
         [b"{right}\x00" as *const u8 as *const libc::c_char,
          b"right\x00" as *const u8 as *const libc::c_char],
         [b"{top-left}\x00" as *const u8 as *const libc::c_char,
          b"top-left\x00" as *const u8 as *const libc::c_char],
         [b"{top-right}\x00" as *const u8 as *const libc::c_char,
          b"top-right\x00" as *const u8 as *const libc::c_char],
         [b"{bottom-left}\x00" as *const u8 as *const libc::c_char,
          b"bottom-left\x00" as *const u8 as *const libc::c_char],
         [b"{bottom-right}\x00" as *const u8 as *const libc::c_char,
          b"bottom-right\x00" as *const u8 as *const libc::c_char],
         [b"{up-of}\x00" as *const u8 as *const libc::c_char,
          b"{up-of}\x00" as *const u8 as *const libc::c_char],
         [b"{down-of}\x00" as *const u8 as *const libc::c_char,
          b"{down-of}\x00" as *const u8 as *const libc::c_char],
         [b"{left-of}\x00" as *const u8 as *const libc::c_char,
          b"{left-of}\x00" as *const u8 as *const libc::c_char],
         [b"{right-of}\x00" as *const u8 as *const libc::c_char,
          b"{right-of}\x00" as *const u8 as *const libc::c_char],
         [0 as *const libc::c_char, 0 as *const libc::c_char]]
    };
unsafe extern "C" fn cmd_find_map_table(mut table:
                                            *mut [*const libc::c_char; 2],
                                        mut s: *const libc::c_char)
 -> *const libc::c_char {
    let mut i: u_int = 0;
    i = 0i32 as u_int;
    loop  {
        if (*table.offset(i as isize))[0usize] !=
               0 as *mut libc::c_void as *const libc::c_char {
            if strcmp(s, (*table.offset(i as isize))[0usize]) == 0i32 {
                return (*table.offset(i as isize))[1usize]
            } else { i = i.wrapping_add(1) }
        } else { return s }
    };
}
static mut cmd_find_window_table: [[*const libc::c_char; 2]; 6] =
    unsafe {
        [[b"{start}\x00" as *const u8 as *const libc::c_char,
          b"^\x00" as *const u8 as *const libc::c_char],
         [b"{last}\x00" as *const u8 as *const libc::c_char,
          b"!\x00" as *const u8 as *const libc::c_char],
         [b"{end}\x00" as *const u8 as *const libc::c_char,
          b"$\x00" as *const u8 as *const libc::c_char],
         [b"{next}\x00" as *const u8 as *const libc::c_char,
          b"+\x00" as *const u8 as *const libc::c_char],
         [b"{previous}\x00" as *const u8 as *const libc::c_char,
          b"-\x00" as *const u8 as *const libc::c_char],
         [0 as *const libc::c_char, 0 as *const libc::c_char]]
    };
static mut cmd_find_session_table: [[*const libc::c_char; 2]; 1] =
    unsafe { [[0 as *const libc::c_char, 0 as *const libc::c_char]] };
#[no_mangle]
pub unsafe extern "C" fn cmd_find_valid_state(mut fs: *mut cmd_find_state)
 -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    if (*fs).s == 0 as *mut libc::c_void as *mut session ||
           (*fs).wl == 0 as *mut libc::c_void as *mut winlink ||
           (*fs).w == 0 as *mut libc::c_void as *mut window ||
           (*fs).wp == 0 as *mut libc::c_void as *mut window_pane {
        return 0i32
    } else if 0 == session_alive((*fs).s) {
        return 0i32
    } else {
        wl =
            winlinks_RB_MINMAX(&mut (*(*fs).s).windows as *mut winlinks,
                               1i32.wrapping_neg());
        while wl != 0 as *mut libc::c_void as *mut winlink {
            if (*wl).window == (*fs).w && wl == (*fs).wl { break ; }
            wl = winlinks_RB_NEXT(wl)
        }
        if wl == 0 as *mut libc::c_void as *mut winlink {
            return 0i32
        } else if (*fs).w != (*(*fs).wl).window {
            return 0i32
        } else { return window_has_pane((*fs).w, (*fs).wp) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_client(mut fs: *mut cmd_find_state,
                                              mut c: *mut client,
                                              mut flags: libc::c_int)
 -> libc::c_int {
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if c == 0 as *mut libc::c_void as *mut client {
        return cmd_find_from_nothing(fs, flags)
    } else if (*c).session != 0 as *mut libc::c_void as *mut session {
        cmd_find_from_session(fs, (*c).session, flags);
        return 0i32
    } else {
        cmd_find_clear_state(fs, flags);
        wp = cmd_find_inside_pane(c);
        if !(wp == 0 as *mut libc::c_void as *mut window_pane) {
            s = cmd_find_try_TMUX(c);
            if s != 0 as *mut libc::c_void as *mut session {
                wl =
                    winlinks_RB_MINMAX(&mut (*s).windows as *mut winlinks,
                                       1i32.wrapping_neg());
                while wl != 0 as *mut libc::c_void as *mut winlink {
                    if 0 != window_has_pane((*wl).window, wp) { break ; }
                    wl = winlinks_RB_NEXT(wl)
                }
                if wl != 0 as *mut libc::c_void as *mut winlink {
                    (*fs).s = s;
                    (*fs).wl = (*s).curw;
                    (*fs).w = (*(*fs).wl).window;
                    (*fs).wp = (*(*fs).w).active;
                    cmd_find_log_state((*::std::mem::transmute::<&[u8; 21],
                                                                 &[libc::c_char; 21]>(b"cmd_find_from_client\x00")).as_ptr(),
                                       fs);
                    return 0i32
                }
            }
            (*fs).w = (*wp).window;
            if !(cmd_find_best_session_with_window(fs) != 0i32) {
                (*fs).wl = (*(*fs).s).curw;
                (*fs).w = (*(*fs).wl).window;
                (*fs).wp = (*(*fs).w).active;
                cmd_find_log_state((*::std::mem::transmute::<&[u8; 21],
                                                             &[libc::c_char; 21]>(b"cmd_find_from_client\x00")).as_ptr(),
                                   fs);
                return 0i32
            }
        }
        s = cmd_find_try_TMUX(c);
        if s != 0 as *mut libc::c_void as *mut session {
            cmd_find_from_session(fs, s, flags);
            return 0i32
        } else { return cmd_find_from_nothing(fs, flags) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_nothing(mut fs: *mut cmd_find_state,
                                               mut flags: libc::c_int)
 -> libc::c_int {
    cmd_find_clear_state(fs, flags);
    (*fs).s =
        cmd_find_best_session(0 as *mut *mut session, 0i32 as u_int, flags);
    if (*fs).s == 0 as *mut libc::c_void as *mut session {
        cmd_find_clear_state(fs, flags);
        return 1i32.wrapping_neg()
    } else {
        (*fs).wl = (*(*fs).s).curw;
        (*fs).idx = (*(*fs).wl).idx;
        (*fs).w = (*(*fs).wl).window;
        (*fs).wp = (*(*fs).w).active;
        cmd_find_log_state((*::std::mem::transmute::<&[u8; 22],
                                                     &[libc::c_char; 22]>(b"cmd_find_from_nothing\x00")).as_ptr(),
                           fs);
        return 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_session(mut fs: *mut cmd_find_state,
                                               mut s: *mut session,
                                               mut flags: libc::c_int) -> () {
    cmd_find_clear_state(fs, flags);
    (*fs).s = s;
    (*fs).wl = (*(*fs).s).curw;
    (*fs).w = (*(*fs).wl).window;
    (*fs).wp = (*(*fs).w).active;
    cmd_find_log_state((*::std::mem::transmute::<&[u8; 22],
                                                 &[libc::c_char; 22]>(b"cmd_find_from_session\x00")).as_ptr(),
                       fs);
}
unsafe extern "C" fn cmd_find_try_TMUX(mut c: *mut client) -> *mut session {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut tmp: [libc::c_char; 256] = [0; 256];
    let mut pid: libc::c_longlong = 0;
    let mut session: u_int = 0;
    envent =
        environ_find((*c).environ,
                     b"TMUX\x00" as *const u8 as *const libc::c_char);
    if envent == 0 as *mut libc::c_void as *mut environ_entry {
        return 0 as *mut session
    } else if sscanf((*envent).value,
                     b"%255[^,],%lld,%d\x00" as *const u8 as
                         *const libc::c_char, tmp.as_mut_ptr(),
                     &mut pid as *mut libc::c_longlong,
                     &mut session as *mut u_int) != 3i32 {
        return 0 as *mut session
    } else if pid != getpid() as libc::c_longlong {
        return 0 as *mut session
    } else {
        log_debug(b"client %p TMUX %s (session @%u)\x00" as *const u8 as
                      *const libc::c_char, c, (*envent).value, session);
        return session_find_by_id(session)
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_empty_state(mut fs: *mut cmd_find_state)
 -> libc::c_int {
    if (*fs).s == 0 as *mut libc::c_void as *mut session &&
           (*fs).wl == 0 as *mut libc::c_void as *mut winlink &&
           (*fs).w == 0 as *mut libc::c_void as *mut window &&
           (*fs).wp == 0 as *mut libc::c_void as *mut window_pane {
        return 1i32
    } else { return 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_winlink(mut fs: *mut cmd_find_state,
                                               mut wl: *mut winlink,
                                               mut flags: libc::c_int) -> () {
    cmd_find_clear_state(fs, flags);
    (*fs).s = (*wl).session;
    (*fs).wl = wl;
    (*fs).w = (*wl).window;
    (*fs).wp = (*(*wl).window).active;
    cmd_find_log_state((*::std::mem::transmute::<&[u8; 22],
                                                 &[libc::c_char; 22]>(b"cmd_find_from_winlink\x00")).as_ptr(),
                       fs);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_session_window(mut fs:
                                                          *mut cmd_find_state,
                                                      mut s: *mut session,
                                                      mut w: *mut window,
                                                      mut flags: libc::c_int)
 -> libc::c_int {
    cmd_find_clear_state(fs, flags);
    (*fs).s = s;
    (*fs).w = w;
    if cmd_find_best_winlink_with_window(fs) != 0i32 {
        cmd_find_clear_state(fs, flags);
        return 1i32.wrapping_neg()
    } else {
        (*fs).wp = (*(*fs).w).active;
        cmd_find_log_state((*::std::mem::transmute::<&[u8; 29],
                                                     &[libc::c_char; 29]>(b"cmd_find_from_session_window\x00")).as_ptr(),
                           fs);
        return 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_window(mut fs: *mut cmd_find_state,
                                              mut w: *mut window,
                                              mut flags: libc::c_int)
 -> libc::c_int {
    cmd_find_clear_state(fs, flags);
    (*fs).w = w;
    if cmd_find_best_session_with_window(fs) != 0i32 {
        cmd_find_clear_state(fs, flags);
        return 1i32.wrapping_neg()
    } else if cmd_find_best_winlink_with_window(fs) != 0i32 {
        cmd_find_clear_state(fs, flags);
        return 1i32.wrapping_neg()
    } else {
        (*fs).wp = (*(*fs).w).active;
        cmd_find_log_state((*::std::mem::transmute::<&[u8; 21],
                                                     &[libc::c_char; 21]>(b"cmd_find_from_window\x00")).as_ptr(),
                           fs);
        return 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_winlink_pane(mut fs:
                                                        *mut cmd_find_state,
                                                    mut wl: *mut winlink,
                                                    mut wp: *mut window_pane,
                                                    mut flags: libc::c_int)
 -> () {
    cmd_find_clear_state(fs, flags);
    (*fs).s = (*wl).session;
    (*fs).wl = wl;
    (*fs).idx = (*(*fs).wl).idx;
    (*fs).w = (*(*fs).wl).window;
    (*fs).wp = wp;
    cmd_find_log_state((*::std::mem::transmute::<&[u8; 27],
                                                 &[libc::c_char; 27]>(b"cmd_find_from_winlink_pane\x00")).as_ptr(),
                       fs);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_pane(mut fs: *mut cmd_find_state,
                                            mut wp: *mut window_pane,
                                            mut flags: libc::c_int)
 -> libc::c_int {
    if cmd_find_from_window(fs, (*wp).window, flags) != 0i32 {
        return 1i32.wrapping_neg()
    } else {
        (*fs).wp = wp;
        cmd_find_log_state((*::std::mem::transmute::<&[u8; 19],
                                                     &[libc::c_char; 19]>(b"cmd_find_from_pane\x00")).as_ptr(),
                           fs);
        return 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_mouse(mut fs: *mut cmd_find_state,
                                             mut m: *mut mouse_event,
                                             mut flags: libc::c_int)
 -> libc::c_int {
    cmd_find_clear_state(fs, flags);
    if 0 == (*m).valid {
        return 1i32.wrapping_neg()
    } else {
        (*fs).wp =
            cmd_mouse_pane(m, &mut (*fs).s as *mut *mut session,
                           &mut (*fs).wl as *mut *mut winlink);
        if (*fs).wp == 0 as *mut libc::c_void as *mut window_pane {
            cmd_find_clear_state(fs, flags);
            return 1i32.wrapping_neg()
        } else {
            (*fs).w = (*(*fs).wl).window;
            cmd_find_log_state((*::std::mem::transmute::<&[u8; 20],
                                                         &[libc::c_char; 20]>(b"cmd_find_from_mouse\x00")).as_ptr(),
                               fs);
            return 0i32
        }
    };
}

