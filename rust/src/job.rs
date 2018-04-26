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
    pub type tty_code;
    pub type _IO_FILE_plus;
    pub type options;
    pub type environ;
    pub type screen_titles;
    pub type hooks;
    pub type args_entry;
    pub type format_tree;
    pub type tmuxproc;
    pub type tmuxpeer;
    pub type evbuffer;
    pub type format_job_tree;
    pub type bufferevent_ops;
    pub type event_base;
    #[no_mangle]
    fn socketpair(__domain: libc::c_int, __type: libc::c_int,
                  __protocol: libc::c_int, __fds: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, ...)
     -> libc::c_int;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    fn fork() -> __pid_t;
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
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn bufferevent_free(bufev: *mut bufferevent) -> ();
    #[no_mangle]
    fn bufferevent_get_output(bufev: *mut bufferevent) -> *mut evbuffer;
    #[no_mangle]
    fn bufferevent_enable(bufev: *mut bufferevent, event: libc::c_short)
     -> libc::c_int;
    #[no_mangle]
    fn bufferevent_disable(bufev: *mut bufferevent, event: libc::c_short)
     -> libc::c_int;
    #[no_mangle]
    fn bufferevent_new(fd: libc::c_int, readcb: bufferevent_data_cb,
                       writecb: bufferevent_data_cb,
                       errorcb: bufferevent_event_cb,
                       cbarg: *mut libc::c_void) -> *mut bufferevent;
    #[no_mangle]
    fn closefrom(_: libc::c_int) -> ();
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
    fn setblocking(_: libc::c_int, _: libc::c_int) -> ();
    #[no_mangle]
    fn find_home() -> *const libc::c_char;
    #[no_mangle]
    fn proc_clear_signals(_: *mut tmuxproc, _: libc::c_int) -> ();
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn environ_free(_: *mut environ) -> ();
    #[no_mangle]
    fn fatal(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn environ_push(_: *mut environ) -> ();
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    fn environ_for_session(_: *mut session, _: libc::c_int) -> *mut environ;
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    static mut key_tables: key_tables;
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
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    offset: u_int,
    data: unnamed_11,
}
pub type time_t = __time_t;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_SEQPACKET: __socket_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub ev_signal_next: unnamed_4,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub type tcflag_t = libc::c_uint;
pub const SHUT_RDWR: unnamed_21 = 2;
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
    pub term_type: unnamed_36,
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
pub const TTY_VT102: unnamed_36 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub type __sigset_t = sigset_t;
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_25 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const TTY_VT220: unnamed_36 = 3;
pub type unnamed_5 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
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
    pub entry: unnamed_6,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub const TTY_UNKNOWN: unnamed_36 = 6;
pub const LINE_SEL_RIGHT_LEFT: unnamed_25 = 2;
pub type u_char = __u_char;
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
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
pub struct unnamed_7 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_18,
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
pub const CMD_FIND_SESSION: cmd_find_type = 2;
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
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type __socket_type = libc::c_uint;
pub const SOCK_DGRAM: __socket_type = 2;
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
pub const TTY_VT101: unnamed_36 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_39,
    pub ev_next: unnamed_20,
    pub ev_timeout_pos: unnamed_32,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_22,
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
pub struct unnamed_8 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
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
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
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
pub type options_table_type = libc::c_uint;
pub const SHUT_RD: unnamed_21 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_25,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub const LINE_SEL_NONE: unnamed_25 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_17,
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
pub const PROMPT_COMMAND: unnamed_5 = 1;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_37,
    pub entry: unnamed_9,
}
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const CMDQ_COMMAND: cmdq_type = 0;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_29,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_30,
}
pub type unnamed_13 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub type __off64_t = libc::c_long;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const JOB_DEAD: unnamed_13 = 1;
pub type cmd_retval = libc::c_int;
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
    pub message_log: unnamed_2,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_5,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_12,
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
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
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_10,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const SOCK_RAW: __socket_type = 3;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type __pid_t = libc::c_int;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub const JOB_CLOSED: unnamed_13 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const SOCK_PACKET: __socket_type = 10;
pub const TTY_VT420: unnamed_36 = 5;
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
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const SOCK_DCCP: __socket_type = 6;
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
pub struct unnamed_20 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type unnamed_21 = libc::c_uint;
pub const TTY_VT320: unnamed_36 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_22 {
    ev_io: unnamed_28,
    ev_signal: unnamed_1,
}
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
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
    pub gentry: unnamed_35,
    pub entry: unnamed_33,
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
pub struct sessions {
    pub rbh_root: *mut session,
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const SOCK_STREAM: __socket_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
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
pub type cc_t = libc::c_uchar;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub type unnamed_25 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub ev_io_next: unnamed_34,
    pub ev_timeout: timeval,
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
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const SOCK_NONBLOCK: __socket_type = 2048;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
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
    pub entry: unnamed_7,
    pub tree_entry: unnamed_24,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_30 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub type cmdq_type = libc::c_uint;
pub const TTY_VT100: unnamed_36 = 0;
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
    pub alerts_entry: unnamed_3,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_15,
    pub entry: unnamed_23,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_26,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_13,
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
    pub entry: unnamed_8,
}
pub const JOB_RUNNING: unnamed_13 = 0;
pub type layout_type = libc::c_uint;
pub const SHUT_WR: unnamed_21 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
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
pub struct winlink {
    pub idx: libc::c_int,
    pub session: *mut session,
    pub window: *mut window,
    pub status_width: size_t,
    pub status_cell: grid_cell,
    pub status_text: *mut libc::c_char,
    pub flags: libc::c_int,
    pub entry: unnamed_16,
    pub wentry: unnamed_19,
    pub sentry: unnamed_14,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_32 {
    ev_next_with_common_timeout: unnamed_27,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
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
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type u_int = __u_int;
pub type __u_short = libc::c_ushort;
pub const SOCK_RDM: __socket_type = 4;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub type __off_t = libc::c_long;
pub type bitstr_t = libc::c_uchar;
pub type options_table_scope = libc::c_uint;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type unnamed_36 = libc::c_uint;
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_31,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const PROMPT_ENTRY: unnamed_5 = 0;
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
pub struct unnamed_39 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[no_mangle]
pub unsafe extern "C" fn job_run(mut cmd: *const libc::c_char,
                                 mut s: *mut session,
                                 mut cwd: *const libc::c_char,
                                 mut updatecb: job_update_cb,
                                 mut completecb: job_complete_cb,
                                 mut freecb: job_free_cb,
                                 mut data: *mut libc::c_void,
                                 mut flags: libc::c_int) -> *mut job {
    let mut job: *mut job = 0 as *mut job;
    let mut env: *mut environ = 0 as *mut environ;
    let mut pid: pid_t = 0;
    let mut nullfd: libc::c_int = 0;
    let mut out: [libc::c_int; 2] = [0; 2];
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    let mut set: sigset_t = sigset_t{__val: [0; 16],};
    let mut oldset: sigset_t = sigset_t{__val: [0; 16],};
    if socketpair(1i32, SOCK_STREAM as libc::c_int, 0i32, out.as_mut_ptr()) !=
           0i32 {
        return 0 as *mut job
    } else {
        env = environ_for_session(s, (0 == cfg_finished) as libc::c_int);
        sigfillset(&mut set as *mut sigset_t);
        sigprocmask(0i32, &mut set as *mut sigset_t,
                    &mut oldset as *mut sigset_t);
        pid = fork();
        match pid {
            -1 => {
                sigprocmask(2i32, &mut oldset as *mut sigset_t,
                            0 as *mut sigset_t);
                environ_free(env);
                close(out[0usize]);
                close(out[1usize]);
                return 0 as *mut job
            }
            0 => {
                proc_clear_signals(server_proc, 1i32);
                sigprocmask(2i32, &mut oldset as *mut sigset_t,
                            0 as *mut sigset_t);
                if cwd == 0 as *mut libc::c_void as *const libc::c_char ||
                       chdir(cwd) != 0i32 {
                    home = find_home();
                    if home == 0 as *mut libc::c_void as *const libc::c_char
                           || chdir(home) != 0i32 {
                        chdir(b"/\x00" as *const u8 as *const libc::c_char);
                    }
                }
                environ_push(env);
                environ_free(env);
                if dup2(out[1usize], 0i32) == 1i32.wrapping_neg() {
                    fatal(b"dup2 failed\x00" as *const u8 as
                              *const libc::c_char);
                } else if dup2(out[1usize], 1i32) == 1i32.wrapping_neg() {
                    fatal(b"dup2 failed\x00" as *const u8 as
                              *const libc::c_char);
                } else {
                    if out[1usize] != 0i32 && out[1usize] != 1i32 {
                        close(out[1usize]);
                    }
                    close(out[0usize]);
                    nullfd =
                        open(b"/dev/null\x00" as *const u8 as
                                 *const libc::c_char, 2i32, 0i32);
                    if nullfd < 0i32 {
                        fatal(b"open failed\x00" as *const u8 as
                                  *const libc::c_char);
                    } else if dup2(nullfd, 2i32) == 1i32.wrapping_neg() {
                        fatal(b"dup2 failed\x00" as *const u8 as
                                  *const libc::c_char);
                    } else {
                        if nullfd != 2i32 { close(nullfd); }
                        closefrom(2i32 + 1i32);
                        execl(b"/bin/sh\x00" as *const u8 as
                                  *const libc::c_char,
                              b"sh\x00" as *const u8 as *const libc::c_char,
                              b"-c\x00" as *const u8 as *const libc::c_char,
                              cmd,
                              0 as *mut libc::c_void as *mut libc::c_char);
                        fatal(b"execl failed\x00" as *const u8 as
                                  *const libc::c_char);
                    }
                }
            }
            _ => {
                sigprocmask(2i32, &mut oldset as *mut sigset_t,
                            0 as *mut sigset_t);
                environ_free(env);
                close(out[1usize]);
                job =
                    xmalloc(::std::mem::size_of::<job>() as libc::c_ulong) as
                        *mut job;
                (*job).state = JOB_RUNNING;
                (*job).flags = flags;
                (*job).cmd = xstrdup(cmd);
                (*job).pid = pid;
                (*job).status = 0i32;
                loop  {
                    (*job).entry.le_next =
                        (*(&mut all_jobs as *mut joblist)).lh_first;
                    if (*job).entry.le_next !=
                           0 as *mut libc::c_void as *mut job {
                        let ref mut fresh0 =
                            (*(*(&mut all_jobs as
                                     *mut joblist)).lh_first).entry.le_prev;
                        *fresh0 = &mut (*job).entry.le_next as *mut *mut job
                    }
                    let ref mut fresh1 =
                        (*(&mut all_jobs as *mut joblist)).lh_first;
                    *fresh1 = job;
                    (*job).entry.le_prev =
                        &mut (*(&mut all_jobs as *mut joblist)).lh_first as
                            *mut *mut job;
                    if !(0 != 0i32) { break ; }
                }
                (*job).updatecb = updatecb;
                (*job).completecb = completecb;
                (*job).freecb = freecb;
                (*job).data = data;
                (*job).fd = out[0usize];
                setblocking((*job).fd, 0i32);
                (*job).event =
                    bufferevent_new((*job).fd, Some(job_read_callback),
                                    Some(job_write_callback),
                                    Some(job_error_callback),
                                    job as *mut libc::c_void);
                bufferevent_enable((*job).event,
                                   (2i32 | 4i32) as libc::c_short);
                log_debug(b"run job %p: %s, pid %ld\x00" as *const u8 as
                              *const libc::c_char, job, (*job).cmd,
                          (*job).pid as libc::c_long);
                return job
            }
        }
    };
}
unsafe extern "C" fn job_error_callback(mut bufev: *mut bufferevent,
                                        mut events: libc::c_short,
                                        mut data: *mut libc::c_void) -> () {
    let mut job: *mut job = data as *mut job;
    log_debug(b"job error %p: %s, pid %ld\x00" as *const u8 as
                  *const libc::c_char, job, (*job).cmd,
              (*job).pid as libc::c_long);
    if (*job).state as libc::c_uint == JOB_DEAD as libc::c_int as libc::c_uint
       {
        if (*job).completecb !=
               ::std::mem::transmute::<*mut libc::c_void,
                                       job_complete_cb>(0 as
                                                            *mut libc::c_void)
           {
            (*job).completecb.expect("non-null function pointer")(job);
        }
        job_free(job);
    } else {
        bufferevent_disable((*job).event, 2i32 as libc::c_short);
        (*job).state = JOB_CLOSED
    };
}
#[no_mangle]
pub unsafe extern "C" fn job_free(mut job: *mut job) -> () {
    log_debug(b"free job %p: %s\x00" as *const u8 as *const libc::c_char, job,
              (*job).cmd);
    loop  {
        if (*job).entry.le_next != 0 as *mut libc::c_void as *mut job {
            (*(*job).entry.le_next).entry.le_prev = (*job).entry.le_prev
        }
        *(*job).entry.le_prev = (*job).entry.le_next;
        if !(0 != 0i32) { break ; }
    }
    free((*job).cmd as *mut libc::c_void);
    if (*job).freecb !=
           ::std::mem::transmute::<*mut libc::c_void,
                                   job_free_cb>(0 as *mut libc::c_void) &&
           (*job).data != 0 as *mut libc::c_void {
        (*job).freecb.expect("non-null function pointer")((*job).data);
    }
    if (*job).pid != 1i32.wrapping_neg() { kill((*job).pid, 15i32); }
    if (*job).event != 0 as *mut libc::c_void as *mut bufferevent {
        bufferevent_free((*job).event);
    }
    if (*job).fd != 1i32.wrapping_neg() { close((*job).fd); }
    free(job as *mut libc::c_void);
}
unsafe extern "C" fn job_write_callback(mut bufev: *mut bufferevent,
                                        mut data: *mut libc::c_void) -> () {
    let mut job: *mut job = data as *mut job;
    let mut len: size_t =
        evbuffer_get_length(bufferevent_get_output((*job).event));
    log_debug(b"job write %p: %s, pid %ld, output left %zu\x00" as *const u8
                  as *const libc::c_char, job, (*job).cmd,
              (*job).pid as libc::c_long, len);
    if len == 0i32 as libc::c_ulong {
        shutdown((*job).fd, SHUT_WR as libc::c_int);
        bufferevent_disable((*job).event, 4i32 as libc::c_short);
    };
}
unsafe extern "C" fn job_read_callback(mut bufev: *mut bufferevent,
                                       mut data: *mut libc::c_void) -> () {
    let mut job: *mut job = data as *mut job;
    if (*job).updatecb !=
           ::std::mem::transmute::<*mut libc::c_void,
                                   job_update_cb>(0 as *mut libc::c_void) {
        (*job).updatecb.expect("non-null function pointer")(job);
    };
}
#[no_mangle]
pub unsafe extern "C" fn job_died(mut job: *mut job, mut status: libc::c_int)
 -> () {
    log_debug(b"job died %p: %s, pid %ld\x00" as *const u8 as
                  *const libc::c_char, job, (*job).cmd,
              (*job).pid as libc::c_long);
    (*job).status = status;
    if (*job).state as libc::c_uint ==
           JOB_CLOSED as libc::c_int as libc::c_uint {
        if (*job).completecb !=
               ::std::mem::transmute::<*mut libc::c_void,
                                       job_complete_cb>(0 as
                                                            *mut libc::c_void)
           {
            (*job).completecb.expect("non-null function pointer")(job);
        }
        job_free(job);
    } else { (*job).pid = 1i32.wrapping_neg(); (*job).state = JOB_DEAD };
}

