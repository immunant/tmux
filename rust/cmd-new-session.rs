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
    pub type input_ctx;
    pub type hooks;
    pub type _IO_FILE_plus;
    pub type format_job_tree;
    pub type screen_titles;
    pub type args_entry;
    pub type options;
    pub type options_entry;
    pub type evbuffer;
    pub type bufferevent_ops;
    pub type format_tree;
    pub type event_base;
    pub type tmuxproc;
    pub type tmuxpeer;
    pub type tty_code;
    pub type environ;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
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
    fn proc_send(_: *mut tmuxpeer, _: msgtype, _: libc::c_int,
                 _: *const libc::c_void, _: size_t) -> libc::c_int;
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn cfg_show_causes(_: *mut session) -> ();
    #[no_mangle]
    fn format_single(_: *mut cmdq_item, _: *const libc::c_char,
                     _: *mut client, _: *mut session, _: *mut winlink,
                     _: *mut window_pane) -> *mut libc::c_char;
    #[no_mangle]
    fn hooks_insert(_: *mut hooks, _: *mut cmdq_item, _: *mut cmd_find_state,
                    _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn notify_client(_: *const libc::c_char, _: *mut client) -> ();
    #[no_mangle]
    fn notify_session(_: *const libc::c_char, _: *mut session) -> ();
    #[no_mangle]
    fn options_get_string(_: *mut options, _: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    fn options_set_number(_: *mut options, _: *const libc::c_char,
                          _: libc::c_longlong) -> *mut options_entry;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn environ_create() -> *mut environ;
    #[no_mangle]
    fn environ_free(_: *mut environ) -> ();
    #[no_mangle]
    fn environ_find(_: *mut environ, _: *const libc::c_char)
     -> *mut environ_entry;
    #[no_mangle]
    fn environ_update(_: *mut options, _: *mut environ, _: *mut environ)
     -> ();
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_find_from_session(_: *mut cmd_find_state, _: *mut session,
                             _: libc::c_int) -> ();
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn cmd_attach_session(_: *mut cmdq_item, _: *const libc::c_char,
                          _: libc::c_int, _: libc::c_int,
                          _: *const libc::c_char, _: libc::c_int)
     -> cmd_retval;
    #[no_mangle]
    fn cmdq_print(_: *mut cmdq_item, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    fn server_update_socket() -> ();
    #[no_mangle]
    fn server_client_set_key_table(_: *mut client, _: *const libc::c_char)
     -> ();
    #[no_mangle]
    fn server_client_check_nested(_: *mut client) -> libc::c_int;
    #[no_mangle]
    fn server_client_open(_: *mut client, _: *mut *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn server_redraw_client(_: *mut client) -> ();
    #[no_mangle]
    fn status_timer_start(_: *mut client) -> ();
    #[no_mangle]
    fn recalculate_sizes() -> ();
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn window_set_name(_: *mut window, _: *const libc::c_char) -> ();
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
    #[no_mangle]
    fn session_find(_: *const libc::c_char) -> *mut session;
    #[no_mangle]
    fn session_create(_: *const libc::c_char, _: *const libc::c_char,
                      _: libc::c_int, _: *mut *mut libc::c_char,
                      _: *const libc::c_char, _: *const libc::c_char,
                      _: *mut environ, _: *mut termios, _: libc::c_int,
                      _: u_int, _: u_int, _: *mut *mut libc::c_char)
     -> *mut session;
    #[no_mangle]
    fn session_check_name(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn session_update_activity(_: *mut session, _: *mut timeval) -> ();
    #[no_mangle]
    fn session_select(_: *mut session, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn session_group_contains(_: *mut session) -> *mut session_group;
    #[no_mangle]
    fn session_group_find(_: *const libc::c_char) -> *mut session_group;
    #[no_mangle]
    fn session_group_new(_: *const libc::c_char) -> *mut session_group;
    #[no_mangle]
    fn session_group_add(_: *mut session_group, _: *mut session) -> ();
    #[no_mangle]
    fn session_group_synchronize_to(_: *mut session) -> ();
    #[no_mangle]
    fn fatal(_: *const libc::c_char, ...) -> !;
}
pub const MSG_EXEC: msgtype = 217;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_0,
}
pub type uint8_t = libc::c_uchar;
pub type unnamed = libc::c_uint;
pub type pid_t = __pid_t;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
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
    pub gentry: unnamed_19,
    pub entry: unnamed_34,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub type layout_type = libc::c_uint;
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const TTY_VT102: unnamed_10 = 2;
pub const MSG_IDENTIFY_CLIENTPID: msgtype = 107;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const MSG_WAKEUP: msgtype = 216;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const MSG_LOCK: msgtype = 206;
pub const CMDQ_COMMAND: cmdq_type = 0;
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub type __suseconds_t = libc::c_long;
pub const MSG_EXIT: msgtype = 203;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub ev_signal_next: unnamed_31,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
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
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type uint32_t = libc::c_uint;
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
pub type __off64_t = libc::c_long;
pub const JOB_CLOSED: unnamed_9 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_38 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const MSG_STDERR: msgtype = 211;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_39,
    pub entry: unnamed_29,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub type u_char = __u_char;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_9,
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
    pub entry: unnamed_4,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub type unnamed_9 = libc::c_uint;
pub type tcflag_t = libc::c_uint;
pub type __u_short = libc::c_ushort;
pub const JOB_RUNNING: unnamed_9 = 0;
pub const TTY_VT220: unnamed_10 = 3;
pub type unnamed_10 = libc::c_uint;
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
    pub message_log: unnamed_21,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_27,
}
pub const MSG_IDENTIFY_STDIN: msgtype = 104;
pub const TTY_UNKNOWN: unnamed_10 = 6;
pub type options_table_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_36,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const MSG_SHELL: msgtype = 209;
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
    pub wentry: unnamed_16,
    pub sentry: unnamed_15,
}
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_12 {
    ev_next_with_common_timeout: unnamed_32,
    min_heap_idx: libc::c_int,
}
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const JOB_DEAD: unnamed_9 = 1;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const MSG_EXITING: msgtype = 205;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const TTY_VT101: unnamed_10 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const MSG_SUSPEND: msgtype = 214;
pub const MSG_IDENTIFY_FLAGS: msgtype = 100;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub type cmd_find_type = libc::c_uint;
pub type uint16_t = libc::c_ushort;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub type __timezone_ptr_t = *mut timezone;
pub const MSG_DETACHKILL: msgtype = 202;
pub const MSG_RESIZE: msgtype = 208;
pub const MSG_IDENTIFY_TTYNAME: msgtype = 102;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub const MSG_SHUTDOWN: msgtype = 210;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub ev_io_next: unnamed_1,
    pub ev_timeout: timeval,
}
pub type speed_t = libc::c_uint;
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
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_18 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_30,
    pub ev_next: unnamed_25,
    pub ev_timeout_pos: unnamed_12,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_35,
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
pub struct unnamed_19 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const MSG_IDENTIFY_ENVIRON: msgtype = 105;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_28,
}
pub type __off_t = libc::c_long;
pub const MSG_IDENTIFY_TERM: msgtype = 101;
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
pub struct unnamed_20 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct environ_entry {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub entry: unnamed_37,
}
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
    pub term_type: unnamed_10,
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
pub const TTY_VT320: unnamed_10 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub type cc_t = libc::c_uchar;
pub type u_short = __u_short;
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
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub type u_int = __u_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const MSG_COMMAND: msgtype = 200;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const PROMPT_ENTRY: unnamed = 0;
pub const MSG_UNLOCK: msgtype = 215;
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_14,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub const MSG_IDENTIFY_DONE: msgtype = 106;
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_28 {
    offset: u_int,
    data: unnamed_3,
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
    pub entry: unnamed_20,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const MSG_DETACH: msgtype = 201;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const TTY_VT100: unnamed_10 = 0;
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
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_11,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub type msgtype = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_38 = 1;
pub const MSG_READY: msgtype = 207;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_24,
}
pub type _IO_lock_t = ();
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
    pub alerts_entry: unnamed_22,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_6,
    pub entry: unnamed_13,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
pub type time_t = __time_t;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const MSG_EXITED: msgtype = 204;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_18,
}
pub type __time_t = libc::c_long;
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
    pub entry: unnamed_5,
    pub tree_entry: unnamed_33,
}
pub const MSG_VERSION: msgtype = 12;
pub const LINE_SEL_NONE: unnamed_38 = 0;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_35 {
    ev_io: unnamed_17,
    ev_signal: unnamed_7,
}
pub const MSG_IDENTIFY_CWD: msgtype = 108;
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
    pub entry: unnamed_26,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const MSG_STDIN: msgtype = 212;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_2,
}
pub const MSG_IDENTIFY_OLDCWD: msgtype = 103;
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
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_38,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub const TTY_VT420: unnamed_10 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_23,
}
pub type unnamed_38 = libc::c_uint;
pub const PROMPT_COMMAND: unnamed = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const MSG_STDOUT: msgtype = 213;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
unsafe extern "C" fn cmd_new_session_exec(mut self_0: *mut cmd,
                                          mut item: *mut cmdq_item)
 -> cmd_retval {
    let mut current_block: u64;
    let mut args: *mut args = (*self_0).args;
    let mut c: *mut client = (*item).client;
    let mut s: *mut session = 0 as *mut session;
    let mut as_0: *mut session = 0 as *mut session;
    let mut groupwith: *mut session = 0 as *mut session;
    let mut w: *mut window = 0 as *mut window;
    let mut env: *mut environ = 0 as *mut environ;
    let mut tio: termios =
        termios{c_iflag: 0,
                c_oflag: 0,
                c_cflag: 0,
                c_lflag: 0,
                c_line: 0,
                c_cc: [0; 32],
                c_ispeed: 0,
                c_ospeed: 0,};
    let mut tiop: *mut termios = 0 as *mut termios;
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut template: *const libc::c_char = 0 as *const libc::c_char;
    let mut group: *const libc::c_char = 0 as *const libc::c_char;
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut detached: libc::c_int = 0;
    let mut already_attached: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut argc: libc::c_int = 0;
    let mut is_control: libc::c_int = 0i32;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut fs: cmd_find_state =
        cmd_find_state{flags: 0,
                       current:
                           0 as *const cmd_find_state as *mut cmd_find_state,
                       s: 0 as *const session as *mut session,
                       wl: 0 as *const winlink as *mut winlink,
                       w: 0 as *const window as *mut window,
                       wp: 0 as *const window_pane as *mut window_pane,
                       idx: 0,};
    let mut retval: cmd_retval = CMD_RETURN_NORMAL;
    if (*self_0).entry == &cmd_has_session_entry as *const cmd_entry {
        return CMD_RETURN_NORMAL
    } else if 0 != args_has(args, 116 as u_char) &&
                  ((*args).argc != 0i32 || 0 != args_has(args, 110 as u_char))
     {
        cmdq_error(item,
                   b"command or window name given with target\x00" as
                       *const u8 as *const libc::c_char);
        return CMD_RETURN_ERROR
    } else {
        newname = 0 as *mut libc::c_char;
        if 0 != args_has(args, 115 as u_char) {
            newname =
                format_single(item, args_get(args, 115 as u_char), c,
                              0 as *mut session, 0 as *mut winlink,
                              0 as *mut window_pane);
            if 0 == session_check_name(newname) {
                cmdq_error(item,
                           b"bad session name: %s\x00" as *const u8 as
                               *const libc::c_char, newname);
                current_block = 14594744208347999533;
            } else {
                as_0 = session_find(newname);
                if as_0 != 0 as *mut libc::c_void as *mut session {
                    if 0 != args_has(args, 65 as u_char) {
                        retval =
                            cmd_attach_session(item, newname,
                                               args_has(args, 68 as u_char),
                                               0i32, 0 as *const libc::c_char,
                                               args_has(args, 69 as u_char));
                        free(newname as *mut libc::c_void);
                        return retval
                    } else {
                        cmdq_error(item,
                                   b"duplicate session: %s\x00" as *const u8
                                       as *const libc::c_char, newname);
                    }
                    current_block = 14594744208347999533;
                } else { current_block = 11875828834189669668; }
            }
        } else { current_block = 11875828834189669668; }
        match current_block {
            11875828834189669668 => {
                group = args_get(args, 116 as u_char);
                if group != 0 as *mut libc::c_void as *const libc::c_char {
                    groupwith = (*item).target.s;
                    if groupwith == 0 as *mut libc::c_void as *mut session {
                        if 0 == session_check_name(group) {
                            cmdq_error(item,
                                       b"bad group name: %s\x00" as *const u8
                                           as *const libc::c_char, group);
                            current_block = 14594744208347999533;
                        } else {
                            sg = session_group_find(group);
                            current_block = 1841672684692190573;
                        }
                    } else {
                        sg = session_group_contains(groupwith);
                        current_block = 1841672684692190573;
                    }
                    match current_block {
                        14594744208347999533 => { }
                        _ => {
                            if sg !=
                                   0 as *mut libc::c_void as
                                       *mut session_group {
                                prefix = (*sg).name
                            } else if groupwith !=
                                          0 as *mut libc::c_void as
                                              *mut session {
                                prefix = (*groupwith).name
                            } else { prefix = group }
                            current_block = 17965632435239708295;
                        }
                    }
                } else {
                    groupwith = 0 as *mut session;
                    sg = 0 as *mut session_group;
                    prefix = 0 as *const libc::c_char;
                    current_block = 17965632435239708295;
                }
                match current_block {
                    14594744208347999533 => { }
                    _ => {
                        detached = args_has(args, 100 as u_char);
                        if c == 0 as *mut libc::c_void as *mut client {
                            detached = 1i32
                        } else if 0 != (*c).flags & 8192i32 {
                            is_control = 1i32
                        }
                        already_attached = 0i32;
                        if c != 0 as *mut libc::c_void as *mut client &&
                               (*c).session !=
                                   0 as *mut libc::c_void as *mut session {
                            already_attached = 1i32
                        }
                        tmp = args_get(args, 99 as u_char);
                        if tmp !=
                               0 as *mut libc::c_void as *const libc::c_char {
                            cwd =
                                format_single(item, tmp, c, 0 as *mut session,
                                              0 as *mut winlink,
                                              0 as *mut window_pane)
                        } else if c != 0 as *mut libc::c_void as *mut client
                                      &&
                                      (*c).session ==
                                          0 as *mut libc::c_void as
                                              *mut session &&
                                      (*c).cwd !=
                                          0 as *mut libc::c_void as
                                              *const libc::c_char {
                            cwd = xstrdup((*c).cwd)
                        } else {
                            cwd =
                                xstrdup(b".\x00" as *const u8 as
                                            *const libc::c_char)
                        }
                        if 0 == detached && 0 == already_attached &&
                               (*c).tty.fd != 1i32.wrapping_neg() {
                            if 0 != server_client_check_nested((*item).client)
                               {
                                cmdq_error(item,
                                           b"sessions should be nested with care, unset $TMUX to force\x00"
                                               as *const u8 as
                                               *const libc::c_char);
                                return CMD_RETURN_ERROR
                            } else if tcgetattr((*c).tty.fd,
                                                &mut tio as *mut termios) !=
                                          0i32 {
                                fatal(b"tcgetattr failed\x00" as *const u8 as
                                          *const libc::c_char);
                            } else { tiop = &mut tio as *mut termios }
                        } else { tiop = 0 as *mut termios }
                        if 0 == detached && 0 == already_attached {
                            if server_client_open(c,
                                                  &mut cause as
                                                      *mut *mut libc::c_char)
                                   != 0i32 {
                                cmdq_error(item,
                                           b"open terminal failed: %s\x00" as
                                               *const u8 as
                                               *const libc::c_char, cause);
                                free(cause as *mut libc::c_void);
                                current_block = 14594744208347999533;
                            } else { current_block = 13472856163611868459; }
                        } else { current_block = 13472856163611868459; }
                        match current_block {
                            14594744208347999533 => { }
                            _ => {
                                if 0 == detached {
                                    sx = (*c).tty.sx;
                                    sy = (*c).tty.sy;
                                    if 0 == is_control &&
                                           sy > 0i32 as libc::c_uint &&
                                           0 !=
                                               options_get_number(global_s_options,
                                                                  b"status\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char)
                                       {
                                        sy = sy.wrapping_sub(1)
                                    }
                                } else {
                                    sx = 80i32 as u_int;
                                    sy = 24i32 as u_int
                                }
                                if (0 != is_control || 0 != detached) &&
                                       0 != args_has(args, 120 as u_char) {
                                    sx =
                                        strtonum(args_get(args,
                                                          120 as u_char),
                                                 1i32 as libc::c_longlong,
                                                 (32767i32 * 2i32 + 1i32) as
                                                     libc::c_longlong,
                                                 &mut errstr as
                                                     *mut *const libc::c_char)
                                            as u_int;
                                    if errstr !=
                                           0 as *mut libc::c_void as
                                               *const libc::c_char {
                                        cmdq_error(item,
                                                   b"width %s\x00" as
                                                       *const u8 as
                                                       *const libc::c_char,
                                                   errstr);
                                        current_block = 14594744208347999533;
                                    } else {
                                        current_block = 5689316957504528238;
                                    }
                                } else {
                                    current_block = 5689316957504528238;
                                }
                                match current_block {
                                    14594744208347999533 => { }
                                    _ => {
                                        if (0 != is_control || 0 != detached)
                                               &&
                                               0 !=
                                                   args_has(args,
                                                            121 as u_char) {
                                            sy =
                                                strtonum(args_get(args,
                                                                  121 as
                                                                      u_char),
                                                         1i32 as
                                                             libc::c_longlong,
                                                         (32767i32 * 2i32 +
                                                              1i32) as
                                                             libc::c_longlong,
                                                         &mut errstr as
                                                             *mut *const libc::c_char)
                                                    as u_int;
                                            if errstr !=
                                                   0 as *mut libc::c_void as
                                                       *const libc::c_char {
                                                cmdq_error(item,
                                                           b"height %s\x00" as
                                                               *const u8 as
                                                               *const libc::c_char,
                                                           errstr);
                                                current_block =
                                                    14594744208347999533;
                                            } else {
                                                current_block =
                                                    652864300344834934;
                                            }
                                        } else {
                                            current_block =
                                                652864300344834934;
                                        }
                                        match current_block {
                                            14594744208347999533 => { }
                                            _ => {
                                                if sx == 0i32 as libc::c_uint
                                                   {
                                                    sx = 1i32 as u_int
                                                }
                                                if sy == 0i32 as libc::c_uint
                                                   {
                                                    sy = 1i32 as u_int
                                                }
                                                argc = 1i32.wrapping_neg();
                                                argv =
                                                    0 as
                                                        *mut *mut libc::c_char;
                                                if 0 ==
                                                       args_has(args,
                                                                116 as u_char)
                                                       && (*args).argc != 0i32
                                                   {
                                                    argc = (*args).argc;
                                                    argv = (*args).argv
                                                } else if sg ==
                                                              0 as
                                                                  *mut libc::c_void
                                                                  as
                                                                  *mut session_group
                                                              &&
                                                              groupwith ==
                                                                  0 as
                                                                      *mut libc::c_void
                                                                      as
                                                                      *mut session
                                                 {
                                                    cmd =
                                                        options_get_string(global_s_options,
                                                                           b"default-command\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char);
                                                    if cmd !=
                                                           0 as
                                                               *mut libc::c_void
                                                               as
                                                               *const libc::c_char
                                                           &&
                                                           *cmd as libc::c_int
                                                               != 0 {
                                                        argc = 1i32;
                                                        argv =
                                                            &mut cmd as
                                                                *mut *const libc::c_char
                                                                as
                                                                *mut *mut libc::c_char
                                                    } else {
                                                        argc = 0i32;
                                                        argv =
                                                            0 as
                                                                *mut *mut libc::c_char
                                                    }
                                                }
                                                path =
                                                    0 as *const libc::c_char;
                                                if c !=
                                                       0 as *mut libc::c_void
                                                           as *mut client &&
                                                       (*c).session ==
                                                           0 as
                                                               *mut libc::c_void
                                                               as *mut session
                                                   {
                                                    envent =
                                                        environ_find((*c).environ,
                                                                     b"PATH\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char)
                                                } else {
                                                    envent =
                                                        environ_find(global_environ,
                                                                     b"PATH\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char)
                                                }
                                                if envent !=
                                                       0 as *mut libc::c_void
                                                           as
                                                           *mut environ_entry
                                                   {
                                                    path = (*envent).value
                                                }
                                                env = environ_create();
                                                if c !=
                                                       0 as *mut libc::c_void
                                                           as *mut client &&
                                                       0 ==
                                                           args_has(args,
                                                                    69 as
                                                                        u_char)
                                                   {
                                                    environ_update(global_s_options,
                                                                   (*c).environ,
                                                                   env);
                                                }
                                                idx =
                                                    (1i32.wrapping_neg() as
                                                         libc::c_longlong -
                                                         options_get_number(global_s_options,
                                                                            b"base-index\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char))
                                                        as libc::c_int;
                                                s =
                                                    session_create(prefix,
                                                                   newname,
                                                                   argc, argv,
                                                                   path, cwd,
                                                                   env, tiop,
                                                                   idx, sx,
                                                                   sy,
                                                                   &mut cause
                                                                       as
                                                                       *mut *mut libc::c_char);
                                                environ_free(env);
                                                if s ==
                                                       0 as *mut libc::c_void
                                                           as *mut session {
                                                    cmdq_error(item,
                                                               b"create session failed: %s\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               cause);
                                                    free(cause as
                                                             *mut libc::c_void);
                                                } else {
                                                    if argc >= 0i32 &&
                                                           {
                                                               tmp =
                                                                   args_get(args,
                                                                            110
                                                                                as
                                                                                u_char);
                                                               tmp !=
                                                                   0 as
                                                                       *mut libc::c_void
                                                                       as
                                                                       *const libc::c_char
                                                           } {
                                                        cp =
                                                            format_single(item,
                                                                          tmp,
                                                                          c,
                                                                          s,
                                                                          0 as
                                                                              *mut winlink,
                                                                          0 as
                                                                              *mut window_pane);
                                                        w =
                                                            (*(*s).curw).window;
                                                        window_set_name(w,
                                                                        cp);
                                                        options_set_number((*w).options,
                                                                           b"automatic-rename\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           0i32
                                                                               as
                                                                               libc::c_longlong);
                                                        free(cp as
                                                                 *mut libc::c_void);
                                                    }
                                                    if group !=
                                                           0 as
                                                               *mut libc::c_void
                                                               as
                                                               *const libc::c_char
                                                       {
                                                        if sg ==
                                                               0 as
                                                                   *mut libc::c_void
                                                                   as
                                                                   *mut session_group
                                                           {
                                                            if groupwith !=
                                                                   0 as
                                                                       *mut libc::c_void
                                                                       as
                                                                       *mut session
                                                               {
                                                                sg =
                                                                    session_group_new((*groupwith).name);
                                                                session_group_add(sg,
                                                                                  groupwith);
                                                            } else {
                                                                sg =
                                                                    session_group_new(group)
                                                            }
                                                        }
                                                        session_group_add(sg,
                                                                          s);
                                                        session_group_synchronize_to(s);
                                                        session_select(s,
                                                                       (*winlinks_RB_MINMAX(&mut (*s).windows
                                                                                                as
                                                                                                *mut winlinks,
                                                                                            1i32.wrapping_neg())).idx);
                                                    }
                                                    notify_session(b"session-created\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   s);
                                                    if 0 == detached {
                                                        if 0 ==
                                                               already_attached
                                                           {
                                                            if 0 !=
                                                                   !(*c).flags
                                                                       &
                                                                       8192i32
                                                               {
                                                                proc_send((*c).peer,
                                                                          MSG_READY,
                                                                          1i32.wrapping_neg(),
                                                                          0 as
                                                                              *const libc::c_void,
                                                                          0i32
                                                                              as
                                                                              size_t);
                                                            }
                                                        } else if (*c).session
                                                                      !=
                                                                      0 as
                                                                          *mut libc::c_void
                                                                          as
                                                                          *mut session
                                                         {
                                                            (*c).last_session
                                                                = (*c).session
                                                        }
                                                        (*c).session = s;
                                                        if 0 !=
                                                               !(*(*item).shared).flags
                                                                   & 1i32 {
                                                            server_client_set_key_table(c,
                                                                                        0
                                                                                            as
                                                                                            *const libc::c_char);
                                                        }
                                                        status_timer_start(c);
                                                        notify_client(b"client-session-changed\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      c);
                                                        session_update_activity(s,
                                                                                0
                                                                                    as
                                                                                    *mut timeval);
                                                        gettimeofday(&mut (*s).last_attached_time
                                                                         as
                                                                         *mut timeval,
                                                                     0 as
                                                                         *mut timezone);
                                                        server_redraw_client(c);
                                                    }
                                                    recalculate_sizes();
                                                    server_update_socket();
                                                    if 0 != cfg_finished {
                                                        cfg_show_causes(s);
                                                    }
                                                    if 0 !=
                                                           args_has(args,
                                                                    80 as
                                                                        u_char)
                                                       {
                                                        template =
                                                            args_get(args,
                                                                     70 as
                                                                         u_char);
                                                        if template ==
                                                               0 as
                                                                   *mut libc::c_void
                                                                   as
                                                                   *const libc::c_char
                                                           {
                                                            template =
                                                                b"#{session_name}:\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                        }
                                                        cp =
                                                            format_single(item,
                                                                          template,
                                                                          c,
                                                                          s,
                                                                          0 as
                                                                              *mut winlink,
                                                                          0 as
                                                                              *mut window_pane);
                                                        cmdq_print(item,
                                                                   b"%s\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   cp);
                                                        free(cp as
                                                                 *mut libc::c_void);
                                                    }
                                                    if 0 == detached {
                                                        (*c).flags |= 128i32;
                                                        cmd_find_from_session(&mut (*(*item).shared).current
                                                                                  as
                                                                                  *mut cmd_find_state,
                                                                              s,
                                                                              0i32);
                                                    }
                                                    cmd_find_from_session(&mut fs
                                                                              as
                                                                              *mut cmd_find_state,
                                                                          s,
                                                                          0i32);
                                                    hooks_insert((*s).hooks,
                                                                 item,
                                                                 &mut fs as
                                                                     *mut cmd_find_state,
                                                                 b"after-new-session\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
                                                    free(cwd as
                                                             *mut libc::c_void);
                                                    free(newname as
                                                             *mut libc::c_void);
                                                    return CMD_RETURN_NORMAL
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        free(cwd as *mut libc::c_void);
        free(newname as *mut libc::c_void);
        return CMD_RETURN_ERROR
    };
}
#[no_mangle]
pub static mut cmd_has_session_entry: cmd_entry =
    unsafe {
        cmd_entry{name:
                      b"has-session\x00" as *const u8 as *const libc::c_char,
                  alias: b"has\x00" as *const u8 as *const libc::c_char,
                  args:
                      unnamed_11{template:
                                     b"t:\x00" as *const u8 as
                                         *const libc::c_char,
                                 lower: 0i32,
                                 upper: 0i32,},
                  usage:
                      b"[-t target-session]\x00" as *const u8 as
                          *const libc::c_char,
                  source:
                      cmd_entry_flag{flag: 0,
                                     type_0: CMD_FIND_PANE,
                                     flags: 0,},
                  target:
                      cmd_entry_flag{flag: 116 as libc::c_char,
                                     type_0: CMD_FIND_SESSION,
                                     flags: 0i32,},
                  flags: 0i32,
                  exec: Some(cmd_new_session_exec),}
    };
#[no_mangle]
pub static mut cmd_new_session_entry: cmd_entry =
    unsafe {
        cmd_entry{name:
                      b"new-session\x00" as *const u8 as *const libc::c_char,
                  alias: b"new\x00" as *const u8 as *const libc::c_char,
                  args:
                      unnamed_11{template:
                                     b"Ac:dDEF:n:Ps:t:x:y:\x00" as *const u8
                                         as *const libc::c_char,
                                 lower: 0i32,
                                 upper: 1i32.wrapping_neg(),},
                  usage:
                      b"[-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name] [-t target-session] [-x width] [-y height] [command]\x00"
                          as *const u8 as *const libc::c_char,
                  source:
                      cmd_entry_flag{flag: 0,
                                     type_0: CMD_FIND_PANE,
                                     flags: 0,},
                  target:
                      cmd_entry_flag{flag: 116 as libc::c_char,
                                     type_0: CMD_FIND_SESSION,
                                     flags: 64i32,},
                  flags: 1i32,
                  exec: Some(cmd_new_session_exec),}
    };

