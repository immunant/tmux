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
    pub type bufferevent_ops;
    pub type tmuxproc;
    pub type args_entry;
    pub type input_ctx;
    pub type evbuffer;
    pub type format_job_tree;
    pub type hooks;
    pub type _IO_FILE_plus;
    pub type screen_titles;
    pub type environ;
    pub type tmuxpeer;
    pub type options;
    pub type format_tree;
    pub type tty_code;
    pub type event_base;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int,
              __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn socketpair(__domain: libc::c_int, __type: libc::c_int,
                  __protocol: libc::c_int, __fds: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t)
     -> libc::c_int;
    #[no_mangle]
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn accept(__fd: libc::c_int, __addr: *mut sockaddr,
              __addr_len: *mut socklen_t) -> libc::c_int;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn umask(__mask: __mode_t) -> __mode_t;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strsignal(__sig: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn killpg(__pgrp: __pid_t, __sig: libc::c_int) -> libc::c_int;
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
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
               __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
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
    fn event_reinit(base: *mut event_base) -> libc::c_int;
    #[no_mangle]
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
    #[no_mangle]
    fn event_initialized(ev: *const event) -> libc::c_int;
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
    fn exit(_: libc::c_int) -> !;
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
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn fork() -> __pid_t;
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
    fn daemon(__nochdir: libc::c_int, __noclose: libc::c_int) -> libc::c_int;
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
    fn proc_send(_: *mut tmuxpeer, _: msgtype, _: libc::c_int,
                 _: *const libc::c_void, _: size_t) -> libc::c_int;
    #[no_mangle]
    fn proc_start(_: *const libc::c_char) -> *mut tmuxproc;
    #[no_mangle]
    fn proc_loop(_: *mut tmuxproc,
                 _: Option<unsafe extern "C" fn() -> libc::c_int>) -> ();
    #[no_mangle]
    fn proc_set_signals(_: *mut tmuxproc,
                        _: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>)
     -> ();
    #[no_mangle]
    fn proc_clear_signals(_: *mut tmuxproc, _: libc::c_int) -> ();
    #[no_mangle]
    fn proc_toggle_log(_: *mut tmuxproc) -> ();
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn start_cfg() -> ();
    #[no_mangle]
    fn notify_client(_: *const libc::c_char, _: *mut client) -> ();
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn job_died(_: *mut job, _: libc::c_int) -> ();
    #[no_mangle]
    fn tty_create_log() -> ();
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn cmd_find_clear_state(_: *mut cmd_find_state, _: libc::c_int) -> ();
    #[no_mangle]
    fn cmd_find_valid_state(_: *mut cmd_find_state) -> libc::c_int;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn cmdq_get_callback1(_: *const libc::c_char, _: cmdq_cb,
                          _: *mut libc::c_void) -> *mut cmdq_item;
    #[no_mangle]
    fn cmdq_append(_: *mut client, _: *mut cmdq_item) -> ();
    #[no_mangle]
    fn cmdq_next(_: *mut client) -> u_int;
    #[no_mangle]
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn cmd_wait_for_flush() -> ();
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    fn key_bindings_init() -> ();
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    fn status_prompt_save_history() -> ();
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    fn server_client_loop() -> ();
    #[no_mangle]
    fn server_client_create(_: libc::c_int) -> *mut client;
    #[no_mangle]
    fn fatal(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    #[no_mangle]
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    #[no_mangle]
    static mut session_groups: session_groups;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    fn log_get_level() -> libc::c_int;
    #[no_mangle]
    fn server_destroy_pane(_: *mut window_pane, _: libc::c_int) -> ();
    #[no_mangle]
    fn window_pane_destroy_ready(_: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn windows_RB_NEXT(_: *mut window) -> *mut window;
    #[no_mangle]
    fn windows_RB_MINMAX(_: *mut windows, _: libc::c_int) -> *mut window;
    #[no_mangle]
    fn session_destroy(_: *mut session, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn server_client_lost(_: *mut client) -> ();
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    static grid_default_cell: grid_cell;
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
}
pub const TTY_VT320: unnamed_20 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub type __blksize_t = libc::c_long;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type bitstr_t = libc::c_uchar;
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const PROMPT_ENTRY: unnamed_24 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_5,
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
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_42,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
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
pub type sa_family_t = libc::c_ushort;
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub type key_code = libc::c_ulonglong;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_32,
    pub ev_next: unnamed_23,
    pub ev_timeout_pos: unnamed_28,
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
pub const LINE_SEL_RIGHT_LEFT: unnamed_5 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub const MSG_STDIN: msgtype = 212;
pub type pid_t = __pid_t;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
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
pub type cmd_retval = libc::c_int;
pub const JOB_CLOSED: unnamed_21 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub type unnamed_5 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub type u_int = __u_int;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const MSG_DETACH: msgtype = 201;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub type __off64_t = libc::c_long;
pub const MSG_RESIZE: msgtype = 208;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const MSG_COMMAND: msgtype = 200;
pub type __blkcnt_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_8 {
    __in: libc::c_int,
    __i: libc::c_int,
}
pub type __gid_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
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
    pub entry: unnamed_39,
    pub wentry: unnamed_17,
    pub sentry: unnamed_18,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const MSG_VERSION: msgtype = 12;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_35,
}
pub type __uid_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const MSG_EXEC: msgtype = 217;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const MSG_READY: msgtype = 207;
pub type msgtype = libc::c_uint;
pub const LINE_SEL_NONE: unnamed_5 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type u_short = __u_short;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub type __sigset_t = sigset_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub const SOCK_NONBLOCK: __socket_type = 2048;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const MSG_DETACHKILL: msgtype = 202;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_12 {
    __in: libc::c_int,
    __i: libc::c_int,
}
pub type __syscall_slong_t = libc::c_long;
pub const MSG_SUSPEND: msgtype = 214;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub type time_t = __time_t;
pub type uint8_t = libc::c_uchar;
pub type __socket_type = libc::c_uint;
pub const TTY_UNKNOWN: unnamed_20 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_14 {
    __in: libc::c_int,
    __i: libc::c_int,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub type __u_int = libc::c_uint;
pub const MSG_IDENTIFY_CWD: msgtype = 108;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const MSG_UNLOCK: msgtype = 215;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub type __dev_t = libc::c_ulong;
pub const CMDQ_CALLBACK: cmdq_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
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
pub struct unnamed_17 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_25,
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
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_4,
    pub entry: unnamed_7,
}
pub const MSG_IDENTIFY_STDIN: msgtype = 104;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub const MSG_SHELL: msgtype = 209;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __u_short = libc::c_ushort;
pub type size_t = libc::c_ulong;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub type unnamed_20 = libc::c_uint;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub type __off_t = libc::c_long;
pub type options_table_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub type unnamed_21 = libc::c_uint;
pub const MSG_IDENTIFY_DONE: msgtype = 106;
pub type cmdq_type = libc::c_uint;
pub const MSG_STDERR: msgtype = 211;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const MSG_IDENTIFY_OLDCWD: msgtype = 103;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
    pub entry: unnamed_38,
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
pub const TTY_VT101: unnamed_20 = 1;
pub type unnamed_24 = libc::c_uint;
pub const CMDQ_COMMAND: cmdq_type = 0;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_25 {
    offset: u_int,
    data: unnamed_6,
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
    pub term_type: unnamed_20,
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
pub const JOB_DEAD: unnamed_21 = 1;
pub const MSG_IDENTIFY_CLIENTPID: msgtype = 107;
pub const TTY_VT100: unnamed_20 = 0;
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
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const MSG_EXITED: msgtype = 204;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub type uint32_t = libc::c_uint;
pub type __timezone_ptr_t = *mut timezone;
pub const MSG_IDENTIFY_ENVIRON: msgtype = 105;
pub const MSG_IDENTIFY_TERM: msgtype = 101;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_26 {
    ev_io: unnamed_34,
    ev_signal: unnamed_41,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_30,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_27 {
    __in: libc::c_int,
    __i: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const MSG_WAKEUP: msgtype = 216;
pub const TTY_VT102: unnamed_20 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub const MSG_STDOUT: msgtype = 213;
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
pub type socklen_t = __socklen_t;
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
pub const SOCK_RAW: __socket_type = 3;
pub const TTY_VT420: unnamed_20 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_28 {
    ev_next_with_common_timeout: unnamed_10,
    min_heap_idx: libc::c_int,
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
    pub winlinks: unnamed_22,
    pub entry: unnamed_19,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
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
    pub message_log: unnamed_40,
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
    pub entry: unnamed_43,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_21,
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
    pub entry: unnamed_33,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type u_char = __u_char;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const SOCK_DCCP: __socket_type = 6;
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
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_13,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type options_table_scope = libc::c_uint;
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
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const MSG_SHUTDOWN: msgtype = 210;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const SOCK_DGRAM: __socket_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_9,
}
pub type __u_char = libc::c_uchar;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type __nlink_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_11,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const MSG_EXITING: msgtype = 205;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const PROMPT_COMMAND: unnamed_24 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_2,
}
pub type mode_t = __mode_t;
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
    pub entry: unnamed_15,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub ev_io_next: unnamed_29,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_37 {
    __in: libc::c_int,
    __i: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const TTY_VT220: unnamed_20 = 3;
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const JOB_RUNNING: unnamed_21 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_40 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const SOCK_PACKET: __socket_type = 10;
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
    pub gentry: unnamed_36,
    pub entry: unnamed_31,
}
pub type tcflag_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub const SOCK_RDM: __socket_type = 4;
pub const LINE_SEL_LEFT_RIGHT: unnamed_5 = 1;
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
    pub entry: unnamed_0,
    pub tree_entry: unnamed_1,
}
pub type __socklen_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_41 {
    pub ev_signal_next: unnamed_16,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const MSG_EXIT: msgtype = 203;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_42 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const MSG_IDENTIFY_TTYNAME: msgtype = 102;
pub const SOCK_STREAM: __socket_type = 1;
pub const MSG_LOCK: msgtype = 206;
pub const MSG_IDENTIFY_FLAGS: msgtype = 100;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_43 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
#[no_mangle]
pub unsafe extern "C" fn server_set_marked(mut s: *mut session,
                                           mut wl: *mut winlink,
                                           mut wp: *mut window_pane) -> () {
    cmd_find_clear_state(&mut marked_pane as *mut cmd_find_state, 0i32);
    marked_pane.s = s;
    marked_pane.wl = wl;
    marked_pane.w = (*wl).window;
    marked_pane.wp = wp;
}
#[no_mangle]
pub unsafe extern "C" fn server_clear_marked() -> () {
    cmd_find_clear_state(&mut marked_pane as *mut cmd_find_state, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn server_is_marked(mut s: *mut session,
                                          mut wl: *mut winlink,
                                          mut wp: *mut window_pane)
 -> libc::c_int {
    if s == 0 as *mut libc::c_void as *mut session ||
           wl == 0 as *mut libc::c_void as *mut winlink ||
           wp == 0 as *mut libc::c_void as *mut window_pane {
        return 0i32
    } else if marked_pane.s != s || marked_pane.wl != wl {
        return 0i32
    } else if marked_pane.wp != wp {
        return 0i32
    } else { return server_check_marked() };
}
#[no_mangle]
pub unsafe extern "C" fn server_check_marked() -> libc::c_int {
    return cmd_find_valid_state(&mut marked_pane as *mut cmd_find_state);
}
#[no_mangle]
pub unsafe extern "C" fn server_start(mut client: *mut tmuxproc,
                                      mut base: *mut event_base,
                                      mut lockfd: libc::c_int,
                                      mut lockfile: *mut libc::c_char)
 -> libc::c_int {
    let mut pair: [libc::c_int; 2] = [0; 2];
    let mut job: *mut job = 0 as *mut job;
    let mut set: sigset_t = sigset_t{__val: [0; 16],};
    let mut oldset: sigset_t = sigset_t{__val: [0; 16],};
    let mut c: *mut client = 0 as *mut client;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    if socketpair(1i32, SOCK_STREAM as libc::c_int, 0i32, pair.as_mut_ptr())
           != 0i32 {
        fatal(b"socketpair failed\x00" as *const u8 as *const libc::c_char);
    } else {
        sigfillset(&mut set as *mut sigset_t);
        sigprocmask(0i32, &mut set as *mut sigset_t,
                    &mut oldset as *mut sigset_t);
        match fork() {
            -1 => {
                fatal(b"fork failed\x00" as *const u8 as *const libc::c_char);
            }
            0 => {
                close(pair[0usize]);
                if daemon(1i32, 0i32) != 0i32 {
                    fatal(b"daemon failed\x00" as *const u8 as
                              *const libc::c_char);
                } else {
                    proc_clear_signals(client, 0i32);
                    if event_reinit(base) != 0i32 {
                        fatalx(b"event_reinit failed\x00" as *const u8 as
                                   *const libc::c_char);
                    } else {
                        server_proc =
                            proc_start(b"server\x00" as *const u8 as
                                           *const libc::c_char);
                        proc_set_signals(server_proc, Some(server_signal));
                        sigprocmask(2i32, &mut oldset as *mut sigset_t,
                                    0 as *mut sigset_t);
                        if log_get_level() > 1i32 { tty_create_log(); }
                        if 0i32 != 0i32 {
                            fatal(b"pledge failed\x00" as *const u8 as
                                      *const libc::c_char);
                        } else {
                            loop  {
                                let ref mut fresh0 =
                                    (*(&mut windows as
                                           *mut windows)).rbh_root;
                                *fresh0 = 0 as *mut window;
                                if !(0 != 0i32) { break ; }
                            }
                            loop  {
                                let ref mut fresh1 =
                                    (*(&mut all_window_panes as
                                           *mut window_pane_tree)).rbh_root;
                                *fresh1 = 0 as *mut window_pane;
                                if !(0 != 0i32) { break ; }
                            }
                            loop  {
                                let ref mut fresh2 =
                                    (*(&mut clients as
                                           *mut clients)).tqh_first;
                                *fresh2 = 0 as *mut client;
                                let ref mut fresh3 =
                                    (*(&mut clients as
                                           *mut clients)).tqh_last;
                                *fresh3 =
                                    &mut (*(&mut clients as
                                                *mut clients)).tqh_first as
                                        *mut *mut client;
                                if !(0 != 0i32) { break ; }
                            }
                            loop  {
                                let ref mut fresh4 =
                                    (*(&mut sessions as
                                           *mut sessions)).rbh_root;
                                *fresh4 = 0 as *mut session;
                                if !(0 != 0i32) { break ; }
                            }
                            loop  {
                                let ref mut fresh5 =
                                    (*(&mut session_groups as
                                           *mut session_groups)).rbh_root;
                                *fresh5 = 0 as *mut session_group;
                                if !(0 != 0i32) { break ; }
                            }
                            key_bindings_init();
                            gettimeofday(&mut start_time as *mut timeval,
                                         0 as *mut timezone);
                            server_fd =
                                server_create_socket(&mut cause as
                                                         *mut *mut libc::c_char);
                            if server_fd != 1i32.wrapping_neg() {
                                server_update_socket();
                            }
                            c = server_client_create(pair[1usize]);
                            if lockfd >= 0i32 {
                                unlink(lockfile);
                                free(lockfile as *mut libc::c_void);
                                close(lockfd);
                            }
                            if cause !=
                                   0 as *mut libc::c_void as *mut libc::c_char
                               {
                                cmdq_append(c,
                                            cmdq_get_callback1(b"server_start_error\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               Some(server_start_error),
                                                               cause as
                                                                   *mut libc::c_void));
                                (*c).flags |= 4i32
                            }
                            start_cfg();
                            server_add_accept(0i32);
                            proc_loop(server_proc, Some(server_loop));
                            job = (*(&mut all_jobs as *mut joblist)).lh_first;
                            while job != 0 as *mut libc::c_void as *mut job {
                                if (*job).pid != 1i32.wrapping_neg() {
                                    kill((*job).pid, 15i32);
                                }
                                job = (*job).entry.le_next
                            }
                            status_prompt_save_history();
                            exit(0i32);
                        }
                    }
                }
            }
            _ => {
                sigprocmask(2i32, &mut oldset as *mut sigset_t,
                            0 as *mut sigset_t);
                close(pair[1usize]);
                return pair[0usize]
            }
        }
    };
}
unsafe extern "C" fn server_loop() -> libc::c_int {
    let mut current_block: u64;
    let mut c: *mut client = 0 as *mut client;
    let mut items: u_int = 0;
    let mut job: *mut job = 0 as *mut job;
    loop  {
        items = cmdq_next(0 as *mut client);
        c = (*(&mut clients as *mut clients)).tqh_first;
        while c != 0 as *mut libc::c_void as *mut client {
            if 0 != (*c).flags & 262144i32 {
                items =
                    (items as libc::c_uint).wrapping_add(cmdq_next(c)) as
                        u_int as u_int
            }
            c = (*c).entry.tqe_next
        }
        if !(items != 0i32 as libc::c_uint) { break ; }
    }
    server_client_loop();
    if 0 ==
           options_get_number(global_options,
                              b"exit-empty\x00" as *const u8 as
                                  *const libc::c_char) && 0 == server_exit {
        return 0i32
    } else {
        if 0 ==
               options_get_number(global_options,
                                  b"exit-unattached\x00" as *const u8 as
                                      *const libc::c_char) {
            if 0 !=
                   !((*(&mut sessions as *mut sessions)).rbh_root ==
                         0 as *mut libc::c_void as *mut session) as
                       libc::c_int {
                return 0i32
            }
        }
        c = (*(&mut clients as *mut clients)).tqh_first;
        loop  {
            if c != 0 as *mut libc::c_void as *mut client {
                if (*c).session != 0 as *mut libc::c_void as *mut session {
                    return 0i32
                } else { c = (*c).entry.tqe_next }
            } else {
                cmd_wait_for_flush();
                if 0 !=
                       !((*(&mut clients as *mut clients)).tqh_first ==
                             0 as *mut libc::c_void as *mut client) as
                           libc::c_int {
                    current_block = 1394248824506584008;
                    break ;
                } else { current_block = 11812396948646013369; break ; }
            }
        }
        match current_block {
            1394248824506584008 => { return 0i32 }
            _ => {
                job = (*(&mut all_jobs as *mut joblist)).lh_first;
                loop  {
                    if job != 0 as *mut libc::c_void as *mut job {
                        if 0 != !(*job).flags & 1i32 &&
                               (*job).state as libc::c_uint ==
                                   JOB_RUNNING as libc::c_int as libc::c_uint
                           {
                            return 0i32
                        } else { job = (*job).entry.le_next }
                    } else { return 1i32 }
                }
            }
        }
    };
}
static mut server_exit: libc::c_int = unsafe { 0 };
#[no_mangle]
pub unsafe extern "C" fn server_add_accept(mut timeout: libc::c_int) -> () {
    let mut tv: timeval =
        timeval{tv_sec: timeout as __time_t, tv_usec: 0i32 as __suseconds_t,};
    if 0 != event_initialized(&mut server_ev_accept as *mut event) {
        event_del(&mut server_ev_accept as *mut event);
    }
    if timeout == 0i32 {
        event_set(&mut server_ev_accept as *mut event, server_fd,
                  2i32 as libc::c_short, Some(server_accept),
                  0 as *mut libc::c_void);
        event_add(&mut server_ev_accept as *mut event, 0 as *const timeval);
    } else {
        event_set(&mut server_ev_accept as *mut event, server_fd,
                  1i32 as libc::c_short, Some(server_accept),
                  0 as *mut libc::c_void);
        event_add(&mut server_ev_accept as *mut event,
                  &mut tv as *mut timeval);
    };
}
static mut server_ev_accept: event =
    unsafe {
        event{ev_active_next:
                  unnamed_32{tqe_next: 0 as *const event as *mut event,
                             tqe_prev:
                                 0 as *const *mut event as *mut *mut event,},
              ev_next:
                  unnamed_23{tqe_next: 0 as *const event as *mut event,
                             tqe_prev:
                                 0 as *const *mut event as *mut *mut event,},
              ev_timeout_pos:
                  unnamed_28{ev_next_with_common_timeout:
                                 unnamed_10{tqe_next:
                                                0 as *const event as
                                                    *mut event,
                                            tqe_prev:
                                                0 as *const *mut event as
                                                    *mut *mut event,},},
              ev_fd: 0,
              ev_base: 0 as *const event_base as *mut event_base,
              _ev:
                  unnamed_26{ev_io:
                                 unnamed_34{ev_io_next:
                                                unnamed_29{tqe_next:
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
unsafe extern "C" fn server_accept(mut fd: libc::c_int,
                                   mut events: libc::c_short,
                                   mut data: *mut libc::c_void) -> () {
    let mut sa: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut slen: socklen_t =
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
            socklen_t;
    let mut newfd: libc::c_int = 0;
    server_add_accept(0i32);
    if 0 == events as libc::c_int & 2i32 {
        return
    } else {
        newfd =
            accept(fd, &mut sa as *mut sockaddr_storage as *mut sockaddr,
                   &mut slen as *mut socklen_t);
        if newfd == 1i32.wrapping_neg() {
            if *__errno_location() == 11i32 || *__errno_location() == 4i32 ||
                   *__errno_location() == 103i32 {
                return
            } else if *__errno_location() == 23i32 ||
                          *__errno_location() == 24i32 {
                server_add_accept(1i32);
                return
            } else {
                fatal(b"accept failed\x00" as *const u8 as
                          *const libc::c_char);
            }
        } else if 0 != server_exit {
            close(newfd);
            return
        } else { server_client_create(newfd); return; }
    };
}
static mut server_fd: libc::c_int = unsafe { 0 };
unsafe extern "C" fn server_start_error(mut item: *mut cmdq_item,
                                        mut data: *mut libc::c_void)
 -> cmd_retval {
    let mut error: *mut libc::c_char = data as *mut libc::c_char;
    cmdq_error(item, b"%s\x00" as *const u8 as *const libc::c_char, error);
    free(error as *mut libc::c_void);
    return CMD_RETURN_NORMAL;
}
#[no_mangle]
pub unsafe extern "C" fn server_update_socket() -> () {
    let mut s: *mut session = 0 as *mut session;
    static mut last: libc::c_int = unsafe { 1i32.wrapping_neg() };
    let mut n: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    let mut sb: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    n = 0i32;
    s =
        sessions_RB_MINMAX(&mut sessions as *mut sessions,
                           1i32.wrapping_neg());
    while s != 0 as *mut libc::c_void as *mut session {
        if 0 == (*s).flags & 1i32 {
            n += 1;
            break ;
        } else { s = sessions_RB_NEXT(s) }
    }
    if n != last {
        last = n;
        if stat(socket_path, &mut sb as *mut stat) != 0i32 {
            return
        } else {
            mode =
                (sb.st_mode &
                     (256i32 | 128i32 | 64i32 |
                          (256i32 | 128i32 | 64i32) >> 3i32 |
                          (256i32 | 128i32 | 64i32) >> 3i32 >> 3i32) as
                         libc::c_uint) as libc::c_int;
            if n != 0i32 {
                if 0 != mode & 256i32 { mode |= 64i32 }
                if 0 != mode & 256i32 >> 3i32 { mode |= 64i32 >> 3i32 }
                if 0 != mode & 256i32 >> 3i32 >> 3i32 {
                    mode |= 64i32 >> 3i32 >> 3i32
                }
            } else {
                mode &= !(64i32 | 64i32 >> 3i32 | 64i32 >> 3i32 >> 3i32)
            }
            chmod(socket_path, mode as __mode_t);
        }
    };
}
unsafe extern "C" fn server_create_socket(mut cause: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut sa: sockaddr_un = sockaddr_un{sun_family: 0, sun_path: [0; 108],};
    let mut size: size_t = 0;
    let mut mask: mode_t = 0;
    let mut fd: libc::c_int = 0;
    let mut saved_errno: libc::c_int = 0;
    memset(&mut sa as *mut sockaddr_un as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong);
    sa.sun_family = 1i32 as sa_family_t;
    size =
        strlcpy(sa.sun_path.as_mut_ptr(), socket_path,
                ::std::mem::size_of::<[libc::c_char; 108]>() as
                    libc::c_ulong);
    if size >= ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong {
        *__errno_location() = 36i32
    } else {
        unlink(sa.sun_path.as_mut_ptr());
        fd = socket(1i32, SOCK_STREAM as libc::c_int, 0i32);
        if !(fd == 1i32.wrapping_neg()) {
            mask =
                umask((64i32 | 64i32 >> 3i32 |
                           (256i32 | 128i32 | 64i32) >> 3i32 >> 3i32) as
                          __mode_t);
            if bind(fd, &mut sa as *mut sockaddr_un as *mut sockaddr,
                    ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as
                        socklen_t) == 1i32.wrapping_neg() {
                saved_errno = *__errno_location();
                close(fd);
                *__errno_location() = saved_errno
            } else {
                umask(mask);
                if listen(fd, 128i32) == 1i32.wrapping_neg() {
                    saved_errno = *__errno_location();
                    close(fd);
                    *__errno_location() = saved_errno
                } else { setblocking(fd, 0i32); return fd }
            }
        }
    }
    if cause != 0 as *mut libc::c_void as *mut *mut libc::c_char {
        xasprintf(cause,
                  b"error creating %s (%s)\x00" as *const u8 as
                      *const libc::c_char, socket_path,
                  strerror(*__errno_location()));
    }
    return 1i32.wrapping_neg();
}
unsafe extern "C" fn server_signal(mut sig: libc::c_int) -> () {
    let mut fd: libc::c_int = 0;
    log_debug(b"%s: %s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"server_signal\x00")).as_ptr(),
              strsignal(sig));
    match sig {
        15 => { server_exit = 1i32; server_send_exit(); }
        17 => { server_child_signal(); }
        10 => {
            event_del(&mut server_ev_accept as *mut event);
            fd = server_create_socket(0 as *mut *mut libc::c_char);
            if fd != 1i32.wrapping_neg() {
                close(server_fd);
                server_fd = fd;
                server_update_socket();
            }
            server_add_accept(0i32);
        }
        12 => { proc_toggle_log(server_proc); }
        _ => { }
    };
}
unsafe extern "C" fn server_child_signal() -> () {
    let mut current_block: u64;
    let mut status: libc::c_int = 0;
    let mut pid: pid_t = 0;
    loop  {
        pid =
            waitpid(1i32.wrapping_neg(), &mut status as *mut libc::c_int,
                    1i32 | 2i32);
        match pid {
            -1 => {
                if *__errno_location() == 10i32 {
                    current_block = 12675440807659640239;
                    break ;
                } else { current_block = 12517898123489920830; break ; }
            }
            0 => { return }
            _ => {
                if (unnamed_8{__in: status,}.__i & 255i32 == 127i32) {
                    server_child_stopped(pid, status);
                } else {
                    if (!(unnamed_14{__in: status,}.__i & 127i32 == 0i32 ||
                              ((unnamed_12{__in: status,}.__i & 127i32) +
                                   1i32) as libc::c_schar as libc::c_int >>
                                  1i32 > 0i32)) {
                        continue ;
                    }
                    server_child_exited(pid, status);
                }
            }
        }
    }
    match current_block {
        12675440807659640239 => { return }
        _ => {
            fatal(b"waitpid failed\x00" as *const u8 as *const libc::c_char);
        }
    };
}
unsafe extern "C" fn server_child_exited(mut pid: pid_t,
                                         mut status: libc::c_int) -> () {
    let mut current_block: u64;
    let mut w: *mut window = 0 as *mut window;
    let mut w1: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut job: *mut job = 0 as *mut job;
    w = windows_RB_MINMAX(&mut windows as *mut windows, 1i32.wrapping_neg());
    while w != 0 as *mut libc::c_void as *mut window &&
              { w1 = windows_RB_NEXT(w); 0 != 1i32 } {
        wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
        loop  {
            if !(wp != 0 as *mut libc::c_void as *mut window_pane) {
                current_block = 12237857397564741460;
                break ;
            }
            if (*wp).pid == pid {
                (*wp).status = status;
                (*wp).flags |= 512i32;
                log_debug(b"%%%u exited\x00" as *const u8 as
                              *const libc::c_char, (*wp).id);
                (*wp).flags |= 256i32;
                if 0 != window_pane_destroy_ready(wp) {
                    current_block = 15427931788582360902;
                    break ;
                } else { current_block = 12237857397564741460; break ; }
            } else { wp = (*wp).entry.tqe_next }
        }
        match current_block {
            15427931788582360902 => { server_destroy_pane(wp, 1i32); }
            _ => { }
        }
        w = w1
    }
    job = (*(&mut all_jobs as *mut joblist)).lh_first;
    while job != 0 as *mut libc::c_void as *mut job {
        if pid == (*job).pid {
            job_died(job, status);
            break ;
        } else { job = (*job).entry.le_next }
    };
}
unsafe extern "C" fn server_child_stopped(mut pid: pid_t,
                                          mut status: libc::c_int) -> () {
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if ((unnamed_27{__in: status,}.__i & 65280i32) >> 8i32 == 21i32 ||
            (unnamed_37{__in: status,}.__i & 65280i32) >> 8i32 == 22i32) {
        return
    } else {
        w =
            windows_RB_MINMAX(&mut windows as *mut windows,
                              1i32.wrapping_neg());
        while w != 0 as *mut libc::c_void as *mut window {
            wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
            while wp != 0 as *mut libc::c_void as *mut window_pane {
                if (*wp).pid == pid {
                    if killpg(pid, 18i32) != 0i32 { kill(pid, 18i32); }
                }
                wp = (*wp).entry.tqe_next
            }
            w = windows_RB_NEXT(w)
        }
        return;
    };
}
unsafe extern "C" fn server_send_exit() -> () {
    let mut c: *mut client = 0 as *mut client;
    let mut c1: *mut client = 0 as *mut client;
    let mut s: *mut session = 0 as *mut session;
    let mut s1: *mut session = 0 as *mut session;
    cmd_wait_for_flush();
    c = (*(&mut clients as *mut clients)).tqh_first;
    while c != 0 as *mut libc::c_void as *mut client &&
              { c1 = (*c).entry.tqe_next; 0 != 1i32 } {
        if 0 != (*c).flags & 64i32 {
            server_client_lost(c);
        } else {
            if 0 != (*c).flags & 128i32 {
                notify_client(b"client-detached\x00" as *const u8 as
                                  *const libc::c_char, c);
            }
            proc_send((*c).peer, MSG_SHUTDOWN, 1i32.wrapping_neg(),
                      0 as *const libc::c_void, 0i32 as size_t);
        }
        (*c).session = 0 as *mut session;
        c = c1
    }
    s =
        sessions_RB_MINMAX(&mut sessions as *mut sessions,
                           1i32.wrapping_neg());
    while s != 0 as *mut libc::c_void as *mut session &&
              { s1 = sessions_RB_NEXT(s); 0 != 1i32 } {
        session_destroy(s,
                        (*::std::mem::transmute::<&[u8; 17],
                                                  &[libc::c_char; 17]>(b"server_send_exit\x00")).as_ptr());
        s = s1
    };
}

