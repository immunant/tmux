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
    pub type tty_code;
    pub type _IO_FILE_plus;
    pub type screen_titles;
    pub type event_base;
    pub type hooks;
    pub type args_entry;
    pub type bufferevent_ops;
    pub type format_job_tree;
    pub type tmuxpeer;
    pub type input_ctx;
    pub type options;
    pub type environ;
    pub type evbuffer;
    pub type tmuxproc;
    pub type format_tree;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
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
    fn cmd_list_free(_: *mut cmd_list) -> ();
    #[no_mangle]
    fn cmdq_get_command(_: *mut cmd_list, _: *mut cmd_find_state,
                        _: *mut mouse_event, _: libc::c_int)
     -> *mut cmdq_item;
    #[no_mangle]
    fn cmdq_get_callback1(_: *const libc::c_char, _: cmdq_cb,
                          _: *mut libc::c_void) -> *mut cmdq_item;
    #[no_mangle]
    fn cmdq_insert_after(_: *mut cmdq_item, _: *mut cmdq_item) -> ();
    #[no_mangle]
    fn cmdq_append(_: *mut client, _: *mut cmdq_item) -> ();
    #[no_mangle]
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn cmd_string_parse(_: *const libc::c_char, _: *const libc::c_char,
                        _: u_int, _: *mut *mut libc::c_char) -> *mut cmd_list;
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    fn server_client_set_key_table(_: *mut client, _: *const libc::c_char)
     -> ();
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
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
pub const LINE_SEL_NONE: unnamed_26 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const TTY_VT420: unnamed_35 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub ev_signal_next: unnamed_17,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
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
pub struct unnamed_3 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type bitstr_t = libc::c_uchar;
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
    pub tree_entry: unnamed_15,
}
pub const JOB_RUNNING: unnamed_23 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_33,
}
pub const TTY_UNKNOWN: unnamed_35 = 6;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_5 {
    ev_io: unnamed_13,
    ev_signal: unnamed_1,
}
pub const TTY_VT102: unnamed_35 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_3,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const JOB_CLOSED: unnamed_23 = 2;
pub const PROMPT_COMMAND: unnamed_22 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub type cmd_find_type = libc::c_uint;
pub const TTY_VT101: unnamed_35 = 1;
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_7 {
    offset: u_int,
    data: unnamed_0,
}
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub type speed_t = libc::c_uint;
pub type key_code = libc::c_ulonglong;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
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
    pub entry: unnamed_12,
}
pub type tcflag_t = libc::c_uint;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
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
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub type options_table_scope = libc::c_uint;
pub type pid_t = __pid_t;
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_7,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const PROMPT_ENTRY: unnamed_22 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub type __suseconds_t = libc::c_long;
pub type layout_type = libc::c_uint;
pub const LINE_SEL_RIGHT_LEFT: unnamed_26 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_26 = 1;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_11,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
pub const TTY_VT220: unnamed_35 = 3;
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
    pub entry: unnamed_10,
    pub wentry: unnamed_25,
    pub sentry: unnamed_14,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub ev_io_next: unnamed_20,
    pub ev_timeout: timeval,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const JOB_DEAD: unnamed_23 = 1;
pub type _IO_lock_t = ();
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
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_30,
}
pub const TTY_VT320: unnamed_35 = 4;
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
    pub alerts_entry: unnamed_19,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_36,
    pub entry: unnamed_31,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_27,
    pub entry: unnamed_21,
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
    pub gentry: unnamed,
    pub entry: unnamed_16,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
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
pub struct unnamed_17 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
    pub entry: unnamed_2,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub type options_table_type = libc::c_uint;
pub type uint16_t = libc::c_ushort;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
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
pub type __u_int = libc::c_uint;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type unnamed_22 = libc::c_uint;
pub type __u_short = libc::c_ushort;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub type unnamed_23 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_9,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_4,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
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
pub type unnamed_26 = libc::c_uint;
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
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_26,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub type u_char = __u_char;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type __u_char = libc::c_uchar;
pub type __off64_t = libc::c_long;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_28 {
    ev_next_with_common_timeout: unnamed_6,
    min_heap_idx: libc::c_int,
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
    pub term_type: unnamed_35,
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
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_24,
    pub ev_next: unnamed_29,
    pub ev_timeout_pos: unnamed_28,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_5,
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
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
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
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_38,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_37,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub type u_short = __u_short;
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
    pub prompt_mode: unnamed_22,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_18,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
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
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub const TTY_VT100: unnamed_35 = 0;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub type u_int = __u_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub type unnamed_35 = libc::c_uint;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type __time_t = libc::c_long;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_37 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
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
pub struct unnamed_38 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_RB_INSERT_COLOR(mut head:
                                                          *mut key_bindings,
                                                      mut elm:
                                                          *mut key_binding)
 -> () {
    let mut current_block: u64;
    let mut parent: *mut key_binding = 0 as *mut key_binding;
    let mut gparent: *mut key_binding = 0 as *mut key_binding;
    let mut tmp: *mut key_binding = 0 as *mut key_binding;
    loop  {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() && (*parent).entry.rbe_color == 1i32) {
            break ;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                loop  {
                    (*parent).entry.rbe_color = 0i32;
                    (*gparent).entry.rbe_color = 1i32;
                    if !(0 != 0i32) { break ; }
                }
                elm = gparent
            } else {
                if (*parent).entry.rbe_right == elm {
                    current_block = 7351195479953500246;
                } else { current_block = 4956146061682418353; }
                's_87:
                    loop  {
                        match current_block {
                            4956146061682418353 => {
                                (*parent).entry.rbe_color = 0i32;
                                (*gparent).entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4956146061682418353;
                                } else { break ; }
                            }
                            _ => {
                                tmp = (*parent).entry.rbe_right;
                                (*parent).entry.rbe_right =
                                    (*tmp).entry.rbe_left;
                                if !(*parent).entry.rbe_right.is_null() {
                                    (*(*tmp).entry.rbe_left).entry.rbe_parent
                                        = parent
                                }
                                while 0 != 0i32 { }
                                (*tmp).entry.rbe_parent =
                                    (*parent).entry.rbe_parent;
                                if !(*tmp).entry.rbe_parent.is_null() {
                                    if parent ==
                                           (*(*parent).entry.rbe_parent).entry.rbe_left
                                       {
                                        (*(*parent).entry.rbe_parent).entry.rbe_left
                                            = tmp
                                    } else {
                                        (*(*parent).entry.rbe_parent).entry.rbe_right
                                            = tmp
                                    }
                                } else { (*head).rbh_root = tmp }
                                (*tmp).entry.rbe_left = parent;
                                (*parent).entry.rbe_parent = tmp;
                                while 0 != 0i32 { }
                                if !(*tmp).entry.rbe_parent.is_null() {
                                    current_block = 10048703153582371463;
                                } else {
                                    current_block = 10879442775620481940;
                                }
                                loop  {
                                    match current_block {
                                        10879442775620481940 => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    7351195479953500246;
                                                continue 's_87 ;
                                            } else { break ; }
                                        }
                                        _ => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    10048703153582371463;
                                            } else {
                                                current_block =
                                                    10879442775620481940;
                                            }
                                        }
                                    }
                                }
                                tmp = parent;
                                parent = elm;
                                elm = tmp;
                                current_block = 4956146061682418353;
                            }
                        }
                    }
                's_95:
                    loop  {
                        tmp = (*gparent).entry.rbe_left;
                        (*gparent).entry.rbe_left = (*tmp).entry.rbe_right;
                        if !(*gparent).entry.rbe_left.is_null() {
                            (*(*tmp).entry.rbe_right).entry.rbe_parent =
                                gparent
                        }
                        while 0 != 0i32 { }
                        (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                        if !(*tmp).entry.rbe_parent.is_null() {
                            if gparent ==
                                   (*(*gparent).entry.rbe_parent).entry.rbe_left
                               {
                                (*(*gparent).entry.rbe_parent).entry.rbe_left
                                    = tmp
                            } else {
                                (*(*gparent).entry.rbe_parent).entry.rbe_right
                                    = tmp
                            }
                        } else { (*head).rbh_root = tmp }
                        (*tmp).entry.rbe_right = gparent;
                        (*gparent).entry.rbe_parent = tmp;
                        while 0 != 0i32 { }
                        if !(*tmp).entry.rbe_parent.is_null() {
                            current_block = 6669252993407410313;
                        } else { current_block = 5948590327928692120; }
                        loop  {
                            match current_block {
                                6669252993407410313 => {
                                    if 0 != 0i32 {
                                        current_block = 6669252993407410313;
                                    } else {
                                        current_block = 5948590327928692120;
                                    }
                                }
                                _ => {
                                    if 0 != 0i32 {
                                        break ;
                                    } else { break 's_95 ; }
                                }
                            }
                        }
                    }
            }
        } else {
            tmp = (*gparent).entry.rbe_left;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                loop  {
                    (*parent).entry.rbe_color = 0i32;
                    (*gparent).entry.rbe_color = 1i32;
                    if !(0 != 0i32) { break ; }
                }
                elm = gparent
            } else {
                if (*parent).entry.rbe_left == elm {
                    current_block = 652864300344834934;
                } else { current_block = 4567019141635105728; }
                's_211:
                    loop  {
                        match current_block {
                            652864300344834934 => {
                                tmp = (*parent).entry.rbe_left;
                                (*parent).entry.rbe_left =
                                    (*tmp).entry.rbe_right;
                                if !(*parent).entry.rbe_left.is_null() {
                                    (*(*tmp).entry.rbe_right).entry.rbe_parent
                                        = parent
                                }
                                while 0 != 0i32 { }
                                (*tmp).entry.rbe_parent =
                                    (*parent).entry.rbe_parent;
                                if !(*tmp).entry.rbe_parent.is_null() {
                                    if parent ==
                                           (*(*parent).entry.rbe_parent).entry.rbe_left
                                       {
                                        (*(*parent).entry.rbe_parent).entry.rbe_left
                                            = tmp
                                    } else {
                                        (*(*parent).entry.rbe_parent).entry.rbe_right
                                            = tmp
                                    }
                                } else { (*head).rbh_root = tmp }
                                (*tmp).entry.rbe_right = parent;
                                (*parent).entry.rbe_parent = tmp;
                                while 0 != 0i32 { }
                                if !(*tmp).entry.rbe_parent.is_null() {
                                    current_block = 980989089337379490;
                                } else {
                                    current_block = 5494826135382683477;
                                }
                                loop  {
                                    match current_block {
                                        980989089337379490 => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    980989089337379490;
                                            } else {
                                                current_block =
                                                    5494826135382683477;
                                            }
                                        }
                                        _ => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    652864300344834934;
                                                continue 's_211 ;
                                            } else { break ; }
                                        }
                                    }
                                }
                                tmp = parent;
                                parent = elm;
                                elm = tmp;
                                current_block = 4567019141635105728;
                            }
                            _ => {
                                (*parent).entry.rbe_color = 0i32;
                                (*gparent).entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4567019141635105728;
                                } else { break ; }
                            }
                        }
                    }
                's_219:
                    loop  {
                        tmp = (*gparent).entry.rbe_right;
                        (*gparent).entry.rbe_right = (*tmp).entry.rbe_left;
                        if !(*gparent).entry.rbe_right.is_null() {
                            (*(*tmp).entry.rbe_left).entry.rbe_parent =
                                gparent
                        }
                        while 0 != 0i32 { }
                        (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                        if !(*tmp).entry.rbe_parent.is_null() {
                            if gparent ==
                                   (*(*gparent).entry.rbe_parent).entry.rbe_left
                               {
                                (*(*gparent).entry.rbe_parent).entry.rbe_left
                                    = tmp
                            } else {
                                (*(*gparent).entry.rbe_parent).entry.rbe_right
                                    = tmp
                            }
                        } else { (*head).rbh_root = tmp }
                        (*tmp).entry.rbe_left = gparent;
                        (*gparent).entry.rbe_parent = tmp;
                        while 0 != 0i32 { }
                        if !(*tmp).entry.rbe_parent.is_null() {
                            current_block = 11793792312832361944;
                        } else { current_block = 2543120759711851213; }
                        loop  {
                            match current_block {
                                2543120759711851213 => {
                                    if 0 != 0i32 {
                                        break ;
                                    } else { break 's_219 ; }
                                }
                                _ => {
                                    if 0 != 0i32 {
                                        current_block = 11793792312832361944;
                                    } else {
                                        current_block = 2543120759711851213;
                                    }
                                }
                            }
                        }
                    }
            }
        }
    }
    (*(*head).rbh_root).entry.rbe_color = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_RB_REMOVE_COLOR(mut head:
                                                          *mut key_bindings,
                                                      mut parent:
                                                          *mut key_binding,
                                                      mut elm:
                                                          *mut key_binding)
 -> () {
    let mut current_block: u64;
    let mut tmp: *mut key_binding = 0 as *mut key_binding;
    's_4:
        loop  {
            if !((elm == 0 as *mut libc::c_void as *mut key_binding ||
                      (*elm).entry.rbe_color == 0i32) &&
                     elm != (*head).rbh_root) {
                current_block = 11174649648027449784;
                break ;
            }
            if (*parent).entry.rbe_left == elm {
                tmp = (*parent).entry.rbe_right;
                if (*tmp).entry.rbe_color == 1i32 {
                    current_block = 17179679302217393232;
                } else { current_block = 14155750587950065367; }
                loop  {
                    match current_block {
                        14155750587950065367 => {
                            if ((*tmp).entry.rbe_left ==
                                    0 as *mut libc::c_void as *mut key_binding
                                    ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut key_binding ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_right ==
                                          0 as *mut libc::c_void as
                                              *mut key_binding ||
                                          (*(*tmp).entry.rbe_right).entry.rbe_color
                                              == 0i32 {
                                current_block = 15976848397966268834;
                                break 's_4 ;
                            } else {
                                current_block = 7149356873433890176;
                                break 's_4 ;
                            }
                        }
                        _ => {
                            (*tmp).entry.rbe_color = 0i32;
                            (*parent).entry.rbe_color = 1i32;
                            if 0 != 0i32 {
                                current_block = 17179679302217393232;
                                continue ;
                            }
                            's_30:
                                loop  {
                                    tmp = (*parent).entry.rbe_right;
                                    (*parent).entry.rbe_right =
                                        (*tmp).entry.rbe_left;
                                    if !(*parent).entry.rbe_right.is_null() {
                                        (*(*tmp).entry.rbe_left).entry.rbe_parent
                                            = parent
                                    }
                                    while 0 != 0i32 { }
                                    (*tmp).entry.rbe_parent =
                                        (*parent).entry.rbe_parent;
                                    if !(*tmp).entry.rbe_parent.is_null() {
                                        if parent ==
                                               (*(*parent).entry.rbe_parent).entry.rbe_left
                                           {
                                            (*(*parent).entry.rbe_parent).entry.rbe_left
                                                = tmp
                                        } else {
                                            (*(*parent).entry.rbe_parent).entry.rbe_right
                                                = tmp
                                        }
                                    } else { (*head).rbh_root = tmp }
                                    (*tmp).entry.rbe_left = parent;
                                    (*parent).entry.rbe_parent = tmp;
                                    while 0 != 0i32 { }
                                    if !(*tmp).entry.rbe_parent.is_null() {
                                        current_block = 11050875288958768710;
                                    } else {
                                        current_block = 15240798224410183470;
                                    }
                                    loop  {
                                        match current_block {
                                            15240798224410183470 => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_30 ; }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        11050875288958768710;
                                                } else {
                                                    current_block =
                                                        15240798224410183470;
                                                }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).entry.rbe_right;
                            current_block = 14155750587950065367;
                        }
                    }
                }
            } else {
                tmp = (*parent).entry.rbe_left;
                if (*tmp).entry.rbe_color == 1i32 {
                    current_block = 6450597802325118133;
                } else { current_block = 7746103178988627676; }
                loop  {
                    match current_block {
                        7746103178988627676 => {
                            if ((*tmp).entry.rbe_left ==
                                    0 as *mut libc::c_void as *mut key_binding
                                    ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut key_binding ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_left ==
                                          0 as *mut libc::c_void as
                                              *mut key_binding ||
                                          (*(*tmp).entry.rbe_left).entry.rbe_color
                                              == 0i32 {
                                current_block = 13826291924415791078;
                                break 's_4 ;
                            } else {
                                current_block = 5892776923941496671;
                                break 's_4 ;
                            }
                        }
                        _ => {
                            (*tmp).entry.rbe_color = 0i32;
                            (*parent).entry.rbe_color = 1i32;
                            if 0 != 0i32 {
                                current_block = 6450597802325118133;
                                continue ;
                            }
                            's_210:
                                loop  {
                                    tmp = (*parent).entry.rbe_left;
                                    (*parent).entry.rbe_left =
                                        (*tmp).entry.rbe_right;
                                    if !(*parent).entry.rbe_left.is_null() {
                                        (*(*tmp).entry.rbe_right).entry.rbe_parent
                                            = parent
                                    }
                                    while 0 != 0i32 { }
                                    (*tmp).entry.rbe_parent =
                                        (*parent).entry.rbe_parent;
                                    if !(*tmp).entry.rbe_parent.is_null() {
                                        if parent ==
                                               (*(*parent).entry.rbe_parent).entry.rbe_left
                                           {
                                            (*(*parent).entry.rbe_parent).entry.rbe_left
                                                = tmp
                                        } else {
                                            (*(*parent).entry.rbe_parent).entry.rbe_right
                                                = tmp
                                        }
                                    } else { (*head).rbh_root = tmp }
                                    (*tmp).entry.rbe_right = parent;
                                    (*parent).entry.rbe_parent = tmp;
                                    while 0 != 0i32 { }
                                    if !(*tmp).entry.rbe_parent.is_null() {
                                        current_block = 16738040538446813684;
                                    } else {
                                        current_block = 17784502470059252271;
                                    }
                                    loop  {
                                        match current_block {
                                            16738040538446813684 => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        16738040538446813684;
                                                } else {
                                                    current_block =
                                                        17784502470059252271;
                                                }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_210 ; }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).entry.rbe_left;
                            current_block = 7746103178988627676;
                        }
                    }
                }
            }
        }
    match current_block {
        15976848397966268834 => {
            let mut oleft: *mut key_binding = 0 as *mut key_binding;
            oleft = (*tmp).entry.rbe_left;
            if !oleft.is_null() { (*oleft).entry.rbe_color = 0i32 }
            (*tmp).entry.rbe_color = 1i32;
            's_96:
                loop  {
                    oleft = (*tmp).entry.rbe_left;
                    (*tmp).entry.rbe_left = (*oleft).entry.rbe_right;
                    if !(*tmp).entry.rbe_left.is_null() {
                        (*(*oleft).entry.rbe_right).entry.rbe_parent = tmp
                    }
                    while 0 != 0i32 { }
                    (*oleft).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oleft).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oleft
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oleft
                        }
                    } else { (*head).rbh_root = oleft }
                    (*oleft).entry.rbe_right = tmp;
                    (*tmp).entry.rbe_parent = oleft;
                    while 0 != 0i32 { }
                    if !(*oleft).entry.rbe_parent.is_null() {
                        current_block = 2232869372362427478;
                    } else { current_block = 15904375183555213903; }
                    loop  {
                        match current_block {
                            15904375183555213903 => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_96 ; }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    current_block = 2232869372362427478;
                                } else {
                                    current_block = 15904375183555213903;
                                }
                            }
                        }
                    }
                }
            tmp = (*parent).entry.rbe_right;
            current_block = 7149356873433890176;
        }
        13826291924415791078 => {
            let mut oright: *mut key_binding = 0 as *mut key_binding;
            oright = (*tmp).entry.rbe_right;
            if !oright.is_null() { (*oright).entry.rbe_color = 0i32 }
            (*tmp).entry.rbe_color = 1i32;
            's_276:
                loop  {
                    oright = (*tmp).entry.rbe_right;
                    (*tmp).entry.rbe_right = (*oright).entry.rbe_left;
                    if !(*tmp).entry.rbe_right.is_null() {
                        (*(*oright).entry.rbe_left).entry.rbe_parent = tmp
                    }
                    while 0 != 0i32 { }
                    (*oright).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oright).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oright
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right =
                                oright
                        }
                    } else { (*head).rbh_root = oright }
                    (*oright).entry.rbe_left = tmp;
                    (*tmp).entry.rbe_parent = oright;
                    while 0 != 0i32 { }
                    if !(*oright).entry.rbe_parent.is_null() {
                        current_block = 3392087639489470149;
                    } else { current_block = 1854459640724737493; }
                    loop  {
                        match current_block {
                            1854459640724737493 => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_276 ; }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    current_block = 3392087639489470149;
                                } else {
                                    current_block = 1854459640724737493;
                                }
                            }
                        }
                    }
                }
            tmp = (*parent).entry.rbe_left;
            current_block = 5892776923941496671;
        }
        _ => { }
    }
    match current_block {
        7149356873433890176 => {
            (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
            (*parent).entry.rbe_color = 0i32;
            if !(*tmp).entry.rbe_right.is_null() {
                (*(*tmp).entry.rbe_right).entry.rbe_color = 0i32
            }
            's_148:
                loop  {
                    tmp = (*parent).entry.rbe_right;
                    (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                    if !(*parent).entry.rbe_right.is_null() {
                        (*(*tmp).entry.rbe_left).entry.rbe_parent = parent
                    }
                    while 0 != 0i32 { }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent ==
                               (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right =
                                tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).entry.rbe_left = parent;
                    (*parent).entry.rbe_parent = tmp;
                    while 0 != 0i32 { }
                    if !(*tmp).entry.rbe_parent.is_null() {
                        current_block = 6450636197030046351;
                    } else { current_block = 16924917904204750491; }
                    loop  {
                        match current_block {
                            16924917904204750491 => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_148 ; }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    current_block = 6450636197030046351;
                                } else {
                                    current_block = 16924917904204750491;
                                }
                            }
                        }
                    }
                }
            elm = (*head).rbh_root
        }
        5892776923941496671 => {
            (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
            (*parent).entry.rbe_color = 0i32;
            if !(*tmp).entry.rbe_left.is_null() {
                (*(*tmp).entry.rbe_left).entry.rbe_color = 0i32
            }
            's_328:
                loop  {
                    tmp = (*parent).entry.rbe_left;
                    (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                    if !(*parent).entry.rbe_left.is_null() {
                        (*(*tmp).entry.rbe_right).entry.rbe_parent = parent
                    }
                    while 0 != 0i32 { }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent ==
                               (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right =
                                tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).entry.rbe_right = parent;
                    (*parent).entry.rbe_parent = tmp;
                    while 0 != 0i32 { }
                    if !(*tmp).entry.rbe_parent.is_null() {
                        current_block = 13910774313357589740;
                    } else { current_block = 13707613154239713890; }
                    loop  {
                        match current_block {
                            13910774313357589740 => {
                                if 0 != 0i32 {
                                    current_block = 13910774313357589740;
                                } else {
                                    current_block = 13707613154239713890;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_328 ; }
                            }
                        }
                    }
                }
            elm = (*head).rbh_root
        }
        _ => { }
    }
    if !elm.is_null() { (*elm).entry.rbe_color = 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_RB_REMOVE(mut head: *mut key_bindings,
                                                mut elm: *mut key_binding)
 -> *mut key_binding {
    let mut current_block: u64;
    let mut child: *mut key_binding = 0 as *mut key_binding;
    let mut parent: *mut key_binding = 0 as *mut key_binding;
    let mut old: *mut key_binding = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left == 0 as *mut libc::c_void as *mut key_binding {
        child = (*elm).entry.rbe_right;
        current_block = 9386390421034826751;
    } else if (*elm).entry.rbe_right ==
                  0 as *mut libc::c_void as *mut key_binding {
        child = (*elm).entry.rbe_left;
        current_block = 9386390421034826751;
    } else {
        let mut left: *mut key_binding = 0 as *mut key_binding;
        elm = (*elm).entry.rbe_right;
        loop  {
            left = (*elm).entry.rbe_left;
            if left.is_null() { break ; }
            elm = left
        }
        child = (*elm).entry.rbe_right;
        parent = (*elm).entry.rbe_parent;
        color = (*elm).entry.rbe_color;
        if !child.is_null() { (*child).entry.rbe_parent = parent }
        if !parent.is_null() {
            if (*parent).entry.rbe_left == elm {
                (*parent).entry.rbe_left = child
            } else { (*parent).entry.rbe_right = child }
            while 0 != 0i32 { }
        } else { (*head).rbh_root = child }
        if (*elm).entry.rbe_parent == old { parent = elm }
        (*elm).entry = (*old).entry;
        if !(*old).entry.rbe_parent.is_null() {
            if (*(*old).entry.rbe_parent).entry.rbe_left == old {
                (*(*old).entry.rbe_parent).entry.rbe_left = elm
            } else { (*(*old).entry.rbe_parent).entry.rbe_right = elm }
            while 0 != 0i32 { }
        } else { (*head).rbh_root = elm }
        (*(*old).entry.rbe_left).entry.rbe_parent = elm;
        if !(*old).entry.rbe_right.is_null() {
            (*(*old).entry.rbe_right).entry.rbe_parent = elm
        }
        if !parent.is_null() {
            left = parent;
            loop  {
                if 0 != 0i32 { continue ; }
                left = (*left).entry.rbe_parent;
                if left.is_null() { break ; }
            }
            current_block = 10131319541265114523;
        } else { current_block = 10131319541265114523; }
    }
    match current_block {
        9386390421034826751 => {
            parent = (*elm).entry.rbe_parent;
            color = (*elm).entry.rbe_color;
            if !child.is_null() { (*child).entry.rbe_parent = parent }
            if !parent.is_null() {
                if (*parent).entry.rbe_left == elm {
                    (*parent).entry.rbe_left = child
                } else { (*parent).entry.rbe_right = child }
                while 0 != 0i32 { }
            } else { (*head).rbh_root = child }
        }
        _ => { }
    }
    if color == 0i32 { key_bindings_RB_REMOVE_COLOR(head, parent, child); }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_RB_INSERT(mut head: *mut key_bindings,
                                                mut elm: *mut key_binding)
 -> *mut key_binding {
    let mut tmp: *mut key_binding = 0 as *mut key_binding;
    let mut parent: *mut key_binding = 0 as *mut key_binding;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = key_bindings_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    loop  {
        (*elm).entry.rbe_parent = parent;
        (*elm).entry.rbe_right = 0 as *mut key_binding;
        (*elm).entry.rbe_left = (*elm).entry.rbe_right;
        (*elm).entry.rbe_color = 1i32;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut key_binding {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else { (*parent).entry.rbe_right = elm }
        while 0 != 0i32 { }
    } else { (*head).rbh_root = elm }
    key_bindings_RB_INSERT_COLOR(head, elm);
    return 0 as *mut key_binding;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_cmp(mut bd1: *mut key_binding,
                                          mut bd2: *mut key_binding)
 -> libc::c_int {
    if (*bd1).key < (*bd2).key {
        return 1i32.wrapping_neg()
    } else if (*bd1).key > (*bd2).key { return 1i32 } else { return 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_RB_FIND(mut head: *mut key_bindings,
                                              mut elm: *mut key_binding)
 -> *mut key_binding {
    let mut tmp: *mut key_binding = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = key_bindings_cmp(elm, tmp);
            if comp < 0i32 {
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return 0 as *mut key_binding }
    };
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_RB_NFIND(mut head: *mut key_bindings,
                                               mut elm: *mut key_binding)
 -> *mut key_binding {
    let mut tmp: *mut key_binding = (*head).rbh_root;
    let mut res: *mut key_binding = 0 as *mut key_binding;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = key_bindings_cmp(elm, tmp);
            if comp < 0i32 {
                res = tmp;
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return res }
    };
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_RB_NEXT(mut elm: *mut key_binding)
 -> *mut key_binding {
    if !(*elm).entry.rbe_right.is_null() {
        elm = (*elm).entry.rbe_right;
        while !(*elm).entry.rbe_left.is_null() { elm = (*elm).entry.rbe_left }
    } else if !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_left {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_right {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_RB_PREV(mut elm: *mut key_binding)
 -> *mut key_binding {
    if !(*elm).entry.rbe_left.is_null() {
        elm = (*elm).entry.rbe_left;
        while !(*elm).entry.rbe_right.is_null() {
            elm = (*elm).entry.rbe_right
        }
    } else if !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_right {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_left {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_RB_MINMAX(mut head: *mut key_bindings,
                                                mut val: libc::c_int)
 -> *mut key_binding {
    let mut tmp: *mut key_binding = (*head).rbh_root;
    let mut parent: *mut key_binding = 0 as *mut key_binding;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else { tmp = (*tmp).entry.rbe_right }
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn key_tables_RB_INSERT_COLOR(mut head: *mut key_tables,
                                                    mut elm: *mut key_table)
 -> () {
    let mut current_block: u64;
    let mut parent: *mut key_table = 0 as *mut key_table;
    let mut gparent: *mut key_table = 0 as *mut key_table;
    let mut tmp: *mut key_table = 0 as *mut key_table;
    loop  {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() && (*parent).entry.rbe_color == 1i32) {
            break ;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                loop  {
                    (*parent).entry.rbe_color = 0i32;
                    (*gparent).entry.rbe_color = 1i32;
                    if !(0 != 0i32) { break ; }
                }
                elm = gparent
            } else {
                if (*parent).entry.rbe_right == elm {
                    current_block = 7351195479953500246;
                } else { current_block = 4956146061682418353; }
                's_38:
                    loop  {
                        match current_block {
                            4956146061682418353 => {
                                (*parent).entry.rbe_color = 0i32;
                                (*gparent).entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4956146061682418353;
                                } else { break ; }
                            }
                            _ => {
                                tmp = (*parent).entry.rbe_right;
                                (*parent).entry.rbe_right =
                                    (*tmp).entry.rbe_left;
                                if !(*parent).entry.rbe_right.is_null() {
                                    (*(*tmp).entry.rbe_left).entry.rbe_parent
                                        = parent
                                }
                                while 0 != 0i32 { }
                                (*tmp).entry.rbe_parent =
                                    (*parent).entry.rbe_parent;
                                if !(*tmp).entry.rbe_parent.is_null() {
                                    if parent ==
                                           (*(*parent).entry.rbe_parent).entry.rbe_left
                                       {
                                        (*(*parent).entry.rbe_parent).entry.rbe_left
                                            = tmp
                                    } else {
                                        (*(*parent).entry.rbe_parent).entry.rbe_right
                                            = tmp
                                    }
                                } else { (*head).rbh_root = tmp }
                                (*tmp).entry.rbe_left = parent;
                                (*parent).entry.rbe_parent = tmp;
                                while 0 != 0i32 { }
                                if !(*tmp).entry.rbe_parent.is_null() {
                                    current_block = 10048703153582371463;
                                } else {
                                    current_block = 10879442775620481940;
                                }
                                loop  {
                                    match current_block {
                                        10879442775620481940 => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    7351195479953500246;
                                                continue 's_38 ;
                                            } else { break ; }
                                        }
                                        _ => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    10048703153582371463;
                                            } else {
                                                current_block =
                                                    10879442775620481940;
                                            }
                                        }
                                    }
                                }
                                tmp = parent;
                                parent = elm;
                                elm = tmp;
                                current_block = 4956146061682418353;
                            }
                        }
                    }
                's_95:
                    loop  {
                        tmp = (*gparent).entry.rbe_left;
                        (*gparent).entry.rbe_left = (*tmp).entry.rbe_right;
                        if !(*gparent).entry.rbe_left.is_null() {
                            (*(*tmp).entry.rbe_right).entry.rbe_parent =
                                gparent
                        }
                        while 0 != 0i32 { }
                        (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                        if !(*tmp).entry.rbe_parent.is_null() {
                            if gparent ==
                                   (*(*gparent).entry.rbe_parent).entry.rbe_left
                               {
                                (*(*gparent).entry.rbe_parent).entry.rbe_left
                                    = tmp
                            } else {
                                (*(*gparent).entry.rbe_parent).entry.rbe_right
                                    = tmp
                            }
                        } else { (*head).rbh_root = tmp }
                        (*tmp).entry.rbe_right = gparent;
                        (*gparent).entry.rbe_parent = tmp;
                        while 0 != 0i32 { }
                        if !(*tmp).entry.rbe_parent.is_null() {
                            current_block = 6669252993407410313;
                        } else { current_block = 5948590327928692120; }
                        loop  {
                            match current_block {
                                6669252993407410313 => {
                                    if 0 != 0i32 {
                                        current_block = 6669252993407410313;
                                    } else {
                                        current_block = 5948590327928692120;
                                    }
                                }
                                _ => {
                                    if 0 != 0i32 {
                                        break ;
                                    } else { break 's_95 ; }
                                }
                            }
                        }
                    }
            }
        } else {
            tmp = (*gparent).entry.rbe_left;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                loop  {
                    (*parent).entry.rbe_color = 0i32;
                    (*gparent).entry.rbe_color = 1i32;
                    if !(0 != 0i32) { break ; }
                }
                elm = gparent
            } else {
                if (*parent).entry.rbe_left == elm {
                    current_block = 652864300344834934;
                } else { current_block = 4567019141635105728; }
                's_162:
                    loop  {
                        match current_block {
                            652864300344834934 => {
                                tmp = (*parent).entry.rbe_left;
                                (*parent).entry.rbe_left =
                                    (*tmp).entry.rbe_right;
                                if !(*parent).entry.rbe_left.is_null() {
                                    (*(*tmp).entry.rbe_right).entry.rbe_parent
                                        = parent
                                }
                                while 0 != 0i32 { }
                                (*tmp).entry.rbe_parent =
                                    (*parent).entry.rbe_parent;
                                if !(*tmp).entry.rbe_parent.is_null() {
                                    if parent ==
                                           (*(*parent).entry.rbe_parent).entry.rbe_left
                                       {
                                        (*(*parent).entry.rbe_parent).entry.rbe_left
                                            = tmp
                                    } else {
                                        (*(*parent).entry.rbe_parent).entry.rbe_right
                                            = tmp
                                    }
                                } else { (*head).rbh_root = tmp }
                                (*tmp).entry.rbe_right = parent;
                                (*parent).entry.rbe_parent = tmp;
                                while 0 != 0i32 { }
                                if !(*tmp).entry.rbe_parent.is_null() {
                                    current_block = 980989089337379490;
                                } else {
                                    current_block = 5494826135382683477;
                                }
                                loop  {
                                    match current_block {
                                        5494826135382683477 => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    652864300344834934;
                                                continue 's_162 ;
                                            } else { break ; }
                                        }
                                        _ => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    980989089337379490;
                                            } else {
                                                current_block =
                                                    5494826135382683477;
                                            }
                                        }
                                    }
                                }
                                tmp = parent;
                                parent = elm;
                                elm = tmp;
                                current_block = 4567019141635105728;
                            }
                            _ => {
                                (*parent).entry.rbe_color = 0i32;
                                (*gparent).entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4567019141635105728;
                                } else { break ; }
                            }
                        }
                    }
                's_219:
                    loop  {
                        tmp = (*gparent).entry.rbe_right;
                        (*gparent).entry.rbe_right = (*tmp).entry.rbe_left;
                        if !(*gparent).entry.rbe_right.is_null() {
                            (*(*tmp).entry.rbe_left).entry.rbe_parent =
                                gparent
                        }
                        while 0 != 0i32 { }
                        (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                        if !(*tmp).entry.rbe_parent.is_null() {
                            if gparent ==
                                   (*(*gparent).entry.rbe_parent).entry.rbe_left
                               {
                                (*(*gparent).entry.rbe_parent).entry.rbe_left
                                    = tmp
                            } else {
                                (*(*gparent).entry.rbe_parent).entry.rbe_right
                                    = tmp
                            }
                        } else { (*head).rbh_root = tmp }
                        (*tmp).entry.rbe_left = gparent;
                        (*gparent).entry.rbe_parent = tmp;
                        while 0 != 0i32 { }
                        if !(*tmp).entry.rbe_parent.is_null() {
                            current_block = 11793792312832361944;
                        } else { current_block = 2543120759711851213; }
                        loop  {
                            match current_block {
                                2543120759711851213 => {
                                    if 0 != 0i32 {
                                        break ;
                                    } else { break 's_219 ; }
                                }
                                _ => {
                                    if 0 != 0i32 {
                                        current_block = 11793792312832361944;
                                    } else {
                                        current_block = 2543120759711851213;
                                    }
                                }
                            }
                        }
                    }
            }
        }
    }
    (*(*head).rbh_root).entry.rbe_color = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn key_tables_RB_REMOVE_COLOR(mut head: *mut key_tables,
                                                    mut parent:
                                                        *mut key_table,
                                                    mut elm: *mut key_table)
 -> () {
    let mut current_block: u64;
    let mut tmp: *mut key_table = 0 as *mut key_table;
    's_4:
        loop  {
            if !((elm == 0 as *mut libc::c_void as *mut key_table ||
                      (*elm).entry.rbe_color == 0i32) &&
                     elm != (*head).rbh_root) {
                current_block = 11174649648027449784;
                break ;
            }
            if (*parent).entry.rbe_left == elm {
                tmp = (*parent).entry.rbe_right;
                if (*tmp).entry.rbe_color == 1i32 {
                    current_block = 17179679302217393232;
                } else { current_block = 14155750587950065367; }
                loop  {
                    match current_block {
                        14155750587950065367 => {
                            if ((*tmp).entry.rbe_left ==
                                    0 as *mut libc::c_void as *mut key_table
                                    ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut key_table ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_right ==
                                          0 as *mut libc::c_void as
                                              *mut key_table ||
                                          (*(*tmp).entry.rbe_right).entry.rbe_color
                                              == 0i32 {
                                current_block = 15976848397966268834;
                                break 's_4 ;
                            } else {
                                current_block = 7149356873433890176;
                                break 's_4 ;
                            }
                        }
                        _ => {
                            (*tmp).entry.rbe_color = 0i32;
                            (*parent).entry.rbe_color = 1i32;
                            if 0 != 0i32 {
                                current_block = 17179679302217393232;
                                continue ;
                            }
                            's_30:
                                loop  {
                                    tmp = (*parent).entry.rbe_right;
                                    (*parent).entry.rbe_right =
                                        (*tmp).entry.rbe_left;
                                    if !(*parent).entry.rbe_right.is_null() {
                                        (*(*tmp).entry.rbe_left).entry.rbe_parent
                                            = parent
                                    }
                                    while 0 != 0i32 { }
                                    (*tmp).entry.rbe_parent =
                                        (*parent).entry.rbe_parent;
                                    if !(*tmp).entry.rbe_parent.is_null() {
                                        if parent ==
                                               (*(*parent).entry.rbe_parent).entry.rbe_left
                                           {
                                            (*(*parent).entry.rbe_parent).entry.rbe_left
                                                = tmp
                                        } else {
                                            (*(*parent).entry.rbe_parent).entry.rbe_right
                                                = tmp
                                        }
                                    } else { (*head).rbh_root = tmp }
                                    (*tmp).entry.rbe_left = parent;
                                    (*parent).entry.rbe_parent = tmp;
                                    while 0 != 0i32 { }
                                    if !(*tmp).entry.rbe_parent.is_null() {
                                        current_block = 11050875288958768710;
                                    } else {
                                        current_block = 15240798224410183470;
                                    }
                                    loop  {
                                        match current_block {
                                            15240798224410183470 => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_30 ; }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        11050875288958768710;
                                                } else {
                                                    current_block =
                                                        15240798224410183470;
                                                }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).entry.rbe_right;
                            current_block = 14155750587950065367;
                        }
                    }
                }
            } else {
                tmp = (*parent).entry.rbe_left;
                if (*tmp).entry.rbe_color == 1i32 {
                    current_block = 6450597802325118133;
                } else { current_block = 7746103178988627676; }
                loop  {
                    match current_block {
                        6450597802325118133 => {
                            (*tmp).entry.rbe_color = 0i32;
                            (*parent).entry.rbe_color = 1i32;
                            if 0 != 0i32 {
                                current_block = 6450597802325118133;
                                continue ;
                            }
                            's_210:
                                loop  {
                                    tmp = (*parent).entry.rbe_left;
                                    (*parent).entry.rbe_left =
                                        (*tmp).entry.rbe_right;
                                    if !(*parent).entry.rbe_left.is_null() {
                                        (*(*tmp).entry.rbe_right).entry.rbe_parent
                                            = parent
                                    }
                                    while 0 != 0i32 { }
                                    (*tmp).entry.rbe_parent =
                                        (*parent).entry.rbe_parent;
                                    if !(*tmp).entry.rbe_parent.is_null() {
                                        if parent ==
                                               (*(*parent).entry.rbe_parent).entry.rbe_left
                                           {
                                            (*(*parent).entry.rbe_parent).entry.rbe_left
                                                = tmp
                                        } else {
                                            (*(*parent).entry.rbe_parent).entry.rbe_right
                                                = tmp
                                        }
                                    } else { (*head).rbh_root = tmp }
                                    (*tmp).entry.rbe_right = parent;
                                    (*parent).entry.rbe_parent = tmp;
                                    while 0 != 0i32 { }
                                    if !(*tmp).entry.rbe_parent.is_null() {
                                        current_block = 16738040538446813684;
                                    } else {
                                        current_block = 17784502470059252271;
                                    }
                                    loop  {
                                        match current_block {
                                            16738040538446813684 => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        16738040538446813684;
                                                } else {
                                                    current_block =
                                                        17784502470059252271;
                                                }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_210 ; }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).entry.rbe_left;
                            current_block = 7746103178988627676;
                        }
                        _ => {
                            if ((*tmp).entry.rbe_left ==
                                    0 as *mut libc::c_void as *mut key_table
                                    ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut key_table ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_left ==
                                          0 as *mut libc::c_void as
                                              *mut key_table ||
                                          (*(*tmp).entry.rbe_left).entry.rbe_color
                                              == 0i32 {
                                current_block = 13826291924415791078;
                                break 's_4 ;
                            } else {
                                current_block = 5892776923941496671;
                                break 's_4 ;
                            }
                        }
                    }
                }
            }
        }
    match current_block {
        15976848397966268834 => {
            let mut oleft: *mut key_table = 0 as *mut key_table;
            oleft = (*tmp).entry.rbe_left;
            if !oleft.is_null() { (*oleft).entry.rbe_color = 0i32 }
            (*tmp).entry.rbe_color = 1i32;
            's_96:
                loop  {
                    oleft = (*tmp).entry.rbe_left;
                    (*tmp).entry.rbe_left = (*oleft).entry.rbe_right;
                    if !(*tmp).entry.rbe_left.is_null() {
                        (*(*oleft).entry.rbe_right).entry.rbe_parent = tmp
                    }
                    while 0 != 0i32 { }
                    (*oleft).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oleft).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oleft
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oleft
                        }
                    } else { (*head).rbh_root = oleft }
                    (*oleft).entry.rbe_right = tmp;
                    (*tmp).entry.rbe_parent = oleft;
                    while 0 != 0i32 { }
                    if !(*oleft).entry.rbe_parent.is_null() {
                        current_block = 2232869372362427478;
                    } else { current_block = 15904375183555213903; }
                    loop  {
                        match current_block {
                            15904375183555213903 => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_96 ; }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    current_block = 2232869372362427478;
                                } else {
                                    current_block = 15904375183555213903;
                                }
                            }
                        }
                    }
                }
            tmp = (*parent).entry.rbe_right;
            current_block = 7149356873433890176;
        }
        13826291924415791078 => {
            let mut oright: *mut key_table = 0 as *mut key_table;
            oright = (*tmp).entry.rbe_right;
            if !oright.is_null() { (*oright).entry.rbe_color = 0i32 }
            (*tmp).entry.rbe_color = 1i32;
            's_276:
                loop  {
                    oright = (*tmp).entry.rbe_right;
                    (*tmp).entry.rbe_right = (*oright).entry.rbe_left;
                    if !(*tmp).entry.rbe_right.is_null() {
                        (*(*oright).entry.rbe_left).entry.rbe_parent = tmp
                    }
                    while 0 != 0i32 { }
                    (*oright).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oright).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oright
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right =
                                oright
                        }
                    } else { (*head).rbh_root = oright }
                    (*oright).entry.rbe_left = tmp;
                    (*tmp).entry.rbe_parent = oright;
                    while 0 != 0i32 { }
                    if !(*oright).entry.rbe_parent.is_null() {
                        current_block = 3392087639489470149;
                    } else { current_block = 1854459640724737493; }
                    loop  {
                        match current_block {
                            1854459640724737493 => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_276 ; }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    current_block = 3392087639489470149;
                                } else {
                                    current_block = 1854459640724737493;
                                }
                            }
                        }
                    }
                }
            tmp = (*parent).entry.rbe_left;
            current_block = 5892776923941496671;
        }
        _ => { }
    }
    match current_block {
        7149356873433890176 => {
            (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
            (*parent).entry.rbe_color = 0i32;
            if !(*tmp).entry.rbe_right.is_null() {
                (*(*tmp).entry.rbe_right).entry.rbe_color = 0i32
            }
            's_148:
                loop  {
                    tmp = (*parent).entry.rbe_right;
                    (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                    if !(*parent).entry.rbe_right.is_null() {
                        (*(*tmp).entry.rbe_left).entry.rbe_parent = parent
                    }
                    while 0 != 0i32 { }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent ==
                               (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right =
                                tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).entry.rbe_left = parent;
                    (*parent).entry.rbe_parent = tmp;
                    while 0 != 0i32 { }
                    if !(*tmp).entry.rbe_parent.is_null() {
                        current_block = 6450636197030046351;
                    } else { current_block = 16924917904204750491; }
                    loop  {
                        match current_block {
                            6450636197030046351 => {
                                if 0 != 0i32 {
                                    current_block = 6450636197030046351;
                                } else {
                                    current_block = 16924917904204750491;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_148 ; }
                            }
                        }
                    }
                }
            elm = (*head).rbh_root
        }
        5892776923941496671 => {
            (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
            (*parent).entry.rbe_color = 0i32;
            if !(*tmp).entry.rbe_left.is_null() {
                (*(*tmp).entry.rbe_left).entry.rbe_color = 0i32
            }
            's_328:
                loop  {
                    tmp = (*parent).entry.rbe_left;
                    (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                    if !(*parent).entry.rbe_left.is_null() {
                        (*(*tmp).entry.rbe_right).entry.rbe_parent = parent
                    }
                    while 0 != 0i32 { }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent ==
                               (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right =
                                tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).entry.rbe_right = parent;
                    (*parent).entry.rbe_parent = tmp;
                    while 0 != 0i32 { }
                    if !(*tmp).entry.rbe_parent.is_null() {
                        current_block = 13910774313357589740;
                    } else { current_block = 13707613154239713890; }
                    loop  {
                        match current_block {
                            13910774313357589740 => {
                                if 0 != 0i32 {
                                    current_block = 13910774313357589740;
                                } else {
                                    current_block = 13707613154239713890;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_328 ; }
                            }
                        }
                    }
                }
            elm = (*head).rbh_root
        }
        _ => { }
    }
    if !elm.is_null() { (*elm).entry.rbe_color = 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn key_tables_RB_REMOVE(mut head: *mut key_tables,
                                              mut elm: *mut key_table)
 -> *mut key_table {
    let mut current_block: u64;
    let mut child: *mut key_table = 0 as *mut key_table;
    let mut parent: *mut key_table = 0 as *mut key_table;
    let mut old: *mut key_table = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left == 0 as *mut libc::c_void as *mut key_table {
        child = (*elm).entry.rbe_right;
        current_block = 9386390421034826751;
    } else if (*elm).entry.rbe_right ==
                  0 as *mut libc::c_void as *mut key_table {
        child = (*elm).entry.rbe_left;
        current_block = 9386390421034826751;
    } else {
        let mut left: *mut key_table = 0 as *mut key_table;
        elm = (*elm).entry.rbe_right;
        loop  {
            left = (*elm).entry.rbe_left;
            if left.is_null() { break ; }
            elm = left
        }
        child = (*elm).entry.rbe_right;
        parent = (*elm).entry.rbe_parent;
        color = (*elm).entry.rbe_color;
        if !child.is_null() { (*child).entry.rbe_parent = parent }
        if !parent.is_null() {
            if (*parent).entry.rbe_left == elm {
                (*parent).entry.rbe_left = child
            } else { (*parent).entry.rbe_right = child }
            while 0 != 0i32 { }
        } else { (*head).rbh_root = child }
        if (*elm).entry.rbe_parent == old { parent = elm }
        (*elm).entry = (*old).entry;
        if !(*old).entry.rbe_parent.is_null() {
            if (*(*old).entry.rbe_parent).entry.rbe_left == old {
                (*(*old).entry.rbe_parent).entry.rbe_left = elm
            } else { (*(*old).entry.rbe_parent).entry.rbe_right = elm }
            while 0 != 0i32 { }
        } else { (*head).rbh_root = elm }
        (*(*old).entry.rbe_left).entry.rbe_parent = elm;
        if !(*old).entry.rbe_right.is_null() {
            (*(*old).entry.rbe_right).entry.rbe_parent = elm
        }
        if !parent.is_null() {
            left = parent;
            loop  {
                if 0 != 0i32 { continue ; }
                left = (*left).entry.rbe_parent;
                if left.is_null() { break ; }
            }
            current_block = 9380139882745843906;
        } else { current_block = 9380139882745843906; }
    }
    match current_block {
        9386390421034826751 => {
            parent = (*elm).entry.rbe_parent;
            color = (*elm).entry.rbe_color;
            if !child.is_null() { (*child).entry.rbe_parent = parent }
            if !parent.is_null() {
                if (*parent).entry.rbe_left == elm {
                    (*parent).entry.rbe_left = child
                } else { (*parent).entry.rbe_right = child }
                while 0 != 0i32 { }
            } else { (*head).rbh_root = child }
        }
        _ => { }
    }
    if color == 0i32 { key_tables_RB_REMOVE_COLOR(head, parent, child); }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn key_tables_RB_INSERT(mut head: *mut key_tables,
                                              mut elm: *mut key_table)
 -> *mut key_table {
    let mut tmp: *mut key_table = 0 as *mut key_table;
    let mut parent: *mut key_table = 0 as *mut key_table;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = key_table_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    loop  {
        (*elm).entry.rbe_parent = parent;
        (*elm).entry.rbe_right = 0 as *mut key_table;
        (*elm).entry.rbe_left = (*elm).entry.rbe_right;
        (*elm).entry.rbe_color = 1i32;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut key_table {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else { (*parent).entry.rbe_right = elm }
        while 0 != 0i32 { }
    } else { (*head).rbh_root = elm }
    key_tables_RB_INSERT_COLOR(head, elm);
    return 0 as *mut key_table;
}
#[no_mangle]
pub unsafe extern "C" fn key_table_cmp(mut e1: *mut key_table,
                                       mut e2: *mut key_table)
 -> libc::c_int {
    return strcmp((*e1).name, (*e2).name);
}
#[no_mangle]
pub unsafe extern "C" fn key_tables_RB_FIND(mut head: *mut key_tables,
                                            mut elm: *mut key_table)
 -> *mut key_table {
    let mut tmp: *mut key_table = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = key_table_cmp(elm, tmp);
            if comp < 0i32 {
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return 0 as *mut key_table }
    };
}
#[no_mangle]
pub unsafe extern "C" fn key_tables_RB_NFIND(mut head: *mut key_tables,
                                             mut elm: *mut key_table)
 -> *mut key_table {
    let mut tmp: *mut key_table = (*head).rbh_root;
    let mut res: *mut key_table = 0 as *mut key_table;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = key_table_cmp(elm, tmp);
            if comp < 0i32 {
                res = tmp;
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return res }
    };
}
#[no_mangle]
pub unsafe extern "C" fn key_tables_RB_NEXT(mut elm: *mut key_table)
 -> *mut key_table {
    if !(*elm).entry.rbe_right.is_null() {
        elm = (*elm).entry.rbe_right;
        while !(*elm).entry.rbe_left.is_null() { elm = (*elm).entry.rbe_left }
    } else if !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_left {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_right {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn key_tables_RB_PREV(mut elm: *mut key_table)
 -> *mut key_table {
    if !(*elm).entry.rbe_left.is_null() {
        elm = (*elm).entry.rbe_left;
        while !(*elm).entry.rbe_right.is_null() {
            elm = (*elm).entry.rbe_right
        }
    } else if !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_right {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_left {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn key_tables_RB_MINMAX(mut head: *mut key_tables,
                                              mut val: libc::c_int)
 -> *mut key_table {
    let mut tmp: *mut key_table = (*head).rbh_root;
    let mut parent: *mut key_table = 0 as *mut key_table;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else { tmp = (*tmp).entry.rbe_right }
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_get_table(mut name: *const libc::c_char,
                                                mut create: libc::c_int)
 -> *mut key_table {
    let mut table_find: key_table =
        key_table{name: 0 as *const libc::c_char,
                  key_bindings:
                      key_bindings{rbh_root: 0 as *mut key_binding,},
                  references: 0,
                  entry:
                      unnamed_38{rbe_left: 0 as *mut key_table,
                                 rbe_right: 0 as *mut key_table,
                                 rbe_parent: 0 as *mut key_table,
                                 rbe_color: 0,},};
    let mut table: *mut key_table = 0 as *mut key_table;
    table_find.name = name;
    table =
        key_tables_RB_FIND(&mut key_tables as *mut key_tables,
                           &mut table_find as *mut key_table);
    if table != 0 as *mut libc::c_void as *mut key_table || 0 == create {
        return table
    } else {
        table =
            xmalloc(::std::mem::size_of::<key_table>() as libc::c_ulong) as
                *mut key_table;
        (*table).name = xstrdup(name);
        loop  {
            let ref mut fresh0 =
                (*(&mut (*table).key_bindings as *mut key_bindings)).rbh_root;
            *fresh0 = 0 as *mut key_binding;
            if !(0 != 0i32) { break ; }
        }
        (*table).references = 1i32 as u_int;
        key_tables_RB_INSERT(&mut key_tables as *mut key_tables, table);
        return table
    };
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_unref_table(mut table: *mut key_table)
 -> () {
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut bd1: *mut key_binding = 0 as *mut key_binding;
    (*table).references = (*table).references.wrapping_sub(1);
    if (*table).references != 0i32 as libc::c_uint {
        return
    } else {
        bd =
            key_bindings_RB_MINMAX(&mut (*table).key_bindings as
                                       *mut key_bindings,
                                   1i32.wrapping_neg());
        while bd != 0 as *mut libc::c_void as *mut key_binding &&
                  { bd1 = key_bindings_RB_NEXT(bd); 0 != 1i32 } {
            key_bindings_RB_REMOVE(&mut (*table).key_bindings as
                                       *mut key_bindings, bd);
            cmd_list_free((*bd).cmdlist);
            free(bd as *mut libc::c_void);
            bd = bd1
        }
        free((*table).name as *mut libc::c_void);
        free(table as *mut libc::c_void);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_add(mut name: *const libc::c_char,
                                          mut key: key_code,
                                          mut repeat: libc::c_int,
                                          mut cmdlist: *mut cmd_list) -> () {
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut bd_find: key_binding =
        key_binding{key: 0,
                    cmdlist: 0 as *mut cmd_list,
                    flags: 0,
                    entry:
                        unnamed_9{rbe_left: 0 as *mut key_binding,
                                  rbe_right: 0 as *mut key_binding,
                                  rbe_parent: 0 as *mut key_binding,
                                  rbe_color: 0,},};
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    table = key_bindings_get_table(name, 1i32);
    bd_find.key = key & !281474976710656u64;
    bd =
        key_bindings_RB_FIND(&mut (*table).key_bindings as *mut key_bindings,
                             &mut bd_find as *mut key_binding);
    if bd != 0 as *mut libc::c_void as *mut key_binding {
        key_bindings_RB_REMOVE(&mut (*table).key_bindings as
                                   *mut key_bindings, bd);
        cmd_list_free((*bd).cmdlist);
        free(bd as *mut libc::c_void);
    }
    bd =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<key_binding>() as libc::c_ulong) as
            *mut key_binding;
    (*bd).key = key;
    key_bindings_RB_INSERT(&mut (*table).key_bindings as *mut key_bindings,
                           bd);
    if 0 != repeat { (*bd).flags |= 1i32 }
    (*bd).cmdlist = cmdlist;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_remove(mut name: *const libc::c_char,
                                             mut key: key_code) -> () {
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut bd_find: key_binding =
        key_binding{key: 0,
                    cmdlist: 0 as *mut cmd_list,
                    flags: 0,
                    entry:
                        unnamed_9{rbe_left: 0 as *mut key_binding,
                                  rbe_right: 0 as *mut key_binding,
                                  rbe_parent: 0 as *mut key_binding,
                                  rbe_color: 0,},};
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    table = key_bindings_get_table(name, 0i32);
    if table == 0 as *mut libc::c_void as *mut key_table {
        return
    } else {
        bd_find.key = key & !281474976710656u64;
        bd =
            key_bindings_RB_FIND(&mut (*table).key_bindings as
                                     *mut key_bindings,
                                 &mut bd_find as *mut key_binding);
        if bd == 0 as *mut libc::c_void as *mut key_binding {
            return
        } else {
            key_bindings_RB_REMOVE(&mut (*table).key_bindings as
                                       *mut key_bindings, bd);
            cmd_list_free((*bd).cmdlist);
            free(bd as *mut libc::c_void);
            if (*(&mut (*table).key_bindings as *mut key_bindings)).rbh_root
                   == 0 as *mut libc::c_void as *mut key_binding {
                key_tables_RB_REMOVE(&mut key_tables as *mut key_tables,
                                     table);
                key_bindings_unref_table(table);
            }
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_remove_table(mut name:
                                                       *const libc::c_char)
 -> () {
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut c: *mut client = 0 as *mut client;
    table = key_bindings_get_table(name, 0i32);
    if table != 0 as *mut libc::c_void as *mut key_table {
        key_tables_RB_REMOVE(&mut key_tables as *mut key_tables, table);
        key_bindings_unref_table(table);
    }
    c = (*(&mut clients as *mut clients)).tqh_first;
    while c != 0 as *mut libc::c_void as *mut client {
        if (*c).keytable == table {
            server_client_set_key_table(c, 0 as *const libc::c_char);
        }
        c = (*c).entry.tqe_next
    };
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_init() -> () {
    static mut defaults: [*const libc::c_char; 227] =
        unsafe {
            [b"bind C-b send-prefix\x00" as *const u8 as *const libc::c_char,
             b"bind C-o rotate-window\x00" as *const u8 as
                 *const libc::c_char,
             b"bind C-z suspend-client\x00" as *const u8 as
                 *const libc::c_char,
             b"bind Space next-layout\x00" as *const u8 as
                 *const libc::c_char,
             b"bind ! break-pane\x00" as *const u8 as *const libc::c_char,
             b"bind \'\"\' split-window\x00" as *const u8 as
                 *const libc::c_char,
             b"bind \'#\' list-buffers\x00" as *const u8 as
                 *const libc::c_char,
             b"bind \'$\' command-prompt -I\'#S\' \"rename-session -- \'%%\'\"\x00"
                 as *const u8 as *const libc::c_char,
             b"bind % split-window -h\x00" as *const u8 as
                 *const libc::c_char,
             b"bind & confirm-before -p\"kill-window #W? (y/n)\" kill-window\x00"
                 as *const u8 as *const libc::c_char,
             b"bind \"\'\" command-prompt -pindex \"select-window -t \':%%\'\"\x00"
                 as *const u8 as *const libc::c_char,
             b"bind ( switch-client -p\x00" as *const u8 as
                 *const libc::c_char,
             b"bind ) switch-client -n\x00" as *const u8 as
                 *const libc::c_char,
             b"bind , command-prompt -I\'#W\' \"rename-window -- \'%%\'\"\x00"
                 as *const u8 as *const libc::c_char,
             b"bind - delete-buffer\x00" as *const u8 as *const libc::c_char,
             b"bind . command-prompt \"move-window -t \'%%\'\"\x00" as
                 *const u8 as *const libc::c_char,
             b"bind 0 select-window -t:=0\x00" as *const u8 as
                 *const libc::c_char,
             b"bind 1 select-window -t:=1\x00" as *const u8 as
                 *const libc::c_char,
             b"bind 2 select-window -t:=2\x00" as *const u8 as
                 *const libc::c_char,
             b"bind 3 select-window -t:=3\x00" as *const u8 as
                 *const libc::c_char,
             b"bind 4 select-window -t:=4\x00" as *const u8 as
                 *const libc::c_char,
             b"bind 5 select-window -t:=5\x00" as *const u8 as
                 *const libc::c_char,
             b"bind 6 select-window -t:=6\x00" as *const u8 as
                 *const libc::c_char,
             b"bind 7 select-window -t:=7\x00" as *const u8 as
                 *const libc::c_char,
             b"bind 8 select-window -t:=8\x00" as *const u8 as
                 *const libc::c_char,
             b"bind 9 select-window -t:=9\x00" as *const u8 as
                 *const libc::c_char,
             b"bind : command-prompt\x00" as *const u8 as *const libc::c_char,
             b"bind \\; last-pane\x00" as *const u8 as *const libc::c_char,
             b"bind = choose-buffer -Z\x00" as *const u8 as
                 *const libc::c_char,
             b"bind ? list-keys\x00" as *const u8 as *const libc::c_char,
             b"bind D choose-client -Z\x00" as *const u8 as
                 *const libc::c_char,
             b"bind E select-layout -E\x00" as *const u8 as
                 *const libc::c_char,
             b"bind L switch-client -l\x00" as *const u8 as
                 *const libc::c_char,
             b"bind M select-pane -M\x00" as *const u8 as *const libc::c_char,
             b"bind [ copy-mode\x00" as *const u8 as *const libc::c_char,
             b"bind ] paste-buffer\x00" as *const u8 as *const libc::c_char,
             b"bind c new-window\x00" as *const u8 as *const libc::c_char,
             b"bind d detach-client\x00" as *const u8 as *const libc::c_char,
             b"bind f command-prompt \"find-window -- \'%%\'\"\x00" as
                 *const u8 as *const libc::c_char,
             b"bind i display-message\x00" as *const u8 as
                 *const libc::c_char,
             b"bind l last-window\x00" as *const u8 as *const libc::c_char,
             b"bind m select-pane -m\x00" as *const u8 as *const libc::c_char,
             b"bind n next-window\x00" as *const u8 as *const libc::c_char,
             b"bind o select-pane -t:.+\x00" as *const u8 as
                 *const libc::c_char,
             b"bind p previous-window\x00" as *const u8 as
                 *const libc::c_char,
             b"bind q display-panes\x00" as *const u8 as *const libc::c_char,
             b"bind r refresh-client\x00" as *const u8 as *const libc::c_char,
             b"bind s choose-tree -Zs\x00" as *const u8 as
                 *const libc::c_char,
             b"bind t clock-mode\x00" as *const u8 as *const libc::c_char,
             b"bind w choose-tree -Zw\x00" as *const u8 as
                 *const libc::c_char,
             b"bind x confirm-before -p\"kill-pane #P? (y/n)\" kill-pane\x00"
                 as *const u8 as *const libc::c_char,
             b"bind z resize-pane -Z\x00" as *const u8 as *const libc::c_char,
             b"bind { swap-pane -U\x00" as *const u8 as *const libc::c_char,
             b"bind } swap-pane -D\x00" as *const u8 as *const libc::c_char,
             b"bind \'~\' show-messages\x00" as *const u8 as
                 *const libc::c_char,
             b"bind PPage copy-mode -u\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -r Up select-pane -U\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -r Down select-pane -D\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -r Left select-pane -L\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -r Right select-pane -R\x00" as *const u8 as
                 *const libc::c_char,
             b"bind M-1 select-layout even-horizontal\x00" as *const u8 as
                 *const libc::c_char,
             b"bind M-2 select-layout even-vertical\x00" as *const u8 as
                 *const libc::c_char,
             b"bind M-3 select-layout main-horizontal\x00" as *const u8 as
                 *const libc::c_char,
             b"bind M-4 select-layout main-vertical\x00" as *const u8 as
                 *const libc::c_char,
             b"bind M-5 select-layout tiled\x00" as *const u8 as
                 *const libc::c_char,
             b"bind M-n next-window -a\x00" as *const u8 as
                 *const libc::c_char,
             b"bind M-o rotate-window -D\x00" as *const u8 as
                 *const libc::c_char,
             b"bind M-p previous-window -a\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -r M-Up resize-pane -U 5\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -r M-Down resize-pane -D 5\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -r M-Left resize-pane -L 5\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -r M-Right resize-pane -R 5\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -r C-Up resize-pane -U\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -r C-Down resize-pane -D\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -r C-Left resize-pane -L\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -r C-Right resize-pane -R\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -n MouseDown1Pane select-pane -t=\\; send-keys -M\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -n MouseDrag1Border resize-pane -M\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -n MouseDown1Status select-window -t=\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -n WheelDownStatus next-window\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -n WheelUpStatus previous-window\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -n MouseDrag1Pane if -Ft= \'#{mouse_any_flag}\' \'if -Ft= \"#{pane_in_mode}\" \"copy-mode -M\" \"send-keys -M\"\' \'copy-mode -M\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -n MouseDown3Pane if-shell -Ft= \'#{mouse_any_flag}\' \'select-pane -t=; send-keys -M\' \'select-pane -mt=\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -n WheelUpPane if-shell -Ft= \'#{mouse_any_flag}\' \'send-keys -M\' \'if -Ft= \"#{pane_in_mode}\" \"send-keys -M\" \"copy-mode -et=\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode C-Space send -X begin-selection\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode C-a send -X start-of-line\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode C-c send -X cancel\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode C-e send -X end-of-line\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode C-f send -X cursor-right\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode C-b send -X cursor-left\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode C-g send -X clear-selection\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode C-k send -X copy-end-of-line\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode C-n send -X cursor-down\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode C-p send -X cursor-up\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode C-r command-prompt -ip\'(search up)\' -I\'#{pane_search_string}\' \'send -X search-backward-incremental \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode C-s command-prompt -ip\'(search down)\' -I\'#{pane_search_string}\' \'send -X search-forward-incremental \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode C-v send -X page-down\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode C-w send -X copy-selection-and-cancel\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode Escape send -X cancel\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode Space send -X page-down\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode , send -X jump-reverse\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode \\; send -X jump-again\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode F command-prompt -1p\'(jump backward)\' \'send -X jump-backward \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode N send -X search-reverse\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode R send -X rectangle-toggle\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode T command-prompt -1p\'(jump to backward)\' \'send -X jump-to-backward \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode f command-prompt -1p\'(jump forward)\' \'send -X jump-forward \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode g command-prompt -p\'(goto line)\' \'send -X goto-line \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode n send -X search-again\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode q send -X cancel\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode t command-prompt -1p\'(jump to forward)\' \'send -X jump-to-forward \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode Home send -X start-of-line\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode End send -X end-of-line\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode MouseDown1Pane select-pane\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode MouseDrag1Pane select-pane\\; send -X begin-selection\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode MouseDragEnd1Pane send -X copy-selection-and-cancel\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode WheelUpPane select-pane\\; send -N5 -X scroll-up\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode WheelDownPane select-pane\\; send -N5 -X scroll-down\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode DoubleClick1Pane select-pane\\; send -X select-word\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode TripleClick1Pane select-pane\\; send -X select-line\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode NPage send -X page-down\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode PPage send -X page-up\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode Up send -X cursor-up\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode Down send -X cursor-down\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode Left send -X cursor-left\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode Right send -X cursor-right\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode M-1 command-prompt -Np\'(repeat)\' -I1 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode M-2 command-prompt -Np\'(repeat)\' -I2 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode M-3 command-prompt -Np\'(repeat)\' -I3 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode M-4 command-prompt -Np\'(repeat)\' -I4 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode M-5 command-prompt -Np\'(repeat)\' -I5 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode M-6 command-prompt -Np\'(repeat)\' -I6 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode M-7 command-prompt -Np\'(repeat)\' -I7 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode M-8 command-prompt -Np\'(repeat)\' -I8 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode M-9 command-prompt -Np\'(repeat)\' -I9 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode M-< send -X history-top\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode M-> send -X history-bottom\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode M-R send -X top-line\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode M-b send -X previous-word\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode M-f send -X next-word-end\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode M-m send -X back-to-indentation\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode M-r send -X middle-line\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode M-v send -X page-up\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode M-w send -X copy-selection-and-cancel\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode M-{ send -X previous-paragraph\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode M-} send -X next-paragraph\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode M-Up send -X halfpage-up\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode M-Down send -X halfpage-down\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode C-Up send -X scroll-up\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode C-Down send -X scroll-down\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi C-c send -X cancel\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi C-d send -X halfpage-down\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi C-e send -X scroll-down\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi C-b send -X page-up\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi C-f send -X page-down\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi C-h send -X cursor-left\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi C-j send -X copy-selection-and-cancel\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi Enter send -X copy-selection-and-cancel\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi C-u send -X halfpage-up\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi C-v send -X rectangle-toggle\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi C-y send -X scroll-up\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi Escape send -X clear-selection\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi Space send -X begin-selection\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi \'$\' send -X end-of-line\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi , send -X jump-reverse\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi / command-prompt -p\'(search down)\' \'send -X search-forward \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi 0 send -X start-of-line\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi 1 command-prompt -Np\'(repeat)\' -I1 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi 2 command-prompt -Np\'(repeat)\' -I2 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi 3 command-prompt -Np\'(repeat)\' -I3 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi 4 command-prompt -Np\'(repeat)\' -I4 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi 5 command-prompt -Np\'(repeat)\' -I5 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi 6 command-prompt -Np\'(repeat)\' -I6 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi 7 command-prompt -Np\'(repeat)\' -I7 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi 8 command-prompt -Np\'(repeat)\' -I8 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi 9 command-prompt -Np\'(repeat)\' -I9 \'send -N \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi : command-prompt -p\'(goto line)\' \'send -X goto-line \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi \\; send -X jump-again\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi ? command-prompt -p\'(search up)\' \'send -X search-backward \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi A send -X append-selection-and-cancel\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi B send -X previous-space\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi D send -X copy-end-of-line\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi E send -X next-space-end\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi F command-prompt -1p\'(jump backward)\' \'send -X jump-backward \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi G send -X history-bottom\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi H send -X top-line\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi J send -X scroll-down\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi K send -X scroll-up\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi L send -X bottom-line\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi M send -X middle-line\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi N send -X search-reverse\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi T command-prompt -1p\'(jump to backward)\' \'send -X jump-to-backward \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi V send -X select-line\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi W send -X next-space\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi ^ send -X back-to-indentation\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi b send -X previous-word\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi e send -X next-word-end\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi f command-prompt -1p\'(jump forward)\' \'send -X jump-forward \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi g send -X history-top\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi h send -X cursor-left\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi j send -X cursor-down\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi k send -X cursor-up\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi l send -X cursor-right\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi n send -X search-again\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi o send -X other-end\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi q send -X cancel\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi t command-prompt -1p\'(jump to forward)\' \'send -X jump-to-forward \"%%%\"\'\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi v send -X rectangle-toggle\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi w send -X next-word\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi { send -X previous-paragraph\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi } send -X next-paragraph\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi MouseDown1Pane select-pane\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi MouseDrag1Pane select-pane\\; send -X begin-selection\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi MouseDragEnd1Pane send -X copy-selection-and-cancel\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi WheelUpPane select-pane\\; send -N5 -X scroll-up\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi WheelDownPane select-pane\\; send -N5 -X scroll-down\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi DoubleClick1Pane select-pane\\; send -X select-word\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi TripleClick1Pane select-pane\\; send -X select-line\x00"
                 as *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi BSpace send -X cursor-left\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi NPage send -X page-down\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi PPage send -X page-up\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi Up send -X cursor-up\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi Down send -X cursor-down\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi Left send -X cursor-left\x00" as *const u8
                 as *const libc::c_char,
             b"bind -Tcopy-mode-vi Right send -X cursor-right\x00" as
                 *const u8 as *const libc::c_char,
             b"bind -Tcopy-mode-vi C-Up send -X scroll-up\x00" as *const u8 as
                 *const libc::c_char,
             b"bind -Tcopy-mode-vi C-Down send -X scroll-down\x00" as
                 *const u8 as *const libc::c_char]
        };
    let mut i: u_int = 0;
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0i32 as u_int;
    loop  {
        if (i as libc::c_ulong) <
               (::std::mem::size_of::<[*const libc::c_char; 227]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                    as libc::c_ulong) {
            cmdlist =
                cmd_string_parse(defaults[i as usize],
                                 b"<default>\x00" as *const u8 as
                                     *const libc::c_char, i,
                                 &mut cause as *mut *mut libc::c_char);
            if cmdlist == 0 as *mut libc::c_void as *mut cmd_list {
                fatalx(b"bad default key: %s\x00" as *const u8 as
                           *const libc::c_char, defaults[i as usize]);
            } else {
                cmdq_append(0 as *mut client,
                            cmdq_get_command(cmdlist,
                                             0 as *mut cmd_find_state,
                                             0 as *mut mouse_event, 0i32));
                cmd_list_free(cmdlist);
                i = i.wrapping_add(1)
            }
        } else { return; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_dispatch(mut bd: *mut key_binding,
                                               mut item: *mut cmdq_item,
                                               mut c: *mut client,
                                               mut m: *mut mouse_event,
                                               mut fs: *mut cmd_find_state)
 -> () {
    let mut cmd: *mut cmd = 0 as *mut cmd;
    let mut new_item: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut readonly: libc::c_int = 0;
    readonly = 1i32;
    cmd = (*(&mut (*(*bd).cmdlist).list as *mut unnamed_30)).tqh_first;
    while cmd != 0 as *mut libc::c_void as *mut cmd {
        if 0 == (*(*cmd).entry).flags & 2i32 { readonly = 0i32 }
        cmd = (*cmd).qentry.tqe_next
    }
    if 0 == readonly && 0 != (*c).flags & 2048i32 {
        new_item =
            cmdq_get_callback1(b"key_bindings_read_only\x00" as *const u8 as
                                   *const libc::c_char,
                               Some(key_bindings_read_only),
                               0 as *mut libc::c_void)
    } else {
        new_item = cmdq_get_command((*bd).cmdlist, fs, m, 0i32);
        if 0 != (*bd).flags & 1i32 { (*(*new_item).shared).flags |= 1i32 }
    }
    if item != 0 as *mut libc::c_void as *mut cmdq_item {
        cmdq_insert_after(item, new_item);
    } else { cmdq_append(c, new_item); };
}
unsafe extern "C" fn key_bindings_read_only(mut item: *mut cmdq_item,
                                            mut data: *mut libc::c_void)
 -> cmd_retval {
    cmdq_error(item,
               b"client is read-only\x00" as *const u8 as
                   *const libc::c_char);
    return CMD_RETURN_ERROR;
}

