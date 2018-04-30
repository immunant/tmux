extern crate libc;
extern "C" {
    pub type args_entry;
    pub type _IO_FILE_plus;
    pub type format_job_tree;
    pub type tty_code;
    pub type format_tree;
    pub type options;
    pub type input_ctx;
    pub type hooks;
    pub type tmuxproc;
    pub type event_base;
    pub type tmuxpeer;
    pub type environ;
    pub type screen_titles;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn server_redraw_window(_: *mut window) -> ();
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn window_count_panes(_: *mut window) -> u_int;
    #[no_mangle]
    fn layout_create_cell(_: *mut layout_cell) -> *mut layout_cell;
    #[no_mangle]
    fn layout_print_cell(_: *mut layout_cell, _: *const libc::c_char,
                         _: u_int) -> ();
    #[no_mangle]
    fn layout_set_size(_: *mut layout_cell, _: u_int, _: u_int, _: u_int,
                       _: u_int) -> ();
    #[no_mangle]
    fn layout_make_leaf(_: *mut layout_cell, _: *mut window_pane) -> ();
    #[no_mangle]
    fn layout_make_node(_: *mut layout_cell, _: layout_type) -> ();
    #[no_mangle]
    fn layout_fix_offsets(_: *mut layout_cell) -> ();
    #[no_mangle]
    fn layout_fix_panes(_: *mut window, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn layout_resize_adjust(_: *mut window, _: *mut layout_cell,
                            _: layout_type, _: libc::c_int) -> ();
    #[no_mangle]
    fn layout_free(_: *mut window) -> ();
    #[no_mangle]
    fn layout_spread_cell(_: *mut window, _: *mut layout_cell) -> libc::c_int;
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
    pub term_type: unnamed_11,
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
pub struct unnamed {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const JOB_RUNNING: unnamed_4 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
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
    pub entry: unnamed_28,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_32,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub name: *const libc::c_char,
    pub arrange: Option<unsafe extern "C" fn(_: *mut window) -> ()>,
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub type __suseconds_t = libc::c_long;
pub type unnamed_4 = libc::c_uint;
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
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
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_7 {
    offset: u_int,
    data: unnamed_3,
}
pub type tcflag_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_21,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
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
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub type options_table_scope = libc::c_uint;
pub type time_t = __time_t;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
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
    pub gentry: unnamed_18,
    pub entry: unnamed_30,
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
pub type cmd_retval = libc::c_int;
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub ev_io_next: unnamed_17,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_14,
}
pub type unnamed_11 = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type __off_t = libc::c_long;
pub type u_int = __u_int;
pub type unnamed_12 = libc::c_uint;
pub const LINE_SEL_RIGHT_LEFT: unnamed_21 = 2;
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_20,
    pub entry: unnamed_29,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
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
pub struct unnamed_14 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_0,
    pub ev_next: unnamed_16,
    pub ev_timeout_pos: unnamed_27,
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
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_7,
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
pub struct unnamed_15 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
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
pub struct unnamed_16 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
    pub winlinks: unnamed_5,
    pub entry: unnamed_24,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const TTY_VT100: unnamed_11 = 0;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const PROMPT_ENTRY: unnamed_12 = 0;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_19 {
    ev_io: unnamed_10,
    ev_signal: unnamed_37,
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_38,
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
    pub entry: unnamed_8,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub type key_code = libc::c_ulonglong;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_35,
}
pub const PROMPT_COMMAND: unnamed_12 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub type unnamed_21 = libc::c_uint;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
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
    pub message_log: unnamed_36,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_12,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_26,
}
pub type _IO_lock_t = ();
pub type options_table_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_33,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_21 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
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
pub struct winlink {
    pub idx: libc::c_int,
    pub session: *mut session,
    pub window: *mut window,
    pub status_width: size_t,
    pub status_cell: grid_cell,
    pub status_text: *mut libc::c_char,
    pub flags: libc::c_int,
    pub entry: unnamed_34,
    pub wentry: unnamed_39,
    pub sentry: unnamed_6,
}
pub type u_short = __u_short;
pub const JOB_DEAD: unnamed_4 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub type uint32_t = libc::c_uint;
pub const TTY_VT102: unnamed_11 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub type __u_char = libc::c_uchar;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_27 {
    ev_next_with_common_timeout: unnamed_1,
    min_heap_idx: libc::c_int,
}
pub type uint8_t = libc::c_uchar;
pub type pid_t = __pid_t;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub const TTY_UNKNOWN: unnamed_11 = 6;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_31,
}
pub type u_char = __u_char;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const TTY_VT320: unnamed_11 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
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
pub struct unnamed_35 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
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
pub type cmd_find_type = libc::c_uint;
pub const TTY_VT220: unnamed_11 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub type __u_short = libc::c_ushort;
pub const TTY_VT420: unnamed_11 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
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
pub struct unnamed_37 {
    pub ev_signal_next: unnamed_22,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
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
pub type cmdq_type = libc::c_uint;
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
    pub entry: unnamed_13,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_38 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const LINE_SEL_NONE: unnamed_21 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const JOB_CLOSED: unnamed_4 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
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
    pub entry: unnamed_15,
    pub tree_entry: unnamed_23,
}
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
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
pub struct grid_line {
    pub cellused: u_int,
    pub cellsize: u_int,
    pub celldata: *mut grid_cell_entry,
    pub extdsize: u_int,
    pub extddata: *mut grid_cell,
    pub flags: libc::c_int,
}
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_25,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const TTY_VT101: unnamed_11 = 1;
#[no_mangle]
pub unsafe extern "C" fn layout_set_lookup(mut name: *const libc::c_char)
 -> libc::c_int {
    let mut i: u_int = 0;
    let mut matched: libc::c_int = 1i32.wrapping_neg();
    i = 0i32 as u_int;
    loop  {
        if (i as libc::c_ulong) <
               (::std::mem::size_of::<[unnamed_2; 5]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<unnamed_2>()
                                                    as libc::c_ulong) {
            if strncmp(layout_sets[i as usize].name, name, strlen(name)) ==
                   0i32 {
                if matched != 1i32.wrapping_neg() {
                    return 1i32.wrapping_neg()
                } else { matched = i as libc::c_int }
            }
            i = i.wrapping_add(1)
        } else { return matched }
    };
}
static mut layout_sets: [unnamed_2; 5] =
    unsafe {
        [unnamed_2{name:
                       b"even-horizontal\x00" as *const u8 as
                           *const libc::c_char,
                   arrange: Some(layout_set_even_h),},
         unnamed_2{name:
                       b"even-vertical\x00" as *const u8 as
                           *const libc::c_char,
                   arrange: Some(layout_set_even_v),},
         unnamed_2{name:
                       b"main-horizontal\x00" as *const u8 as
                           *const libc::c_char,
                   arrange: Some(layout_set_main_h),},
         unnamed_2{name:
                       b"main-vertical\x00" as *const u8 as
                           *const libc::c_char,
                   arrange: Some(layout_set_main_v),},
         unnamed_2{name: b"tiled\x00" as *const u8 as *const libc::c_char,
                   arrange: Some(layout_set_tiled),}]
    };
unsafe extern "C" fn layout_set_tiled(mut w: *mut window) -> () {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcrow: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut n: u_int = 0;
    let mut width: u_int = 0;
    let mut height: u_int = 0;
    let mut used: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut columns: u_int = 0;
    let mut rows: u_int = 0;
    layout_print_cell((*w).layout_root,
                      (*::std::mem::transmute::<&[u8; 17],
                                                &[libc::c_char; 17]>(b"layout_set_tiled\x00")).as_ptr(),
                      1i32 as u_int);
    n = window_count_panes(w);
    if n <= 1i32 as libc::c_uint {
        return
    } else {
        columns = 1i32 as u_int;
        rows = columns;
        while rows.wrapping_mul(columns) < n {
            rows = rows.wrapping_add(1);
            if !(rows.wrapping_mul(columns) < n) { continue ; }
            columns = columns.wrapping_add(1)
        }
        width =
            (*w).sx.wrapping_sub(columns.wrapping_sub(1i32 as
                                                          libc::c_uint)).wrapping_div(columns);
        if width < 2i32 as libc::c_uint { width = 2i32 as u_int }
        height =
            (*w).sy.wrapping_sub(rows.wrapping_sub(1i32 as
                                                       libc::c_uint)).wrapping_div(rows);
        if height < 2i32 as libc::c_uint { height = 2i32 as u_int }
        layout_free(w);
        (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
        lc = (*w).layout_root;
        layout_set_size(lc,
                        width.wrapping_add(1i32 as
                                               libc::c_uint).wrapping_mul(columns).wrapping_sub(1i32
                                                                                                    as
                                                                                                    libc::c_uint),
                        height.wrapping_add(1i32 as
                                                libc::c_uint).wrapping_mul(rows).wrapping_sub(1i32
                                                                                                  as
                                                                                                  libc::c_uint),
                        0i32 as u_int, 0i32 as u_int);
        layout_make_node(lc, LAYOUT_TOPBOTTOM);
        wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
        j = 0i32 as u_int;
        while j < rows {
            if wp == 0 as *mut libc::c_void as *mut window_pane { break ; }
            lcrow = layout_create_cell(lc);
            layout_set_size(lcrow, (*w).sx, height, 0i32 as u_int,
                            0i32 as u_int);
            loop  {
                (*lcrow).entry.tqe_next = 0 as *mut layout_cell;
                (*lcrow).entry.tqe_prev =
                    (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
                let ref mut fresh0 =
                    *(*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
                *fresh0 = lcrow;
                let ref mut fresh1 =
                    (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
                *fresh1 =
                    &mut (*lcrow).entry.tqe_next as *mut *mut layout_cell;
                if !(0 != 0i32) { break ; }
            }
            if n.wrapping_sub(j.wrapping_mul(columns)) == 1i32 as libc::c_uint
                   || columns == 1i32 as libc::c_uint {
                layout_make_leaf(lcrow, wp);
                wp = (*wp).entry.tqe_next
            } else {
                layout_make_node(lcrow, LAYOUT_LEFTRIGHT);
                i = 0i32 as u_int;
                while i < columns {
                    lcchild = layout_create_cell(lcrow);
                    layout_set_size(lcchild, width, height, 0i32 as u_int,
                                    0i32 as u_int);
                    layout_make_leaf(lcchild, wp);
                    loop  {
                        (*lcchild).entry.tqe_next = 0 as *mut layout_cell;
                        (*lcchild).entry.tqe_prev =
                            (*(&mut (*lcrow).cells as
                                   *mut layout_cells)).tqh_last;
                        let ref mut fresh2 =
                            *(*(&mut (*lcrow).cells as
                                    *mut layout_cells)).tqh_last;
                        *fresh2 = lcchild;
                        let ref mut fresh3 =
                            (*(&mut (*lcrow).cells as
                                   *mut layout_cells)).tqh_last;
                        *fresh3 =
                            &mut (*lcchild).entry.tqe_next as
                                *mut *mut layout_cell;
                        if !(0 != 0i32) { break ; }
                    }
                    wp = (*wp).entry.tqe_next;
                    if wp == 0 as *mut libc::c_void as *mut window_pane {
                        break ;
                    }
                    i = i.wrapping_add(1)
                }
                if i == columns { i = i.wrapping_sub(1) }
                used =
                    i.wrapping_add(1i32 as
                                       libc::c_uint).wrapping_mul(width.wrapping_add(1i32
                                                                                         as
                                                                                         libc::c_uint)).wrapping_sub(1i32
                                                                                                                         as
                                                                                                                         libc::c_uint);
                if !((*w).sx <= used) {
                    lcchild =
                        *(*((*(&mut (*lcrow).cells as
                                   *mut layout_cells)).tqh_last as
                                *mut layout_cells)).tqh_last;
                    layout_resize_adjust(w, lcchild, LAYOUT_LEFTRIGHT,
                                         (*w).sx.wrapping_sub(used) as
                                             libc::c_int);
                }
            }
            j = j.wrapping_add(1)
        }
        used =
            rows.wrapping_mul(height).wrapping_add(rows).wrapping_sub(1i32 as
                                                                          libc::c_uint);
        if (*w).sy > used {
            lcrow =
                *(*((*(&mut (*lc).cells as *mut layout_cells)).tqh_last as
                        *mut layout_cells)).tqh_last;
            layout_resize_adjust(w, lcrow, LAYOUT_TOPBOTTOM,
                                 (*w).sy.wrapping_sub(used) as libc::c_int);
        }
        layout_fix_offsets(lc);
        layout_fix_panes(w, (*w).sx, (*w).sy);
        layout_print_cell((*w).layout_root,
                          (*::std::mem::transmute::<&[u8; 17],
                                                    &[libc::c_char; 17]>(b"layout_set_tiled\x00")).as_ptr(),
                          1i32 as u_int);
        notify_window(b"window-layout-changed\x00" as *const u8 as
                          *const libc::c_char, w);
        server_redraw_window(w);
        return;
    };
}
unsafe extern "C" fn layout_set_main_v(mut w: *mut window) -> () {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcmain: *mut layout_cell = 0 as *mut layout_cell;
    let mut lccolumn: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut n: u_int = 0;
    let mut mainwidth: u_int = 0;
    let mut otherwidth: u_int = 0;
    let mut width: u_int = 0;
    let mut height: u_int = 0;
    let mut used: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut columns: u_int = 0;
    let mut rows: u_int = 0;
    let mut totalcolumns: u_int = 0;
    layout_print_cell((*w).layout_root,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"layout_set_main_v\x00")).as_ptr(),
                      1i32 as u_int);
    n = window_count_panes(w);
    if n <= 1i32 as libc::c_uint {
        return
    } else {
        n = n.wrapping_sub(1);
        rows =
            (*w).sy.wrapping_add(1i32 as
                                     libc::c_uint).wrapping_div((2i32 + 1i32)
                                                                    as
                                                                    libc::c_uint);
        if rows == 0i32 as libc::c_uint { rows = 1i32 as u_int }
        columns =
            (1i32 as
                 libc::c_uint).wrapping_add(n.wrapping_sub(1i32 as
                                                               libc::c_uint).wrapping_div(rows));
        rows =
            (1i32 as
                 libc::c_uint).wrapping_add(n.wrapping_sub(1i32 as
                                                               libc::c_uint).wrapping_div(columns));
        height =
            (*w).sy.wrapping_sub(n.wrapping_sub(1i32 as
                                                    libc::c_uint)).wrapping_div(rows);
        mainwidth =
            (options_get_number((*w).options,
                                b"main-pane-width\x00" as *const u8 as
                                    *const libc::c_char) +
                 1i32 as libc::c_longlong) as u_int;
        otherwidth =
            (options_get_number((*w).options,
                                b"other-pane-width\x00" as *const u8 as
                                    *const libc::c_char) +
                 1i32 as libc::c_longlong) as u_int;
        if otherwidth > 1i32 as libc::c_uint &&
               (*w).sx.wrapping_sub(otherwidth) > mainwidth {
            mainwidth = (*w).sx.wrapping_sub(otherwidth)
        }
        if mainwidth < (2i32 + 1i32) as libc::c_uint {
            mainwidth = (2i32 + 1i32) as u_int
        }
        totalcolumns =
            columns.wrapping_mul((2i32 + 1i32) as
                                     libc::c_uint).wrapping_sub(1i32 as
                                                                    libc::c_uint);
        if mainwidth.wrapping_add(totalcolumns) > (*w).sx {
            if totalcolumns.wrapping_add(2i32 as
                                             libc::c_uint).wrapping_add(1i32
                                                                            as
                                                                            libc::c_uint)
                   > (*w).sx {
                mainwidth = (2i32 + 2i32) as u_int
            } else { mainwidth = (*w).sx.wrapping_sub(totalcolumns) }
            width = 2i32 as u_int
        } else {
            width =
                (*w).sx.wrapping_sub(mainwidth).wrapping_sub(columns.wrapping_sub(1i32
                                                                                      as
                                                                                      libc::c_uint)).wrapping_div(columns)
        }
        layout_free(w);
        (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
        lc = (*w).layout_root;
        layout_set_size(lc,
                        mainwidth.wrapping_add(columns.wrapping_mul(width.wrapping_add(1i32
                                                                                           as
                                                                                           libc::c_uint))).wrapping_sub(1i32
                                                                                                                            as
                                                                                                                            libc::c_uint),
                        (*w).sy, 0i32 as u_int, 0i32 as u_int);
        layout_make_node(lc, LAYOUT_LEFTRIGHT);
        lcmain = layout_create_cell(lc);
        layout_set_size(lcmain, mainwidth.wrapping_sub(1i32 as libc::c_uint),
                        (*w).sy, 0i32 as u_int, 0i32 as u_int);
        layout_make_leaf(lcmain,
                         (*(&mut (*w).panes as *mut window_panes)).tqh_first);
        loop  {
            (*lcmain).entry.tqe_next = 0 as *mut layout_cell;
            (*lcmain).entry.tqe_prev =
                (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
            let ref mut fresh4 =
                *(*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
            *fresh4 = lcmain;
            let ref mut fresh5 =
                (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
            *fresh5 = &mut (*lcmain).entry.tqe_next as *mut *mut layout_cell;
            if !(0 != 0i32) { break ; }
        }
        wp =
            (*(*(&mut (*w).panes as
                     *mut window_panes)).tqh_first).entry.tqe_next;
        j = 0i32 as u_int;
        while j < columns {
            if wp == 0 as *mut libc::c_void as *mut window_pane { break ; }
            lccolumn = layout_create_cell(lc);
            layout_set_size(lccolumn, width, (*w).sy, 0i32 as u_int,
                            0i32 as u_int);
            loop  {
                (*lccolumn).entry.tqe_next = 0 as *mut layout_cell;
                (*lccolumn).entry.tqe_prev =
                    (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
                let ref mut fresh6 =
                    *(*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
                *fresh6 = lccolumn;
                let ref mut fresh7 =
                    (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
                *fresh7 =
                    &mut (*lccolumn).entry.tqe_next as *mut *mut layout_cell;
                if !(0 != 0i32) { break ; }
            }
            if rows == 1i32 as libc::c_uint {
                layout_make_leaf(lccolumn, wp);
                wp = (*wp).entry.tqe_next
            } else {
                layout_make_node(lccolumn, LAYOUT_TOPBOTTOM);
                i = 0i32 as u_int;
                while i < rows {
                    lcchild = layout_create_cell(lccolumn);
                    layout_set_size(lcchild, width, height, 0i32 as u_int,
                                    0i32 as u_int);
                    layout_make_leaf(lcchild, wp);
                    loop  {
                        (*lcchild).entry.tqe_next = 0 as *mut layout_cell;
                        (*lcchild).entry.tqe_prev =
                            (*(&mut (*lccolumn).cells as
                                   *mut layout_cells)).tqh_last;
                        let ref mut fresh8 =
                            *(*(&mut (*lccolumn).cells as
                                    *mut layout_cells)).tqh_last;
                        *fresh8 = lcchild;
                        let ref mut fresh9 =
                            (*(&mut (*lccolumn).cells as
                                   *mut layout_cells)).tqh_last;
                        *fresh9 =
                            &mut (*lcchild).entry.tqe_next as
                                *mut *mut layout_cell;
                        if !(0 != 0i32) { break ; }
                    }
                    wp = (*wp).entry.tqe_next;
                    if wp == 0 as *mut libc::c_void as *mut window_pane {
                        break ;
                    }
                    i = i.wrapping_add(1)
                }
                if i == rows { i = i.wrapping_sub(1) }
                used =
                    i.wrapping_add(1i32 as
                                       libc::c_uint).wrapping_mul(height.wrapping_add(1i32
                                                                                          as
                                                                                          libc::c_uint)).wrapping_sub(1i32
                                                                                                                          as
                                                                                                                          libc::c_uint);
                if !((*w).sy <= used) {
                    lcchild =
                        *(*((*(&mut (*lccolumn).cells as
                                   *mut layout_cells)).tqh_last as
                                *mut layout_cells)).tqh_last;
                    layout_resize_adjust(w, lcchild, LAYOUT_TOPBOTTOM,
                                         (*w).sy.wrapping_sub(used) as
                                             libc::c_int);
                }
            }
            j = j.wrapping_add(1)
        }
        used =
            mainwidth.wrapping_add(columns.wrapping_mul(width)).wrapping_add(columns).wrapping_sub(1i32
                                                                                                       as
                                                                                                       libc::c_uint);
        if (*w).sx > used {
            lccolumn =
                *(*((*(&mut (*lc).cells as *mut layout_cells)).tqh_last as
                        *mut layout_cells)).tqh_last;
            layout_resize_adjust(w, lccolumn, LAYOUT_LEFTRIGHT,
                                 (*w).sx.wrapping_sub(used) as libc::c_int);
        }
        layout_fix_offsets(lc);
        layout_fix_panes(w, (*w).sx, (*w).sy);
        layout_print_cell((*w).layout_root,
                          (*::std::mem::transmute::<&[u8; 18],
                                                    &[libc::c_char; 18]>(b"layout_set_main_v\x00")).as_ptr(),
                          1i32 as u_int);
        notify_window(b"window-layout-changed\x00" as *const u8 as
                          *const libc::c_char, w);
        server_redraw_window(w);
        return;
    };
}
unsafe extern "C" fn layout_set_main_h(mut w: *mut window) -> () {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcmain: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcrow: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut n: u_int = 0;
    let mut mainheight: u_int = 0;
    let mut otherheight: u_int = 0;
    let mut width: u_int = 0;
    let mut height: u_int = 0;
    let mut used: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut columns: u_int = 0;
    let mut rows: u_int = 0;
    let mut totalrows: u_int = 0;
    layout_print_cell((*w).layout_root,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"layout_set_main_h\x00")).as_ptr(),
                      1i32 as u_int);
    n = window_count_panes(w);
    if n <= 1i32 as libc::c_uint {
        return
    } else {
        n = n.wrapping_sub(1);
        columns =
            (*w).sx.wrapping_add(1i32 as
                                     libc::c_uint).wrapping_div((2i32 + 1i32)
                                                                    as
                                                                    libc::c_uint);
        if columns == 0i32 as libc::c_uint { columns = 1i32 as u_int }
        rows =
            (1i32 as
                 libc::c_uint).wrapping_add(n.wrapping_sub(1i32 as
                                                               libc::c_uint).wrapping_div(columns));
        columns =
            (1i32 as
                 libc::c_uint).wrapping_add(n.wrapping_sub(1i32 as
                                                               libc::c_uint).wrapping_div(rows));
        width =
            (*w).sx.wrapping_sub(n.wrapping_sub(1i32 as
                                                    libc::c_uint)).wrapping_div(columns);
        mainheight =
            (options_get_number((*w).options,
                                b"main-pane-height\x00" as *const u8 as
                                    *const libc::c_char) +
                 1i32 as libc::c_longlong) as u_int;
        otherheight =
            (options_get_number((*w).options,
                                b"other-pane-height\x00" as *const u8 as
                                    *const libc::c_char) +
                 1i32 as libc::c_longlong) as u_int;
        if otherheight > 1i32 as libc::c_uint &&
               (*w).sy.wrapping_sub(otherheight) > mainheight {
            mainheight = (*w).sy.wrapping_sub(otherheight)
        }
        if mainheight < (2i32 + 1i32) as libc::c_uint {
            mainheight = (2i32 + 1i32) as u_int
        }
        totalrows =
            rows.wrapping_mul((2i32 + 1i32) as
                                  libc::c_uint).wrapping_sub(1i32 as
                                                                 libc::c_uint);
        if mainheight.wrapping_add(totalrows) > (*w).sy {
            if totalrows.wrapping_add(2i32 as
                                          libc::c_uint).wrapping_add(1i32 as
                                                                         libc::c_uint)
                   > (*w).sy {
                mainheight = (2i32 + 2i32) as u_int
            } else { mainheight = (*w).sy.wrapping_sub(totalrows) }
            height = 2i32 as u_int
        } else {
            height =
                (*w).sy.wrapping_sub(mainheight).wrapping_sub(rows.wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_uint)).wrapping_div(rows)
        }
        layout_free(w);
        (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
        lc = (*w).layout_root;
        layout_set_size(lc, (*w).sx,
                        mainheight.wrapping_add(rows.wrapping_mul(height.wrapping_add(1i32
                                                                                          as
                                                                                          libc::c_uint))).wrapping_sub(1i32
                                                                                                                           as
                                                                                                                           libc::c_uint),
                        0i32 as u_int, 0i32 as u_int);
        layout_make_node(lc, LAYOUT_TOPBOTTOM);
        lcmain = layout_create_cell(lc);
        layout_set_size(lcmain, (*w).sx,
                        mainheight.wrapping_sub(1i32 as libc::c_uint),
                        0i32 as u_int, 0i32 as u_int);
        layout_make_leaf(lcmain,
                         (*(&mut (*w).panes as *mut window_panes)).tqh_first);
        loop  {
            (*lcmain).entry.tqe_next = 0 as *mut layout_cell;
            (*lcmain).entry.tqe_prev =
                (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
            let ref mut fresh10 =
                *(*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
            *fresh10 = lcmain;
            let ref mut fresh11 =
                (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
            *fresh11 = &mut (*lcmain).entry.tqe_next as *mut *mut layout_cell;
            if !(0 != 0i32) { break ; }
        }
        wp =
            (*(*(&mut (*w).panes as
                     *mut window_panes)).tqh_first).entry.tqe_next;
        j = 0i32 as u_int;
        while j < rows {
            if wp == 0 as *mut libc::c_void as *mut window_pane { break ; }
            lcrow = layout_create_cell(lc);
            layout_set_size(lcrow, (*w).sx, height, 0i32 as u_int,
                            0i32 as u_int);
            loop  {
                (*lcrow).entry.tqe_next = 0 as *mut layout_cell;
                (*lcrow).entry.tqe_prev =
                    (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
                let ref mut fresh12 =
                    *(*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
                *fresh12 = lcrow;
                let ref mut fresh13 =
                    (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
                *fresh13 =
                    &mut (*lcrow).entry.tqe_next as *mut *mut layout_cell;
                if !(0 != 0i32) { break ; }
            }
            if columns == 1i32 as libc::c_uint {
                layout_make_leaf(lcrow, wp);
                wp = (*wp).entry.tqe_next
            } else {
                layout_make_node(lcrow, LAYOUT_LEFTRIGHT);
                i = 0i32 as u_int;
                while i < columns {
                    lcchild = layout_create_cell(lcrow);
                    layout_set_size(lcchild, width, height, 0i32 as u_int,
                                    0i32 as u_int);
                    layout_make_leaf(lcchild, wp);
                    loop  {
                        (*lcchild).entry.tqe_next = 0 as *mut layout_cell;
                        (*lcchild).entry.tqe_prev =
                            (*(&mut (*lcrow).cells as
                                   *mut layout_cells)).tqh_last;
                        let ref mut fresh14 =
                            *(*(&mut (*lcrow).cells as
                                    *mut layout_cells)).tqh_last;
                        *fresh14 = lcchild;
                        let ref mut fresh15 =
                            (*(&mut (*lcrow).cells as
                                   *mut layout_cells)).tqh_last;
                        *fresh15 =
                            &mut (*lcchild).entry.tqe_next as
                                *mut *mut layout_cell;
                        if !(0 != 0i32) { break ; }
                    }
                    wp = (*wp).entry.tqe_next;
                    if wp == 0 as *mut libc::c_void as *mut window_pane {
                        break ;
                    }
                    i = i.wrapping_add(1)
                }
                if i == columns { i = i.wrapping_sub(1) }
                used =
                    i.wrapping_add(1i32 as
                                       libc::c_uint).wrapping_mul(width.wrapping_add(1i32
                                                                                         as
                                                                                         libc::c_uint)).wrapping_sub(1i32
                                                                                                                         as
                                                                                                                         libc::c_uint);
                if !((*w).sx <= used) {
                    lcchild =
                        *(*((*(&mut (*lcrow).cells as
                                   *mut layout_cells)).tqh_last as
                                *mut layout_cells)).tqh_last;
                    layout_resize_adjust(w, lcchild, LAYOUT_LEFTRIGHT,
                                         (*w).sx.wrapping_sub(used) as
                                             libc::c_int);
                }
            }
            j = j.wrapping_add(1)
        }
        used =
            mainheight.wrapping_add(rows.wrapping_mul(height)).wrapping_add(rows).wrapping_sub(1i32
                                                                                                   as
                                                                                                   libc::c_uint);
        if (*w).sy > used {
            lcrow =
                *(*((*(&mut (*lc).cells as *mut layout_cells)).tqh_last as
                        *mut layout_cells)).tqh_last;
            layout_resize_adjust(w, lcrow, LAYOUT_TOPBOTTOM,
                                 (*w).sy.wrapping_sub(used) as libc::c_int);
        }
        layout_fix_offsets(lc);
        layout_fix_panes(w, (*w).sx, (*w).sy);
        layout_print_cell((*w).layout_root,
                          (*::std::mem::transmute::<&[u8; 18],
                                                    &[libc::c_char; 18]>(b"layout_set_main_h\x00")).as_ptr(),
                          1i32 as u_int);
        notify_window(b"window-layout-changed\x00" as *const u8 as
                          *const libc::c_char, w);
        server_redraw_window(w);
        return;
    };
}
unsafe extern "C" fn layout_set_even_v(mut w: *mut window) -> () {
    layout_set_even(w, LAYOUT_TOPBOTTOM);
}
unsafe extern "C" fn layout_set_even(mut w: *mut window,
                                     mut type_0: layout_type) -> () {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcnew: *mut layout_cell = 0 as *mut layout_cell;
    let mut n: u_int = 0;
    layout_print_cell((*w).layout_root,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"layout_set_even\x00")).as_ptr(),
                      1i32 as u_int);
    n = window_count_panes(w);
    if n <= 1i32 as libc::c_uint {
        return
    } else {
        layout_free(w);
        (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
        lc = (*w).layout_root;
        layout_set_size(lc, (*w).sx, (*w).sy, 0i32 as u_int, 0i32 as u_int);
        layout_make_node(lc, type_0);
        wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
        while wp != 0 as *mut libc::c_void as *mut window_pane {
            lcnew = layout_create_cell(lc);
            layout_make_leaf(lcnew, wp);
            (*lcnew).sx = (*w).sx;
            (*lcnew).sy = (*w).sy;
            loop  {
                (*lcnew).entry.tqe_next = 0 as *mut layout_cell;
                (*lcnew).entry.tqe_prev =
                    (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
                let ref mut fresh16 =
                    *(*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
                *fresh16 = lcnew;
                let ref mut fresh17 =
                    (*(&mut (*lc).cells as *mut layout_cells)).tqh_last;
                *fresh17 =
                    &mut (*lcnew).entry.tqe_next as *mut *mut layout_cell;
                if !(0 != 0i32) { break ; }
            }
            wp = (*wp).entry.tqe_next
        }
        layout_spread_cell(w, lc);
        layout_fix_offsets(lc);
        layout_fix_panes(w, (*w).sx, (*w).sy);
        layout_print_cell((*w).layout_root,
                          (*::std::mem::transmute::<&[u8; 16],
                                                    &[libc::c_char; 16]>(b"layout_set_even\x00")).as_ptr(),
                          1i32 as u_int);
        notify_window(b"window-layout-changed\x00" as *const u8 as
                          *const libc::c_char, w);
        server_redraw_window(w);
        return;
    };
}
unsafe extern "C" fn layout_set_even_h(mut w: *mut window) -> () {
    layout_set_even(w, LAYOUT_LEFTRIGHT);
}
#[no_mangle]
pub unsafe extern "C" fn layout_set_select(mut w: *mut window,
                                           mut layout: u_int) -> u_int {
    if layout as libc::c_ulong >
           (::std::mem::size_of::<[unnamed_2; 5]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<unnamed_2>()
                                                as
                                                libc::c_ulong).wrapping_sub(1i32
                                                                                as
                                                                                libc::c_ulong)
       {
        layout =
            (::std::mem::size_of::<[unnamed_2; 5]>() as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<unnamed_2>()
                                                 as
                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_ulong)
                as u_int
    }
    if layout_sets[layout as usize].arrange !=
           ::std::mem::transmute::<*mut libc::c_void,
                                   Option<unsafe extern "C" fn(_: *mut window)
                                              -> ()>>(0 as *mut libc::c_void)
       {
        layout_sets[layout as
                        usize].arrange.expect("non-null function pointer")(w);
    }
    (*w).lastlayout = layout as libc::c_int;
    return layout;
}
#[no_mangle]
pub unsafe extern "C" fn layout_set_next(mut w: *mut window) -> u_int {
    let mut layout: u_int = 0;
    if (*w).lastlayout == 1i32.wrapping_neg() {
        layout = 0i32 as u_int
    } else {
        layout = ((*w).lastlayout + 1i32) as u_int;
        if layout as libc::c_ulong >
               (::std::mem::size_of::<[unnamed_2; 5]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<unnamed_2>()
                                                    as
                                                    libc::c_ulong).wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_ulong)
           {
            layout = 0i32 as u_int
        }
    }
    if layout_sets[layout as usize].arrange !=
           ::std::mem::transmute::<*mut libc::c_void,
                                   Option<unsafe extern "C" fn(_: *mut window)
                                              -> ()>>(0 as *mut libc::c_void)
       {
        layout_sets[layout as
                        usize].arrange.expect("non-null function pointer")(w);
    }
    (*w).lastlayout = layout as libc::c_int;
    return layout;
}
#[no_mangle]
pub unsafe extern "C" fn layout_set_previous(mut w: *mut window) -> u_int {
    let mut layout: u_int = 0;
    if (*w).lastlayout == 1i32.wrapping_neg() {
        layout =
            (::std::mem::size_of::<[unnamed_2; 5]>() as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<unnamed_2>()
                                                 as
                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_ulong)
                as u_int
    } else {
        layout = (*w).lastlayout as u_int;
        if layout == 0i32 as libc::c_uint {
            layout =
                (::std::mem::size_of::<[unnamed_2; 5]>() as
                     libc::c_ulong).wrapping_div(::std::mem::size_of::<unnamed_2>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_ulong)
                    as u_int
        } else { layout = layout.wrapping_sub(1) }
    }
    if layout_sets[layout as usize].arrange !=
           ::std::mem::transmute::<*mut libc::c_void,
                                   Option<unsafe extern "C" fn(_: *mut window)
                                              -> ()>>(0 as *mut libc::c_void)
       {
        layout_sets[layout as
                        usize].arrange.expect("non-null function pointer")(w);
    }
    (*w).lastlayout = layout as libc::c_int;
    return layout;
}

