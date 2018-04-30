extern crate libc;
extern "C" {
    pub type screen_titles;
    pub type environ;
    pub type bufferevent_ops;
    pub type tty_code;
    pub type event_base;
    pub type evbuffer;
    pub type format_job_tree;
    pub type args_entry;
    pub type _IO_FILE_plus;
    pub type format_tree;
    pub type hooks;
    pub type options;
    pub type input_ctx;
    #[no_mangle]
    fn uname(__name: *mut utsname) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut program_invocation_name: *mut libc::c_char;
    #[no_mangle]
    static mut program_invocation_short_name: *mut libc::c_char;
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
    fn event_get_version() -> *const libc::c_char;
    #[no_mangle]
    fn event_loop(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn event_get_method() -> *const libc::c_char;
    #[no_mangle]
    fn event_set(_: *mut event, _: libc::c_int, _: libc::c_short,
                 _:
                     Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_short,
                                                 _: *mut libc::c_void) -> ()>,
                 _: *mut libc::c_void) -> ();
    #[no_mangle]
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                 __oact: *mut sigaction) -> libc::c_int;
    #[no_mangle]
    static _sys_siglist: [*const libc::c_char; 65];
    #[no_mangle]
    static sys_siglist: [*const libc::c_char; 65];
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
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
    fn msgbuf_write(_: *mut msgbuf) -> libc::c_int;
    #[no_mangle]
    fn imsg_init(_: *mut imsgbuf, _: libc::c_int) -> ();
    #[no_mangle]
    fn imsg_read(_: *mut imsgbuf) -> ssize_t;
    #[no_mangle]
    fn imsg_get(_: *mut imsgbuf, _: *mut imsg) -> ssize_t;
    #[no_mangle]
    fn imsg_compose(_: *mut imsgbuf, _: uint32_t, _: uint32_t, _: pid_t,
                    _: libc::c_int, _: *const libc::c_void, _: uint16_t)
     -> libc::c_int;
    #[no_mangle]
    fn imsg_free(_: *mut imsg) -> ();
    #[no_mangle]
    fn imsg_clear(_: *mut imsgbuf) -> ();
    #[no_mangle]
    fn setproctitle(_: *const libc::c_char, ...) -> ();
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
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
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
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn log_open(_: *const libc::c_char) -> ();
    #[no_mangle]
    fn log_toggle(_: *const libc::c_char) -> ();
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
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type _IO_lock_t = ();
pub type __uid_t = libc::c_uint;
pub type sigval_t = sigval;
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_19,
}
pub type __suseconds_t = libc::c_long;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const MSG_EXIT: msgtype = 203;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
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
pub const MSG_WAKEUP: msgtype = 216;
pub type msgtype = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub type pid_t = __pid_t;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type cmdq_type = libc::c_uint;
pub type __u_char = libc::c_uchar;
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
    pub entry: unnamed,
}
pub type cmd_find_type = libc::c_uint;
pub const MSG_READY: msgtype = 207;
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const MSG_IDENTIFY_FLAGS: msgtype = 100;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_5 {
    ev_next_with_common_timeout: unnamed_4,
    min_heap_idx: libc::c_int,
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
    pub entry: unnamed_43,
    pub tree_entry: unnamed_17,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sigaction {
    pub __sigaction_handler: unnamed_33,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
pub const MSG_IDENTIFY_ENVIRON: msgtype = 105;
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
pub struct unnamed_6 {
    pub tqh_first: *mut ibuf,
    pub tqh_last: *mut *mut ibuf,
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
pub const MSG_DETACHKILL: msgtype = 202;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut imsg_fd,
    pub tqe_prev: *mut *mut imsg_fd,
}
pub const MSG_IDENTIFY_TERM: msgtype = 101;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const LINE_SEL_RIGHT_LEFT: unnamed_47 = 2;
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
pub const PROMPT_COMMAND: unnamed_11 = 1;
pub const MSG_DETACH: msgtype = 201;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub type ssize_t = __ssize_t;
pub const MSG_LOCK: msgtype = 206;
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union sigval {
    sival_int: libc::c_int,
    sival_ptr: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
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
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const MSG_IDENTIFY_STDIN: msgtype = 104;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __sigchld_clock_t,
    pub si_stime: __sigchld_clock_t,
}
pub type unnamed_11 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub ev_io_next: unnamed_45,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut ibuf,
    pub tqe_prev: *mut *mut ibuf,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
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
pub struct unnamed_14 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const MSG_STDERR: msgtype = 211;
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
    pub gentry: unnamed_9,
    pub entry: unnamed_1,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_15 {
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
pub const TTY_VT101: unnamed_37 = 1;
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
    pub term_type: unnamed_37,
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
pub struct unnamed_16 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct imsg {
    pub hdr: imsg_hdr,
    pub fd: libc::c_int,
    pub data: *mut libc::c_void,
}
pub const MSG_VERSION: msgtype = 12;
pub type options_table_scope = libc::c_uint;
pub const TTY_VT220: unnamed_37 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_18,
    pub ev_next: unnamed_50,
    pub ev_timeout_pos: unnamed_5,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_38,
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
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const JOB_CLOSED: unnamed_51 = 2;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const MSG_IDENTIFY_OLDCWD: msgtype = 103;
pub const LINE_SEL_LEFT_RIGHT: unnamed_47 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
pub const TTY_VT420: unnamed_37 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_27,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: sigval_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const MSG_RESIZE: msgtype = 208;
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
pub const MSG_STDOUT: msgtype = 213;
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
pub struct unnamed_23 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: sigval_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_25 {
    offset: u_int,
    data: unnamed_34,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub type uint32_t = libc::c_uint;
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
    pub winlinks: unnamed_24,
    pub entry: unnamed_29,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_25,
}
pub const MSG_SUSPEND: msgtype = 214;
pub const JOB_DEAD: unnamed_51 = 1;
pub const JOB_RUNNING: unnamed_51 = 0;
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
pub const MSG_IDENTIFY_TTYNAME: msgtype = 102;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct imsg_hdr {
    pub type_0: uint32_t,
    pub len: uint16_t,
    pub flags: uint16_t,
    pub peerid: uint32_t,
    pub pid: uint32_t,
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
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_41,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type tcflag_t = libc::c_uint;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct imsg_fd {
    pub entry: unnamed_7,
    pub fd: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const TTY_VT320: unnamed_37 = 4;
pub const MSG_COMMAND: msgtype = 200;
pub type __pid_t = libc::c_int;
pub type sigset_t = __sigset_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub _sifields: unnamed_49,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_33 {
    sa_handler: __sighandler_t,
    sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                              _: *mut siginfo_t,
                                              _: *mut libc::c_void) -> ()>,
}
pub type __off64_t = libc::c_long;
pub type bitstr_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub const MSG_SHELL: msgtype = 209;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type uint16_t = libc::c_ushort;
pub const MSG_IDENTIFY_CWD: msgtype = 108;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
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
pub const MSG_UNLOCK: msgtype = 215;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const TTY_VT102: unnamed_37 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
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
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
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
    pub message_log: unnamed_26,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_11,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_40,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_15,
}
pub type u_char = __u_char;
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
pub struct unnamed_35 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub const MSG_IDENTIFY_CLIENTPID: msgtype = 107;
pub type unnamed_37 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_38 {
    ev_io: unnamed_12,
    ev_signal: unnamed_42,
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
    pub entry: unnamed_36,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_40 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub type key_code = libc::c_ulonglong;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub const LINE_SEL_NONE: unnamed_47 = 0;
pub type time_t = __time_t;
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
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tmuxproc {
    pub name: *const libc::c_char,
    pub exit: libc::c_int,
    pub signalcb: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub ev_sighup: event,
    pub ev_sigchld: event,
    pub ev_sigcont: event,
    pub ev_sigterm: event,
    pub ev_sigusr1: event,
    pub ev_sigusr2: event,
    pub ev_sigwinch: event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct msgbuf {
    pub bufs: unnamed_6,
    pub queued: uint32_t,
    pub fd: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ibuf_read {
    pub buf: [u_char; 65535],
    pub rptr: *mut u_char,
    pub wpos: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tmuxpeer {
    pub parent: *mut tmuxproc,
    pub ibuf: imsgbuf,
    pub event: event,
    pub flags: libc::c_int,
    pub dispatchcb: Option<unsafe extern "C" fn(_: *mut imsg,
                                                _: *mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_41 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub type u_int = __u_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_42 {
    pub ev_signal_next: unnamed_8,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const TTY_VT100: unnamed_37 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_2,
    pub entry: unnamed_39,
}
pub const PROMPT_ENTRY: unnamed_11 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_43 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_16,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_47,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const CMDQ_COMMAND: cmdq_type = 0;
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
    pub entry: unnamed_3,
    pub wentry: unnamed_21,
    pub sentry: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_51,
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
    pub entry: unnamed_35,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_14,
}
pub const MSG_IDENTIFY_DONE: msgtype = 106;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const MSG_EXEC: msgtype = 217;
pub const TTY_UNKNOWN: unnamed_37 = 6;
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_44 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub si_addr_bnd: unnamed_48,
}
pub const MSG_EXITING: msgtype = 205;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_45 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_46 {
    pub tqh_first: *mut imsg_fd,
    pub tqh_last: *mut *mut imsg_fd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct imsgbuf {
    pub fds: unnamed_46,
    pub r: ibuf_read,
    pub w: msgbuf,
    pub fd: libc::c_int,
    pub pid: pid_t,
}
pub type unnamed_47 = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub const MSG_SHUTDOWN: msgtype = 210;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub type options_table_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_48 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_49 {
    _pad: [libc::c_int; 28],
    _kill: unnamed_28,
    _timer: unnamed_20,
    _rt: unnamed_23,
    _sigchld: unnamed_10,
    _sigfault: unnamed_44,
    _sigpoll: unnamed_30,
    _sigsys: unnamed_52,
}
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_50 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const MSG_STDIN: msgtype = 212;
pub type unnamed_51 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_31,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_52 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_32,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const MSG_EXITED: msgtype = 204;
pub type __sigchld_clock_t = __clock_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct ibuf {
    pub entry: unnamed_13,
    pub buf: *mut u_char,
    pub size: size_t,
    pub max: size_t,
    pub wpos: size_t,
    pub rpos: size_t,
    pub fd: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn proc_send(mut peer: *mut tmuxpeer,
                                   mut type_0: msgtype, mut fd: libc::c_int,
                                   mut buf: *const libc::c_void,
                                   mut len: size_t) -> libc::c_int {
    let mut ibuf: *mut imsgbuf = &mut (*peer).ibuf as *mut imsgbuf;
    let mut vp: *mut libc::c_void = buf as *mut libc::c_void;
    let mut retval: libc::c_int = 0;
    if 0 != (*peer).flags & 1i32 {
        return 1i32.wrapping_neg()
    } else {
        log_debug(b"sending message %d to peer %p (%zu bytes)\x00" as
                      *const u8 as *const libc::c_char,
                  type_0 as libc::c_uint, peer, len);
        retval =
            imsg_compose(ibuf, type_0 as uint32_t, 8i32 as uint32_t,
                         1i32.wrapping_neg(), fd, vp, len as uint16_t);
        if retval != 1i32 {
            return 1i32.wrapping_neg()
        } else { proc_update_event(peer); return 0i32 }
    };
}
unsafe extern "C" fn proc_update_event(mut peer: *mut tmuxpeer) -> () {
    let mut events: libc::c_short = 0;
    event_del(&mut (*peer).event as *mut event);
    events = 2i32 as libc::c_short;
    if (*peer).ibuf.w.queued > 0i32 as libc::c_uint {
        events = (events as libc::c_int | 4i32) as libc::c_short
    }
    event_set(&mut (*peer).event as *mut event, (*peer).ibuf.fd, events,
              Some(proc_event_cb), peer as *mut libc::c_void);
    event_add(&mut (*peer).event as *mut event, 0 as *const timeval);
}
unsafe extern "C" fn proc_event_cb(mut fd: libc::c_int,
                                   mut events: libc::c_short,
                                   mut arg: *mut libc::c_void) -> () {
    let mut current_block: u64;
    let mut peer: *mut tmuxpeer = arg as *mut tmuxpeer;
    let mut n: ssize_t = 0;
    let mut imsg: imsg =
        imsg{hdr: imsg_hdr{type_0: 0, len: 0, flags: 0, peerid: 0, pid: 0,},
             fd: 0,
             data: 0 as *mut libc::c_void,};
    if 0 == (*peer).flags & 1i32 && 0 != events as libc::c_int & 2i32 {
        n = imsg_read(&mut (*peer).ibuf as *mut imsgbuf);
        if n == 1i32.wrapping_neg() as libc::c_long &&
               *__errno_location() != 11i32 || n == 0i32 as libc::c_long {
            (*peer).dispatchcb.expect("non-null function pointer")(0 as
                                                                       *mut imsg,
                                                                   (*peer).arg);
            return
        } else {
            loop  {
                n =
                    imsg_get(&mut (*peer).ibuf as *mut imsgbuf,
                             &mut imsg as *mut imsg);
                if n == 1i32.wrapping_neg() as libc::c_long {
                    (*peer).dispatchcb.expect("non-null function pointer")(0
                                                                               as
                                                                               *mut imsg,
                                                                           (*peer).arg);
                    return
                } else {
                    if n == 0i32 as libc::c_long {
                        current_block = 11174649648027449784;
                        break ;
                    }
                    log_debug(b"peer %p message %d\x00" as *const u8 as
                                  *const libc::c_char, peer, imsg.hdr.type_0);
                    if peer_check_version(peer, &mut imsg as *mut imsg) !=
                           0i32 {
                        if imsg.fd != 1i32.wrapping_neg() {
                            current_block = 7351195479953500246;
                            break ;
                        } else {
                            current_block = 11006700562992250127;
                            break ;
                        }
                    } else {
                        (*peer).dispatchcb.expect("non-null function pointer")(&mut imsg
                                                                                   as
                                                                                   *mut imsg,
                                                                               (*peer).arg);
                        imsg_free(&mut imsg as *mut imsg);
                    }
                }
            }
            match current_block {
                11174649648027449784 => { }
                _ => {
                    match current_block {
                        7351195479953500246 => { close(imsg.fd); }
                        _ => { }
                    }
                    imsg_free(&mut imsg as *mut imsg);
                }
            }
        }
    }
    if 0 != events as libc::c_int & 4i32 {
        if msgbuf_write(&mut (*peer).ibuf.w as *mut msgbuf) <= 0i32 &&
               *__errno_location() != 11i32 {
            (*peer).dispatchcb.expect("non-null function pointer")(0 as
                                                                       *mut imsg,
                                                                   (*peer).arg);
            return
        }
    }
    if 0 != (*peer).flags & 1i32 &&
           (*peer).ibuf.w.queued == 0i32 as libc::c_uint {
        (*peer).dispatchcb.expect("non-null function pointer")(0 as *mut imsg,
                                                               (*peer).arg);
        return
    } else { proc_update_event(peer); return; };
}
unsafe extern "C" fn peer_check_version(mut peer: *mut tmuxpeer,
                                        mut imsg: *mut imsg) -> libc::c_int {
    let mut version: libc::c_int = 0;
    version = ((*imsg).hdr.peerid & 255i32 as libc::c_uint) as libc::c_int;
    if (*imsg).hdr.type_0 != MSG_VERSION as libc::c_int as libc::c_uint &&
           version != 8i32 {
        log_debug(b"peer %p bad version %d\x00" as *const u8 as
                      *const libc::c_char, peer, version);
        proc_send(peer, MSG_VERSION, 1i32.wrapping_neg(),
                  0 as *const libc::c_void, 0i32 as size_t);
        (*peer).flags |= 1i32;
        return 1i32.wrapping_neg()
    } else { return 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn proc_start(mut name: *const libc::c_char)
 -> *mut tmuxproc {
    let mut tp: *mut tmuxproc = 0 as *mut tmuxproc;
    let mut u: utsname =
        utsname{sysname: [0; 65],
                nodename: [0; 65],
                release: [0; 65],
                version: [0; 65],
                machine: [0; 65],
                domainname: [0; 65],};
    log_open(name);
    setproctitle(b"%s (%s)\x00" as *const u8 as *const libc::c_char, name,
                 socket_path);
    if uname(&mut u as *mut utsname) < 0i32 {
        memset(&mut u as *mut utsname as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<utsname>() as libc::c_ulong);
    }
    log_debug(b"%s started (%ld): version %s, socket %s, protocol %d\x00" as
                  *const u8 as *const libc::c_char, name,
              getpid() as libc::c_long,
              b"xmaster-rs\x00" as *const u8 as *const libc::c_char,
              socket_path, 8i32);
    log_debug(b"on %s %s %s; libevent %s (%s)\x00" as *const u8 as
                  *const libc::c_char, u.sysname.as_mut_ptr(),
              u.release.as_mut_ptr(), u.version.as_mut_ptr(),
              event_get_version(), event_get_method());
    tp =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<tmuxproc>() as libc::c_ulong) as
            *mut tmuxproc;
    (*tp).name = xstrdup(name);
    return tp;
}
#[no_mangle]
pub unsafe extern "C" fn proc_loop(mut tp: *mut tmuxproc,
                                   mut loopcb:
                                       Option<unsafe extern "C" fn()
                                                  -> libc::c_int>) -> () {
    log_debug(b"%s loop enter\x00" as *const u8 as *const libc::c_char,
              (*tp).name);
    loop  {
        event_loop(1i32);
        if !(0 == (*tp).exit &&
                 (loopcb ==
                      ::std::mem::transmute::<*mut libc::c_void,
                                              Option<unsafe extern "C" fn()
                                                         ->
                                                             libc::c_int>>(0
                                                                               as
                                                                               *mut libc::c_void)
                      || 0 == loopcb.expect("non-null function pointer")())) {
            break ;
        }
    }
    log_debug(b"%s loop exit\x00" as *const u8 as *const libc::c_char,
              (*tp).name);
}
#[no_mangle]
pub unsafe extern "C" fn proc_exit(mut tp: *mut tmuxproc) -> () {
    (*tp).exit = 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn proc_set_signals(mut tp: *mut tmuxproc,
                                          mut signalcb:
                                              Option<unsafe extern "C" fn(_:
                                                                              libc::c_int)
                                                         -> ()>) -> () {
    let mut sa: sigaction =
        sigaction{__sigaction_handler: unnamed_33{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    (*tp).signalcb = signalcb;
    memset(&mut sa as *mut sigaction as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<sigaction>() as libc::c_ulong);
    sigemptyset(&mut sa.sa_mask as *mut __sigset_t);
    sa.sa_flags = 268435456i32;
    sa.__sigaction_handler.sa_handler =
        ::std::mem::transmute::<libc::intptr_t,
                                __sighandler_t>(1i32 as libc::intptr_t);
    sigaction(2i32, &mut sa as *mut sigaction, 0 as *mut sigaction);
    sigaction(13i32, &mut sa as *mut sigaction, 0 as *mut sigaction);
    sigaction(20i32, &mut sa as *mut sigaction, 0 as *mut sigaction);
    event_set(&mut (*tp).ev_sighup as *mut event, 1i32,
              (8i32 | 16i32) as libc::c_short, Some(proc_signal_cb),
              tp as *mut libc::c_void);
    event_add(&mut (*tp).ev_sighup as *mut event, 0 as *const timeval);
    event_set(&mut (*tp).ev_sigchld as *mut event, 17i32,
              (8i32 | 16i32) as libc::c_short, Some(proc_signal_cb),
              tp as *mut libc::c_void);
    event_add(&mut (*tp).ev_sigchld as *mut event, 0 as *const timeval);
    event_set(&mut (*tp).ev_sigcont as *mut event, 18i32,
              (8i32 | 16i32) as libc::c_short, Some(proc_signal_cb),
              tp as *mut libc::c_void);
    event_add(&mut (*tp).ev_sigcont as *mut event, 0 as *const timeval);
    event_set(&mut (*tp).ev_sigterm as *mut event, 15i32,
              (8i32 | 16i32) as libc::c_short, Some(proc_signal_cb),
              tp as *mut libc::c_void);
    event_add(&mut (*tp).ev_sigterm as *mut event, 0 as *const timeval);
    event_set(&mut (*tp).ev_sigusr1 as *mut event, 10i32,
              (8i32 | 16i32) as libc::c_short, Some(proc_signal_cb),
              tp as *mut libc::c_void);
    event_add(&mut (*tp).ev_sigusr1 as *mut event, 0 as *const timeval);
    event_set(&mut (*tp).ev_sigusr2 as *mut event, 12i32,
              (8i32 | 16i32) as libc::c_short, Some(proc_signal_cb),
              tp as *mut libc::c_void);
    event_add(&mut (*tp).ev_sigusr2 as *mut event, 0 as *const timeval);
    event_set(&mut (*tp).ev_sigwinch as *mut event, 28i32,
              (8i32 | 16i32) as libc::c_short, Some(proc_signal_cb),
              tp as *mut libc::c_void);
    event_add(&mut (*tp).ev_sigwinch as *mut event, 0 as *const timeval);
}
unsafe extern "C" fn proc_signal_cb(mut signo: libc::c_int,
                                    mut events: libc::c_short,
                                    mut arg: *mut libc::c_void) -> () {
    let mut tp: *mut tmuxproc = arg as *mut tmuxproc;
    (*tp).signalcb.expect("non-null function pointer")(signo);
}
#[no_mangle]
pub unsafe extern "C" fn proc_clear_signals(mut tp: *mut tmuxproc,
                                            mut defaults: libc::c_int) -> () {
    let mut sa: sigaction =
        sigaction{__sigaction_handler: unnamed_33{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    memset(&mut sa as *mut sigaction as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<sigaction>() as libc::c_ulong);
    sigemptyset(&mut sa.sa_mask as *mut __sigset_t);
    sa.sa_flags = 268435456i32;
    sa.__sigaction_handler.sa_handler = None;
    sigaction(2i32, &mut sa as *mut sigaction, 0 as *mut sigaction);
    sigaction(13i32, &mut sa as *mut sigaction, 0 as *mut sigaction);
    sigaction(20i32, &mut sa as *mut sigaction, 0 as *mut sigaction);
    event_del(&mut (*tp).ev_sighup as *mut event);
    event_del(&mut (*tp).ev_sigchld as *mut event);
    event_del(&mut (*tp).ev_sigcont as *mut event);
    event_del(&mut (*tp).ev_sigterm as *mut event);
    event_del(&mut (*tp).ev_sigusr1 as *mut event);
    event_del(&mut (*tp).ev_sigusr2 as *mut event);
    event_del(&mut (*tp).ev_sigwinch as *mut event);
    if 0 != defaults {
        sigaction(1i32, &mut sa as *mut sigaction, 0 as *mut sigaction);
        sigaction(17i32, &mut sa as *mut sigaction, 0 as *mut sigaction);
        sigaction(18i32, &mut sa as *mut sigaction, 0 as *mut sigaction);
        sigaction(15i32, &mut sa as *mut sigaction, 0 as *mut sigaction);
        sigaction(10i32, &mut sa as *mut sigaction, 0 as *mut sigaction);
        sigaction(12i32, &mut sa as *mut sigaction, 0 as *mut sigaction);
        sigaction(28i32, &mut sa as *mut sigaction, 0 as *mut sigaction);
    };
}
#[no_mangle]
pub unsafe extern "C" fn proc_add_peer(mut tp: *mut tmuxproc,
                                       mut fd: libc::c_int,
                                       mut dispatchcb:
                                           Option<unsafe extern "C" fn(_:
                                                                           *mut imsg,
                                                                       _:
                                                                           *mut libc::c_void)
                                                      -> ()>,
                                       mut arg: *mut libc::c_void)
 -> *mut tmuxpeer {
    let mut peer: *mut tmuxpeer = 0 as *mut tmuxpeer;
    peer =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<tmuxpeer>() as libc::c_ulong) as
            *mut tmuxpeer;
    (*peer).parent = tp;
    (*peer).dispatchcb = dispatchcb;
    (*peer).arg = arg;
    imsg_init(&mut (*peer).ibuf as *mut imsgbuf, fd);
    event_set(&mut (*peer).event as *mut event, fd, 2i32 as libc::c_short,
              Some(proc_event_cb), peer as *mut libc::c_void);
    log_debug(b"add peer %p: %d (%p)\x00" as *const u8 as *const libc::c_char,
              peer, fd, arg);
    proc_update_event(peer);
    return peer;
}
#[no_mangle]
pub unsafe extern "C" fn proc_remove_peer(mut peer: *mut tmuxpeer) -> () {
    log_debug(b"remove peer %p\x00" as *const u8 as *const libc::c_char,
              peer);
    event_del(&mut (*peer).event as *mut event);
    imsg_clear(&mut (*peer).ibuf as *mut imsgbuf);
    close((*peer).ibuf.fd);
    free(peer as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn proc_kill_peer(mut peer: *mut tmuxpeer) -> () {
    (*peer).flags |= 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn proc_toggle_log(mut tp: *mut tmuxproc) -> () {
    log_toggle((*tp).name);
}

