extern crate libc;
extern "C" {
    pub type input_ctx;
    pub type format_tree;
    pub type environ;
    pub type evbuffer;
    pub type hooks;
    pub type bufferevent_ops;
    pub type args_entry;
    pub type tmuxpeer;
    pub type options;
    pub type tty_code;
    pub type options_entry;
    pub type screen_titles;
    pub type event_base;
    pub type format_job_tree;
    pub type tmuxproc;
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    static in6addr_any: in6_addr;
    #[no_mangle]
    static in6addr_loopback: in6_addr;
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
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn format_single(_: *mut cmdq_item, _: *const libc::c_char,
                     _: *mut client, _: *mut session, _: *mut winlink,
                     _: *mut window_pane) -> *mut libc::c_char;
    #[no_mangle]
    fn options_empty(_: *mut options, _: *const options_table_entry)
     -> *mut options_entry;
    #[no_mangle]
    fn options_default(_: *mut options, _: *const options_table_entry)
     -> *mut options_entry;
    #[no_mangle]
    fn options_table_entry(_: *mut options_entry)
     -> *const options_table_entry;
    #[no_mangle]
    fn options_get_only(_: *mut options, _: *const libc::c_char)
     -> *mut options_entry;
    #[no_mangle]
    fn options_get(_: *mut options, _: *const libc::c_char)
     -> *mut options_entry;
    #[no_mangle]
    fn options_remove(_: *mut options_entry) -> ();
    #[no_mangle]
    fn options_array_clear(_: *mut options_entry) -> ();
    #[no_mangle]
    fn options_array_get(_: *mut options_entry, _: u_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn options_array_set(_: *mut options_entry, _: u_int,
                         _: *const libc::c_char, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn options_array_size(_: *mut options_entry, _: *mut u_int)
     -> libc::c_int;
    #[no_mangle]
    fn options_array_assign(_: *mut options_entry, _: *const libc::c_char)
     -> ();
    #[no_mangle]
    fn options_match(_: *const libc::c_char, _: *mut libc::c_int,
                     _: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    fn options_set_string(_: *mut options, _: *const libc::c_char,
                          _: libc::c_int, _: *const libc::c_char, ...)
     -> *mut options_entry;
    #[no_mangle]
    fn options_set_number(_: *mut options, _: *const libc::c_char,
                          _: libc::c_longlong) -> *mut options_entry;
    #[no_mangle]
    fn options_set_style(_: *mut options, _: *const libc::c_char,
                         _: libc::c_int, _: *const libc::c_char)
     -> *mut options_entry;
    #[no_mangle]
    fn options_scope_from_flags(_: *mut args, _: libc::c_int,
                                _: *mut cmd_find_state, _: *mut *mut options,
                                _: *mut *mut libc::c_char)
     -> options_table_scope;
    #[no_mangle]
    fn options_style_update_new(_: *mut options, _: *mut options_entry) -> ();
    #[no_mangle]
    fn options_style_update_old(_: *mut options, _: *mut options_entry) -> ();
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn tty_keys_build(_: *mut tty) -> ();
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_find_client(_: *mut cmdq_item, _: *const libc::c_char,
                       _: libc::c_int) -> *mut client;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    fn key_string_lookup_string(_: *const libc::c_char) -> key_code;
    #[no_mangle]
    fn alerts_reset_all() -> ();
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    fn server_client_set_key_table(_: *mut client, _: *const libc::c_char)
     -> ();
    #[no_mangle]
    fn server_redraw_client(_: *mut client) -> ();
    #[no_mangle]
    fn status_timer_start_all() -> ();
    #[no_mangle]
    fn status_update_saved(s: *mut session) -> ();
    #[no_mangle]
    fn recalculate_sizes() -> ();
    #[no_mangle]
    fn colour_fromstring(s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn attributes_fromstring(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn windows_RB_NEXT(_: *mut window) -> *mut window;
    #[no_mangle]
    fn windows_RB_MINMAX(_: *mut windows, _: libc::c_int) -> *mut window;
    #[no_mangle]
    fn layout_fix_panes(_: *mut window, _: u_int, _: u_int) -> ();
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
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    #[no_mangle]
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
}
pub type __time_t = libc::c_long;
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
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub type size_t = libc::c_ulong;
pub type u_int = __u_int;
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_31,
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
pub struct unnamed_0 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
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
pub struct unnamed_3 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_30,
    pub entry: unnamed_33,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
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
pub struct unnamed_4 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
    pub message_log: unnamed_32,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_8,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_11,
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
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_25,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_28,
}
pub const JOB_CLOSED: unnamed_31 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
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
    pub entry: unnamed_6,
    pub tree_entry: unnamed_16,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub type u_char = __u_char;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const TTY_VT101: unnamed_14 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_29,
}
pub type options_table_type = libc::c_uint;
pub type unnamed_7 = libc::c_uint;
pub type unnamed_8 = libc::c_uint;
pub type __uint32_t = libc::c_uint;
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
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_37,
}
pub type __uint64_t = libc::c_ulong;
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_7,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_10 {
    evcb_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: libc::c_short,
                                               _: *mut libc::c_void) -> ()>,
    evcb_selfcb: Option<unsafe extern "C" fn(_: *mut event_callback,
                                             _: *mut libc::c_void) -> ()>,
    evcb_evfinalize: Option<unsafe extern "C" fn(_: *mut event,
                                                 _: *mut libc::c_void) -> ()>,
    evcb_cbfinalize: Option<unsafe extern "C" fn(_: *mut event_callback,
                                                 _: *mut libc::c_void) -> ()>,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub ev_signal_next: unnamed_1,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
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
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const PROMPT_ENTRY: unnamed_8 = 0;
pub type unnamed_14 = libc::c_uint;
pub const TTY_UNKNOWN: unnamed_14 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const LINE_SEL_NONE: unnamed_7 = 0;
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
    pub entry: unnamed_5,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_18,
}
pub type cmd_find_type = libc::c_uint;
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
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
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
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
pub struct unnamed_20 {
    pub ev_io_next: unnamed_36,
    pub ev_timeout: timeval,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub type _IO_lock_t = ();
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_23,
    pub ev_next: unnamed_23,
    pub ev_timeout_pos: unnamed_38,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_26,
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
pub type uint16_t = __uint16_t;
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
    pub gentry: unnamed_21,
    pub entry: unnamed_3,
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
pub struct unnamed_21 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const TTY_VT102: unnamed_14 = 2;
pub type time_t = __time_t;
pub const JOB_DEAD: unnamed_31 = 1;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
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
pub struct unnamed_23 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
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
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const TTY_VT220: unnamed_14 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub type __u_int = libc::c_uint;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_26 {
    ev_io: unnamed_20,
    ev_signal: unnamed_12,
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
pub type layout_type = libc::c_uint;
pub const TTY_VT420: unnamed_14 = 5;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_15,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
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
pub struct unnamed_27 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_7 = 1;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_22,
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
pub union unnamed_28 {
    offset: u_int,
    data: unnamed,
}
pub type cc_t = libc::c_uchar;
pub const PROMPT_COMMAND: unnamed_8 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_29 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_17,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_7 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_callback {
    pub evcb_active_next: unnamed_9,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: unnamed_10,
    pub evcb_arg: *mut libc::c_void,
}
pub type uint32_t = __uint32_t;
pub type unnamed_31 = libc::c_uint;
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
pub struct unnamed_32 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
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
    pub wentry: unnamed_24,
    pub sentry: unnamed_35,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub const TTY_VT320: unnamed_14 = 4;
pub const TTY_VT100: unnamed_14 = 0;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub type __uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub type __off64_t = libc::c_long;
pub type u_short = __u_short;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub type uint8_t = __uint8_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub type pid_t = __pid_t;
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
    pub alerts_entry: unnamed_13,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_19,
    pub entry: unnamed_34,
}
pub type tcflag_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
pub const JOB_RUNNING: unnamed_31 = 0;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_38 {
    ev_next_with_common_timeout: unnamed_23,
    min_heap_idx: libc::c_int,
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
pub type bitstr_t = libc::c_uchar;
unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
    return __x;
}
unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
    return __x;
}
unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
    return __x;
}
unsafe extern "C" fn cmd_set_option_exec(mut self_0: *mut cmd,
                                         mut item: *mut cmdq_item)
 -> cmd_retval {
    let mut current_block: u64;
    let mut args: *mut args = (*self_0).args;
    let mut append: libc::c_int = args_has(args, 97 as u_char);
    let mut fs: *mut cmd_find_state =
        &mut (*item).target as *mut cmd_find_state;
    let mut c: *mut client = 0 as *mut client;
    let mut loop_0: *mut client = 0 as *mut client;
    let mut s: *mut session = (*fs).s;
    let mut wl: *mut winlink = (*fs).wl;
    let mut w: *mut window = 0 as *mut window;
    let mut scope: options_table_scope = OPTIONS_TABLE_NONE;
    let mut oo: *mut options = 0 as *mut options;
    let mut parent: *mut options_entry = 0 as *mut options_entry;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argument: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut target: *const libc::c_char = 0 as *const libc::c_char;
    let mut window: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut already: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut ambiguous: libc::c_int = 0;
    c = cmd_find_client(item, 0 as *const libc::c_char, 1i32);
    argument =
        format_single(item, *(*args).argv.offset(0isize), c, s, wl,
                      0 as *mut window_pane);
    name =
        options_match(argument, &mut idx as *mut libc::c_int,
                      &mut ambiguous as *mut libc::c_int);
    if name == 0 as *mut libc::c_void as *mut libc::c_char {
        if 0 != args_has(args, 113 as u_char) {
            current_block = 14889457129568826429;
        } else {
            if 0 != ambiguous {
                cmdq_error(item,
                           b"ambiguous option: %s\x00" as *const u8 as
                               *const libc::c_char, argument);
            } else {
                cmdq_error(item,
                           b"invalid option: %s\x00" as *const u8 as
                               *const libc::c_char, argument);
            }
            current_block = 12862109772890101110;
        }
    } else {
        if (*args).argc < 2i32 {
            value = 0 as *mut libc::c_char
        } else if 0 != args_has(args, 70 as u_char) {
            value =
                format_single(item, *(*args).argv.offset(1isize), c, s, wl,
                              0 as *mut window_pane)
        } else { value = xstrdup(*(*args).argv.offset(1isize)) }
        if *name as libc::c_int == 64 {
            window =
                ((*self_0).entry ==
                     &cmd_set_window_option_entry as *const cmd_entry) as
                    libc::c_int;
            scope =
                options_scope_from_flags(args, window, fs,
                                         &mut oo as *mut *mut options,
                                         &mut cause as *mut *mut libc::c_char)
        } else if options_get_only(global_options, name) !=
                      0 as *mut libc::c_void as *mut options_entry {
            scope = OPTIONS_TABLE_SERVER
        } else if options_get_only(global_s_options, name) !=
                      0 as *mut libc::c_void as *mut options_entry {
            scope = OPTIONS_TABLE_SESSION
        } else if options_get_only(global_w_options, name) !=
                      0 as *mut libc::c_void as *mut options_entry {
            scope = OPTIONS_TABLE_WINDOW
        } else {
            scope = OPTIONS_TABLE_NONE;
            xasprintf(&mut cause as *mut *mut libc::c_char,
                      b"unknown option: %s\x00" as *const u8 as
                          *const libc::c_char, argument);
        }
        if scope as libc::c_uint ==
               OPTIONS_TABLE_NONE as libc::c_int as libc::c_uint {
            if 0 != args_has(args, 113 as u_char) {
                current_block = 14889457129568826429;
            } else {
                cmdq_error(item,
                           b"%s\x00" as *const u8 as *const libc::c_char,
                           cause);
                free(cause as *mut libc::c_void);
                current_block = 12862109772890101110;
            }
        } else {
            if scope as libc::c_uint ==
                   OPTIONS_TABLE_SERVER as libc::c_int as libc::c_uint {
                oo = global_options;
                current_block = 12599329904712511516;
            } else if scope as libc::c_uint ==
                          OPTIONS_TABLE_SESSION as libc::c_int as libc::c_uint
             {
                if 0 != args_has((*self_0).args, 103 as u_char) {
                    oo = global_s_options;
                    current_block = 12599329904712511516;
                } else if s == 0 as *mut libc::c_void as *mut session {
                    target = args_get(args, 116 as u_char);
                    if target != 0 as *mut libc::c_void as *const libc::c_char
                       {
                        cmdq_error(item,
                                   b"no such session: %s\x00" as *const u8 as
                                       *const libc::c_char, target);
                    } else {
                        cmdq_error(item,
                                   b"no current session\x00" as *const u8 as
                                       *const libc::c_char);
                    }
                    current_block = 12862109772890101110;
                } else {
                    oo = (*s).options;
                    current_block = 12599329904712511516;
                }
            } else if scope as libc::c_uint ==
                          OPTIONS_TABLE_WINDOW as libc::c_int as libc::c_uint
             {
                if 0 != args_has((*self_0).args, 103 as u_char) {
                    oo = global_w_options;
                    current_block = 12599329904712511516;
                } else if wl == 0 as *mut libc::c_void as *mut winlink {
                    target = args_get(args, 116 as u_char);
                    if target != 0 as *mut libc::c_void as *const libc::c_char
                       {
                        cmdq_error(item,
                                   b"no such window: %s\x00" as *const u8 as
                                       *const libc::c_char, target);
                    } else {
                        cmdq_error(item,
                                   b"no current window\x00" as *const u8 as
                                       *const libc::c_char);
                    }
                    current_block = 12862109772890101110;
                } else {
                    oo = (*(*wl).window).options;
                    current_block = 12599329904712511516;
                }
            } else { current_block = 12599329904712511516; }
            match current_block {
                12862109772890101110 => { }
                _ => {
                    o = options_get_only(oo, name);
                    parent = options_get(oo, name);
                    if idx != 1i32.wrapping_neg() {
                        if *name as libc::c_int == 64 ||
                               options_array_size(parent, 0 as *mut u_int) ==
                                   1i32.wrapping_neg() {
                            cmdq_error(item,
                                       b"not an array: %s\x00" as *const u8 as
                                           *const libc::c_char, argument);
                            current_block = 12862109772890101110;
                        } else { current_block = 1109700713171191020; }
                    } else { current_block = 1109700713171191020; }
                    match current_block {
                        12862109772890101110 => { }
                        _ => {
                            if 0 == args_has(args, 117 as u_char) &&
                                   0 != args_has(args, 111 as u_char) {
                                if idx == 1i32.wrapping_neg() {
                                    already =
                                        (o !=
                                             0 as *mut libc::c_void as
                                                 *mut options_entry) as
                                            libc::c_int
                                } else if o ==
                                              0 as *mut libc::c_void as
                                                  *mut options_entry {
                                    already = 0i32
                                } else {
                                    already =
                                        (options_array_get(o, idx as u_int) !=
                                             0 as *mut libc::c_void as
                                                 *const libc::c_char) as
                                            libc::c_int
                                }
                                if 0 != already {
                                    if 0 != args_has(args, 113 as u_char) {
                                        current_block = 14889457129568826429;
                                    } else {
                                        cmdq_error(item,
                                                   b"already set: %s\x00" as
                                                       *const u8 as
                                                       *const libc::c_char,
                                                   argument);
                                        current_block = 12862109772890101110;
                                    }
                                } else {
                                    current_block = 15768484401365413375;
                                }
                            } else { current_block = 15768484401365413375; }
                            match current_block {
                                14889457129568826429 => { }
                                12862109772890101110 => { }
                                _ => {
                                    if 0 != args_has(args, 117 as u_char) {
                                        if o ==
                                               0 as *mut libc::c_void as
                                                   *mut options_entry {
                                            current_block =
                                                14889457129568826429;
                                        } else {
                                            if idx == 1i32.wrapping_neg() {
                                                if *name as libc::c_int == 64
                                                   {
                                                    options_remove(o);
                                                } else if oo == global_options
                                                              ||
                                                              oo ==
                                                                  global_s_options
                                                              ||
                                                              oo ==
                                                                  global_w_options
                                                 {
                                                    options_default(oo,
                                                                    options_table_entry(o));
                                                } else { options_remove(o); }
                                            } else {
                                                options_array_set(o,
                                                                  idx as
                                                                      u_int,
                                                                  0 as
                                                                      *const libc::c_char,
                                                                  0i32);
                                            }
                                            current_block =
                                                5689316957504528238;
                                        }
                                    } else if *name as libc::c_int == 64 {
                                        if value ==
                                               0 as *mut libc::c_void as
                                                   *mut libc::c_char {
                                            cmdq_error(item,
                                                       b"empty value\x00" as
                                                           *const u8 as
                                                           *const libc::c_char);
                                            current_block =
                                                12862109772890101110;
                                        } else {
                                            options_set_string(oo, name,
                                                               append,
                                                               b"%s\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               value);
                                            current_block =
                                                5689316957504528238;
                                        }
                                    } else if idx == 1i32.wrapping_neg() &&
                                                  options_array_size(parent,
                                                                     0 as
                                                                         *mut u_int)
                                                      == 1i32.wrapping_neg() {
                                        error =
                                            cmd_set_option_set(self_0, item,
                                                               oo, parent,
                                                               value);
                                        if error != 0i32 {
                                            current_block =
                                                12862109772890101110;
                                        } else {
                                            current_block =
                                                5689316957504528238;
                                        }
                                    } else if value ==
                                                  0 as *mut libc::c_void as
                                                      *mut libc::c_char {
                                        cmdq_error(item,
                                                   b"empty value\x00" as
                                                       *const u8 as
                                                       *const libc::c_char);
                                        current_block = 12862109772890101110;
                                    } else {
                                        if o ==
                                               0 as *mut libc::c_void as
                                                   *mut options_entry {
                                            o =
                                                options_empty(oo,
                                                              options_table_entry(parent))
                                        }
                                        if idx == 1i32.wrapping_neg() {
                                            if 0 == append {
                                                options_array_clear(o);
                                            }
                                            options_array_assign(o, value);
                                            current_block =
                                                5689316957504528238;
                                        } else if options_array_set(o,
                                                                    idx as
                                                                        u_int,
                                                                    value,
                                                                    append) !=
                                                      0i32 {
                                            cmdq_error(item,
                                                       b"invalid index: %s\x00"
                                                           as *const u8 as
                                                           *const libc::c_char,
                                                       argument);
                                            current_block =
                                                12862109772890101110;
                                        } else {
                                            current_block =
                                                5689316957504528238;
                                        }
                                    }
                                    match current_block {
                                        14889457129568826429 => { }
                                        12862109772890101110 => { }
                                        _ => {
                                            if strcmp(name,
                                                      b"automatic-rename\x00"
                                                          as *const u8 as
                                                          *const libc::c_char)
                                                   == 0i32 {
                                                w =
                                                    windows_RB_MINMAX(&mut windows
                                                                          as
                                                                          *mut windows,
                                                                      1i32.wrapping_neg());
                                                while w !=
                                                          0 as
                                                              *mut libc::c_void
                                                              as *mut window {
                                                    if !((*w).active ==
                                                             0 as
                                                                 *mut libc::c_void
                                                                 as
                                                                 *mut window_pane)
                                                       {
                                                        if 0 !=
                                                               options_get_number((*w).options,
                                                                                  b"automatic-rename\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char)
                                                           {
                                                            (*(*w).active).flags
                                                                |= 128i32
                                                        }
                                                    }
                                                    w = windows_RB_NEXT(w)
                                                }
                                            }
                                            if strcmp(name,
                                                      b"key-table\x00" as
                                                          *const u8 as
                                                          *const libc::c_char)
                                                   == 0i32 {
                                                loop_0 =
                                                    (*(&mut clients as
                                                           *mut clients)).tqh_first;
                                                while loop_0 !=
                                                          0 as
                                                              *mut libc::c_void
                                                              as *mut client {
                                                    server_client_set_key_table(loop_0,
                                                                                0
                                                                                    as
                                                                                    *const libc::c_char);
                                                    loop_0 =
                                                        (*loop_0).entry.tqe_next
                                                }
                                            }
                                            if strcmp(name,
                                                      b"user-keys\x00" as
                                                          *const u8 as
                                                          *const libc::c_char)
                                                   == 0i32 {
                                                loop_0 =
                                                    (*(&mut clients as
                                                           *mut clients)).tqh_first;
                                                while loop_0 !=
                                                          0 as
                                                              *mut libc::c_void
                                                              as *mut client {
                                                    if 0 !=
                                                           (*loop_0).tty.flags
                                                               & 32i32 {
                                                        tty_keys_build(&mut (*loop_0).tty
                                                                           as
                                                                           *mut tty);
                                                    }
                                                    loop_0 =
                                                        (*loop_0).entry.tqe_next
                                                }
                                            }
                                            if strcmp(name,
                                                      b"status\x00" as
                                                          *const u8 as
                                                          *const libc::c_char)
                                                   == 0i32 ||
                                                   strcmp(name,
                                                          b"status-interval\x00"
                                                              as *const u8 as
                                                              *const libc::c_char)
                                                       == 0i32 {
                                                status_timer_start_all();
                                            }
                                            if strcmp(name,
                                                      b"monitor-silence\x00"
                                                          as *const u8 as
                                                          *const libc::c_char)
                                                   == 0i32 {
                                                alerts_reset_all();
                                            }
                                            if strcmp(name,
                                                      b"window-style\x00" as
                                                          *const u8 as
                                                          *const libc::c_char)
                                                   == 0i32 ||
                                                   strcmp(name,
                                                          b"window-active-style\x00"
                                                              as *const u8 as
                                                              *const libc::c_char)
                                                       == 0i32 {
                                                w =
                                                    windows_RB_MINMAX(&mut windows
                                                                          as
                                                                          *mut windows,
                                                                      1i32.wrapping_neg());
                                                while w !=
                                                          0 as
                                                              *mut libc::c_void
                                                              as *mut window {
                                                    (*w).flags |= 32768i32;
                                                    w = windows_RB_NEXT(w)
                                                }
                                            }
                                            if strcmp(name,
                                                      b"pane-border-status\x00"
                                                          as *const u8 as
                                                          *const libc::c_char)
                                                   == 0i32 {
                                                w =
                                                    windows_RB_MINMAX(&mut windows
                                                                          as
                                                                          *mut windows,
                                                                      1i32.wrapping_neg());
                                                while w !=
                                                          0 as
                                                              *mut libc::c_void
                                                              as *mut window {
                                                    layout_fix_panes(w,
                                                                     (*w).sx,
                                                                     (*w).sy);
                                                    w = windows_RB_NEXT(w)
                                                }
                                            }
                                            s =
                                                sessions_RB_MINMAX(&mut sessions
                                                                       as
                                                                       *mut sessions,
                                                                   1i32.wrapping_neg());
                                            while s !=
                                                      0 as *mut libc::c_void
                                                          as *mut session {
                                                status_update_saved(s);
                                                s = sessions_RB_NEXT(s)
                                            }
                                            recalculate_sizes();
                                            loop_0 =
                                                (*(&mut clients as
                                                       *mut clients)).tqh_first;
                                            while loop_0 !=
                                                      0 as *mut libc::c_void
                                                          as *mut client {
                                                if (*loop_0).session !=
                                                       0 as *mut libc::c_void
                                                           as *mut session {
                                                    server_redraw_client(loop_0);
                                                }
                                                loop_0 =
                                                    (*loop_0).entry.tqe_next
                                            }
                                            current_block =
                                                14889457129568826429;
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
    match current_block {
        12862109772890101110 => {
            free(argument as *mut libc::c_void);
            free(value as *mut libc::c_void);
            free(name as *mut libc::c_void);
            return CMD_RETURN_ERROR
        }
        _ => {
            free(argument as *mut libc::c_void);
            free(value as *mut libc::c_void);
            free(name as *mut libc::c_void);
            return CMD_RETURN_NORMAL
        }
    };
}
unsafe extern "C" fn cmd_set_option_set(mut self_0: *mut cmd,
                                        mut item: *mut cmdq_item,
                                        mut oo: *mut options,
                                        mut parent: *mut options_entry,
                                        mut value: *const libc::c_char)
 -> libc::c_int {
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
    let mut args: *mut args = (*self_0).args;
    let mut append: libc::c_int = args_has(args, 97 as u_char);
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut number: libc::c_longlong = 0;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: key_code = 0;
    oe = options_table_entry(parent);
    if value == 0 as *mut libc::c_void as *const libc::c_char &&
           (*oe).type_0 as libc::c_uint !=
               OPTIONS_TABLE_FLAG as libc::c_int as libc::c_uint &&
           (*oe).type_0 as libc::c_uint !=
               OPTIONS_TABLE_CHOICE as libc::c_int as libc::c_uint {
        cmdq_error(item,
                   b"empty value\x00" as *const u8 as *const libc::c_char);
        return 1i32.wrapping_neg()
    } else {
        match (*oe).type_0 as libc::c_uint {
            0 => {
                options_set_string(oo, (*oe).name, append,
                                   b"%s\x00" as *const u8 as
                                       *const libc::c_char, value);
                return 0i32
            }
            1 => {
                number =
                    strtonum(value, (*oe).minimum as libc::c_longlong,
                             (*oe).maximum as libc::c_longlong,
                             &mut errstr as *mut *const libc::c_char);
                if errstr != 0 as *mut libc::c_void as *const libc::c_char {
                    cmdq_error(item,
                               b"value is %s: %s\x00" as *const u8 as
                                   *const libc::c_char, errstr, value);
                    return 1i32.wrapping_neg()
                } else {
                    options_set_number(oo, (*oe).name, number);
                    return 0i32
                }
            }
            2 => {
                key = key_string_lookup_string(value);
                if key == 281466386776064u64 {
                    cmdq_error(item,
                               b"bad key: %s\x00" as *const u8 as
                                   *const libc::c_char, value);
                    return 1i32.wrapping_neg()
                } else {
                    options_set_number(oo, (*oe).name,
                                       key as libc::c_longlong);
                    return 0i32
                }
            }
            3 => {
                number = colour_fromstring(value) as libc::c_longlong;
                if number == 1i32.wrapping_neg() as libc::c_longlong {
                    cmdq_error(item,
                               b"bad colour: %s\x00" as *const u8 as
                                   *const libc::c_char, value);
                    return 1i32.wrapping_neg()
                } else {
                    o = options_set_number(oo, (*oe).name, number);
                    options_style_update_new(oo, o);
                    return 0i32
                }
            }
            4 => {
                number = attributes_fromstring(value) as libc::c_longlong;
                if number == 1i32.wrapping_neg() as libc::c_longlong {
                    cmdq_error(item,
                               b"bad attributes: %s\x00" as *const u8 as
                                   *const libc::c_char, value);
                    return 1i32.wrapping_neg()
                } else {
                    o = options_set_number(oo, (*oe).name, number);
                    options_style_update_new(oo, o);
                    return 0i32
                }
            }
            5 => { return cmd_set_option_flag(item, oe, oo, value) }
            6 => { return cmd_set_option_choice(item, oe, oo, value) }
            7 => {
                o = options_set_style(oo, (*oe).name, append, value);
                if o == 0 as *mut libc::c_void as *mut options_entry {
                    cmdq_error(item,
                               b"bad style: %s\x00" as *const u8 as
                                   *const libc::c_char, value);
                    return 1i32.wrapping_neg()
                } else { options_style_update_old(oo, o); return 0i32 }
            }
            8 | _ => { return 1i32.wrapping_neg() }
        }
    };
}
unsafe extern "C" fn cmd_set_option_choice(mut item: *mut cmdq_item,
                                           mut oe: *const options_table_entry,
                                           mut oo: *mut options,
                                           mut value: *const libc::c_char)
 -> libc::c_int {
    let mut cp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut n: libc::c_int = 0;
    let mut choice: libc::c_int = 1i32.wrapping_neg();
    if value == 0 as *mut libc::c_void as *const libc::c_char {
        choice = options_get_number(oo, (*oe).name) as libc::c_int;
        if choice < 2i32 { choice = (0 == choice) as libc::c_int }
    } else {
        n = 0i32;
        cp = (*oe).choices;
        while *cp != 0 as *mut libc::c_void as *const libc::c_char {
            if strcmp(*cp, value) == 0i32 { choice = n }
            n += 1;
            cp = cp.offset(1isize)
        }
        if choice == 1i32.wrapping_neg() {
            cmdq_error(item,
                       b"unknown value: %s\x00" as *const u8 as
                           *const libc::c_char, value);
            return 1i32.wrapping_neg()
        }
    }
    options_set_number(oo, (*oe).name, choice as libc::c_longlong);
    return 0i32;
}
unsafe extern "C" fn cmd_set_option_flag(mut item: *mut cmdq_item,
                                         mut oe: *const options_table_entry,
                                         mut oo: *mut options,
                                         mut value: *const libc::c_char)
 -> libc::c_int {
    let mut flag: libc::c_int = 0;
    if value == 0 as *mut libc::c_void as *const libc::c_char ||
           *value as libc::c_int == 0 {
        flag = (0 == options_get_number(oo, (*oe).name)) as libc::c_int
    } else if strcmp(value, b"1\x00" as *const u8 as *const libc::c_char) ==
                  0i32 ||
                  strcasecmp(value,
                             b"on\x00" as *const u8 as *const libc::c_char) ==
                      0i32 ||
                  strcasecmp(value,
                             b"yes\x00" as *const u8 as *const libc::c_char)
                      == 0i32 {
        flag = 1i32
    } else if strcmp(value, b"0\x00" as *const u8 as *const libc::c_char) ==
                  0i32 ||
                  strcasecmp(value,
                             b"off\x00" as *const u8 as *const libc::c_char)
                      == 0i32 ||
                  strcasecmp(value,
                             b"no\x00" as *const u8 as *const libc::c_char) ==
                      0i32 {
        flag = 0i32
    } else {
        cmdq_error(item,
                   b"bad value: %s\x00" as *const u8 as *const libc::c_char,
                   value);
        return 1i32.wrapping_neg()
    }
    options_set_number(oo, (*oe).name, flag as libc::c_longlong);
    return 0i32;
}
#[no_mangle]
pub static mut cmd_set_window_option_entry: cmd_entry =
    unsafe {
        cmd_entry{name:
                      b"set-window-option\x00" as *const u8 as
                          *const libc::c_char,
                  alias: b"setw\x00" as *const u8 as *const libc::c_char,
                  args:
                      unnamed_25{template:
                                     b"aFgoqt:u\x00" as *const u8 as
                                         *const libc::c_char,
                                 lower: 1i32,
                                 upper: 2i32,},
                  usage:
                      b"[-aFgoqu] [-t target-window] option [value]\x00" as
                          *const u8 as *const libc::c_char,
                  source:
                      cmd_entry_flag{flag: 0,
                                     type_0: CMD_FIND_PANE,
                                     flags: 0,},
                  target:
                      cmd_entry_flag{flag: 116 as libc::c_char,
                                     type_0: CMD_FIND_WINDOW,
                                     flags: 64i32,},
                  flags: 4i32,
                  exec: Some(cmd_set_option_exec),}
    };
#[no_mangle]
pub static mut cmd_set_option_entry: cmd_entry =
    unsafe {
        cmd_entry{name: b"set-option\x00" as *const u8 as *const libc::c_char,
                  alias: b"set\x00" as *const u8 as *const libc::c_char,
                  args:
                      unnamed_25{template:
                                     b"aFgoqst:uw\x00" as *const u8 as
                                         *const libc::c_char,
                                 lower: 1i32,
                                 upper: 2i32,},
                  usage:
                      b"[-aFgosquw] [-t target-window] option [value]\x00" as
                          *const u8 as *const libc::c_char,
                  source:
                      cmd_entry_flag{flag: 0,
                                     type_0: CMD_FIND_PANE,
                                     flags: 0,},
                  target:
                      cmd_entry_flag{flag: 116 as libc::c_char,
                                     type_0: CMD_FIND_WINDOW,
                                     flags: 64i32,},
                  flags: 4i32,
                  exec: Some(cmd_set_option_exec),}
    };

