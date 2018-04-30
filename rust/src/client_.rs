extern crate libc;
extern "C" {
    pub type input_ctx;
    pub type tty_code;
    pub type hooks;
    pub type sockaddr_iso;
    pub type format_job_tree;
    pub type _IO_FILE_plus;
    pub type format_tree;
    pub type sockaddr_x25;
    pub type options;
    pub type args_entry;
    pub type environ;
    pub type sockaddr_ipx;
    pub type sockaddr_at;
    pub type evbuffer;
    pub type sockaddr_ax25;
    pub type sockaddr_eon;
    pub type screen_titles;
    pub type tmuxproc;
    pub type sockaddr_ns;
    pub type sockaddr_inarp;
    pub type event_base;
    pub type bufferevent_ops;
    pub type sockaddr_dl;
    pub type tmuxpeer;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, ...)
     -> libc::c_int;
    #[no_mangle]
    fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int,
              __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
               __len: socklen_t) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: __sighandler_t)
     -> __sighandler_t;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
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
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
               __options: libc::c_int) -> __pid_t;
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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, ...) -> libc::c_int;
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
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn setenv(__name: *const libc::c_char, __value: *const libc::c_char,
              __replace: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn system(__command: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    #[no_mangle]
    fn dup(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn getppid() -> __pid_t;
    #[no_mangle]
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    fn cfgetospeed(__termios_p: *const termios) -> speed_t;
    #[no_mangle]
    fn cfgetispeed(__termios_p: *const termios) -> speed_t;
    #[no_mangle]
    fn cfsetospeed(__termios_p: *mut termios, __speed: speed_t)
     -> libc::c_int;
    #[no_mangle]
    fn cfsetispeed(__termios_p: *mut termios, __speed: speed_t)
     -> libc::c_int;
    #[no_mangle]
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    #[no_mangle]
    fn tcsetattr(__fd: libc::c_int, __optional_actions: libc::c_int,
                 __termios_p: *const termios) -> libc::c_int;
    #[no_mangle]
    fn cfmakeraw(__termios_p: *mut termios) -> ();
    #[no_mangle]
    fn closefrom(_: libc::c_int) -> ();
    #[no_mangle]
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
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
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, ...)
     -> libc::c_int;
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
    fn proc_send(_: *mut tmuxpeer, _: msgtype, _: libc::c_int,
                 _: *const libc::c_void, _: size_t) -> libc::c_int;
    #[no_mangle]
    fn proc_start(_: *const libc::c_char) -> *mut tmuxproc;
    #[no_mangle]
    fn proc_loop(_: *mut tmuxproc,
                 _: Option<unsafe extern "C" fn() -> libc::c_int>) -> ();
    #[no_mangle]
    fn proc_exit(_: *mut tmuxproc) -> ();
    #[no_mangle]
    fn proc_set_signals(_: *mut tmuxproc,
                        _: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>)
     -> ();
    #[no_mangle]
    fn proc_clear_signals(_: *mut tmuxproc, _: libc::c_int) -> ();
    #[no_mangle]
    fn proc_add_peer(_: *mut tmuxproc, _: libc::c_int,
                     _:
                         Option<unsafe extern "C" fn(_: *mut imsg,
                                                     _: *mut libc::c_void)
                                    -> ()>, _: *mut libc::c_void)
     -> *mut tmuxpeer;
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn options_free(_: *mut options) -> ();
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn environ_free(_: *mut environ) -> ();
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn cmd_pack_argv(_: libc::c_int, _: *mut *mut libc::c_char,
                     _: *mut libc::c_char, _: size_t) -> libc::c_int;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn cmd_list_parse(_: libc::c_int, _: *mut *mut libc::c_char,
                      _: *const libc::c_char, _: u_int,
                      _: *mut *mut libc::c_char) -> *mut cmd_list;
    #[no_mangle]
    fn cmd_list_free(_: *mut cmd_list) -> ();
    #[no_mangle]
    fn fatal(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn server_start(_: *mut tmuxproc, _: *mut event_base, _: libc::c_int,
                    _: *mut libc::c_char) -> libc::c_int;
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
pub struct unnamed {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const MSG_IDENTIFY_ENVIRON: msgtype = 105;
pub const MSG_IDENTIFY_CLIENTPID: msgtype = 107;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub type options_table_type = libc::c_uint;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const SOCK_DGRAM: __socket_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const TTY_VT100: unnamed_5 = 0;
pub type unnamed_1 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const JOB_RUNNING: unnamed_1 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union __CONST_SOCKADDR_ARG {
    __sockaddr__: *const sockaddr,
    __sockaddr_at__: *const sockaddr_at,
    __sockaddr_ax25__: *const sockaddr_ax25,
    __sockaddr_dl__: *const sockaddr_dl,
    __sockaddr_eon__: *const sockaddr_eon,
    __sockaddr_in__: *const sockaddr_in,
    __sockaddr_in6__: *const sockaddr_in6,
    __sockaddr_inarp__: *const sockaddr_inarp,
    __sockaddr_ipx__: *const sockaddr_ipx,
    __sockaddr_iso__: *const sockaddr_iso,
    __sockaddr_ns__: *const sockaddr_ns,
    __sockaddr_un__: *const sockaddr_un,
    __sockaddr_x25__: *const sockaddr_x25,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub type unnamed_5 = libc::c_uint;
pub const CLIENT_EXIT_NONE: unnamed_25 = 0;
pub const PROMPT_ENTRY: unnamed_36 = 0;
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
pub type __u_char = libc::c_uchar;
pub const JOB_DEAD: unnamed_1 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub const MSG_EXITING: msgtype = 205;
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __ssize_t = libc::c_long;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct imsg {
    pub hdr: imsg_hdr,
    pub fd: libc::c_int,
    pub data: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
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
    pub qentry: unnamed_9,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type layout_type = libc::c_uint;
pub const SOCK_STREAM: __socket_type = 1;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_15,
}
pub const JOB_CLOSED: unnamed_1 = 2;
pub type u_int = __u_int;
pub type __u_short = libc::c_ushort;
pub type pid_t = __pid_t;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
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
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union sigval {
    sival_int: libc::c_int,
    sival_ptr: *mut libc::c_void,
}
pub const MSG_SHELL: msgtype = 209;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
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
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct msg_stdout_data {
    pub size: ssize_t,
    pub data: [libc::c_char; 8192],
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub type __off_t = libc::c_long;
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: sigval_t,
}
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type __socket_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub const CLIENT_EXIT_SERVER_EXITED: unnamed_25 = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_13 {
    ev_next_with_common_timeout: unnamed_41,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const CLIENT_EXIT_LOST_SERVER: unnamed_25 = 5;
pub const SOCK_CLOEXEC: __socket_type = 524288;
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
pub const SOCK_NONBLOCK: __socket_type = 2048;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_27,
}
pub const MSG_IDENTIFY_FLAGS: msgtype = 100;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
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
    pub alerts_entry: unnamed_38,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_30,
    pub entry: unnamed_8,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
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
pub const MSG_DETACH: msgtype = 201;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct imsg_hdr {
    pub type_0: uint32_t,
    pub len: uint16_t,
    pub flags: uint16_t,
    pub peerid: uint32_t,
    pub pid: uint32_t,
}
pub type msgtype = libc::c_uint;
pub const SOCK_PACKET: __socket_type = 10;
pub type cc_t = libc::c_uchar;
pub type u_short = __u_short;
pub const MSG_EXEC: msgtype = 217;
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
pub type unnamed_19 = libc::c_uint;
pub const CLIENT_EXIT_LOST_TTY: unnamed_25 = 3;
pub const TTY_VT102: unnamed_5 = 2;
pub const LINE_SEL_RIGHT_LEFT: unnamed_19 = 2;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub type uint8_t = libc::c_uchar;
pub type options_table_scope = libc::c_uint;
pub type uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub const SOCK_DCCP: __socket_type = 6;
pub const MSG_COMMAND: msgtype = 200;
pub const MSG_SHUTDOWN: msgtype = 210;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
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
pub type cmdq_type = libc::c_uint;
pub type __time_t = libc::c_long;
pub const LINE_SEL_LEFT_RIGHT: unnamed_19 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct msg_stderr_data {
    pub size: ssize_t,
    pub data: [libc::c_char; 8192],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sigaction {
    pub __sigaction_handler: unnamed_31,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
pub type sigval_t = sigval;
pub const MSG_IDENTIFY_STDIN: msgtype = 104;
pub const MSG_IDENTIFY_TTYNAME: msgtype = 102;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __sigchld_clock_t,
    pub si_stime: __sigchld_clock_t,
}
pub const MSG_UNLOCK: msgtype = 215;
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
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub _sifields: unnamed_28,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const MSG_IDENTIFY_DONE: msgtype = 106;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_17,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_45,
}
pub type cmd_retval = libc::c_int;
pub const MSG_LOCK: msgtype = 206;
pub const TTY_VT101: unnamed_5 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_23 {
    offset: u_int,
    data: unnamed_37,
}
pub type ssize_t = __ssize_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_21,
    pub entry: unnamed_10,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub type bitstr_t = libc::c_uchar;
pub const SOCK_RAW: __socket_type = 3;
pub const CLIENT_EXIT_DETACHED_HUP: unnamed_25 = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type unnamed_25 = libc::c_uint;
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
    pub entry: unnamed_44,
}
pub const MSG_VERSION: msgtype = 12;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_1,
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
    pub entry: unnamed_32,
}
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_28 {
    _pad: [libc::c_int; 28],
    _kill: unnamed_7,
    _timer: unnamed_12,
    _rt: unnamed_49,
    _sigchld: unnamed_20,
    _sigfault: unnamed_40,
    _sigpoll: unnamed_22,
    _sigsys: unnamed_18,
}
pub const MSG_STDOUT: msgtype = 213;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
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
pub const TTY_VT320: unnamed_5 = 4;
pub type sa_family_t = libc::c_ushort;
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
    pub entry: unnamed_35,
    pub wentry: unnamed_16,
    pub sentry: unnamed_24,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const LINE_SEL_NONE: unnamed_19 = 0;
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
    pub entry: unnamed_11,
    pub tree_entry: unnamed_43,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
pub type __uid_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_31 {
    sa_handler: __sighandler_t,
    sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                              _: *mut siginfo_t,
                                              _: *mut libc::c_void) -> ()>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub type __sigset_t = sigset_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub ev_io_next: unnamed_2,
    pub ev_timeout: timeval,
}
pub const MSG_DETACHKILL: msgtype = 202;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_4,
    pub ev_next: unnamed_26,
    pub ev_timeout_pos: unnamed_13,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_48,
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
pub const MSG_STDIN: msgtype = 212;
pub const MSG_SUSPEND: msgtype = 214;
pub const CMDQ_COMMAND: cmdq_type = 0;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub const MSG_EXITED: msgtype = 204;
pub type _IO_lock_t = ();
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type __clock_t = libc::c_long;
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
    pub entry: unnamed_14,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub type u_char = __u_char;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_0,
}
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const SOCK_RDM: __socket_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub type tcflag_t = libc::c_uint;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const MSG_IDENTIFY_CWD: msgtype = 108;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const MSG_READY: msgtype = 207;
pub type unnamed_36 = libc::c_uint;
pub const TTY_VT420: unnamed_5 = 5;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const MSG_IDENTIFY_TERM: msgtype = 101;
pub const CLIENT_EXIT_TERMINATED: unnamed_25 = 4;
pub const MSG_IDENTIFY_OLDCWD: msgtype = 103;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_23,
}
pub type in_addr_t = uint32_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub ev_signal_next: unnamed_47,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
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
pub const MSG_STDERR: msgtype = 211;
pub const PROMPT_COMMAND: unnamed_36 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct msg_stdin_data {
    pub size: ssize_t,
    pub data: [libc::c_char; 8192],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed,
}
pub const MSG_WAKEUP: msgtype = 216;
pub const CLIENT_EXIT_EXITED: unnamed_25 = 6;
pub const MSG_EXIT: msgtype = 203;
pub type socklen_t = __socklen_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_40 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub si_addr_bnd: unnamed_34,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_41 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type __off64_t = libc::c_long;
pub type FILE = _IO_FILE;
pub type __socklen_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_42 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub type __u_int = libc::c_uint;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_43 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_46,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_44 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
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
pub struct unnamed_45 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const TTY_UNKNOWN: unnamed_5 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_46 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub const CLIENT_EXIT_DETACHED: unnamed_25 = 1;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub type __sigchld_clock_t = __clock_t;
pub type uint16_t = libc::c_ushort;
pub const TTY_VT220: unnamed_5 = 3;
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
    pub gentry: unnamed_42,
    pub entry: unnamed_3,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
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
pub struct msg_command_data {
    pub argc: libc::c_int,
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
    pub message_log: unnamed_6,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_36,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_29,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
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
pub struct unnamed_47 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const SOCK_SEQPACKET: __socket_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_48 {
    ev_io: unnamed_33,
    ev_signal: unnamed_39,
}
pub const MSG_RESIZE: msgtype = 208;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub type in_port_t = uint16_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_49 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: sigval_t,
}
#[no_mangle]
pub unsafe extern "C" fn client_main(mut base: *mut event_base,
                                     mut argc: libc::c_int,
                                     mut argv: *mut *mut libc::c_char,
                                     mut flags: libc::c_int) -> libc::c_int {
    let mut cmd: *mut cmd = 0 as *mut cmd;
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    let mut data: *mut msg_command_data = 0 as *mut msg_command_data;
    let mut cmdflags: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ttynam: *const libc::c_char = 0 as *const libc::c_char;
    let mut cwd: *const libc::c_char = 0 as *const libc::c_char;
    let mut ppid: pid_t = 0;
    let mut msg: msgtype = 0 as msgtype;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut tio: termios =
        termios{c_iflag: 0,
                c_oflag: 0,
                c_cflag: 0,
                c_lflag: 0,
                c_line: 0,
                c_cc: [0; 32],
                c_ispeed: 0,
                c_ospeed: 0,};
    let mut saved_tio: termios =
        termios{c_iflag: 0,
                c_oflag: 0,
                c_cflag: 0,
                c_lflag: 0,
                c_line: 0,
                c_cc: [0; 32],
                c_ispeed: 0,
                c_ospeed: 0,};
    let mut size: size_t = 0;
    signal(17i32,
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(1i32 as libc::intptr_t));
    client_flags = flags;
    cmdflags = 0i32;
    if shell_command != 0 as *mut libc::c_void as *const libc::c_char {
        msg = MSG_SHELL;
        cmdflags = 1i32
    } else if argc == 0i32 {
        msg = MSG_COMMAND;
        cmdflags = 1i32
    } else {
        msg = MSG_COMMAND;
        cmdlist =
            cmd_list_parse(argc, argv, 0 as *const libc::c_char,
                           0i32 as u_int,
                           &mut cause as *mut *mut libc::c_char);
        if cmdlist != 0 as *mut libc::c_void as *mut cmd_list {
            cmd = (*(&mut (*cmdlist).list as *mut unnamed)).tqh_first;
            while cmd != 0 as *mut libc::c_void as *mut cmd {
                if 0 != (*(*cmd).entry).flags & 1i32 { cmdflags |= 1i32 }
                cmd = (*cmd).qentry.tqe_next
            }
            cmd_list_free(cmdlist);
        }
    }
    client_proc =
        proc_start(b"client\x00" as *const u8 as *const libc::c_char);
    proc_set_signals(client_proc, Some(client_signal));
    fd = client_connect(base, socket_path, cmdflags & 1i32);
    if fd == 1i32.wrapping_neg() {
        if *__errno_location() == 111i32 {
            fprintf(stderr,
                    b"no server running on %s\n\x00" as *const u8 as
                        *const libc::c_char, socket_path);
        } else {
            fprintf(stderr,
                    b"error connecting to %s (%s)\n\x00" as *const u8 as
                        *const libc::c_char, socket_path,
                    strerror(*__errno_location()));
        }
        return 1i32
    } else {
        client_peer =
            proc_add_peer(client_proc, fd, Some(client_dispatch),
                          0 as *mut libc::c_void);
        cwd = getenv(b"PWD\x00" as *const u8 as *const libc::c_char);
        if cwd == 0 as *mut libc::c_void as *const libc::c_char &&
               {
                   cwd =
                       getcwd(path.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 4096]>() as
                                  libc::c_ulong);
                   cwd == 0 as *mut libc::c_void as *const libc::c_char
               } &&
               {
                   cwd = find_home();
                   cwd == 0 as *mut libc::c_void as *const libc::c_char
               } {
            cwd = b"/\x00" as *const u8 as *const libc::c_char
        }
        ttynam = ttyname(0i32);
        if ttynam == 0 as *mut libc::c_void as *const libc::c_char {
            ttynam = b"\x00" as *const u8 as *const libc::c_char
        }
        if 0i32 != 0i32 {
            fatal(b"pledge failed\x00" as *const u8 as *const libc::c_char);
        } else {
            if ptm_fd != 1i32.wrapping_neg() { close(ptm_fd); }
            options_free(global_options);
            options_free(global_s_options);
            options_free(global_w_options);
            environ_free(global_environ);
            setblocking(0i32, 0i32);
            event_set(&mut client_stdin as *mut event, 0i32,
                      (2i32 | 16i32) as libc::c_short,
                      Some(client_stdin_callback), 0 as *mut libc::c_void);
            if 0 != client_flags & 16384i32 {
                if tcgetattr(0i32, &mut saved_tio as *mut termios) != 0i32 {
                    fprintf(stderr,
                            b"tcgetattr failed: %s\n\x00" as *const u8 as
                                *const libc::c_char,
                            strerror(*__errno_location()));
                    return 1i32
                } else {
                    cfmakeraw(&mut tio as *mut termios);
                    tio.c_iflag = (256i32 | 2048i32) as tcflag_t;
                    tio.c_oflag = (1i32 | 4i32) as tcflag_t;
                    tio.c_cflag = (128i32 | 48i32 | 1024i32) as tcflag_t;
                    tio.c_cc[6usize] = 1i32 as cc_t;
                    tio.c_cc[5usize] = 0i32 as cc_t;
                    cfsetispeed(&mut tio as *mut termios,
                                cfgetispeed(&mut saved_tio as *mut termios));
                    cfsetospeed(&mut tio as *mut termios,
                                cfgetospeed(&mut saved_tio as *mut termios));
                    tcsetattr(0i32, 0i32, &mut tio as *mut termios);
                }
            }
            client_send_identify(ttynam, cwd);
            if msg as libc::c_uint ==
                   MSG_COMMAND as libc::c_int as libc::c_uint {
                size = 0i32 as size_t;
                i = 0i32;
                while i < argc {
                    size =
                        (size as
                             libc::c_ulong).wrapping_add(strlen(*argv.offset(i
                                                                                 as
                                                                                 isize)).wrapping_add(1i32
                                                                                                          as
                                                                                                          libc::c_ulong))
                            as size_t as size_t;
                    i += 1
                }
                if size >
                       (16384i32 as
                            libc::c_ulong).wrapping_sub(::std::mem::size_of::<msg_command_data>()
                                                            as libc::c_ulong)
                   {
                    fprintf(stderr,
                            b"command too long\n\x00" as *const u8 as
                                *const libc::c_char);
                    return 1i32
                } else {
                    data =
                        xmalloc((::std::mem::size_of::<msg_command_data>() as
                                     libc::c_ulong).wrapping_add(size)) as
                            *mut msg_command_data;
                    (*data).argc = argc;
                    if cmd_pack_argv(argc, argv,
                                     data.offset(1isize) as *mut libc::c_char,
                                     size) != 0i32 {
                        fprintf(stderr,
                                b"command too long\n\x00" as *const u8 as
                                    *const libc::c_char);
                        free(data as *mut libc::c_void);
                        return 1i32
                    } else {
                        size =
                            (size as
                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<msg_command_data>()
                                                                 as
                                                                 libc::c_ulong)
                                as size_t as size_t;
                        if proc_send(client_peer, msg, 1i32.wrapping_neg(),
                                     data as *const libc::c_void, size) !=
                               0i32 {
                            fprintf(stderr,
                                    b"failed to send command\n\x00" as
                                        *const u8 as *const libc::c_char);
                            free(data as *mut libc::c_void);
                            return 1i32
                        } else { free(data as *mut libc::c_void); }
                    }
                }
            } else if msg as libc::c_uint ==
                          MSG_SHELL as libc::c_int as libc::c_uint {
                proc_send(client_peer, msg, 1i32.wrapping_neg(),
                          0 as *const libc::c_void, 0i32 as size_t);
            }
            proc_loop(client_proc, None);
            if client_exittype as libc::c_uint ==
                   MSG_EXEC as libc::c_int as libc::c_uint {
                if 0 != client_flags & 16384i32 {
                    tcsetattr(1i32, 2i32, &mut saved_tio as *mut termios);
                }
                client_exec(client_execshell, client_execcmd);
            } else {
                if 0 != client_attached {
                    if client_exitreason as libc::c_uint !=
                           CLIENT_EXIT_NONE as libc::c_int as libc::c_uint {
                        printf(b"[%s]\n\x00" as *const u8 as
                                   *const libc::c_char,
                               client_exit_message());
                    }
                    ppid = getppid();
                    if client_exittype as libc::c_uint ==
                           MSG_DETACHKILL as libc::c_int as libc::c_uint &&
                           ppid > 1i32 {
                        kill(ppid, 1i32);
                    }
                } else if 0 != client_flags & 16384i32 {
                    if client_exitreason as libc::c_uint !=
                           CLIENT_EXIT_NONE as libc::c_int as libc::c_uint {
                        printf(b"%%exit %s\n\x00" as *const u8 as
                                   *const libc::c_char,
                               client_exit_message());
                    } else {
                        printf(b"%%exit\n\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    printf(b"\x1b\\\x00" as *const u8 as *const libc::c_char);
                    tcsetattr(1i32, 2i32, &mut saved_tio as *mut termios);
                } else if client_exitreason as libc::c_uint !=
                              CLIENT_EXIT_NONE as libc::c_int as libc::c_uint
                 {
                    fprintf(stderr,
                            b"%s\n\x00" as *const u8 as *const libc::c_char,
                            client_exit_message());
                }
                setblocking(0i32, 1i32);
                return client_exitval
            }
        }
    };
}
static mut client_exitval: libc::c_int = unsafe { 0 };
unsafe extern "C" fn client_exit_message() -> *const libc::c_char {
    static mut msg: [libc::c_char; 256] = unsafe { [0; 256] };
    match client_exitreason as libc::c_uint {
        1 => {
            if client_exitsession !=
                   0 as *mut libc::c_void as *const libc::c_char {
                xsnprintf(msg.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 256]>() as
                              libc::c_ulong,
                          b"detached (from session %s)\x00" as *const u8 as
                              *const libc::c_char, client_exitsession);
                return msg.as_mut_ptr()
            } else {
                return b"detached\x00" as *const u8 as *const libc::c_char
            }
        }
        2 => {
            if client_exitsession !=
                   0 as *mut libc::c_void as *const libc::c_char {
                xsnprintf(msg.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 256]>() as
                              libc::c_ulong,
                          b"detached and SIGHUP (from session %s)\x00" as
                              *const u8 as *const libc::c_char,
                          client_exitsession);
                return msg.as_mut_ptr()
            } else {
                return b"detached and SIGHUP\x00" as *const u8 as
                           *const libc::c_char
            }
        }
        3 => { return b"lost tty\x00" as *const u8 as *const libc::c_char }
        4 => { return b"terminated\x00" as *const u8 as *const libc::c_char }
        5 => { return b"lost server\x00" as *const u8 as *const libc::c_char }
        6 => { return b"exited\x00" as *const u8 as *const libc::c_char }
        7 => {
            return b"server exited\x00" as *const u8 as *const libc::c_char
        }
        0 | _ => {
            return b"unknown reason\x00" as *const u8 as *const libc::c_char
        }
    };
}
static mut client_exitsession: *const libc::c_char =
    unsafe { 0 as *const libc::c_char };
static mut client_exitreason: unnamed_25 = unsafe { CLIENT_EXIT_NONE };
static mut client_flags: libc::c_int = unsafe { 0 };
static mut client_exittype: msgtype = unsafe { 0 as msgtype };
static mut client_attached: libc::c_int = unsafe { 0 };
static mut client_execcmd: *const libc::c_char =
    unsafe { 0 as *const libc::c_char };
static mut client_execshell: *const libc::c_char =
    unsafe { 0 as *const libc::c_char };
unsafe extern "C" fn client_exec(mut shell: *const libc::c_char,
                                 mut shellcmd: *const libc::c_char) -> ! {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut argv0: *mut libc::c_char = 0 as *mut libc::c_char;
    log_debug(b"shell %s, command %s\x00" as *const u8 as *const libc::c_char,
              shell, shellcmd);
    ptr = strrchr(shell, 47);
    if ptr != 0 as *mut libc::c_void as *const libc::c_char &&
           *ptr.offset(1isize) as libc::c_int != 0 {
        name = ptr.offset(1isize)
    } else { name = shell }
    if 0 != client_flags & 2i32 {
        xasprintf(&mut argv0 as *mut *mut libc::c_char,
                  b"-%s\x00" as *const u8 as *const libc::c_char, name);
    } else {
        xasprintf(&mut argv0 as *mut *mut libc::c_char,
                  b"%s\x00" as *const u8 as *const libc::c_char, name);
    }
    setenv(b"SHELL\x00" as *const u8 as *const libc::c_char, shell, 1i32);
    proc_clear_signals(client_proc, 1i32);
    setblocking(0i32, 1i32);
    setblocking(1i32, 1i32);
    setblocking(2i32, 1i32);
    closefrom(2i32 + 1i32);
    execl(shell, argv0, b"-c\x00" as *const u8 as *const libc::c_char,
          shellcmd, 0 as *mut libc::c_void as *mut libc::c_char);
    fatal(b"execl failed\x00" as *const u8 as *const libc::c_char);
}
static mut client_proc: *mut tmuxproc =
    unsafe { 0 as *const tmuxproc as *mut tmuxproc };
static mut client_peer: *mut tmuxpeer =
    unsafe { 0 as *const tmuxpeer as *mut tmuxpeer };
unsafe extern "C" fn client_send_identify(mut ttynam: *const libc::c_char,
                                          mut cwd: *const libc::c_char)
 -> () {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ss: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut sslen: size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut flags: libc::c_int = client_flags;
    let mut pid: pid_t = 0;
    proc_send(client_peer, MSG_IDENTIFY_FLAGS, 1i32.wrapping_neg(),
              &mut flags as *mut libc::c_int as *const libc::c_void,
              ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    s = getenv(b"TERM\x00" as *const u8 as *const libc::c_char);
    if s == 0 as *mut libc::c_void as *const libc::c_char {
        s = b"\x00" as *const u8 as *const libc::c_char
    }
    proc_send(client_peer, MSG_IDENTIFY_TERM, 1i32.wrapping_neg(),
              s as *const libc::c_void,
              strlen(s).wrapping_add(1i32 as libc::c_ulong));
    proc_send(client_peer, MSG_IDENTIFY_TTYNAME, 1i32.wrapping_neg(),
              ttynam as *const libc::c_void,
              strlen(ttynam).wrapping_add(1i32 as libc::c_ulong));
    proc_send(client_peer, MSG_IDENTIFY_CWD, 1i32.wrapping_neg(),
              cwd as *const libc::c_void,
              strlen(cwd).wrapping_add(1i32 as libc::c_ulong));
    fd = dup(0i32);
    if fd == 1i32.wrapping_neg() {
        fatal(b"dup failed\x00" as *const u8 as *const libc::c_char);
    } else {
        proc_send(client_peer, MSG_IDENTIFY_STDIN, fd,
                  0 as *const libc::c_void, 0i32 as size_t);
        pid = getpid();
        proc_send(client_peer, MSG_IDENTIFY_CLIENTPID, 1i32.wrapping_neg(),
                  &mut pid as *mut pid_t as *const libc::c_void,
                  ::std::mem::size_of::<pid_t>() as libc::c_ulong);
        ss = environ;
        while *ss != 0 as *mut libc::c_void as *mut libc::c_char {
            sslen = strlen(*ss).wrapping_add(1i32 as libc::c_ulong);
            if !(sslen >
                     (16384i32 as
                          libc::c_ulong).wrapping_sub(::std::mem::size_of::<imsg_hdr>()
                                                          as libc::c_ulong)) {
                proc_send(client_peer, MSG_IDENTIFY_ENVIRON,
                          1i32.wrapping_neg(), *ss as *const libc::c_void,
                          sslen);
            }
            ss = ss.offset(1isize)
        }
        proc_send(client_peer, MSG_IDENTIFY_DONE, 1i32.wrapping_neg(),
                  0 as *const libc::c_void, 0i32 as size_t);
        return;
    };
}
unsafe extern "C" fn client_stdin_callback(mut fd: libc::c_int,
                                           mut events: libc::c_short,
                                           mut arg: *mut libc::c_void) -> () {
    let mut data: msg_stdin_data = msg_stdin_data{size: 0, data: [0; 8192],};
    data.size =
        read(0i32, data.data.as_mut_ptr() as *mut libc::c_void,
             ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong);
    if data.size < 0i32 as libc::c_long &&
           (*__errno_location() == 4i32 || *__errno_location() == 11i32) {
        return
    } else {
        proc_send(client_peer, MSG_STDIN, 1i32.wrapping_neg(),
                  &mut data as *mut msg_stdin_data as *const libc::c_void,
                  ::std::mem::size_of::<msg_stdin_data>() as libc::c_ulong);
        if data.size <= 0i32 as libc::c_long {
            event_del(&mut client_stdin as *mut event);
        }
        return;
    };
}
static mut client_stdin: event =
    unsafe {
        event{ev_active_next:
                  unnamed_4{tqe_next: 0 as *const event as *mut event,
                            tqe_prev:
                                0 as *const *mut event as *mut *mut event,},
              ev_next:
                  unnamed_26{tqe_next: 0 as *const event as *mut event,
                             tqe_prev:
                                 0 as *const *mut event as *mut *mut event,},
              ev_timeout_pos:
                  unnamed_13{ev_next_with_common_timeout:
                                 unnamed_41{tqe_next:
                                                0 as *const event as
                                                    *mut event,
                                            tqe_prev:
                                                0 as *const *mut event as
                                                    *mut *mut event,},},
              ev_fd: 0,
              ev_base: 0 as *const event_base as *mut event_base,
              _ev:
                  unnamed_48{ev_io:
                                 unnamed_33{ev_io_next:
                                                unnamed_2{tqe_next:
                                                              0 as
                                                                  *const event
                                                                  as
                                                                  *mut event,
                                                          tqe_prev:
                                                              0 as
                                                                  *const *mut event
                                                                  as
                                                                  *mut *mut event,},
                                            ev_timeout:
                                                timeval{tv_sec: 0,
                                                        tv_usec: 0,},},},
              ev_events: 0,
              ev_res: 0,
              ev_flags: 0,
              ev_pri: 0,
              ev_closure: 0,
              ev_timeout: timeval{tv_sec: 0, tv_usec: 0,},
              ev_callback: None,
              ev_arg: 0 as *const libc::c_void as *mut libc::c_void,}
    };
unsafe extern "C" fn client_dispatch(mut imsg: *mut imsg,
                                     mut arg: *mut libc::c_void) -> () {
    if imsg == 0 as *mut libc::c_void as *mut imsg {
        client_exitreason = CLIENT_EXIT_LOST_SERVER;
        client_exitval = 1i32;
        proc_exit(client_proc);
        return
    } else {
        if 0 != client_attached {
            client_dispatch_attached(imsg);
        } else { client_dispatch_wait(imsg); }
        return;
    };
}
unsafe extern "C" fn client_dispatch_wait(mut imsg: *mut imsg) -> () {
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut datalen: ssize_t = 0;
    let mut stdoutdata: msg_stdout_data =
        msg_stdout_data{size: 0, data: [0; 8192],};
    let mut stderrdata: msg_stderr_data =
        msg_stderr_data{size: 0, data: [0; 8192],};
    let mut retval: libc::c_int = 0;
    static mut pledge_applied: libc::c_int = unsafe { 0 };
    if 0 == pledge_applied {
        if 0i32 != 0i32 {
            fatal(b"pledge failed\x00" as *const u8 as *const libc::c_char);
        } else { pledge_applied = 1i32 }
    }
    data = (*imsg).data as *mut libc::c_char;
    datalen =
        ((*imsg).hdr.len as
             libc::c_ulong).wrapping_sub(::std::mem::size_of::<imsg_hdr>() as
                                             libc::c_ulong) as ssize_t;
    match (*imsg).hdr.type_0 {
        203 | 210 => {
            if datalen as libc::c_ulong !=
                   ::std::mem::size_of::<libc::c_int>() as libc::c_ulong &&
                   datalen != 0i32 as libc::c_long {
                fatalx(b"bad MSG_EXIT size\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                if datalen as libc::c_ulong ==
                       ::std::mem::size_of::<libc::c_int>() as libc::c_ulong {
                    memcpy(&mut retval as *mut libc::c_int as
                               *mut libc::c_void, data as *const libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong);
                    client_exitval = retval
                }
                proc_exit(client_proc);
            }
        }
        207 => {
            if datalen != 0i32 as libc::c_long {
                fatalx(b"bad MSG_READY size\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                event_del(&mut client_stdin as *mut event);
                client_attached = 1i32;
                proc_send(client_peer, MSG_RESIZE, 1i32.wrapping_neg(),
                          0 as *const libc::c_void, 0i32 as size_t);
            }
        }
        212 => {
            if datalen != 0i32 as libc::c_long {
                fatalx(b"bad MSG_STDIN size\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                event_add(&mut client_stdin as *mut event,
                          0 as *const timeval);
            }
        }
        213 => {
            if datalen as libc::c_ulong !=
                   ::std::mem::size_of::<msg_stdout_data>() as libc::c_ulong {
                fatalx(b"bad MSG_STDOUT size\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                memcpy(&mut stdoutdata as *mut msg_stdout_data as
                           *mut libc::c_void, data as *const libc::c_void,
                       ::std::mem::size_of::<msg_stdout_data>() as
                           libc::c_ulong);
                client_write(1i32, stdoutdata.data.as_mut_ptr(),
                             stdoutdata.size as size_t);
            }
        }
        211 => {
            if datalen as libc::c_ulong !=
                   ::std::mem::size_of::<msg_stderr_data>() as libc::c_ulong {
                fatalx(b"bad MSG_STDERR size\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                memcpy(&mut stderrdata as *mut msg_stderr_data as
                           *mut libc::c_void, data as *const libc::c_void,
                       ::std::mem::size_of::<msg_stderr_data>() as
                           libc::c_ulong);
                client_write(2i32, stderrdata.data.as_mut_ptr(),
                             stderrdata.size as size_t);
            }
        }
        12 => {
            if datalen != 0i32 as libc::c_long {
                fatalx(b"bad MSG_VERSION size\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                fprintf(stderr,
                        b"protocol version mismatch (client %d, server %u)\n\x00"
                            as *const u8 as *const libc::c_char, 8i32,
                        (*imsg).hdr.peerid & 255i32 as libc::c_uint);
                client_exitval = 1i32;
                proc_exit(client_proc);
            }
        }
        209 => {
            if datalen == 0i32 as libc::c_long ||
                   *data.offset((datalen - 1i32 as libc::c_long) as isize) as
                       libc::c_int != 0 {
                fatalx(b"bad MSG_SHELL string\x00" as *const u8 as
                           *const libc::c_char);
            } else { client_exec(data, shell_command); }
        }
        201 | 202 => {
            proc_send(client_peer, MSG_EXITING, 1i32.wrapping_neg(),
                      0 as *const libc::c_void, 0i32 as size_t);
        }
        204 => { proc_exit(client_proc); }
        _ => { }
    };
}
unsafe extern "C" fn client_write(mut fd: libc::c_int,
                                  mut data: *const libc::c_char,
                                  mut size: size_t) -> () {
    let mut used: ssize_t = 0;
    log_debug(b"%s: %.*s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"client_write\x00")).as_ptr(),
              size as libc::c_int, data);
    while size != 0i32 as libc::c_ulong {
        used = write(fd, data as *const libc::c_void, size);
        if used == 1i32.wrapping_neg() as libc::c_long {
            if !(*__errno_location() == 4i32 || *__errno_location() == 11i32)
               {
                break ;
            }
        } else {
            data = data.offset(used as isize);
            size =
                (size as libc::c_ulong).wrapping_sub(used as libc::c_ulong) as
                    size_t as size_t
        }
    };
}
unsafe extern "C" fn client_dispatch_attached(mut imsg: *mut imsg) -> () {
    let mut sigact: sigaction =
        sigaction{__sigaction_handler: unnamed_31{sa_handler: None,},
                  sa_mask: sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut datalen: ssize_t = 0;
    data = (*imsg).data as *mut libc::c_char;
    datalen =
        ((*imsg).hdr.len as
             libc::c_ulong).wrapping_sub(::std::mem::size_of::<imsg_hdr>() as
                                             libc::c_ulong) as ssize_t;
    match (*imsg).hdr.type_0 {
        201 | 202 => {
            if datalen == 0i32 as libc::c_long ||
                   *data.offset((datalen - 1i32 as libc::c_long) as isize) as
                       libc::c_int != 0 {
                fatalx(b"bad MSG_DETACH string\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                client_exitsession = xstrdup(data);
                client_exittype = (*imsg).hdr.type_0 as msgtype;
                if (*imsg).hdr.type_0 ==
                       MSG_DETACHKILL as libc::c_int as libc::c_uint {
                    client_exitreason = CLIENT_EXIT_DETACHED_HUP
                } else { client_exitreason = CLIENT_EXIT_DETACHED }
                proc_send(client_peer, MSG_EXITING, 1i32.wrapping_neg(),
                          0 as *const libc::c_void, 0i32 as size_t);
            }
        }
        217 => {
            if datalen == 0i32 as libc::c_long ||
                   *data.offset((datalen - 1i32 as libc::c_long) as isize) as
                       libc::c_int != 0 ||
                   strlen(data).wrapping_add(1i32 as libc::c_ulong) ==
                       datalen as size_t {
                fatalx(b"bad MSG_EXEC string\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                client_execcmd = xstrdup(data);
                client_execshell =
                    xstrdup(data.offset(strlen(data) as
                                            isize).offset(1isize));
                client_exittype = (*imsg).hdr.type_0 as msgtype;
                proc_send(client_peer, MSG_EXITING, 1i32.wrapping_neg(),
                          0 as *const libc::c_void, 0i32 as size_t);
            }
        }
        203 => {
            if datalen != 0i32 as libc::c_long &&
                   datalen as libc::c_ulong !=
                       ::std::mem::size_of::<libc::c_int>() as libc::c_ulong {
                fatalx(b"bad MSG_EXIT size\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                proc_send(client_peer, MSG_EXITING, 1i32.wrapping_neg(),
                          0 as *const libc::c_void, 0i32 as size_t);
                client_exitreason = CLIENT_EXIT_EXITED
            }
        }
        204 => {
            if datalen != 0i32 as libc::c_long {
                fatalx(b"bad MSG_EXITED size\x00" as *const u8 as
                           *const libc::c_char);
            } else { proc_exit(client_proc); }
        }
        210 => {
            if datalen != 0i32 as libc::c_long {
                fatalx(b"bad MSG_SHUTDOWN size\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                proc_send(client_peer, MSG_EXITING, 1i32.wrapping_neg(),
                          0 as *const libc::c_void, 0i32 as size_t);
                client_exitreason = CLIENT_EXIT_SERVER_EXITED;
                client_exitval = 1i32
            }
        }
        214 => {
            if datalen != 0i32 as libc::c_long {
                fatalx(b"bad MSG_SUSPEND size\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                memset(&mut sigact as *mut sigaction as *mut libc::c_void,
                       0i32,
                       ::std::mem::size_of::<sigaction>() as libc::c_ulong);
                sigemptyset(&mut sigact.sa_mask as *mut __sigset_t);
                sigact.sa_flags = 268435456i32;
                sigact.__sigaction_handler.sa_handler = None;
                if sigaction(20i32, &mut sigact as *mut sigaction,
                             0 as *mut sigaction) != 0i32 {
                    fatal(b"sigaction failed\x00" as *const u8 as
                              *const libc::c_char);
                } else { kill(getpid(), 20i32); }
            }
        }
        206 => {
            if datalen == 0i32 as libc::c_long ||
                   *data.offset((datalen - 1i32 as libc::c_long) as isize) as
                       libc::c_int != 0 {
                fatalx(b"bad MSG_LOCK string\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                system(data);
                proc_send(client_peer, MSG_UNLOCK, 1i32.wrapping_neg(),
                          0 as *const libc::c_void, 0i32 as size_t);
            }
        }
        _ => { }
    };
}
unsafe extern "C" fn client_connect(mut base: *mut event_base,
                                    mut path: *const libc::c_char,
                                    mut start_server: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut sa: sockaddr_un = sockaddr_un{sun_family: 0, sun_path: [0; 108],};
    let mut size: size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut lockfd: libc::c_int = 1i32.wrapping_neg();
    let mut locked: libc::c_int = 0i32;
    let mut lockfile: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(&mut sa as *mut sockaddr_un as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong);
    sa.sun_family = 1i32 as sa_family_t;
    size =
        strlcpy(sa.sun_path.as_mut_ptr(), path,
                ::std::mem::size_of::<[libc::c_char; 108]>() as
                    libc::c_ulong);
    if size >= ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong {
        *__errno_location() = 36i32;
        return 1i32.wrapping_neg()
    } else {
        log_debug(b"socket is %s\x00" as *const u8 as *const libc::c_char,
                  path);
        loop  {
            fd = socket(1i32, SOCK_STREAM as libc::c_int, 0i32);
            if fd == 1i32.wrapping_neg() {
                return 1i32.wrapping_neg()
            } else {
                log_debug(b"trying connect\x00" as *const u8 as
                              *const libc::c_char);
                if !(connect(fd,
                             __CONST_SOCKADDR_ARG{__sockaddr__:
                                                      &mut sa as
                                                          *mut sockaddr_un as
                                                          *mut sockaddr,},
                             ::std::mem::size_of::<sockaddr_un>() as
                                 libc::c_ulong as socklen_t) ==
                         1i32.wrapping_neg()) {
                    current_block = 17778012151635330486;
                    break ;
                }
                log_debug(b"connect failed: %s\x00" as *const u8 as
                              *const libc::c_char,
                          strerror(*__errno_location()));
                if *__errno_location() != 111i32 &&
                       *__errno_location() != 2i32 {
                    current_block = 9005509770874399122;
                    break ;
                }
                if 0 == start_server {
                    current_block = 9005509770874399122;
                    break ;
                }
                close(fd);
                if 0 == locked {
                    xasprintf(&mut lockfile as *mut *mut libc::c_char,
                              b"%s.lock\x00" as *const u8 as
                                  *const libc::c_char, path);
                    lockfd = client_get_lock(lockfile);
                    if lockfd < 0i32 {
                        log_debug(b"didn\'t get lock (%d)\x00" as *const u8 as
                                      *const libc::c_char, lockfd);
                        free(lockfile as *mut libc::c_void);
                        lockfile = 0 as *mut libc::c_char;
                        if lockfd == 2i32.wrapping_neg() { continue ; }
                    }
                    log_debug(b"got lock (%d)\x00" as *const u8 as
                                  *const libc::c_char, lockfd);
                    locked = 1i32
                } else if lockfd >= 0i32 && unlink(path) != 0i32 &&
                              *__errno_location() != 2i32 {
                    current_block = 2868539653012386629;
                    break ;
                } else { current_block = 1917311967535052937; break ; }
            }
        }
        match current_block {
            2868539653012386629 => {
                free(lockfile as *mut libc::c_void);
                close(lockfd);
                return 1i32.wrapping_neg()
            }
            9005509770874399122 => {
                if 0 != locked {
                    free(lockfile as *mut libc::c_void);
                    close(lockfd);
                }
                close(fd);
                return 1i32.wrapping_neg()
            }
            1917311967535052937 => {
                fd = server_start(client_proc, base, lockfd, lockfile)
            }
            _ => { }
        }
        if 0 != locked && lockfd >= 0i32 {
            free(lockfile as *mut libc::c_void);
            close(lockfd);
        }
        setblocking(fd, 0i32);
        return fd
    };
}
unsafe extern "C" fn client_get_lock(mut lockfile: *mut libc::c_char)
 -> libc::c_int {
    let mut lockfd: libc::c_int = 0;
    log_debug(b"lock file is %s\x00" as *const u8 as *const libc::c_char,
              lockfile);
    lockfd = open(lockfile, 1i32 | 64i32, 384i32);
    if lockfd == 1i32.wrapping_neg() {
        log_debug(b"open failed: %s\x00" as *const u8 as *const libc::c_char,
                  strerror(*__errno_location()));
        return 1i32.wrapping_neg()
    } else if flock(lockfd, 2i32 | 4i32) == 1i32.wrapping_neg() {
        log_debug(b"flock failed: %s\x00" as *const u8 as *const libc::c_char,
                  strerror(*__errno_location()));
        if *__errno_location() != 11i32 {
            return lockfd
        } else {
            while flock(lockfd, 2i32) == 1i32.wrapping_neg() &&
                      *__errno_location() == 4i32 {
            }
            close(lockfd);
            return 2i32.wrapping_neg()
        }
    } else {
        log_debug(b"flock succeeded\x00" as *const u8 as *const libc::c_char);
        return lockfd
    };
}
unsafe extern "C" fn client_signal(mut sig: libc::c_int) -> () {
    let mut current_block: u64;
    let mut sigact: sigaction =
        sigaction{__sigaction_handler: unnamed_31{sa_handler: None,},
                  sa_mask: sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    let mut status: libc::c_int = 0;
    if sig == 17i32 {
        waitpid(1i32.wrapping_neg(), &mut status as *mut libc::c_int, 1i32);
    } else if 0 == client_attached {
        if sig == 15i32 { proc_exit(client_proc); }
    } else {
        match sig {
            1 => {
                current_block = 4336949444775314641;
                match current_block {
                    5703566159712876446 => {
                        memset(&mut sigact as *mut sigaction as
                                   *mut libc::c_void, 0i32,
                               ::std::mem::size_of::<sigaction>() as
                                   libc::c_ulong);
                        sigemptyset(&mut sigact.sa_mask as *mut __sigset_t);
                        sigact.sa_flags = 268435456i32;
                        sigact.__sigaction_handler.sa_handler =
                            ::std::mem::transmute::<libc::intptr_t,
                                                    __sighandler_t>(1i32 as
                                                                        libc::intptr_t);
                        if sigaction(20i32, &mut sigact as *mut sigaction,
                                     0 as *mut sigaction) != 0i32 {
                            fatal(b"sigaction failed\x00" as *const u8 as
                                      *const libc::c_char);
                        } else {
                            proc_send(client_peer, MSG_WAKEUP,
                                      1i32.wrapping_neg(),
                                      0 as *const libc::c_void,
                                      0i32 as size_t);
                        }
                    }
                    4336949444775314641 => {
                        client_exitreason = CLIENT_EXIT_LOST_TTY;
                        client_exitval = 1i32;
                        proc_send(client_peer, MSG_EXITING,
                                  1i32.wrapping_neg(),
                                  0 as *const libc::c_void, 0i32 as size_t);
                    }
                    17210892384610329727 => {
                        client_exitreason = CLIENT_EXIT_TERMINATED;
                        client_exitval = 1i32;
                        proc_send(client_peer, MSG_EXITING,
                                  1i32.wrapping_neg(),
                                  0 as *const libc::c_void, 0i32 as size_t);
                    }
                    _ => {
                        proc_send(client_peer, MSG_RESIZE,
                                  1i32.wrapping_neg(),
                                  0 as *const libc::c_void, 0i32 as size_t);
                    }
                }
            }
            15 => {
                current_block = 17210892384610329727;
                match current_block {
                    5703566159712876446 => {
                        memset(&mut sigact as *mut sigaction as
                                   *mut libc::c_void, 0i32,
                               ::std::mem::size_of::<sigaction>() as
                                   libc::c_ulong);
                        sigemptyset(&mut sigact.sa_mask as *mut __sigset_t);
                        sigact.sa_flags = 268435456i32;
                        sigact.__sigaction_handler.sa_handler =
                            ::std::mem::transmute::<libc::intptr_t,
                                                    __sighandler_t>(1i32 as
                                                                        libc::intptr_t);
                        if sigaction(20i32, &mut sigact as *mut sigaction,
                                     0 as *mut sigaction) != 0i32 {
                            fatal(b"sigaction failed\x00" as *const u8 as
                                      *const libc::c_char);
                        } else {
                            proc_send(client_peer, MSG_WAKEUP,
                                      1i32.wrapping_neg(),
                                      0 as *const libc::c_void,
                                      0i32 as size_t);
                        }
                    }
                    4336949444775314641 => {
                        client_exitreason = CLIENT_EXIT_LOST_TTY;
                        client_exitval = 1i32;
                        proc_send(client_peer, MSG_EXITING,
                                  1i32.wrapping_neg(),
                                  0 as *const libc::c_void, 0i32 as size_t);
                    }
                    17210892384610329727 => {
                        client_exitreason = CLIENT_EXIT_TERMINATED;
                        client_exitval = 1i32;
                        proc_send(client_peer, MSG_EXITING,
                                  1i32.wrapping_neg(),
                                  0 as *const libc::c_void, 0i32 as size_t);
                    }
                    _ => {
                        proc_send(client_peer, MSG_RESIZE,
                                  1i32.wrapping_neg(),
                                  0 as *const libc::c_void, 0i32 as size_t);
                    }
                }
            }
            28 => {
                current_block = 1432322269928448905;
                match current_block {
                    5703566159712876446 => {
                        memset(&mut sigact as *mut sigaction as
                                   *mut libc::c_void, 0i32,
                               ::std::mem::size_of::<sigaction>() as
                                   libc::c_ulong);
                        sigemptyset(&mut sigact.sa_mask as *mut __sigset_t);
                        sigact.sa_flags = 268435456i32;
                        sigact.__sigaction_handler.sa_handler =
                            ::std::mem::transmute::<libc::intptr_t,
                                                    __sighandler_t>(1i32 as
                                                                        libc::intptr_t);
                        if sigaction(20i32, &mut sigact as *mut sigaction,
                                     0 as *mut sigaction) != 0i32 {
                            fatal(b"sigaction failed\x00" as *const u8 as
                                      *const libc::c_char);
                        } else {
                            proc_send(client_peer, MSG_WAKEUP,
                                      1i32.wrapping_neg(),
                                      0 as *const libc::c_void,
                                      0i32 as size_t);
                        }
                    }
                    4336949444775314641 => {
                        client_exitreason = CLIENT_EXIT_LOST_TTY;
                        client_exitval = 1i32;
                        proc_send(client_peer, MSG_EXITING,
                                  1i32.wrapping_neg(),
                                  0 as *const libc::c_void, 0i32 as size_t);
                    }
                    17210892384610329727 => {
                        client_exitreason = CLIENT_EXIT_TERMINATED;
                        client_exitval = 1i32;
                        proc_send(client_peer, MSG_EXITING,
                                  1i32.wrapping_neg(),
                                  0 as *const libc::c_void, 0i32 as size_t);
                    }
                    _ => {
                        proc_send(client_peer, MSG_RESIZE,
                                  1i32.wrapping_neg(),
                                  0 as *const libc::c_void, 0i32 as size_t);
                    }
                }
            }
            18 => {
                current_block = 5703566159712876446;
                match current_block {
                    5703566159712876446 => {
                        memset(&mut sigact as *mut sigaction as
                                   *mut libc::c_void, 0i32,
                               ::std::mem::size_of::<sigaction>() as
                                   libc::c_ulong);
                        sigemptyset(&mut sigact.sa_mask as *mut __sigset_t);
                        sigact.sa_flags = 268435456i32;
                        sigact.__sigaction_handler.sa_handler =
                            ::std::mem::transmute::<libc::intptr_t,
                                                    __sighandler_t>(1i32 as
                                                                        libc::intptr_t);
                        if sigaction(20i32, &mut sigact as *mut sigaction,
                                     0 as *mut sigaction) != 0i32 {
                            fatal(b"sigaction failed\x00" as *const u8 as
                                      *const libc::c_char);
                        } else {
                            proc_send(client_peer, MSG_WAKEUP,
                                      1i32.wrapping_neg(),
                                      0 as *const libc::c_void,
                                      0i32 as size_t);
                        }
                    }
                    4336949444775314641 => {
                        client_exitreason = CLIENT_EXIT_LOST_TTY;
                        client_exitval = 1i32;
                        proc_send(client_peer, MSG_EXITING,
                                  1i32.wrapping_neg(),
                                  0 as *const libc::c_void, 0i32 as size_t);
                    }
                    17210892384610329727 => {
                        client_exitreason = CLIENT_EXIT_TERMINATED;
                        client_exitval = 1i32;
                        proc_send(client_peer, MSG_EXITING,
                                  1i32.wrapping_neg(),
                                  0 as *const libc::c_void, 0i32 as size_t);
                    }
                    _ => {
                        proc_send(client_peer, MSG_RESIZE,
                                  1i32.wrapping_neg(),
                                  0 as *const libc::c_void, 0i32 as size_t);
                    }
                }
            }
            _ => { }
        }
    };
}

