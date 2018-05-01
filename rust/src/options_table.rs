extern crate libc;
extern "C" {
    pub type evbuffer;
    pub type tty_code;
    pub type event_base;
    pub type tmuxpeer;
    pub type format_job_tree;
    pub type tmuxproc;
    pub type options;
    pub type args_entry;
    pub type screen_titles;
    pub type input_ctx;
    pub type hooks;
    pub type format_tree;
    pub type bufferevent_ops;
    pub type environ;
    pub type _IO_FILE_plus;
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
pub struct unnamed {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_10,
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
    pub entry: unnamed_15,
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
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub type unnamed_2 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const TTY_VT102: unnamed_2 = 2;
pub type uint8_t = libc::c_uchar;
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
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub type cc_t = libc::c_uchar;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_4 {
    ev_next_with_common_timeout: unnamed_28,
    min_heap_idx: libc::c_int,
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
    pub entry: unnamed_24,
}
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type key_code = libc::c_ulonglong;
pub const TTY_VT420: unnamed_2 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub ev_io_next: unnamed_14,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub const LINE_SEL_NONE: unnamed_30 = 0;
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub type bitstr_t = libc::c_uchar;
pub const PROMPT_COMMAND: unnamed_27 = 1;
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
    pub entry: unnamed_29,
    pub wentry: unnamed_32,
    pub sentry: unnamed_19,
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
    pub entry: unnamed_1,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_9,
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
    pub gentry: unnamed_20,
    pub entry: unnamed_26,
}
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub type unnamed_10 = libc::c_uint;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
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
    pub alerts_entry: unnamed_34,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_5,
    pub entry: unnamed,
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_18,
    pub entry: unnamed_13,
}
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_21,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_17,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_30 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
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
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_30,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type __time_t = libc::c_long;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const JOB_RUNNING: unnamed_10 = 0;
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const TTY_VT320: unnamed_2 = 4;
pub const LINE_SEL_RIGHT_LEFT: unnamed_30 = 2;
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
pub type options_table_type = libc::c_uint;
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_22,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const TTY_VT220: unnamed_2 = 3;
pub type pid_t = __pid_t;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_31,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type u_int = __u_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_7,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_23 {
    ev_io: unnamed_6,
    ev_signal: unnamed_25,
}
pub type uint32_t = libc::c_uint;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const PROMPT_ENTRY: unnamed_27 = 0;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
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
    pub tree_entry: unnamed_8,
}
pub type u_char = __u_char;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub ev_signal_next: unnamed_37,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_36,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_33,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub const TTY_VT100: unnamed_2 = 0;
pub const JOB_DEAD: unnamed_10 = 1;
pub type unnamed_27 = libc::c_uint;
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
pub struct unnamed_28 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub type __pid_t = libc::c_int;
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
pub struct unnamed_29 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub type unnamed_30 = libc::c_uint;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_31 {
    offset: u_int,
    data: unnamed_35,
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
pub type speed_t = libc::c_uint;
pub const TTY_UNKNOWN: unnamed_2 = 6;
pub const TTY_VT101: unnamed_2 = 1;
pub const JOB_CLOSED: unnamed_10 = 2;
pub type tcflag_t = libc::c_uint;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_16,
    pub ev_next: unnamed_12,
    pub ev_timeout_pos: unnamed_4,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_23,
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
pub struct unnamed_34 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
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
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_36 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type layout_type = libc::c_uint;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub type uint16_t = libc::c_ushort;
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
    pub message_log: unnamed_38,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_27,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_3,
}
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
#[no_mangle]
pub static mut options_table: [options_table_entry; 130] =
    unsafe {
        [options_table_entry{name:
                                 b"buffer-limit\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_SERVER,
                             minimum: 1i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 50i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"command-alias\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_ARRAY,
                             scope: OPTIONS_TABLE_SERVER,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"split-pane=split-window,splitp=split-window,server-info=show-messages -JT,info=show-messages -JT,choose-window=choose-tree -w,choose-session=choose-tree -s\x00"
                                     as *const u8 as *const libc::c_char,
                             default_num: 0,
                             separator:
                                 b",\x00" as *const u8 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"default-terminal\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_SERVER,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"screen\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"escape-time\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_SERVER,
                             minimum: 0i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 500i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"exit-empty\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_SERVER,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"exit-unattached\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_SERVER,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"focus-events\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_SERVER,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"history-file\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_SERVER,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"\x00" as *const u8 as *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"message-limit\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_SERVER,
                             minimum: 0i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 100i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"set-clipboard\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_CHOICE,
                             scope: OPTIONS_TABLE_SERVER,
                             minimum: 0,
                             maximum: 0,
                             choices: 0 as *mut *const _, // Initialized in main
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"terminal-overrides\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_ARRAY,
                             scope: OPTIONS_TABLE_SERVER,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q,screen*:XT\x00"
                                     as *const u8 as *const libc::c_char,
                             default_num: 0,
                             separator:
                                 b",\x00" as *const u8 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"user-keys\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_ARRAY,
                             scope: OPTIONS_TABLE_SERVER,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"\x00" as *const u8 as *const libc::c_char,
                             default_num: 0,
                             separator:
                                 b",\x00" as *const u8 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"activity-action\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_CHOICE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices: 0 as *mut *const _, // Initialized in main
                             default_str: 0 as *const libc::c_char,
                             default_num: 3i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"assume-paste-time\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"base-index\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"bell-action\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_CHOICE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices: 0 as *mut *const _, // Initialized in main
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"default-command\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"\x00" as *const u8 as *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"default-shell\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"/bin/sh\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"destroy-unattached\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"detach-on-destroy\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"display-panes-active-colour\x00" as
                                     *const u8 as *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"display-panes-colour\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 4i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"display-panes-time\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 1i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 1000i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"display-time\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 750i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"history-limit\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 2000i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"key-table\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"root\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"lock-after-time\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"lock-command\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"lock -np\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"message-attr\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_ATTRIBUTES,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"message-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"message-bg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 3i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"message-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"message-command-attr\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_ATTRIBUTES,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"message-command-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"message-command-bg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"message-command-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"message-command-fg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 3i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"message-command-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"message-command-style\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"bg=black,fg=yellow\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"message-fg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"message-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"message-style\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"bg=yellow,fg=black\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"mouse\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"prefix\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_KEY,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 2 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"prefix2\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_KEY,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num:
                                 281470681743360u64 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"renumber-windows\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"repeat-time\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0i32 as u_int,
                             maximum: 32767i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 500i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"set-titles\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"set-titles-string\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"#S:#I:#W - \"#T\" #{session_alerts}\x00" as
                                     *const u8 as *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"silence-action\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_CHOICE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices: 0 as *mut *const _, // Initialized in main
                             default_str: 0 as *const libc::c_char,
                             default_num: 3i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"status\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"status-attr\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_ATTRIBUTES,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"status-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"status-bg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 2i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"status-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"status-fg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"status-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"status-interval\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 15i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"status-justify\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_CHOICE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices: 0 as *mut *const _, // Initialized in main
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"status-keys\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_CHOICE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices: 0 as *mut *const _, // Initialized in main
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"status-left\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"[#S] \x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"status-left-attr\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_ATTRIBUTES,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"status-left-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"status-left-bg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"status-left-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"status-left-fg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"status-left-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"status-left-length\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0i32 as u_int,
                             maximum: 32767i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 10i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"status-left-style\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"default\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"status-position\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_CHOICE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices: 0 as *mut *const _, // Initialized in main
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"status-right\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b" \"#{=21:pane_title}\" %H:%M %d-%b-%y\x00"
                                     as *const u8 as *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"status-right-attr\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_ATTRIBUTES,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"status-right-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"status-right-bg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"status-right-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"status-right-fg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"status-right-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"status-right-length\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0i32 as u_int,
                             maximum: 32767i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 40i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"status-right-style\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"default\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"status-style\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"bg=green,fg=black\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"update-environment\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_ARRAY,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"DISPLAY KRB5CCNAME SSH_ASKPASS SSH_AUTH_SOCK SSH_AGENT_PID SSH_CONNECTION WINDOWID XAUTHORITY\x00"
                                     as *const u8 as *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"visual-activity\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_CHOICE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices: 0 as *mut *const _, // Initialized in main
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"visual-bell\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_CHOICE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices: 0 as *mut *const _, // Initialized in main
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"visual-silence\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_CHOICE,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices: 0 as *mut *const _, // Initialized in main
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"word-separators\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_SESSION,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b" -_@\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"aggressive-resize\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"allow-rename\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"alternate-screen\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"automatic-rename\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"automatic-rename-format\x00" as *const u8
                                     as *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}\x00"
                                     as *const u8 as *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"clock-mode-colour\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 4i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"clock-mode-style\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_CHOICE,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices: 0 as *mut *const _, // Initialized in main
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"force-height\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"force-width\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"main-pane-height\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 1i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 24i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"main-pane-width\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 1i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 80i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"mode-attr\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_ATTRIBUTES,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"mode-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"mode-bg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 3i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"mode-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"mode-fg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"mode-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"mode-keys\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_CHOICE,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices: 0 as *mut *const _, // Initialized in main
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"mode-style\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"bg=yellow,fg=black\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"monitor-activity\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"monitor-bell\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"monitor-silence\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"other-pane-height\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"other-pane-width\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0i32 as u_int,
                             maximum: 2147483647i32 as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"pane-active-border-bg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"pane-active-border-style\x00" as *const u8
                                     as *const libc::c_char,},
         options_table_entry{name:
                                 b"pane-active-border-fg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 2i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"pane-active-border-style\x00" as *const u8
                                     as *const libc::c_char,},
         options_table_entry{name:
                                 b"pane-active-border-style\x00" as *const u8
                                     as *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"fg=green\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"pane-base-index\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_NUMBER,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0i32 as u_int,
                             maximum: (32767i32 * 2i32 + 1i32) as u_int,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"pane-border-bg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"pane-border-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"pane-border-fg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"pane-border-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"pane-border-format\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"#{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"\x00"
                                     as *const u8 as *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"pane-border-status\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_CHOICE,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices: 0 as *mut *const _, // Initialized in main
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"pane-border-style\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"default\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"remain-on-exit\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"synchronize-panes\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-active-style\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"default\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-style\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"default\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-activity-attr\x00" as
                                     *const u8 as *const libc::c_char,
                             type_0: OPTIONS_TABLE_ATTRIBUTES,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 16i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-activity-style\x00" as
                                     *const u8 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-activity-bg\x00" as *const u8
                                     as *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-activity-style\x00" as
                                     *const u8 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-activity-fg\x00" as *const u8
                                     as *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-activity-style\x00" as
                                     *const u8 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-activity-style\x00" as
                                     *const u8 as *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"reverse\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-attr\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_ATTRIBUTES,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-bell-attr\x00" as *const u8
                                     as *const libc::c_char,
                             type_0: OPTIONS_TABLE_ATTRIBUTES,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 16i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-bell-style\x00" as *const u8
                                     as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-bell-bg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-bell-style\x00" as *const u8
                                     as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-bell-fg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-bell-style\x00" as *const u8
                                     as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-bell-style\x00" as *const u8
                                     as *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"reverse\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-bg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-current-attr\x00" as
                                     *const u8 as *const libc::c_char,
                             type_0: OPTIONS_TABLE_ATTRIBUTES,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-current-style\x00" as
                                     *const u8 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-current-bg\x00" as *const u8
                                     as *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-current-style\x00" as
                                     *const u8 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-current-fg\x00" as *const u8
                                     as *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-current-style\x00" as
                                     *const u8 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-current-format\x00" as
                                     *const u8 as *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"#I:#W#{?window_flags,#{window_flags}, }\x00"
                                     as *const u8 as *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-current-style\x00" as
                                     *const u8 as *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"default\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-fg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-style\x00" as *const u8 as
                                     *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-format\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"#I:#W#{?window_flags,#{window_flags}, }\x00"
                                     as *const u8 as *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-last-attr\x00" as *const u8
                                     as *const libc::c_char,
                             type_0: OPTIONS_TABLE_ATTRIBUTES,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-last-style\x00" as *const u8
                                     as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-last-bg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-last-style\x00" as *const u8
                                     as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-last-fg\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_COLOUR,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 8i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style:
                                 b"window-status-last-style\x00" as *const u8
                                     as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-last-style\x00" as *const u8
                                     as *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"default\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-separator\x00" as *const u8
                                     as *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b" \x00" as *const u8 as *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"window-status-style\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_STYLE,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str:
                                 b"default\x00" as *const u8 as
                                     *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"wrap-search\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name:
                                 b"xterm-keys\x00" as *const u8 as
                                     *const libc::c_char,
                             type_0: OPTIONS_TABLE_FLAG,
                             scope: OPTIONS_TABLE_WINDOW,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 1i32 as libc::c_longlong,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,},
         options_table_entry{name: 0 as *const libc::c_char,
                             type_0: OPTIONS_TABLE_STRING,
                             scope: OPTIONS_TABLE_NONE,
                             minimum: 0,
                             maximum: 0,
                             choices:
                                 0 as *const *const libc::c_char as
                                     *mut *const libc::c_char,
                             default_str: 0 as *const libc::c_char,
                             default_num: 0,
                             separator: 0 as *const libc::c_char,
                             style: 0 as *const libc::c_char,}]
    };
pub static mut options_table_pane_status_list: [*const libc::c_char; 4] =
    unsafe {
        [b"off\x00" as *const u8 as *const libc::c_char,
         b"top\x00" as *const u8 as *const libc::c_char,
         b"bottom\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char]
    };
pub static mut options_table_mode_keys_list: [*const libc::c_char; 3] =
    unsafe {
        [b"emacs\x00" as *const u8 as *const libc::c_char,
         b"vi\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char]
    };
pub static mut options_table_clock_mode_style_list: [*const libc::c_char; 3] =
    unsafe {
        [b"12\x00" as *const u8 as *const libc::c_char,
         b"24\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char]
    };
pub static mut options_table_visual_bell_list: [*const libc::c_char; 4] =
    unsafe {
        [b"off\x00" as *const u8 as *const libc::c_char,
         b"on\x00" as *const u8 as *const libc::c_char,
         b"both\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char]
    };
pub static mut options_table_status_position_list: [*const libc::c_char; 3] =
    unsafe {
        [b"top\x00" as *const u8 as *const libc::c_char,
         b"bottom\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char]
    };
pub static mut options_table_status_keys_list: [*const libc::c_char; 3] =
    unsafe {
        [b"emacs\x00" as *const u8 as *const libc::c_char,
         b"vi\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char]
    };
pub static mut options_table_status_justify_list: [*const libc::c_char; 4] =
    unsafe {
        [b"left\x00" as *const u8 as *const libc::c_char,
         b"centre\x00" as *const u8 as *const libc::c_char,
         b"right\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char]
    };
pub static mut options_table_bell_action_list: [*const libc::c_char; 5] =
    unsafe {
        [b"none\x00" as *const u8 as *const libc::c_char,
         b"any\x00" as *const u8 as *const libc::c_char,
         b"current\x00" as *const u8 as *const libc::c_char,
         b"other\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char]
    };
pub static mut options_table_set_clipboard_list: [*const libc::c_char; 4] =
    unsafe {
        [b"off\x00" as *const u8 as *const libc::c_char,
         b"external\x00" as *const u8 as *const libc::c_char,
         b"on\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char]
    };

