extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    pub type tty_code;
    pub type format_job_tree;
    pub type environ;
    pub type options;
    pub type tmuxpeer;
    pub type hooks;
    pub type format_tree;
    pub type input_ctx;
    pub type tmuxproc;
    pub type screen_titles;
    pub type event_base;
    pub type args_entry;
    pub type bufferevent_ops;
    pub type evbuffer;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    static mut _sys_nerr: libc::c_int;
    #[no_mangle]
    static _sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
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
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t)
     -> *mut libc::c_void;
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
    static options_table: [options_table_entry; 0];
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
    fn colour_split_rgb(_: libc::c_int, _: *mut u_char, _: *mut u_char,
                        _: *mut u_char) -> ();
    #[no_mangle]
    fn utf8_set(_: *mut utf8_data, _: u_char) -> ();
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
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
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const TTY_VT320: unnamed_19 = 4;
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
pub type unnamed = libc::c_uint;
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
    pub term_type: unnamed_19,
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
pub struct unnamed_0 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pid_t = __pid_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
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
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const TTY_VT102: unnamed_19 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub type __suseconds_t = libc::c_long;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub type __u_short = libc::c_ushort;
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
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub type __u_int = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type options_table_type = libc::c_uint;
pub const JOB_DEAD: unnamed = 1;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_3,
    pub entry: unnamed_35,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub type time_t = __time_t;
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
    pub gentry: unnamed_15,
    pub entry: unnamed_5,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_22,
}
pub const TTY_VT101: unnamed_19 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
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
pub struct event {
    pub ev_active_next: unnamed_6,
    pub ev_next: unnamed_34,
    pub ev_timeout_pos: unnamed_33,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_32,
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
pub const CMDQ_CALLBACK: cmdq_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
pub const LINE_SEL_LEFT_RIGHT: unnamed_17 = 1;
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
    pub entry: unnamed_39,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub type options_table_scope = libc::c_uint;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_28,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_17,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type uint32_t = libc::c_uint;
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
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
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
    pub alerts_entry: unnamed_9,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_12,
    pub entry: unnamed_38,
}
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
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
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type uint8_t = libc::c_uchar;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const TTY_VT100: unnamed_19 = 0;
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub ev_signal_next: unnamed_23,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub type key_code = libc::c_ulonglong;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_25,
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
pub type unnamed_16 = libc::c_uint;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_30,
}
pub type unnamed_17 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
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
pub const TTY_VT220: unnamed_19 = 3;
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub type unnamed_19 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub type __timezone_ptr_t = *mut timezone;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const TTY_UNKNOWN: unnamed_19 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub type u_short = __u_short;
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
    pub entry: unnamed_20,
    pub tree_entry: unnamed_4,
}
pub type u_int = __u_int;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub const JOB_CLOSED: unnamed = 2;
pub type cmdq_type = libc::c_uint;
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
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_22 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
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
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_27,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type bitstr_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_25 {
    offset: u_int,
    data: unnamed_10,
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub ev_io_next: unnamed_0,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub const TTY_VT420: unnamed_19 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub mask: u_int,
    pub code: u_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub type tcflag_t = libc::c_uint;
pub const LINE_SEL_RIGHT_LEFT: unnamed_17 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_24,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_32 {
    ev_io: unnamed_26,
    ev_signal: unnamed_13,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_33 {
    ev_next_with_common_timeout: unnamed_31,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_18,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
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
    pub message_log: unnamed_21,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_16,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_7,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const PROMPT_ENTRY: unnamed_16 = 0;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type uint16_t = libc::c_ushort;
pub const PROMPT_COMMAND: unnamed_16 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
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
    pub entry: unnamed_14,
    pub wentry: unnamed_36,
    pub sentry: unnamed_37,
}
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed,
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
pub const JOB_RUNNING: unnamed = 0;
pub const LINE_SEL_NONE: unnamed_17 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub type u_char = __u_char;
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
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_8,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[no_mangle]
pub static mut grid_default_cell: grid_cell =
    unsafe {
        grid_cell{flags: 0i32 as u_char,
                  attr: 0i32 as u_short,
                  fg: 8i32,
                  bg: 8i32,
                  data:
                      utf8_data{data: [32 as u_char, 0, 0, 0, 0, 0, 0, 0, 0],
                                have: 0i32 as u_char,
                                size: 1i32 as u_char,
                                width: 1i32 as u_char,},}
    };
#[no_mangle]
pub unsafe extern "C" fn grid_cells_equal(mut gca: *const grid_cell,
                                          mut gcb: *const grid_cell)
 -> libc::c_int {
    if (*gca).fg != (*gcb).fg || (*gca).bg != (*gcb).bg {
        return 0i32
    } else if (*gca).attr as libc::c_int != (*gcb).attr as libc::c_int ||
                  (*gca).flags as libc::c_int != (*gcb).flags as libc::c_int {
        return 0i32
    } else if (*gca).data.width as libc::c_int !=
                  (*gcb).data.width as libc::c_int {
        return 0i32
    } else if (*gca).data.size as libc::c_int !=
                  (*gcb).data.size as libc::c_int {
        return 0i32
    } else {
        return (memcmp((*gca).data.data.as_ptr() as *const libc::c_void,
                       (*gcb).data.data.as_ptr() as *const libc::c_void,
                       (*gca).data.size as libc::c_ulong) == 0i32) as
                   libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn grid_create(mut sx: u_int, mut sy: u_int,
                                     mut hlimit: u_int) -> *mut grid {
    let mut gd: *mut grid = 0 as *mut grid;
    gd = xmalloc(::std::mem::size_of::<grid>() as libc::c_ulong) as *mut grid;
    (*gd).sx = sx;
    (*gd).sy = sy;
    (*gd).flags = 1i32;
    (*gd).hscrolled = 0i32 as u_int;
    (*gd).hsize = 0i32 as u_int;
    (*gd).hlimit = hlimit;
    if (*gd).sy != 0i32 as libc::c_uint {
        (*gd).linedata =
            xcalloc((*gd).sy as size_t,
                    ::std::mem::size_of::<grid_line>() as libc::c_ulong) as
                *mut grid_line
    } else { (*gd).linedata = 0 as *mut grid_line }
    return gd;
}
#[no_mangle]
pub unsafe extern "C" fn grid_destroy(mut gd: *mut grid) -> () {
    grid_free_lines(gd, 0i32 as u_int, (*gd).hsize.wrapping_add((*gd).sy));
    free((*gd).linedata as *mut libc::c_void);
    free(gd as *mut libc::c_void);
}
unsafe extern "C" fn grid_free_lines(mut gd: *mut grid, mut py: u_int,
                                     mut ny: u_int) -> () {
    let mut yy: u_int = 0;
    yy = py;
    while yy < py.wrapping_add(ny) {
        grid_free_line(gd, yy);
        yy = yy.wrapping_add(1)
    };
}
unsafe extern "C" fn grid_free_line(mut gd: *mut grid, mut py: u_int) -> () {
    free((*(*gd).linedata.offset(py as isize)).celldata as *mut libc::c_void);
    let ref mut fresh0 = (*(*gd).linedata.offset(py as isize)).celldata;
    *fresh0 = 0 as *mut grid_cell_entry;
    free((*(*gd).linedata.offset(py as isize)).extddata as *mut libc::c_void);
    let ref mut fresh1 = (*(*gd).linedata.offset(py as isize)).extddata;
    *fresh1 = 0 as *mut grid_cell;
}
#[no_mangle]
pub unsafe extern "C" fn grid_compare(mut ga: *mut grid, mut gb: *mut grid)
 -> libc::c_int {
    let mut gla: *mut grid_line = 0 as *mut grid_line;
    let mut glb: *mut grid_line = 0 as *mut grid_line;
    let mut gca: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut gcb: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    if (*ga).sx != (*gb).sx || (*ga).sy != (*gb).sy {
        return 1i32
    } else {
        yy = 0i32 as u_int;
        loop  {
            if yy < (*ga).sy {
                gla =
                    &mut *(*ga).linedata.offset(yy as isize) as
                        *mut grid_line;
                glb =
                    &mut *(*gb).linedata.offset(yy as isize) as
                        *mut grid_line;
                if (*gla).cellsize != (*glb).cellsize {
                    return 1i32
                } else {
                    xx = 0i32 as u_int;
                    loop  {
                        if xx < (*gla).cellsize {
                            grid_get_cell(ga, xx, yy,
                                          &mut gca as *mut grid_cell);
                            grid_get_cell(gb, xx, yy,
                                          &mut gcb as *mut grid_cell);
                            if 0 ==
                                   grid_cells_equal(&mut gca as
                                                        *mut grid_cell,
                                                    &mut gcb as
                                                        *mut grid_cell) {
                                return 1i32
                            } else { xx = xx.wrapping_add(1) }
                        } else { yy = yy.wrapping_add(1); break ; }
                    }
                }
            } else { return 0i32 }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn grid_get_cell(mut gd: *mut grid, mut px: u_int,
                                       mut py: u_int, mut gc: *mut grid_cell)
 -> () {
    if grid_check_y(gd,
                    (*::std::mem::transmute::<&[u8; 14],
                                              &[libc::c_char; 14]>(b"grid_get_cell\x00")).as_ptr(),
                    py) != 0i32 ||
           px >= (*(*gd).linedata.offset(py as isize)).cellsize {
        memcpy(gc as *mut libc::c_void,
               &grid_default_cell as *const grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        return
    } else {
        return grid_get_cell1(&mut *(*gd).linedata.offset(py as isize) as
                                  *mut grid_line, px, gc)
    };
}
unsafe extern "C" fn grid_get_cell1(mut gl: *mut grid_line, mut px: u_int,
                                    mut gc: *mut grid_cell) -> () {
    let mut gce: *mut grid_cell_entry =
        &mut *(*gl).celldata.offset(px as isize) as *mut grid_cell_entry;
    if 0 != (*gce).flags as libc::c_int & 8i32 {
        if (*gce).unnamed.offset >= (*gl).extdsize {
            memcpy(gc as *mut libc::c_void,
                   &grid_default_cell as *const grid_cell as
                       *const libc::c_void,
                   ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        } else {
            memcpy(gc as *mut libc::c_void,
                   &mut *(*gl).extddata.offset((*gce).unnamed.offset as isize)
                       as *mut grid_cell as *const libc::c_void,
                   ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        }
        return
    } else {
        (*gc).flags =
            ((*gce).flags as libc::c_int & !(1i32 | 2i32)) as u_char;
        (*gc).attr = (*gce).unnamed.data.attr as u_short;
        (*gc).fg = (*gce).unnamed.data.fg as libc::c_int;
        if 0 != (*gce).flags as libc::c_int & 1i32 { (*gc).fg |= 16777216i32 }
        (*gc).bg = (*gce).unnamed.data.bg as libc::c_int;
        if 0 != (*gce).flags as libc::c_int & 2i32 { (*gc).bg |= 16777216i32 }
        utf8_set(&mut (*gc).data as *mut utf8_data, (*gce).unnamed.data.data);
        return;
    };
}
unsafe extern "C" fn grid_check_y(mut gd: *mut grid,
                                  mut from: *const libc::c_char,
                                  mut py: u_int) -> libc::c_int {
    if py >= (*gd).hsize.wrapping_add((*gd).sy) {
        log_debug(b"%s: y out of range: %u\x00" as *const u8 as
                      *const libc::c_char, from, py);
        return 1i32.wrapping_neg()
    } else { return 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn grid_collect_history(mut gd: *mut grid) -> () {
    let mut ny: u_int = 0;
    if (*gd).hsize == 0i32 as libc::c_uint || (*gd).hsize < (*gd).hlimit {
        return
    } else {
        ny = (*gd).hlimit.wrapping_div(10i32 as libc::c_uint);
        if ny < 1i32 as libc::c_uint { ny = 1i32 as u_int }
        if ny > (*gd).hsize { ny = (*gd).hsize }
        grid_free_lines(gd, 0i32 as u_int, ny);
        memmove(&mut *(*gd).linedata.offset(0isize) as *mut grid_line as
                    *mut libc::c_void,
                &mut *(*gd).linedata.offset(ny as isize) as *mut grid_line as
                    *const libc::c_void,
                ((*gd).hsize.wrapping_add((*gd).sy).wrapping_sub(ny) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<grid_line>()
                                                     as libc::c_ulong));
        (*gd).hsize =
            ((*gd).hsize as libc::c_uint).wrapping_sub(ny) as u_int as u_int;
        if (*gd).hscrolled > (*gd).hsize { (*gd).hscrolled = (*gd).hsize }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn grid_scroll_history(mut gd: *mut grid, mut bg: u_int)
 -> () {
    let mut yy: u_int = 0;
    yy = (*gd).hsize.wrapping_add((*gd).sy);
    (*gd).linedata =
        xreallocarray((*gd).linedata as *mut libc::c_void,
                      yy.wrapping_add(1i32 as libc::c_uint) as size_t,
                      ::std::mem::size_of::<grid_line>() as libc::c_ulong) as
            *mut grid_line;
    grid_empty_line(gd, yy, bg);
    (*gd).hscrolled = (*gd).hscrolled.wrapping_add(1);
    grid_compact_line(&mut *(*gd).linedata.offset((*gd).hsize as isize) as
                          *mut grid_line);
    (*gd).hsize = (*gd).hsize.wrapping_add(1);
}
unsafe extern "C" fn grid_compact_line(mut gl: *mut grid_line) -> () {
    let mut new_extdsize: libc::c_int = 0i32;
    let mut new_extddata: *mut grid_cell = 0 as *mut grid_cell;
    let mut gce: *mut grid_cell_entry = 0 as *mut grid_cell_entry;
    let mut gc: *mut grid_cell = 0 as *mut grid_cell;
    let mut px: u_int = 0;
    let mut idx: u_int = 0;
    if (*gl).extdsize == 0i32 as libc::c_uint {
        return
    } else {
        px = 0i32 as u_int;
        while px < (*gl).cellsize {
            gce =
                &mut *(*gl).celldata.offset(px as isize) as
                    *mut grid_cell_entry;
            if 0 != (*gce).flags as libc::c_int & 8i32 { new_extdsize += 1 }
            px = px.wrapping_add(1)
        }
        if new_extdsize == 0i32 {
            free((*gl).extddata as *mut libc::c_void);
            (*gl).extddata = 0 as *mut grid_cell;
            (*gl).extdsize = 0i32 as u_int;
            return
        } else {
            new_extddata =
                xreallocarray(0 as *mut libc::c_void, new_extdsize as size_t,
                              ::std::mem::size_of::<grid_cell>() as
                                  libc::c_ulong) as *mut grid_cell;
            idx = 0i32 as u_int;
            px = 0i32 as u_int;
            while px < (*gl).cellsize {
                gce =
                    &mut *(*gl).celldata.offset(px as isize) as
                        *mut grid_cell_entry;
                if 0 != (*gce).flags as libc::c_int & 8i32 {
                    gc =
                        &mut *(*gl).extddata.offset((*gce).unnamed.offset as
                                                        isize) as
                            *mut grid_cell;
                    memcpy(&mut *new_extddata.offset(idx as isize) as
                               *mut grid_cell as *mut libc::c_void,
                           gc as *const libc::c_void,
                           ::std::mem::size_of::<grid_cell>() as
                               libc::c_ulong);
                    let fresh2 = idx;
                    idx = idx.wrapping_add(1);
                    (*gce).unnamed.offset = fresh2
                }
                px = px.wrapping_add(1)
            }
            free((*gl).extddata as *mut libc::c_void);
            (*gl).extddata = new_extddata;
            (*gl).extdsize = new_extdsize as u_int;
            return;
        }
    };
}
unsafe extern "C" fn grid_empty_line(mut gd: *mut grid, mut py: u_int,
                                     mut bg: u_int) -> () {
    memset(&mut *(*gd).linedata.offset(py as isize) as *mut grid_line as
               *mut libc::c_void, 0i32,
           ::std::mem::size_of::<grid_line>() as libc::c_ulong);
    if bg != 8i32 as libc::c_uint { grid_expand_line(gd, py, (*gd).sx, bg); };
}
unsafe extern "C" fn grid_expand_line(mut gd: *mut grid, mut py: u_int,
                                      mut sx: u_int, mut bg: u_int) -> () {
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut xx: u_int = 0;
    gl = &mut *(*gd).linedata.offset(py as isize) as *mut grid_line;
    if sx <= (*gl).cellsize {
        return
    } else {
        if sx < (*gd).sx.wrapping_div(4i32 as libc::c_uint) {
            sx = (*gd).sx.wrapping_div(4i32 as libc::c_uint)
        } else if sx < (*gd).sx.wrapping_div(2i32 as libc::c_uint) {
            sx = (*gd).sx.wrapping_div(2i32 as libc::c_uint)
        } else { sx = (*gd).sx }
        (*gl).celldata =
            xreallocarray((*gl).celldata as *mut libc::c_void, sx as size_t,
                          ::std::mem::size_of::<grid_cell_entry>() as
                              libc::c_ulong) as *mut grid_cell_entry;
        xx = (*gl).cellsize;
        while xx < sx {
            grid_clear_cell(gd, xx, py, bg);
            xx = xx.wrapping_add(1)
        }
        (*gl).cellsize = sx;
        return;
    };
}
unsafe extern "C" fn grid_clear_cell(mut gd: *mut grid, mut px: u_int,
                                     mut py: u_int, mut bg: u_int) -> () {
    let mut gl: *mut grid_line =
        &mut *(*gd).linedata.offset(py as isize) as *mut grid_line;
    let mut gce: *mut grid_cell_entry =
        &mut *(*gl).celldata.offset(px as isize) as *mut grid_cell_entry;
    let mut gc: *mut grid_cell = 0 as *mut grid_cell;
    memcpy(gce as *mut libc::c_void,
           &grid_default_entry as *const grid_cell_entry as
               *const libc::c_void,
           ::std::mem::size_of::<grid_cell_entry>() as libc::c_ulong);
    if 0 != bg & 33554432i32 as libc::c_uint {
        gc =
            grid_extended_cell(gl, gce,
                               &grid_default_cell as *const grid_cell);
        (*gc).bg = bg as libc::c_int
    } else {
        if 0 != bg & 16777216i32 as libc::c_uint {
            (*gce).flags = ((*gce).flags as libc::c_int | 2i32) as u_char
        }
        (*gce).unnamed.data.bg = bg as u_char
    };
}
unsafe extern "C" fn grid_extended_cell(mut gl: *mut grid_line,
                                        mut gce: *mut grid_cell_entry,
                                        mut gc: *const grid_cell)
 -> *mut grid_cell {
    let mut gcp: *mut grid_cell = 0 as *mut grid_cell;
    (*gl).flags |= 2i32;
    if 0 != !((*gce).flags as libc::c_int) & 8i32 {
        (*gl).extddata =
            xreallocarray((*gl).extddata as *mut libc::c_void,
                          (*gl).extdsize.wrapping_add(1i32 as libc::c_uint) as
                              size_t,
                          ::std::mem::size_of::<grid_cell>() as libc::c_ulong)
                as *mut grid_cell;
        let fresh3 = (*gl).extdsize;
        (*gl).extdsize = (*gl).extdsize.wrapping_add(1);
        (*gce).unnamed.offset = fresh3;
        (*gce).flags = ((*gc).flags as libc::c_int | 8i32) as u_char
    }
    if (*gce).unnamed.offset >= (*gl).extdsize {
        fatalx(b"offset too big\x00" as *const u8 as *const libc::c_char);
    } else {
        gcp =
            &mut *(*gl).extddata.offset((*gce).unnamed.offset as isize) as
                *mut grid_cell;
        memcpy(gcp as *mut libc::c_void, gc as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        return gcp
    };
}
static mut grid_default_entry: grid_cell_entry =
    unsafe {
        grid_cell_entry{flags: 0i32 as u_char,
                        unnamed:
                            unnamed_25{data:
                                           unnamed_10{attr: 0i32 as u_char,
                                                      fg: 8i32 as u_char,
                                                      bg: 8i32 as u_char,
                                                      data: 32 as u_char,},},}
    };
#[no_mangle]
pub unsafe extern "C" fn grid_scroll_history_region(mut gd: *mut grid,
                                                    mut upper: u_int,
                                                    mut lower: u_int,
                                                    mut bg: u_int) -> () {
    let mut gl_history: *mut grid_line = 0 as *mut grid_line;
    let mut gl_upper: *mut grid_line = 0 as *mut grid_line;
    let mut yy: u_int = 0;
    yy = (*gd).hsize.wrapping_add((*gd).sy);
    (*gd).linedata =
        xreallocarray((*gd).linedata as *mut libc::c_void,
                      yy.wrapping_add(1i32 as libc::c_uint) as size_t,
                      ::std::mem::size_of::<grid_line>() as libc::c_ulong) as
            *mut grid_line;
    gl_history =
        &mut *(*gd).linedata.offset((*gd).hsize as isize) as *mut grid_line;
    memmove(gl_history.offset(1isize) as *mut libc::c_void,
            gl_history as *const libc::c_void,
            ((*gd).sy as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<grid_line>()
                                                 as libc::c_ulong));
    upper = upper.wrapping_add(1);
    gl_upper = &mut *(*gd).linedata.offset(upper as isize) as *mut grid_line;
    lower = lower.wrapping_add(1);
    memcpy(gl_history as *mut libc::c_void, gl_upper as *const libc::c_void,
           ::std::mem::size_of::<grid_line>() as libc::c_ulong);
    memmove(gl_upper as *mut libc::c_void,
            gl_upper.offset(1isize) as *const libc::c_void,
            (lower.wrapping_sub(upper) as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<grid_line>()
                                                 as libc::c_ulong));
    grid_empty_line(gd, lower, bg);
    (*gd).hscrolled = (*gd).hscrolled.wrapping_add(1);
    (*gd).hsize = (*gd).hsize.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn grid_clear_history(mut gd: *mut grid) -> () {
    grid_free_lines(gd, 0i32 as u_int, (*gd).hsize);
    memmove(&mut *(*gd).linedata.offset(0isize) as *mut grid_line as
                *mut libc::c_void,
            &mut *(*gd).linedata.offset((*gd).hsize as isize) as
                *mut grid_line as *const libc::c_void,
            ((*gd).sy as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<grid_line>()
                                                 as libc::c_ulong));
    (*gd).hscrolled = 0i32 as u_int;
    (*gd).hsize = 0i32 as u_int;
    (*gd).linedata =
        xreallocarray((*gd).linedata as *mut libc::c_void, (*gd).sy as size_t,
                      ::std::mem::size_of::<grid_line>() as libc::c_ulong) as
            *mut grid_line;
}
#[no_mangle]
pub unsafe extern "C" fn grid_peek_line(mut gd: *mut grid, mut py: u_int)
 -> *const grid_line {
    if grid_check_y(gd,
                    (*::std::mem::transmute::<&[u8; 15],
                                              &[libc::c_char; 15]>(b"grid_peek_line\x00")).as_ptr(),
                    py) != 0i32 {
        return 0 as *const grid_line
    } else {
        return &mut *(*gd).linedata.offset(py as isize) as *mut grid_line
    };
}
#[no_mangle]
pub unsafe extern "C" fn grid_set_cell(mut gd: *mut grid, mut px: u_int,
                                       mut py: u_int,
                                       mut gc: *const grid_cell) -> () {
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut gce: *mut grid_cell_entry = 0 as *mut grid_cell_entry;
    if grid_check_y(gd,
                    (*::std::mem::transmute::<&[u8; 14],
                                              &[libc::c_char; 14]>(b"grid_set_cell\x00")).as_ptr(),
                    py) != 0i32 {
        return
    } else {
        grid_expand_line(gd, py, px.wrapping_add(1i32 as libc::c_uint),
                         8i32 as u_int);
        gl = &mut *(*gd).linedata.offset(py as isize) as *mut grid_line;
        if px.wrapping_add(1i32 as libc::c_uint) > (*gl).cellused {
            (*gl).cellused = px.wrapping_add(1i32 as libc::c_uint)
        }
        gce =
            &mut *(*gl).celldata.offset(px as isize) as *mut grid_cell_entry;
        if 0 != grid_need_extended_cell(gce, gc) {
            grid_extended_cell(gl, gce, gc);
        } else { grid_store_cell(gce, gc, (*gc).data.data[0usize]); }
        return;
    };
}
unsafe extern "C" fn grid_store_cell(mut gce: *mut grid_cell_entry,
                                     mut gc: *const grid_cell, mut c: u_char)
 -> () {
    (*gce).flags = (*gc).flags;
    (*gce).unnamed.data.fg = ((*gc).fg & 255i32) as u_char;
    if 0 != (*gc).fg & 16777216i32 {
        (*gce).flags = ((*gce).flags as libc::c_int | 1i32) as u_char
    }
    (*gce).unnamed.data.bg = ((*gc).bg & 255i32) as u_char;
    if 0 != (*gc).bg & 16777216i32 {
        (*gce).flags = ((*gce).flags as libc::c_int | 2i32) as u_char
    }
    (*gce).unnamed.data.attr = (*gc).attr as u_char;
    (*gce).unnamed.data.data = c;
}
unsafe extern "C" fn grid_need_extended_cell(mut gce: *const grid_cell_entry,
                                             mut gc: *const grid_cell)
 -> libc::c_int {
    if 0 != (*gce).flags as libc::c_int & 8i32 {
        return 1i32
    } else if (*gc).attr as libc::c_int > 255i32 {
        return 1i32
    } else if (*gc).data.size as libc::c_int != 1i32 ||
                  (*gc).data.width as libc::c_int != 1i32 {
        return 1i32
    } else if 0 != (*gc).fg & 33554432i32 || 0 != (*gc).bg & 33554432i32 {
        return 1i32
    } else { return 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn grid_set_cells(mut gd: *mut grid, mut px: u_int,
                                        mut py: u_int,
                                        mut gc: *const grid_cell,
                                        mut s: *const libc::c_char,
                                        mut slen: size_t) -> () {
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut gce: *mut grid_cell_entry = 0 as *mut grid_cell_entry;
    let mut gcp: *mut grid_cell = 0 as *mut grid_cell;
    let mut i: u_int = 0;
    if grid_check_y(gd,
                    (*::std::mem::transmute::<&[u8; 15],
                                              &[libc::c_char; 15]>(b"grid_set_cells\x00")).as_ptr(),
                    py) != 0i32 {
        return
    } else {
        grid_expand_line(gd, py,
                         (px as libc::c_ulong).wrapping_add(slen) as u_int,
                         8i32 as u_int);
        gl = &mut *(*gd).linedata.offset(py as isize) as *mut grid_line;
        if (px as libc::c_ulong).wrapping_add(slen) >
               (*gl).cellused as libc::c_ulong {
            (*gl).cellused = (px as libc::c_ulong).wrapping_add(slen) as u_int
        }
        i = 0i32 as u_int;
        while (i as libc::c_ulong) < slen {
            gce =
                &mut *(*gl).celldata.offset(px.wrapping_add(i) as isize) as
                    *mut grid_cell_entry;
            if 0 != grid_need_extended_cell(gce, gc) {
                gcp = grid_extended_cell(gl, gce, gc);
                utf8_set(&mut (*gcp).data as *mut utf8_data,
                         *s.offset(i as isize) as u_char);
            } else {
                grid_store_cell(gce, gc, *s.offset(i as isize) as u_char);
            }
            i = i.wrapping_add(1)
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn grid_clear(mut gd: *mut grid, mut px: u_int,
                                    mut py: u_int, mut nx: u_int,
                                    mut ny: u_int, mut bg: u_int) -> () {
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    if nx == 0i32 as libc::c_uint || ny == 0i32 as libc::c_uint {
        return
    } else if px == 0i32 as libc::c_uint && nx == (*gd).sx {
        grid_clear_lines(gd, py, ny, bg);
        return
    } else if grid_check_y(gd,
                           (*::std::mem::transmute::<&[u8; 11],
                                                     &[libc::c_char; 11]>(b"grid_clear\x00")).as_ptr(),
                           py) != 0i32 {
        return
    } else if grid_check_y(gd,
                           (*::std::mem::transmute::<&[u8; 11],
                                                     &[libc::c_char; 11]>(b"grid_clear\x00")).as_ptr(),
                           py.wrapping_add(ny).wrapping_sub(1i32 as
                                                                libc::c_uint))
                  != 0i32 {
        return
    } else {
        yy = py;
        while yy < py.wrapping_add(ny) {
            if px.wrapping_add(nx) >= (*gd).sx &&
                   px < (*(*gd).linedata.offset(yy as isize)).cellused {
                (*(*gd).linedata.offset(yy as isize)).cellused = px
            }
            if !(px > (*(*gd).linedata.offset(yy as isize)).cellsize &&
                     bg == 8i32 as libc::c_uint) {
                if px.wrapping_add(nx) >=
                       (*(*gd).linedata.offset(yy as isize)).cellsize &&
                       bg == 8i32 as libc::c_uint {
                    (*(*gd).linedata.offset(yy as isize)).cellsize = px
                } else {
                    grid_expand_line(gd, yy, px.wrapping_add(nx),
                                     8i32 as u_int);
                    xx = px;
                    while xx < px.wrapping_add(nx) {
                        grid_clear_cell(gd, xx, yy, bg);
                        xx = xx.wrapping_add(1)
                    }
                }
            }
            yy = yy.wrapping_add(1)
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn grid_clear_lines(mut gd: *mut grid, mut py: u_int,
                                          mut ny: u_int, mut bg: u_int)
 -> () {
    let mut yy: u_int = 0;
    if ny == 0i32 as libc::c_uint {
        return
    } else if grid_check_y(gd,
                           (*::std::mem::transmute::<&[u8; 17],
                                                     &[libc::c_char; 17]>(b"grid_clear_lines\x00")).as_ptr(),
                           py) != 0i32 {
        return
    } else if grid_check_y(gd,
                           (*::std::mem::transmute::<&[u8; 17],
                                                     &[libc::c_char; 17]>(b"grid_clear_lines\x00")).as_ptr(),
                           py.wrapping_add(ny).wrapping_sub(1i32 as
                                                                libc::c_uint))
                  != 0i32 {
        return
    } else {
        yy = py;
        while yy < py.wrapping_add(ny) {
            grid_free_line(gd, yy);
            grid_empty_line(gd, yy, bg);
            yy = yy.wrapping_add(1)
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn grid_move_lines(mut gd: *mut grid, mut dy: u_int,
                                         mut py: u_int, mut ny: u_int,
                                         mut bg: u_int) -> () {
    let mut yy: u_int = 0;
    if ny == 0i32 as libc::c_uint || py == dy {
        return
    } else if grid_check_y(gd,
                           (*::std::mem::transmute::<&[u8; 16],
                                                     &[libc::c_char; 16]>(b"grid_move_lines\x00")).as_ptr(),
                           py) != 0i32 {
        return
    } else if grid_check_y(gd,
                           (*::std::mem::transmute::<&[u8; 16],
                                                     &[libc::c_char; 16]>(b"grid_move_lines\x00")).as_ptr(),
                           py.wrapping_add(ny).wrapping_sub(1i32 as
                                                                libc::c_uint))
                  != 0i32 {
        return
    } else if grid_check_y(gd,
                           (*::std::mem::transmute::<&[u8; 16],
                                                     &[libc::c_char; 16]>(b"grid_move_lines\x00")).as_ptr(),
                           dy) != 0i32 {
        return
    } else if grid_check_y(gd,
                           (*::std::mem::transmute::<&[u8; 16],
                                                     &[libc::c_char; 16]>(b"grid_move_lines\x00")).as_ptr(),
                           dy.wrapping_add(ny).wrapping_sub(1i32 as
                                                                libc::c_uint))
                  != 0i32 {
        return
    } else {
        yy = dy;
        while yy < dy.wrapping_add(ny) {
            if !(yy >= py && yy < py.wrapping_add(ny)) {
                grid_free_line(gd, yy);
            }
            yy = yy.wrapping_add(1)
        }
        memmove(&mut *(*gd).linedata.offset(dy as isize) as *mut grid_line as
                    *mut libc::c_void,
                &mut *(*gd).linedata.offset(py as isize) as *mut grid_line as
                    *const libc::c_void,
                (ny as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<grid_line>()
                                                     as libc::c_ulong));
        yy = py;
        while yy < py.wrapping_add(ny) {
            if yy < dy || yy >= dy.wrapping_add(ny) {
                grid_empty_line(gd, yy, bg);
            }
            yy = yy.wrapping_add(1)
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn grid_move_cells(mut gd: *mut grid, mut dx: u_int,
                                         mut px: u_int, mut py: u_int,
                                         mut nx: u_int, mut bg: u_int) -> () {
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut xx: u_int = 0;
    if nx == 0i32 as libc::c_uint || px == dx {
        return
    } else if grid_check_y(gd,
                           (*::std::mem::transmute::<&[u8; 16],
                                                     &[libc::c_char; 16]>(b"grid_move_cells\x00")).as_ptr(),
                           py) != 0i32 {
        return
    } else {
        gl = &mut *(*gd).linedata.offset(py as isize) as *mut grid_line;
        grid_expand_line(gd, py, px.wrapping_add(nx), 8i32 as u_int);
        grid_expand_line(gd, py, dx.wrapping_add(nx), 8i32 as u_int);
        memmove(&mut *(*gl).celldata.offset(dx as isize) as
                    *mut grid_cell_entry as *mut libc::c_void,
                &mut *(*gl).celldata.offset(px as isize) as
                    *mut grid_cell_entry as *const libc::c_void,
                (nx as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<grid_cell_entry>()
                                                     as libc::c_ulong));
        if dx.wrapping_add(nx) > (*gl).cellused {
            (*gl).cellused = dx.wrapping_add(nx)
        }
        xx = px;
        while xx < px.wrapping_add(nx) {
            if !(xx >= dx && xx < dx.wrapping_add(nx)) {
                grid_clear_cell(gd, xx, py, bg);
            }
            xx = xx.wrapping_add(1)
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn grid_string_cells(mut gd: *mut grid, mut px: u_int,
                                           mut py: u_int, mut nx: u_int,
                                           mut lastgc: *mut *mut grid_cell,
                                           mut with_codes: libc::c_int,
                                           mut escape_c0: libc::c_int,
                                           mut trim: libc::c_int)
 -> *mut libc::c_char {
    let mut current_block: u64;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    static mut lastgc1: grid_cell =
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
    let mut data: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut code: [libc::c_char; 128] = [0; 128];
    let mut len: size_t = 0;
    let mut off: size_t = 0;
    let mut size: size_t = 0;
    let mut codelen: size_t = 0;
    let mut xx: u_int = 0;
    let mut gl: *const grid_line = 0 as *const grid_line;
    if lastgc != 0 as *mut libc::c_void as *mut *mut grid_cell &&
           *lastgc == 0 as *mut libc::c_void as *mut grid_cell {
        memcpy(&mut lastgc1 as *mut grid_cell as *mut libc::c_void,
               &grid_default_cell as *const grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        *lastgc = &mut lastgc1 as *mut grid_cell
    }
    len = 128i32 as size_t;
    buf = xmalloc(len) as *mut libc::c_char;
    off = 0i32 as size_t;
    gl = grid_peek_line(gd, py);
    xx = px;
    while xx < px.wrapping_add(nx) {
        if gl == 0 as *mut libc::c_void as *const grid_line ||
               xx >= (*gl).cellsize {
            break ;
        }
        grid_get_cell(gd, xx, py, &mut gc as *mut grid_cell);
        if !(0 != gc.flags as libc::c_int & 4i32) {
            if 0 != with_codes {
                grid_string_cells_code(*lastgc, &mut gc as *mut grid_cell,
                                       code.as_mut_ptr(),
                                       ::std::mem::size_of::<[libc::c_char; 128]>()
                                           as libc::c_ulong, escape_c0);
                codelen = strlen(code.as_mut_ptr());
                memcpy(*lastgc as *mut libc::c_void,
                       &mut gc as *mut grid_cell as *const libc::c_void,
                       ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
            } else { codelen = 0i32 as size_t }
            data = gc.data.data.as_mut_ptr() as *const libc::c_char;
            size = gc.data.size as size_t;
            if 0 != escape_c0 && size == 1i32 as libc::c_ulong &&
                   *data as libc::c_int == 92 {
                data = b"\\\\\x00" as *const u8 as *const libc::c_char;
                size = 2i32 as size_t
            }
            while len <
                      off.wrapping_add(size).wrapping_add(codelen).wrapping_add(1i32
                                                                                    as
                                                                                    libc::c_ulong)
                  {
                buf =
                    xreallocarray(buf as *mut libc::c_void, 2i32 as size_t,
                                  len) as *mut libc::c_char;
                len =
                    (len as libc::c_ulong).wrapping_mul(2i32 as libc::c_ulong)
                        as size_t as size_t
            }
            if codelen != 0i32 as libc::c_ulong {
                memcpy(buf.offset(off as isize) as *mut libc::c_void,
                       code.as_mut_ptr() as *const libc::c_void, codelen);
                off =
                    (off as libc::c_ulong).wrapping_add(codelen) as size_t as
                        size_t
            }
            memcpy(buf.offset(off as isize) as *mut libc::c_void,
                   data as *const libc::c_void, size);
            off =
                (off as libc::c_ulong).wrapping_add(size) as size_t as size_t
        }
        xx = xx.wrapping_add(1)
    }
    if 0 != trim {
        current_block = 8236137900636309791;
    } else { current_block = 3512920355445576850; }
    loop  {
        match current_block {
            3512920355445576850 => {
                *buf.offset(off as isize) = 0 as libc::c_char;
                return buf
            }
            _ => {
                if !(off > 0i32 as libc::c_ulong &&
                         *buf.offset(off.wrapping_sub(1i32 as libc::c_ulong)
                                         as isize) as libc::c_int == 32) {
                    current_block = 3512920355445576850;
                    continue ;
                }
                off = off.wrapping_sub(1);
                current_block = 8236137900636309791;
            }
        }
    };
}
unsafe extern "C" fn grid_string_cells_code(mut lastgc: *const grid_cell,
                                            mut gc: *const grid_cell,
                                            mut buf: *mut libc::c_char,
                                            mut len: size_t,
                                            mut escape_c0: libc::c_int)
 -> () {
    let mut oldc: [libc::c_int; 64] = [0; 64];
    let mut newc: [libc::c_int; 64] = [0; 64];
    let mut s: [libc::c_int; 128] = [0; 128];
    let mut noldc: size_t = 0;
    let mut nnewc: size_t = 0;
    let mut n: size_t = 0;
    let mut i: size_t = 0;
    let mut attr: u_int = (*gc).attr as u_int;
    let mut lastattr: u_int = (*lastgc).attr as u_int;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    let mut attrs: [unnamed_29; 8] =
        [unnamed_29{mask: 1i32 as u_int, code: 1i32 as u_int,},
         unnamed_29{mask: 2i32 as u_int, code: 2i32 as u_int,},
         unnamed_29{mask: 64i32 as u_int, code: 3i32 as u_int,},
         unnamed_29{mask: 4i32 as u_int, code: 4i32 as u_int,},
         unnamed_29{mask: 8i32 as u_int, code: 5i32 as u_int,},
         unnamed_29{mask: 16i32 as u_int, code: 7i32 as u_int,},
         unnamed_29{mask: 32i32 as u_int, code: 8i32 as u_int,},
         unnamed_29{mask: 256i32 as u_int, code: 9i32 as u_int,}];
    n = 0i32 as size_t;
    i = 0i32 as size_t;
    while i <
              (::std::mem::size_of::<[unnamed_29; 8]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<unnamed_29>()
                                                   as libc::c_ulong) {
        if 0 == attr & attrs[i as usize].mask &&
               0 != lastattr & attrs[i as usize].mask {
            let fresh4 = n;
            n = n.wrapping_add(1);
            s[fresh4 as usize] = 0i32;
            lastattr &= 128i32 as libc::c_uint;
            break ;
        } else { i = i.wrapping_add(1) }
    }
    i = 0i32 as size_t;
    while i <
              (::std::mem::size_of::<[unnamed_29; 8]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<unnamed_29>()
                                                   as libc::c_ulong) {
        if 0 != attr & attrs[i as usize].mask &&
               0 == lastattr & attrs[i as usize].mask {
            let fresh5 = n;
            n = n.wrapping_add(1);
            s[fresh5 as usize] = attrs[i as usize].code as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    *buf = 0 as libc::c_char;
    if n > 0i32 as libc::c_ulong {
        if 0 != escape_c0 {
            strlcat(buf, b"\\033[\x00" as *const u8 as *const libc::c_char,
                    len);
        } else {
            strlcat(buf, b"\x1b[\x00" as *const u8 as *const libc::c_char,
                    len);
        }
        i = 0i32 as size_t;
        while i < n {
            if i.wrapping_add(1i32 as libc::c_ulong) < n {
                xsnprintf(tmp.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong,
                          b"%d;\x00" as *const u8 as *const libc::c_char,
                          s[i as usize]);
            } else {
                xsnprintf(tmp.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong,
                          b"%d\x00" as *const u8 as *const libc::c_char,
                          s[i as usize]);
            }
            strlcat(buf, tmp.as_mut_ptr(), len);
            i = i.wrapping_add(1)
        }
        strlcat(buf, b"m\x00" as *const u8 as *const libc::c_char, len);
    }
    nnewc = grid_string_cells_fg(gc, newc.as_mut_ptr());
    noldc = grid_string_cells_fg(lastgc, oldc.as_mut_ptr());
    if nnewc != noldc ||
           memcmp(newc.as_mut_ptr() as *const libc::c_void,
                  oldc.as_mut_ptr() as *const libc::c_void,
                  nnewc.wrapping_mul(::std::mem::size_of::<libc::c_int>() as
                                         libc::c_ulong)) != 0i32 ||
           n != 0i32 as libc::c_ulong && s[0usize] == 0i32 {
        if 0 != escape_c0 {
            strlcat(buf, b"\\033[\x00" as *const u8 as *const libc::c_char,
                    len);
        } else {
            strlcat(buf, b"\x1b[\x00" as *const u8 as *const libc::c_char,
                    len);
        }
        i = 0i32 as size_t;
        while i < nnewc {
            if i.wrapping_add(1i32 as libc::c_ulong) < nnewc {
                xsnprintf(tmp.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong,
                          b"%d;\x00" as *const u8 as *const libc::c_char,
                          newc[i as usize]);
            } else {
                xsnprintf(tmp.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong,
                          b"%d\x00" as *const u8 as *const libc::c_char,
                          newc[i as usize]);
            }
            strlcat(buf, tmp.as_mut_ptr(), len);
            i = i.wrapping_add(1)
        }
        strlcat(buf, b"m\x00" as *const u8 as *const libc::c_char, len);
    }
    nnewc = grid_string_cells_bg(gc, newc.as_mut_ptr());
    noldc = grid_string_cells_bg(lastgc, oldc.as_mut_ptr());
    if nnewc != noldc ||
           memcmp(newc.as_mut_ptr() as *const libc::c_void,
                  oldc.as_mut_ptr() as *const libc::c_void,
                  nnewc.wrapping_mul(::std::mem::size_of::<libc::c_int>() as
                                         libc::c_ulong)) != 0i32 ||
           n != 0i32 as libc::c_ulong && s[0usize] == 0i32 {
        if 0 != escape_c0 {
            strlcat(buf, b"\\033[\x00" as *const u8 as *const libc::c_char,
                    len);
        } else {
            strlcat(buf, b"\x1b[\x00" as *const u8 as *const libc::c_char,
                    len);
        }
        i = 0i32 as size_t;
        while i < nnewc {
            if i.wrapping_add(1i32 as libc::c_ulong) < nnewc {
                xsnprintf(tmp.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong,
                          b"%d;\x00" as *const u8 as *const libc::c_char,
                          newc[i as usize]);
            } else {
                xsnprintf(tmp.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong,
                          b"%d\x00" as *const u8 as *const libc::c_char,
                          newc[i as usize]);
            }
            strlcat(buf, tmp.as_mut_ptr(), len);
            i = i.wrapping_add(1)
        }
        strlcat(buf, b"m\x00" as *const u8 as *const libc::c_char, len);
    }
    if 0 != attr & 128i32 as libc::c_uint &&
           0 == lastattr & 128i32 as libc::c_uint {
        if 0 != escape_c0 {
            strlcat(buf, b"\\016\x00" as *const u8 as *const libc::c_char,
                    len);
        } else {
            strlcat(buf, b"\x0e\x00" as *const u8 as *const libc::c_char,
                    len);
        }
    }
    if 0 == attr & 128i32 as libc::c_uint &&
           0 != lastattr & 128i32 as libc::c_uint {
        if 0 != escape_c0 {
            strlcat(buf, b"\\017\x00" as *const u8 as *const libc::c_char,
                    len);
        } else {
            strlcat(buf, b"\x0f\x00" as *const u8 as *const libc::c_char,
                    len);
        }
    };
}
unsafe extern "C" fn grid_string_cells_bg(mut gc: *const grid_cell,
                                          mut values: *mut libc::c_int)
 -> size_t {
    let mut current_block: u64;
    let mut n: size_t = 0;
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    n = 0i32 as size_t;
    if 0 != (*gc).bg & 16777216i32 {
        let fresh6 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh6 as isize) = 48i32;
        let fresh7 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh7 as isize) = 5i32;
        let fresh8 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh8 as isize) = (*gc).bg & 255i32
    } else if 0 != (*gc).bg & 33554432i32 {
        let fresh9 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh9 as isize) = 48i32;
        let fresh10 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh10 as isize) = 2i32;
        colour_split_rgb((*gc).bg, &mut r as *mut u_char,
                         &mut g as *mut u_char, &mut b as *mut u_char);
        let fresh11 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh11 as isize) = r as libc::c_int;
        let fresh12 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh12 as isize) = g as libc::c_int;
        let fresh13 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh13 as isize) = b as libc::c_int
    } else {
        match (*gc).bg {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                current_block = 3106966694870267009;
                match current_block {
                    3106966694870267009 => {
                        let fresh14 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh14 as isize) = (*gc).bg + 40i32
                    }
                    17401181849968117280 => {
                        let fresh16 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh16 as isize) = (*gc).bg - 10i32
                    }
                    _ => {
                        let fresh15 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh15 as isize) = 49i32
                    }
                }
            }
            8 => {
                current_block = 17810489416690824808;
                match current_block {
                    3106966694870267009 => {
                        let fresh14 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh14 as isize) = (*gc).bg + 40i32
                    }
                    17401181849968117280 => {
                        let fresh16 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh16 as isize) = (*gc).bg - 10i32
                    }
                    _ => {
                        let fresh15 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh15 as isize) = 49i32
                    }
                }
            }
            100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 => {
                current_block = 17401181849968117280;
                match current_block {
                    3106966694870267009 => {
                        let fresh14 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh14 as isize) = (*gc).bg + 40i32
                    }
                    17401181849968117280 => {
                        let fresh16 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh16 as isize) = (*gc).bg - 10i32
                    }
                    _ => {
                        let fresh15 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh15 as isize) = 49i32
                    }
                }
            }
            _ => { }
        }
    }
    return n;
}
unsafe extern "C" fn grid_string_cells_fg(mut gc: *const grid_cell,
                                          mut values: *mut libc::c_int)
 -> size_t {
    let mut current_block: u64;
    let mut n: size_t = 0;
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    n = 0i32 as size_t;
    if 0 != (*gc).fg & 16777216i32 {
        let fresh17 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh17 as isize) = 38i32;
        let fresh18 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh18 as isize) = 5i32;
        let fresh19 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh19 as isize) = (*gc).fg & 255i32
    } else if 0 != (*gc).fg & 33554432i32 {
        let fresh20 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh20 as isize) = 38i32;
        let fresh21 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh21 as isize) = 2i32;
        colour_split_rgb((*gc).fg, &mut r as *mut u_char,
                         &mut g as *mut u_char, &mut b as *mut u_char);
        let fresh22 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh22 as isize) = r as libc::c_int;
        let fresh23 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh23 as isize) = g as libc::c_int;
        let fresh24 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh24 as isize) = b as libc::c_int
    } else {
        match (*gc).fg {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                current_block = 10827708281775530352;
                match current_block {
                    2267897363793182377 => {
                        let fresh26 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh26 as isize) = 39i32
                    }
                    10827708281775530352 => {
                        let fresh25 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh25 as isize) = (*gc).fg + 30i32
                    }
                    _ => {
                        let fresh27 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh27 as isize) = (*gc).fg
                    }
                }
            }
            8 => {
                current_block = 2267897363793182377;
                match current_block {
                    2267897363793182377 => {
                        let fresh26 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh26 as isize) = 39i32
                    }
                    10827708281775530352 => {
                        let fresh25 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh25 as isize) = (*gc).fg + 30i32
                    }
                    _ => {
                        let fresh27 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh27 as isize) = (*gc).fg
                    }
                }
            }
            90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 => {
                current_block = 11516588741310721008;
                match current_block {
                    2267897363793182377 => {
                        let fresh26 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh26 as isize) = 39i32
                    }
                    10827708281775530352 => {
                        let fresh25 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh25 as isize) = (*gc).fg + 30i32
                    }
                    _ => {
                        let fresh27 = n;
                        n = n.wrapping_add(1);
                        *values.offset(fresh27 as isize) = (*gc).fg
                    }
                }
            }
            _ => { }
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn grid_duplicate_lines(mut dst: *mut grid,
                                              mut dy: u_int,
                                              mut src: *mut grid,
                                              mut sy: u_int, mut ny: u_int)
 -> () {
    let mut dstl: *mut grid_line = 0 as *mut grid_line;
    let mut srcl: *mut grid_line = 0 as *mut grid_line;
    let mut yy: u_int = 0;
    if dy.wrapping_add(ny) > (*dst).hsize.wrapping_add((*dst).sy) {
        ny = (*dst).hsize.wrapping_add((*dst).sy).wrapping_sub(dy)
    }
    if sy.wrapping_add(ny) > (*src).hsize.wrapping_add((*src).sy) {
        ny = (*src).hsize.wrapping_add((*src).sy).wrapping_sub(sy)
    }
    grid_free_lines(dst, dy, ny);
    yy = 0i32 as u_int;
    while yy < ny {
        srcl = &mut *(*src).linedata.offset(sy as isize) as *mut grid_line;
        dstl = &mut *(*dst).linedata.offset(dy as isize) as *mut grid_line;
        memcpy(dstl as *mut libc::c_void, srcl as *const libc::c_void,
               ::std::mem::size_of::<grid_line>() as libc::c_ulong);
        if (*srcl).cellsize != 0i32 as libc::c_uint {
            (*dstl).celldata =
                xreallocarray(0 as *mut libc::c_void,
                              (*srcl).cellsize as size_t,
                              ::std::mem::size_of::<grid_cell_entry>() as
                                  libc::c_ulong) as *mut grid_cell_entry;
            memcpy((*dstl).celldata as *mut libc::c_void,
                   (*srcl).celldata as *const libc::c_void,
                   ((*srcl).cellsize as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<grid_cell_entry>()
                                                        as libc::c_ulong));
        } else { (*dstl).celldata = 0 as *mut grid_cell_entry }
        if (*srcl).extdsize != 0i32 as libc::c_uint {
            (*dstl).extdsize = (*srcl).extdsize;
            (*dstl).extddata =
                xreallocarray(0 as *mut libc::c_void,
                              (*dstl).extdsize as size_t,
                              ::std::mem::size_of::<grid_cell>() as
                                  libc::c_ulong) as *mut grid_cell;
            memcpy((*dstl).extddata as *mut libc::c_void,
                   (*srcl).extddata as *const libc::c_void,
                   ((*dstl).extdsize as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<grid_cell>()
                                                        as libc::c_ulong));
        }
        sy = sy.wrapping_add(1);
        dy = dy.wrapping_add(1);
        yy = yy.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn grid_reflow(mut gd: *mut grid, mut sx: u_int,
                                     mut cursor: *mut u_int) -> () {
    let mut target: *mut grid = 0 as *mut grid;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut yy: u_int = 0;
    let mut cy: u_int = 0;
    let mut width: u_int = 0;
    let mut i: u_int = 0;
    let mut at: u_int = 0;
    let mut first: u_int = 0;
    let mut start: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    gettimeofday(&mut start as *mut timeval, 0 as *mut timezone);
    log_debug(b"%s: %u lines, new width %u\x00" as *const u8 as
                  *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"grid_reflow\x00")).as_ptr(),
              (*gd).hsize.wrapping_add((*gd).sy), sx);
    cy = (*gd).hsize.wrapping_add(*cursor);
    target = grid_create((*gd).sx, 0i32 as u_int, 0i32 as u_int);
    yy = 0i32 as u_int;
    while yy < (*gd).hsize.wrapping_add((*gd).sy) {
        gl = &mut *(*gd).linedata.offset(yy as isize) as *mut grid_line;
        if !(0 != (*gl).flags & 4i32) {
            width = 0i32 as u_int;
            at = width;
            first = at;
            if 0 != !(*gl).flags & 2i32 {
                first = 1i32 as u_int;
                width = (*gl).cellused;
                if width > sx { at = sx } else { at = width }
            } else {
                i = 0i32 as u_int;
                while i < (*gl).cellused {
                    grid_get_cell1(gl, i, &mut gc as *mut grid_cell);
                    if i == 0i32 as libc::c_uint {
                        first = gc.data.width as u_int
                    }
                    if at == 0i32 as libc::c_uint &&
                           width.wrapping_add(gc.data.width as libc::c_uint) >
                               sx {
                        at = i
                    }
                    width =
                        (width as
                             libc::c_uint).wrapping_add(gc.data.width as
                                                            libc::c_uint) as
                            u_int as u_int;
                    i = i.wrapping_add(1)
                }
            }
            if width == sx || first > sx {
                grid_reflow_move(target, gl);
            } else if width > sx {
                grid_reflow_split(target, gd, sx, yy, at,
                                  &mut cy as *mut u_int);
            } else if 0 != (*gl).flags & 1i32 {
                grid_reflow_join(target, gd, sx, yy, width,
                                 &mut cy as *mut u_int, 0i32);
            } else { grid_reflow_move(target, gl); }
        }
        yy = yy.wrapping_add(1)
    }
    if (*target).sy < (*gd).sy {
        grid_reflow_add(target, (*gd).sy.wrapping_sub((*target).sy));
    }
    (*gd).hsize = (*target).sy.wrapping_sub((*gd).sy);
    free((*gd).linedata as *mut libc::c_void);
    (*gd).linedata = (*target).linedata;
    free(target as *mut libc::c_void);
    if (*gd).hscrolled > (*gd).hsize { (*gd).hscrolled = (*gd).hsize }
    if cy < (*gd).hsize {
        *cursor = 0i32 as u_int
    } else { *cursor = cy.wrapping_sub((*gd).hsize) }
    gettimeofday(&mut tv as *mut timeval, 0 as *mut timezone);
    loop  {
        (*(&mut tv as *mut timeval)).tv_sec =
            (*(&mut tv as *mut timeval)).tv_sec -
                (*(&mut start as *mut timeval)).tv_sec;
        (*(&mut tv as *mut timeval)).tv_usec =
            (*(&mut tv as *mut timeval)).tv_usec -
                (*(&mut start as *mut timeval)).tv_usec;
        if (*(&mut tv as *mut timeval)).tv_usec < 0i32 as libc::c_long {
            let ref mut fresh28 = (*(&mut tv as *mut timeval)).tv_sec;
            *fresh28 -= 1;
            let ref mut fresh29 = (*(&mut tv as *mut timeval)).tv_usec;
            *fresh29 += 1000000i32 as libc::c_long
        }
        if !(0 != 0i32) { break ; }
    }
    log_debug(b"%s: now %u lines (in %llu.%06u seconds)\x00" as *const u8 as
                  *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"grid_reflow\x00")).as_ptr(),
              (*gd).hsize.wrapping_add((*gd).sy),
              tv.tv_sec as libc::c_ulonglong, tv.tv_usec as u_int);
}
unsafe extern "C" fn grid_reflow_add(mut gd: *mut grid, mut n: u_int)
 -> *mut grid_line {
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut sy: u_int = (*gd).sy.wrapping_add(n);
    (*gd).linedata =
        xreallocarray((*gd).linedata as *mut libc::c_void, sy as size_t,
                      ::std::mem::size_of::<grid_line>() as libc::c_ulong) as
            *mut grid_line;
    gl = &mut *(*gd).linedata.offset((*gd).sy as isize) as *mut grid_line;
    memset(gl as *mut libc::c_void, 0i32,
           (n as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<grid_line>()
                                                as libc::c_ulong));
    (*gd).sy = sy;
    return gl;
}
unsafe extern "C" fn grid_reflow_move(mut gd: *mut grid,
                                      mut from: *mut grid_line)
 -> *mut grid_line {
    let mut to: *mut grid_line = 0 as *mut grid_line;
    to = grid_reflow_add(gd, 1i32 as u_int);
    memcpy(to as *mut libc::c_void, from as *const libc::c_void,
           ::std::mem::size_of::<grid_line>() as libc::c_ulong);
    grid_reflow_dead(from);
    return to;
}
unsafe extern "C" fn grid_reflow_dead(mut gl: *mut grid_line) -> () {
    memset(gl as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<grid_line>() as libc::c_ulong);
    (*gl).flags = 4i32;
}
unsafe extern "C" fn grid_reflow_join(mut target: *mut grid,
                                      mut gd: *mut grid, mut sx: u_int,
                                      mut yy: u_int, mut width: u_int,
                                      mut cy: *mut u_int,
                                      mut already: libc::c_int) -> () {
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut from: *mut grid_line = 0 as *mut grid_line;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut lines: u_int = 0;
    let mut want: u_int = 0;
    let mut left: u_int = 0;
    let mut i: u_int = 0;
    let mut to: u_int = 0;
    let mut line: u_int = 0;
    let mut at: u_int = 0;
    let mut wrapped: libc::c_int = 1i32;
    if 0 == already {
        to = (*target).sy;
        gl =
            grid_reflow_move(target,
                             &mut *(*gd).linedata.offset(yy as isize) as
                                 *mut grid_line)
    } else {
        to = (*target).sy.wrapping_sub(1i32 as libc::c_uint);
        gl = &mut *(*target).linedata.offset(to as isize) as *mut grid_line
    }
    at = (*gl).cellused;
    lines = 0i32 as u_int;
    while !(yy.wrapping_add(lines) == (*gd).hsize.wrapping_add((*gd).sy)) {
        line = yy.wrapping_add(1i32 as libc::c_uint).wrapping_add(lines);
        if 0 != !(*(*gd).linedata.offset(line as isize)).flags & 1i32 {
            wrapped = 0i32
        }
        if (*(*gd).linedata.offset(line as isize)).cellused ==
               0i32 as libc::c_uint {
            if 0 == wrapped { break ; }
        } else {
            grid_get_cell1(&mut *(*gd).linedata.offset(line as isize) as
                               *mut grid_line, 0i32 as u_int,
                           &mut gc as *mut grid_cell);
            if width.wrapping_add(gc.data.width as libc::c_uint) > sx {
                break ;
            }
            width =
                (width as
                     libc::c_uint).wrapping_add(gc.data.width as libc::c_uint)
                    as u_int as u_int;
            grid_set_cell(target, at, to, &mut gc as *mut grid_cell);
            at = at.wrapping_add(1);
            from =
                &mut *(*gd).linedata.offset(line as isize) as *mut grid_line;
            want = 1i32 as u_int;
            while want < (*from).cellused {
                grid_get_cell1(from, want, &mut gc as *mut grid_cell);
                if width.wrapping_add(gc.data.width as libc::c_uint) > sx {
                    break ;
                }
                width =
                    (width as
                         libc::c_uint).wrapping_add(gc.data.width as
                                                        libc::c_uint) as u_int
                        as u_int;
                grid_set_cell(target, at, to, &mut gc as *mut grid_cell);
                at = at.wrapping_add(1);
                want = want.wrapping_add(1)
            }
            lines = lines.wrapping_add(1);
            if 0 == wrapped || want != (*from).cellused || width == sx {
                break ;
            }
        }
    }
    if lines == 0i32 as libc::c_uint {
        return
    } else {
        left = (*from).cellused.wrapping_sub(want);
        if left != 0i32 as libc::c_uint {
            grid_move_cells(gd, 0i32 as u_int, want, yy.wrapping_add(lines),
                            left, 8i32 as u_int);
            (*from).cellused = left;
            (*from).cellsize = (*from).cellused;
            lines = lines.wrapping_sub(1)
        } else if 0 == wrapped { (*gl).flags &= !1i32 }
        i = yy.wrapping_add(1i32 as libc::c_uint);
        while i < yy.wrapping_add(1i32 as libc::c_uint).wrapping_add(lines) {
            free((*(*gd).linedata.offset(i as isize)).celldata as
                     *mut libc::c_void);
            free((*(*gd).linedata.offset(i as isize)).extddata as
                     *mut libc::c_void);
            grid_reflow_dead(&mut *(*gd).linedata.offset(i as isize) as
                                 *mut grid_line);
            i = i.wrapping_add(1)
        }
        if *cy > to.wrapping_add(lines) {
            *cy = (*cy as libc::c_uint).wrapping_sub(lines) as u_int as u_int
        } else if *cy > to { *cy = to }
        if (*gd).hscrolled > to.wrapping_add(lines) {
            (*gd).hscrolled =
                ((*gd).hscrolled as libc::c_uint).wrapping_sub(lines) as u_int
                    as u_int
        } else if (*gd).hscrolled > to { (*gd).hscrolled = to }
        return;
    };
}
unsafe extern "C" fn grid_reflow_split(mut target: *mut grid,
                                       mut gd: *mut grid, mut sx: u_int,
                                       mut yy: u_int, mut at: u_int,
                                       mut cy: *mut u_int) -> () {
    let mut gl: *mut grid_line =
        &mut *(*gd).linedata.offset(yy as isize) as *mut grid_line;
    let mut first: *mut grid_line = 0 as *mut grid_line;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut line: u_int = 0;
    let mut lines: u_int = 0;
    let mut width: u_int = 0;
    let mut i: u_int = 0;
    let mut xx: u_int = 0;
    let mut used: u_int = (*gl).cellused;
    let mut flags: libc::c_int = (*gl).flags;
    if 0 != !(*gl).flags & 2i32 {
        lines =
            (1i32 as
                 libc::c_uint).wrapping_add((*gl).cellused.wrapping_sub(1i32
                                                                            as
                                                                            libc::c_uint).wrapping_div(sx))
    } else {
        lines = 2i32 as u_int;
        width = 0i32 as u_int;
        i = at;
        while i < used {
            grid_get_cell1(gl, i, &mut gc as *mut grid_cell);
            if width.wrapping_add(gc.data.width as libc::c_uint) > sx {
                lines = lines.wrapping_add(1);
                width = 0i32 as u_int
            }
            width =
                (width as
                     libc::c_uint).wrapping_add(gc.data.width as libc::c_uint)
                    as u_int as u_int;
            i = i.wrapping_add(1)
        }
    }
    line = (*target).sy.wrapping_add(1i32 as libc::c_uint);
    first = grid_reflow_add(target, lines);
    width = 0i32 as u_int;
    xx = 0i32 as u_int;
    i = at;
    while i < used {
        grid_get_cell1(gl, i, &mut gc as *mut grid_cell);
        if width.wrapping_add(gc.data.width as libc::c_uint) > sx {
            (*(*target).linedata.offset(line as isize)).flags |= 1i32;
            line = line.wrapping_add(1);
            width = 0i32 as u_int;
            xx = 0i32 as u_int
        }
        width =
            (width as
                 libc::c_uint).wrapping_add(gc.data.width as libc::c_uint) as
                u_int as u_int;
        grid_set_cell(target, xx, line, &mut gc as *mut grid_cell);
        xx = xx.wrapping_add(1);
        i = i.wrapping_add(1)
    }
    if 0 != flags & 1i32 {
        (*(*target).linedata.offset(line as isize)).flags |= 1i32
    }
    (*gl).cellused = at;
    (*gl).cellsize = (*gl).cellused;
    (*gl).flags |= 1i32;
    memcpy(first as *mut libc::c_void, gl as *const libc::c_void,
           ::std::mem::size_of::<grid_line>() as libc::c_ulong);
    grid_reflow_dead(gl);
    if yy <= *cy {
        *cy =
            (*cy as
                 libc::c_uint).wrapping_add(lines.wrapping_sub(1i32 as
                                                                   libc::c_uint))
                as u_int as u_int
    }
    if yy <= (*gd).hscrolled {
        (*gd).hscrolled =
            ((*gd).hscrolled as
                 libc::c_uint).wrapping_add(lines.wrapping_sub(1i32 as
                                                                   libc::c_uint))
                as u_int as u_int
    }
    if width < sx && 0 != flags & 1i32 {
        grid_reflow_join(target, gd, sx, yy, width, cy, 1i32);
    };
}

