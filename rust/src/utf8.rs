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
    pub type args_entry;
    pub type format_job_tree;
    pub type evbuffer;
    pub type format_tree;
    pub type options;
    pub type tty_code;
    pub type _IO_FILE_plus;
    pub type input_ctx;
    pub type tmuxproc;
    pub type tmuxpeer;
    pub type environ;
    pub type screen_titles;
    pub type event_base;
    pub type hooks;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __ctype_get_mb_cur_max() -> size_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn mbtowc(__pwc: *mut wchar_t, __s: *const libc::c_char, __n: size_t)
     -> libc::c_int;
    #[no_mangle]
    fn wctomb(__s: *mut libc::c_char, __wchar: wchar_t) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn wcwidth(__c: wchar_t) -> libc::c_int;
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
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    fn vis(_: *mut libc::c_char, _: libc::c_int, _: libc::c_int,
           _: libc::c_int) -> *mut libc::c_char;
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
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: libc::c_uint,
    pub flags: libc::c_int,
    pub qentry: unnamed_23,
}
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: libc::c_uchar,
    pub attr: libc::c_ushort,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_29,
}
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_17,
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
    pub entry: unnamed_19,
}
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cell {
    pub type_0: layout_type,
    pub parent: *mut layout_cell,
    pub sx: libc::c_uint,
    pub sy: libc::c_uint,
    pub xoff: libc::c_uint,
    pub yoff: libc::c_uint,
    pub wp: *mut window_pane,
    pub cells: layout_cells,
    pub entry: unnamed_13,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_item {
    pub name: *const libc::c_char,
    pub queue: *mut cmdq_list,
    pub next: *mut cmdq_item,
    pub client: *mut client,
    pub type_0: cmdq_type,
    pub group: libc::c_uint,
    pub number: libc::c_uint,
    pub time: time_t,
    pub flags: libc::c_int,
    pub shared: *mut cmdq_shared,
    pub source: cmd_find_state,
    pub target: cmd_find_state,
    pub cmdlist: *mut cmd_list,
    pub cmd: *mut cmd,
    pub cb: cmdq_cb,
    pub data: *mut libc::c_void,
    pub entry: unnamed_18,
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
    pub entry: unnamed_6,
    pub wentry: unnamed,
    pub sentry: unnamed_12,
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
pub type key_code = libc::c_ulonglong;
pub const TTY_VT100: unnamed_2 = 0;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen {
    pub title: *mut libc::c_char,
    pub titles: *mut screen_titles,
    pub grid: *mut grid,
    pub cx: libc::c_uint,
    pub cy: libc::c_uint,
    pub cstyle: libc::c_uint,
    pub ccolour: *mut libc::c_char,
    pub rupper: libc::c_uint,
    pub rlower: libc::c_uint,
    pub mode: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: screen_sel,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: libc::c_uint,
    pub entry: unnamed_9,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const TTY_VT101: unnamed_2 = 1;
pub type unnamed_2 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const PROMPT_ENTRY: unnamed_25 = 0;
pub const LINE_SEL_LEFT_RIGHT: unnamed_36 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub ev_signal_next: unnamed_30,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub type cmdq_type = libc::c_uint;
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const UTF8_MORE: utf8_state = 0;
pub const CMDQ_COMMAND: cmdq_type = 0;
pub type options_table_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub type tcflag_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_7 {
    ev_next_with_common_timeout: unnamed_38,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_8 {
    offset: libc::c_uint,
    data: unnamed_37,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty {
    pub client: *mut client,
    pub sx: libc::c_uint,
    pub sy: libc::c_uint,
    pub cx: libc::c_uint,
    pub cy: libc::c_uint,
    pub cstyle: libc::c_uint,
    pub ccolour: *mut libc::c_char,
    pub mode: libc::c_int,
    pub rlower: libc::c_uint,
    pub rupper: libc::c_uint,
    pub rleft: libc::c_uint,
    pub rright: libc::c_uint,
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
    pub term_type: unnamed_2,
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
pub const JOB_CLOSED: unnamed_17 = 2;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub type uint8_t = libc::c_uchar;
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type __suseconds_t = libc::c_long;
pub const TTY_UNKNOWN: unnamed_2 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_26,
    pub ev_next: unnamed_32,
    pub ev_timeout_pos: unnamed_7,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_34,
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
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
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
pub struct unnamed_12 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type __ssize_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const PROMPT_COMMAND: unnamed_25 = 1;
pub type layout_type = libc::c_uint;
pub const LINE_SEL_RIGHT_LEFT: unnamed_36 = 2;
pub type __off_t = libc::c_long;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const TTY_VT320: unnamed_2 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub type __pid_t = libc::c_int;
pub type unnamed_17 = libc::c_uint;
pub const TTY_VT420: unnamed_2 = 5;
pub const JOB_DEAD: unnamed_17 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const UTF8_ERROR: utf8_state = 2;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct mouse_event {
    pub valid: libc::c_int,
    pub key: key_code,
    pub statusat: libc::c_int,
    pub x: libc::c_uint,
    pub y: libc::c_uint,
    pub b: libc::c_uint,
    pub lx: libc::c_uint,
    pub ly: libc::c_uint,
    pub lb: libc::c_uint,
    pub s: libc::c_int,
    pub w: libc::c_int,
    pub wp: libc::c_int,
    pub sgr_type: libc::c_uint,
    pub sgr_b: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_5,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type options_table_scope = libc::c_uint;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type unnamed_25 = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: libc::c_uint,
    pub msg_time: time_t,
    pub entry: unnamed_21,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window {
    pub id: libc::c_uint,
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
    pub sx: libc::c_uint,
    pub sy: libc::c_uint,
    pub flags: libc::c_int,
    pub alerts_queued: libc::c_int,
    pub alerts_entry: unnamed_27,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: libc::c_uint,
    pub winlinks: unnamed_11,
    pub entry: unnamed_0,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const UTF8_DONE: utf8_state = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTY_VT102: unnamed_2 = 2;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: libc::c_uint,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_16,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub type cc_t = libc::c_uchar;
pub type ssize_t = __ssize_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub ev_io_next: unnamed_24,
    pub ev_timeout: timeval,
}
pub const TTY_VT220: unnamed_2 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_line {
    pub cellused: libc::c_uint,
    pub cellsize: libc::c_uint,
    pub celldata: *mut grid_cell_entry,
    pub extdsize: libc::c_uint,
    pub extddata: *mut grid_cell,
    pub flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_29 {
    __u6_addr8: [uint8_t; 16],
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
pub struct grid {
    pub flags: libc::c_int,
    pub sx: libc::c_uint,
    pub sy: libc::c_uint,
    pub hscrolled: libc::c_uint,
    pub hsize: libc::c_uint,
    pub hlimit: libc::c_uint,
    pub linedata: *mut grid_line,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct options_table_entry {
    pub name: *const libc::c_char,
    pub type_0: options_table_type,
    pub scope: options_table_scope,
    pub minimum: libc::c_uint,
    pub maximum: libc::c_uint,
    pub choices: *mut *const libc::c_char,
    pub default_str: *const libc::c_char,
    pub default_num: libc::c_longlong,
    pub separator: *const libc::c_char,
    pub style: *const libc::c_char,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_36,
    pub modekeys: libc::c_int,
    pub sx: libc::c_uint,
    pub sy: libc::c_uint,
    pub ex: libc::c_uint,
    pub ey: libc::c_uint,
    pub cell: grid_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub type utf8_state = libc::c_uint;
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
    pub click_button: libc::c_uint,
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
    pub message_next: libc::c_uint,
    pub message_log: unnamed_14,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: libc::c_uint,
    pub prompt_mode: unnamed_25,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_1,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_22,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_34 {
    ev_io: unnamed_28,
    ev_signal: unnamed_3,
}
pub const JOB_RUNNING: unnamed_17 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_10,
    pub entry: unnamed_33,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
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
pub type unnamed_36 = libc::c_uint;
pub type time_t = __time_t;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub attr: libc::c_uchar,
    pub fg: libc::c_uchar,
    pub bg: libc::c_uchar,
    pub data: libc::c_uchar,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session {
    pub id: libc::c_uint,
    pub name: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub creation_time: timeval,
    pub last_attached_time: timeval,
    pub activity_time: timeval,
    pub last_activity_time: timeval,
    pub lock_timer: event,
    pub sx: libc::c_uint,
    pub sy: libc::c_uint,
    pub curw: *mut winlink,
    pub lastw: winlink_stack,
    pub windows: winlinks,
    pub statusat: libc::c_int,
    pub hooks: *mut hooks,
    pub options: *mut options,
    pub flags: libc::c_int,
    pub attached: libc::c_uint,
    pub tio: *mut termios,
    pub environ: *mut environ,
    pub references: libc::c_int,
    pub gentry: unnamed_20,
    pub entry: unnamed_35,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: libc::c_uchar,
    pub unnamed: unnamed_8,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_mode {
    pub name: *const libc::c_char,
    pub init: Option<unsafe extern "C" fn(_: *mut window_pane,
                                          _: *mut cmd_find_state,
                                          _: *mut args) -> *mut screen>,
    pub free: Option<unsafe extern "C" fn(_: *mut window_pane) -> ()>,
    pub resize: Option<unsafe extern "C" fn(_: *mut window_pane,
                                            _: libc::c_uint, _: libc::c_uint)
                           -> ()>,
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
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub type wchar_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type __time_t = libc::c_long;
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
pub struct window_pane {
    pub id: libc::c_uint,
    pub active_point: libc::c_uint,
    pub window: *mut window,
    pub layout_cell: *mut layout_cell,
    pub saved_layout_cell: *mut layout_cell,
    pub sx: libc::c_uint,
    pub sy: libc::c_uint,
    pub osx: libc::c_uint,
    pub osy: libc::c_uint,
    pub xoff: libc::c_uint,
    pub yoff: libc::c_uint,
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
    pub saved_cx: libc::c_uint,
    pub saved_cy: libc::c_uint,
    pub saved_grid: *mut grid,
    pub saved_cell: grid_cell,
    pub mode: *const window_mode,
    pub modedata: *mut libc::c_void,
    pub modetimer: event,
    pub modelast: time_t,
    pub modeprefix: libc::c_uint,
    pub searchstr: *mut libc::c_char,
    pub entry: unnamed_15,
    pub tree_entry: unnamed_31,
}
pub const LINE_SEL_NONE: unnamed_36 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [libc::c_uchar; 9],
    pub have: libc::c_uchar,
    pub size: libc::c_uchar,
    pub width: libc::c_uchar,
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
#[no_mangle]
pub unsafe extern "C" fn utf8_set(mut ud: *mut utf8_data,
                                  mut ch: libc::c_uchar) -> () {
    static mut empty: utf8_data =
        unsafe {
            utf8_data{data: [0i32 as libc::c_uchar, 0, 0, 0, 0, 0, 0, 0, 0],
                      have: 1i32 as libc::c_uchar,
                      size: 1i32 as libc::c_uchar,
                      width: 1i32 as libc::c_uchar,}
        };
    memcpy(ud as *mut libc::c_void,
           &empty as *const utf8_data as *const libc::c_void,
           ::std::mem::size_of::<utf8_data>() as libc::c_ulong);
    *(*ud).data.as_mut_ptr() = ch;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_copy(mut to: *mut utf8_data,
                                   mut from: *const utf8_data) -> () {
    let mut i: libc::c_uint = 0;
    memcpy(to as *mut libc::c_void, from as *const libc::c_void,
           ::std::mem::size_of::<utf8_data>() as libc::c_ulong);
    i = (*to).size as libc::c_uint;
    while (i as libc::c_ulong) <
              ::std::mem::size_of::<[libc::c_uchar; 9]>() as libc::c_ulong {
        (*to).data[i as usize] = 0 as libc::c_uchar;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn utf8_open(mut ud: *mut utf8_data,
                                   mut ch: libc::c_uchar) -> utf8_state {
    memset(ud as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<utf8_data>() as libc::c_ulong);
    if ch as libc::c_int >= 194i32 && ch as libc::c_int <= 223i32 {
        (*ud).size = 2i32 as libc::c_uchar
    } else if ch as libc::c_int >= 224i32 && ch as libc::c_int <= 239i32 {
        (*ud).size = 3i32 as libc::c_uchar
    } else if ch as libc::c_int >= 240i32 && ch as libc::c_int <= 244i32 {
        (*ud).size = 4i32 as libc::c_uchar
    } else { return UTF8_ERROR }
    utf8_append(ud, ch);
    return UTF8_MORE;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_append(mut ud: *mut utf8_data,
                                     mut ch: libc::c_uchar) -> utf8_state {
    let mut wc: wchar_t = 0;
    let mut width: libc::c_int = 0;
    if (*ud).have as libc::c_int >= (*ud).size as libc::c_int {
        fatalx(b"UTF-8 character overflow\x00" as *const u8 as
                   *const libc::c_char);
    } else if (*ud).size as libc::c_ulong >
                  ::std::mem::size_of::<[libc::c_uchar; 9]>() as libc::c_ulong
     {
        fatalx(b"UTF-8 character size too large\x00" as *const u8 as
                   *const libc::c_char);
    } else {
        if (*ud).have as libc::c_int != 0i32 &&
               ch as libc::c_int & 192i32 != 128i32 {
            (*ud).width = 255i32 as libc::c_uchar
        }
        let fresh0 = (*ud).have;
        (*ud).have = (*ud).have.wrapping_add(1);
        (*ud).data[fresh0 as usize] = ch;
        if (*ud).have as libc::c_int != (*ud).size as libc::c_int {
            return UTF8_MORE
        } else if (*ud).width as libc::c_int == 255i32 {
            return UTF8_ERROR
        } else if utf8_combine(ud, &mut wc as *mut wchar_t) as libc::c_uint !=
                      UTF8_DONE as libc::c_int as libc::c_uint {
            return UTF8_ERROR
        } else {
            width = utf8_width(wc);
            if width < 0i32 {
                return UTF8_ERROR
            } else { (*ud).width = width as libc::c_uchar; return UTF8_DONE }
        }
    };
}
unsafe extern "C" fn utf8_width(mut wc: wchar_t) -> libc::c_int {
    let mut width: libc::c_int = 0;
    width = wcwidth(wc);
    if width < 0i32 || width > 255i32 {
        log_debug(b"Unicode %04lx, wcwidth() %d\x00" as *const u8 as
                      *const libc::c_char, wc as libc::c_long, width);
        if width < 0i32 { return 1i32 } else { return 1i32.wrapping_neg() }
    } else { return width };
}
#[no_mangle]
pub unsafe extern "C" fn utf8_combine(mut ud: *const utf8_data,
                                      mut wc: *mut wchar_t) -> utf8_state {
    match mbtowc(wc, (*ud).data.as_ptr() as *const libc::c_char,
                 (*ud).size as size_t) {
        -1 => {
            log_debug(b"UTF-8 %.*s, mbtowc() %d\x00" as *const u8 as
                          *const libc::c_char, (*ud).size as libc::c_int,
                      (*ud).data.as_ptr(), *__errno_location());
            mbtowc(0 as *mut wchar_t, 0 as *const libc::c_char,
                   __ctype_get_mb_cur_max());
            return UTF8_ERROR
        }
        0 => { return UTF8_ERROR }
        _ => { return UTF8_DONE }
    };
}
#[no_mangle]
pub unsafe extern "C" fn utf8_split(mut wc: wchar_t, mut ud: *mut utf8_data)
 -> utf8_state {
    let mut s: [libc::c_char; 16] = [0; 16];
    let mut slen: libc::c_int = 0;
    slen = wctomb(s.as_mut_ptr(), wc);
    if slen <= 0i32 ||
           slen >
               ::std::mem::size_of::<[libc::c_uchar; 9]>() as libc::c_ulong as
                   libc::c_int {
        return UTF8_ERROR
    } else {
        memcpy((*ud).data.as_mut_ptr() as *mut libc::c_void,
               s.as_mut_ptr() as *const libc::c_void, slen as libc::c_ulong);
        (*ud).size = slen as libc::c_uchar;
        (*ud).width = utf8_width(wc) as libc::c_uchar;
        return UTF8_DONE
    };
}
#[no_mangle]
pub unsafe extern "C" fn utf8_isvalid(mut s: *const libc::c_char)
 -> libc::c_int {
    let mut ud: utf8_data =
        utf8_data{data: [0; 9], have: 0, size: 0, width: 0,};
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut more: utf8_state = UTF8_MORE;
    end = s.offset(strlen(s) as isize);
    loop  {
        if s < end {
            more = utf8_open(&mut ud as *mut utf8_data, *s as libc::c_uchar);
            if more as libc::c_uint ==
                   UTF8_MORE as libc::c_int as libc::c_uint {
                loop  {
                    s = s.offset(1isize);
                    if !(s < end &&
                             more as libc::c_uint ==
                                 UTF8_MORE as libc::c_int as libc::c_uint) {
                        break ;
                    }
                    more =
                        utf8_append(&mut ud as *mut utf8_data,
                                    *s as libc::c_uchar)
                }
                if more as libc::c_uint ==
                       UTF8_DONE as libc::c_int as libc::c_uint {
                    continue ;
                }
                return 0i32
            } else if (*s as libc::c_int) < 32i32 ||
                          *s as libc::c_int > 126i32 {
                return 0i32
            } else { s = s.offset(1isize) }
        } else { return 1i32 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn utf8_strvis(mut dst: *mut libc::c_char,
                                     mut src: *const libc::c_char,
                                     mut len: size_t, mut flag: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut ud: utf8_data =
        utf8_data{data: [0; 9], have: 0, size: 0, width: 0,};
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut more: utf8_state = UTF8_MORE;
    let mut i: size_t = 0;
    start = dst;
    end = src.offset(len as isize);
    's_9:
        while src < end {
            more =
                utf8_open(&mut ud as *mut utf8_data, *src as libc::c_uchar);
            if more as libc::c_uint ==
                   UTF8_MORE as libc::c_int as libc::c_uint {
                current_block = 16658872821858055392;
            } else { current_block = 14916268686031723178; }
            loop  {
                match current_block {
                    14916268686031723178 => {
                        if src < end.offset(-1isize) {
                            dst =
                                vis(dst, *src.offset(0isize) as libc::c_int,
                                    flag, *src.offset(1isize) as libc::c_int);
                            current_block = 4906268039856690917;
                            break ;
                        } else if src < end {
                            current_block = 7815301370352969686;
                            break ;
                        } else {
                            current_block = 4906268039856690917;
                            break ;
                        }
                    }
                    _ => {
                        src = src.offset(1isize);
                        if src < end &&
                               more as libc::c_uint ==
                                   UTF8_MORE as libc::c_int as libc::c_uint {
                            more =
                                utf8_append(&mut ud as *mut utf8_data,
                                            *src as libc::c_uchar);
                            current_block = 16658872821858055392;
                        } else if more as libc::c_uint ==
                                      UTF8_DONE as libc::c_int as libc::c_uint
                         {
                            i = 0i32 as size_t;
                            current_block = 7095457783677275021;
                            break ;
                        } else {
                            src =
                                src.offset(-(ud.have as libc::c_int as
                                                 isize));
                            current_block = 14916268686031723178;
                        }
                    }
                }
            }
            match current_block {
                7095457783677275021 => {
                    loop  {
                        if !(i < ud.size as libc::c_ulong) { continue 's_9 ; }
                        let fresh1 = dst;
                        dst = dst.offset(1);
                        *fresh1 = ud.data[i as usize] as libc::c_char;
                        i = i.wrapping_add(1)
                    }
                }
                7815301370352969686 => {
                    dst =
                        vis(dst, *src.offset(0isize) as libc::c_int, flag, 0)
                }
                _ => { }
            }
            src = src.offset(1isize)
        }
    *dst = 0 as libc::c_char;
    return start.offset_to(dst).expect("bad offset_to") as libc::c_long as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_stravis(mut dst: *mut *mut libc::c_char,
                                      mut src: *const libc::c_char,
                                      mut flag: libc::c_int) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    buf =
        xreallocarray(0 as *mut libc::c_void, 4i32 as size_t,
                      strlen(src).wrapping_add(1i32 as libc::c_ulong)) as
            *mut libc::c_char;
    len = utf8_strvis(buf, src, strlen(src), flag);
    *dst =
        xrealloc(buf as *mut libc::c_void, (len + 1i32) as size_t) as
            *mut libc::c_char;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_sanitize(mut src: *const libc::c_char)
 -> *mut libc::c_char {
    let mut current_block: u64;
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: size_t = 0;
    let mut more: utf8_state = UTF8_MORE;
    let mut ud: utf8_data =
        utf8_data{data: [0; 9], have: 0, size: 0, width: 0,};
    let mut i: libc::c_uint = 0;
    dst = 0 as *mut libc::c_char;
    n = 0i32 as size_t;
    while *src as libc::c_int != 0 {
        dst =
            xreallocarray(dst as *mut libc::c_void,
                          n.wrapping_add(1i32 as libc::c_ulong),
                          ::std::mem::size_of::<libc::c_char>() as
                              libc::c_ulong) as *mut libc::c_char;
        more = utf8_open(&mut ud as *mut utf8_data, *src as libc::c_uchar);
        if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
            current_block = 4644295000439058019;
        } else { current_block = 12517898123489920830; }
        loop  {
            match current_block {
                4644295000439058019 => {
                    src = src.offset(1isize);
                    if *src as libc::c_int != 0 &&
                           more as libc::c_uint ==
                               UTF8_MORE as libc::c_int as libc::c_uint {
                        more =
                            utf8_append(&mut ud as *mut utf8_data,
                                        *src as libc::c_uchar);
                        current_block = 4644295000439058019;
                    } else if more as libc::c_uint ==
                                  UTF8_DONE as libc::c_int as libc::c_uint {
                        dst =
                            xreallocarray(dst as *mut libc::c_void,
                                          n.wrapping_add(ud.width as
                                                             libc::c_ulong),
                                          ::std::mem::size_of::<libc::c_char>()
                                              as libc::c_ulong) as
                                *mut libc::c_char;
                        i = 0i32 as libc::c_uint;
                        current_block = 6873731126896040597;
                        break ;
                    } else {
                        src = src.offset(-(ud.have as libc::c_int as isize));
                        current_block = 12517898123489920830;
                    }
                }
                _ => {
                    if *src as libc::c_int > 31i32 &&
                           (*src as libc::c_int) < 127i32 {
                        let fresh3 = n;
                        n = n.wrapping_add(1);
                        *dst.offset(fresh3 as isize) = *src;
                        current_block = 7351195479953500246;
                        break ;
                    } else {
                        let fresh4 = n;
                        n = n.wrapping_add(1);
                        *dst.offset(fresh4 as isize) = 95 as libc::c_char;
                        current_block = 7351195479953500246;
                        break ;
                    }
                }
            }
        }
        match current_block {
            6873731126896040597 => {
                while i < ud.width as libc::c_uint {
                    let fresh2 = n;
                    n = n.wrapping_add(1);
                    *dst.offset(fresh2 as isize) = 95 as libc::c_char;
                    i = i.wrapping_add(1)
                }
            }
            _ => { src = src.offset(1isize) }
        }
    }
    dst =
        xreallocarray(dst as *mut libc::c_void,
                      n.wrapping_add(1i32 as libc::c_ulong),
                      ::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as *mut libc::c_char;
    *dst.offset(n as isize) = 0 as libc::c_char;
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_strlen(mut s: *const utf8_data) -> size_t {
    let mut i: size_t = 0;
    i = 0i32 as size_t;
    while (*s.offset(i as isize)).size as libc::c_int != 0i32 {
        i = i.wrapping_add(1)
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_strwidth(mut s: *const utf8_data,
                                       mut n: ssize_t) -> libc::c_uint {
    let mut i: ssize_t = 0;
    let mut width: libc::c_uint = 0;
    width = 0i32 as libc::c_uint;
    i = 0i32 as ssize_t;
    while (*s.offset(i as isize)).size as libc::c_int != 0i32 {
        if n != 1i32.wrapping_neg() as libc::c_long && n == i { break ; }
        width =
            width.wrapping_add((*s.offset(i as isize)).width as libc::c_uint);
        i += 1
    }
    return width;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_fromcstr(mut src: *const libc::c_char)
 -> *mut utf8_data {
    let mut current_block: u64;
    let mut dst: *mut utf8_data = 0 as *mut utf8_data;
    let mut n: size_t = 0;
    let mut more: utf8_state = UTF8_MORE;
    dst = 0 as *mut utf8_data;
    n = 0i32 as size_t;
    's_8:
        while *src as libc::c_int != 0 {
            dst =
                xreallocarray(dst as *mut libc::c_void,
                              n.wrapping_add(1i32 as libc::c_ulong),
                              ::std::mem::size_of::<utf8_data>() as
                                  libc::c_ulong) as *mut utf8_data;
            more =
                utf8_open(&mut *dst.offset(n as isize) as *mut utf8_data,
                          *src as libc::c_uchar);
            if more as libc::c_uint ==
                   UTF8_MORE as libc::c_int as libc::c_uint {
                current_block = 16658872821858055392;
            } else { current_block = 14916268686031723178; }
            loop  {
                match current_block {
                    16658872821858055392 => {
                        src = src.offset(1isize);
                        if *src as libc::c_int != 0 &&
                               more as libc::c_uint ==
                                   UTF8_MORE as libc::c_int as libc::c_uint {
                            more =
                                utf8_append(&mut *dst.offset(n as isize) as
                                                *mut utf8_data,
                                            *src as libc::c_uchar);
                            current_block = 16658872821858055392;
                        } else {
                            if more as libc::c_uint ==
                                   UTF8_DONE as libc::c_int as libc::c_uint {
                                break ;
                            }
                            src =
                                src.offset(-((*dst.offset(n as isize)).have as
                                                 libc::c_int as isize));
                            current_block = 14916268686031723178;
                        }
                    }
                    _ => {
                        utf8_set(&mut *dst.offset(n as isize) as
                                     *mut utf8_data, *src as libc::c_uchar);
                        n = n.wrapping_add(1);
                        src = src.offset(1isize);
                        continue 's_8 ;
                    }
                }
            }
            n = n.wrapping_add(1)
        }
    dst =
        xreallocarray(dst as *mut libc::c_void,
                      n.wrapping_add(1i32 as libc::c_ulong),
                      ::std::mem::size_of::<utf8_data>() as libc::c_ulong) as
            *mut utf8_data;
    (*dst.offset(n as isize)).size = 0i32 as libc::c_uchar;
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_tocstr(mut src: *mut utf8_data)
 -> *mut libc::c_char {
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: size_t = 0;
    dst = 0 as *mut libc::c_char;
    n = 0i32 as size_t;
    while (*src).size as libc::c_int != 0i32 {
        dst =
            xreallocarray(dst as *mut libc::c_void,
                          n.wrapping_add((*src).size as libc::c_ulong),
                          1i32 as size_t) as *mut libc::c_char;
        memcpy(dst.offset(n as isize) as *mut libc::c_void,
               (*src).data.as_mut_ptr() as *const libc::c_void,
               (*src).size as libc::c_ulong);
        n =
            (n as libc::c_ulong).wrapping_add((*src).size as libc::c_ulong) as
                size_t as size_t;
        src = src.offset(1isize)
    }
    dst =
        xreallocarray(dst as *mut libc::c_void,
                      n.wrapping_add(1i32 as libc::c_ulong), 1i32 as size_t)
            as *mut libc::c_char;
    *dst.offset(n as isize) = 0 as libc::c_char;
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_cstrwidth(mut s: *const libc::c_char)
 -> libc::c_uint {
    let mut current_block: u64;
    let mut tmp: utf8_data =
        utf8_data{data: [0; 9], have: 0, size: 0, width: 0,};
    let mut width: libc::c_uint = 0;
    let mut more: utf8_state = UTF8_MORE;
    width = 0i32 as libc::c_uint;
    's_7:
        while *s as libc::c_int != 0 {
            more = utf8_open(&mut tmp as *mut utf8_data, *s as libc::c_uchar);
            if more as libc::c_uint ==
                   UTF8_MORE as libc::c_int as libc::c_uint {
                current_block = 12517898123489920830;
            } else { current_block = 8258075665625361029; }
            loop  {
                match current_block {
                    12517898123489920830 => {
                        s = s.offset(1isize);
                        if *s as libc::c_int != 0 &&
                               more as libc::c_uint ==
                                   UTF8_MORE as libc::c_int as libc::c_uint {
                            more =
                                utf8_append(&mut tmp as *mut utf8_data,
                                            *s as libc::c_uchar);
                            current_block = 12517898123489920830;
                        } else if more as libc::c_uint ==
                                      UTF8_DONE as libc::c_int as libc::c_uint
                         {
                            width =
                                width.wrapping_add(tmp.width as libc::c_uint);
                            continue 's_7 ;
                        } else {
                            s = s.offset(-(tmp.have as libc::c_int as isize));
                            current_block = 8258075665625361029;
                        }
                    }
                    _ => {
                        if !(*s as libc::c_int > 31i32 &&
                                 *s as libc::c_int != 127i32) {
                            break ;
                        }
                        width = width.wrapping_add(1);
                        break ;
                    }
                }
            }
            s = s.offset(1isize)
        }
    return width;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_rtrimcstr(mut s: *const libc::c_char,
                                        mut width: libc::c_uint)
 -> *mut libc::c_char {
    let mut tmp: *mut utf8_data = 0 as *mut utf8_data;
    let mut next: *mut utf8_data = 0 as *mut utf8_data;
    let mut end: *mut utf8_data = 0 as *mut utf8_data;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut at: libc::c_uint = 0;
    tmp = utf8_fromcstr(s);
    end = tmp;
    while (*end).size as libc::c_int != 0i32 { end = end.offset(1isize) }
    if end == tmp {
        free(tmp as *mut libc::c_void);
        return xstrdup(b"\x00" as *const u8 as *const libc::c_char)
    } else {
        next = end.offset(-1isize);
        at = 0i32 as libc::c_uint;
        loop  {
            if at.wrapping_add((*next).width as libc::c_uint) > width {
                next = next.offset(1isize);
                break ;
            } else {
                at = at.wrapping_add((*next).width as libc::c_uint);
                if next == tmp { break ; }
                next = next.offset(-1isize)
            }
        }
        out = utf8_tocstr(next);
        free(tmp as *mut libc::c_void);
        return out
    };
}
#[no_mangle]
pub unsafe extern "C" fn utf8_trimcstr(mut s: *const libc::c_char,
                                       mut width: libc::c_uint)
 -> *mut libc::c_char {
    let mut tmp: *mut utf8_data = 0 as *mut utf8_data;
    let mut next: *mut utf8_data = 0 as *mut utf8_data;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut at: libc::c_uint = 0;
    tmp = utf8_fromcstr(s);
    at = 0i32 as libc::c_uint;
    next = tmp;
    while (*next).size as libc::c_int != 0i32 {
        if at.wrapping_add((*next).width as libc::c_uint) > width {
            (*next).size = 0i32 as libc::c_uchar;
            break ;
        } else {
            at = at.wrapping_add((*next).width as libc::c_uint);
            next = next.offset(1isize)
        }
    }
    out = utf8_tocstr(tmp);
    free(tmp as *mut libc::c_void);
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_padcstr(mut s: *const libc::c_char,
                                      mut width: libc::c_uint)
 -> *mut libc::c_char {
    let mut slen: size_t = 0;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    n = utf8_cstrwidth(s);
    if n >= width {
        return xstrdup(s)
    } else {
        slen = strlen(s);
        out =
            xmalloc(slen.wrapping_add(1i32 as
                                          libc::c_ulong).wrapping_add(width.wrapping_sub(n)
                                                                          as
                                                                          libc::c_ulong))
                as *mut libc::c_char;
        memcpy(out as *mut libc::c_void, s as *const libc::c_void, slen);
        i = n;
        while i < width {
            let fresh5 = slen;
            slen = slen.wrapping_add(1);
            *out.offset(fresh5 as isize) = 32 as libc::c_char;
            i = i.wrapping_add(1)
        }
        *out.offset(slen as isize) = 0 as libc::c_char;
        return out
    };
}

