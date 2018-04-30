extern crate libc;
extern "C" {
    pub type evbuffer;
    pub type screen_titles;
    pub type tmuxproc;
    pub type args_entry;
    pub type options;
    pub type format_tree;
    pub type environ;
    pub type input_ctx;
    pub type hooks;
    pub type tmuxpeer;
    pub type event_base;
    pub type bufferevent_ops;
    pub type format_job_tree;
    pub type _IO_FILE_plus;
    pub type tty_code;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
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
    fn notify_window(_: *const libc::c_char, _: *mut window) -> ();
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
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
    static grid_default_cell: grid_cell;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn window_pane_resize(_: *mut window_pane, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
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
pub const JOB_DEAD: unnamed = 1;
pub type unnamed = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_2,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type key_code = libc::c_ulonglong;
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
    pub entry: unnamed_32,
    pub tree_entry: unnamed_5,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
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
    pub message_log: unnamed_10,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_7,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_34,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const TTY_VT102: unnamed_18 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub type __off_t = libc::c_long;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub type unnamed_2 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
pub struct unnamed_4 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
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
    pub entry: unnamed_23,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_38,
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
pub type uint8_t = libc::c_uchar;
pub type _IO_lock_t = ();
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_2 = 1;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub type u_char = __u_char;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
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
    pub qentry: unnamed_0,
}
pub const JOB_RUNNING: unnamed = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type __u_char = libc::c_uchar;
pub const CMDQ_COMMAND: cmdq_type = 0;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub type unnamed_7 = libc::c_uint;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_9 {
    ev_io: unnamed_19,
    ev_signal: unnamed_24,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
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
    pub alerts_entry: unnamed_12,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_25,
    pub entry: unnamed_11,
}
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
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
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_15 {
    ev_next_with_common_timeout: unnamed_6,
    min_heap_idx: libc::c_int,
}
pub const TTY_VT101: unnamed_18 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_20,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_37,
}
pub const LINE_SEL_NONE: unnamed_2 = 0;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_17 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
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
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub type tcflag_t = libc::c_uint;
pub type speed_t = libc::c_uint;
pub type unnamed_18 = libc::c_uint;
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
    pub entry: unnamed_1,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_29,
}
pub type __suseconds_t = libc::c_long;
pub const TTY_VT220: unnamed_18 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub ev_io_next: unnamed_3,
    pub ev_timeout: timeval,
}
pub type time_t = __time_t;
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_17,
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
pub struct cmd_find_state {
    pub flags: libc::c_int,
    pub current: *mut cmd_find_state,
    pub s: *mut session,
    pub wl: *mut winlink,
    pub w: *mut window,
    pub wp: *mut window_pane,
    pub idx: libc::c_int,
}
pub type __pid_t = libc::c_int;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_30,
    pub ev_next: unnamed_28,
    pub ev_timeout_pos: unnamed_15,
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
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_22,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_2 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_20 {
    offset: u_int,
    data: unnamed_26,
}
pub const TTY_UNKNOWN: unnamed_18 = 6;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const JOB_CLOSED: unnamed = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type cmd_find_type = libc::c_uint;
pub const TTY_VT100: unnamed_18 = 0;
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
    pub term_type: unnamed_18,
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_21,
    pub entry: unnamed_36,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub const PROMPT_COMMAND: unnamed_7 = 1;
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
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_31,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const CMD_RETURN_STOP: cmd_retval = 2;
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
    pub entry: unnamed_14,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub ev_signal_next: unnamed_13,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
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
pub type pid_t = __pid_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const TTY_VT420: unnamed_18 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub type cmd_retval = libc::c_int;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const PROMPT_ENTRY: unnamed_7 = 0;
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
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
pub struct unnamed_28 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type u_int = __u_int;
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
pub struct unnamed_29 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
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
    pub entry: unnamed_27,
    pub wentry: unnamed_33,
    pub sentry: unnamed_35,
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
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type layout_type = libc::c_uint;
pub const CMDQ_CALLBACK: cmdq_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_4,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const TTY_VT320: unnamed_18 = 4;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
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
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub type options_table_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
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
    pub gentry: unnamed_8,
    pub entry: unnamed_16,
}
#[no_mangle]
pub unsafe extern "C" fn layout_count_cells(mut lc: *mut layout_cell)
 -> u_int {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut count: u_int = 0;
    match (*lc).type_0 as libc::c_uint {
        2 => { return 1i32 as u_int }
        0 | 1 => {
            count = 0i32 as u_int;
            lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
            while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
                count =
                    (count as
                         libc::c_uint).wrapping_add(layout_count_cells(lcchild))
                        as u_int as u_int;
                lcchild = (*lcchild).entry.tqe_next
            }
            return count
        }
        _ => {
            fatalx(b"bad layout type\x00" as *const u8 as
                       *const libc::c_char);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_create_cell(mut lcparent: *mut layout_cell)
 -> *mut layout_cell {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    lc =
        xmalloc(::std::mem::size_of::<layout_cell>() as libc::c_ulong) as
            *mut layout_cell;
    (*lc).type_0 = LAYOUT_WINDOWPANE;
    (*lc).parent = lcparent;
    loop  {
        let ref mut fresh0 =
            (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
        *fresh0 = 0 as *mut layout_cell;
        let ref mut fresh1 =
            (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
        *fresh1 =
            &mut (*(&mut (*lc).cells as *mut layout_cells)).tqh_first as
                *mut *mut layout_cell;
        if !(0 != 0i32) { break ; }
    }
    (*lc).sx =
        (2147483647i32 as libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32);
    (*lc).sy =
        (2147483647i32 as libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32);
    (*lc).xoff =
        (2147483647i32 as libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32);
    (*lc).yoff =
        (2147483647i32 as libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32);
    (*lc).wp = 0 as *mut window_pane;
    return lc;
}
#[no_mangle]
pub unsafe extern "C" fn layout_free_cell(mut lc: *mut layout_cell) -> () {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    match (*lc).type_0 as libc::c_uint {
        0 | 1 => {
            while !((*(&mut (*lc).cells as *mut layout_cells)).tqh_first ==
                        0 as *mut libc::c_void as *mut layout_cell) {
                lcchild =
                    (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
                loop  {
                    if (*lcchild).entry.tqe_next !=
                           0 as *mut libc::c_void as *mut layout_cell {
                        (*(*lcchild).entry.tqe_next).entry.tqe_prev =
                            (*lcchild).entry.tqe_prev
                    } else {
                        let ref mut fresh2 =
                            (*(&mut (*lc).cells as
                                   *mut layout_cells)).tqh_last;
                        *fresh2 = (*lcchild).entry.tqe_prev
                    }
                    *(*lcchild).entry.tqe_prev = (*lcchild).entry.tqe_next;
                    if !(0 != 0i32) { break ; }
                }
                layout_free_cell(lcchild);
            }
        }
        2 => {
            if (*lc).wp != 0 as *mut libc::c_void as *mut window_pane {
                (*(*lc).wp).layout_cell = 0 as *mut layout_cell
            }
        }
        _ => { }
    }
    free(lc as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn layout_print_cell(mut lc: *mut layout_cell,
                                           mut hdr: *const libc::c_char,
                                           mut n: u_int) -> () {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    match (*lc).type_0 as libc::c_uint {
        0 => { type_0 = b"LEFTRIGHT\x00" as *const u8 as *const libc::c_char }
        1 => { type_0 = b"TOPBOTTOM\x00" as *const u8 as *const libc::c_char }
        2 => {
            type_0 = b"WINDOWPANE\x00" as *const u8 as *const libc::c_char
        }
        _ => { type_0 = b"UNKNOWN\x00" as *const u8 as *const libc::c_char }
    }
    log_debug(b"%s:%*s%p type %s [parent %p] wp=%p [%u,%u %ux%u]\x00" as
                  *const u8 as *const libc::c_char, hdr, n,
              b" \x00" as *const u8 as *const libc::c_char, lc, type_0,
              (*lc).parent, (*lc).wp, (*lc).xoff, (*lc).yoff, (*lc).sx,
              (*lc).sy);
    match (*lc).type_0 as libc::c_uint {
        0 | 1 => {
            lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
            while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
                layout_print_cell(lcchild, hdr,
                                  n.wrapping_add(1i32 as libc::c_uint));
                lcchild = (*lcchild).entry.tqe_next
            }
        }
        2 | _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_destroy_cell(mut w: *mut window,
                                             mut lc: *mut layout_cell,
                                             mut lcroot:
                                                 *mut *mut layout_cell)
 -> () {
    let mut current_block: u64;
    let mut lcother: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcparent: *mut layout_cell = 0 as *mut layout_cell;
    lcparent = (*lc).parent;
    if lcparent == 0 as *mut libc::c_void as *mut layout_cell {
        layout_free_cell(lc);
        *lcroot = 0 as *mut layout_cell;
        return
    } else {
        if lc == (*(&mut (*lcparent).cells as *mut layout_cells)).tqh_first {
            lcother = (*lc).entry.tqe_next
        } else {
            lcother = *(*((*lc).entry.tqe_prev as *mut layout_cells)).tqh_last
        }
        if (*lcparent).type_0 as libc::c_uint ==
               LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
            layout_resize_adjust(w, lcother, (*lcparent).type_0,
                                 (*lc).sx.wrapping_add(1i32 as libc::c_uint)
                                     as libc::c_int);
        } else {
            layout_resize_adjust(w, lcother, (*lcparent).type_0,
                                 (*lc).sy.wrapping_add(1i32 as libc::c_uint)
                                     as libc::c_int);
        }
        loop  {
            if (*lc).entry.tqe_next !=
                   0 as *mut libc::c_void as *mut layout_cell {
                (*(*lc).entry.tqe_next).entry.tqe_prev = (*lc).entry.tqe_prev
            } else {
                let ref mut fresh3 =
                    (*(&mut (*lcparent).cells as *mut layout_cells)).tqh_last;
                *fresh3 = (*lc).entry.tqe_prev
            }
            *(*lc).entry.tqe_prev = (*lc).entry.tqe_next;
            if !(0 != 0i32) { break ; }
        }
        layout_free_cell(lc);
        lc = (*(&mut (*lcparent).cells as *mut layout_cells)).tqh_first;
        if (*lc).entry.tqe_next == 0 as *mut libc::c_void as *mut layout_cell
           {
            current_block = 14523784380283086299;
        } else { current_block = 17860125682698302841; }
        loop  {
            match current_block {
                17860125682698302841 => { return; }
                _ => {
                    if (*lc).entry.tqe_next !=
                           0 as *mut libc::c_void as *mut layout_cell {
                        (*(*lc).entry.tqe_next).entry.tqe_prev =
                            (*lc).entry.tqe_prev
                    } else {
                        let ref mut fresh4 =
                            (*(&mut (*lcparent).cells as
                                   *mut layout_cells)).tqh_last;
                        *fresh4 = (*lc).entry.tqe_prev
                    }
                    *(*lc).entry.tqe_prev = (*lc).entry.tqe_next;
                    if 0 != 0i32 {
                        current_block = 14523784380283086299;
                        continue ;
                    }
                    (*lc).parent = (*lcparent).parent;
                    if (*lc).parent ==
                           0 as *mut libc::c_void as *mut layout_cell {
                        (*lc).xoff = 0i32 as u_int;
                        (*lc).yoff = 0i32 as u_int;
                        *lcroot = lc
                    } else {
                        loop  {
                            (*lc).entry.tqe_next = (*lcparent).entry.tqe_next;
                            if (*lc).entry.tqe_next !=
                                   0 as *mut libc::c_void as *mut layout_cell
                               {
                                (*(*lc).entry.tqe_next).entry.tqe_prev =
                                    &mut (*lc).entry.tqe_next as
                                        *mut *mut layout_cell
                            } else {
                                let ref mut fresh5 =
                                    (*(&mut (*(*lc).parent).cells as
                                           *mut layout_cells)).tqh_last;
                                *fresh5 =
                                    &mut (*lc).entry.tqe_next as
                                        *mut *mut layout_cell
                            }
                            (*lc).entry.tqe_prev = (*lcparent).entry.tqe_prev;
                            *(*lc).entry.tqe_prev = lc;
                            if !(0 != 0i32) { break ; }
                        }
                    }
                    layout_free_cell(lcparent);
                    current_block = 17860125682698302841;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_resize_adjust(mut w: *mut window,
                                              mut lc: *mut layout_cell,
                                              mut type_0: layout_type,
                                              mut change: libc::c_int) -> () {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    if type_0 as libc::c_uint ==
           LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
        (*lc).sx =
            ((*lc).sx as libc::c_uint).wrapping_add(change as libc::c_uint) as
                u_int as u_int
    } else {
        (*lc).sy =
            ((*lc).sy as libc::c_uint).wrapping_add(change as libc::c_uint) as
                u_int as u_int
    }
    if type_0 as libc::c_uint ==
           LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
        return
    } else if (*lc).type_0 as libc::c_uint != type_0 as libc::c_uint {
        lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
        while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
            layout_resize_adjust(w, lcchild, type_0, change);
            lcchild = (*lcchild).entry.tqe_next
        }
        return
    } else {
        while change != 0i32 {
            lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
            while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
                if change == 0i32 { break ; }
                if change > 0i32 {
                    layout_resize_adjust(w, lcchild, type_0, 1i32);
                    change -= 1
                } else if layout_resize_check(w, lcchild, type_0) >
                              0i32 as libc::c_uint {
                    layout_resize_adjust(w, lcchild, type_0,
                                         1i32.wrapping_neg());
                    change += 1
                }
                lcchild = (*lcchild).entry.tqe_next
            }
        }
        return;
    };
}
unsafe extern "C" fn layout_resize_check(mut w: *mut window,
                                         mut lc: *mut layout_cell,
                                         mut type_0: layout_type) -> u_int {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut available: u_int = 0;
    let mut minimum: u_int = 0;
    if (*lc).type_0 as libc::c_uint ==
           LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
        minimum = 2i32 as u_int;
        if type_0 as libc::c_uint ==
               LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
            available = (*lc).sx
        } else {
            available = (*lc).sy;
            minimum =
                (minimum as
                     libc::c_uint).wrapping_add(layout_need_status(lc,
                                                                   (options_get_number((*w).options,
                                                                                       b"pane-border-status\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const libc::c_char)
                                                                        ==
                                                                        1i32
                                                                            as
                                                                            libc::c_longlong)
                                                                       as
                                                                       libc::c_int)
                                                    as libc::c_uint) as u_int
                    as u_int
        }
        if available > minimum {
            available =
                (available as libc::c_uint).wrapping_sub(minimum) as u_int as
                    u_int
        } else { available = 0i32 as u_int }
    } else if (*lc).type_0 as libc::c_uint == type_0 as libc::c_uint {
        available = 0i32 as u_int;
        lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
        while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
            available =
                (available as
                     libc::c_uint).wrapping_add(layout_resize_check(w,
                                                                    lcchild,
                                                                    type_0))
                    as u_int as u_int;
            lcchild = (*lcchild).entry.tqe_next
        }
    } else {
        minimum =
            (2147483647i32 as
                 libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32);
        lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
        while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
            available = layout_resize_check(w, lcchild, type_0);
            if available < minimum { minimum = available }
            lcchild = (*lcchild).entry.tqe_next
        }
        available = minimum
    }
    return available;
}
unsafe extern "C" fn layout_need_status(mut lc: *mut layout_cell,
                                        mut at_top: libc::c_int)
 -> libc::c_int {
    let mut first_lc: *mut layout_cell = 0 as *mut layout_cell;
    if !(*lc).parent.is_null() {
        if (*(*lc).parent).type_0 as libc::c_uint ==
               LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
            return layout_need_status((*lc).parent, at_top)
        } else {
            if 0 != at_top {
                first_lc =
                    (*(&mut (*(*lc).parent).cells as
                           *mut layout_cells)).tqh_first
            } else {
                first_lc =
                    *(*((*(&mut (*(*lc).parent).cells as
                               *mut layout_cells)).tqh_last as
                            *mut layout_cells)).tqh_last
            }
            if lc == first_lc {
                return layout_need_status((*lc).parent, at_top)
            } else { return 0i32 }
        }
    } else { return 1i32 };
}
#[no_mangle]
pub unsafe extern "C" fn layout_set_size(mut lc: *mut layout_cell,
                                         mut sx: u_int, mut sy: u_int,
                                         mut xoff: u_int, mut yoff: u_int)
 -> () {
    (*lc).sx = sx;
    (*lc).sy = sy;
    (*lc).xoff = xoff;
    (*lc).yoff = yoff;
}
#[no_mangle]
pub unsafe extern "C" fn layout_make_leaf(mut lc: *mut layout_cell,
                                          mut wp: *mut window_pane) -> () {
    (*lc).type_0 = LAYOUT_WINDOWPANE;
    loop  {
        let ref mut fresh6 =
            (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
        *fresh6 = 0 as *mut layout_cell;
        let ref mut fresh7 =
            (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
        *fresh7 =
            &mut (*(&mut (*lc).cells as *mut layout_cells)).tqh_first as
                *mut *mut layout_cell;
        if !(0 != 0i32) { break ; }
    }
    (*wp).layout_cell = lc;
    (*lc).wp = wp;
}
#[no_mangle]
pub unsafe extern "C" fn layout_make_node(mut lc: *mut layout_cell,
                                          mut type_0: layout_type) -> () {
    if type_0 as libc::c_uint ==
           LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
        fatalx(b"bad layout type\x00" as *const u8 as *const libc::c_char);
    } else {
        (*lc).type_0 = type_0;
        loop  {
            let ref mut fresh8 =
                (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
            *fresh8 = 0 as *mut layout_cell;
            let ref mut fresh9 =
                (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
            *fresh9 =
                &mut (*(&mut (*lc).cells as *mut layout_cells)).tqh_first as
                    *mut *mut layout_cell;
            if !(0 != 0i32) { break ; }
        }
        if (*lc).wp != 0 as *mut libc::c_void as *mut window_pane {
            (*(*lc).wp).layout_cell = 0 as *mut layout_cell
        }
        (*lc).wp = 0 as *mut window_pane;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_fix_offsets(mut lc: *mut layout_cell) -> () {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut xoff: u_int = 0;
    let mut yoff: u_int = 0;
    if (*lc).type_0 as libc::c_uint ==
           LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
        xoff = (*lc).xoff;
        lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
        while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
            (*lcchild).xoff = xoff;
            (*lcchild).yoff = (*lc).yoff;
            if (*lcchild).type_0 as libc::c_uint !=
                   LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
                layout_fix_offsets(lcchild);
            }
            xoff =
                (xoff as
                     libc::c_uint).wrapping_add((*lcchild).sx.wrapping_add(1i32
                                                                               as
                                                                               libc::c_uint))
                    as u_int as u_int;
            lcchild = (*lcchild).entry.tqe_next
        }
    } else {
        yoff = (*lc).yoff;
        lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
        while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
            (*lcchild).xoff = (*lc).xoff;
            (*lcchild).yoff = yoff;
            if (*lcchild).type_0 as libc::c_uint !=
                   LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
                layout_fix_offsets(lcchild);
            }
            yoff =
                (yoff as
                     libc::c_uint).wrapping_add((*lcchild).sy.wrapping_add(1i32
                                                                               as
                                                                               libc::c_uint))
                    as u_int as u_int;
            lcchild = (*lcchild).entry.tqe_next
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_fix_panes(mut w: *mut window, mut wsx: u_int,
                                          mut wsy: u_int) -> () {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut shift: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut at_top: libc::c_int = 0;
    status =
        options_get_number((*w).options,
                           b"pane-border-status\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    at_top = (status == 1i32) as libc::c_int;
    wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
    while wp != 0 as *mut libc::c_void as *mut window_pane {
        lc = (*wp).layout_cell;
        if !(lc == 0 as *mut libc::c_void as *mut layout_cell) {
            if status != 0i32 {
                shift = layout_need_status(lc, at_top)
            } else { shift = 0i32 }
            (*wp).xoff = (*lc).xoff;
            (*wp).yoff = (*lc).yoff;
            if 0 != shift && 0 != at_top {
                (*wp).yoff =
                    ((*wp).yoff as
                         libc::c_uint).wrapping_add(1i32 as libc::c_uint) as
                        u_int as u_int
            }
            if (*lc).xoff >= wsx || (*lc).xoff.wrapping_add((*lc).sx) < wsx {
                sx = (*lc).sx
            } else {
                sx = wsx.wrapping_sub((*lc).xoff);
                if sx < 1i32 as libc::c_uint { sx = (*lc).sx }
            }
            if (*lc).yoff >= wsy || (*lc).yoff.wrapping_add((*lc).sy) < wsy {
                sy = (*lc).sy
            } else {
                sy = wsy.wrapping_sub((*lc).yoff);
                if sy < 2i32 as libc::c_uint { sy = (*lc).sy }
            }
            if 0 != shift {
                sy =
                    (sy as libc::c_uint).wrapping_sub(1i32 as libc::c_uint) as
                        u_int as u_int
            }
            window_pane_resize(wp, sx, sy);
        }
        wp = (*wp).entry.tqe_next
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_init(mut w: *mut window,
                                     mut wp: *mut window_pane) -> () {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
    lc = (*w).layout_root;
    layout_set_size(lc, (*w).sx, (*w).sy, 0i32 as u_int, 0i32 as u_int);
    layout_make_leaf(lc, wp);
    layout_fix_panes(w, (*w).sx, (*w).sy);
}
#[no_mangle]
pub unsafe extern "C" fn layout_free(mut w: *mut window) -> () {
    layout_free_cell((*w).layout_root);
}
#[no_mangle]
pub unsafe extern "C" fn layout_resize(mut w: *mut window, mut sx: u_int,
                                       mut sy: u_int) -> () {
    let mut lc: *mut layout_cell = (*w).layout_root;
    let mut xlimit: libc::c_int = 0;
    let mut ylimit: libc::c_int = 0;
    let mut xchange: libc::c_int = 0;
    let mut ychange: libc::c_int = 0;
    xchange = sx.wrapping_sub((*w).sx) as libc::c_int;
    xlimit = layout_resize_check(w, lc, LAYOUT_LEFTRIGHT) as libc::c_int;
    if xchange < 0i32 && xchange < xlimit.wrapping_neg() {
        xchange = xlimit.wrapping_neg()
    }
    if xlimit == 0i32 {
        if sx <= (*lc).sx {
            xchange = 0i32
        } else { xchange = sx.wrapping_sub((*lc).sx) as libc::c_int }
    }
    if xchange != 0i32 {
        layout_resize_adjust(w, lc, LAYOUT_LEFTRIGHT, xchange);
    }
    ychange = sy.wrapping_sub((*w).sy) as libc::c_int;
    ylimit = layout_resize_check(w, lc, LAYOUT_TOPBOTTOM) as libc::c_int;
    if ychange < 0i32 && ychange < ylimit.wrapping_neg() {
        ychange = ylimit.wrapping_neg()
    }
    if ylimit == 0i32 {
        if sy <= (*lc).sy {
            ychange = 0i32
        } else { ychange = sy.wrapping_sub((*lc).sy) as libc::c_int }
    }
    if ychange != 0i32 {
        layout_resize_adjust(w, lc, LAYOUT_TOPBOTTOM, ychange);
    }
    layout_fix_offsets(lc);
    layout_fix_panes(w, sx, sy);
}
#[no_mangle]
pub unsafe extern "C" fn layout_resize_pane(mut wp: *mut window_pane,
                                            mut type_0: layout_type,
                                            mut change: libc::c_int,
                                            mut opposite: libc::c_int) -> () {
    let mut w: *mut window = (*wp).window;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcparent: *mut layout_cell = 0 as *mut layout_cell;
    let mut needed: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    lc = (*wp).layout_cell;
    lcparent = (*lc).parent;
    while lcparent != 0 as *mut libc::c_void as *mut layout_cell &&
              (*lcparent).type_0 as libc::c_uint != type_0 as libc::c_uint {
        lc = lcparent;
        lcparent = (*lc).parent
    }
    if lcparent == 0 as *mut libc::c_void as *mut layout_cell {
        return
    } else {
        if lc ==
               *(*((*(&mut (*lcparent).cells as *mut layout_cells)).tqh_last
                       as *mut layout_cells)).tqh_last {
            lc = *(*((*lc).entry.tqe_prev as *mut layout_cells)).tqh_last
        }
        needed = change;
        while needed != 0i32 {
            if change > 0i32 {
                size =
                    layout_resize_pane_grow(w, lc, type_0, needed, opposite);
                needed -= size
            } else {
                size = layout_resize_pane_shrink(w, lc, type_0, needed);
                needed += size
            }
            if size == 0i32 { break ; }
        }
        layout_fix_offsets((*(*wp).window).layout_root);
        layout_fix_panes((*wp).window, (*(*wp).window).sx,
                         (*(*wp).window).sy);
        notify_window(b"window-layout-changed\x00" as *const u8 as
                          *const libc::c_char, (*wp).window);
        return;
    };
}
unsafe extern "C" fn layout_resize_pane_shrink(mut w: *mut window,
                                               mut lc: *mut layout_cell,
                                               mut type_0: layout_type,
                                               mut needed: libc::c_int)
 -> libc::c_int {
    let mut lcadd: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcremove: *mut layout_cell = 0 as *mut layout_cell;
    let mut size: u_int = 0;
    lcremove = lc;
    loop  {
        size = layout_resize_check(w, lcremove, type_0);
        if size != 0i32 as libc::c_uint { break ; }
        lcremove =
            *(*((*lcremove).entry.tqe_prev as *mut layout_cells)).tqh_last;
        if !(lcremove != 0 as *mut libc::c_void as *mut layout_cell) {
            break ;
        }
    }
    if lcremove == 0 as *mut libc::c_void as *mut layout_cell {
        return 0i32
    } else {
        lcadd = (*lc).entry.tqe_next;
        if lcadd == 0 as *mut libc::c_void as *mut layout_cell {
            return 0i32
        } else {
            if size > needed.wrapping_neg() as u_int {
                size = needed.wrapping_neg() as u_int
            }
            layout_resize_adjust(w, lcadd, type_0, size as libc::c_int);
            layout_resize_adjust(w, lcremove, type_0,
                                 size.wrapping_neg() as libc::c_int);
            return size as libc::c_int
        }
    };
}
unsafe extern "C" fn layout_resize_pane_grow(mut w: *mut window,
                                             mut lc: *mut layout_cell,
                                             mut type_0: layout_type,
                                             mut needed: libc::c_int,
                                             mut opposite: libc::c_int)
 -> libc::c_int {
    let mut lcadd: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcremove: *mut layout_cell = 0 as *mut layout_cell;
    let mut size: u_int = 0i32 as u_int;
    lcadd = lc;
    lcremove = (*lc).entry.tqe_next;
    while lcremove != 0 as *mut libc::c_void as *mut layout_cell {
        size = layout_resize_check(w, lcremove, type_0);
        if size > 0i32 as libc::c_uint { break ; }
        lcremove = (*lcremove).entry.tqe_next
    }
    if 0 != opposite && lcremove == 0 as *mut libc::c_void as *mut layout_cell
       {
        lcremove = *(*((*lc).entry.tqe_prev as *mut layout_cells)).tqh_last;
        while lcremove != 0 as *mut libc::c_void as *mut layout_cell {
            size = layout_resize_check(w, lcremove, type_0);
            if size > 0i32 as libc::c_uint { break ; }
            lcremove =
                *(*((*lcremove).entry.tqe_prev as *mut layout_cells)).tqh_last
        }
    }
    if lcremove == 0 as *mut libc::c_void as *mut layout_cell {
        return 0i32
    } else {
        if size > needed as u_int { size = needed as u_int }
        layout_resize_adjust(w, lcadd, type_0, size as libc::c_int);
        layout_resize_adjust(w, lcremove, type_0,
                             size.wrapping_neg() as libc::c_int);
        return size as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_resize_pane_to(mut wp: *mut window_pane,
                                               mut type_0: layout_type,
                                               mut new_size: u_int) -> () {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcparent: *mut layout_cell = 0 as *mut layout_cell;
    let mut change: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    lc = (*wp).layout_cell;
    lcparent = (*lc).parent;
    while lcparent != 0 as *mut libc::c_void as *mut layout_cell &&
              (*lcparent).type_0 as libc::c_uint != type_0 as libc::c_uint {
        lc = lcparent;
        lcparent = (*lc).parent
    }
    if lcparent == 0 as *mut libc::c_void as *mut layout_cell {
        return
    } else {
        if type_0 as libc::c_uint ==
               LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
            size = (*lc).sx as libc::c_int
        } else { size = (*lc).sy as libc::c_int }
        if lc ==
               *(*((*(&mut (*lcparent).cells as *mut layout_cells)).tqh_last
                       as *mut layout_cells)).tqh_last {
            change =
                (size as libc::c_uint).wrapping_sub(new_size) as libc::c_int
        } else {
            change =
                new_size.wrapping_sub(size as libc::c_uint) as libc::c_int
        }
        layout_resize_pane(wp, type_0, change, 1i32);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_assign_pane(mut lc: *mut layout_cell,
                                            mut wp: *mut window_pane) -> () {
    layout_make_leaf(lc, wp);
    layout_fix_panes((*wp).window, (*(*wp).window).sx, (*(*wp).window).sy);
}
#[no_mangle]
pub unsafe extern "C" fn layout_split_pane(mut wp: *mut window_pane,
                                           mut type_0: layout_type,
                                           mut size: libc::c_int,
                                           mut insert_before: libc::c_int,
                                           mut full_size: libc::c_int)
 -> *mut layout_cell {
    let mut current_block: u64;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcparent: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcnew: *mut layout_cell = 0 as *mut layout_cell;
    let mut lc1: *mut layout_cell = 0 as *mut layout_cell;
    let mut lc2: *mut layout_cell = 0 as *mut layout_cell;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut xoff: u_int = 0;
    let mut yoff: u_int = 0;
    let mut size1: u_int = 0;
    let mut size2: u_int = 0;
    let mut new_size: u_int = 0;
    let mut saved_size: u_int = 0;
    let mut resize_first: u_int = 0i32 as u_int;
    if 0 != full_size {
        lc = (*(*wp).window).layout_root
    } else { lc = (*wp).layout_cell }
    sx = (*lc).sx;
    sy = (*lc).sy;
    xoff = (*lc).xoff;
    yoff = (*lc).yoff;
    match type_0 as libc::c_uint {
        0 => {
            if sx < (2i32 * 2i32 + 1i32) as libc::c_uint {
                return 0 as *mut layout_cell
            }
        }
        1 => {
            if sy < (2i32 * 2i32 + 1i32) as libc::c_uint {
                return 0 as *mut layout_cell
            }
        }
        _ => {
            fatalx(b"bad layout type\x00" as *const u8 as
                       *const libc::c_char);
        }
    }
    if type_0 as libc::c_uint ==
           LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
        saved_size = sx
    } else { saved_size = sy }
    if size < 0i32 {
        size2 =
            saved_size.wrapping_add(1i32 as
                                        libc::c_uint).wrapping_div(2i32 as
                                                                       libc::c_uint).wrapping_sub(1i32
                                                                                                      as
                                                                                                      libc::c_uint)
    } else if 0 != insert_before {
        size2 =
            saved_size.wrapping_sub(size as
                                        libc::c_uint).wrapping_sub(1i32 as
                                                                       libc::c_uint)
    } else { size2 = size as u_int }
    if size2 < 2i32 as libc::c_uint {
        size2 = 2i32 as u_int
    } else if size2 > saved_size.wrapping_sub(2i32 as libc::c_uint) {
        size2 = saved_size.wrapping_sub(2i32 as libc::c_uint)
    }
    size1 = saved_size.wrapping_sub(1i32 as libc::c_uint).wrapping_sub(size2);
    if 0 != insert_before { new_size = size2 } else { new_size = size1 }
    if 0 != full_size &&
           0 ==
               layout_set_size_check((*wp).window, lc, type_0,
                                     new_size as libc::c_int) {
        return 0 as *mut layout_cell
    } else {
        if (*lc).parent != 0 as *mut libc::c_void as *mut layout_cell &&
               (*(*lc).parent).type_0 as libc::c_uint ==
                   type_0 as libc::c_uint {
            lcparent = (*lc).parent;
            lcnew = layout_create_cell(lcparent);
            if 0 != insert_before {
                current_block = 2979737022853876585;
            } else { current_block = 10048703153582371463; }
            loop  {
                match current_block {
                    2979737022853876585 => {
                        (*lcnew).entry.tqe_prev = (*lc).entry.tqe_prev;
                        (*lcnew).entry.tqe_next = lc;
                        *(*lc).entry.tqe_prev = lcnew;
                        (*lc).entry.tqe_prev =
                            &mut (*lcnew).entry.tqe_next as
                                *mut *mut layout_cell;
                        if 0 != 0i32 {
                            current_block = 2979737022853876585;
                        } else { break ; }
                    }
                    _ => {
                        (*lcnew).entry.tqe_next = (*lc).entry.tqe_next;
                        if (*lcnew).entry.tqe_next !=
                               0 as *mut libc::c_void as *mut layout_cell {
                            (*(*lcnew).entry.tqe_next).entry.tqe_prev =
                                &mut (*lcnew).entry.tqe_next as
                                    *mut *mut layout_cell
                        } else {
                            let ref mut fresh10 =
                                (*(&mut (*lcparent).cells as
                                       *mut layout_cells)).tqh_last;
                            *fresh10 =
                                &mut (*lcnew).entry.tqe_next as
                                    *mut *mut layout_cell
                        }
                        (*lc).entry.tqe_next = lcnew;
                        (*lcnew).entry.tqe_prev =
                            &mut (*lc).entry.tqe_next as
                                *mut *mut layout_cell;
                        if 0 != 0i32 {
                            current_block = 10048703153582371463;
                        } else { break ; }
                    }
                }
            }
        } else if 0 != full_size &&
                      (*lc).parent ==
                          0 as *mut libc::c_void as *mut layout_cell &&
                      (*lc).type_0 as libc::c_uint == type_0 as libc::c_uint {
            if (*lc).type_0 as libc::c_uint ==
                   LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
                (*lc).sx = new_size;
                layout_resize_child_cells((*wp).window, lc);
                (*lc).sx = saved_size
            } else if (*lc).type_0 as libc::c_uint ==
                          LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint {
                (*lc).sy = new_size;
                layout_resize_child_cells((*wp).window, lc);
                (*lc).sy = saved_size
            }
            resize_first = 1i32 as u_int;
            lcnew = layout_create_cell(lc);
            size =
                saved_size.wrapping_sub(1i32 as
                                            libc::c_uint).wrapping_sub(new_size)
                    as libc::c_int;
            if (*lc).type_0 as libc::c_uint ==
                   LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
                layout_set_size(lcnew, size as u_int, sy, 0i32 as u_int,
                                0i32 as u_int);
            } else if (*lc).type_0 as libc::c_uint ==
                          LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint {
                layout_set_size(lcnew, sx, size as u_int, 0i32 as u_int,
                                0i32 as u_int);
            }
            if 0 != insert_before {
                current_block = 11298138898191919651;
            } else { current_block = 18386322304582297246; }
            loop  {
                match current_block {
                    18386322304582297246 => {
                        (*lcnew).entry.tqe_next = 0 as *mut layout_cell;
                        (*lcnew).entry.tqe_prev =
                            (*(&mut (*lc).cells as
                                   *mut layout_cells)).tqh_last;
                        let ref mut fresh14 =
                            *(*(&mut (*lc).cells as
                                    *mut layout_cells)).tqh_last;
                        *fresh14 = lcnew;
                        let ref mut fresh15 =
                            (*(&mut (*lc).cells as
                                   *mut layout_cells)).tqh_last;
                        *fresh15 =
                            &mut (*lcnew).entry.tqe_next as
                                *mut *mut layout_cell;
                        if 0 != 0i32 {
                            current_block = 18386322304582297246;
                        } else { break ; }
                    }
                    _ => {
                        (*lcnew).entry.tqe_next =
                            (*(&mut (*lc).cells as
                                   *mut layout_cells)).tqh_first;
                        if (*lcnew).entry.tqe_next !=
                               0 as *mut libc::c_void as *mut layout_cell {
                            let ref mut fresh11 =
                                (*(*(&mut (*lc).cells as
                                         *mut layout_cells)).tqh_first).entry.tqe_prev;
                            *fresh11 =
                                &mut (*lcnew).entry.tqe_next as
                                    *mut *mut layout_cell
                        } else {
                            let ref mut fresh12 =
                                (*(&mut (*lc).cells as
                                       *mut layout_cells)).tqh_last;
                            *fresh12 =
                                &mut (*lcnew).entry.tqe_next as
                                    *mut *mut layout_cell
                        }
                        let ref mut fresh13 =
                            (*(&mut (*lc).cells as
                                   *mut layout_cells)).tqh_first;
                        *fresh13 = lcnew;
                        (*lcnew).entry.tqe_prev =
                            &mut (*(&mut (*lc).cells as
                                        *mut layout_cells)).tqh_first as
                                *mut *mut layout_cell;
                        if 0 != 0i32 {
                            current_block = 11298138898191919651;
                        } else { break ; }
                    }
                }
            }
        } else {
            lcparent = layout_create_cell((*lc).parent);
            layout_make_node(lcparent, type_0);
            layout_set_size(lcparent, sx, sy, xoff, yoff);
            if (*lc).parent == 0 as *mut libc::c_void as *mut layout_cell {
                (*(*wp).window).layout_root = lcparent
            } else {
                loop  {
                    (*lcparent).entry.tqe_next = (*lc).entry.tqe_next;
                    if (*lcparent).entry.tqe_next !=
                           0 as *mut libc::c_void as *mut layout_cell {
                        (*(*lcparent).entry.tqe_next).entry.tqe_prev =
                            &mut (*lcparent).entry.tqe_next as
                                *mut *mut layout_cell
                    } else {
                        let ref mut fresh16 =
                            (*(&mut (*(*lc).parent).cells as
                                   *mut layout_cells)).tqh_last;
                        *fresh16 =
                            &mut (*lcparent).entry.tqe_next as
                                *mut *mut layout_cell
                    }
                    (*lcparent).entry.tqe_prev = (*lc).entry.tqe_prev;
                    *(*lcparent).entry.tqe_prev = lcparent;
                    if !(0 != 0i32) { break ; }
                }
            }
            (*lc).parent = lcparent;
            loop  {
                (*lc).entry.tqe_next =
                    (*(&mut (*lcparent).cells as
                           *mut layout_cells)).tqh_first;
                if (*lc).entry.tqe_next !=
                       0 as *mut libc::c_void as *mut layout_cell {
                    let ref mut fresh17 =
                        (*(*(&mut (*lcparent).cells as
                                 *mut layout_cells)).tqh_first).entry.tqe_prev;
                    *fresh17 =
                        &mut (*lc).entry.tqe_next as *mut *mut layout_cell
                } else {
                    let ref mut fresh18 =
                        (*(&mut (*lcparent).cells as
                               *mut layout_cells)).tqh_last;
                    *fresh18 =
                        &mut (*lc).entry.tqe_next as *mut *mut layout_cell
                }
                let ref mut fresh19 =
                    (*(&mut (*lcparent).cells as
                           *mut layout_cells)).tqh_first;
                *fresh19 = lc;
                (*lc).entry.tqe_prev =
                    &mut (*(&mut (*lcparent).cells as
                                *mut layout_cells)).tqh_first as
                        *mut *mut layout_cell;
                if !(0 != 0i32) { break ; }
            }
            lcnew = layout_create_cell(lcparent);
            if 0 != insert_before {
                current_block = 7828949454673616476;
            } else { current_block = 14832935472441733737; }
            loop  {
                match current_block {
                    14832935472441733737 => {
                        (*lcnew).entry.tqe_next = 0 as *mut layout_cell;
                        (*lcnew).entry.tqe_prev =
                            (*(&mut (*lcparent).cells as
                                   *mut layout_cells)).tqh_last;
                        let ref mut fresh23 =
                            *(*(&mut (*lcparent).cells as
                                    *mut layout_cells)).tqh_last;
                        *fresh23 = lcnew;
                        let ref mut fresh24 =
                            (*(&mut (*lcparent).cells as
                                   *mut layout_cells)).tqh_last;
                        *fresh24 =
                            &mut (*lcnew).entry.tqe_next as
                                *mut *mut layout_cell;
                        if 0 != 0i32 {
                            current_block = 14832935472441733737;
                        } else { break ; }
                    }
                    _ => {
                        (*lcnew).entry.tqe_next =
                            (*(&mut (*lcparent).cells as
                                   *mut layout_cells)).tqh_first;
                        if (*lcnew).entry.tqe_next !=
                               0 as *mut libc::c_void as *mut layout_cell {
                            let ref mut fresh20 =
                                (*(*(&mut (*lcparent).cells as
                                         *mut layout_cells)).tqh_first).entry.tqe_prev;
                            *fresh20 =
                                &mut (*lcnew).entry.tqe_next as
                                    *mut *mut layout_cell
                        } else {
                            let ref mut fresh21 =
                                (*(&mut (*lcparent).cells as
                                       *mut layout_cells)).tqh_last;
                            *fresh21 =
                                &mut (*lcnew).entry.tqe_next as
                                    *mut *mut layout_cell
                        }
                        let ref mut fresh22 =
                            (*(&mut (*lcparent).cells as
                                   *mut layout_cells)).tqh_first;
                        *fresh22 = lcnew;
                        (*lcnew).entry.tqe_prev =
                            &mut (*(&mut (*lcparent).cells as
                                        *mut layout_cells)).tqh_first as
                                *mut *mut layout_cell;
                        if 0 != 0i32 {
                            current_block = 7828949454673616476;
                        } else { break ; }
                    }
                }
            }
        }
        if 0 != insert_before {
            lc1 = lcnew;
            lc2 = lc
        } else { lc1 = lc; lc2 = lcnew }
        if 0 == resize_first &&
               type_0 as libc::c_uint ==
                   LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
            layout_set_size(lc1, size1, sy, xoff, yoff);
            layout_set_size(lc2, size2, sy,
                            xoff.wrapping_add((*lc1).sx).wrapping_add(1i32 as
                                                                          libc::c_uint),
                            yoff);
        } else if 0 == resize_first &&
                      type_0 as libc::c_uint ==
                          LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint {
            layout_set_size(lc1, sx, size1, xoff, yoff);
            layout_set_size(lc2, sx, size2, xoff,
                            yoff.wrapping_add((*lc1).sy).wrapping_add(1i32 as
                                                                          libc::c_uint));
        }
        if 0 != full_size {
            if 0 == resize_first {
                layout_resize_child_cells((*wp).window, lc);
            }
            layout_fix_offsets((*(*wp).window).layout_root);
        } else { layout_make_leaf(lc, wp); }
        return lcnew
    };
}
unsafe extern "C" fn layout_resize_child_cells(mut w: *mut window,
                                               mut lc: *mut layout_cell)
 -> () {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut previous: u_int = 0;
    let mut available: u_int = 0;
    let mut count: u_int = 0;
    let mut idx: u_int = 0;
    if (*lc).type_0 as libc::c_uint ==
           LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
        return
    } else {
        count = 0i32 as u_int;
        previous = 0i32 as u_int;
        lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
        while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
            count = count.wrapping_add(1);
            if (*lc).type_0 as libc::c_uint ==
                   LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
                previous =
                    (previous as libc::c_uint).wrapping_add((*lcchild).sx) as
                        u_int as u_int
            } else if (*lc).type_0 as libc::c_uint ==
                          LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint {
                previous =
                    (previous as libc::c_uint).wrapping_add((*lcchild).sy) as
                        u_int as u_int
            }
            lcchild = (*lcchild).entry.tqe_next
        }
        previous =
            (previous as
                 libc::c_uint).wrapping_add(count.wrapping_sub(1i32 as
                                                                   libc::c_uint))
                as u_int as u_int;
        available = 0i32 as u_int;
        if (*lc).type_0 as libc::c_uint ==
               LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
            available = (*lc).sx
        } else if (*lc).type_0 as libc::c_uint ==
                      LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint {
            available = (*lc).sy
        }
        idx = 0i32 as u_int;
        lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
        while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
            if (*lc).type_0 as libc::c_uint ==
                   LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint {
                (*lcchild).sx = (*lc).sx;
                (*lcchild).xoff = (*lc).xoff
            } else {
                (*lcchild).sx =
                    layout_new_pane_size(w, previous, lcchild, (*lc).type_0,
                                         (*lc).sx, count.wrapping_sub(idx),
                                         available);
                available =
                    (available as
                         libc::c_uint).wrapping_sub((*lcchild).sx.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint))
                        as u_int as u_int
            }
            if (*lc).type_0 as libc::c_uint ==
                   LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
                (*lcchild).sy = (*lc).sy
            } else {
                (*lcchild).sy =
                    layout_new_pane_size(w, previous, lcchild, (*lc).type_0,
                                         (*lc).sy, count.wrapping_sub(idx),
                                         available);
                available =
                    (available as
                         libc::c_uint).wrapping_sub((*lcchild).sy.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint))
                        as u_int as u_int
            }
            layout_resize_child_cells(w, lcchild);
            idx = idx.wrapping_add(1);
            lcchild = (*lcchild).entry.tqe_next
        }
        return;
    };
}
unsafe extern "C" fn layout_new_pane_size(mut w: *mut window,
                                          mut previous: u_int,
                                          mut lc: *mut layout_cell,
                                          mut type_0: layout_type,
                                          mut size: u_int,
                                          mut count_left: u_int,
                                          mut size_left: u_int) -> u_int {
    let mut new_size: u_int = 0;
    let mut min: u_int = 0;
    let mut max: u_int = 0;
    let mut available: u_int = 0;
    if count_left == 1i32 as libc::c_uint {
        return size_left
    } else {
        available = layout_resize_check(w, lc, type_0);
        min =
            ((2i32 + 1i32) as
                 libc::c_uint).wrapping_mul(count_left.wrapping_sub(1i32 as
                                                                        libc::c_uint));
        if type_0 as libc::c_uint ==
               LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
            if (*lc).sx.wrapping_sub(available) > min {
                min = (*lc).sx.wrapping_sub(available)
            }
            new_size = (*lc).sx.wrapping_mul(size).wrapping_div(previous)
        } else {
            if (*lc).sy.wrapping_sub(available) > min {
                min = (*lc).sy.wrapping_sub(available)
            }
            new_size = (*lc).sy.wrapping_mul(size).wrapping_div(previous)
        }
        max = size_left.wrapping_sub(min);
        if new_size > max { new_size = max }
        if new_size < 2i32 as libc::c_uint { new_size = 2i32 as u_int }
        return new_size
    };
}
unsafe extern "C" fn layout_set_size_check(mut w: *mut window,
                                           mut lc: *mut layout_cell,
                                           mut type_0: layout_type,
                                           mut size: libc::c_int)
 -> libc::c_int {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut new_size: u_int = 0;
    let mut available: u_int = 0;
    let mut previous: u_int = 0;
    let mut count: u_int = 0;
    let mut idx: u_int = 0;
    if (*lc).type_0 as libc::c_uint ==
           LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
        return (size >= 2i32) as libc::c_int
    } else {
        available = size as u_int;
        count = 0i32 as u_int;
        lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
        while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
            count = count.wrapping_add(1);
            lcchild = (*lcchild).entry.tqe_next
        }
        if (*lc).type_0 as libc::c_uint == type_0 as libc::c_uint {
            if type_0 as libc::c_uint ==
                   LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
                previous = (*lc).sx
            } else { previous = (*lc).sy }
            idx = 0i32 as u_int;
            lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
            while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
                new_size =
                    layout_new_pane_size(w, previous, lcchild, type_0,
                                         size as u_int,
                                         count.wrapping_sub(idx), available);
                if new_size > available {
                    return 0i32
                } else {
                    available =
                        (available as
                             libc::c_uint).wrapping_sub(new_size.wrapping_add(1i32
                                                                                  as
                                                                                  libc::c_uint))
                            as u_int as u_int;
                    if 0 ==
                           layout_set_size_check(w, lcchild, type_0,
                                                 new_size as libc::c_int) {
                        return 0i32
                    } else {
                        idx = idx.wrapping_add(1);
                        lcchild = (*lcchild).entry.tqe_next
                    }
                }
            }
        } else {
            lcchild = (*(&mut (*lc).cells as *mut layout_cells)).tqh_first;
            while lcchild != 0 as *mut libc::c_void as *mut layout_cell {
                if !((*lcchild).type_0 as libc::c_uint ==
                         LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint) {
                    if 0 == layout_set_size_check(w, lcchild, type_0, size) {
                        return 0i32
                    }
                }
                lcchild = (*lcchild).entry.tqe_next
            }
        }
        return 1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_close_pane(mut wp: *mut window_pane) -> () {
    let mut w: *mut window = (*wp).window;
    layout_destroy_cell(w, (*wp).layout_cell,
                        &mut (*w).layout_root as *mut *mut layout_cell);
    if (*w).layout_root != 0 as *mut libc::c_void as *mut layout_cell {
        layout_fix_offsets((*w).layout_root);
        layout_fix_panes(w, (*w).sx, (*w).sy);
    }
    notify_window(b"window-layout-changed\x00" as *const u8 as
                      *const libc::c_char, w);
}
#[no_mangle]
pub unsafe extern "C" fn layout_spread_cell(mut w: *mut window,
                                            mut parent: *mut layout_cell)
 -> libc::c_int {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut number: u_int = 0;
    let mut each: u_int = 0;
    let mut size: u_int = 0;
    let mut change: libc::c_int = 0;
    let mut changed: libc::c_int = 0;
    number = 0i32 as u_int;
    lc = (*(&mut (*parent).cells as *mut layout_cells)).tqh_first;
    while lc != 0 as *mut libc::c_void as *mut layout_cell {
        number = number.wrapping_add(1);
        lc = (*lc).entry.tqe_next
    }
    if number <= 1i32 as libc::c_uint {
        return 0i32
    } else {
        if (*parent).type_0 as libc::c_uint ==
               LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
            size = (*parent).sx
        } else if (*parent).type_0 as libc::c_uint ==
                      LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint {
            size = (*parent).sy
        } else { return 0i32 }
        each =
            size.wrapping_sub(number.wrapping_sub(1i32 as
                                                      libc::c_uint)).wrapping_div(number);
        changed = 0i32;
        lc = (*(&mut (*parent).cells as *mut layout_cells)).tqh_first;
        while lc != 0 as *mut libc::c_void as *mut layout_cell {
            if (*lc).entry.tqe_next ==
                   0 as *mut libc::c_void as *mut layout_cell {
                each =
                    size.wrapping_sub(each.wrapping_add(1i32 as
                                                            libc::c_uint).wrapping_mul(number.wrapping_sub(1i32
                                                                                                               as
                                                                                                               libc::c_uint)))
            }
            change = 0i32;
            if (*parent).type_0 as libc::c_uint ==
                   LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
                change =
                    each.wrapping_sub((*lc).sx as libc::c_int as libc::c_uint)
                        as libc::c_int;
                layout_resize_adjust(w, lc, LAYOUT_LEFTRIGHT, change);
            } else if (*parent).type_0 as libc::c_uint ==
                          LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint {
                change =
                    each.wrapping_sub((*lc).sy as libc::c_int as libc::c_uint)
                        as libc::c_int;
                layout_resize_adjust(w, lc, LAYOUT_TOPBOTTOM, change);
            }
            if change != 0i32 { changed = 1i32 }
            lc = (*lc).entry.tqe_next
        }
        return changed
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_spread_out(mut wp: *mut window_pane) -> () {
    let mut parent: *mut layout_cell = 0 as *mut layout_cell;
    let mut w: *mut window = (*wp).window;
    parent = (*(*wp).layout_cell).parent;
    if parent == 0 as *mut libc::c_void as *mut layout_cell {
        return
    } else {
        loop  {
            if 0 != layout_spread_cell(w, parent) {
                layout_fix_offsets(parent);
                layout_fix_panes(w, (*w).sx, (*w).sy);
                break ;
            } else {
                parent = (*parent).parent;
                if !(parent != 0 as *mut libc::c_void as *mut layout_cell) {
                    break ;
                }
            }
        }
        return;
    };
}

