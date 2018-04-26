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
    pub type _IO_FILE_plus;
    pub type screen_titles;
    pub type tmuxproc;
    pub type input_ctx;
    pub type environ;
    pub type format_job_tree;
    pub type tty_code;
    pub type evbuffer;
    pub type args_entry;
    pub type options;
    pub type bufferevent_ops;
    pub type event_base;
    pub type hooks;
    pub type tmuxpeer;
    pub type format_tree;
    #[no_mangle]
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    #[no_mangle]
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
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
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn getuid() -> __uid_t;
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
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t)
     -> *mut libc::c_void;
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
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn environ_find(_: *mut environ, _: *const libc::c_char)
     -> *mut environ_entry;
    #[no_mangle]
    fn environ_put(_: *mut environ, _: *const libc::c_char) -> ();
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn cmd_free_argv(_: libc::c_int, _: *mut *mut libc::c_char) -> ();
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn cmd_list_parse(_: libc::c_int, _: *mut *mut libc::c_char,
                      _: *const libc::c_char, _: u_int,
                      _: *mut *mut libc::c_char) -> *mut cmd_list;
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
pub const TTY_VT101: unnamed_28 = 1;
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
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub type options_table_scope = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_21,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_31,
    pub ev_next: unnamed_16,
    pub ev_timeout_pos: unnamed_15,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_4,
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
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_32,
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
pub type key_code = libc::c_ulonglong;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_11 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_3 {
    offset: u_int,
    data: unnamed_27,
}
pub const TTY_VT102: unnamed_28 = 2;
pub type size_t = libc::c_ulong;
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
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_24,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const TTY_VT320: unnamed_28 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_4 {
    ev_io: unnamed_38,
    ev_signal: unnamed_18,
}
pub type u_char = __u_char;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const LINE_SEL_NONE: unnamed_11 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_11,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const JOB_DEAD: unnamed_29 = 1;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub type unnamed_8 = libc::c_uint;
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
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const TTY_VT220: unnamed_28 = 3;
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
pub type cmd_retval = libc::c_int;
pub type _IO_lock_t = ();
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
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
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub type time_t = __time_t;
pub const PROMPT_ENTRY: unnamed_8 = 0;
pub type unnamed_11 = libc::c_uint;
pub const JOB_CLOSED: unnamed_29 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_35,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type __u_short = libc::c_ushort;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
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
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_5,
    pub entry: unnamed_12,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_36,
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
    pub entry: unnamed_1,
    pub wentry: unnamed_22,
    pub sentry: unnamed_39,
}
pub type speed_t = libc::c_uint;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub type u_int = __u_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
pub union unnamed_15 {
    ev_next_with_common_timeout: unnamed_33,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
    pub entry: unnamed_13,
    pub tree_entry: unnamed_37,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub ev_signal_next: unnamed_14,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub type __gid_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
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
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_3,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
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
    pub message_log: unnamed_26,
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
    pub entry: unnamed_23,
}
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub type __pid_t = libc::c_int;
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
    pub term_type: unnamed_28,
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
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
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
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const JOB_RUNNING: unnamed_29 = 0;
pub type options_table_type = libc::c_uint;
pub const TTY_VT100: unnamed_28 = 0;
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub type tcflag_t = libc::c_uint;
pub type unnamed_28 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type unnamed_29 = libc::c_uint;
pub type __u_char = libc::c_uchar;
pub type bitstr_t = libc::c_uchar;
pub type pid_t = __pid_t;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
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
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct environ_entry {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub entry: unnamed_30,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_11 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_19,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_29,
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
    pub entry: unnamed_6,
}
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_10,
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
    pub alerts_entry: unnamed,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_0,
    pub entry: unnamed_9,
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
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
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
    pub entry: unnamed_17,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
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
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_36 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_25,
}
pub const TTY_UNKNOWN: unnamed_28 = 6;
pub const CMDQ_CALLBACK: cmdq_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub ev_io_next: unnamed_34,
    pub ev_timeout: timeval,
}
pub const PROMPT_COMMAND: unnamed_8 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const TTY_VT420: unnamed_28 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
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
    pub gentry: unnamed_2,
    pub entry: unnamed_7,
}
#[no_mangle]
pub unsafe extern "C" fn cmd_string_split(mut s: *const libc::c_char,
                                          mut rargc: *mut libc::c_int,
                                          mut rargv:
                                              *mut *mut *mut libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut p: size_t = 0i32 as size_t;
    let mut ch: libc::c_int = 0;
    let mut argc: libc::c_int = 0i32;
    let mut append: libc::c_int = 0i32;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut whitespace: *const libc::c_char = 0 as *const libc::c_char;
    let mut equals: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0i32 as size_t;
    loop  {
        ch = cmd_string_getc(s, &mut p as *mut size_t);
        match ch {
            39 => {
                t =
                    cmd_string_string(s, &mut p as *mut size_t,
                                      39 as libc::c_char, 0i32);
                if t == 0 as *mut libc::c_void as *mut libc::c_char {
                    current_block = 4005560482714005869;
                    break ;
                }
                cmd_string_copy(&mut buf as *mut *mut libc::c_char, t,
                                &mut len as *mut size_t);
                current_block = 12517898123489920830;
            }
            34 => {
                t =
                    cmd_string_string(s, &mut p as *mut size_t,
                                      34 as libc::c_char, 1i32);
                if t == 0 as *mut libc::c_void as *mut libc::c_char {
                    current_block = 4005560482714005869;
                    break ;
                }
                cmd_string_copy(&mut buf as *mut *mut libc::c_char, t,
                                &mut len as *mut size_t);
                current_block = 12517898123489920830;
            }
            36 => {
                t = cmd_string_variable(s, &mut p as *mut size_t);
                if t == 0 as *mut libc::c_void as *mut libc::c_char {
                    current_block = 4005560482714005869;
                    break ;
                }
                cmd_string_copy(&mut buf as *mut *mut libc::c_char, t,
                                &mut len as *mut size_t);
                current_block = 12517898123489920830;
            }
            35 => {
                loop  {
                    ch = cmd_string_getc(s, &mut p as *mut size_t);
                    if !(ch != 1i32.wrapping_neg()) { break ; }
                }
                current_block = 3466532690800553084;
            }
            -1 | 32 | 9 => { current_block = 3466532690800553084; }
            126 => {
                if buf != 0 as *mut libc::c_void as *mut libc::c_char {
                    append = 1i32
                } else {
                    t = cmd_string_expand_tilde(s, &mut p as *mut size_t);
                    if t == 0 as *mut libc::c_void as *mut libc::c_char {
                        current_block = 4005560482714005869;
                        break ;
                    }
                    cmd_string_copy(&mut buf as *mut *mut libc::c_char, t,
                                    &mut len as *mut size_t);
                }
                current_block = 12517898123489920830;
            }
            _ => { append = 1i32; current_block = 12517898123489920830; }
        }
        match current_block {
            3466532690800553084 => {
                if buf != 0 as *mut libc::c_void as *mut libc::c_char {
                    buf =
                        xrealloc(buf as *mut libc::c_void,
                                 len.wrapping_add(1i32 as libc::c_ulong)) as
                            *mut libc::c_char;
                    *buf.offset(len as isize) = 0 as libc::c_char;
                    argv =
                        xreallocarray(argv as *mut libc::c_void,
                                      (argc + 1i32) as size_t,
                                      ::std::mem::size_of::<*mut libc::c_char>()
                                          as libc::c_ulong) as
                            *mut *mut libc::c_char;
                    let fresh0 = argc;
                    argc = argc + 1;
                    let ref mut fresh1 = *argv.offset(fresh0 as isize);
                    *fresh1 = buf;
                    buf = 0 as *mut libc::c_char;
                    len = 0i32 as size_t
                }
                if !(ch != 1i32.wrapping_neg()) {
                    current_block = 13109137661213826276;
                    break ;
                }
            }
            _ => { }
        }
        if 0 != append {
            if len >=
                   18446744073709551615u64.wrapping_sub(2i32 as libc::c_ulong)
               {
                current_block = 4005560482714005869;
                break ;
            }
            buf =
                xrealloc(buf as *mut libc::c_void,
                         len.wrapping_add(1i32 as libc::c_ulong)) as
                    *mut libc::c_char;
            let fresh2 = len;
            len = len.wrapping_add(1);
            *buf.offset(fresh2 as isize) = ch as libc::c_char
        }
        append = 0i32
    }
    match current_block {
        4005560482714005869 => {
            if argv != 0 as *mut libc::c_void as *mut *mut libc::c_char {
                cmd_free_argv(argc, argv);
            }
            free(buf as *mut libc::c_void);
            return 1i32.wrapping_neg()
        }
        _ => {
            while argc != 0i32 {
                equals = strchr(*argv.offset(0isize), 61);
                whitespace =
                    (*argv.offset(0isize)).offset(strcspn(*argv.offset(0isize),
                                                          b" \t\x00" as
                                                              *const u8 as
                                                              *const libc::c_char)
                                                      as isize);
                if equals == 0 as *mut libc::c_void as *const libc::c_char ||
                       equals > whitespace {
                    break ;
                }
                environ_put(global_environ, *argv.offset(0isize));
                argc -= 1;
                memmove(argv as *mut libc::c_void,
                        argv.offset(1isize) as *const libc::c_void,
                        (argc as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                             as
                                                             libc::c_ulong));
            }
            *rargc = argc;
            *rargv = argv;
            free(buf as *mut libc::c_void);
            return 0i32
        }
    };
}
unsafe extern "C" fn cmd_string_copy(mut dst: *mut *mut libc::c_char,
                                     mut src: *mut libc::c_char,
                                     mut len: *mut size_t) -> () {
    let mut srclen: size_t = 0;
    srclen = strlen(src);
    *dst =
        xrealloc(*dst as *mut libc::c_void,
                 (*len).wrapping_add(srclen).wrapping_add(1i32 as
                                                              libc::c_ulong))
            as *mut libc::c_char;
    strlcpy((*dst).offset(*len as isize), src,
            srclen.wrapping_add(1i32 as libc::c_ulong));
    *len = (*len as libc::c_ulong).wrapping_add(srclen) as size_t as size_t;
    free(src as *mut libc::c_void);
}
unsafe extern "C" fn cmd_string_expand_tilde(mut s: *const libc::c_char,
                                             mut p: *mut size_t)
 -> *mut libc::c_char {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut home: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last: libc::c_int = 0;
    home = 0 as *mut libc::c_char;
    last = cmd_string_getc(s, p);
    if last == 1i32.wrapping_neg() || last == 47 || last == 32 || last == 9 {
        envent =
            environ_find(global_environ,
                         b"HOME\x00" as *const u8 as *const libc::c_char);
        if envent != 0 as *mut libc::c_void as *mut environ_entry &&
               *(*envent).value as libc::c_int != 0 {
            home = (*envent).value
        } else {
            pw = getpwuid(getuid());
            if pw != 0 as *mut libc::c_void as *mut passwd {
                home = (*pw).pw_dir
            }
        }
    } else {
        cmd_string_ungetc(p);
        user = xmalloc(strlen(s)) as *mut libc::c_char;
        cp = user;
        loop  {
            last = cmd_string_getc(s, p);
            if last == 1i32.wrapping_neg() || last == 47 || last == 32 ||
                   last == 9 {
                break ;
            }
            let fresh3 = cp;
            cp = cp.offset(1);
            *fresh3 = last as libc::c_char
        }
        *cp = 0 as libc::c_char;
        pw = getpwnam(user);
        if pw != 0 as *mut libc::c_void as *mut passwd { home = (*pw).pw_dir }
        free(user as *mut libc::c_void);
    }
    if home == 0 as *mut libc::c_void as *mut libc::c_char {
        return 0 as *mut libc::c_char
    } else {
        if last != 1i32.wrapping_neg() {
            xasprintf(&mut path as *mut *mut libc::c_char,
                      b"%s%c\x00" as *const u8 as *const libc::c_char, home,
                      last);
        } else {
            xasprintf(&mut path as *mut *mut libc::c_char,
                      b"%s\x00" as *const u8 as *const libc::c_char, home);
        }
        return path
    };
}
unsafe extern "C" fn cmd_string_getc(mut s: *const libc::c_char,
                                     mut p: *mut size_t) -> libc::c_int {
    let mut ucs: *const u_char = s as *const u_char;
    if *ucs.offset(*p as isize) as libc::c_int == 0 {
        return 1i32.wrapping_neg()
    } else {
        let fresh4 = *p;
        *p = (*p).wrapping_add(1);
        return *ucs.offset(fresh4 as isize) as libc::c_int
    };
}
unsafe extern "C" fn cmd_string_ungetc(mut p: *mut size_t) -> () {
    *p = (*p).wrapping_sub(1);
}
unsafe extern "C" fn cmd_string_variable(mut s: *const libc::c_char,
                                         mut p: *mut size_t)
 -> *mut libc::c_char {
    let mut current_block: u64;
    let mut ch: libc::c_int = 0;
    let mut fch: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    buf = 0 as *mut libc::c_char;
    len = 0i32 as size_t;
    fch = 1i32.wrapping_neg();
    ch = cmd_string_getc(s, p);
    match ch {
        -1 => { current_block = 4156105261607767724; }
        123 => {
            fch = 123;
            ch = cmd_string_getc(s, p);
            if 0 !=
                   !(ch == 95 || ch >= 97 && ch <= 122 ||
                         ch >= 65 && ch <= 90) as libc::c_int {
                current_block = 4156105261607767724;
            } else { current_block = 16903213892611502954; }
        }
        _ => { current_block = 16903213892611502954; }
    }
    match current_block {
        16903213892611502954 => {
            if 0 !=
                   !(ch == 95 || ch >= 97 && ch <= 122 ||
                         ch >= 65 && ch <= 90) as libc::c_int {
                xasprintf(&mut t as *mut *mut libc::c_char,
                          b"$%c\x00" as *const u8 as *const libc::c_char, ch);
                return t
            } else {
                buf =
                    xrealloc(buf as *mut libc::c_void,
                             len.wrapping_add(1i32 as libc::c_ulong)) as
                        *mut libc::c_char;
                let fresh5 = len;
                len = len.wrapping_add(1);
                *buf.offset(fresh5 as isize) = ch as libc::c_char;
                loop  {
                    ch = cmd_string_getc(s, p);
                    if ch == 1i32.wrapping_neg() ||
                           0 !=
                               !(ch == 95 || ch >= 97 && ch <= 122 ||
                                     ch >= 65 && ch <= 90 ||
                                     ch >= 48 && ch <= 57) as libc::c_int {
                        if fch == 123 && ch != 125 {
                            current_block = 4156105261607767724;
                            break ;
                        } else {
                            current_block = 1394248824506584008;
                            break ;
                        }
                    } else {
                        if len >=
                               18446744073709551615u64.wrapping_sub(3i32 as
                                                                        libc::c_ulong)
                           {
                            current_block = 4156105261607767724;
                            break ;
                        }
                        buf =
                            xrealloc(buf as *mut libc::c_void,
                                     len.wrapping_add(1i32 as libc::c_ulong))
                                as *mut libc::c_char;
                        let fresh6 = len;
                        len = len.wrapping_add(1);
                        *buf.offset(fresh6 as isize) = ch as libc::c_char
                    }
                }
                match current_block {
                    4156105261607767724 => { }
                    _ => {
                        if ch != 1i32.wrapping_neg() && fch != 123 {
                            cmd_string_ungetc(p);
                        }
                        buf =
                            xrealloc(buf as *mut libc::c_void,
                                     len.wrapping_add(1i32 as libc::c_ulong))
                                as *mut libc::c_char;
                        *buf.offset(len as isize) = 0 as libc::c_char;
                        envent = environ_find(global_environ, buf);
                        free(buf as *mut libc::c_void);
                        if envent ==
                               0 as *mut libc::c_void as *mut environ_entry {
                            return xstrdup(b"\x00" as *const u8 as
                                               *const libc::c_char)
                        } else { return xstrdup((*envent).value) }
                    }
                }
            }
        }
        _ => { }
    }
    free(buf as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn cmd_string_string(mut s: *const libc::c_char,
                                       mut p: *mut size_t,
                                       mut endch: libc::c_char,
                                       mut esc: libc::c_int)
 -> *mut libc::c_char {
    let mut current_block: u64;
    let mut ch: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    buf = 0 as *mut libc::c_char;
    len = 0i32 as size_t;
    loop  {
        ch = cmd_string_getc(s, p);
        if ch != endch as libc::c_int {
            match ch {
                -1 => { break ; }
                92 => {
                    if !(0 == esc) {
                        ch = cmd_string_getc(s, p);
                        match ch {
                            -1 => { break ; }
                            101 => {
                                current_block = 7854939589785639399;
                                match current_block {
                                    7854939589785639399 => { ch = 27 }
                                    4241207660285069302 => { ch = 9 }
                                    3271708476614423781 => { ch = 10 }
                                    _ => { ch = 13 }
                                }
                            }
                            114 => {
                                current_block = 4134344216241821271;
                                match current_block {
                                    7854939589785639399 => { ch = 27 }
                                    4241207660285069302 => { ch = 9 }
                                    3271708476614423781 => { ch = 10 }
                                    _ => { ch = 13 }
                                }
                            }
                            110 => {
                                current_block = 3271708476614423781;
                                match current_block {
                                    7854939589785639399 => { ch = 27 }
                                    4241207660285069302 => { ch = 9 }
                                    3271708476614423781 => { ch = 10 }
                                    _ => { ch = 13 }
                                }
                            }
                            116 => {
                                current_block = 4241207660285069302;
                                match current_block {
                                    7854939589785639399 => { ch = 27 }
                                    4241207660285069302 => { ch = 9 }
                                    3271708476614423781 => { ch = 10 }
                                    _ => { ch = 13 }
                                }
                            }
                            _ => { }
                        }
                    }
                }
                36 => {
                    if !(0 == esc) {
                        t = cmd_string_variable(s, p);
                        if t == 0 as *mut libc::c_void as *mut libc::c_char {
                            break ;
                        }
                        cmd_string_copy(&mut buf as *mut *mut libc::c_char, t,
                                        &mut len as *mut size_t);
                        continue ;
                    }
                }
                _ => { }
            }
            if len >=
                   18446744073709551615u64.wrapping_sub(2i32 as libc::c_ulong)
               {
                break ;
            }
            buf =
                xrealloc(buf as *mut libc::c_void,
                         len.wrapping_add(1i32 as libc::c_ulong)) as
                    *mut libc::c_char;
            let fresh7 = len;
            len = len.wrapping_add(1);
            *buf.offset(fresh7 as isize) = ch as libc::c_char
        } else {
            buf =
                xrealloc(buf as *mut libc::c_void,
                         len.wrapping_add(1i32 as libc::c_ulong)) as
                    *mut libc::c_char;
            *buf.offset(len as isize) = 0 as libc::c_char;
            return buf
        }
    }
    free(buf as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_string_parse(mut s: *const libc::c_char,
                                          mut file: *const libc::c_char,
                                          mut line: u_int,
                                          mut cause: *mut *mut libc::c_char)
 -> *mut cmd_list {
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    let mut argc: libc::c_int = 0;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    *cause = 0 as *mut libc::c_char;
    if cmd_string_split(s, &mut argc as *mut libc::c_int,
                        &mut argv as *mut *mut *mut libc::c_char) != 0i32 {
        xasprintf(cause,
                  b"invalid or unknown command: %s\x00" as *const u8 as
                      *const libc::c_char, s);
        return 0 as *mut cmd_list
    } else {
        if argc != 0i32 {
            cmdlist = cmd_list_parse(argc, argv, file, line, cause);
            if cmdlist == 0 as *mut libc::c_void as *mut cmd_list {
                cmd_free_argv(argc, argv);
                return 0 as *mut cmd_list
            }
        }
        cmd_free_argv(argc, argv);
        return cmdlist
    };
}

