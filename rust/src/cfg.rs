extern crate libc;
extern "C" {
    pub type hooks;
    pub type args_entry;
    pub type event_base;
    pub type bufferevent_ops;
    pub type _IO_FILE_plus;
    pub type input_ctx;
    pub type tmuxpeer;
    pub type tty_code;
    pub type environ;
    pub type tmuxproc;
    pub type screen_titles;
    pub type format_tree;
    pub type options;
    pub type format_job_tree;
    pub type evbuffer;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut program_invocation_name: *mut libc::c_char;
    #[no_mangle]
    static mut program_invocation_short_name: *mut libc::c_char;
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
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    static mut _sys_nerr: libc::c_int;
    #[no_mangle]
    static _sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    static in6addr_any: in6_addr;
    #[no_mangle]
    static in6addr_loopback: in6_addr;
    #[no_mangle]
    fn fparseln(_: *mut FILE, _: *mut size_t, _: *mut size_t,
                _: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    fn find_home() -> *const libc::c_char;
    #[no_mangle]
    fn status_prompt_load_history() -> ();
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    #[no_mangle]
    fn window_copy_add(_: *mut window_pane, _: *const libc::c_char, ...)
     -> ();
    #[no_mangle]
    fn window_copy_init_for_output(_: *mut window_pane) -> ();
    #[no_mangle]
    static window_copy_mode: window_mode;
    #[no_mangle]
    fn window_pane_set_mode(_: *mut window_pane, _: *const window_mode,
                            _: *mut cmd_find_state, _: *mut args)
     -> libc::c_int;
    #[no_mangle]
    fn cmdq_get_callback1(_: *const libc::c_char, _: cmdq_cb,
                          _: *mut libc::c_void) -> *mut cmdq_item;
    #[no_mangle]
    fn cmdq_append(_: *mut client, _: *mut cmdq_item) -> ();
    #[no_mangle]
    fn cfg_add_cause(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn cmd_list_free(_: *mut cmd_list) -> ();
    #[no_mangle]
    fn cmdq_insert_after(_: *mut cmdq_item, _: *mut cmdq_item) -> ();
    #[no_mangle]
    fn cmdq_get_command(_: *mut cmd_list, _: *mut cmd_find_state,
                        _: *mut mouse_event, _: libc::c_int)
     -> *mut cmdq_item;
    #[no_mangle]
    fn cmd_string_parse(_: *const libc::c_char, _: *const libc::c_char,
                        _: u_int, _: *mut *mut libc::c_char) -> *mut cmd_list;
    #[no_mangle]
    fn format_free(_: *mut format_tree) -> ();
    #[no_mangle]
    fn format_true(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn format_expand(_: *mut format_tree, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn format_create(_: *mut client, _: *mut cmdq_item, _: libc::c_int,
                     _: libc::c_int) -> *mut format_tree;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn cmdq_print(_: *mut cmdq_item, _: *const libc::c_char, ...) -> ();
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
    static mut session_groups: session_groups;
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const _IScntrl: unnamed_40 = 2;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
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
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type __off64_t = libc::c_long;
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_28,
    pub entry: unnamed_24,
}
pub const _ISprint: unnamed_40 = 16384;
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_7,
    pub ev_next: unnamed_16,
    pub ev_timeout_pos: unnamed_23,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_39,
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
pub struct unnamed_2 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
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
    pub entry: unnamed_17,
    pub tree_entry: unnamed_31,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqe_next: *mut cfg_cond,
    pub tqe_prev: *mut *mut cfg_cond,
}
pub type u_int = __u_int;
pub type u_short = __u_short;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_6 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const PROMPT_ENTRY: unnamed_32 = 0;
pub type __pid_t = libc::c_int;
pub type cmdq_type = libc::c_uint;
pub const JOB_CLOSED: unnamed_20 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cfg_conds {
    pub tqh_first: *mut cfg_cond,
    pub tqh_last: *mut *mut cfg_cond,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_9 {
    offset: u_int,
    data: unnamed_19,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint32_t = libc::c_uint;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub type cmd_retval = libc::c_int;
pub type tcflag_t = libc::c_uint;
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
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub type unnamed_11 = libc::c_uint;
pub type bitstr_t = libc::c_uchar;
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
    pub entry: unnamed_4,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_8,
}
pub type __u_int = libc::c_uint;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const _ISblank: unnamed_40 = 1;
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
pub struct unnamed_13 {
    pub ev_io_next: unnamed_26,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub type options_table_type = libc::c_uint;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const TTY_VT102: unnamed_34 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const JOB_RUNNING: unnamed_20 = 0;
pub const _ISupper: unnamed_40 = 256;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub type layout_type = libc::c_uint;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const LINE_SEL_RIGHT_LEFT: unnamed_11 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_33,
}
pub const PROMPT_COMMAND: unnamed_32 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type pid_t = __pid_t;
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
    pub wentry: unnamed_12,
    pub sentry: unnamed_37,
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
    pub entry: unnamed_36,
}
pub const JOB_DEAD: unnamed_20 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const _ISxdigit: unnamed_40 = 4096;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_9,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type unnamed_20 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
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
pub union unnamed_23 {
    ev_next_with_common_timeout: unnamed_1,
    min_heap_idx: libc::c_int,
}
pub const _ISgraph: unnamed_40 = 32768;
pub const _ISalpha: unnamed_40 = 1024;
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
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const TTY_VT420: unnamed_34 = 5;
pub type FILE = _IO_FILE;
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub type speed_t = libc::c_uint;
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
pub type time_t = __time_t;
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub type cmd_find_type = libc::c_uint;
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const _ISpunct: unnamed_40 = 4;
pub const _ISalnum: unnamed_40 = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub const TTY_VT101: unnamed_34 = 1;
pub const LINE_SEL_NONE: unnamed_11 = 0;
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
pub struct unnamed_25 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub const TTY_VT100: unnamed_34 = 0;
pub const TTY_VT220: unnamed_34 = 3;
pub const _ISspace: unnamed_40 = 8192;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub ev_signal_next: unnamed_15,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
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
pub const _ISdigit: unnamed_40 = 2048;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
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
pub struct unnamed_30 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub type options_table_scope = libc::c_uint;
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
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_20,
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
    pub entry: unnamed_18,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cfg_cond {
    pub line: size_t,
    pub met: libc::c_int,
    pub skip: libc::c_int,
    pub saw_else: libc::c_int,
    pub entry: unnamed_5,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub type unnamed_32 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
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
    pub message_log: unnamed_14,
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
    pub entry: unnamed_25,
}
pub type unnamed_34 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_35,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const TTY_UNKNOWN: unnamed_34 = 6;
pub type __off_t = libc::c_long;
pub const LINE_SEL_LEFT_RIGHT: unnamed_11 = 1;
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
    pub lineflag: unnamed_11,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
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
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_21,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
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
    pub alerts_entry: unnamed_10,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_38,
    pub entry: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_6,
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_2,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
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
pub const TTY_VT320: unnamed_34 = 4;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
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
    pub entry: unnamed_30,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_39 {
    ev_io: unnamed_13,
    ev_signal: unnamed_27,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub type __suseconds_t = libc::c_long;
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
    pub term_type: unnamed_34,
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
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub type unnamed_40 = libc::c_uint;
pub const _ISlower: unnamed_40 = 512;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub type u_char = __u_char;
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_29,
}
#[no_mangle]
pub static mut cfg_finished: libc::c_int = unsafe { 0 };
#[no_mangle]
pub unsafe extern "C" fn start_cfg() -> () {
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    let mut quiet: libc::c_int = 0i32;
    let mut c: *mut client = 0 as *mut client;
    c = (*(&mut clients as *mut clients)).tqh_first;
    if c != 0 as *mut libc::c_void as *mut client {
        cfg_item =
            cmdq_get_callback1(b"cfg_client_done\x00" as *const u8 as
                                   *const libc::c_char, Some(cfg_client_done),
                               0 as *mut libc::c_void);
        cmdq_append(c, cfg_item);
    }
    load_cfg(b"/etc/tmux.conf\x00" as *const u8 as *const libc::c_char,
             0 as *mut client, 0 as *mut cmdq_item, 1i32);
    if cfg_file == 0 as *mut libc::c_void as *mut libc::c_char &&
           {
               home = find_home();
               home != 0 as *mut libc::c_void as *const libc::c_char
           } {
        xasprintf(&mut cfg_file as *mut *mut libc::c_char,
                  b"%s/.tmux.conf\x00" as *const u8 as *const libc::c_char,
                  home);
        quiet = 1i32
    }
    if cfg_file != 0 as *mut libc::c_void as *mut libc::c_char {
        load_cfg(cfg_file, 0 as *mut client, 0 as *mut cmdq_item, quiet);
    }
    cmdq_append(0 as *mut client,
                cmdq_get_callback1(b"cfg_done\x00" as *const u8 as
                                       *const libc::c_char, Some(cfg_done),
                                   0 as *mut libc::c_void));
}
unsafe extern "C" fn cfg_done(mut item: *mut cmdq_item,
                              mut data: *mut libc::c_void) -> cmd_retval {
    if 0 != cfg_finished {
        return CMD_RETURN_NORMAL
    } else {
        cfg_finished = 1i32;
        if !((*(&mut sessions as *mut sessions)).rbh_root ==
                 0 as *mut libc::c_void as *mut session) {
            cfg_show_causes(sessions_RB_MINMAX(&mut sessions as *mut sessions,
                                               1i32.wrapping_neg()));
        }
        if cfg_item != 0 as *mut libc::c_void as *mut cmdq_item {
            (*cfg_item).flags &= !2i32
        }
        status_prompt_load_history();
        return CMD_RETURN_NORMAL
    };
}
static mut cfg_item: *mut cmdq_item =
    unsafe { 0 as *const cmdq_item as *mut cmdq_item };
#[no_mangle]
pub unsafe extern "C" fn cfg_show_causes(mut s: *mut session) -> () {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut i: u_int = 0;
    if s == 0 as *mut libc::c_void as *mut session ||
           cfg_ncauses == 0i32 as libc::c_uint {
        return
    } else {
        wp = (*(*(*s).curw).window).active;
        window_pane_set_mode(wp, &window_copy_mode as *const window_mode,
                             0 as *mut cmd_find_state, 0 as *mut args);
        window_copy_init_for_output(wp);
        i = 0i32 as u_int;
        while i < cfg_ncauses {
            window_copy_add(wp, b"%s\x00" as *const u8 as *const libc::c_char,
                            *cfg_causes.offset(i as isize));
            free(*cfg_causes.offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        free(cfg_causes as *mut libc::c_void);
        cfg_causes = 0 as *mut *mut libc::c_char;
        cfg_ncauses = 0i32 as u_int;
        return;
    };
}
#[no_mangle]
pub static mut cfg_ncauses: u_int = unsafe { 0 };
#[no_mangle]
pub static mut cfg_causes: *mut *mut libc::c_char =
    unsafe { 0 as *const *mut libc::c_char as *mut *mut libc::c_char };
static mut cfg_file: *mut libc::c_char =
    unsafe { 0 as *const libc::c_char as *mut libc::c_char };
#[no_mangle]
pub unsafe extern "C" fn load_cfg(mut path: *const libc::c_char,
                                  mut c: *mut client,
                                  mut item: *mut cmdq_item,
                                  mut quiet: libc::c_int) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let delim: [libc::c_char; 3] =
        [92 as libc::c_char, 92 as libc::c_char, 0 as libc::c_char];
    let mut found: u_int = 0i32 as u_int;
    let mut line: size_t = 0i32 as size_t;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cause1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    let mut new_item: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut cond: *mut cfg_cond = 0 as *mut cfg_cond;
    let mut cond1: *mut cfg_cond = 0 as *mut cfg_cond;
    let mut conds: cfg_conds =
        cfg_conds{tqh_first: 0 as *mut cfg_cond,
                  tqh_last: 0 as *mut *mut cfg_cond,};
    loop  {
        let ref mut fresh0 = (*(&mut conds as *mut cfg_conds)).tqh_first;
        *fresh0 = 0 as *mut cfg_cond;
        let ref mut fresh1 = (*(&mut conds as *mut cfg_conds)).tqh_last;
        *fresh1 =
            &mut (*(&mut conds as *mut cfg_conds)).tqh_first as
                *mut *mut cfg_cond;
        if !(0 != 0i32) { break ; }
    }
    log_debug(b"loading %s\x00" as *const u8 as *const libc::c_char, path);
    f = fopen(path, b"rb\x00" as *const u8 as *const libc::c_char);
    if f == 0 as *mut libc::c_void as *mut FILE {
        if *__errno_location() == 2i32 && 0 != quiet {
            return 0i32
        } else {
            cfg_add_cause(b"%s: %s\x00" as *const u8 as *const libc::c_char,
                          path, strerror(*__errno_location()));
            return 1i32.wrapping_neg()
        }
    } else {
        loop  {
            buf =
                fparseln(f, 0 as *mut size_t, &mut line as *mut size_t,
                         delim.as_ptr(), 0i32);
            if !(buf != 0 as *mut libc::c_void as *mut libc::c_char) {
                break ;
            }
            log_debug(b"%s: %s\x00" as *const u8 as *const libc::c_char, path,
                      buf);
            p = buf;
            while 0 !=
                      *(*__ctype_b_loc()).offset(*p as u_char as libc::c_int
                                                     as isize) as libc::c_int
                          &
                          _ISspace as libc::c_int as libc::c_ushort as
                              libc::c_int {
                p = p.offset(1isize)
            }
            if *p as libc::c_int == 0 {
                free(buf as *mut libc::c_void);
            } else {
                q = p.offset(strlen(p) as isize).offset(-1isize);
                while q != p &&
                          0 !=
                              *(*__ctype_b_loc()).offset(*q as u_char as
                                                             libc::c_int as
                                                             isize) as
                                  libc::c_int &
                                  _ISspace as libc::c_int as libc::c_ushort as
                                      libc::c_int {
                    let fresh2 = q;
                    q = q.offset(-1);
                    *fresh2 = 0 as libc::c_char
                }
                if *p as libc::c_int == 37 {
                    cfg_handle_directive(p, path, line,
                                         &mut conds as *mut cfg_conds);
                } else {
                    cond = (*(&mut conds as *mut cfg_conds)).tqh_first;
                    if cond != 0 as *mut libc::c_void as *mut cfg_cond &&
                           0 == (*cond).met {
                        continue ;
                    }
                    cmdlist =
                        cmd_string_parse(p, path, line as u_int,
                                         &mut cause1 as
                                             *mut *mut libc::c_char);
                    if cmdlist == 0 as *mut libc::c_void as *mut cmd_list {
                        free(buf as *mut libc::c_void);
                        if cause1 ==
                               0 as *mut libc::c_void as *mut libc::c_char {
                            continue ;
                        }
                        cfg_add_cause(b"%s:%zu: %s\x00" as *const u8 as
                                          *const libc::c_char, path, line,
                                      cause1);
                        free(cause1 as *mut libc::c_void);
                    } else {
                        free(buf as *mut libc::c_void);
                        new_item =
                            cmdq_get_command(cmdlist,
                                             0 as *mut cmd_find_state,
                                             0 as *mut mouse_event, 0i32);
                        if item != 0 as *mut libc::c_void as *mut cmdq_item {
                            cmdq_insert_after(item, new_item);
                        } else { cmdq_append(c, new_item); }
                        cmd_list_free(cmdlist);
                        found = found.wrapping_add(1)
                    }
                }
            }
        }
        fclose(f);
        cond =
            *(*((*(&mut conds as *mut cfg_conds)).tqh_last as
                    *mut cfg_conds)).tqh_last;
        while cond != 0 as *mut libc::c_void as *mut cfg_cond &&
                  {
                      cond1 =
                          *(*((*cond).entry.tqe_prev as
                                  *mut cfg_conds)).tqh_last;
                      0 != 1i32
                  } {
            cfg_add_cause(b"%s:%zu: unterminated %%if\x00" as *const u8 as
                              *const libc::c_char, path, (*cond).line);
            loop  {
                if (*cond).entry.tqe_next !=
                       0 as *mut libc::c_void as *mut cfg_cond {
                    (*(*cond).entry.tqe_next).entry.tqe_prev =
                        (*cond).entry.tqe_prev
                } else {
                    let ref mut fresh3 =
                        (*(&mut conds as *mut cfg_conds)).tqh_last;
                    *fresh3 = (*cond).entry.tqe_prev
                }
                *(*cond).entry.tqe_prev = (*cond).entry.tqe_next;
                if !(0 != 0i32) { break ; }
            }
            free(cond as *mut libc::c_void);
            cond = cond1
        }
        return found as libc::c_int
    };
}
unsafe extern "C" fn cfg_handle_directive(mut p: *const libc::c_char,
                                          mut path: *const libc::c_char,
                                          mut line: size_t,
                                          mut conds: *mut cfg_conds) -> () {
    let mut n: libc::c_int = 0i32;
    while *p.offset(n as isize) as libc::c_int != 0 &&
              0 ==
                  *(*__ctype_b_loc()).offset(*p.offset(n as isize) as u_char
                                                 as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
          {
        n += 1
    }
    if strncmp(p, b"%if\x00" as *const u8 as *const libc::c_char,
               n as libc::c_ulong) == 0i32 {
        cfg_handle_if(path, line, conds, p.offset(n as isize));
    } else if strncmp(p, b"%elif\x00" as *const u8 as *const libc::c_char,
                      n as libc::c_ulong) == 0i32 {
        cfg_handle_elif(path, line, conds, p.offset(n as isize));
    } else if strcmp(p, b"%else\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        cfg_handle_else(path, line, conds);
    } else if strcmp(p, b"%endif\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        cfg_handle_endif(path, line, conds);
    } else {
        cfg_add_cause(b"%s:%zu: invalid directive: %s\x00" as *const u8 as
                          *const libc::c_char, path, line, p);
    };
}
unsafe extern "C" fn cfg_handle_endif(mut path: *const libc::c_char,
                                      mut line: size_t,
                                      mut conds: *mut cfg_conds) -> () {
    let mut cond: *mut cfg_cond = (*conds).tqh_first;
    if cond == 0 as *mut libc::c_void as *mut cfg_cond {
        cfg_add_cause(b"%s:%zu: unexpected %%endif\x00" as *const u8 as
                          *const libc::c_char, path, line);
        return
    } else {
        loop  {
            if (*cond).entry.tqe_next !=
                   0 as *mut libc::c_void as *mut cfg_cond {
                (*(*cond).entry.tqe_next).entry.tqe_prev =
                    (*cond).entry.tqe_prev
            } else { (*conds).tqh_last = (*cond).entry.tqe_prev }
            *(*cond).entry.tqe_prev = (*cond).entry.tqe_next;
            if !(0 != 0i32) { break ; }
        }
        free(cond as *mut libc::c_void);
        return;
    };
}
unsafe extern "C" fn cfg_handle_else(mut path: *const libc::c_char,
                                     mut line: size_t,
                                     mut conds: *mut cfg_conds) -> () {
    let mut cond: *mut cfg_cond = (*conds).tqh_first;
    if cond == 0 as *mut libc::c_void as *mut cfg_cond ||
           0 != (*cond).saw_else {
        cfg_add_cause(b"%s:%zu: unexpected %%else\x00" as *const u8 as
                          *const libc::c_char, path, line);
        return
    } else {
        (*cond).saw_else = 1i32;
        (*cond).met = (0 == (*cond).skip) as libc::c_int;
        (*cond).skip = 1i32;
        return;
    };
}
unsafe extern "C" fn cfg_handle_elif(mut path: *const libc::c_char,
                                     mut line: size_t,
                                     mut conds: *mut cfg_conds,
                                     mut p: *const libc::c_char) -> () {
    let mut cond: *mut cfg_cond = (*conds).tqh_first;
    if cond == 0 as *mut libc::c_void as *mut cfg_cond ||
           0 != (*cond).saw_else {
        cfg_add_cause(b"%s:%zu: unexpected %%elif\x00" as *const u8 as
                          *const libc::c_char, path, line);
    } else if 0 == (*cond).skip {
        (*cond).met =
            cfg_check_condition(path, line, p,
                                &mut (*cond).skip as *mut libc::c_int)
    } else { (*cond).met = 0i32 };
}
unsafe extern "C" fn cfg_check_condition(mut path: *const libc::c_char,
                                         mut line: size_t,
                                         mut p: *const libc::c_char,
                                         mut skip: *mut libc::c_int)
 -> libc::c_int {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: libc::c_int = 0;
    while 0 !=
              *(*__ctype_b_loc()).offset(*p as u_char as libc::c_int as isize)
                  as libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int {
        p = p.offset(1isize)
    }
    if *p.offset(0isize) as libc::c_int == 0 {
        cfg_add_cause(b"%s:%zu: invalid condition\x00" as *const u8 as
                          *const libc::c_char, path, line);
        *skip = 1i32;
        return 0i32
    } else {
        ft = format_create(0 as *mut client, 0 as *mut cmdq_item, 0i32, 4i32);
        s = format_expand(ft, p);
        result = format_true(s);
        free(s as *mut libc::c_void);
        format_free(ft);
        *skip = result;
        return result
    };
}
unsafe extern "C" fn cfg_handle_if(mut path: *const libc::c_char,
                                   mut line: size_t,
                                   mut conds: *mut cfg_conds,
                                   mut p: *const libc::c_char) -> () {
    let mut cond: *mut cfg_cond = 0 as *mut cfg_cond;
    let mut parent: *mut cfg_cond = (*conds).tqh_first;
    cond =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<cfg_cond>() as libc::c_ulong) as
            *mut cfg_cond;
    (*cond).line = line;
    if parent == 0 as *mut libc::c_void as *mut cfg_cond || 0 != (*parent).met
       {
        (*cond).met =
            cfg_check_condition(path, line, p,
                                &mut (*cond).skip as *mut libc::c_int)
    } else { (*cond).skip = 1i32 }
    (*cond).saw_else = 0i32;
    loop  {
        (*cond).entry.tqe_next = (*conds).tqh_first;
        if (*cond).entry.tqe_next != 0 as *mut libc::c_void as *mut cfg_cond {
            (*(*conds).tqh_first).entry.tqe_prev =
                &mut (*cond).entry.tqe_next as *mut *mut cfg_cond
        } else {
            (*conds).tqh_last =
                &mut (*cond).entry.tqe_next as *mut *mut cfg_cond
        }
        (*conds).tqh_first = cond;
        (*cond).entry.tqe_prev =
            &mut (*conds).tqh_first as *mut *mut cfg_cond;
        if !(0 != 0i32) { break ; }
    };
}
unsafe extern "C" fn cfg_client_done(mut item: *mut cmdq_item,
                                     mut data: *mut libc::c_void)
 -> cmd_retval {
    if 0 == cfg_finished {
        return CMD_RETURN_WAIT
    } else { return CMD_RETURN_NORMAL };
}
#[no_mangle]
pub unsafe extern "C" fn set_cfg_file(mut path: *const libc::c_char) -> () {
    free(cfg_file as *mut libc::c_void);
    cfg_file = xstrdup(path);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_print_causes(mut item: *mut cmdq_item) -> () {
    let mut i: u_int = 0;
    i = 0i32 as u_int;
    while i < cfg_ncauses {
        cmdq_print(item, b"%s\x00" as *const u8 as *const libc::c_char,
                   *cfg_causes.offset(i as isize));
        free(*cfg_causes.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free(cfg_causes as *mut libc::c_void);
    cfg_causes = 0 as *mut *mut libc::c_char;
    cfg_ncauses = 0i32 as u_int;
}

