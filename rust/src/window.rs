extern crate libc;

use alerts::alerts_queue;
use cmd::{cmd_copy_argv, cmd_free_argv, cmd_stringify_argv};
use common::{termios, timeval};
use compat::fdforkpty::fdforkpty;
use compat::closefrom::closefrom;
use environ::{environ, environ_push};
use grid::{grid_cell, grid_duplicate_lines, grid_default_cell};
use session::{session, session_group, session_groups, sessions};
use log::log_close;
use style::{style_equal};
use notify::{notify_pane, notify_window};
use options::{options, options_create, options_get_number, options_get_style, options_set_number, options_free};
use proc_::{event, proc_clear_signals, tmuxpeer, tmuxproc, event_add, event_del, event_set, event_initialized, event_base};
use proc_::{unnamed_40 as unnamed, unnamed_13 as unnamed_39, unnamed as unnamed_0, unnamed_43 as unnamed_38, unnamed_14 as unnamed_27};
use proc_::{unnamed_16 as unnamed_13, unnamed_32 as unnamed_20};
use grid::{grid_destroy, grid_create, grid};
use utf8::{utf8_data, utf8_stravis};
use xmalloc::{xcalloc};

extern "C" {
    pub type args_entry;
    pub type format_job_tree;
    pub type hooks;
    pub type options_entry;
    pub type input_ctx;
    pub type format_tree;
    pub type screen_titles;
    pub type _IO_FILE_plus;
    pub type tty_code;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, ...) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fnmatch(__pattern: *const libc::c_char, __name: *const libc::c_char,
               __flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigprocmask(__how: libc::c_int, __set: *const sigset_t,
                   __oset: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    static _sys_siglist: [*const libc::c_char; 65];
    #[no_mangle]
    static sys_siglist: [*const libc::c_char; 65];
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn setenv(__name: *const libc::c_char, __value: *const libc::c_char,
              __replace: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    #[no_mangle]
    fn tcsetattr(__fd: libc::c_int, __optional_actions: libc::c_int,
                 __termios_p: *const termios) -> libc::c_int;
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
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
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t)
     -> *mut libc::c_uchar;
    #[no_mangle]
    fn bufferevent_free(bufev: *mut bufferevent) -> ();
    #[no_mangle]
    fn bufferevent_write(bufev: *mut bufferevent, data: *const libc::c_void,
                         size: size_t) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_enable(bufev: *mut bufferevent, event: libc::c_short)
     -> libc::c_int;
    #[no_mangle]
    fn bufferevent_setwatermark(bufev: *mut bufferevent,
                                events: libc::c_short, lowmark: size_t,
                                highmark: size_t) -> ();
    #[no_mangle]
    fn bufferevent_new(fd: libc::c_int, readcb: bufferevent_data_cb,
                       writecb: bufferevent_data_cb,
                       errorcb: bufferevent_event_cb,
                       cbarg: *mut libc::c_void) -> *mut bufferevent;
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
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, ...)
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
    fn find_home() -> *const libc::c_char;
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn environ_set(_: *mut environ, _: *const libc::c_char,
                   _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn environ_log(_: *mut environ, _: *const libc::c_char, ...) -> ();
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
    fn server_clear_marked() -> ();
    #[no_mangle]
    fn server_check_marked() -> libc::c_int;
    #[no_mangle]
    fn server_status_session(_: *mut session) -> ();
    #[no_mangle]
    fn server_status_window(_: *mut window) -> ();
    #[no_mangle]
    fn server_link_window(_: *mut session, _: *mut winlink, _: *mut session,
                          _: libc::c_int, _: libc::c_int, _: libc::c_int,
                          _: *mut *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn server_unlink_window(_: *mut session, _: *mut winlink) -> ();
    #[no_mangle]
    fn server_destroy_pane(_: *mut window_pane, _: libc::c_int) -> ();
    #[no_mangle]
    fn input_init(_: *mut window_pane) -> ();
    #[no_mangle]
    fn input_free(_: *mut window_pane) -> ();
    #[no_mangle]
    fn input_parse(_: *mut window_pane) -> ();
    #[no_mangle]
    fn input_key(_: *mut window_pane, _: key_code, _: *mut mouse_event) -> ();
    #[no_mangle]
    fn grid_view_clear(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int,
                       _: u_int) -> ();
    #[no_mangle]
    fn grid_view_string_cells(_: *mut grid, _: u_int, _: u_int, _: u_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn screen_free(_: *mut screen) -> ();
    #[no_mangle]
    fn screen_set_title(_: *mut screen, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int)
     -> ();
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn layout_free_cell(_: *mut layout_cell) -> ();
    #[no_mangle]
    fn default_window_name(_: *mut window) -> *mut libc::c_char;
    #[no_mangle]
    fn fatal(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn layout_init(_: *mut window, _: *mut window_pane) -> ();
    #[no_mangle]
    fn layout_fix_panes(_: *mut window, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn layout_free(_: *mut window) -> ();
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
pub const KEYC_MOUSEUP1_PANE: unnamed_37 = 268435474;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const KEYC_F4: unnamed_37 = 268435529;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const KEYC_DOUBLECLICK2_STATUS: unnamed_37 = 268435511;
pub const PROMPT_ENTRY: unnamed_28 = 0;
pub const KEYC_TRIPLECLICK2_PANE: unnamed_37 = 268435519;
pub const KEYC_TRIPLECLICK2_STATUS: unnamed_37 = 268435520;
pub const KEYC_MOUSEDRAGEND3_STATUS: unnamed_37 = 268435499;
pub type __ssize_t = libc::c_long;
pub const KEYC_MOUSEDRAGEND2_PANE: unnamed_37 = 268435495;
pub const KEYC_DOUBLECLICK1_STATUS: unnamed_37 = 268435508;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_4,
}
pub const KEYC_KP_EIGHT: unnamed_37 = 268435553;
pub type u_int = __u_int;
pub type tcflag_t = libc::c_uint;
pub const TTY_VT102: unnamed_26 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
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
    pub entry: unnamed_31,
    pub wentry: unnamed_19,
    pub sentry: unnamed_7,
}
pub const KEYC_KP_NINE: unnamed_37 = 268435554;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub const KEYC_KP_TWO: unnamed_37 = 268435560;
pub type layout_type = libc::c_uint;
pub const JOB_DEAD: unnamed_30 = 1;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const KEYC_MOUSEDOWN2_STATUS: unnamed_37 = 268435469;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub const KEYC_KP_PERIOD: unnamed_37 = 268435564;
pub type ssize_t = __ssize_t;
pub const KEYC_MOUSEDRAG2_PANE: unnamed_37 = 268435486;
pub const KEYC_MOUSEDRAG3_STATUS: unnamed_37 = 268435490;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_35,
}
pub const KEYC_TRIPLECLICK1_BORDER: unnamed_37 = 268435518;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_6 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const KEYC_KP_ENTER: unnamed_37 = 268435562;
pub const KEYC_MOUSEUP2_BORDER: unnamed_37 = 268435479;
pub const PROMPT_COMMAND: unnamed_28 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_21,
}
pub const KEYC_MOUSEDRAG3_PANE: unnamed_37 = 268435489;
pub const KEYC_MOUSEUP3_PANE: unnamed_37 = 268435480;
pub const KEYC_WHEELDOWN_PANE: unnamed_37 = 268435504;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const KEYC_KP_ONE: unnamed_37 = 268435559;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
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
pub type __sigset_t = sigset_t;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const KEYC_KP_THREE: unnamed_37 = 268435561;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_8,
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
pub struct layout_cell {
    pub type_0: layout_type,
    pub parent: *mut layout_cell,
    pub sx: u_int,
    pub sy: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub wp: *mut window_pane,
    pub cells: layout_cells,
    pub entry: unnamed_2,
}
pub type __pid_t = libc::c_int;
pub const KEYC_MOUSEDOWN1_BORDER: unnamed_37 = 268435467;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub ev_signal_next: unnamed_15,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const KEYC_MOUSEDRAG1_PANE: unnamed_37 = 268435483;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_6,
}
pub const KEYC_MOUSEDRAG1_BORDER: unnamed_37 = 268435485;
pub const KEYC_BSPACE: unnamed_37 = 268435525;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const KEYC_F12: unnamed_37 = 268435537;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const KEYC_BTAB: unnamed_37 = 268435544;
pub type unnamed_10 = libc::c_uint;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const KEYC_IC: unnamed_37 = 268435538;
pub const KEYC_F3: unnamed_37 = 268435528;
pub const KEYC_DRAGGING: unnamed_37 = 268435461;
pub type key_code = libc::c_ulonglong;
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
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
    pub entry: unnamed_33,
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
    pub entry: unnamed_14,
}
pub type bitstr_t = libc::c_uchar;
pub const KEYC_MOUSEDRAG1_STATUS: unnamed_37 = 268435484;
pub const KEYC_DC: unnamed_37 = 268435539;
pub const LINE_SEL_LEFT_RIGHT: unnamed_10 = 1;
pub const KEYC_MOUSEDRAGEND1_BORDER: unnamed_37 = 268435494;
pub const TTY_VT420: unnamed_26 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub type __u_char = libc::c_uchar;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub type __u_short = libc::c_ushort;
pub const KEYC_MOUSEUP1_BORDER: unnamed_37 = 268435476;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub type uint16_t = libc::c_ushort;
pub type __suseconds_t = libc::c_long;
pub const KEYC_F10: unnamed_37 = 268435535;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const KEYC_KP_ZERO: unnamed_37 = 268435563;
pub type pid_t = __pid_t;
pub const KEYC_TRIPLECLICK3_BORDER: unnamed_37 = 268435524;
pub const TTY_VT101: unnamed_26 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub const KEYC_WHEELUP_BORDER: unnamed_37 = 268435503;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_3,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const KEYC_PASTE_END: unnamed_37 = 268435459;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const KEYC_HOME: unnamed_37 = 268435540;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub type speed_t = libc::c_uint;
pub const KEYC_KP_SIX: unnamed_37 = 268435558;
pub const KEYC_PASTE_START: unnamed_37 = 268435458;
pub const KEYC_TRIPLECLICK2_BORDER: unnamed_37 = 268435521;
pub const KEYC_MOUSEDOWN1_PANE: unnamed_37 = 268435465;
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type __off_t = libc::c_long;
pub const KEYC_MOUSEDRAG3_BORDER: unnamed_37 = 268435491;
pub const KEYC_F5: unnamed_37 = 268435530;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const KEYC_FOCUS_OUT: unnamed_37 = 268435457;
pub const KEYC_TRIPLECLICK1_PANE: unnamed_37 = 268435516;
pub const KEYC_DOUBLECLICK3_STATUS: unnamed_37 = 268435514;
pub const KEYC_DOUBLECLICK2_BORDER: unnamed_37 = 268435512;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub type time_t = __time_t;
pub const TTY_VT320: unnamed_26 = 4;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const JOB_CLOSED: unnamed_30 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
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
pub const KEYC_F6: unnamed_37 = 268435531;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_25,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const KEYC_MOUSE: unnamed_37 = 268435460;
pub const KEYC_MOUSEDRAGEND3_PANE: unnamed_37 = 268435498;
pub const KEYC_MOUSEDRAGEND1_PANE: unnamed_37 = 268435492;
pub const KEYC_MOUSEDRAGEND2_BORDER: unnamed_37 = 268435497;
pub const JOB_RUNNING: unnamed_30 = 0;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const KEYC_MOUSEDOWN3_STATUS: unnamed_37 = 268435472;
pub const KEYC_F11: unnamed_37 = 268435536;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const KEYC_KP_STAR: unnamed_37 = 268435550;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const KEYC_KP_MINUS: unnamed_37 = 268435551;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
    pub alerts_entry: unnamed_17,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_11,
    pub entry: unnamed_5,
}
pub const KEYC_NPAGE: unnamed_37 = 268435542;
pub const KEYC_MOUSEDOWN2_PANE: unnamed_37 = 268435468;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const TTY_VT100: unnamed_26 = 0;
pub const KEYC_END: unnamed_37 = 268435541;
pub const KEYC_RIGHT: unnamed_37 = 268435548;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const KEYC_PPAGE: unnamed_37 = 268435543;
pub const KEYC_UP: unnamed_37 = 268435545;
pub const KEYC_MOUSEUP2_STATUS: unnamed_37 = 268435478;
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
    pub message_log: unnamed_29,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_28,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_23,
}
pub const KEYC_TRIPLECLICK3_STATUS: unnamed_37 = 268435523;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const KEYC_F8: unnamed_37 = 268435533;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const KEYC_DOUBLECLICK3_BORDER: unnamed_37 = 268435515;
pub const KEYC_F7: unnamed_37 = 268435532;
pub const TTY_VT220: unnamed_26 = 3;
pub const KEYC_WHEELUP_STATUS: unnamed_37 = 268435502;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub const KEYC_MOUSEMOVE_BORDER: unnamed_37 = 268435464;
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
pub struct unnamed_24 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const KEYC_MOUSEDRAG2_STATUS: unnamed_37 = 268435487;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const KEYC_MOUSEMOVE_PANE: unnamed_37 = 268435462;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
pub type unnamed_26 = libc::c_uint;
pub type options_table_type = libc::c_uint;
pub const KEYC_DOWN: unnamed_37 = 268435546;
pub const KEYC_MOUSEMOVE_STATUS: unnamed_37 = 268435463;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const KEYC_WHEELUP_PANE: unnamed_37 = 268435501;
pub const KEYC_DOUBLECLICK2_PANE: unnamed_37 = 268435510;
pub type __timezone_ptr_t = *mut timezone;
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_10,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type unnamed_28 = libc::c_uint;
pub const TTY_UNKNOWN: unnamed_26 = 6;
pub const KEYC_MOUSEDRAGEND3_BORDER: unnamed_37 = 268435500;
pub type cmdq_type = libc::c_uint;
pub type uint8_t = libc::c_uchar;
pub type cmd_retval = libc::c_int;
pub type cc_t = libc::c_uchar;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const KEYC_MOUSEUP3_STATUS: unnamed_37 = 268435481;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const KEYC_WHEELDOWN_STATUS: unnamed_37 = 268435505;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const KEYC_F2: unnamed_37 = 268435527;
pub type options_table_scope = libc::c_uint;
pub const KEYC_LEFT: unnamed_37 = 268435547;
pub const KEYC_F1: unnamed_37 = 268435526;
pub type unnamed_30 = libc::c_uint;
pub const KEYC_TRIPLECLICK3_PANE: unnamed_37 = 268435522;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const KEYC_DOUBLECLICK1_BORDER: unnamed_37 = 268435509;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
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
    pub entry: unnamed_12,
    pub tree_entry: unnamed_24,
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const KEYC_MOUSEUP3_BORDER: unnamed_37 = 268435482;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const LINE_SEL_NONE: unnamed_10 = 0;
pub const LINE_SEL_RIGHT_LEFT: unnamed_10 = 2;
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
pub const KEYC_MOUSEDRAG2_BORDER: unnamed_37 = 268435488;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const KEYC_KP_FIVE: unnamed_37 = 268435557;
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_36,
}
pub type _IO_lock_t = ();
pub const KEYC_MOUSEDRAGEND2_STATUS: unnamed_37 = 268435496;
pub const KEYC_KP_FOUR: unnamed_37 = 268435556;
pub const KEYC_KP_SEVEN: unnamed_37 = 268435552;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
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
pub const KEYC_MOUSEUP1_STATUS: unnamed_37 = 268435475;
pub const KEYC_MOUSEDOWN1_STATUS: unnamed_37 = 268435466;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_35 {
    offset: u_int,
    data: unnamed_32,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const KEYC_FOCUS_IN: unnamed_37 = 268435456;
pub const KEYC_MOUSEUP2_PANE: unnamed_37 = 268435477;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_16,
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
pub const KEYC_F9: unnamed_37 = 268435534;
pub const KEYC_KP_SLASH: unnamed_37 = 268435549;
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
pub const KEYC_MOUSEDOWN3_PANE: unnamed_37 = 268435471;
pub const KEYC_MOUSEDOWN2_BORDER: unnamed_37 = 268435470;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub type unnamed_37 = libc::c_uint;
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type u_char = __u_char;
pub const KEYC_KP_PLUS: unnamed_37 = 268435555;
pub const KEYC_TRIPLECLICK1_STATUS: unnamed_37 = 268435517;
pub const KEYC_DOUBLECLICK1_PANE: unnamed_37 = 268435507;
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
pub const KEYC_MOUSEDRAGEND1_STATUS: unnamed_37 = 268435493;
pub const KEYC_WHEELDOWN_BORDER: unnamed_37 = 268435506;
pub const KEYC_DOUBLECLICK3_PANE: unnamed_37 = 268435513;
pub const KEYC_MOUSEDOWN3_BORDER: unnamed_37 = 268435473;
#[no_mangle]
pub static mut windows_static: windows = // REMINDER: Renamed from "windows" due to conflicting with windows type
    unsafe { windows{rbh_root: 0 as *const window as *mut window,} };
#[no_mangle]
pub static mut all_window_panes: window_pane_tree =
    unsafe {
        window_pane_tree{rbh_root:
                             0 as *const window_pane as *mut window_pane,}
    };
#[no_mangle]
pub unsafe extern "C" fn window_cmp(mut w1: *mut window, mut w2: *mut window)
 -> libc::c_int {
    return (*w1).id.wrapping_sub((*w2).id) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn windows_RB_INSERT_COLOR(mut head: *mut windows,
                                                 mut elm: *mut window) -> () {
    let mut current_block: u64;
    let mut parent: *mut window = 0 as *mut window;
    let mut gparent: *mut window = 0 as *mut window;
    let mut tmp: *mut window = 0 as *mut window;
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
                's_38:
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
                                                continue 's_38 ;
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
                            4567019141635105728 => {
                                (*parent).entry.rbe_color = 0i32;
                                (*gparent).entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4567019141635105728;
                                } else { break ; }
                            }
                            _ => {
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
#[no_mangle]
pub unsafe extern "C" fn windows_RB_REMOVE_COLOR(mut head: *mut windows,
                                                 mut parent: *mut window,
                                                 mut elm: *mut window) -> () {
    let mut current_block: u64;
    let mut tmp: *mut window = 0 as *mut window;
    's_4:
        loop  {
            if !((elm == 0 as *mut libc::c_void as *mut window ||
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
                                            15240798224410183470 => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_30 ; }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        11050875288958768710;
                                                } else {
                                                    current_block =
                                                        15240798224410183470;
                                                }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).entry.rbe_right;
                            current_block = 14155750587950065367;
                        }
                        _ => {
                            if ((*tmp).entry.rbe_left ==
                                    0 as *mut libc::c_void as *mut window ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as *mut window
                                        ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_right ==
                                          0 as *mut libc::c_void as
                                              *mut window ||
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
                                    0 as *mut libc::c_void as *mut window ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as *mut window
                                        ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_left ==
                                          0 as *mut libc::c_void as
                                              *mut window ||
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
        15976848397966268834 => {
            let mut oleft: *mut window = 0 as *mut window;
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
        13826291924415791078 => {
            let mut oright: *mut window = 0 as *mut window;
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
        _ => { }
    }
    match current_block {
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
                            6450636197030046351 => {
                                if 0 != 0i32 {
                                    current_block = 6450636197030046351;
                                } else {
                                    current_block = 16924917904204750491;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_148 ; }
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
#[no_mangle]
pub unsafe extern "C" fn windows_RB_REMOVE(mut head: *mut windows,
                                           mut elm: *mut window)
 -> *mut window {
    let mut current_block: u64;
    let mut child: *mut window = 0 as *mut window;
    let mut parent: *mut window = 0 as *mut window;
    let mut old: *mut window = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left == 0 as *mut libc::c_void as *mut window {
        child = (*elm).entry.rbe_right;
        current_block = 9386390421034826751;
    } else if (*elm).entry.rbe_right == 0 as *mut libc::c_void as *mut window
     {
        child = (*elm).entry.rbe_left;
        current_block = 9386390421034826751;
    } else {
        let mut left: *mut window = 0 as *mut window;
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
            current_block = 3244726397434400022;
        } else { current_block = 3244726397434400022; }
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
    if color == 0i32 { windows_RB_REMOVE_COLOR(head, parent, child); }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn windows_RB_INSERT(mut head: *mut windows,
                                           mut elm: *mut window)
 -> *mut window {
    let mut tmp: *mut window = 0 as *mut window;
    let mut parent: *mut window = 0 as *mut window;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = window_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    loop  {
        (*elm).entry.rbe_parent = parent;
        (*elm).entry.rbe_right = 0 as *mut window;
        (*elm).entry.rbe_left = (*elm).entry.rbe_right;
        (*elm).entry.rbe_color = 1i32;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut window {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else { (*parent).entry.rbe_right = elm }
        while 0 != 0i32 { }
    } else { (*head).rbh_root = elm }
    windows_RB_INSERT_COLOR(head, elm);
    return 0 as *mut window;
}
#[no_mangle]
pub unsafe extern "C" fn windows_RB_FIND(mut head: *mut windows,
                                         mut elm: *mut window)
 -> *mut window {
    let mut tmp: *mut window = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = window_cmp(elm, tmp);
            if comp < 0i32 {
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return 0 as *mut window }
    };
}
#[no_mangle]
pub unsafe extern "C" fn windows_RB_NFIND(mut head: *mut windows,
                                          mut elm: *mut window)
 -> *mut window {
    let mut tmp: *mut window = (*head).rbh_root;
    let mut res: *mut window = 0 as *mut window;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = window_cmp(elm, tmp);
            if comp < 0i32 {
                res = tmp;
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return res }
    };
}
#[no_mangle]
pub unsafe extern "C" fn windows_RB_NEXT(mut elm: *mut window)
 -> *mut window {
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
#[no_mangle]
pub unsafe extern "C" fn windows_RB_PREV(mut elm: *mut window)
 -> *mut window {
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
#[no_mangle]
pub unsafe extern "C" fn windows_RB_MINMAX(mut head: *mut windows,
                                           mut val: libc::c_int)
 -> *mut window {
    let mut tmp: *mut window = (*head).rbh_root;
    let mut parent: *mut window = 0 as *mut window;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else { tmp = (*tmp).entry.rbe_right }
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_cmp(mut wl1: *mut winlink,
                                     mut wl2: *mut winlink) -> libc::c_int {
    return (*wl1).idx - (*wl2).idx;
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_INSERT_COLOR(mut head: *mut winlinks,
                                                  mut elm: *mut winlink)
 -> () {
    let mut current_block: u64;
    let mut parent: *mut winlink = 0 as *mut winlink;
    let mut gparent: *mut winlink = 0 as *mut winlink;
    let mut tmp: *mut winlink = 0 as *mut winlink;
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
                            7351195479953500246 => {
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
                            _ => {
                                (*parent).entry.rbe_color = 0i32;
                                (*gparent).entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4956146061682418353;
                                } else { break ; }
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
                's_162:
                    loop  {
                        match current_block {
                            4567019141635105728 => {
                                (*parent).entry.rbe_color = 0i32;
                                (*gparent).entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4567019141635105728;
                                } else { break ; }
                            }
                            _ => {
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
                                                continue 's_162 ;
                                            } else { break ; }
                                        }
                                    }
                                }
                                tmp = parent;
                                parent = elm;
                                elm = tmp;
                                current_block = 4567019141635105728;
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
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_REMOVE_COLOR(mut head: *mut winlinks,
                                                  mut parent: *mut winlink,
                                                  mut elm: *mut winlink)
 -> () {
    let mut current_block: u64;
    let mut tmp: *mut winlink = 0 as *mut winlink;
    's_4:
        loop  {
            if !((elm == 0 as *mut libc::c_void as *mut winlink ||
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
                                    0 as *mut libc::c_void as *mut winlink ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as *mut winlink
                                        ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_right ==
                                          0 as *mut libc::c_void as
                                              *mut winlink ||
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
                                    0 as *mut libc::c_void as *mut winlink ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as *mut winlink
                                        ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_left ==
                                          0 as *mut libc::c_void as
                                              *mut winlink ||
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
                                            17784502470059252271 => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_210 ; }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        16738040538446813684;
                                                } else {
                                                    current_block =
                                                        17784502470059252271;
                                                }
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
            let mut oright: *mut winlink = 0 as *mut winlink;
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
            let mut oleft: *mut winlink = 0 as *mut winlink;
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
                            6450636197030046351 => {
                                if 0 != 0i32 {
                                    current_block = 6450636197030046351;
                                } else {
                                    current_block = 16924917904204750491;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_148 ; }
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
                            13910774313357589740 => {
                                if 0 != 0i32 {
                                    current_block = 13910774313357589740;
                                } else {
                                    current_block = 13707613154239713890;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_328 ; }
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
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_REMOVE(mut head: *mut winlinks,
                                            mut elm: *mut winlink)
 -> *mut winlink {
    let mut current_block: u64;
    let mut child: *mut winlink = 0 as *mut winlink;
    let mut parent: *mut winlink = 0 as *mut winlink;
    let mut old: *mut winlink = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left == 0 as *mut libc::c_void as *mut winlink {
        child = (*elm).entry.rbe_right;
        current_block = 9386390421034826751;
    } else if (*elm).entry.rbe_right == 0 as *mut libc::c_void as *mut winlink
     {
        child = (*elm).entry.rbe_left;
        current_block = 9386390421034826751;
    } else {
        let mut left: *mut winlink = 0 as *mut winlink;
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
            current_block = 17547528762010270795;
        } else { current_block = 17547528762010270795; }
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
    if color == 0i32 { winlinks_RB_REMOVE_COLOR(head, parent, child); }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_INSERT(mut head: *mut winlinks,
                                            mut elm: *mut winlink)
 -> *mut winlink {
    let mut tmp: *mut winlink = 0 as *mut winlink;
    let mut parent: *mut winlink = 0 as *mut winlink;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = winlink_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    loop  {
        (*elm).entry.rbe_parent = parent;
        (*elm).entry.rbe_right = 0 as *mut winlink;
        (*elm).entry.rbe_left = (*elm).entry.rbe_right;
        (*elm).entry.rbe_color = 1i32;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut winlink {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else { (*parent).entry.rbe_right = elm }
        while 0 != 0i32 { }
    } else { (*head).rbh_root = elm }
    winlinks_RB_INSERT_COLOR(head, elm);
    return 0 as *mut winlink;
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_FIND(mut head: *mut winlinks,
                                          mut elm: *mut winlink)
 -> *mut winlink {
    let mut tmp: *mut winlink = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = winlink_cmp(elm, tmp);
            if comp < 0i32 {
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return 0 as *mut winlink }
    };
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_NFIND(mut head: *mut winlinks,
                                           mut elm: *mut winlink)
 -> *mut winlink {
    let mut tmp: *mut winlink = (*head).rbh_root;
    let mut res: *mut winlink = 0 as *mut winlink;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = winlink_cmp(elm, tmp);
            if comp < 0i32 {
                res = tmp;
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return res }
    };
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_NEXT(mut elm: *mut winlink)
 -> *mut winlink {
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
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_PREV(mut elm: *mut winlink)
 -> *mut winlink {
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
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_MINMAX(mut head: *mut winlinks,
                                            mut val: libc::c_int)
 -> *mut winlink {
    let mut tmp: *mut winlink = (*head).rbh_root;
    let mut parent: *mut winlink = 0 as *mut winlink;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else { tmp = (*tmp).entry.rbe_right }
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_cmp(mut wp1: *mut window_pane,
                                         mut wp2: *mut window_pane)
 -> libc::c_int {
    return (*wp1).id.wrapping_sub((*wp2).id) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_INSERT_COLOR(mut head:
                                                              *mut window_pane_tree,
                                                          mut elm:
                                                              *mut window_pane)
 -> () {
    let mut current_block: u64;
    let mut parent: *mut window_pane = 0 as *mut window_pane;
    let mut gparent: *mut window_pane = 0 as *mut window_pane;
    let mut tmp: *mut window_pane = 0 as *mut window_pane;
    loop  {
        parent = (*elm).tree_entry.rbe_parent;
        if !(!parent.is_null() && (*parent).tree_entry.rbe_color == 1i32) {
            break ;
        }
        gparent = (*parent).tree_entry.rbe_parent;
        if parent == (*gparent).tree_entry.rbe_left {
            tmp = (*gparent).tree_entry.rbe_right;
            if !tmp.is_null() && (*tmp).tree_entry.rbe_color == 1i32 {
                (*tmp).tree_entry.rbe_color = 0i32;
                loop  {
                    (*parent).tree_entry.rbe_color = 0i32;
                    (*gparent).tree_entry.rbe_color = 1i32;
                    if !(0 != 0i32) { break ; }
                }
                elm = gparent
            } else {
                if (*parent).tree_entry.rbe_right == elm {
                    current_block = 7351195479953500246;
                } else { current_block = 4956146061682418353; }
                's_38:
                    loop  {
                        match current_block {
                            4956146061682418353 => {
                                (*parent).tree_entry.rbe_color = 0i32;
                                (*gparent).tree_entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4956146061682418353;
                                } else { break ; }
                            }
                            _ => {
                                tmp = (*parent).tree_entry.rbe_right;
                                (*parent).tree_entry.rbe_right =
                                    (*tmp).tree_entry.rbe_left;
                                if !(*parent).tree_entry.rbe_right.is_null() {
                                    (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_parent
                                        = parent
                                }
                                while 0 != 0i32 { }
                                (*tmp).tree_entry.rbe_parent =
                                    (*parent).tree_entry.rbe_parent;
                                if !(*tmp).tree_entry.rbe_parent.is_null() {
                                    if parent ==
                                           (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                                       {
                                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                                            = tmp
                                    } else {
                                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right
                                            = tmp
                                    }
                                } else { (*head).rbh_root = tmp }
                                (*tmp).tree_entry.rbe_left = parent;
                                (*parent).tree_entry.rbe_parent = tmp;
                                while 0 != 0i32 { }
                                if !(*tmp).tree_entry.rbe_parent.is_null() {
                                    current_block = 10048703153582371463;
                                } else {
                                    current_block = 10879442775620481940;
                                }
                                loop  {
                                    match current_block {
                                        10048703153582371463 => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    10048703153582371463;
                                            } else {
                                                current_block =
                                                    10879442775620481940;
                                            }
                                        }
                                        _ => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    7351195479953500246;
                                                continue 's_38 ;
                                            } else { break ; }
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
                        tmp = (*gparent).tree_entry.rbe_left;
                        (*gparent).tree_entry.rbe_left =
                            (*tmp).tree_entry.rbe_right;
                        if !(*gparent).tree_entry.rbe_left.is_null() {
                            (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_parent
                                = gparent
                        }
                        while 0 != 0i32 { }
                        (*tmp).tree_entry.rbe_parent =
                            (*gparent).tree_entry.rbe_parent;
                        if !(*tmp).tree_entry.rbe_parent.is_null() {
                            if gparent ==
                                   (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_left
                               {
                                (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_left
                                    = tmp
                            } else {
                                (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_right
                                    = tmp
                            }
                        } else { (*head).rbh_root = tmp }
                        (*tmp).tree_entry.rbe_right = gparent;
                        (*gparent).tree_entry.rbe_parent = tmp;
                        while 0 != 0i32 { }
                        if !(*tmp).tree_entry.rbe_parent.is_null() {
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
            tmp = (*gparent).tree_entry.rbe_left;
            if !tmp.is_null() && (*tmp).tree_entry.rbe_color == 1i32 {
                (*tmp).tree_entry.rbe_color = 0i32;
                loop  {
                    (*parent).tree_entry.rbe_color = 0i32;
                    (*gparent).tree_entry.rbe_color = 1i32;
                    if !(0 != 0i32) { break ; }
                }
                elm = gparent
            } else {
                if (*parent).tree_entry.rbe_left == elm {
                    current_block = 652864300344834934;
                } else { current_block = 4567019141635105728; }
                's_211:
                    loop  {
                        match current_block {
                            652864300344834934 => {
                                tmp = (*parent).tree_entry.rbe_left;
                                (*parent).tree_entry.rbe_left =
                                    (*tmp).tree_entry.rbe_right;
                                if !(*parent).tree_entry.rbe_left.is_null() {
                                    (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_parent
                                        = parent
                                }
                                while 0 != 0i32 { }
                                (*tmp).tree_entry.rbe_parent =
                                    (*parent).tree_entry.rbe_parent;
                                if !(*tmp).tree_entry.rbe_parent.is_null() {
                                    if parent ==
                                           (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                                       {
                                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                                            = tmp
                                    } else {
                                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right
                                            = tmp
                                    }
                                } else { (*head).rbh_root = tmp }
                                (*tmp).tree_entry.rbe_right = parent;
                                (*parent).tree_entry.rbe_parent = tmp;
                                while 0 != 0i32 { }
                                if !(*tmp).tree_entry.rbe_parent.is_null() {
                                    current_block = 980989089337379490;
                                } else {
                                    current_block = 5494826135382683477;
                                }
                                loop  {
                                    match current_block {
                                        5494826135382683477 => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    652864300344834934;
                                                continue 's_211 ;
                                            } else { break ; }
                                        }
                                        _ => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    980989089337379490;
                                            } else {
                                                current_block =
                                                    5494826135382683477;
                                            }
                                        }
                                    }
                                }
                                tmp = parent;
                                parent = elm;
                                elm = tmp;
                                current_block = 4567019141635105728;
                            }
                            _ => {
                                (*parent).tree_entry.rbe_color = 0i32;
                                (*gparent).tree_entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4567019141635105728;
                                } else { break ; }
                            }
                        }
                    }
                's_219:
                    loop  {
                        tmp = (*gparent).tree_entry.rbe_right;
                        (*gparent).tree_entry.rbe_right =
                            (*tmp).tree_entry.rbe_left;
                        if !(*gparent).tree_entry.rbe_right.is_null() {
                            (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_parent
                                = gparent
                        }
                        while 0 != 0i32 { }
                        (*tmp).tree_entry.rbe_parent =
                            (*gparent).tree_entry.rbe_parent;
                        if !(*tmp).tree_entry.rbe_parent.is_null() {
                            if gparent ==
                                   (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_left
                               {
                                (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_left
                                    = tmp
                            } else {
                                (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_right
                                    = tmp
                            }
                        } else { (*head).rbh_root = tmp }
                        (*tmp).tree_entry.rbe_left = gparent;
                        (*gparent).tree_entry.rbe_parent = tmp;
                        while 0 != 0i32 { }
                        if !(*tmp).tree_entry.rbe_parent.is_null() {
                            current_block = 11793792312832361944;
                        } else { current_block = 2543120759711851213; }
                        loop  {
                            match current_block {
                                2543120759711851213 => {
                                    if 0 != 0i32 {
                                        break ;
                                    } else { break 's_219 ; }
                                }
                                _ => {
                                    if 0 != 0i32 {
                                        current_block = 11793792312832361944;
                                    } else {
                                        current_block = 2543120759711851213;
                                    }
                                }
                            }
                        }
                    }
            }
        }
    }
    (*(*head).rbh_root).tree_entry.rbe_color = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_REMOVE_COLOR(mut head:
                                                              *mut window_pane_tree,
                                                          mut parent:
                                                              *mut window_pane,
                                                          mut elm:
                                                              *mut window_pane)
 -> () {
    let mut current_block: u64;
    let mut tmp: *mut window_pane = 0 as *mut window_pane;
    's_4:
        loop  {
            if !((elm == 0 as *mut libc::c_void as *mut window_pane ||
                      (*elm).tree_entry.rbe_color == 0i32) &&
                     elm != (*head).rbh_root) {
                current_block = 11174649648027449784;
                break ;
            }
            if (*parent).tree_entry.rbe_left == elm {
                tmp = (*parent).tree_entry.rbe_right;
                if (*tmp).tree_entry.rbe_color == 1i32 {
                    current_block = 17179679302217393232;
                } else { current_block = 14155750587950065367; }
                loop  {
                    match current_block {
                        17179679302217393232 => {
                            (*tmp).tree_entry.rbe_color = 0i32;
                            (*parent).tree_entry.rbe_color = 1i32;
                            if 0 != 0i32 {
                                current_block = 17179679302217393232;
                                continue ;
                            }
                            's_30:
                                loop  {
                                    tmp = (*parent).tree_entry.rbe_right;
                                    (*parent).tree_entry.rbe_right =
                                        (*tmp).tree_entry.rbe_left;
                                    if !(*parent).tree_entry.rbe_right.is_null()
                                       {
                                        (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_parent
                                            = parent
                                    }
                                    while 0 != 0i32 { }
                                    (*tmp).tree_entry.rbe_parent =
                                        (*parent).tree_entry.rbe_parent;
                                    if !(*tmp).tree_entry.rbe_parent.is_null()
                                       {
                                        if parent ==
                                               (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                                           {
                                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                                                = tmp
                                        } else {
                                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right
                                                = tmp
                                        }
                                    } else { (*head).rbh_root = tmp }
                                    (*tmp).tree_entry.rbe_left = parent;
                                    (*parent).tree_entry.rbe_parent = tmp;
                                    while 0 != 0i32 { }
                                    if !(*tmp).tree_entry.rbe_parent.is_null()
                                       {
                                        current_block = 11050875288958768710;
                                    } else {
                                        current_block = 15240798224410183470;
                                    }
                                    loop  {
                                        match current_block {
                                            15240798224410183470 => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_30 ; }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        11050875288958768710;
                                                } else {
                                                    current_block =
                                                        15240798224410183470;
                                                }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).tree_entry.rbe_right;
                            current_block = 14155750587950065367;
                        }
                        _ => {
                            if ((*tmp).tree_entry.rbe_left ==
                                    0 as *mut libc::c_void as *mut window_pane
                                    ||
                                    (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).tree_entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut window_pane ||
                                        (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_color
                                            == 0i32) {
                                (*tmp).tree_entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).tree_entry.rbe_parent;
                                break ;
                            } else if (*tmp).tree_entry.rbe_right ==
                                          0 as *mut libc::c_void as
                                              *mut window_pane ||
                                          (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_color
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
                tmp = (*parent).tree_entry.rbe_left;
                if (*tmp).tree_entry.rbe_color == 1i32 {
                    current_block = 6450597802325118133;
                } else { current_block = 7746103178988627676; }
                loop  {
                    match current_block {
                        6450597802325118133 => {
                            (*tmp).tree_entry.rbe_color = 0i32;
                            (*parent).tree_entry.rbe_color = 1i32;
                            if 0 != 0i32 {
                                current_block = 6450597802325118133;
                                continue ;
                            }
                            's_210:
                                loop  {
                                    tmp = (*parent).tree_entry.rbe_left;
                                    (*parent).tree_entry.rbe_left =
                                        (*tmp).tree_entry.rbe_right;
                                    if !(*parent).tree_entry.rbe_left.is_null()
                                       {
                                        (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_parent
                                            = parent
                                    }
                                    while 0 != 0i32 { }
                                    (*tmp).tree_entry.rbe_parent =
                                        (*parent).tree_entry.rbe_parent;
                                    if !(*tmp).tree_entry.rbe_parent.is_null()
                                       {
                                        if parent ==
                                               (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                                           {
                                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                                                = tmp
                                        } else {
                                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right
                                                = tmp
                                        }
                                    } else { (*head).rbh_root = tmp }
                                    (*tmp).tree_entry.rbe_right = parent;
                                    (*parent).tree_entry.rbe_parent = tmp;
                                    while 0 != 0i32 { }
                                    if !(*tmp).tree_entry.rbe_parent.is_null()
                                       {
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
                            tmp = (*parent).tree_entry.rbe_left;
                            current_block = 7746103178988627676;
                        }
                        _ => {
                            if ((*tmp).tree_entry.rbe_left ==
                                    0 as *mut libc::c_void as *mut window_pane
                                    ||
                                    (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).tree_entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut window_pane ||
                                        (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_color
                                            == 0i32) {
                                (*tmp).tree_entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).tree_entry.rbe_parent;
                                break ;
                            } else if (*tmp).tree_entry.rbe_left ==
                                          0 as *mut libc::c_void as
                                              *mut window_pane ||
                                          (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_color
                                              == 0i32 {
                                current_block = 13826291924415791078;
                                break 's_4 ;
                            } else {
                                current_block = 5892776923941496671;
                                break 's_4 ;
                            }
                        }
                    }
                }
            }
        }
    match current_block {
        13826291924415791078 => {
            let mut oright: *mut window_pane = 0 as *mut window_pane;
            oright = (*tmp).tree_entry.rbe_right;
            if !oright.is_null() { (*oright).tree_entry.rbe_color = 0i32 }
            (*tmp).tree_entry.rbe_color = 1i32;
            's_276:
                loop  {
                    oright = (*tmp).tree_entry.rbe_right;
                    (*tmp).tree_entry.rbe_right =
                        (*oright).tree_entry.rbe_left;
                    if !(*tmp).tree_entry.rbe_right.is_null() {
                        (*(*oright).tree_entry.rbe_left).tree_entry.rbe_parent
                            = tmp
                    }
                    while 0 != 0i32 { }
                    (*oright).tree_entry.rbe_parent =
                        (*tmp).tree_entry.rbe_parent;
                    if !(*oright).tree_entry.rbe_parent.is_null() {
                        if tmp ==
                               (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_left
                           {
                            (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_left
                                = oright
                        } else {
                            (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_right
                                = oright
                        }
                    } else { (*head).rbh_root = oright }
                    (*oright).tree_entry.rbe_left = tmp;
                    (*tmp).tree_entry.rbe_parent = oright;
                    while 0 != 0i32 { }
                    if !(*oright).tree_entry.rbe_parent.is_null() {
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
            tmp = (*parent).tree_entry.rbe_left;
            current_block = 5892776923941496671;
        }
        15976848397966268834 => {
            let mut oleft: *mut window_pane = 0 as *mut window_pane;
            oleft = (*tmp).tree_entry.rbe_left;
            if !oleft.is_null() { (*oleft).tree_entry.rbe_color = 0i32 }
            (*tmp).tree_entry.rbe_color = 1i32;
            's_96:
                loop  {
                    oleft = (*tmp).tree_entry.rbe_left;
                    (*tmp).tree_entry.rbe_left =
                        (*oleft).tree_entry.rbe_right;
                    if !(*tmp).tree_entry.rbe_left.is_null() {
                        (*(*oleft).tree_entry.rbe_right).tree_entry.rbe_parent
                            = tmp
                    }
                    while 0 != 0i32 { }
                    (*oleft).tree_entry.rbe_parent =
                        (*tmp).tree_entry.rbe_parent;
                    if !(*oleft).tree_entry.rbe_parent.is_null() {
                        if tmp ==
                               (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_left
                           {
                            (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_left
                                = oleft
                        } else {
                            (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_right
                                = oleft
                        }
                    } else { (*head).rbh_root = oleft }
                    (*oleft).tree_entry.rbe_right = tmp;
                    (*tmp).tree_entry.rbe_parent = oleft;
                    while 0 != 0i32 { }
                    if !(*oleft).tree_entry.rbe_parent.is_null() {
                        current_block = 2232869372362427478;
                    } else { current_block = 15904375183555213903; }
                    loop  {
                        match current_block {
                            2232869372362427478 => {
                                if 0 != 0i32 {
                                    current_block = 2232869372362427478;
                                } else {
                                    current_block = 15904375183555213903;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_96 ; }
                            }
                        }
                    }
                }
            tmp = (*parent).tree_entry.rbe_right;
            current_block = 7149356873433890176;
        }
        _ => { }
    }
    match current_block {
        5892776923941496671 => {
            (*tmp).tree_entry.rbe_color = (*parent).tree_entry.rbe_color;
            (*parent).tree_entry.rbe_color = 0i32;
            if !(*tmp).tree_entry.rbe_left.is_null() {
                (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_color = 0i32
            }
            's_328:
                loop  {
                    tmp = (*parent).tree_entry.rbe_left;
                    (*parent).tree_entry.rbe_left =
                        (*tmp).tree_entry.rbe_right;
                    if !(*parent).tree_entry.rbe_left.is_null() {
                        (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_parent =
                            parent
                    }
                    while 0 != 0i32 { }
                    (*tmp).tree_entry.rbe_parent =
                        (*parent).tree_entry.rbe_parent;
                    if !(*tmp).tree_entry.rbe_parent.is_null() {
                        if parent ==
                               (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                           {
                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                                = tmp
                        } else {
                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right
                                = tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).tree_entry.rbe_right = parent;
                    (*parent).tree_entry.rbe_parent = tmp;
                    while 0 != 0i32 { }
                    if !(*tmp).tree_entry.rbe_parent.is_null() {
                        current_block = 13910774313357589740;
                    } else { current_block = 13707613154239713890; }
                    loop  {
                        match current_block {
                            13910774313357589740 => {
                                if 0 != 0i32 {
                                    current_block = 13910774313357589740;
                                } else {
                                    current_block = 13707613154239713890;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_328 ; }
                            }
                        }
                    }
                }
            elm = (*head).rbh_root
        }
        7149356873433890176 => {
            (*tmp).tree_entry.rbe_color = (*parent).tree_entry.rbe_color;
            (*parent).tree_entry.rbe_color = 0i32;
            if !(*tmp).tree_entry.rbe_right.is_null() {
                (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_color = 0i32
            }
            's_148:
                loop  {
                    tmp = (*parent).tree_entry.rbe_right;
                    (*parent).tree_entry.rbe_right =
                        (*tmp).tree_entry.rbe_left;
                    if !(*parent).tree_entry.rbe_right.is_null() {
                        (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_parent =
                            parent
                    }
                    while 0 != 0i32 { }
                    (*tmp).tree_entry.rbe_parent =
                        (*parent).tree_entry.rbe_parent;
                    if !(*tmp).tree_entry.rbe_parent.is_null() {
                        if parent ==
                               (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                           {
                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                                = tmp
                        } else {
                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right
                                = tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).tree_entry.rbe_left = parent;
                    (*parent).tree_entry.rbe_parent = tmp;
                    while 0 != 0i32 { }
                    if !(*tmp).tree_entry.rbe_parent.is_null() {
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
        _ => { }
    }
    if !elm.is_null() { (*elm).tree_entry.rbe_color = 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_REMOVE(mut head:
                                                        *mut window_pane_tree,
                                                    mut elm: *mut window_pane)
 -> *mut window_pane {
    let mut current_block: u64;
    let mut child: *mut window_pane = 0 as *mut window_pane;
    let mut parent: *mut window_pane = 0 as *mut window_pane;
    let mut old: *mut window_pane = elm;
    let mut color: libc::c_int = 0;
    if (*elm).tree_entry.rbe_left ==
           0 as *mut libc::c_void as *mut window_pane {
        child = (*elm).tree_entry.rbe_right;
        current_block = 9386390421034826751;
    } else if (*elm).tree_entry.rbe_right ==
                  0 as *mut libc::c_void as *mut window_pane {
        child = (*elm).tree_entry.rbe_left;
        current_block = 9386390421034826751;
    } else {
        let mut left: *mut window_pane = 0 as *mut window_pane;
        elm = (*elm).tree_entry.rbe_right;
        loop  {
            left = (*elm).tree_entry.rbe_left;
            if left.is_null() { break ; }
            elm = left
        }
        child = (*elm).tree_entry.rbe_right;
        parent = (*elm).tree_entry.rbe_parent;
        color = (*elm).tree_entry.rbe_color;
        if !child.is_null() { (*child).tree_entry.rbe_parent = parent }
        if !parent.is_null() {
            if (*parent).tree_entry.rbe_left == elm {
                (*parent).tree_entry.rbe_left = child
            } else { (*parent).tree_entry.rbe_right = child }
            while 0 != 0i32 { }
        } else { (*head).rbh_root = child }
        if (*elm).tree_entry.rbe_parent == old { parent = elm }
        (*elm).tree_entry = (*old).tree_entry;
        if !(*old).tree_entry.rbe_parent.is_null() {
            if (*(*old).tree_entry.rbe_parent).tree_entry.rbe_left == old {
                (*(*old).tree_entry.rbe_parent).tree_entry.rbe_left = elm
            } else {
                (*(*old).tree_entry.rbe_parent).tree_entry.rbe_right = elm
            }
            while 0 != 0i32 { }
        } else { (*head).rbh_root = elm }
        (*(*old).tree_entry.rbe_left).tree_entry.rbe_parent = elm;
        if !(*old).tree_entry.rbe_right.is_null() {
            (*(*old).tree_entry.rbe_right).tree_entry.rbe_parent = elm
        }
        if !parent.is_null() {
            left = parent;
            loop  {
                if 0 != 0i32 { continue ; }
                left = (*left).tree_entry.rbe_parent;
                if left.is_null() { break ; }
            }
            current_block = 10175054104804762244;
        } else { current_block = 10175054104804762244; }
    }
    match current_block {
        9386390421034826751 => {
            parent = (*elm).tree_entry.rbe_parent;
            color = (*elm).tree_entry.rbe_color;
            if !child.is_null() { (*child).tree_entry.rbe_parent = parent }
            if !parent.is_null() {
                if (*parent).tree_entry.rbe_left == elm {
                    (*parent).tree_entry.rbe_left = child
                } else { (*parent).tree_entry.rbe_right = child }
                while 0 != 0i32 { }
            } else { (*head).rbh_root = child }
        }
        _ => { }
    }
    if color == 0i32 {
        window_pane_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_INSERT(mut head:
                                                        *mut window_pane_tree,
                                                    mut elm: *mut window_pane)
 -> *mut window_pane {
    let mut tmp: *mut window_pane = 0 as *mut window_pane;
    let mut parent: *mut window_pane = 0 as *mut window_pane;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = window_pane_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).tree_entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).tree_entry.rbe_right
        } else { return tmp }
    }
    loop  {
        (*elm).tree_entry.rbe_parent = parent;
        (*elm).tree_entry.rbe_right = 0 as *mut window_pane;
        (*elm).tree_entry.rbe_left = (*elm).tree_entry.rbe_right;
        (*elm).tree_entry.rbe_color = 1i32;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut window_pane {
        if comp < 0i32 {
            (*parent).tree_entry.rbe_left = elm
        } else { (*parent).tree_entry.rbe_right = elm }
        while 0 != 0i32 { }
    } else { (*head).rbh_root = elm }
    window_pane_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut window_pane;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_FIND(mut head:
                                                      *mut window_pane_tree,
                                                  mut elm: *mut window_pane)
 -> *mut window_pane {
    let mut tmp: *mut window_pane = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = window_pane_cmp(elm, tmp);
            if comp < 0i32 {
                tmp = (*tmp).tree_entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).tree_entry.rbe_right
            } else { return tmp }
        } else { return 0 as *mut window_pane }
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_NFIND(mut head:
                                                       *mut window_pane_tree,
                                                   mut elm: *mut window_pane)
 -> *mut window_pane {
    let mut tmp: *mut window_pane = (*head).rbh_root;
    let mut res: *mut window_pane = 0 as *mut window_pane;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = window_pane_cmp(elm, tmp);
            if comp < 0i32 {
                res = tmp;
                tmp = (*tmp).tree_entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).tree_entry.rbe_right
            } else { return tmp }
        } else { return res }
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_NEXT(mut elm: *mut window_pane)
 -> *mut window_pane {
    if !(*elm).tree_entry.rbe_right.is_null() {
        elm = (*elm).tree_entry.rbe_right;
        while !(*elm).tree_entry.rbe_left.is_null() {
            elm = (*elm).tree_entry.rbe_left
        }
    } else if !(*elm).tree_entry.rbe_parent.is_null() &&
                  elm == (*(*elm).tree_entry.rbe_parent).tree_entry.rbe_left {
        elm = (*elm).tree_entry.rbe_parent
    } else {
        while !(*elm).tree_entry.rbe_parent.is_null() &&
                  elm == (*(*elm).tree_entry.rbe_parent).tree_entry.rbe_right
              {
            elm = (*elm).tree_entry.rbe_parent
        }
        elm = (*elm).tree_entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_PREV(mut elm: *mut window_pane)
 -> *mut window_pane {
    if !(*elm).tree_entry.rbe_left.is_null() {
        elm = (*elm).tree_entry.rbe_left;
        while !(*elm).tree_entry.rbe_right.is_null() {
            elm = (*elm).tree_entry.rbe_right
        }
    } else if !(*elm).tree_entry.rbe_parent.is_null() &&
                  elm == (*(*elm).tree_entry.rbe_parent).tree_entry.rbe_right
     {
        elm = (*elm).tree_entry.rbe_parent
    } else {
        while !(*elm).tree_entry.rbe_parent.is_null() &&
                  elm == (*(*elm).tree_entry.rbe_parent).tree_entry.rbe_left {
            elm = (*elm).tree_entry.rbe_parent
        }
        elm = (*elm).tree_entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_MINMAX(mut head:
                                                        *mut window_pane_tree,
                                                    mut val: libc::c_int)
 -> *mut window_pane {
    let mut tmp: *mut window_pane = (*head).rbh_root;
    let mut parent: *mut window_pane = 0 as *mut window_pane;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).tree_entry.rbe_left
        } else { tmp = (*tmp).tree_entry.rbe_right }
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_find_by_index(mut wwl: *mut winlinks,
                                               mut idx: libc::c_int)
 -> *mut winlink {
    let mut wl: winlink =
        winlink{idx: 0,
                session: 0 as *mut session,
                window: 0 as *mut window,
                status_width: 0,
                status_cell:
                    grid_cell{flags: 0,
                              attr: 0,
                              fg: 0,
                              bg: 0,
                              data:
                                  utf8_data{data: [0; 9],
                                            have: 0,
                                            size: 0,
                                            width: 0,},},
                status_text: 0 as *mut libc::c_char,
                flags: 0,
                entry:
                    unnamed_31{rbe_left: 0 as *mut winlink,
                               rbe_right: 0 as *mut winlink,
                               rbe_parent: 0 as *mut winlink,
                               rbe_color: 0,},
                wentry:
                    unnamed_19{tqe_next: 0 as *mut winlink,
                               tqe_prev: 0 as *mut *mut winlink,},
                sentry:
                    unnamed_7{tqe_next: 0 as *mut winlink,
                              tqe_prev: 0 as *mut *mut winlink,},};
    if idx < 0i32 {
        fatalx(b"bad index\x00" as *const u8 as *const libc::c_char);
    } else {
        wl.idx = idx;
        return winlinks_RB_FIND(wwl, &mut wl as *mut winlink)
    };
}
#[no_mangle]
pub unsafe extern "C" fn winlink_find_by_window(mut wwl: *mut winlinks,
                                                mut w: *mut window)
 -> *mut winlink {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(wwl, 1i32.wrapping_neg());
    loop  {
        if wl != 0 as *mut libc::c_void as *mut winlink {
            if (*wl).window == w {
                return wl
            } else { wl = winlinks_RB_NEXT(wl) }
        } else { return 0 as *mut winlink }
    };
}
#[no_mangle]
pub unsafe extern "C" fn winlink_find_by_window_id(mut wwl: *mut winlinks,
                                                   mut id: u_int)
 -> *mut winlink {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(wwl, 1i32.wrapping_neg());
    loop  {
        if wl != 0 as *mut libc::c_void as *mut winlink {
            if (*(*wl).window).id == id {
                return wl
            } else { wl = winlinks_RB_NEXT(wl) }
        } else { return 0 as *mut winlink }
    };
}
#[no_mangle]
pub unsafe extern "C" fn winlink_count(mut wwl: *mut winlinks) -> u_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut n: u_int = 0;
    n = 0i32 as u_int;
    wl = winlinks_RB_MINMAX(wwl, 1i32.wrapping_neg());
    while wl != 0 as *mut libc::c_void as *mut winlink {
        n = n.wrapping_add(1);
        wl = winlinks_RB_NEXT(wl)
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_add(mut wwl: *mut winlinks,
                                     mut idx: libc::c_int) -> *mut winlink {
    let mut wl: *mut winlink = 0 as *mut winlink;
    if idx < 0i32 {
        idx = winlink_next_index(wwl, idx.wrapping_neg() - 1i32);
        if idx == 1i32.wrapping_neg() { return 0 as *mut winlink }
    } else if winlink_find_by_index(wwl, idx) !=
                  0 as *mut libc::c_void as *mut winlink {
        return 0 as *mut winlink
    }
    wl =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<winlink>() as libc::c_ulong) as
            *mut winlink;
    (*wl).idx = idx;
    winlinks_RB_INSERT(wwl, wl);
    return wl;
}
unsafe extern "C" fn winlink_next_index(mut wwl: *mut winlinks,
                                        mut idx: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = idx;
    loop  {
        if winlink_find_by_index(wwl, i) ==
               0 as *mut libc::c_void as *mut winlink {
            return i
        } else {
            if i == 2147483647i32 { i = 0i32 } else { i += 1 }
            if i != idx { continue ; }
            return 1i32.wrapping_neg()
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn winlink_set_window(mut wl: *mut winlink,
                                            mut w: *mut window) -> () {
    let mut current_block: u64;
    if (*wl).window != 0 as *mut libc::c_void as *mut window {
        current_block = 6239978542346980191;
    } else { current_block = 7502529970979898288; }
    loop  {
        match current_block {
            7502529970979898288 => {
                (*wl).wentry.tqe_next = 0 as *mut winlink;
                (*wl).wentry.tqe_prev =
                    (*(&mut (*w).winlinks as *mut unnamed_11)).tqh_last;
                let ref mut fresh1 =
                    *(*(&mut (*w).winlinks as *mut unnamed_11)).tqh_last;
                *fresh1 = wl;
                let ref mut fresh2 =
                    (*(&mut (*w).winlinks as *mut unnamed_11)).tqh_last;
                *fresh2 = &mut (*wl).wentry.tqe_next as *mut *mut winlink;
                if 0 != 0i32 {
                    current_block = 7502529970979898288;
                } else { break ; }
            }
            _ => {
                if (*wl).wentry.tqe_next !=
                       0 as *mut libc::c_void as *mut winlink {
                    (*(*wl).wentry.tqe_next).wentry.tqe_prev =
                        (*wl).wentry.tqe_prev
                } else {
                    let ref mut fresh0 =
                        (*(&mut (*(*wl).window).winlinks as
                               *mut unnamed_11)).tqh_last;
                    *fresh0 = (*wl).wentry.tqe_prev
                }
                *(*wl).wentry.tqe_prev = (*wl).wentry.tqe_next;
                if 0 != 0i32 {
                    current_block = 6239978542346980191;
                    continue ;
                }
                window_remove_ref((*wl).window,
                                  (*::std::mem::transmute::<&[u8; 19],
                                                            &[libc::c_char; 19]>(b"winlink_set_window\x00")).as_ptr());
                current_block = 7502529970979898288;
            }
        }
    }
    (*wl).window = w;
    window_add_ref(w,
                   (*::std::mem::transmute::<&[u8; 19],
                                             &[libc::c_char; 19]>(b"winlink_set_window\x00")).as_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn window_add_ref(mut w: *mut window,
                                        mut from: *const libc::c_char) -> () {
    (*w).references = (*w).references.wrapping_add(1);
    log_debug(b"%s: @%u %s, now %d\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"window_add_ref\x00")).as_ptr(),
              (*w).id, from, (*w).references);
}
#[no_mangle]
pub unsafe extern "C" fn window_remove_ref(mut w: *mut window,
                                           mut from: *const libc::c_char)
 -> () {
    (*w).references = (*w).references.wrapping_sub(1);
    log_debug(b"%s: @%u %s, now %d\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"window_remove_ref\x00")).as_ptr(),
              (*w).id, from, (*w).references);
    if (*w).references == 0i32 as libc::c_uint { window_destroy(w); };
}
unsafe extern "C" fn window_destroy(mut w: *mut window) -> () {
    log_debug(b"window @%u destroyed (%d references)\x00" as *const u8 as
                  *const libc::c_char, (*w).id, (*w).references);
    windows_RB_REMOVE(&mut windows_static as *mut windows, w);
    if (*w).layout_root != 0 as *mut libc::c_void as *mut layout_cell {
        layout_free_cell((*w).layout_root);
    }
    if (*w).saved_layout_root != 0 as *mut libc::c_void as *mut layout_cell {
        layout_free_cell((*w).saved_layout_root);
    }
    free((*w).old_layout as *mut libc::c_void);
    if 0 != event_initialized(&mut (*w).name_event as *mut event) {
        event_del(&mut (*w).name_event as *mut event);
    }
    if 0 != event_initialized(&mut (*w).alerts_timer as *mut event) {
        event_del(&mut (*w).alerts_timer as *mut event);
    }
    options_free((*w).options);
    window_destroy_panes(w);
    free((*w).name as *mut libc::c_void);
    free(w as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_destroy_panes(mut w: *mut window) -> () {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    while 0 !=
              !((*(&mut (*w).panes as *mut window_panes)).tqh_first ==
                    0 as *mut libc::c_void as *mut window_pane) as libc::c_int
          {
        wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
        loop  {
            if (*wp).entry.tqe_next !=
                   0 as *mut libc::c_void as *mut window_pane {
                (*(*wp).entry.tqe_next).entry.tqe_prev = (*wp).entry.tqe_prev
            } else {
                let ref mut fresh3 =
                    (*(&mut (*w).panes as *mut window_panes)).tqh_last;
                *fresh3 = (*wp).entry.tqe_prev
            }
            *(*wp).entry.tqe_prev = (*wp).entry.tqe_next;
            if !(0 != 0i32) { break ; }
        }
        window_pane_destroy(wp);
    };
}
unsafe extern "C" fn window_pane_destroy(mut wp: *mut window_pane) -> () {
    window_pane_reset_mode(wp);
    free((*wp).searchstr as *mut libc::c_void);
    if (*wp).fd != 1i32.wrapping_neg() {
        bufferevent_free((*wp).event);
        close((*wp).fd);
    }
    input_free(wp);
    screen_free(&mut (*wp).base as *mut screen);
    if (*wp).saved_grid != 0 as *mut libc::c_void as *mut grid {
        grid_destroy((*wp).saved_grid);
    }
    if (*wp).pipe_fd != 1i32.wrapping_neg() {
        bufferevent_free((*wp).pipe_event);
        close((*wp).pipe_fd);
    }
    if 0 != event_initialized(&mut (*wp).resize_timer as *mut event) {
        event_del(&mut (*wp).resize_timer as *mut event);
    }
    window_pane_tree_RB_REMOVE(&mut all_window_panes as *mut window_pane_tree,
                               wp);
    free((*wp).cwd as *mut libc::c_void);
    free((*wp).shell as *mut libc::c_void);
    cmd_free_argv((*wp).argc, (*wp).argv);
    free((*wp).palette as *mut libc::c_void);
    free(wp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_reset_mode(mut wp: *mut window_pane)
 -> () {
    if (*wp).mode == 0 as *mut libc::c_void as *const window_mode {
        return
    } else {
        event_del(&mut (*wp).modetimer as *mut event);
        (*(*wp).mode).free.expect("non-null function pointer")(wp);
        (*wp).mode = 0 as *const window_mode;
        (*wp).modeprefix = 1i32 as u_int;
        (*wp).screen = &mut (*wp).base as *mut screen;
        (*wp).flags |= 1i32 | 128i32;
        server_status_window((*wp).window);
        notify_pane(b"pane-mode-changed\x00" as *const u8 as
                        *const libc::c_char, wp);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn winlink_remove(mut wwl: *mut winlinks,
                                        mut wl: *mut winlink) -> () {
    let mut current_block: u64;
    let mut w: *mut window = (*wl).window;
    if w != 0 as *mut libc::c_void as *mut window {
        current_block = 16559507199688588974;
    } else { current_block = 792017965103506125; }
    loop  {
        match current_block {
            792017965103506125 => {
                winlinks_RB_REMOVE(wwl, wl);
                free((*wl).status_text as *mut libc::c_void);
                free(wl as *mut libc::c_void);
                break ;
            }
            _ => {
                if (*wl).wentry.tqe_next !=
                       0 as *mut libc::c_void as *mut winlink {
                    (*(*wl).wentry.tqe_next).wentry.tqe_prev =
                        (*wl).wentry.tqe_prev
                } else {
                    let ref mut fresh4 =
                        (*(&mut (*w).winlinks as *mut unnamed_11)).tqh_last;
                    *fresh4 = (*wl).wentry.tqe_prev
                }
                *(*wl).wentry.tqe_prev = (*wl).wentry.tqe_next;
                if 0 != 0i32 {
                    current_block = 16559507199688588974;
                    continue ;
                }
                window_remove_ref(w,
                                  (*::std::mem::transmute::<&[u8; 15],
                                                            &[libc::c_char; 15]>(b"winlink_remove\x00")).as_ptr());
                current_block = 792017965103506125;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn winlink_next(mut wl: *mut winlink) -> *mut winlink {
    return winlinks_RB_NEXT(wl);
}
#[no_mangle]
pub unsafe extern "C" fn winlink_previous(mut wl: *mut winlink)
 -> *mut winlink {
    return winlinks_RB_PREV(wl);
}
#[no_mangle]
pub unsafe extern "C" fn winlink_next_by_number(mut wl: *mut winlink,
                                                mut s: *mut session,
                                                mut n: libc::c_int)
 -> *mut winlink {
    while n > 0i32 {
        wl = winlinks_RB_NEXT(wl);
        if wl == 0 as *mut libc::c_void as *mut winlink {
            wl =
                winlinks_RB_MINMAX(&mut (*s).windows as *mut winlinks,
                                   1i32.wrapping_neg())
        }
        n -= 1
    }
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_previous_by_number(mut wl: *mut winlink,
                                                    mut s: *mut session,
                                                    mut n: libc::c_int)
 -> *mut winlink {
    while n > 0i32 {
        wl = winlinks_RB_PREV(wl);
        if wl == 0 as *mut libc::c_void as *mut winlink {
            wl = winlinks_RB_MINMAX(&mut (*s).windows as *mut winlinks, 1i32)
        }
        n -= 1
    }
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_stack_push(mut stack: *mut winlink_stack,
                                            mut wl: *mut winlink) -> () {
    if wl == 0 as *mut libc::c_void as *mut winlink {
        return
    } else {
        winlink_stack_remove(stack, wl);
        loop  {
            (*wl).sentry.tqe_next = (*stack).tqh_first;
            if (*wl).sentry.tqe_next != 0 as *mut libc::c_void as *mut winlink
               {
                (*(*stack).tqh_first).sentry.tqe_prev =
                    &mut (*wl).sentry.tqe_next as *mut *mut winlink
            } else {
                (*stack).tqh_last =
                    &mut (*wl).sentry.tqe_next as *mut *mut winlink
            }
            (*stack).tqh_first = wl;
            (*wl).sentry.tqe_prev =
                &mut (*stack).tqh_first as *mut *mut winlink;
            if !(0 != 0i32) { break ; }
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn winlink_stack_remove(mut stack: *mut winlink_stack,
                                              mut wl: *mut winlink) -> () {
    let mut wl2: *mut winlink = 0 as *mut winlink;
    if wl == 0 as *mut libc::c_void as *mut winlink {
        return
    } else {
        wl2 = (*stack).tqh_first;
        loop  {
            if wl2 != 0 as *mut libc::c_void as *mut winlink {
                if wl2 == wl { break ; }
                wl2 = (*wl2).sentry.tqe_next
            } else { return; }
        }
        loop  {
            if (*wl).sentry.tqe_next != 0 as *mut libc::c_void as *mut winlink
               {
                (*(*wl).sentry.tqe_next).sentry.tqe_prev =
                    (*wl).sentry.tqe_prev
            } else { (*stack).tqh_last = (*wl).sentry.tqe_prev }
            *(*wl).sentry.tqe_prev = (*wl).sentry.tqe_next;
            if !(0 != 0i32) { break ; }
        }
        return
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_find_by_id_str(mut s: *const libc::c_char)
 -> *mut window {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut id: u_int = 0;
    if *s as libc::c_int != 64 {
        return 0 as *mut window
    } else {
        id =
            strtonum(s.offset(1isize), 0i32 as libc::c_longlong,
                     (2147483647i32 as
                          libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                         as libc::c_longlong,
                     &mut errstr as *mut *const libc::c_char) as u_int;
        if errstr != 0 as *mut libc::c_void as *const libc::c_char {
            return 0 as *mut window
        } else { return window_find_by_id(id) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_find_by_id(mut id: u_int) -> *mut window {
    let mut w: window =
        window{id: 0,
               name: 0 as *mut libc::c_char,
               name_event:
                   event{ev_active_next:
                             unnamed{tqe_next: 0 as *mut event,
                                     tqe_prev: 0 as *mut *mut event,},
                         ev_next:
                             unnamed_39{tqe_next: 0 as *mut event,
                                        tqe_prev: 0 as *mut *mut event,},
                         ev_timeout_pos:
                             unnamed_0{ev_next_with_common_timeout:
                                           unnamed_38{tqe_next:
                                                          0 as *mut event,
                                                      tqe_prev:
                                                          0 as
                                                              *mut *mut event,},},
                         ev_fd: 0,
                         ev_base: 0 as *mut event_base,
                         _ev:
                             unnamed_27{ev_io:
                                            unnamed_13{ev_io_next:
                                                           unnamed_20{tqe_next:
                                                                          0 as
                                                                              *mut event,
                                                                      tqe_prev:
                                                                          0 as
                                                                              *mut *mut event,},
                                                       ev_timeout:
                                                           timeval{tv_sec: 0,
                                                                   tv_usec:
                                                                       0,},},},
                         ev_events: 0,
                         ev_res: 0,
                         ev_flags: 0,
                         ev_pri: 0,
                         ev_closure: 0,
                         ev_timeout: timeval{tv_sec: 0, tv_usec: 0,},
                         ev_callback: None,
                         ev_arg: 0 as *mut libc::c_void,},
               name_time: timeval{tv_sec: 0, tv_usec: 0,},
               alerts_timer:
                   event{ev_active_next:
                             unnamed{tqe_next: 0 as *mut event,
                                     tqe_prev: 0 as *mut *mut event,},
                         ev_next:
                             unnamed_39{tqe_next: 0 as *mut event,
                                        tqe_prev: 0 as *mut *mut event,},
                         ev_timeout_pos:
                             unnamed_0{ev_next_with_common_timeout:
                                           unnamed_38{tqe_next:
                                                          0 as *mut event,
                                                      tqe_prev:
                                                          0 as
                                                              *mut *mut event,},},
                         ev_fd: 0,
                         ev_base: 0 as *mut event_base,
                         _ev:
                             unnamed_27{ev_io:
                                            unnamed_13{ev_io_next:
                                                           unnamed_20{tqe_next:
                                                                          0 as
                                                                              *mut event,
                                                                      tqe_prev:
                                                                          0 as
                                                                              *mut *mut event,},
                                                       ev_timeout:
                                                           timeval{tv_sec: 0,
                                                                   tv_usec:
                                                                       0,},},},
                         ev_events: 0,
                         ev_res: 0,
                         ev_flags: 0,
                         ev_pri: 0,
                         ev_closure: 0,
                         ev_timeout: timeval{tv_sec: 0, tv_usec: 0,},
                         ev_callback: None,
                         ev_arg: 0 as *mut libc::c_void,},
               activity_time: timeval{tv_sec: 0, tv_usec: 0,},
               active: 0 as *mut window_pane,
               last: 0 as *mut window_pane,
               panes:
                   window_panes{tqh_first: 0 as *mut window_pane,
                                tqh_last: 0 as *mut *mut window_pane,},
               lastlayout: 0,
               layout_root: 0 as *mut layout_cell,
               saved_layout_root: 0 as *mut layout_cell,
               old_layout: 0 as *mut libc::c_char,
               sx: 0,
               sy: 0,
               flags: 0,
               alerts_queued: 0,
               alerts_entry:
                   unnamed_17{tqe_next: 0 as *mut window,
                              tqe_prev: 0 as *mut *mut window,},
               options: 0 as *mut options,
               style:
                   grid_cell{flags: 0,
                             attr: 0,
                             fg: 0,
                             bg: 0,
                             data:
                                 utf8_data{data: [0; 9],
                                           have: 0,
                                           size: 0,
                                           width: 0,},},
               active_style:
                   grid_cell{flags: 0,
                             attr: 0,
                             fg: 0,
                             bg: 0,
                             data:
                                 utf8_data{data: [0; 9],
                                           have: 0,
                                           size: 0,
                                           width: 0,},},
               references: 0,
               winlinks:
                   unnamed_11{tqh_first: 0 as *mut winlink,
                              tqh_last: 0 as *mut *mut winlink,},
               entry:
                   unnamed_5{rbe_left: 0 as *mut window,
                             rbe_right: 0 as *mut window,
                             rbe_parent: 0 as *mut window,
                             rbe_color: 0,},};
    w.id = id;
    return windows_RB_FIND(&mut windows_static as *mut windows,
                           &mut w as *mut window);
}
#[no_mangle]
pub unsafe extern "C" fn window_update_activity(mut w: *mut window) -> () {
    gettimeofday(&mut (*w).activity_time as *mut timeval, 0 as *mut timezone);
    alerts_queue(w, 2i32);
}
#[no_mangle]
pub unsafe extern "C" fn window_create(mut sx: u_int, mut sy: u_int)
 -> *mut window {
    let mut w: *mut window = 0 as *mut window;
    w =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<window>() as libc::c_ulong) as
            *mut window;
    (*w).name = 0 as *mut libc::c_char;
    (*w).flags = 32768i32;
    loop  {
        let ref mut fresh5 =
            (*(&mut (*w).panes as *mut window_panes)).tqh_first;
        *fresh5 = 0 as *mut window_pane;
        let ref mut fresh6 =
            (*(&mut (*w).panes as *mut window_panes)).tqh_last;
        *fresh6 =
            &mut (*(&mut (*w).panes as *mut window_panes)).tqh_first as
                *mut *mut window_pane;
        if !(0 != 0i32) { break ; }
    }
    (*w).active = 0 as *mut window_pane;
    (*w).lastlayout = 1i32.wrapping_neg();
    (*w).layout_root = 0 as *mut layout_cell;
    (*w).sx = sx;
    (*w).sy = sy;
    (*w).options = options_create(global_w_options);
    (*w).references = 0i32 as u_int;
    loop  {
        let ref mut fresh7 =
            (*(&mut (*w).winlinks as *mut unnamed_11)).tqh_first;
        *fresh7 = 0 as *mut winlink;
        let ref mut fresh8 =
            (*(&mut (*w).winlinks as *mut unnamed_11)).tqh_last;
        *fresh8 =
            &mut (*(&mut (*w).winlinks as *mut unnamed_11)).tqh_first as
                *mut *mut winlink;
        if !(0 != 0i32) { break ; }
    }
    let fresh9 = next_window_id;
    next_window_id = next_window_id.wrapping_add(1);
    (*w).id = fresh9;
    windows_RB_INSERT(&mut windows_static as *mut windows, w);
    window_update_activity(w);
    return w;
}
static mut next_window_id: u_int = unsafe { 0 };
#[no_mangle]
pub unsafe extern "C" fn window_create_spawn(mut name: *const libc::c_char,
                                             mut argc: libc::c_int,
                                             mut argv: *mut *mut libc::c_char,
                                             mut path: *const libc::c_char,
                                             mut shell: *const libc::c_char,
                                             mut cwd: *const libc::c_char,
                                             mut env: *mut environ,
                                             mut tio: *mut termios,
                                             mut sx: u_int, mut sy: u_int,
                                             mut hlimit: u_int,
                                             mut cause:
                                                 *mut *mut libc::c_char)
 -> *mut window {
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    w = window_create(sx, sy);
    wp = window_add_pane(w, 0 as *mut window_pane, 0i32, 0i32, hlimit);
    layout_init(w, wp);
    if window_pane_spawn(wp, argc, argv, path, shell, cwd, env, tio, cause) !=
           0i32 {
        window_destroy(w);
        return 0 as *mut window
    } else {
        (*w).active = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
        if name != 0 as *mut libc::c_void as *const libc::c_char {
            (*w).name = xstrdup(name);
            options_set_number((*w).options,
                               b"automatic-rename\x00" as *const u8 as
                                   *const libc::c_char,
                               0i32 as libc::c_longlong);
        } else { (*w).name = default_window_name(w) }
        notify_window(b"window-pane-changed\x00" as *const u8 as
                          *const libc::c_char, w);
        return w
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_spawn(mut wp: *mut window_pane,
                                           mut argc: libc::c_int,
                                           mut argv: *mut *mut libc::c_char,
                                           mut path: *const libc::c_char,
                                           mut shell: *const libc::c_char,
                                           mut cwd: *const libc::c_char,
                                           mut env: *mut environ,
                                           mut tio: *mut termios,
                                           mut cause: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut ws: winsize =
        winsize{ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0,};
    let mut argv0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argvp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut first: *const libc::c_char = 0 as *const libc::c_char;
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    let mut tio2: termios =
        termios{c_iflag: 0,
                c_oflag: 0,
                c_cflag: 0,
                c_lflag: 0,
                c_line: 0,
                c_cc: [0; 32],
                c_ispeed: 0,
                c_ospeed: 0,};
    let mut i: libc::c_int = 0;
    let mut set: sigset_t = sigset_t{__val: [0; 16],};
    let mut oldset: sigset_t = sigset_t{__val: [0; 16],};
    if (*wp).fd != 1i32.wrapping_neg() {
        bufferevent_free((*wp).event);
        close((*wp).fd);
    }
    if argc > 0i32 {
        cmd_free_argv((*wp).argc, (*wp).argv);
        (*wp).argc = argc;
        (*wp).argv = cmd_copy_argv(argc, argv)
    }
    if shell != 0 as *mut libc::c_void as *const libc::c_char {
        free((*wp).shell as *mut libc::c_void);
        (*wp).shell = xstrdup(shell)
    }
    if cwd != 0 as *mut libc::c_void as *const libc::c_char {
        free((*wp).cwd as *mut libc::c_void);
        (*wp).cwd = xstrdup(cwd)
    }
    (*wp).flags &= !(512i32 | 1024i32);
    cmd = cmd_stringify_argv((*wp).argc, (*wp).argv);
    log_debug(b"spawn: %s -- %s\x00" as *const u8 as *const libc::c_char,
              (*wp).shell, cmd);
    i = 0i32;
    while i < (*wp).argc {
        log_debug(b"spawn: argv[%d] = %s\x00" as *const u8 as
                      *const libc::c_char, i, *(*wp).argv.offset(i as isize));
        i += 1
    }
    environ_log(env, b"spawn: \x00" as *const u8 as *const libc::c_char);
    memset(&mut ws as *mut winsize as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<winsize>() as libc::c_ulong);
    ws.ws_col =
        (*(*(&mut (*wp).base as *mut screen)).grid).sx as libc::c_ushort;
    ws.ws_row =
        (*(*(&mut (*wp).base as *mut screen)).grid).sy as libc::c_ushort;
    sigfillset(&mut set as *mut sigset_t);
    sigprocmask(0i32, &mut set as *mut sigset_t,
                &mut oldset as *mut sigset_t);
    (*wp).pid =
        fdforkpty(ptm_fd, &mut (*wp).fd as *mut libc::c_int,
                  (*wp).tty.as_mut_ptr(), 0 as *mut termios,
                  &mut ws as *mut winsize);
    match (*wp).pid {
        -1 => {
            (*wp).fd = 1i32.wrapping_neg();
            xasprintf(cause,
                      b"%s: %s\x00" as *const u8 as *const libc::c_char, cmd,
                      strerror(*__errno_location()));
            free(cmd as *mut libc::c_void);
            sigprocmask(2i32, &mut oldset as *mut sigset_t,
                        0 as *mut sigset_t);
            return 1i32.wrapping_neg()
        }
        0 => {
            proc_clear_signals(server_proc, 1i32);
            sigprocmask(2i32, &mut oldset as *mut sigset_t,
                        0 as *mut sigset_t);
            cwd = 0 as *const libc::c_char;
            if chdir((*wp).cwd) == 0i32 {
                cwd = (*wp).cwd
            } else {
                home = find_home();
                if home != 0 as *mut libc::c_void as *const libc::c_char &&
                       chdir(home) == 0i32 {
                    cwd = home
                } else {
                    chdir(b"/\x00" as *const u8 as *const libc::c_char);
                }
            }
            if tcgetattr(0i32, &mut tio2 as *mut termios) != 0i32 {
                fatal(b"tcgetattr failed\x00" as *const u8 as
                          *const libc::c_char);
            } else {
                if tio != 0 as *mut libc::c_void as *mut termios {
                    memcpy(tio2.c_cc.as_mut_ptr() as *mut libc::c_void,
                           (*tio).c_cc.as_mut_ptr() as *const libc::c_void,
                           ::std::mem::size_of::<[cc_t; 32]>() as
                               libc::c_ulong);
                }
                tio2.c_cc[2usize] = 127 as cc_t;
                tio2.c_iflag |= 16384i32 as libc::c_uint;
                if tcsetattr(0i32, 0i32, &mut tio2 as *mut termios) != 0i32 {
                    fatal(b"tcgetattr failed\x00" as *const u8 as
                              *const libc::c_char);
                } else {
                    log_close();
                    closefrom(2i32 + 1i32);
                    if path != 0 as *mut libc::c_void as *const libc::c_char {
                        environ_set(env,
                                    b"PATH\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"%s\x00" as *const u8 as
                                        *const libc::c_char, path);
                    }
                    if cwd != 0 as *mut libc::c_void as *const libc::c_char {
                        environ_set(env,
                                    b"PWD\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"%s\x00" as *const u8 as
                                        *const libc::c_char, cwd);
                    }
                    environ_set(env,
                                b"TMUX_PANE\x00" as *const u8 as
                                    *const libc::c_char,
                                b"%%%u\x00" as *const u8 as
                                    *const libc::c_char, (*wp).id);
                    environ_push(env);
                    setenv(b"SHELL\x00" as *const u8 as *const libc::c_char,
                           (*wp).shell, 1i32);
                    ptr = strrchr((*wp).shell, 47);
                    if (*wp).argc > 0i32 {
                        if (*wp).argc != 1i32 {
                            argvp = cmd_copy_argv((*wp).argc, (*wp).argv);
                            execvp(*argvp.offset(0isize),
                                   argvp as *const *mut libc::c_char);
                            fatal(b"execvp failed\x00" as *const u8 as
                                      *const libc::c_char);
                        } else {
                            first = *(*wp).argv.offset(0isize);
                            if ptr !=
                                   0 as *mut libc::c_void as
                                       *const libc::c_char &&
                                   *ptr.offset(1isize) as libc::c_int != 0 {
                                xasprintf(&mut argv0 as
                                              *mut *mut libc::c_char,
                                          b"%s\x00" as *const u8 as
                                              *const libc::c_char,
                                          ptr.offset(1isize));
                            } else {
                                xasprintf(&mut argv0 as
                                              *mut *mut libc::c_char,
                                          b"%s\x00" as *const u8 as
                                              *const libc::c_char,
                                          (*wp).shell);
                            }
                            execl((*wp).shell, argv0,
                                  b"-c\x00" as *const u8 as
                                      *const libc::c_char, first,
                                  0 as *mut libc::c_void as
                                      *mut libc::c_char);
                            fatal(b"execl failed\x00" as *const u8 as
                                      *const libc::c_char);
                        }
                    } else {
                        if ptr !=
                               0 as *mut libc::c_void as *const libc::c_char
                               && *ptr.offset(1isize) as libc::c_int != 0 {
                            xasprintf(&mut argv0 as *mut *mut libc::c_char,
                                      b"-%s\x00" as *const u8 as
                                          *const libc::c_char,
                                      ptr.offset(1isize));
                        } else {
                            xasprintf(&mut argv0 as *mut *mut libc::c_char,
                                      b"-%s\x00" as *const u8 as
                                          *const libc::c_char, (*wp).shell);
                        }
                        execl((*wp).shell, argv0,
                              0 as *mut libc::c_void as *mut libc::c_char);
                        fatal(b"execl failed\x00" as *const u8 as
                                  *const libc::c_char);
                    }
                }
            }
        }
        _ => {
            sigprocmask(2i32, &mut oldset as *mut sigset_t,
                        0 as *mut sigset_t);
            setblocking((*wp).fd, 0i32);
            (*wp).event =
                bufferevent_new((*wp).fd, Some(window_pane_read_callback),
                                None, Some(window_pane_error_callback),
                                wp as *mut libc::c_void);
            bufferevent_setwatermark((*wp).event, 2i32 as libc::c_short,
                                     0i32 as size_t, 4096i32 as size_t);
            bufferevent_enable((*wp).event, (2i32 | 4i32) as libc::c_short);
            free(cmd as *mut libc::c_void);
            return 0i32
        }
    };
}
unsafe extern "C" fn window_pane_error_callback(mut bufev: *mut bufferevent,
                                                mut what: libc::c_short,
                                                mut data: *mut libc::c_void)
 -> () {
    let mut wp: *mut window_pane = data as *mut window_pane;
    log_debug(b"%%%u error\x00" as *const u8 as *const libc::c_char,
              (*wp).id);
    (*wp).flags |= 256i32;
    if 0 != window_pane_destroy_ready(wp) { server_destroy_pane(wp, 1i32); };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_destroy_ready(mut wp: *mut window_pane)
 -> libc::c_int {
    let mut n: libc::c_int = 0;
    if (*wp).pipe_fd != 1i32.wrapping_neg() {
        if evbuffer_get_length((*(*wp).pipe_event).output) !=
               0i32 as libc::c_ulong {
            return 0i32
        } else if ioctl((*wp).fd, 21531i32 as libc::c_ulong,
                        &mut n as *mut libc::c_int) != 1i32.wrapping_neg() &&
                      n > 0i32 {
            return 0i32
        }
    }
    if 0 != !(*wp).flags & 256i32 { return 0i32 } else { return 1i32 };
}
unsafe extern "C" fn window_pane_read_callback(mut bufev: *mut bufferevent,
                                               mut data: *mut libc::c_void)
 -> () {
    let mut wp: *mut window_pane = data as *mut window_pane;
    let mut evb: *mut evbuffer = (*(*wp).event).input;
    let mut size: size_t = evbuffer_get_length(evb);
    let mut new_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_size: size_t = 0;
    new_size = size.wrapping_sub((*wp).pipe_off);
    if (*wp).pipe_fd != 1i32.wrapping_neg() &&
           new_size > 0i32 as libc::c_ulong {
        new_data =
            evbuffer_pullup(evb,
                            1i32.wrapping_neg() as
                                ssize_t).offset((*wp).pipe_off as isize) as
                *mut libc::c_char;
        bufferevent_write((*wp).pipe_event, new_data as *const libc::c_void,
                          new_size);
    }
    log_debug(b"%%%u has %zu bytes\x00" as *const u8 as *const libc::c_char,
              (*wp).id, size);
    input_parse(wp);
    (*wp).pipe_off = evbuffer_get_length(evb);
}
#[no_mangle]
pub unsafe extern "C" fn window_add_pane(mut w: *mut window,
                                         mut other: *mut window_pane,
                                         mut before: libc::c_int,
                                         mut full_size: libc::c_int,
                                         mut hlimit: u_int)
 -> *mut window_pane {
    let mut current_block: u64;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if other == 0 as *mut libc::c_void as *mut window_pane {
        other = (*w).active
    }
    wp = window_pane_create(w, (*w).sx, (*w).sy, hlimit);
    if (*(&mut (*w).panes as *mut window_panes)).tqh_first ==
           0 as *mut libc::c_void as *mut window_pane {
        log_debug(b"%s: @%u at start\x00" as *const u8 as *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"window_add_pane\x00")).as_ptr(),
                  (*w).id);
        loop  {
            (*wp).entry.tqe_next =
                (*(&mut (*w).panes as *mut window_panes)).tqh_first;
            if (*wp).entry.tqe_next !=
                   0 as *mut libc::c_void as *mut window_pane {
                let ref mut fresh10 =
                    (*(*(&mut (*w).panes as
                             *mut window_panes)).tqh_first).entry.tqe_prev;
                *fresh10 = &mut (*wp).entry.tqe_next as *mut *mut window_pane
            } else {
                let ref mut fresh11 =
                    (*(&mut (*w).panes as *mut window_panes)).tqh_last;
                *fresh11 = &mut (*wp).entry.tqe_next as *mut *mut window_pane
            }
            let ref mut fresh12 =
                (*(&mut (*w).panes as *mut window_panes)).tqh_first;
            *fresh12 = wp;
            (*wp).entry.tqe_prev =
                &mut (*(&mut (*w).panes as *mut window_panes)).tqh_first as
                    *mut *mut window_pane;
            if !(0 != 0i32) { break ; }
        }
    } else if 0 != before {
        log_debug(b"%s: @%u before %%%u\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"window_add_pane\x00")).as_ptr(),
                  (*w).id, (*wp).id);
        if 0 != full_size {
            current_block = 7351195479953500246;
        } else { current_block = 1394248824506584008; }
        loop  {
            match current_block {
                7351195479953500246 => {
                    (*wp).entry.tqe_next =
                        (*(&mut (*w).panes as *mut window_panes)).tqh_first;
                    if (*wp).entry.tqe_next !=
                           0 as *mut libc::c_void as *mut window_pane {
                        let ref mut fresh13 =
                            (*(*(&mut (*w).panes as
                                     *mut window_panes)).tqh_first).entry.tqe_prev;
                        *fresh13 =
                            &mut (*wp).entry.tqe_next as *mut *mut window_pane
                    } else {
                        let ref mut fresh14 =
                            (*(&mut (*w).panes as
                                   *mut window_panes)).tqh_last;
                        *fresh14 =
                            &mut (*wp).entry.tqe_next as *mut *mut window_pane
                    }
                    let ref mut fresh15 =
                        (*(&mut (*w).panes as *mut window_panes)).tqh_first;
                    *fresh15 = wp;
                    (*wp).entry.tqe_prev =
                        &mut (*(&mut (*w).panes as
                                    *mut window_panes)).tqh_first as
                            *mut *mut window_pane;
                    if 0 != 0i32 {
                        current_block = 7351195479953500246;
                    } else { break ; }
                }
                _ => {
                    (*wp).entry.tqe_prev = (*other).entry.tqe_prev;
                    (*wp).entry.tqe_next = other;
                    *(*other).entry.tqe_prev = wp;
                    (*other).entry.tqe_prev =
                        &mut (*wp).entry.tqe_next as *mut *mut window_pane;
                    if 0 != 0i32 {
                        current_block = 1394248824506584008;
                    } else { break ; }
                }
            }
        }
    } else {
        log_debug(b"%s: @%u after %%%u\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"window_add_pane\x00")).as_ptr(),
                  (*w).id, (*wp).id);
        if 0 != full_size {
            current_block = 2979737022853876585;
        } else { current_block = 10048703153582371463; }
        loop  {
            match current_block {
                10048703153582371463 => {
                    (*wp).entry.tqe_next = (*other).entry.tqe_next;
                    if (*wp).entry.tqe_next !=
                           0 as *mut libc::c_void as *mut window_pane {
                        (*(*wp).entry.tqe_next).entry.tqe_prev =
                            &mut (*wp).entry.tqe_next as *mut *mut window_pane
                    } else {
                        let ref mut fresh18 =
                            (*(&mut (*w).panes as
                                   *mut window_panes)).tqh_last;
                        *fresh18 =
                            &mut (*wp).entry.tqe_next as *mut *mut window_pane
                    }
                    (*other).entry.tqe_next = wp;
                    (*wp).entry.tqe_prev =
                        &mut (*other).entry.tqe_next as *mut *mut window_pane;
                    if 0 != 0i32 {
                        current_block = 10048703153582371463;
                    } else { break ; }
                }
                _ => {
                    (*wp).entry.tqe_next = 0 as *mut window_pane;
                    (*wp).entry.tqe_prev =
                        (*(&mut (*w).panes as *mut window_panes)).tqh_last;
                    let ref mut fresh16 =
                        *(*(&mut (*w).panes as *mut window_panes)).tqh_last;
                    *fresh16 = wp;
                    let ref mut fresh17 =
                        (*(&mut (*w).panes as *mut window_panes)).tqh_last;
                    *fresh17 =
                        &mut (*wp).entry.tqe_next as *mut *mut window_pane;
                    if 0 != 0i32 {
                        current_block = 2979737022853876585;
                    } else { break ; }
                }
            }
        }
    }
    return wp;
}
unsafe extern "C" fn window_pane_create(mut w: *mut window, mut sx: u_int,
                                        mut sy: u_int, mut hlimit: u_int)
 -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut host: [libc::c_char; 65] = [0; 65];
    wp =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<window_pane>() as libc::c_ulong) as
            *mut window_pane;
    (*wp).window = w;
    let fresh19 = next_window_pane_id;
    next_window_pane_id = next_window_pane_id.wrapping_add(1);
    (*wp).id = fresh19;
    window_pane_tree_RB_INSERT(&mut all_window_panes as *mut window_pane_tree,
                               wp);
    (*wp).argc = 0i32;
    (*wp).argv = 0 as *mut *mut libc::c_char;
    (*wp).shell = 0 as *mut libc::c_char;
    (*wp).cwd = 0 as *const libc::c_char;
    (*wp).fd = 1i32.wrapping_neg();
    (*wp).event = 0 as *mut bufferevent;
    (*wp).mode = 0 as *const window_mode;
    (*wp).modeprefix = 1i32 as u_int;
    (*wp).layout_cell = 0 as *mut layout_cell;
    (*wp).xoff = 0i32 as u_int;
    (*wp).yoff = 0i32 as u_int;
    (*wp).osx = sx;
    (*wp).sx = (*wp).osx;
    (*wp).osx = sy;
    (*wp).sy = (*wp).osx;
    (*wp).pipe_fd = 1i32.wrapping_neg();
    (*wp).pipe_off = 0i32 as size_t;
    (*wp).pipe_event = 0 as *mut bufferevent;
    (*wp).saved_grid = 0 as *mut grid;
    memcpy(&mut (*wp).colgc as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    screen_init(&mut (*wp).base as *mut screen, sx, sy, hlimit);
    (*wp).screen = &mut (*wp).base as *mut screen;
    screen_init(&mut (*wp).status_screen as *mut screen, 1i32 as u_int,
                1i32 as u_int, 0i32 as u_int);
    if gethostname(host.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 65]>() as
                       libc::c_ulong) == 0i32 {
        screen_set_title(&mut (*wp).base as *mut screen, host.as_mut_ptr());
    }
    input_init(wp);
    return wp;
}
static mut next_window_pane_id: u_int = unsafe { 0 };
#[no_mangle]
pub unsafe extern "C" fn window_get_active_at(mut w: *mut window,
                                              mut x: u_int, mut y: u_int)
 -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
    loop  {
        if wp != 0 as *mut libc::c_void as *mut window_pane {
            if !(0 == window_pane_visible(wp)) {
                if !(x < (*wp).xoff || x > (*wp).xoff.wrapping_add((*wp).sx))
                   {
                    if !(y < (*wp).yoff ||
                             y > (*wp).yoff.wrapping_add((*wp).sy)) {
                        return wp
                    }
                }
            }
            wp = (*wp).entry.tqe_next
        } else { return 0 as *mut window_pane }
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_visible(mut wp: *mut window_pane)
 -> libc::c_int {
    let mut w: *mut window = (*wp).window;
    if (*wp).layout_cell == 0 as *mut libc::c_void as *mut layout_cell {
        return 0i32
    } else if (*wp).xoff >= (*w).sx || (*wp).yoff >= (*w).sy {
        return 0i32
    } else if (*wp).xoff.wrapping_add((*wp).sx) > (*w).sx ||
                  (*wp).yoff.wrapping_add((*wp).sy) > (*w).sy {
        return 0i32
    } else { return 1i32 };
}
#[no_mangle]
pub unsafe extern "C" fn window_find_string(mut w: *mut window,
                                            mut s: *const libc::c_char)
 -> *mut window_pane {
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    x = (*w).sx.wrapping_div(2i32 as libc::c_uint);
    y = (*w).sy.wrapping_div(2i32 as libc::c_uint);
    if strcasecmp(s, b"top\x00" as *const u8 as *const libc::c_char) == 0i32 {
        y = 0i32 as u_int
    } else if strcasecmp(s, b"bottom\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        y = (*w).sy.wrapping_sub(1i32 as libc::c_uint)
    } else if strcasecmp(s, b"left\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        x = 0i32 as u_int
    } else if strcasecmp(s, b"right\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        x = (*w).sx.wrapping_sub(1i32 as libc::c_uint)
    } else if strcasecmp(s,
                         b"top-left\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        x = 0i32 as u_int;
        y = 0i32 as u_int
    } else if strcasecmp(s,
                         b"top-right\x00" as *const u8 as *const libc::c_char)
                  == 0i32 {
        x = (*w).sx.wrapping_sub(1i32 as libc::c_uint);
        y = 0i32 as u_int
    } else if strcasecmp(s,
                         b"bottom-left\x00" as *const u8 as
                             *const libc::c_char) == 0i32 {
        x = 0i32 as u_int;
        y = (*w).sy.wrapping_sub(1i32 as libc::c_uint)
    } else if strcasecmp(s,
                         b"bottom-right\x00" as *const u8 as
                             *const libc::c_char) == 0i32 {
        x = (*w).sx.wrapping_sub(1i32 as libc::c_uint);
        y = (*w).sy.wrapping_sub(1i32 as libc::c_uint)
    } else { return 0 as *mut window_pane }
    return window_get_active_at(w, x, y);
}
#[no_mangle]
pub unsafe extern "C" fn window_has_pane(mut w: *mut window,
                                         mut wp: *mut window_pane)
 -> libc::c_int {
    let mut wp1: *mut window_pane = 0 as *mut window_pane;
    wp1 = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
    loop  {
        if wp1 != 0 as *mut libc::c_void as *mut window_pane {
            if wp1 == wp { return 1i32 } else { wp1 = (*wp1).entry.tqe_next }
        } else { return 0i32 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_set_active_pane(mut w: *mut window,
                                                mut wp: *mut window_pane)
 -> libc::c_int {
    log_debug(b"%s: pane %%%u (was %%%u)\x00" as *const u8 as
                  *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"window_set_active_pane\x00")).as_ptr(),
              (*wp).id, (*(*w).active).id);
    if wp == (*w).active {
        return 0i32
    } else {
        (*w).last = (*w).active;
        (*w).active = wp;
        loop  {
            if 0 == window_pane_visible((*w).active) {
                (*w).active =
                    *(*((*(*w).active).entry.tqe_prev as
                            *mut window_panes)).tqh_last;
                if (*w).active == 0 as *mut libc::c_void as *mut window_pane {
                    (*w).active =
                        *(*((*(&mut (*w).panes as *mut window_panes)).tqh_last
                                as *mut window_panes)).tqh_last
                }
                if !((*w).active == wp) { continue ; }
                notify_window(b"window-pane-changed\x00" as *const u8 as
                                  *const libc::c_char, w);
                return 1i32
            } else {
                let fresh20 = next_active_point;
                next_active_point = next_active_point.wrapping_add(1);
                (*(*w).active).active_point = fresh20;
                (*(*w).active).flags |= 128i32;
                notify_window(b"window-pane-changed\x00" as *const u8 as
                                  *const libc::c_char, w);
                return 1i32
            }
        }
    };
}
static mut next_active_point: u_int = unsafe { 0 };
#[no_mangle]
pub unsafe extern "C" fn window_redraw_active_switch(mut w: *mut window,
                                                     mut wp: *mut window_pane)
 -> () {
    let mut gc: *const grid_cell = 0 as *const grid_cell;
    if wp == (*w).active {
        return
    } else {
        gc =
            options_get_style((*w).options,
                              b"window-active-style\x00" as *const u8 as
                                  *const libc::c_char);
        if 0 !=
               style_equal(gc,
                           options_get_style((*w).options,
                                             b"window-style\x00" as *const u8
                                                 as *const libc::c_char)) {
            return
        } else {
            if window_pane_get_palette((*w).active, (*(*w).active).colgc.fg)
                   != 1i32.wrapping_neg() ||
                   window_pane_get_palette((*w).active,
                                           (*(*w).active).colgc.bg) !=
                       1i32.wrapping_neg() ||
                   0 !=
                       style_equal(&grid_default_cell as *const grid_cell,
                                   &mut (*(*w).active).colgc as
                                       *mut grid_cell) {
                (*(*w).active).flags |= 1i32
            }
            if window_pane_get_palette(wp, (*wp).colgc.fg) !=
                   1i32.wrapping_neg() ||
                   window_pane_get_palette(wp, (*wp).colgc.bg) !=
                       1i32.wrapping_neg() ||
                   0 !=
                       style_equal(&grid_default_cell as *const grid_cell,
                                   &mut (*wp).colgc as *mut grid_cell) {
                (*wp).flags |= 1i32
            }
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_get_palette(mut wp: *const window_pane,
                                                 mut c: libc::c_int)
 -> libc::c_int {
    let mut new: libc::c_int = 0;
    if wp == 0 as *mut libc::c_void as *const window_pane ||
           (*wp).palette == 0 as *mut libc::c_void as *mut libc::c_int {
        return 1i32.wrapping_neg()
    } else {
        new = 1i32.wrapping_neg();
        if c < 8i32 {
            new = *(*wp).palette.offset(c as isize)
        } else if c >= 90i32 && c <= 97i32 {
            new = *(*wp).palette.offset((8i32 + c - 90i32) as isize)
        } else if 0 != c & 16777216i32 {
            new = *(*wp).palette.offset((c & !16777216i32) as isize)
        }
        if new == 0i32 { return 1i32.wrapping_neg() } else { return new }
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_resize(mut w: *mut window, mut sx: u_int,
                                       mut sy: u_int) -> () {
    (*w).sx = sx;
    (*w).sy = sy;
}
#[no_mangle]
pub unsafe extern "C" fn window_zoom(mut wp: *mut window_pane)
 -> libc::c_int {
    let mut w: *mut window = (*wp).window;
    let mut wp1: *mut window_pane = 0 as *mut window_pane;
    if 0 != (*w).flags & 4096i32 {
        return 1i32.wrapping_neg()
    } else if 0 == window_pane_visible(wp) {
        return 1i32.wrapping_neg()
    } else if window_count_panes(w) == 1i32 as libc::c_uint {
        return 1i32.wrapping_neg()
    } else {
        if (*w).active != wp { window_set_active_pane(w, wp); }
        wp1 = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
        while wp1 != 0 as *mut libc::c_void as *mut window_pane {
            (*wp1).saved_layout_cell = (*wp1).layout_cell;
            (*wp1).layout_cell = 0 as *mut layout_cell;
            wp1 = (*wp1).entry.tqe_next
        }
        (*w).saved_layout_root = (*w).layout_root;
        layout_init(w, wp);
        (*w).flags |= 4096i32;
        notify_window(b"window-layout-changed\x00" as *const u8 as
                          *const libc::c_char, w);
        return 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_count_panes(mut w: *mut window) -> u_int {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut n: u_int = 0;
    n = 0i32 as u_int;
    wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
    while wp != 0 as *mut libc::c_void as *mut window_pane {
        n = n.wrapping_add(1);
        wp = (*wp).entry.tqe_next
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn window_unzoom(mut w: *mut window) -> libc::c_int {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if 0 == (*w).flags & 4096i32 {
        return 1i32.wrapping_neg()
    } else {
        (*w).flags &= !4096i32;
        layout_free(w);
        (*w).layout_root = (*w).saved_layout_root;
        (*w).saved_layout_root = 0 as *mut layout_cell;
        wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
        while wp != 0 as *mut libc::c_void as *mut window_pane {
            (*wp).layout_cell = (*wp).saved_layout_cell;
            (*wp).saved_layout_cell = 0 as *mut layout_cell;
            wp = (*wp).entry.tqe_next
        }
        layout_fix_panes(w, (*w).sx, (*w).sy);
        notify_window(b"window-layout-changed\x00" as *const u8 as
                          *const libc::c_char, w);
        return 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_lost_pane(mut w: *mut window,
                                          mut wp: *mut window_pane) -> () {
    log_debug(b"%s: @%u pane %%%u\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"window_lost_pane\x00")).as_ptr(),
              (*w).id, (*wp).id);
    if wp == marked_pane.wp { server_clear_marked(); }
    if wp == (*w).active {
        (*w).active = (*w).last;
        (*w).last = 0 as *mut window_pane;
        if (*w).active == 0 as *mut libc::c_void as *mut window_pane {
            (*w).active =
                *(*((*wp).entry.tqe_prev as *mut window_panes)).tqh_last;
            if (*w).active == 0 as *mut libc::c_void as *mut window_pane {
                (*w).active = (*wp).entry.tqe_next
            }
        }
        if (*w).active != 0 as *mut libc::c_void as *mut window_pane {
            (*(*w).active).flags |= 128i32;
            notify_window(b"window-pane-changed\x00" as *const u8 as
                              *const libc::c_char, w);
        }
    } else if wp == (*w).last { (*w).last = 0 as *mut window_pane };
}
#[no_mangle]
pub unsafe extern "C" fn window_remove_pane(mut w: *mut window,
                                            mut wp: *mut window_pane) -> () {
    window_lost_pane(w, wp);
    loop  {
        if (*wp).entry.tqe_next != 0 as *mut libc::c_void as *mut window_pane
           {
            (*(*wp).entry.tqe_next).entry.tqe_prev = (*wp).entry.tqe_prev
        } else {
            let ref mut fresh21 =
                (*(&mut (*w).panes as *mut window_panes)).tqh_last;
            *fresh21 = (*wp).entry.tqe_prev
        }
        *(*wp).entry.tqe_prev = (*wp).entry.tqe_next;
        if !(0 != 0i32) { break ; }
    }
    window_pane_destroy(wp);
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_at_index(mut w: *mut window,
                                              mut idx: u_int)
 -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut n: u_int = 0;
    n =
        options_get_number((*w).options,
                           b"pane-base-index\x00" as *const u8 as
                               *const libc::c_char) as u_int;
    wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
    loop  {
        if wp != 0 as *mut libc::c_void as *mut window_pane {
            if n == idx {
                return wp
            } else { n = n.wrapping_add(1); wp = (*wp).entry.tqe_next }
        } else { return 0 as *mut window_pane }
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_next_by_number(mut w: *mut window,
                                                    mut wp: *mut window_pane,
                                                    mut n: u_int)
 -> *mut window_pane {
    while n > 0i32 as libc::c_uint {
        wp = (*wp).entry.tqe_next;
        if wp == 0 as *mut libc::c_void as *mut window_pane {
            wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first
        }
        n = n.wrapping_sub(1)
    }
    return wp;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_previous_by_number(mut w: *mut window,
                                                        mut wp:
                                                            *mut window_pane,
                                                        mut n: u_int)
 -> *mut window_pane {
    while n > 0i32 as libc::c_uint {
        wp = *(*((*wp).entry.tqe_prev as *mut window_panes)).tqh_last;
        if wp == 0 as *mut libc::c_void as *mut window_pane {
            wp =
                *(*((*(&mut (*w).panes as *mut window_panes)).tqh_last as
                        *mut window_panes)).tqh_last
        }
        n = n.wrapping_sub(1)
    }
    return wp;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_index(mut wp: *mut window_pane,
                                           mut i: *mut u_int) -> libc::c_int {
    let mut wq: *mut window_pane = 0 as *mut window_pane;
    let mut w: *mut window = (*wp).window;
    *i =
        options_get_number((*w).options,
                           b"pane-base-index\x00" as *const u8 as
                               *const libc::c_char) as u_int;
    wq = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
    loop  {
        if wq != 0 as *mut libc::c_void as *mut window_pane {
            if wp == wq {
                return 0i32
            } else { *i = (*i).wrapping_add(1); wq = (*wq).entry.tqe_next }
        } else { return 1i32.wrapping_neg() }
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_by_id_str(mut s:
                                                        *const libc::c_char)
 -> *mut window_pane {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut id: u_int = 0;
    if *s as libc::c_int != 37 {
        return 0 as *mut window_pane
    } else {
        id =
            strtonum(s.offset(1isize), 0i32 as libc::c_longlong,
                     (2147483647i32 as
                          libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                         as libc::c_longlong,
                     &mut errstr as *mut *const libc::c_char) as u_int;
        if errstr != 0 as *mut libc::c_void as *const libc::c_char {
            return 0 as *mut window_pane
        } else { return window_pane_find_by_id(id) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_by_id(mut id: u_int)
 -> *mut window_pane {
    let mut wp: window_pane =
        window_pane{id: 0,
                    active_point: 0,
                    window: 0 as *mut window,
                    layout_cell: 0 as *mut layout_cell,
                    saved_layout_cell: 0 as *mut layout_cell,
                    sx: 0,
                    sy: 0,
                    osx: 0,
                    osy: 0,
                    xoff: 0,
                    yoff: 0,
                    flags: 0,
                    argc: 0,
                    argv: 0 as *mut *mut libc::c_char,
                    shell: 0 as *mut libc::c_char,
                    cwd: 0 as *const libc::c_char,
                    pid: 0,
                    tty: [0; 32],
                    status: 0,
                    fd: 0,
                    event: 0 as *mut bufferevent,
                    resize_timer:
                        event{ev_active_next:
                                  unnamed{tqe_next: 0 as *mut event,
                                          tqe_prev: 0 as *mut *mut event,},
                              ev_next:
                                  unnamed_39{tqe_next: 0 as *mut event,
                                             tqe_prev: 0 as *mut *mut event,},
                              ev_timeout_pos:
                                  unnamed_0{ev_next_with_common_timeout:
                                                unnamed_38{tqe_next:
                                                               0 as
                                                                   *mut event,
                                                           tqe_prev:
                                                               0 as
                                                                   *mut *mut event,},},
                              ev_fd: 0,
                              ev_base: 0 as *mut event_base,
                              _ev:
                                  unnamed_27{ev_io:
                                                 unnamed_13{ev_io_next:
                                                                unnamed_20{tqe_next:
                                                                               0
                                                                                   as
                                                                                   *mut event,
                                                                           tqe_prev:
                                                                               0
                                                                                   as
                                                                                   *mut *mut event,},
                                                            ev_timeout:
                                                                timeval{tv_sec:
                                                                            0,
                                                                        tv_usec:
                                                                            0,},},},
                              ev_events: 0,
                              ev_res: 0,
                              ev_flags: 0,
                              ev_pri: 0,
                              ev_closure: 0,
                              ev_timeout: timeval{tv_sec: 0, tv_usec: 0,},
                              ev_callback: None,
                              ev_arg: 0 as *mut libc::c_void,},
                    ictx: 0 as *mut input_ctx,
                    colgc:
                        grid_cell{flags: 0,
                                  attr: 0,
                                  fg: 0,
                                  bg: 0,
                                  data:
                                      utf8_data{data: [0; 9],
                                                have: 0,
                                                size: 0,
                                                width: 0,},},
                    palette: 0 as *mut libc::c_int,
                    pipe_fd: 0,
                    pipe_event: 0 as *mut bufferevent,
                    pipe_off: 0,
                    screen: 0 as *mut screen,
                    base:
                        screen{title: 0 as *mut libc::c_char,
                               titles: 0 as *mut screen_titles,
                               grid: 0 as *mut grid,
                               cx: 0,
                               cy: 0,
                               cstyle: 0,
                               ccolour: 0 as *mut libc::c_char,
                               rupper: 0,
                               rlower: 0,
                               mode: 0,
                               tabs: 0 as *mut bitstr_t,
                               sel:
                                   screen_sel{flag: 0,
                                              hidden: 0,
                                              rectflag: 0,
                                              lineflag: LINE_SEL_NONE,
                                              modekeys: 0,
                                              sx: 0,
                                              sy: 0,
                                              ex: 0,
                                              ey: 0,
                                              cell:
                                                  grid_cell{flags: 0,
                                                            attr: 0,
                                                            fg: 0,
                                                            bg: 0,
                                                            data:
                                                                utf8_data{data:
                                                                              [0;
                                                                                  9],
                                                                          have:
                                                                              0,
                                                                          size:
                                                                              0,
                                                                          width:
                                                                              0,},},},},
                    status_screen:
                        screen{title: 0 as *mut libc::c_char,
                               titles: 0 as *mut screen_titles,
                               grid: 0 as *mut grid,
                               cx: 0,
                               cy: 0,
                               cstyle: 0,
                               ccolour: 0 as *mut libc::c_char,
                               rupper: 0,
                               rlower: 0,
                               mode: 0,
                               tabs: 0 as *mut bitstr_t,
                               sel:
                                   screen_sel{flag: 0,
                                              hidden: 0,
                                              rectflag: 0,
                                              lineflag: LINE_SEL_NONE,
                                              modekeys: 0,
                                              sx: 0,
                                              sy: 0,
                                              ex: 0,
                                              ey: 0,
                                              cell:
                                                  grid_cell{flags: 0,
                                                            attr: 0,
                                                            fg: 0,
                                                            bg: 0,
                                                            data:
                                                                utf8_data{data:
                                                                              [0;
                                                                                  9],
                                                                          have:
                                                                              0,
                                                                          size:
                                                                              0,
                                                                          width:
                                                                              0,},},},},
                    status_size: 0,
                    saved_cx: 0,
                    saved_cy: 0,
                    saved_grid: 0 as *mut grid,
                    saved_cell:
                        grid_cell{flags: 0,
                                  attr: 0,
                                  fg: 0,
                                  bg: 0,
                                  data:
                                      utf8_data{data: [0; 9],
                                                have: 0,
                                                size: 0,
                                                width: 0,},},
                    mode: 0 as *const window_mode,
                    modedata: 0 as *mut libc::c_void,
                    modetimer:
                        event{ev_active_next:
                                  unnamed{tqe_next: 0 as *mut event,
                                          tqe_prev: 0 as *mut *mut event,},
                              ev_next:
                                  unnamed_39{tqe_next: 0 as *mut event,
                                             tqe_prev: 0 as *mut *mut event,},
                              ev_timeout_pos:
                                  unnamed_0{ev_next_with_common_timeout:
                                                unnamed_38{tqe_next:
                                                               0 as
                                                                   *mut event,
                                                           tqe_prev:
                                                               0 as
                                                                   *mut *mut event,},},
                              ev_fd: 0,
                              ev_base: 0 as *mut event_base,
                              _ev:
                                  unnamed_27{ev_io:
                                                 unnamed_13{ev_io_next:
                                                                unnamed_20{tqe_next:
                                                                               0
                                                                                   as
                                                                                   *mut event,
                                                                           tqe_prev:
                                                                               0
                                                                                   as
                                                                                   *mut *mut event,},
                                                            ev_timeout:
                                                                timeval{tv_sec:
                                                                            0,
                                                                        tv_usec:
                                                                            0,},},},
                              ev_events: 0,
                              ev_res: 0,
                              ev_flags: 0,
                              ev_pri: 0,
                              ev_closure: 0,
                              ev_timeout: timeval{tv_sec: 0, tv_usec: 0,},
                              ev_callback: None,
                              ev_arg: 0 as *mut libc::c_void,},
                    modelast: 0,
                    modeprefix: 0,
                    searchstr: 0 as *mut libc::c_char,
                    entry:
                        unnamed_12{tqe_next: 0 as *mut window_pane,
                                   tqe_prev: 0 as *mut *mut window_pane,},
                    tree_entry:
                        unnamed_24{rbe_left: 0 as *mut window_pane,
                                   rbe_right: 0 as *mut window_pane,
                                   rbe_parent: 0 as *mut window_pane,
                                   rbe_color: 0,},};
    wp.id = id;
    return window_pane_tree_RB_FIND(&mut all_window_panes as
                                        *mut window_pane_tree,
                                    &mut wp as *mut window_pane);
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_resize(mut wp: *mut window_pane,
                                            mut sx: u_int, mut sy: u_int)
 -> () {
    if sx == (*wp).sx && sy == (*wp).sy {
        return
    } else {
        (*wp).sx = sx;
        (*wp).sy = sy;
        screen_resize(&mut (*wp).base as *mut screen, sx, sy,
                      ((*wp).saved_grid ==
                           0 as *mut libc::c_void as *mut grid) as
                          libc::c_int);
        if (*wp).mode != 0 as *mut libc::c_void as *const window_mode {
            (*(*wp).mode).resize.expect("non-null function pointer")(wp, sx,
                                                                     sy);
        }
        (*wp).flags |= 8i32;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_alternate_on(mut wp: *mut window_pane,
                                                  mut gc: *mut grid_cell,
                                                  mut cursor: libc::c_int)
 -> () {
    let mut s: *mut screen = &mut (*wp).base as *mut screen;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    if (*wp).saved_grid != 0 as *mut libc::c_void as *mut grid {
        return
    } else if 0 ==
                  options_get_number((*(*wp).window).options,
                                     b"alternate-screen\x00" as *const u8 as
                                         *const libc::c_char) {
        return
    } else {
        sx = (*(*s).grid).sx;
        sy = (*(*s).grid).sy;
        (*wp).saved_grid = grid_create(sx, sy, 0i32 as u_int);
        grid_duplicate_lines((*wp).saved_grid, 0i32 as u_int, (*s).grid,
                             (*(*s).grid).hsize, sy);
        if 0 != cursor { (*wp).saved_cx = (*s).cx; (*wp).saved_cy = (*s).cy }
        memcpy(&mut (*wp).saved_cell as *mut grid_cell as *mut libc::c_void,
               gc as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        grid_view_clear((*s).grid, 0i32 as u_int, 0i32 as u_int, sx, sy,
                        8i32 as u_int);
        (*(*wp).base.grid).flags &= !1i32;
        (*wp).flags |= 1i32;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_alternate_off(mut wp: *mut window_pane,
                                                   mut gc: *mut grid_cell,
                                                   mut cursor: libc::c_int)
 -> () {
    let mut s: *mut screen = &mut (*wp).base as *mut screen;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    if (*wp).saved_grid == 0 as *mut libc::c_void as *mut grid {
        return
    } else if 0 ==
                  options_get_number((*(*wp).window).options,
                                     b"alternate-screen\x00" as *const u8 as
                                         *const libc::c_char) {
        return
    } else {
        sx = (*(*s).grid).sx;
        sy = (*(*s).grid).sy;
        if sy > (*(*wp).saved_grid).sy {
            screen_resize(s, sx, (*(*wp).saved_grid).sy, 1i32);
        }
        grid_duplicate_lines((*s).grid, (*(*s).grid).hsize, (*wp).saved_grid,
                             0i32 as u_int, sy);
        if 0 != cursor { (*s).cx = (*wp).saved_cx }
        if (*s).cx > (*(*s).grid).sx.wrapping_sub(1i32 as libc::c_uint) {
            (*s).cx = (*(*s).grid).sx.wrapping_sub(1i32 as libc::c_uint)
        }
        if 0 != cursor { (*s).cy = (*wp).saved_cy }
        if (*s).cy > (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint) {
            (*s).cy = (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint)
        }
        memcpy(gc as *mut libc::c_void,
               &mut (*wp).saved_cell as *mut grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        (*(*wp).base.grid).flags |= 1i32;
        if sy > (*(*wp).saved_grid).sy || sx != (*(*wp).saved_grid).sx {
            screen_resize(s, sx, sy, 1i32);
        }
        grid_destroy((*wp).saved_grid);
        (*wp).saved_grid = 0 as *mut grid;
        (*wp).flags |= 1i32;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_set_palette(mut wp: *mut window_pane,
                                                 mut n: u_int,
                                                 mut colour: libc::c_int)
 -> () {
    if n > 255i32 as libc::c_uint {
        return
    } else {
        if (*wp).palette == 0 as *mut libc::c_void as *mut libc::c_int {
            (*wp).palette =
                xcalloc(256i32 as size_t,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    as *mut libc::c_int
        }
        *(*wp).palette.offset(n as isize) = colour;
        (*wp).flags |= 1i32;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_unset_palette(mut wp: *mut window_pane,
                                                   mut n: u_int) -> () {
    if n > 255i32 as libc::c_uint ||
           (*wp).palette == 0 as *mut libc::c_void as *mut libc::c_int {
        return
    } else {
        *(*wp).palette.offset(n as isize) = 0i32;
        (*wp).flags |= 1i32;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_reset_palette(mut wp: *mut window_pane)
 -> () {
    if (*wp).palette == 0 as *mut libc::c_void as *mut libc::c_int {
        return
    } else {
        free((*wp).palette as *mut libc::c_void);
        (*wp).palette = 0 as *mut libc::c_int;
        (*wp).flags |= 1i32;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_set_mode(mut wp: *mut window_pane,
                                              mut mode: *const window_mode,
                                              mut fs: *mut cmd_find_state,
                                              mut args: *mut args)
 -> libc::c_int {
    let mut s: *mut screen = 0 as *mut screen;
    let mut tv: timeval = timeval{tv_sec: 10i32 as __time_t, tv_usec: 0,};
    if (*wp).mode != 0 as *mut libc::c_void as *const window_mode {
        return 1i32
    } else {
        (*wp).mode = mode;
        (*wp).modelast = time(0 as *mut time_t);
        event_set(&mut (*wp).modetimer as *mut event, 1i32.wrapping_neg(),
                  0i32 as libc::c_short, Some(window_pane_mode_timer),
                  wp as *mut libc::c_void);
        event_add(&mut (*wp).modetimer as *mut event,
                  &mut tv as *mut timeval);
        s =
            (*(*wp).mode).init.expect("non-null function pointer")(wp, fs,
                                                                   args);
        if s != 0 as *mut libc::c_void as *mut screen { (*wp).screen = s }
        (*wp).flags |= 1i32 | 128i32;
        server_status_window((*wp).window);
        notify_pane(b"pane-mode-changed\x00" as *const u8 as
                        *const libc::c_char, wp);
        return 0i32
    };
}
unsafe extern "C" fn window_pane_mode_timer(mut fd: libc::c_int,
                                            mut events: libc::c_short,
                                            mut arg: *mut libc::c_void)
 -> () {
    let mut wp: *mut window_pane = arg as *mut window_pane;
    let mut tv: timeval = timeval{tv_sec: 10i32 as __time_t, tv_usec: 0,};
    let mut n: libc::c_int = 0i32;
    event_del(&mut (*wp).modetimer as *mut event);
    event_add(&mut (*wp).modetimer as *mut event, &mut tv as *mut timeval);
    log_debug(b"%%%u in mode: last=%ld\x00" as *const u8 as
                  *const libc::c_char, (*wp).id, (*wp).modelast);
    if (*wp).modelast < time(0 as *mut time_t) - 180i32 as libc::c_long {
        if ioctl((*wp).fd, 21531i32 as libc::c_ulong,
                 &mut n as *mut libc::c_int) == 1i32.wrapping_neg() ||
               n > 0i32 {
            window_pane_reset_mode(wp);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_key(mut wp: *mut window_pane,
                                         mut c: *mut client,
                                         mut s: *mut session,
                                         mut key: key_code,
                                         mut m: *mut mouse_event) -> () {
    let mut wp2: *mut window_pane = 0 as *mut window_pane;
    if key &
           !(35184372088832u64 | 70368744177664u64 | 140737488355328u64 |
                 281474976710656u64) >=
           KEYC_MOUSE as libc::c_int as libc::c_ulonglong &&
           key &
               !(35184372088832u64 | 70368744177664u64 | 140737488355328u64 |
                     281474976710656u64) <
               KEYC_BSPACE as libc::c_int as libc::c_ulonglong &&
           m == 0 as *mut libc::c_void as *mut mouse_event {
        return
    } else if (*wp).mode != 0 as *mut libc::c_void as *const window_mode {
        (*wp).modelast = time(0 as *mut time_t);
        if (*(*wp).mode).key !=
               ::std::mem::transmute::<*mut libc::c_void,
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut window_pane,
                                                                   _:
                                                                       *mut client,
                                                                   _:
                                                                       *mut session,
                                                                   _:
                                                                       key_code,
                                                                   _:
                                                                       *mut mouse_event)
                                                  ->
                                                      ()>>(0 as
                                                               *mut libc::c_void)
           {
            (*(*wp).mode).key.expect("non-null function pointer")(wp, c, s,
                                                                  key &
                                                                      !281474976710656u64,
                                                                  m);
        }
        return
    } else if (*wp).fd == 1i32.wrapping_neg() || 0 != (*wp).flags & 64i32 {
        return
    } else {
        input_key(wp, key, m);
        if key &
               !(35184372088832u64 | 70368744177664u64 | 140737488355328u64 |
                     281474976710656u64) >=
               KEYC_MOUSE as libc::c_int as libc::c_ulonglong &&
               key &
                   !(35184372088832u64 | 70368744177664u64 |
                         140737488355328u64 | 281474976710656u64) <
                   KEYC_BSPACE as libc::c_int as libc::c_ulonglong {
            return
        } else {
            if 0 !=
                   options_get_number((*(*wp).window).options,
                                      b"synchronize-panes\x00" as *const u8 as
                                          *const libc::c_char) {
                wp2 =
                    (*(&mut (*(*wp).window).panes as
                           *mut window_panes)).tqh_first;
                while wp2 != 0 as *mut libc::c_void as *mut window_pane {
                    if !(wp2 == wp ||
                             (*wp2).mode !=
                                 0 as *mut libc::c_void as *const window_mode)
                       {
                        if !((*wp2).fd == 1i32.wrapping_neg() ||
                                 0 != (*wp2).flags & 64i32) {
                            if 0 != window_pane_visible(wp2) {
                                input_key(wp2, key, 0 as *mut mouse_event);
                            }
                        }
                    }
                    wp2 = (*wp2).entry.tqe_next
                }
            }
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_search(mut wp: *mut window_pane,
                                            mut searchstr:
                                                *const libc::c_char)
 -> u_int {
    let mut s: *mut screen = &mut (*wp).base as *mut screen;
    let mut newsearchstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: u_int = 0;
    xasprintf(&mut newsearchstr as *mut *mut libc::c_char,
              b"*%s*\x00" as *const u8 as *const libc::c_char, searchstr);
    i = 0i32 as u_int;
    while i < (*(*s).grid).sy {
        line =
            grid_view_string_cells((*s).grid, 0i32 as u_int, i,
                                   (*(*s).grid).sx);
        if fnmatch(newsearchstr, line, 0i32) == 0i32 {
            free(line as *mut libc::c_void);
            break ;
        } else { free(line as *mut libc::c_void); i = i.wrapping_add(1) }
    }
    free(newsearchstr as *mut libc::c_void);
    if i == (*(*s).grid).sy {
        return 0i32 as u_int
    } else { return i.wrapping_add(1i32 as libc::c_uint) };
}
#[no_mangle]
pub unsafe extern "C" fn window_printable_flags(mut wl: *mut winlink)
 -> *const libc::c_char {
    let mut s: *mut session = (*wl).session;
    static mut flags: [libc::c_char; 32] = unsafe { [0; 32] };
    let mut pos: libc::c_int = 0;
    pos = 0i32;
    if 0 != (*wl).flags & 2i32 {
        let fresh22 = pos;
        pos = pos + 1;
        flags[fresh22 as usize] = 35 as libc::c_char
    }
    if 0 != (*wl).flags & 1i32 {
        let fresh23 = pos;
        pos = pos + 1;
        flags[fresh23 as usize] = 33 as libc::c_char
    }
    if 0 != (*wl).flags & 4i32 {
        let fresh24 = pos;
        pos = pos + 1;
        flags[fresh24 as usize] = 126 as libc::c_char
    }
    if wl == (*s).curw {
        let fresh25 = pos;
        pos = pos + 1;
        flags[fresh25 as usize] = 42 as libc::c_char
    }
    if wl == (*(&mut (*s).lastw as *mut winlink_stack)).tqh_first {
        let fresh26 = pos;
        pos = pos + 1;
        flags[fresh26 as usize] = 45 as libc::c_char
    }
    if 0 != server_check_marked() && wl == marked_pane.wl {
        let fresh27 = pos;
        pos = pos + 1;
        flags[fresh27 as usize] = 77 as libc::c_char
    }
    if 0 != (*(*wl).window).flags & 4096i32 {
        let fresh28 = pos;
        pos = pos + 1;
        flags[fresh28 as usize] = 90 as libc::c_char
    }
    flags[pos as usize] = 0 as libc::c_char;
    return flags.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_up(mut wp: *mut window_pane)
 -> *mut window_pane {
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut list: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut edge: u_int = 0;
    let mut left: u_int = 0;
    let mut right: u_int = 0;
    let mut end: u_int = 0;
    let mut size: u_int = 0;
    let mut status: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    if wp == 0 as *mut libc::c_void as *mut window_pane ||
           0 == window_pane_visible(wp) {
        return 0 as *mut window_pane
    } else {
        status =
            options_get_number((*(*wp).window).options,
                               b"pane-border-status\x00" as *const u8 as
                                   *const libc::c_char) as libc::c_int;
        list = 0 as *mut *mut window_pane;
        size = 0i32 as u_int;
        edge = (*wp).yoff;
        if edge == if status == 1i32 { 1i32 } else { 0i32 } as libc::c_uint {
            edge =
                (*(*wp).window).sy.wrapping_add(1i32 as
                                                    libc::c_uint).wrapping_sub(if status
                                                                                      ==
                                                                                      2i32
                                                                                  {
                                                                                   1i32
                                                                               } else {
                                                                                   0i32
                                                                               }
                                                                                   as
                                                                                   libc::c_uint)
        }
        left = (*wp).xoff;
        right = (*wp).xoff.wrapping_add((*wp).sx);
        next = (*(&mut (*(*wp).window).panes as *mut window_panes)).tqh_first;
        while next != 0 as *mut libc::c_void as *mut window_pane {
            if !(next == wp || 0 == window_pane_visible(next)) {
                if !((*next).yoff.wrapping_add((*next).sy).wrapping_add(1i32
                                                                            as
                                                                            libc::c_uint)
                         != edge) {
                    end =
                        (*next).xoff.wrapping_add((*next).sx).wrapping_sub(1i32
                                                                               as
                                                                               libc::c_uint);
                    found = 0i32;
                    if (*next).xoff < left && end > right {
                        found = 1i32
                    } else if (*next).xoff >= left && (*next).xoff <= right {
                        found = 1i32
                    } else if end >= left && end <= right { found = 1i32 }
                    if !(0 == found) {
                        list =
                            xreallocarray(list as *mut libc::c_void,
                                          size.wrapping_add(1i32 as
                                                                libc::c_uint)
                                              as size_t,
                                          ::std::mem::size_of::<*mut window_pane>()
                                              as libc::c_ulong) as
                                *mut *mut window_pane;
                        let fresh29 = size;
                        size = size.wrapping_add(1);
                        let ref mut fresh30 = *list.offset(fresh29 as isize);
                        *fresh30 = next
                    }
                }
            }
            next = (*next).entry.tqe_next
        }
        best = window_pane_choose_best(list, size);
        free(list as *mut libc::c_void);
        return best
    };
}
unsafe extern "C" fn window_pane_choose_best(mut list: *mut *mut window_pane,
                                             mut size: u_int)
 -> *mut window_pane {
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut i: u_int = 0;
    if size == 0i32 as libc::c_uint {
        return 0 as *mut window_pane
    } else {
        best = *list.offset(0isize);
        i = 1i32 as u_int;
        while i < size {
            next = *list.offset(i as isize);
            if (*next).active_point > (*best).active_point { best = next }
            i = i.wrapping_add(1)
        }
        return best
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_down(mut wp: *mut window_pane)
 -> *mut window_pane {
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut list: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut edge: u_int = 0;
    let mut left: u_int = 0;
    let mut right: u_int = 0;
    let mut end: u_int = 0;
    let mut size: u_int = 0;
    let mut status: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    if wp == 0 as *mut libc::c_void as *mut window_pane ||
           0 == window_pane_visible(wp) {
        return 0 as *mut window_pane
    } else {
        status =
            options_get_number((*(*wp).window).options,
                               b"pane-border-status\x00" as *const u8 as
                                   *const libc::c_char) as libc::c_int;
        list = 0 as *mut *mut window_pane;
        size = 0i32 as u_int;
        edge =
            (*wp).yoff.wrapping_add((*wp).sy).wrapping_add(1i32 as
                                                               libc::c_uint);
        if edge >=
               (*(*wp).window).sy.wrapping_sub(if status == 2i32 {
                                                   1i32
                                               } else { 0i32 } as
                                                   libc::c_uint) {
            edge = if status == 1i32 { 1i32 } else { 0i32 } as u_int
        }
        left = (*wp).xoff;
        right = (*wp).xoff.wrapping_add((*wp).sx);
        next = (*(&mut (*(*wp).window).panes as *mut window_panes)).tqh_first;
        while next != 0 as *mut libc::c_void as *mut window_pane {
            if !(next == wp || 0 == window_pane_visible(next)) {
                if !((*next).yoff != edge) {
                    end =
                        (*next).xoff.wrapping_add((*next).sx).wrapping_sub(1i32
                                                                               as
                                                                               libc::c_uint);
                    found = 0i32;
                    if (*next).xoff < left && end > right {
                        found = 1i32
                    } else if (*next).xoff >= left && (*next).xoff <= right {
                        found = 1i32
                    } else if end >= left && end <= right { found = 1i32 }
                    if !(0 == found) {
                        list =
                            xreallocarray(list as *mut libc::c_void,
                                          size.wrapping_add(1i32 as
                                                                libc::c_uint)
                                              as size_t,
                                          ::std::mem::size_of::<*mut window_pane>()
                                              as libc::c_ulong) as
                                *mut *mut window_pane;
                        let fresh31 = size;
                        size = size.wrapping_add(1);
                        let ref mut fresh32 = *list.offset(fresh31 as isize);
                        *fresh32 = next
                    }
                }
            }
            next = (*next).entry.tqe_next
        }
        best = window_pane_choose_best(list, size);
        free(list as *mut libc::c_void);
        return best
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_left(mut wp: *mut window_pane)
 -> *mut window_pane {
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut list: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut edge: u_int = 0;
    let mut top: u_int = 0;
    let mut bottom: u_int = 0;
    let mut end: u_int = 0;
    let mut size: u_int = 0;
    let mut found: libc::c_int = 0;
    if wp == 0 as *mut libc::c_void as *mut window_pane ||
           0 == window_pane_visible(wp) {
        return 0 as *mut window_pane
    } else {
        list = 0 as *mut *mut window_pane;
        size = 0i32 as u_int;
        edge = (*wp).xoff;
        if edge == 0i32 as libc::c_uint {
            edge = (*(*wp).window).sx.wrapping_add(1i32 as libc::c_uint)
        }
        top = (*wp).yoff;
        bottom = (*wp).yoff.wrapping_add((*wp).sy);
        next = (*(&mut (*(*wp).window).panes as *mut window_panes)).tqh_first;
        while next != 0 as *mut libc::c_void as *mut window_pane {
            if !(next == wp || 0 == window_pane_visible(next)) {
                if !((*next).xoff.wrapping_add((*next).sx).wrapping_add(1i32
                                                                            as
                                                                            libc::c_uint)
                         != edge) {
                    end =
                        (*next).yoff.wrapping_add((*next).sy).wrapping_sub(1i32
                                                                               as
                                                                               libc::c_uint);
                    found = 0i32;
                    if (*next).yoff < top && end > bottom {
                        found = 1i32
                    } else if (*next).yoff >= top && (*next).yoff <= bottom {
                        found = 1i32
                    } else if end >= top && end <= bottom { found = 1i32 }
                    if !(0 == found) {
                        list =
                            xreallocarray(list as *mut libc::c_void,
                                          size.wrapping_add(1i32 as
                                                                libc::c_uint)
                                              as size_t,
                                          ::std::mem::size_of::<*mut window_pane>()
                                              as libc::c_ulong) as
                                *mut *mut window_pane;
                        let fresh33 = size;
                        size = size.wrapping_add(1);
                        let ref mut fresh34 = *list.offset(fresh33 as isize);
                        *fresh34 = next
                    }
                }
            }
            next = (*next).entry.tqe_next
        }
        best = window_pane_choose_best(list, size);
        free(list as *mut libc::c_void);
        return best
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_right(mut wp: *mut window_pane)
 -> *mut window_pane {
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut list: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut edge: u_int = 0;
    let mut top: u_int = 0;
    let mut bottom: u_int = 0;
    let mut end: u_int = 0;
    let mut size: u_int = 0;
    let mut found: libc::c_int = 0;
    if wp == 0 as *mut libc::c_void as *mut window_pane ||
           0 == window_pane_visible(wp) {
        return 0 as *mut window_pane
    } else {
        list = 0 as *mut *mut window_pane;
        size = 0i32 as u_int;
        edge =
            (*wp).xoff.wrapping_add((*wp).sx).wrapping_add(1i32 as
                                                               libc::c_uint);
        if edge >= (*(*wp).window).sx { edge = 0i32 as u_int }
        top = (*wp).yoff;
        bottom = (*wp).yoff.wrapping_add((*wp).sy);
        next = (*(&mut (*(*wp).window).panes as *mut window_panes)).tqh_first;
        while next != 0 as *mut libc::c_void as *mut window_pane {
            if !(next == wp || 0 == window_pane_visible(next)) {
                if !((*next).xoff != edge) {
                    end =
                        (*next).yoff.wrapping_add((*next).sy).wrapping_sub(1i32
                                                                               as
                                                                               libc::c_uint);
                    found = 0i32;
                    if (*next).yoff < top && end > bottom {
                        found = 1i32
                    } else if (*next).yoff >= top && (*next).yoff <= bottom {
                        found = 1i32
                    } else if end >= top && end <= bottom { found = 1i32 }
                    if !(0 == found) {
                        list =
                            xreallocarray(list as *mut libc::c_void,
                                          size.wrapping_add(1i32 as
                                                                libc::c_uint)
                                              as size_t,
                                          ::std::mem::size_of::<*mut window_pane>()
                                              as libc::c_ulong) as
                                *mut *mut window_pane;
                        let fresh35 = size;
                        size = size.wrapping_add(1);
                        let ref mut fresh36 = *list.offset(fresh35 as isize);
                        *fresh36 = next
                    }
                }
            }
            next = (*next).entry.tqe_next
        }
        best = window_pane_choose_best(list, size);
        free(list as *mut libc::c_void);
        return best
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_set_name(mut w: *mut window,
                                         mut new_name: *const libc::c_char)
 -> () {
    free((*w).name as *mut libc::c_void);
    utf8_stravis(&mut (*w).name as *mut *mut libc::c_char, new_name,
                 1i32 | 2i32 | 8i32 | 16i32);
    notify_window(b"window-renamed\x00" as *const u8 as *const libc::c_char,
                  w);
}
#[no_mangle]
pub unsafe extern "C" fn winlink_clear_flags(mut wl: *mut winlink) -> () {
    let mut loop_0: *mut winlink = 0 as *mut winlink;
    (*(*wl).window).flags &= !(1i32 | 2i32 | 8i32);
    loop_0 = (*(&mut (*(*wl).window).winlinks as *mut unnamed_11)).tqh_first;
    while loop_0 != 0 as *mut libc::c_void as *mut winlink {
        if (*loop_0).flags & (1i32 | 2i32 | 4i32) != 0i32 {
            (*loop_0).flags &= !(1i32 | 2i32 | 4i32);
            server_status_session((*loop_0).session);
        }
        loop_0 = (*loop_0).wentry.tqe_next
    };
}
#[no_mangle]
pub unsafe extern "C" fn winlink_shuffle_up(mut s: *mut session,
                                            mut wl: *mut winlink)
 -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    idx = (*wl).idx + 1i32;
    last = idx;
    while last < 2147483647i32 {
        if winlink_find_by_index(&mut (*s).windows as *mut winlinks, last) ==
               0 as *mut libc::c_void as *mut winlink {
            break ;
        }
        last += 1
    }
    if last == 2147483647i32 {
        return 1i32.wrapping_neg()
    } else {
        while last > idx {
            wl =
                winlink_find_by_index(&mut (*s).windows as *mut winlinks,
                                      last - 1i32);
            server_link_window(s, wl, s, last, 0i32, 0i32,
                               0 as *mut *mut libc::c_char);
            server_unlink_window(s, wl);
            last -= 1
        }
        return idx
    };
}

