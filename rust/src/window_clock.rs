extern crate libc;
extern "C" {
    pub type event_base;
    pub type args_entry;
    pub type environ;
    pub type options;
    pub type bufferevent_ops;
    pub type screen_write_collect_line;
    pub type format_job_tree;
    pub type hooks;
    pub type tmuxpeer;
    pub type format_tree;
    pub type input_ctx;
    pub type _IO_FILE_plus;
    pub type tmuxproc;
    pub type tty_code;
    pub type screen_titles;
    pub type evbuffer;
    pub type screen_write_collect_item;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
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
    static mut getdate_err: libc::c_int;
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
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
    #[no_mangle]
    fn event_set(_: *mut event, _: libc::c_int, _: libc::c_short,
                 _:
                     Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_short,
                                                 _: *mut libc::c_void) -> ()>,
                 _: *mut libc::c_void) -> ();
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
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut window_pane,
                          _: *mut screen) -> ();
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx) -> ();
    #[no_mangle]
    fn screen_write_puts(_: *mut screen_write_ctx, _: *const grid_cell,
                         _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn screen_write_putc(_: *mut screen_write_ctx, _: *const grid_cell,
                         _: u_char) -> ();
    #[no_mangle]
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn screen_write_clearscreen(_: *mut screen_write_ctx, _: u_int) -> ();
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn screen_free(_: *mut screen) -> ();
    #[no_mangle]
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int)
     -> ();
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn window_pane_reset_mode(_: *mut window_pane) -> ();
    #[no_mangle]
    static window_buffer_mode: window_mode;
    #[no_mangle]
    static window_tree_mode: window_mode;
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
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
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
    pub entry: unnamed_34,
    pub tree_entry: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub type uint16_t = libc::c_ushort;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
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
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const JOB_DEAD: unnamed_23 = 1;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const TTY_VT100: unnamed_38 = 0;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    ev_io: unnamed_4,
    ev_signal: unnamed_30,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub ev_io_next: unnamed_7,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_6 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub type __time_t = libc::c_long;
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
    pub gentry: unnamed_25,
    pub entry: unnamed_31,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_write_ctx {
    pub wp: *mut window_pane,
    pub s: *mut screen,
    pub item: *mut screen_write_collect_item,
    pub list: *mut screen_write_collect_line,
    pub scrolled: u_int,
    pub bg: u_int,
    pub cells: u_int,
    pub written: u_int,
    pub skipped: u_int,
}
pub type u_int = __u_int;
pub type pid_t = __pid_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_27,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed,
    pub entry: unnamed_5,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type bitstr_t = libc::c_uchar;
pub const TTY_UNKNOWN: unnamed_38 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_6,
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
pub struct unnamed_12 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_14 {
    ev_next_with_common_timeout: unnamed_13,
    min_heap_idx: libc::c_int,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
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
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_33,
}
pub const TTY_VT420: unnamed_38 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_8,
    pub ev_next: unnamed_1,
    pub ev_timeout_pos: unnamed_14,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_2,
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
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_23,
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
    pub entry: unnamed_20,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_21 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_17 {
    offset: u_int,
    data: unnamed_37,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_clock_mode_data {
    pub screen: screen,
    pub tim: time_t,
    pub timer: event,
}
pub type speed_t = libc::c_uint;
pub const TTY_VT320: unnamed_38 = 4;
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
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const PROMPT_COMMAND: unnamed_24 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_16,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub const PROMPT_ENTRY: unnamed_24 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
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
pub struct unnamed_20 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_12,
}
pub type unnamed_21 = libc::c_uint;
pub type u_char = __u_char;
pub const LINE_SEL_NONE: unnamed_21 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_17,
}
pub type __suseconds_t = libc::c_long;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub type unnamed_23 = libc::c_uint;
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
    pub alerts_entry: unnamed_26,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_11,
    pub entry: unnamed_32,
}
pub type unnamed_24 = libc::c_uint;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub type __u_int = libc::c_uint;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
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
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_21 = 1;
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
    pub entry: unnamed_15,
    pub wentry: unnamed_3,
    pub sentry: unnamed_29,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
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
pub struct unnamed_30 {
    pub ev_signal_next: unnamed_10,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub type tcflag_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
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
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const TTY_VT220: unnamed_38 = 3;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const TTY_VT102: unnamed_38 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
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
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
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
    pub term_type: unnamed_38,
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
pub struct unnamed_35 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_18,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
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
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub type cmd_find_type = libc::c_uint;
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
pub type size_t = libc::c_ulong;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_28,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_36,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type cmd_retval = libc::c_int;
pub type options_table_type = libc::c_uint;
pub type cc_t = libc::c_uchar;
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
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub const JOB_RUNNING: unnamed_23 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub type key_code = libc::c_ulonglong;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub type options_table_scope = libc::c_uint;
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
    pub message_log: unnamed_35,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_24,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_22,
}
pub type __u_short = libc::c_ushort;
pub const TTY_VT101: unnamed_38 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const JOB_CLOSED: unnamed_23 = 2;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
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
pub type unnamed_38 = libc::c_uint;
#[no_mangle]
pub static mut window_clock_mode: window_mode =
    unsafe {
        window_mode{name:
                        b"clock-mode\x00" as *const u8 as *const libc::c_char,
                    init: Some(window_clock_init),
                    free: Some(window_clock_free),
                    resize: Some(window_clock_resize),
                    key: Some(window_clock_key),
                    key_table: None,
                    command: None,}
    };
unsafe extern "C" fn window_clock_key(mut wp: *mut window_pane,
                                      mut c: *mut client,
                                      mut sess: *mut session,
                                      mut key: key_code,
                                      mut m: *mut mouse_event) -> () {
    window_pane_reset_mode(wp);
}
unsafe extern "C" fn window_clock_resize(mut wp: *mut window_pane,
                                         mut sx: u_int, mut sy: u_int) -> () {
    let mut data: *mut window_clock_mode_data =
        (*wp).modedata as *mut window_clock_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    screen_resize(s, sx, sy, 0i32);
    window_clock_draw_screen(wp);
}
unsafe extern "C" fn window_clock_draw_screen(mut wp: *mut window_pane)
 -> () {
    let mut current_block: u64;
    let mut data: *mut window_clock_mode_data =
        (*wp).modedata as *mut window_clock_mode_data;
    let mut ctx: screen_write_ctx =
        screen_write_ctx{wp: 0 as *mut window_pane,
                         s: 0 as *mut screen,
                         item: 0 as *mut screen_write_collect_item,
                         list: 0 as *mut screen_write_collect_line,
                         scrolled: 0,
                         bg: 0,
                         cells: 0,
                         written: 0,
                         skipped: 0,};
    let mut colour: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut tim: [libc::c_char; 64] = [0; 64];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: time_t = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut idx: u_int = 0;
    colour =
        options_get_number((*(*wp).window).options,
                           b"clock-mode-colour\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    style =
        options_get_number((*(*wp).window).options,
                           b"clock-mode-style\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    screen_write_start(&mut ctx as *mut screen_write_ctx,
                       0 as *mut window_pane, s);
    t = time(0 as *mut time_t);
    tm = localtime(&mut t as *mut time_t);
    if style == 0i32 {
        strftime(tim.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                 b"%l:%M \x00" as *const u8 as *const libc::c_char,
                 localtime(&mut t as *mut time_t));
        if (*tm).tm_hour >= 12i32 {
            strlcat(tim.as_mut_ptr(),
                    b"PM\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 64]>() as
                        libc::c_ulong);
        } else {
            strlcat(tim.as_mut_ptr(),
                    b"AM\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 64]>() as
                        libc::c_ulong);
        }
    } else {
        strftime(tim.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                 b"%H:%M\x00" as *const u8 as *const libc::c_char, tm);
    }
    screen_write_clearscreen(&mut ctx as *mut screen_write_ctx,
                             8i32 as u_int);
    if ((*(*s).grid).sx as libc::c_ulong) <
           (6i32 as libc::c_ulong).wrapping_mul(strlen(tim.as_mut_ptr())) ||
           (*(*s).grid).sy < 6i32 as libc::c_uint {
        if (*(*s).grid).sx as libc::c_ulong >= strlen(tim.as_mut_ptr()) &&
               (*(*s).grid).sy != 0i32 as libc::c_uint {
            x =
                ((*(*s).grid).sx.wrapping_div(2i32 as libc::c_uint) as
                     libc::c_ulong).wrapping_sub(strlen(tim.as_mut_ptr()).wrapping_div(2i32
                                                                                           as
                                                                                           libc::c_ulong))
                    as u_int;
            y = (*(*s).grid).sy.wrapping_div(2i32 as libc::c_uint);
            screen_write_cursormove(&mut ctx as *mut screen_write_ctx, x, y);
            memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
                   &grid_default_cell as *const grid_cell as
                       *const libc::c_void,
                   ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
            gc.flags = (gc.flags as libc::c_int | 32i32) as u_char;
            gc.fg = colour;
            screen_write_puts(&mut ctx as *mut screen_write_ctx,
                              &mut gc as *mut grid_cell,
                              b"%s\x00" as *const u8 as *const libc::c_char,
                              tim.as_mut_ptr());
        }
        screen_write_stop(&mut ctx as *mut screen_write_ctx);
        return
    } else {
        x =
            ((*(*s).grid).sx.wrapping_div(2i32 as libc::c_uint) as
                 libc::c_ulong).wrapping_sub((3i32 as
                                                  libc::c_ulong).wrapping_mul(strlen(tim.as_mut_ptr())))
                as u_int;
        y =
            (*(*s).grid).sy.wrapping_div(2i32 as
                                             libc::c_uint).wrapping_sub(3i32
                                                                            as
                                                                            libc::c_uint);
        memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
               &grid_default_cell as *const grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        gc.flags = (gc.flags as libc::c_int | 32i32) as u_char;
        gc.bg = colour;
        ptr = tim.as_mut_ptr();
        while *ptr as libc::c_int != 0 {
            if *ptr as libc::c_int >= 48 && *ptr as libc::c_int <= 57 {
                idx = (*ptr as libc::c_int - 48) as u_int;
                current_block = 9606288038608642794;
            } else if *ptr as libc::c_int == 58 {
                idx = 10i32 as u_int;
                current_block = 9606288038608642794;
            } else if *ptr as libc::c_int == 65 {
                idx = 11i32 as u_int;
                current_block = 9606288038608642794;
            } else if *ptr as libc::c_int == 80 {
                idx = 12i32 as u_int;
                current_block = 9606288038608642794;
            } else if *ptr as libc::c_int == 77 {
                idx = 13i32 as u_int;
                current_block = 9606288038608642794;
            } else {
                x =
                    (x as libc::c_uint).wrapping_add(6i32 as libc::c_uint) as
                        u_int as u_int;
                current_block = 11650488183268122163;
            }
            match current_block {
                9606288038608642794 => {
                    j = 0i32 as u_int;
                    while j < 5i32 as libc::c_uint {
                        i = 0i32 as u_int;
                        while i < 5i32 as libc::c_uint {
                            screen_write_cursormove(&mut ctx as
                                                        *mut screen_write_ctx,
                                                    x.wrapping_add(i),
                                                    y.wrapping_add(j));
                            if 0 !=
                                   window_clock_table[idx as
                                                          usize][j as
                                                                     usize][i
                                                                                as
                                                                                usize]
                               {
                                screen_write_putc(&mut ctx as
                                                      *mut screen_write_ctx,
                                                  &mut gc as *mut grid_cell,
                                                  32 as u_char);
                            }
                            i = i.wrapping_add(1)
                        }
                        j = j.wrapping_add(1)
                    }
                    x =
                        (x as libc::c_uint).wrapping_add(6i32 as libc::c_uint)
                            as u_int as u_int
                }
                _ => { }
            }
            ptr = ptr.offset(1isize)
        }
        screen_write_stop(&mut ctx as *mut screen_write_ctx);
        return;
    };
}
#[no_mangle]
pub static mut window_clock_table: [[[libc::c_char; 5]; 5]; 14] =
    unsafe {
        [[[1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char]],
         [[0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char]],
         [[1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 0i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char]],
         [[1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char]],
         [[1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char]],
         [[1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 0i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char]],
         [[1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 0i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char]],
         [[1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char]],
         [[1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char]],
         [[1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char]],
         [[0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 0i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 1i32 as libc::c_char,
           0i32 as libc::c_char, 0i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 0i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 1i32 as libc::c_char,
           0i32 as libc::c_char, 0i32 as libc::c_char],
          [0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 0i32 as libc::c_char]],
         [[1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char]],
         [[1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 0i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 0i32 as libc::c_char]],
         [[1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 1i32 as libc::c_char, 0i32 as libc::c_char,
           1i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 1i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char],
          [1i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char,
           0i32 as libc::c_char, 1i32 as libc::c_char]]]
    };
unsafe extern "C" fn window_clock_free(mut wp: *mut window_pane) -> () {
    let mut data: *mut window_clock_mode_data =
        (*wp).modedata as *mut window_clock_mode_data;
    event_del(&mut (*data).timer as *mut event);
    screen_free(&mut (*data).screen as *mut screen);
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn window_clock_init(mut wp: *mut window_pane,
                                       mut fs: *mut cmd_find_state,
                                       mut args: *mut args) -> *mut screen {
    let mut data: *mut window_clock_mode_data =
        0 as *mut window_clock_mode_data;
    let mut s: *mut screen = 0 as *mut screen;
    let mut tv: timeval = timeval{tv_sec: 1i32 as __time_t, tv_usec: 0,};
    data =
        xmalloc(::std::mem::size_of::<window_clock_mode_data>() as
                    libc::c_ulong) as *mut window_clock_mode_data;
    (*wp).modedata = data as *mut libc::c_void;
    (*data).tim = time(0 as *mut time_t);
    event_set(&mut (*data).timer as *mut event, 1i32.wrapping_neg(),
              0i32 as libc::c_short, Some(window_clock_timer_callback),
              wp as *mut libc::c_void);
    event_add(&mut (*data).timer as *mut event, &mut tv as *mut timeval);
    s = &mut (*data).screen as *mut screen;
    screen_init(s, (*(*(&mut (*wp).base as *mut screen)).grid).sx,
                (*(*(&mut (*wp).base as *mut screen)).grid).sy,
                0i32 as u_int);
    (*s).mode &= !1i32;
    window_clock_draw_screen(wp);
    return s;
}
unsafe extern "C" fn window_clock_timer_callback(mut fd: libc::c_int,
                                                 mut events: libc::c_short,
                                                 mut arg: *mut libc::c_void)
 -> () {
    let mut wp: *mut window_pane = arg as *mut window_pane;
    let mut data: *mut window_clock_mode_data =
        (*wp).modedata as *mut window_clock_mode_data;
    let mut now: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const libc::c_char,};
    let mut then: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const libc::c_char,};
    let mut t: time_t = 0;
    let mut tv: timeval = timeval{tv_sec: 1i32 as __time_t, tv_usec: 0,};
    event_del(&mut (*data).timer as *mut event);
    event_add(&mut (*data).timer as *mut event, &mut tv as *mut timeval);
    t = time(0 as *mut time_t);
    gmtime_r(&mut t as *mut time_t, &mut now as *mut tm);
    gmtime_r(&mut (*data).tim as *mut time_t, &mut then as *mut tm);
    if now.tm_min == then.tm_min {
        return
    } else {
        (*data).tim = t;
        window_clock_draw_screen(wp);
        server_redraw_window((*wp).window);
        return;
    };
}

