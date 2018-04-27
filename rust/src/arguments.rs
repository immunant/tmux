extern crate libc;
extern "C" {
    pub type bufferevent_ops;
    pub type options;
    pub type event_base;
    pub type evbuffer;
    pub type _IO_FILE_plus;
    pub type tmuxproc;
    pub type environ;
    pub type format_job_tree;
    pub type input_ctx;
    pub type format_tree;
    pub type hooks;
    pub type tty_code;
    pub type tmuxpeer;
    pub type screen_titles;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
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
    fn BSDgetopt(_: libc::c_int, _: *const *mut libc::c_char,
                 _: *const libc::c_char) -> libc::c_int;
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
    fn cmd_copy_argv(_: libc::c_int, _: *mut *mut libc::c_char)
     -> *mut *mut libc::c_char;
    #[no_mangle]
    fn cmd_free_argv(_: libc::c_int, _: *mut *mut libc::c_char) -> ();
    #[no_mangle]
    fn args_print_add(buf: *mut *mut libc::c_char, len: *mut size_t,
                      fmt: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn utf8_stravis(_: *mut *mut libc::c_char, _: *const libc::c_char,
                    _: libc::c_int) -> libc::c_int;
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
pub const TTY_VT320: unnamed_28 = 4;
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
    pub entry: unnamed_33,
    pub tree_entry: unnamed_19,
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
    pub alerts_entry: unnamed_16,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_1,
    pub entry: unnamed_15,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
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
    pub entry: unnamed_6,
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
pub struct unnamed_0 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
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
pub struct unnamed_3 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type pid_t = __pid_t;
pub type layout_type = libc::c_uint;
pub const TTY_VT420: unnamed_28 = 5;
pub type unnamed_4 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
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
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_36,
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
    pub entry: unnamed_13,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub ev_signal_next: unnamed_39,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_23,
}
pub const JOB_CLOSED: unnamed_36 = 2;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const LINE_SEL_NONE: unnamed_4 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub ev_io_next: unnamed_25,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
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
    pub entry: unnamed_14,
}
pub type key_code = libc::c_ulonglong;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
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
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_12 {
    offset: u_int,
    data: unnamed_20,
}
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_29,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type tcflag_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
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
pub struct session_groups {
    pub rbh_root: *mut session_group,
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
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const TTY_UNKNOWN: unnamed_28 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type __suseconds_t = libc::c_long;
pub type cc_t = libc::c_uchar;
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
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub type uint16_t = libc::c_ushort;
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
pub type speed_t = libc::c_uint;
pub const LINE_SEL_RIGHT_LEFT: unnamed_4 = 2;
pub type u_char = __u_char;
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
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const PROMPT_ENTRY: unnamed_32 = 0;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
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
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub type options_table_scope = libc::c_uint;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub type time_t = __time_t;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_4 = 1;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_9,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const TTY_VT220: unnamed_28 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const TTY_VT102: unnamed_28 = 2;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
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
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type options_table_type = libc::c_uint;
pub type __off_t = libc::c_long;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_24 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_10,
}
pub type uint32_t = libc::c_uint;
pub type uint8_t = libc::c_uchar;
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
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub rbe_left: *mut args_entry,
    pub rbe_right: *mut args_entry,
    pub rbe_parent: *mut args_entry,
    pub rbe_color: libc::c_int,
}
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub type unnamed_28 = libc::c_uint;
pub type u_int = __u_int;
pub const TTY_VT101: unnamed_28 = 1;
pub type cmd_retval = libc::c_int;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const JOB_DEAD: unnamed_36 = 1;
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
    pub entry: unnamed_2,
    pub wentry: unnamed_17,
    pub sentry: unnamed_37,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const PROMPT_COMMAND: unnamed_32 = 1;
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
    pub gentry: unnamed_26,
    pub entry: unnamed_7,
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_12,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_24,
}
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_4,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_18,
    pub ev_next: unnamed_22,
    pub ev_timeout_pos: unnamed_31,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_30,
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
pub union unnamed_30 {
    ev_io: unnamed_11,
    ev_signal: unnamed_8,
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_entry {
    pub flag: u_char,
    pub value: *mut libc::c_char,
    pub entry: unnamed_27,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_31 {
    ev_next_with_common_timeout: unnamed_34,
    min_heap_idx: libc::c_int,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type unnamed_32 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const TTY_VT100: unnamed_28 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
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
pub type unnamed_36 = libc::c_uint;
pub type u_short = __u_short;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const CMDQ_CALLBACK: cmdq_type = 1;
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
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
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
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_5,
    pub entry: unnamed_3,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
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
    pub message_log: unnamed_35,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_32,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_38,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const JOB_RUNNING: unnamed_36 = 0;
#[no_mangle]
pub unsafe extern "C" fn args_set(mut args: *mut args, mut ch: u_char,
                                  mut value: *const libc::c_char) -> () {
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    entry = args_find(args, ch);
    if entry != 0 as *mut libc::c_void as *mut args_entry {
        free((*entry).value as *mut libc::c_void);
        (*entry).value = 0 as *mut libc::c_char
    } else {
        entry =
            xcalloc(1i32 as size_t,
                    ::std::mem::size_of::<args_entry>() as libc::c_ulong) as
                *mut args_entry;
        (*entry).flag = ch;
        args_tree_RB_INSERT(&mut (*args).tree as *mut args_tree, entry);
    }
    if value != 0 as *mut libc::c_void as *const libc::c_char {
        (*entry).value = xstrdup(value)
    };
}
unsafe extern "C" fn args_tree_RB_INSERT(mut head: *mut args_tree,
                                         mut elm: *mut args_entry)
 -> *mut args_entry {
    let mut tmp: *mut args_entry = 0 as *mut args_entry;
    let mut parent: *mut args_entry = 0 as *mut args_entry;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = args_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    loop  {
        (*elm).entry.rbe_parent = parent;
        (*elm).entry.rbe_right = 0 as *mut args_entry;
        (*elm).entry.rbe_left = (*elm).entry.rbe_right;
        (*elm).entry.rbe_color = 1i32;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut args_entry {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else { (*parent).entry.rbe_right = elm }
        while 0 != 0i32 { }
    } else { (*head).rbh_root = elm }
    args_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut args_entry;
}
unsafe extern "C" fn args_tree_RB_INSERT_COLOR(mut head: *mut args_tree,
                                               mut elm: *mut args_entry)
 -> () {
    let mut current_block: u64;
    let mut parent: *mut args_entry = 0 as *mut args_entry;
    let mut gparent: *mut args_entry = 0 as *mut args_entry;
    let mut tmp: *mut args_entry = 0 as *mut args_entry;
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
                                5948590327928692120 => {
                                    if 0 != 0i32 {
                                        break ;
                                    } else { break 's_95 ; }
                                }
                                _ => {
                                    if 0 != 0i32 {
                                        current_block = 6669252993407410313;
                                    } else {
                                        current_block = 5948590327928692120;
                                    }
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
                            4567019141635105728 => {
                                (*parent).entry.rbe_color = 0i32;
                                (*gparent).entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4567019141635105728;
                                } else { break ; }
                            }
                            _ => {
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
                                11793792312832361944 => {
                                    if 0 != 0i32 {
                                        current_block = 11793792312832361944;
                                    } else {
                                        current_block = 2543120759711851213;
                                    }
                                }
                                _ => {
                                    if 0 != 0i32 {
                                        break ;
                                    } else { break 's_219 ; }
                                }
                            }
                        }
                    }
            }
        }
    }
    (*(*head).rbh_root).entry.rbe_color = 0i32;
}
unsafe extern "C" fn args_cmp(mut a1: *mut args_entry,
                              mut a2: *mut args_entry) -> libc::c_int {
    return (*a1).flag as libc::c_int - (*a2).flag as libc::c_int;
}
unsafe extern "C" fn args_find(mut args: *mut args, mut ch: u_char)
 -> *mut args_entry {
    let mut entry: args_entry =
        args_entry{flag: 0,
                   value: 0 as *mut libc::c_char,
                   entry:
                       unnamed_27{rbe_left: 0 as *mut args_entry,
                                  rbe_right: 0 as *mut args_entry,
                                  rbe_parent: 0 as *mut args_entry,
                                  rbe_color: 0,},};
    entry.flag = ch;
    return args_tree_RB_FIND(&mut (*args).tree as *mut args_tree,
                             &mut entry as *mut args_entry);
}
unsafe extern "C" fn args_tree_RB_FIND(mut head: *mut args_tree,
                                       mut elm: *mut args_entry)
 -> *mut args_entry {
    let mut tmp: *mut args_entry = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = args_cmp(elm, tmp);
            if comp < 0i32 {
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return 0 as *mut args_entry }
    };
}
#[no_mangle]
pub unsafe extern "C" fn args_parse(mut template: *const libc::c_char,
                                    mut argc: libc::c_int,
                                    mut argv: *mut *mut libc::c_char)
 -> *mut args {
    let mut args: *mut args = 0 as *mut args;
    let mut opt: libc::c_int = 0;
    args =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<args>() as libc::c_ulong) as *mut args;
    BSDoptreset = 1i32;
    BSDoptind = 1i32;
    loop  {
        opt = BSDgetopt(argc, argv, template);
        if opt != 1i32.wrapping_neg() {
            if opt < 0i32 { continue ; }
            if opt == 63 ||
                   strchr(template, opt) ==
                       0 as *mut libc::c_void as *mut libc::c_char {
                args_free(args);
                return 0 as *mut args
            } else { args_set(args, opt as u_char, BSDoptarg); }
        } else {
            argc -= BSDoptind;
            argv = argv.offset(BSDoptind as isize);
            (*args).argc = argc;
            (*args).argv = cmd_copy_argv(argc, argv);
            return args
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn args_free(mut args: *mut args) -> () {
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    let mut entry1: *mut args_entry = 0 as *mut args_entry;
    cmd_free_argv((*args).argc, (*args).argv);
    entry =
        args_tree_RB_MINMAX(&mut (*args).tree as *mut args_tree,
                            1i32.wrapping_neg());
    while entry != 0 as *mut libc::c_void as *mut args_entry &&
              { entry1 = args_tree_RB_NEXT(entry); 0 != 1i32 } {
        args_tree_RB_REMOVE(&mut (*args).tree as *mut args_tree, entry);
        free((*entry).value as *mut libc::c_void);
        free(entry as *mut libc::c_void);
        entry = entry1
    }
    free(args as *mut libc::c_void);
}
unsafe extern "C" fn args_tree_RB_REMOVE(mut head: *mut args_tree,
                                         mut elm: *mut args_entry)
 -> *mut args_entry {
    let mut current_block: u64;
    let mut child: *mut args_entry = 0 as *mut args_entry;
    let mut parent: *mut args_entry = 0 as *mut args_entry;
    let mut old: *mut args_entry = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left == 0 as *mut libc::c_void as *mut args_entry {
        child = (*elm).entry.rbe_right;
        current_block = 9386390421034826751;
    } else if (*elm).entry.rbe_right ==
                  0 as *mut libc::c_void as *mut args_entry {
        child = (*elm).entry.rbe_left;
        current_block = 9386390421034826751;
    } else {
        let mut left: *mut args_entry = 0 as *mut args_entry;
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
            current_block = 13950620942190400307;
        } else { current_block = 13950620942190400307; }
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
    if color == 0i32 { args_tree_RB_REMOVE_COLOR(head, parent, child); }
    return old;
}
unsafe extern "C" fn args_tree_RB_REMOVE_COLOR(mut head: *mut args_tree,
                                               mut parent: *mut args_entry,
                                               mut elm: *mut args_entry)
 -> () {
    let mut current_block: u64;
    let mut tmp: *mut args_entry = 0 as *mut args_entry;
    's_4:
        loop  {
            if !((elm == 0 as *mut libc::c_void as *mut args_entry ||
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
                        17179679302217393232 => {
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
                        _ => {
                            if ((*tmp).entry.rbe_left ==
                                    0 as *mut libc::c_void as *mut args_entry
                                    ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut args_entry ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_right ==
                                          0 as *mut libc::c_void as
                                              *mut args_entry ||
                                          (*(*tmp).entry.rbe_right).entry.rbe_color
                                              == 0i32 {
                                current_block = 15976848397966268834;
                                break 's_4 ;
                            } else {
                                current_block = 7149356873433890176;
                                break 's_4 ;
                            }
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
                                            17784502470059252271 => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_210 ; }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        16738040538446813684;
                                                } else {
                                                    current_block =
                                                        17784502470059252271;
                                                }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).entry.rbe_left;
                            current_block = 7746103178988627676;
                        }
                        _ => {
                            if ((*tmp).entry.rbe_left ==
                                    0 as *mut libc::c_void as *mut args_entry
                                    ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut args_entry ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_left ==
                                          0 as *mut libc::c_void as
                                              *mut args_entry ||
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
            let mut oleft: *mut args_entry = 0 as *mut args_entry;
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
                            2232869372362427478 => {
                                if 0 != 0i32 {
                                    current_block = 2232869372362427478;
                                } else {
                                    current_block = 15904375183555213903;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_96 ; }
                            }
                        }
                    }
                }
            tmp = (*parent).entry.rbe_right;
            current_block = 7149356873433890176;
        }
        13826291924415791078 => {
            let mut oright: *mut args_entry = 0 as *mut args_entry;
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
unsafe extern "C" fn args_tree_RB_NEXT(mut elm: *mut args_entry)
 -> *mut args_entry {
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
unsafe extern "C" fn args_tree_RB_MINMAX(mut head: *mut args_tree,
                                         mut val: libc::c_int)
 -> *mut args_entry {
    let mut tmp: *mut args_entry = (*head).rbh_root;
    let mut parent: *mut args_entry = 0 as *mut args_entry;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else { tmp = (*tmp).entry.rbe_right }
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn args_print(mut args: *mut args)
 -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut escaped: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    static mut quoted: [libc::c_char; 7] =
        unsafe { [32, 35, 34, 39, 59, 36, 0] };
    len = 1i32 as size_t;
    buf = xcalloc(1i32 as size_t, len) as *mut libc::c_char;
    entry =
        args_tree_RB_MINMAX(&mut (*args).tree as *mut args_tree,
                            1i32.wrapping_neg());
    while entry != 0 as *mut libc::c_void as *mut args_entry {
        if !((*entry).value != 0 as *mut libc::c_void as *mut libc::c_char) {
            if *buf as libc::c_int == 0 {
                args_print_add(&mut buf as *mut *mut libc::c_char,
                               &mut len as *mut size_t,
                               b"-\x00" as *const u8 as *const libc::c_char);
            }
            args_print_add(&mut buf as *mut *mut libc::c_char,
                           &mut len as *mut size_t,
                           b"%c\x00" as *const u8 as *const libc::c_char,
                           (*entry).flag as libc::c_int);
        }
        entry = args_tree_RB_NEXT(entry)
    }
    entry =
        args_tree_RB_MINMAX(&mut (*args).tree as *mut args_tree,
                            1i32.wrapping_neg());
    while entry != 0 as *mut libc::c_void as *mut args_entry {
        if !((*entry).value == 0 as *mut libc::c_void as *mut libc::c_char) {
            if *buf as libc::c_int != 0 {
                args_print_add(&mut buf as *mut *mut libc::c_char,
                               &mut len as *mut size_t,
                               b" -%c \x00" as *const u8 as
                                   *const libc::c_char,
                               (*entry).flag as libc::c_int);
            } else {
                args_print_add(&mut buf as *mut *mut libc::c_char,
                               &mut len as *mut size_t,
                               b"-%c \x00" as *const u8 as
                                   *const libc::c_char,
                               (*entry).flag as libc::c_int);
            }
            flags = 1i32 | 8i32 | 16i32;
            if *(*entry).value.offset(strcspn((*entry).value, quoted.as_ptr())
                                          as isize) as libc::c_int != 0 {
                flags |= 512i32
            }
            utf8_stravis(&mut escaped as *mut *mut libc::c_char,
                         (*entry).value, flags);
            if 0 != flags & 512i32 {
                args_print_add(&mut buf as *mut *mut libc::c_char,
                               &mut len as *mut size_t,
                               b"\"%s\"\x00" as *const u8 as
                                   *const libc::c_char, escaped);
            } else {
                args_print_add(&mut buf as *mut *mut libc::c_char,
                               &mut len as *mut size_t,
                               b"%s\x00" as *const u8 as *const libc::c_char,
                               escaped);
            }
            free(escaped as *mut libc::c_void);
        }
        entry = args_tree_RB_NEXT(entry)
    }
    i = 0i32;
    while i < (*args).argc {
        if *buf as libc::c_int != 0 {
            args_print_add(&mut buf as *mut *mut libc::c_char,
                           &mut len as *mut size_t,
                           b" \x00" as *const u8 as *const libc::c_char);
        }
        flags = 1i32 | 8i32 | 16i32;
        if *(*(*args).argv.offset(i as
                                      isize)).offset(strcspn(*(*args).argv.offset(i
                                                                                      as
                                                                                      isize),
                                                             quoted.as_ptr())
                                                         as isize) as
               libc::c_int != 0 {
            flags |= 512i32
        }
        utf8_stravis(&mut escaped as *mut *mut libc::c_char,
                     *(*args).argv.offset(i as isize), flags);
        if 0 != flags & 512i32 {
            args_print_add(&mut buf as *mut *mut libc::c_char,
                           &mut len as *mut size_t,
                           b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                           escaped);
        } else {
            args_print_add(&mut buf as *mut *mut libc::c_char,
                           &mut len as *mut size_t,
                           b"%s\x00" as *const u8 as *const libc::c_char,
                           escaped);
        }
        free(escaped as *mut libc::c_void);
        i += 1
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn args_has(mut args: *mut args, mut ch: u_char)
 -> libc::c_int {
    return (args_find(args, ch) != 0 as *mut libc::c_void as *mut args_entry)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn args_get(mut args: *mut args, mut ch: u_char)
 -> *const libc::c_char {
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    entry = args_find(args, ch);
    if entry == 0 as *mut libc::c_void as *mut args_entry {
        return 0 as *const libc::c_char
    } else { return (*entry).value };
}
#[no_mangle]
pub unsafe extern "C" fn args_strtonum(mut args: *mut args, mut ch: u_char,
                                       mut minval: libc::c_longlong,
                                       mut maxval: libc::c_longlong,
                                       mut cause: *mut *mut libc::c_char)
 -> libc::c_longlong {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut ll: libc::c_longlong = 0;
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    entry = args_find(args, ch);
    if entry == 0 as *mut libc::c_void as *mut args_entry {
        *cause = xstrdup(b"missing\x00" as *const u8 as *const libc::c_char);
        return 0i32 as libc::c_longlong
    } else {
        ll =
            strtonum((*entry).value, minval, maxval,
                     &mut errstr as *mut *const libc::c_char);
        if errstr != 0 as *mut libc::c_void as *const libc::c_char {
            *cause = xstrdup(errstr);
            return 0i32 as libc::c_longlong
        } else { *cause = 0 as *mut libc::c_char; return ll }
    };
}
unsafe extern "C" fn args_tree_RB_NFIND(mut head: *mut args_tree,
                                        mut elm: *mut args_entry)
 -> *mut args_entry {
    let mut tmp: *mut args_entry = (*head).rbh_root;
    let mut res: *mut args_entry = 0 as *mut args_entry;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = args_cmp(elm, tmp);
            if comp < 0i32 {
                res = tmp;
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return res }
    };
}
unsafe extern "C" fn args_tree_RB_PREV(mut elm: *mut args_entry)
 -> *mut args_entry {
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

