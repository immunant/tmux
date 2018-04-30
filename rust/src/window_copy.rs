extern crate libc;
extern "C" {
    pub type options;
    pub type _IO_FILE_plus;
    pub type bufferevent_ops;
    pub type screen_write_collect_item;
    pub type environ;
    pub type hooks;
    pub type tmuxproc;
    pub type screen_write_collect_line;
    pub type format_tree;
    pub type tmuxpeer;
    pub type args_entry;
    pub type input_ctx;
    pub type screen_titles;
    pub type format_job_tree;
    pub type paste_buffer;
    pub type tty_code;
    pub type evbuffer;
    pub type event_base;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn bufferevent_write(bufev: *mut bufferevent, data: *const libc::c_void,
                         size: size_t) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_enable(bufev: *mut bufferevent, event: libc::c_short)
     -> libc::c_int;
    #[no_mangle]
    fn bufferevent_disable(bufev: *mut bufferevent, event: libc::c_short)
     -> libc::c_int;
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
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, ...)
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
    fn paste_buffer_data(_: *mut paste_buffer, _: *mut size_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn paste_get_top(_: *mut *const libc::c_char) -> *mut paste_buffer;
    #[no_mangle]
    fn paste_get_name(_: *const libc::c_char) -> *mut paste_buffer;
    #[no_mangle]
    fn paste_set(_: *mut libc::c_char, _: size_t, _: *const libc::c_char,
                 _: *mut *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn format_add(_: *mut format_tree, _: *const libc::c_char,
                  _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn format_single(_: *mut cmdq_item, _: *const libc::c_char,
                     _: *mut client, _: *mut session, _: *mut winlink,
                     _: *mut window_pane) -> *mut libc::c_char;
    #[no_mangle]
    fn notify_pane(_: *const libc::c_char, _: *mut window_pane) -> ();
    #[no_mangle]
    fn options_get_string(_: *mut options, _: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn job_run(_: *const libc::c_char, _: *mut session,
               _: *const libc::c_char, _: job_update_cb, _: job_complete_cb,
               _: job_free_cb, _: *mut libc::c_void, _: libc::c_int)
     -> *mut job;
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn tty_acs_get(_: *mut tty, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_mouse_at(_: *mut window_pane, _: *mut mouse_event, _: *mut u_int,
                    _: *mut u_int, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cmd_mouse_pane(_: *mut mouse_event, _: *mut *mut session,
                      _: *mut *mut winlink) -> *mut window_pane;
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
    fn grid_get_cell(_: *mut grid, _: u_int, _: u_int, _: *mut grid_cell)
     -> ();
    #[no_mangle]
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut window_pane,
                          _: *mut screen) -> ();
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx) -> ();
    #[no_mangle]
    fn screen_write_strlen(_: *const libc::c_char, ...) -> size_t;
    #[no_mangle]
    fn screen_write_puts(_: *mut screen_write_ctx, _: *const grid_cell,
                         _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn screen_write_nputs(_: *mut screen_write_ctx, _: ssize_t,
                          _: *const grid_cell, _: *const libc::c_char, ...)
     -> ();
    #[no_mangle]
    fn screen_write_vnputs(_: *mut screen_write_ctx, _: ssize_t,
                           _: *const grid_cell, _: *const libc::c_char,
                           _: *mut __va_list_tag) -> ();
    #[no_mangle]
    fn screen_write_putc(_: *mut screen_write_ctx, _: *const grid_cell,
                         _: u_char) -> ();
    #[no_mangle]
    fn screen_write_copy(_: *mut screen_write_ctx, _: *mut screen, _: u_int,
                         _: u_int, _: u_int, _: u_int, _: *mut bitstr_t,
                         _: *const grid_cell) -> ();
    #[no_mangle]
    fn screen_write_insertline(_: *mut screen_write_ctx, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn screen_write_deleteline(_: *mut screen_write_ctx, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn screen_write_linefeed(_: *mut screen_write_ctx, _: libc::c_int,
                             _: u_int) -> ();
    #[no_mangle]
    fn screen_write_carriagereturn(_: *mut screen_write_ctx) -> ();
    #[no_mangle]
    fn screen_write_setselection(_: *mut screen_write_ctx, _: *mut u_char,
                                 _: u_int) -> ();
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn screen_free(_: *mut screen) -> ();
    #[no_mangle]
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int)
     -> ();
    #[no_mangle]
    fn screen_set_selection(_: *mut screen, _: u_int, _: u_int, _: u_int,
                            _: u_int, _: u_int, _: *mut grid_cell) -> ();
    #[no_mangle]
    fn screen_clear_selection(_: *mut screen) -> ();
    #[no_mangle]
    fn screen_hide_selection(_: *mut screen) -> ();
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn window_pane_reset_mode(_: *mut window_pane) -> ();
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
    fn style_apply(_: *mut grid_cell, _: *mut options, _: *const libc::c_char)
     -> ();
    #[no_mangle]
    fn utf8_copy(_: *mut utf8_data, _: *const utf8_data) -> ();
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    static mut session_groups: session_groups;
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
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
pub struct unnamed {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_1,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const JOB_CLOSED: unnamed_11 = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    ev_next_with_common_timeout: unnamed_15,
    min_heap_idx: libc::c_int,
}
pub const TTY_UNKNOWN: unnamed_25 = 6;
pub const WINDOW_COPY_JUMPTOBACKWARD: unnamed_6 = 6;
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
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
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
pub const WINDOW_COPY_REL_POS_ABOVE: unnamed_38 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const JOB_RUNNING: unnamed_11 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const CURSORDRAG_ENDSEL: unnamed_19 = 1;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub type key_code = libc::c_ulonglong;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const WINDOW_COPY_JUMPTOFORWARD: unnamed_6 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const LINE_SEL_LEFT_RIGHT: unnamed_17 = 1;
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
pub type unnamed_6 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_13,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const JOB_DEAD: unnamed_11 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub const WINDOW_COPY_JUMPFORWARD: unnamed_6 = 3;
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_31,
    pub ev_next: unnamed_24,
    pub ev_timeout_pos: unnamed_0,
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
pub struct unnamed_8 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __u_char = libc::c_uchar;
pub const LINE_SEL_NONE: unnamed_17 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_write_ctx {
    pub wp: *mut window_pane,
    pub s: *mut screen,
    pub item: *mut screen_write_collect_item,
    pub list: *mut screen_write_collect_line,
    pub scrolled: u_int,
    pub bg: u_int,
    pub cells: u_int,
    pub written: u_int,
    pub skipped: u_int,
}
pub type unnamed_10 = libc::c_uint;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
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
pub type unnamed_11 = libc::c_uint;
pub type ssize_t = __ssize_t;
pub const TTY_VT220: unnamed_25 = 3;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub ev_io_next: unnamed_5,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_37,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_26,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const WINDOW_COPY_REL_POS_ON_SCREEN: unnamed_38 = 1;
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
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed,
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
    pub entry: unnamed_7,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_17 = 2;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_32,
    pub entry: unnamed_27,
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
    pub entry: unnamed_28,
    pub wentry: unnamed_23,
    pub sentry: unnamed_16,
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
    pub term_type: unnamed_25,
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub type __off_t = libc::c_long;
pub type unnamed_17 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const PROMPT_ENTRY: unnamed_10 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
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
    pub entry: unnamed_40,
    pub tree_entry: unnamed_14,
}
pub type _IO_lock_t = ();
pub type unnamed_19 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
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
pub type u_int = __u_int;
pub const WINDOW_COPY_SEARCHUP: unnamed_6 = 1;
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
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub type uint16_t = libc::c_ushort;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub type tcflag_t = libc::c_uint;
pub type options_table_scope = libc::c_uint;
pub const TTY_VT420: unnamed_25 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const WINDOW_COPY_JUMPBACKWARD: unnamed_6 = 4;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
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
    pub gentry: unnamed_4,
    pub entry: unnamed_20,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type unnamed_25 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type cc_t = libc::c_uchar;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
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
pub const CURSORDRAG_SEL: unnamed_19 = 2;
pub const CURSORDRAG_NONE: unnamed_19 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_34,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_8,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_11,
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
    pub entry: unnamed_29,
}
pub const WINDOW_COPY_SEARCHDOWN: unnamed_6 = 2;
pub const TTY_VT102: unnamed_25 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub type options_table_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_30 {
    ev_io: unnamed_12,
    ev_signal: unnamed_41,
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
    pub unnamed: unnamed_33,
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
pub struct unnamed_31 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_33 {
    offset: u_int,
    data: unnamed_9,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const WINDOW_COPY_OFF: unnamed_6 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const WINDOW_COPY_REL_POS_BELOW: unnamed_38 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const TTY_VT101: unnamed_25 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
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
    pub alerts_entry: unnamed_22,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_39,
    pub entry: unnamed_18,
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
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const TTY_VT100: unnamed_25 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
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
pub type cmdq_type = libc::c_uint;
pub type speed_t = libc::c_uint;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
pub const PROMPT_COMMAND: unnamed_10 = 1;
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
    pub entry: unnamed_3,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub type unnamed_38 = libc::c_uint;
pub type layout_type = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type pid_t = __pid_t;
pub type __u_short = libc::c_ushort;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_17,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type __u_int = libc::c_uint;
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
    pub message_log: unnamed_36,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_10,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_21,
}
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub type __time_t = libc::c_long;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub type __pid_t = libc::c_int;
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
pub struct unnamed_39 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
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
pub struct unnamed_40 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub type u_char = __u_char;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const TTY_VT320: unnamed_25 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_copy_mode_data {
    pub screen: screen,
    pub backing: *mut screen,
    pub backing_written: libc::c_int,
    pub oy: u_int,
    pub selx: u_int,
    pub sely: u_int,
    pub endselx: u_int,
    pub endsely: u_int,
    pub cursordrag: unnamed_19,
    pub rectflag: libc::c_int,
    pub scroll_exit: libc::c_int,
    pub cx: u_int,
    pub cy: u_int,
    pub lastcx: u_int,
    pub lastsx: u_int,
    pub searchtype: libc::c_int,
    pub searchstr: *mut libc::c_char,
    pub searchmark: *mut bitstr_t,
    pub searchcount: u_int,
    pub searchthis: libc::c_int,
    pub searchx: libc::c_int,
    pub searchy: libc::c_int,
    pub searcho: libc::c_int,
    pub jumptype: libc::c_int,
    pub jumpchar: libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_41 {
    pub ev_signal_next: unnamed_2,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[no_mangle]
pub static mut window_copy_mode: window_mode =
    unsafe {
        window_mode{name:
                        b"copy-mode\x00" as *const u8 as *const libc::c_char,
                    init: Some(window_copy_init),
                    free: Some(window_copy_free),
                    resize: Some(window_copy_resize),
                    key: None,
                    key_table: Some(window_copy_key_table),
                    command: Some(window_copy_command),}
    };
unsafe extern "C" fn window_copy_command(mut wp: *mut window_pane,
                                         mut c: *mut client,
                                         mut s: *mut session,
                                         mut args: *mut args,
                                         mut m: *mut mouse_event) -> () {
    let mut current_block: u64;
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut sn: *mut screen = &mut (*data).screen as *mut screen;
    let mut command: *const libc::c_char = 0 as *const libc::c_char;
    let mut argument: *const libc::c_char = 0 as *const libc::c_char;
    let mut ws: *const libc::c_char = 0 as *const libc::c_char;
    let mut np: u_int = (*wp).modeprefix;
    let mut cancel: libc::c_int = 0i32;
    let mut redraw: libc::c_int = 0i32;
    let mut scroll_exit: libc::c_int = 0;
    let mut prefix: libc::c_char = 0;
    if (*args).argc == 0i32 {
        return
    } else {
        command = *(*args).argv.offset(0isize);
        if m != 0 as *mut libc::c_void as *mut mouse_event && 0 != (*m).valid
           {
            window_copy_move_mouse(m);
        }
        if (*args).argc == 1i32 {
            if strcmp(command,
                      b"append-selection\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                if s != 0 as *mut libc::c_void as *mut session {
                    window_copy_append_selection(wp,
                                                 0 as *const libc::c_char);
                }
                window_copy_clear_selection(wp);
                redraw = 1i32
            }
            if strcmp(command,
                      b"append-selection-and-cancel\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                if s != 0 as *mut libc::c_void as *mut session {
                    window_copy_append_selection(wp,
                                                 0 as *const libc::c_char);
                }
                window_copy_clear_selection(wp);
                redraw = 1i32;
                cancel = 1i32
            }
            if strcmp(command,
                      b"back-to-indentation\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                window_copy_cursor_back_to_indentation(wp);
            }
            if strcmp(command,
                      b"begin-selection\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                if m != 0 as *mut libc::c_void as *mut mouse_event {
                    window_copy_start_drag(c, m);
                } else {
                    (*sn).sel.lineflag = LINE_SEL_NONE;
                    window_copy_start_selection(wp);
                    redraw = 1i32
                }
            }
            if strcmp(command,
                      b"stop-selection\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                (*data).cursordrag = CURSORDRAG_NONE
            }
            if strcmp(command,
                      b"bottom-line\x00" as *const u8 as *const libc::c_char)
                   == 0i32 {
                (*data).cx = 0i32 as u_int;
                (*data).cy =
                    (*(*sn).grid).sy.wrapping_sub(1i32 as libc::c_uint);
                window_copy_update_selection(wp, 1i32);
                redraw = 1i32
            }
            if strcmp(command,
                      b"cancel\x00" as *const u8 as *const libc::c_char) ==
                   0i32 {
                cancel = 1i32
            }
            if strcmp(command,
                      b"clear-selection\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                window_copy_clear_selection(wp);
                redraw = 1i32
            }
            if strcmp(command,
                      b"copy-end-of-line\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                window_copy_start_selection(wp);
                while np > 1i32 as libc::c_uint {
                    window_copy_cursor_down(wp, 0i32);
                    np = np.wrapping_sub(1)
                }
                window_copy_cursor_end_of_line(wp);
                redraw = 1i32;
                if s != 0 as *mut libc::c_void as *mut session {
                    window_copy_copy_selection(wp, 0 as *const libc::c_char);
                    cancel = 1i32
                }
            }
            if strcmp(command,
                      b"copy-line\x00" as *const u8 as *const libc::c_char) ==
                   0i32 {
                window_copy_cursor_start_of_line(wp);
                window_copy_start_selection(wp);
                while np > 1i32 as libc::c_uint {
                    window_copy_cursor_down(wp, 0i32);
                    np = np.wrapping_sub(1)
                }
                window_copy_cursor_end_of_line(wp);
                redraw = 1i32;
                if s != 0 as *mut libc::c_void as *mut session {
                    window_copy_copy_selection(wp, 0 as *const libc::c_char);
                    cancel = 1i32
                }
            }
            if strcmp(command,
                      b"copy-selection\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                if s != 0 as *mut libc::c_void as *mut session {
                    window_copy_copy_selection(wp, 0 as *const libc::c_char);
                }
                window_copy_clear_selection(wp);
                redraw = 1i32
            }
            if strcmp(command,
                      b"copy-selection-and-cancel\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                if s != 0 as *mut libc::c_void as *mut session {
                    window_copy_copy_selection(wp, 0 as *const libc::c_char);
                }
                window_copy_clear_selection(wp);
                redraw = 1i32;
                cancel = 1i32
            }
            if strcmp(command,
                      b"cursor-down\x00" as *const u8 as *const libc::c_char)
                   == 0i32 {
                current_block = 5689316957504528238;
            } else { current_block = 14359455889292382949; }
            loop  {
                match current_block {
                    5689316957504528238 => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 14359455889292382949;
                            continue ;
                        }
                        window_copy_cursor_down(wp, 0i32);
                        np = np.wrapping_sub(1);
                        current_block = 5689316957504528238;
                    }
                    _ => {
                        if strcmp(command,
                                  b"cursor-left\x00" as *const u8 as
                                      *const libc::c_char) == 0i32 {
                            current_block = 2891135413264362348;
                            break ;
                        } else {
                            current_block = 11636175345244025579;
                            break ;
                        }
                    }
                }
            }
            loop  {
                match current_block {
                    2891135413264362348 => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 11636175345244025579;
                            continue ;
                        }
                        window_copy_cursor_left(wp);
                        np = np.wrapping_sub(1);
                        current_block = 2891135413264362348;
                    }
                    _ => {
                        if strcmp(command,
                                  b"cursor-right\x00" as *const u8 as
                                      *const libc::c_char) == 0i32 {
                            current_block = 18153031941552419006;
                            break ;
                        } else {
                            current_block = 17500079516916021833;
                            break ;
                        }
                    }
                }
            }
            loop  {
                match current_block {
                    17500079516916021833 => {
                        if strcmp(command,
                                  b"cursor-up\x00" as *const u8 as
                                      *const libc::c_char) == 0i32 {
                            current_block = 1836292691772056875;
                            break ;
                        } else {
                            current_block = 7245201122033322888;
                            break ;
                        }
                    }
                    _ => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 17500079516916021833;
                            continue ;
                        }
                        window_copy_cursor_right(wp);
                        np = np.wrapping_sub(1);
                        current_block = 18153031941552419006;
                    }
                }
            }
            loop  {
                match current_block {
                    7245201122033322888 => {
                        if !(strcmp(command,
                                    b"end-of-line\x00" as *const u8 as
                                        *const libc::c_char) == 0i32) {
                            break ;
                        }
                        window_copy_cursor_end_of_line(wp);
                        break ;
                    }
                    _ => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 7245201122033322888;
                            continue ;
                        }
                        window_copy_cursor_up(wp, 0i32);
                        np = np.wrapping_sub(1);
                        current_block = 1836292691772056875;
                    }
                }
            }
            if strcmp(command,
                      b"halfpage-down\x00" as *const u8 as
                          *const libc::c_char) == 0i32 ||
                   strcmp(command,
                          b"halfpage-down-and-cancel\x00" as *const u8 as
                              *const libc::c_char) == 0i32 {
                if strcmp(command,
                          b"halfpage-down-and-cancel\x00" as *const u8 as
                              *const libc::c_char) == 0i32 {
                    scroll_exit = 1i32
                } else { scroll_exit = (*data).scroll_exit }
                while np != 0i32 as libc::c_uint {
                    if 0 != window_copy_pagedown(wp, 1i32, scroll_exit) {
                        cancel = 1i32;
                        break ;
                    } else { np = np.wrapping_sub(1) }
                }
            }
            if strcmp(command,
                      b"halfpage-up\x00" as *const u8 as *const libc::c_char)
                   == 0i32 {
                current_block = 721385680381463314;
            } else { current_block = 10758786907990354186; }
            loop  {
                match current_block {
                    721385680381463314 => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 10758786907990354186;
                            continue ;
                        }
                        window_copy_pageup(wp, 1i32);
                        np = np.wrapping_sub(1);
                        current_block = 721385680381463314;
                    }
                    _ => {
                        if !(strcmp(command,
                                    b"history-bottom\x00" as *const u8 as
                                        *const libc::c_char) == 0i32) {
                            break ;
                        }
                        (*data).cx = 0i32 as u_int;
                        (*data).cy =
                            (*(*sn).grid).sy.wrapping_sub(1i32 as
                                                              libc::c_uint);
                        (*data).oy = 0i32 as u_int;
                        window_copy_update_selection(wp, 1i32);
                        redraw = 1i32;
                        break ;
                    }
                }
            }
            if strcmp(command,
                      b"history-top\x00" as *const u8 as *const libc::c_char)
                   == 0i32 {
                (*data).cx = 0i32 as u_int;
                (*data).cy = 0i32 as u_int;
                (*data).oy = (*(*(*data).backing).grid).hsize;
                window_copy_update_selection(wp, 1i32);
                redraw = 1i32
            }
            if strcmp(command,
                      b"jump-again\x00" as *const u8 as *const libc::c_char)
                   == 0i32 {
                match (*data).jumptype {
                    3 => {
                        current_block = 11793792312832361944;
                        loop  {
                            match current_block {
                                7420279277351916581 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 7420279277351916581;
                                }
                                9353995356876505083 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 9353995356876505083;
                                }
                                11793792312832361944 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 11793792312832361944;
                                }
                                _ => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 2290177392965769716;
                                }
                            }
                        }
                    }
                    4 => {
                        current_block = 7420279277351916581;
                        loop  {
                            match current_block {
                                7420279277351916581 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 7420279277351916581;
                                }
                                9353995356876505083 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 9353995356876505083;
                                }
                                11793792312832361944 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 11793792312832361944;
                                }
                                _ => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 2290177392965769716;
                                }
                            }
                        }
                    }
                    5 => {
                        current_block = 2290177392965769716;
                        loop  {
                            match current_block {
                                7420279277351916581 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 7420279277351916581;
                                }
                                9353995356876505083 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 9353995356876505083;
                                }
                                11793792312832361944 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 11793792312832361944;
                                }
                                _ => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 2290177392965769716;
                                }
                            }
                        }
                    }
                    6 => {
                        current_block = 9353995356876505083;
                        loop  {
                            match current_block {
                                7420279277351916581 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 7420279277351916581;
                                }
                                9353995356876505083 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 9353995356876505083;
                                }
                                11793792312832361944 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 11793792312832361944;
                                }
                                _ => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 2290177392965769716;
                                }
                            }
                        }
                    }
                    _ => { }
                }
            }
            if strcmp(command,
                      b"jump-reverse\x00" as *const u8 as *const libc::c_char)
                   == 0i32 {
                match (*data).jumptype {
                    3 => {
                        current_block = 9241535491006583629;
                        loop  {
                            match current_block {
                                4741994311446740739 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 4741994311446740739;
                                }
                                9241535491006583629 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 9241535491006583629;
                                }
                                7252614138838059896 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 7252614138838059896;
                                }
                                _ => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 17075014677070940716;
                                }
                            }
                        }
                    }
                    4 => {
                        current_block = 4741994311446740739;
                        loop  {
                            match current_block {
                                4741994311446740739 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 4741994311446740739;
                                }
                                9241535491006583629 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 9241535491006583629;
                                }
                                7252614138838059896 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 7252614138838059896;
                                }
                                _ => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 17075014677070940716;
                                }
                            }
                        }
                    }
                    5 => {
                        current_block = 7252614138838059896;
                        loop  {
                            match current_block {
                                4741994311446740739 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 4741994311446740739;
                                }
                                9241535491006583629 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 9241535491006583629;
                                }
                                7252614138838059896 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 7252614138838059896;
                                }
                                _ => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 17075014677070940716;
                                }
                            }
                        }
                    }
                    6 => {
                        current_block = 17075014677070940716;
                        loop  {
                            match current_block {
                                4741994311446740739 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 4741994311446740739;
                                }
                                9241535491006583629 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 9241535491006583629;
                                }
                                7252614138838059896 => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to_back(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 7252614138838059896;
                                }
                                _ => {
                                    if !(np != 0i32 as libc::c_uint) {
                                        break ;
                                    }
                                    window_copy_cursor_jump_to(wp);
                                    np = np.wrapping_sub(1);
                                    current_block = 17075014677070940716;
                                }
                            }
                        }
                    }
                    _ => { }
                }
            }
            if strcmp(command,
                      b"middle-line\x00" as *const u8 as *const libc::c_char)
                   == 0i32 {
                (*data).cx = 0i32 as u_int;
                (*data).cy =
                    (*(*sn).grid).sy.wrapping_sub(1i32 as
                                                      libc::c_uint).wrapping_div(2i32
                                                                                     as
                                                                                     libc::c_uint);
                window_copy_update_selection(wp, 1i32);
                redraw = 1i32
            }
            if strcmp(command,
                      b"next-paragraph\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                current_block = 1874315696050160458;
            } else { current_block = 981995395831942902; }
            loop  {
                match current_block {
                    1874315696050160458 => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 981995395831942902;
                            continue ;
                        }
                        window_copy_next_paragraph(wp);
                        np = np.wrapping_sub(1);
                        current_block = 1874315696050160458;
                    }
                    _ => {
                        if strcmp(command,
                                  b"next-space\x00" as *const u8 as
                                      *const libc::c_char) == 0i32 {
                            current_block = 4216521074440650966;
                            break ;
                        } else {
                            current_block = 5028470053297453708;
                            break ;
                        }
                    }
                }
            }
            loop  {
                match current_block {
                    4216521074440650966 => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 5028470053297453708;
                            continue ;
                        }
                        window_copy_cursor_next_word(wp,
                                                     b" \x00" as *const u8 as
                                                         *const libc::c_char);
                        np = np.wrapping_sub(1);
                        current_block = 4216521074440650966;
                    }
                    _ => {
                        if strcmp(command,
                                  b"next-space-end\x00" as *const u8 as
                                      *const libc::c_char) == 0i32 {
                            current_block = 13853033528615664019;
                            break ;
                        } else { current_block = 129780949503461575; break ; }
                    }
                }
            }
            loop  {
                match current_block {
                    129780949503461575 => {
                        if !(strcmp(command,
                                    b"next-word\x00" as *const u8 as
                                        *const libc::c_char) == 0i32) {
                            current_block = 777662472977924419;
                            break ;
                        }
                        ws =
                            options_get_string((*s).options,
                                               b"word-separators\x00" as
                                                   *const u8 as
                                                   *const libc::c_char);
                        current_block = 2723324002591448311;
                        break ;
                    }
                    _ => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 129780949503461575;
                            continue ;
                        }
                        window_copy_cursor_next_word_end(wp,
                                                         b" \x00" as *const u8
                                                             as
                                                             *const libc::c_char);
                        np = np.wrapping_sub(1);
                        current_block = 13853033528615664019;
                    }
                }
            }
            loop  {
                match current_block {
                    2723324002591448311 => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 777662472977924419;
                            continue ;
                        }
                        window_copy_cursor_next_word(wp, ws);
                        np = np.wrapping_sub(1);
                        current_block = 2723324002591448311;
                    }
                    _ => {
                        if !(strcmp(command,
                                    b"next-word-end\x00" as *const u8 as
                                        *const libc::c_char) == 0i32) {
                            current_block = 5873035170358615968;
                            break ;
                        }
                        ws =
                            options_get_string((*s).options,
                                               b"word-separators\x00" as
                                                   *const u8 as
                                                   *const libc::c_char);
                        current_block = 919954187481050311;
                        break ;
                    }
                }
            }
            loop  {
                match current_block {
                    5873035170358615968 => {
                        if !(strcmp(command,
                                    b"other-end\x00" as *const u8 as
                                        *const libc::c_char) == 0i32) {
                            current_block = 11052029508375673978;
                            break ;
                        }
                        if np.wrapping_rem(2i32 as libc::c_uint) !=
                               0i32 as libc::c_uint {
                            current_block = 15514718523126015390;
                            break ;
                        } else {
                            current_block = 11052029508375673978;
                            break ;
                        }
                    }
                    _ => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 5873035170358615968;
                            continue ;
                        }
                        window_copy_cursor_next_word_end(wp, ws);
                        np = np.wrapping_sub(1);
                        current_block = 919954187481050311;
                    }
                }
            }
            match current_block {
                15514718523126015390 => { window_copy_other_end(wp); }
                _ => { }
            }
            if strcmp(command,
                      b"page-down\x00" as *const u8 as *const libc::c_char) ==
                   0i32 ||
                   strcmp(command,
                          b"page-down-and-cancel\x00" as *const u8 as
                              *const libc::c_char) == 0i32 {
                if strcmp(command,
                          b"page-down-and-cancel\x00" as *const u8 as
                              *const libc::c_char) == 0i32 {
                    scroll_exit = 1i32
                } else { scroll_exit = (*data).scroll_exit }
                while np != 0i32 as libc::c_uint {
                    if 0 != window_copy_pagedown(wp, 0i32, scroll_exit) {
                        cancel = 1i32;
                        break ;
                    } else { np = np.wrapping_sub(1) }
                }
            }
            if strcmp(command,
                      b"page-up\x00" as *const u8 as *const libc::c_char) ==
                   0i32 {
                current_block = 14294131666767243020;
            } else { current_block = 14648606000749551097; }
            loop  {
                match current_block {
                    14294131666767243020 => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 14648606000749551097;
                            continue ;
                        }
                        window_copy_pageup(wp, 0i32);
                        np = np.wrapping_sub(1);
                        current_block = 14294131666767243020;
                    }
                    _ => {
                        if strcmp(command,
                                  b"previous-paragraph\x00" as *const u8 as
                                      *const libc::c_char) == 0i32 {
                            current_block = 10213293998891106930;
                            break ;
                        } else {
                            current_block = 5722677567366458307;
                            break ;
                        }
                    }
                }
            }
            loop  {
                match current_block {
                    5722677567366458307 => {
                        if strcmp(command,
                                  b"previous-space\x00" as *const u8 as
                                      *const libc::c_char) == 0i32 {
                            current_block = 12705158477165241210;
                            break ;
                        } else {
                            current_block = 5005389895767293342;
                            break ;
                        }
                    }
                    _ => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 5722677567366458307;
                            continue ;
                        }
                        window_copy_previous_paragraph(wp);
                        np = np.wrapping_sub(1);
                        current_block = 10213293998891106930;
                    }
                }
            }
            loop  {
                match current_block {
                    12705158477165241210 => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 5005389895767293342;
                            continue ;
                        }
                        window_copy_cursor_previous_word(wp,
                                                         b" \x00" as *const u8
                                                             as
                                                             *const libc::c_char);
                        np = np.wrapping_sub(1);
                        current_block = 12705158477165241210;
                    }
                    _ => {
                        if !(strcmp(command,
                                    b"previous-word\x00" as *const u8 as
                                        *const libc::c_char) == 0i32) {
                            current_block = 11869735117417356968;
                            break ;
                        }
                        ws =
                            options_get_string((*s).options,
                                               b"word-separators\x00" as
                                                   *const u8 as
                                                   *const libc::c_char);
                        current_block = 4928646496496689183;
                        break ;
                    }
                }
            }
            loop  {
                match current_block {
                    4928646496496689183 => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 11869735117417356968;
                            continue ;
                        }
                        window_copy_cursor_previous_word(wp, ws);
                        np = np.wrapping_sub(1);
                        current_block = 4928646496496689183;
                    }
                    _ => {
                        if !(strcmp(command,
                                    b"rectangle-toggle\x00" as *const u8 as
                                        *const libc::c_char) == 0i32) {
                            break ;
                        }
                        (*sn).sel.lineflag = LINE_SEL_NONE;
                        window_copy_rectangle_toggle(wp);
                        break ;
                    }
                }
            }
            if strcmp(command,
                      b"scroll-down\x00" as *const u8 as *const libc::c_char)
                   == 0i32 ||
                   strcmp(command,
                          b"scroll-down-and-cancel\x00" as *const u8 as
                              *const libc::c_char) == 0i32 {
                if strcmp(command,
                          b"scroll-down-and-cancel\x00" as *const u8 as
                              *const libc::c_char) == 0i32 {
                    scroll_exit = 1i32
                } else { scroll_exit = (*data).scroll_exit }
                while np != 0i32 as libc::c_uint {
                    window_copy_cursor_down(wp, 1i32);
                    np = np.wrapping_sub(1)
                }
                if 0 != scroll_exit && (*data).oy == 0i32 as libc::c_uint {
                    cancel = 1i32
                }
            }
            if strcmp(command,
                      b"scroll-up\x00" as *const u8 as *const libc::c_char) ==
                   0i32 {
                current_block = 14865402277128115059;
            } else { current_block = 15734707049249739970; }
            loop  {
                match current_block {
                    14865402277128115059 => {
                        if !(np != 0i32 as libc::c_uint) {
                            current_block = 15734707049249739970;
                            continue ;
                        }
                        window_copy_cursor_up(wp, 1i32);
                        np = np.wrapping_sub(1);
                        current_block = 14865402277128115059;
                    }
                    _ => {
                        if !(strcmp(command,
                                    b"search-again\x00" as *const u8 as
                                        *const libc::c_char) == 0i32) {
                            current_block = 1934991416718554651;
                            break ;
                        }
                        if (*data).searchtype ==
                               WINDOW_COPY_SEARCHUP as libc::c_int {
                            current_block = 5700653730392116747;
                            break ;
                        } else { current_block = 796174441944384681; break ; }
                    }
                }
            }
            match current_block {
                796174441944384681 => {
                    if (*data).searchtype ==
                           WINDOW_COPY_SEARCHDOWN as libc::c_int {
                        while np != 0i32 as libc::c_uint {
                            window_copy_search_down(wp);
                            np = np.wrapping_sub(1)
                        }
                    }
                }
                5700653730392116747 => {
                    while np != 0i32 as libc::c_uint {
                        window_copy_search_up(wp);
                        np = np.wrapping_sub(1)
                    }
                }
                _ => { }
            }
            if strcmp(command,
                      b"search-reverse\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                if (*data).searchtype == WINDOW_COPY_SEARCHUP as libc::c_int {
                    while np != 0i32 as libc::c_uint {
                        window_copy_search_down(wp);
                        np = np.wrapping_sub(1)
                    }
                } else if (*data).searchtype ==
                              WINDOW_COPY_SEARCHDOWN as libc::c_int {
                    while np != 0i32 as libc::c_uint {
                        window_copy_search_up(wp);
                        np = np.wrapping_sub(1)
                    }
                }
            }
            if strcmp(command,
                      b"select-line\x00" as *const u8 as *const libc::c_char)
                   == 0i32 {
                (*sn).sel.lineflag = LINE_SEL_LEFT_RIGHT;
                (*data).rectflag = 0i32;
                window_copy_cursor_start_of_line(wp);
                window_copy_start_selection(wp);
                while np > 1i32 as libc::c_uint {
                    window_copy_cursor_down(wp, 0i32);
                    np = np.wrapping_sub(1)
                }
                window_copy_cursor_end_of_line(wp);
                redraw = 1i32
            }
            if strcmp(command,
                      b"select-word\x00" as *const u8 as *const libc::c_char)
                   == 0i32 {
                (*sn).sel.lineflag = LINE_SEL_LEFT_RIGHT;
                (*data).rectflag = 0i32;
                ws =
                    options_get_string((*s).options,
                                       b"word-separators\x00" as *const u8 as
                                           *const libc::c_char);
                window_copy_cursor_previous_word(wp, ws);
                window_copy_start_selection(wp);
                window_copy_cursor_next_word_end(wp, ws);
                redraw = 1i32
            }
            if strcmp(command,
                      b"start-of-line\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                window_copy_cursor_start_of_line(wp);
            }
            if strcmp(command,
                      b"top-line\x00" as *const u8 as *const libc::c_char) ==
                   0i32 {
                (*data).cx = 0i32 as u_int;
                (*data).cy = 0i32 as u_int;
                window_copy_update_selection(wp, 1i32);
                redraw = 1i32
            }
        } else if (*args).argc == 2i32 &&
                      **(*args).argv.offset(1isize) as libc::c_int != 0 {
            argument = *(*args).argv.offset(1isize);
            if strcmp(command,
                      b"copy-pipe\x00" as *const u8 as *const libc::c_char) ==
                   0i32 {
                if s != 0 as *mut libc::c_void as *mut session {
                    window_copy_copy_pipe(wp, s, 0 as *const libc::c_char,
                                          argument);
                }
            }
            if strcmp(command,
                      b"copy-pipe-and-cancel\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                if s != 0 as *mut libc::c_void as *mut session {
                    window_copy_copy_pipe(wp, s, 0 as *const libc::c_char,
                                          argument);
                    cancel = 1i32
                }
            }
            if strcmp(command,
                      b"goto-line\x00" as *const u8 as *const libc::c_char) ==
                   0i32 {
                window_copy_goto_line(wp, argument);
            }
            if strcmp(command,
                      b"jump-backward\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                (*data).jumptype = WINDOW_COPY_JUMPBACKWARD as libc::c_int;
                (*data).jumpchar = *argument;
                while np != 0i32 as libc::c_uint {
                    window_copy_cursor_jump_back(wp);
                    np = np.wrapping_sub(1)
                }
            }
            if strcmp(command,
                      b"jump-forward\x00" as *const u8 as *const libc::c_char)
                   == 0i32 {
                (*data).jumptype = WINDOW_COPY_JUMPFORWARD as libc::c_int;
                (*data).jumpchar = *argument;
                while np != 0i32 as libc::c_uint {
                    window_copy_cursor_jump(wp);
                    np = np.wrapping_sub(1)
                }
            }
            if strcmp(command,
                      b"jump-to-backward\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                (*data).jumptype = WINDOW_COPY_JUMPTOBACKWARD as libc::c_int;
                (*data).jumpchar = *argument;
                while np != 0i32 as libc::c_uint {
                    window_copy_cursor_jump_to_back(wp);
                    np = np.wrapping_sub(1)
                }
            }
            if strcmp(command,
                      b"jump-to-forward\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                (*data).jumptype = WINDOW_COPY_JUMPTOFORWARD as libc::c_int;
                (*data).jumpchar = *argument;
                while np != 0i32 as libc::c_uint {
                    window_copy_cursor_jump_to(wp);
                    np = np.wrapping_sub(1)
                }
            }
            if strcmp(command,
                      b"search-backward\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                (*data).searchtype = WINDOW_COPY_SEARCHUP as libc::c_int;
                free((*data).searchstr as *mut libc::c_void);
                (*data).searchstr = xstrdup(argument);
                while np != 0i32 as libc::c_uint {
                    window_copy_search_up(wp);
                    np = np.wrapping_sub(1)
                }
            }
            if strcmp(command,
                      b"search-forward\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                (*data).searchtype = WINDOW_COPY_SEARCHDOWN as libc::c_int;
                free((*data).searchstr as *mut libc::c_void);
                (*data).searchstr = xstrdup(argument);
                while np != 0i32 as libc::c_uint {
                    window_copy_search_down(wp);
                    np = np.wrapping_sub(1)
                }
            }
            if strcmp(command,
                      b"search-backward-incremental\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                let fresh0 = argument;
                argument = argument.offset(1);
                prefix = *fresh0;
                if (*data).searchx == 1i32.wrapping_neg() ||
                       (*data).searchy == 1i32.wrapping_neg() {
                    (*data).searchx = (*data).cx as libc::c_int;
                    (*data).searchy = (*data).cy as libc::c_int;
                    (*data).searcho = (*data).oy as libc::c_int
                } else if (*data).searchstr !=
                              0 as *mut libc::c_void as *mut libc::c_char &&
                              strcmp(argument, (*data).searchstr) != 0i32 {
                    (*data).cx = (*data).searchx as u_int;
                    (*data).cy = (*data).searchy as u_int;
                    (*data).oy = (*data).searcho as u_int;
                    redraw = 1i32
                }
                if *argument as libc::c_int == 0 {
                    window_copy_clear_marks(wp);
                    redraw = 1i32
                } else if prefix as libc::c_int == 61 ||
                              prefix as libc::c_int == 45 {
                    (*data).searchtype = WINDOW_COPY_SEARCHUP as libc::c_int;
                    free((*data).searchstr as *mut libc::c_void);
                    (*data).searchstr = xstrdup(argument);
                    if 0 == window_copy_search_up(wp) {
                        window_copy_clear_marks(wp);
                        redraw = 1i32
                    }
                } else if prefix as libc::c_int == 43 {
                    (*data).searchtype =
                        WINDOW_COPY_SEARCHDOWN as libc::c_int;
                    free((*data).searchstr as *mut libc::c_void);
                    (*data).searchstr = xstrdup(argument);
                    if 0 == window_copy_search_down(wp) {
                        window_copy_clear_marks(wp);
                        redraw = 1i32
                    }
                }
            }
            if strcmp(command,
                      b"search-forward-incremental\x00" as *const u8 as
                          *const libc::c_char) == 0i32 {
                let fresh1 = argument;
                argument = argument.offset(1);
                prefix = *fresh1;
                if (*data).searchx == 1i32.wrapping_neg() ||
                       (*data).searchy == 1i32.wrapping_neg() {
                    (*data).searchx = (*data).cx as libc::c_int;
                    (*data).searchy = (*data).cy as libc::c_int;
                    (*data).searcho = (*data).oy as libc::c_int
                } else if (*data).searchstr !=
                              0 as *mut libc::c_void as *mut libc::c_char &&
                              strcmp(argument, (*data).searchstr) != 0i32 {
                    (*data).cx = (*data).searchx as u_int;
                    (*data).cy = (*data).searchy as u_int;
                    (*data).oy = (*data).searcho as u_int;
                    redraw = 1i32
                }
                if *argument as libc::c_int == 0 {
                    window_copy_clear_marks(wp);
                    redraw = 1i32
                } else if prefix as libc::c_int == 61 ||
                              prefix as libc::c_int == 43 {
                    (*data).searchtype =
                        WINDOW_COPY_SEARCHDOWN as libc::c_int;
                    free((*data).searchstr as *mut libc::c_void);
                    (*data).searchstr = xstrdup(argument);
                    if 0 == window_copy_search_down(wp) {
                        window_copy_clear_marks(wp);
                        redraw = 1i32
                    }
                } else if prefix as libc::c_int == 45 {
                    (*data).searchtype = WINDOW_COPY_SEARCHUP as libc::c_int;
                    free((*data).searchstr as *mut libc::c_void);
                    (*data).searchstr = xstrdup(argument);
                    if 0 == window_copy_search_up(wp) {
                        window_copy_clear_marks(wp);
                        redraw = 1i32
                    }
                }
            }
        }
        if strncmp(command,
                   b"search-\x00" as *const u8 as *const libc::c_char,
                   7i32 as libc::c_ulong) != 0i32 &&
               (*data).searchmark != 0 as *mut libc::c_void as *mut bitstr_t {
            window_copy_clear_marks(wp);
            redraw = 1i32;
            (*data).searchy = 1i32.wrapping_neg();
            (*data).searchx = (*data).searchy
        }
        if 0 != cancel {
            window_pane_reset_mode(wp);
        } else if 0 != redraw { window_copy_redraw_screen(wp); }
        (*wp).modeprefix = 1i32 as u_int;
        return;
    };
}
unsafe extern "C" fn window_copy_redraw_screen(mut wp: *mut window_pane)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    window_copy_redraw_lines(wp, 0i32 as u_int,
                             (*(*(&mut (*data).screen as
                                      *mut screen)).grid).sy);
}
unsafe extern "C" fn window_copy_redraw_lines(mut wp: *mut window_pane,
                                              mut py: u_int, mut ny: u_int)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut ctx: screen_write_ctx =
        screen_write_ctx{wp: 0 as *mut window_pane,
                         s: 0 as *mut screen,
                         item: 0 as *mut screen_write_collect_item,
                         list: 0 as *mut screen_write_collect_line,
                         scrolled: 0,
                         bg: 0,
                         cells: 0,
                         written: 0,
                         skipped: 0,};
    let mut i: u_int = 0;
    screen_write_start(&mut ctx as *mut screen_write_ctx, wp,
                       0 as *mut screen);
    i = py;
    while i < py.wrapping_add(ny) {
        window_copy_write_line(wp, &mut ctx as *mut screen_write_ctx, i);
        i = i.wrapping_add(1)
    }
    screen_write_cursormove(&mut ctx as *mut screen_write_ctx, (*data).cx,
                            (*data).cy);
    screen_write_stop(&mut ctx as *mut screen_write_ctx);
}
unsafe extern "C" fn window_copy_write_line(mut wp: *mut window_pane,
                                            mut ctx: *mut screen_write_ctx,
                                            mut py: u_int) -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut oo: *mut options = (*(*wp).window).options;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut hdr: [libc::c_char; 512] = [0; 512];
    let mut size: size_t = 0i32 as size_t;
    style_apply(&mut gc as *mut grid_cell, oo,
                b"mode-style\x00" as *const u8 as *const libc::c_char);
    gc.flags = (gc.flags as libc::c_int | 32i32) as u_char;
    if py == 0i32 as libc::c_uint {
        if (*data).searchmark == 0 as *mut libc::c_void as *mut bitstr_t {
            size =
                xsnprintf(hdr.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 512]>() as
                              libc::c_ulong,
                          b"[%u/%u]\x00" as *const u8 as *const libc::c_char,
                          (*data).oy, (*(*(*data).backing).grid).hsize) as
                    size_t
        } else if (*data).searchthis == 1i32.wrapping_neg() {
            size =
                xsnprintf(hdr.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 512]>() as
                              libc::c_ulong,
                          b"(%u results) [%d/%u]\x00" as *const u8 as
                              *const libc::c_char, (*data).searchcount,
                          (*data).oy, (*(*(*data).backing).grid).hsize) as
                    size_t
        } else {
            size =
                xsnprintf(hdr.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 512]>() as
                              libc::c_ulong,
                          b"(%u/%u results) [%d/%u]\x00" as *const u8 as
                              *const libc::c_char, (*data).searchthis,
                          (*data).searchcount, (*data).oy,
                          (*(*(*data).backing).grid).hsize) as size_t
        }
        if size > (*(*s).grid).sx as libc::c_ulong {
            size = (*(*s).grid).sx as size_t
        }
        screen_write_cursormove(ctx,
                                ((*(*s).grid).sx as
                                     libc::c_ulong).wrapping_sub(size) as
                                    u_int, 0i32 as u_int);
        screen_write_puts(ctx, &mut gc as *mut grid_cell,
                          b"%s\x00" as *const u8 as *const libc::c_char,
                          hdr.as_mut_ptr());
    } else { size = 0i32 as size_t }
    if size < (*(*s).grid).sx as libc::c_ulong {
        screen_write_cursormove(ctx, 0i32 as u_int, py);
        screen_write_copy(ctx, (*data).backing, 0i32 as u_int,
                          (*(*(*data).backing).grid).hsize.wrapping_sub((*data).oy).wrapping_add(py),
                          ((*(*s).grid).sx as
                               libc::c_ulong).wrapping_sub(size) as u_int,
                          1i32 as u_int, (*data).searchmark,
                          &mut gc as *mut grid_cell);
    }
    if py == (*data).cy && (*data).cx == (*(*s).grid).sx {
        memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
               &grid_default_cell as *const grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        screen_write_cursormove(ctx,
                                (*(*s).grid).sx.wrapping_sub(1i32 as
                                                                 libc::c_uint),
                                py);
        screen_write_putc(ctx, &mut gc as *mut grid_cell, 36 as u_char);
    };
}
unsafe extern "C" fn window_copy_clear_marks(mut wp: *mut window_pane) -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    free((*data).searchmark as *mut libc::c_void);
    (*data).searchmark = 0 as *mut bitstr_t;
}
unsafe extern "C" fn window_copy_search_up(mut wp: *mut window_pane)
 -> libc::c_int {
    return window_copy_search(wp, 0i32);
}
unsafe extern "C" fn window_copy_search(mut wp: *mut window_pane,
                                        mut direction: libc::c_int)
 -> libc::c_int {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = (*data).backing;
    let mut ss: screen =
        screen{title: 0 as *mut libc::c_char,
               titles: 0 as *mut screen_titles,
               grid: 0 as *mut grid,
               cx: 0,
               cy: 0,
               cstyle: 0,
               ccolour: 0 as *mut libc::c_char,
               rupper: 0,
               rlower: 0,
               mode: 0,
               tabs: 0 as *mut bitstr_t,
               sel:
                   screen_sel{flag: 0,
                              hidden: 0,
                              rectflag: 0,
                              lineflag: LINE_SEL_NONE,
                              modekeys: 0,
                              sx: 0,
                              sy: 0,
                              ex: 0,
                              ey: 0,
                              cell:
                                  grid_cell{flags: 0,
                                            attr: 0,
                                            fg: 0,
                                            bg: 0,
                                            data:
                                                utf8_data{data: [0; 9],
                                                          have: 0,
                                                          size: 0,
                                                          width: 0,},},},};
    let mut ctx: screen_write_ctx =
        screen_write_ctx{wp: 0 as *mut window_pane,
                         s: 0 as *mut screen,
                         item: 0 as *mut screen_write_collect_item,
                         list: 0 as *mut screen_write_collect_line,
                         scrolled: 0,
                         bg: 0,
                         cells: 0,
                         written: 0,
                         skipped: 0,};
    let mut gd: *mut grid = (*s).grid;
    let mut fx: u_int = 0;
    let mut fy: u_int = 0;
    let mut endline: u_int = 0;
    let mut wrapflag: libc::c_int = 0;
    let mut cis: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    free((*wp).searchstr as *mut libc::c_void);
    (*wp).searchstr = xstrdup((*data).searchstr);
    fx = (*data).cx;
    fy =
        (*(*(*data).backing).grid).hsize.wrapping_sub((*data).oy).wrapping_add((*data).cy);
    screen_init(&mut ss as *mut screen,
                screen_write_strlen(b"%s\x00" as *const u8 as
                                        *const libc::c_char,
                                    (*data).searchstr) as u_int,
                1i32 as u_int, 0i32 as u_int);
    screen_write_start(&mut ctx as *mut screen_write_ctx,
                       0 as *mut window_pane, &mut ss as *mut screen);
    screen_write_nputs(&mut ctx as *mut screen_write_ctx,
                       1i32.wrapping_neg() as ssize_t,
                       &grid_default_cell as *const grid_cell,
                       b"%s\x00" as *const u8 as *const libc::c_char,
                       (*data).searchstr);
    screen_write_stop(&mut ctx as *mut screen_write_ctx);
    if 0 != direction {
        window_copy_move_right(s, &mut fx as *mut u_int,
                               &mut fy as *mut u_int);
    } else {
        window_copy_move_left(s, &mut fx as *mut u_int,
                              &mut fy as *mut u_int);
    }
    window_copy_clear_selection(wp);
    wrapflag =
        options_get_number((*(*wp).window).options,
                           b"wrap-search\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    cis = window_copy_is_lowercase((*data).searchstr);
    if 0 != direction {
        endline =
            (*gd).hsize.wrapping_add((*gd).sy).wrapping_sub(1i32 as
                                                                libc::c_uint)
    } else { endline = 0i32 as u_int }
    found =
        window_copy_search_jump(wp, gd, ss.grid, fx, fy, endline, cis,
                                wrapflag, direction);
    if 0 != window_copy_search_marks(wp, &mut ss as *mut screen) {
        window_copy_redraw_screen(wp);
    }
    screen_free(&mut ss as *mut screen);
    return found;
}
unsafe extern "C" fn window_copy_search_marks(mut wp: *mut window_pane,
                                              mut ssp: *mut screen)
 -> libc::c_int {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = (*data).backing;
    let mut ss: screen =
        screen{title: 0 as *mut libc::c_char,
               titles: 0 as *mut screen_titles,
               grid: 0 as *mut grid,
               cx: 0,
               cy: 0,
               cstyle: 0,
               ccolour: 0 as *mut libc::c_char,
               rupper: 0,
               rlower: 0,
               mode: 0,
               tabs: 0 as *mut bitstr_t,
               sel:
                   screen_sel{flag: 0,
                              hidden: 0,
                              rectflag: 0,
                              lineflag: LINE_SEL_NONE,
                              modekeys: 0,
                              sx: 0,
                              sy: 0,
                              ex: 0,
                              ey: 0,
                              cell:
                                  grid_cell{flags: 0,
                                            attr: 0,
                                            fg: 0,
                                            bg: 0,
                                            data:
                                                utf8_data{data: [0; 9],
                                                          have: 0,
                                                          size: 0,
                                                          width: 0,},},},};
    let mut ctx: screen_write_ctx =
        screen_write_ctx{wp: 0 as *mut window_pane,
                         s: 0 as *mut screen,
                         item: 0 as *mut screen_write_collect_item,
                         list: 0 as *mut screen_write_collect_line,
                         scrolled: 0,
                         bg: 0,
                         cells: 0,
                         written: 0,
                         skipped: 0,};
    let mut gd: *mut grid = (*s).grid;
    let mut found: libc::c_int = 0;
    let mut cis: libc::c_int = 0;
    let mut which: libc::c_int = 1i32.wrapping_neg();
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut b: u_int = 0;
    let mut nfound: u_int = 0i32 as u_int;
    let mut width: u_int = 0;
    if ssp == 0 as *mut libc::c_void as *mut screen {
        width =
            screen_write_strlen(b"%s\x00" as *const u8 as *const libc::c_char,
                                (*data).searchstr) as u_int;
        screen_init(&mut ss as *mut screen, width, 1i32 as u_int,
                    0i32 as u_int);
        screen_write_start(&mut ctx as *mut screen_write_ctx,
                           0 as *mut window_pane, &mut ss as *mut screen);
        screen_write_nputs(&mut ctx as *mut screen_write_ctx,
                           1i32.wrapping_neg() as ssize_t,
                           &grid_default_cell as *const grid_cell,
                           b"%s\x00" as *const u8 as *const libc::c_char,
                           (*data).searchstr);
        screen_write_stop(&mut ctx as *mut screen_write_ctx);
        ssp = &mut ss as *mut screen
    } else { width = (*(*ssp).grid).sx }
    cis = window_copy_is_lowercase((*data).searchstr);
    free((*data).searchmark as *mut libc::c_void);
    (*data).searchmark =
        calloc(((*gd).hsize.wrapping_add((*gd).sy).wrapping_mul((*gd).sx).wrapping_add(7i32
                                                                                           as
                                                                                           libc::c_uint)
                    >> 3i32) as size_t,
               ::std::mem::size_of::<bitstr_t>() as libc::c_ulong) as
            *mut bitstr_t;
    py = 0i32 as u_int;
    while py < (*gd).hsize.wrapping_add((*gd).sy) {
        px = 0i32 as u_int;
        loop  {
            found =
                window_copy_search_lr(gd, (*ssp).grid, &mut px as *mut u_int,
                                      py, px, (*gd).sx, cis);
            if 0 == found { break ; }
            nfound = nfound.wrapping_add(1);
            if px == (*data).cx &&
                   py ==
                       (*gd).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy)
               {
                which = nfound as libc::c_int
            }
            b = py.wrapping_mul((*gd).sx).wrapping_add(px);
            loop  {
                let mut _name: *mut bitstr_t = (*data).searchmark;
                let mut _start: libc::c_int = b as libc::c_int;
                let mut _stop: libc::c_int =
                    b.wrapping_add(width).wrapping_sub(1i32 as libc::c_uint)
                        as libc::c_int;
                while _start <= _stop {
                    let ref mut fresh2 =
                        *_name.offset((_start >> 3i32) as isize);
                    *fresh2 =
                        (*fresh2 as libc::c_int | 1i32 << (_start & 7i32)) as
                            bitstr_t;
                    _start += 1
                }
                if !(0 != 0i32) { break ; }
            }
            px = px.wrapping_add(1)
        }
        py = py.wrapping_add(1)
    }
    if which != 1i32.wrapping_neg() {
        (*data).searchthis =
            (1i32 as
                 libc::c_uint).wrapping_add(nfound).wrapping_sub(which as
                                                                     libc::c_uint)
                as libc::c_int
    } else { (*data).searchthis = 1i32.wrapping_neg() }
    (*data).searchcount = nfound;
    if ssp == &mut ss as *mut screen { screen_free(&mut ss as *mut screen); }
    return nfound as libc::c_int;
}
unsafe extern "C" fn window_copy_search_lr(mut gd: *mut grid,
                                           mut sgd: *mut grid,
                                           mut ppx: *mut u_int, mut py: u_int,
                                           mut first: u_int, mut last: u_int,
                                           mut cis: libc::c_int)
 -> libc::c_int {
    let mut ax: u_int = 0;
    let mut bx: u_int = 0;
    let mut px: u_int = 0;
    let mut matched: libc::c_int = 0;
    ax = first;
    while ax < last {
        if ax.wrapping_add((*sgd).sx) > (*gd).sx { break ; }
        bx = 0i32 as u_int;
        while bx < (*sgd).sx {
            px = ax.wrapping_add(bx);
            matched = window_copy_search_compare(gd, px, py, sgd, bx, cis);
            if 0 == matched { break ; }
            bx = bx.wrapping_add(1)
        }
        if bx == (*sgd).sx {
            *ppx = ax;
            return 1i32
        } else { ax = ax.wrapping_add(1) }
    }
    return 0i32;
}
unsafe extern "C" fn window_copy_search_compare(mut gd: *mut grid,
                                                mut px: u_int, mut py: u_int,
                                                mut sgd: *mut grid,
                                                mut spx: u_int,
                                                mut cis: libc::c_int)
 -> libc::c_int {
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut sgc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut ud: *const utf8_data = 0 as *const utf8_data;
    let mut sud: *const utf8_data = 0 as *const utf8_data;
    grid_get_cell(gd, px, py, &mut gc as *mut grid_cell);
    ud = &mut gc.data as *mut utf8_data;
    grid_get_cell(sgd, spx, 0i32 as u_int, &mut sgc as *mut grid_cell);
    sud = &mut sgc.data as *mut utf8_data;
    if (*ud).size as libc::c_int != (*sud).size as libc::c_int ||
           (*ud).width as libc::c_int != (*sud).width as libc::c_int {
        return 0i32
    } else if 0 != cis && (*ud).size as libc::c_int == 1i32 {
        return (tolower((*ud).data[0usize] as libc::c_int) ==
                    (*sud).data[0usize] as libc::c_int) as libc::c_int
    } else {
        return (memcmp((*ud).data.as_ptr() as *const libc::c_void,
                       (*sud).data.as_ptr() as *const libc::c_void,
                       (*ud).size as libc::c_ulong) == 0i32) as libc::c_int
    };
}
unsafe extern "C" fn window_copy_is_lowercase(mut ptr: *const libc::c_char)
 -> libc::c_int {
    loop  {
        if *ptr as libc::c_int != 0 {
            if *ptr as libc::c_int != tolower(*ptr as u_char as libc::c_int) {
                return 0i32
            } else { ptr = ptr.offset(1isize) }
        } else { return 1i32 }
    };
}
unsafe extern "C" fn window_copy_search_jump(mut wp: *mut window_pane,
                                             mut gd: *mut grid,
                                             mut sgd: *mut grid,
                                             mut fx: u_int, mut fy: u_int,
                                             mut endline: u_int,
                                             mut cis: libc::c_int,
                                             mut wrap: libc::c_int,
                                             mut direction: libc::c_int)
 -> libc::c_int {
    let mut i: u_int = 0;
    let mut px: u_int = 0;
    let mut found: libc::c_int = 0;
    found = 0i32;
    if 0 != direction {
        i = fy;
        while i <= endline {
            found =
                window_copy_search_lr(gd, sgd, &mut px as *mut u_int, i, fx,
                                      (*gd).sx, cis);
            if 0 != found { break ; }
            fx = 0i32 as u_int;
            i = i.wrapping_add(1)
        }
    } else {
        i = fy.wrapping_add(1i32 as libc::c_uint);
        while endline < i {
            found =
                window_copy_search_rl(gd, sgd, &mut px as *mut u_int,
                                      i.wrapping_sub(1i32 as libc::c_uint),
                                      0i32 as u_int, fx, cis);
            if 0 != found {
                i = i.wrapping_sub(1);
                break ;
            } else { fx = (*gd).sx; i = i.wrapping_sub(1) }
        }
    }
    if 0 != found {
        window_copy_scroll_to(wp, px, i);
        return 1i32
    } else if 0 != wrap {
        return window_copy_search_jump(wp, gd, sgd,
                                       if 0 != direction {
                                           0i32 as libc::c_uint
                                       } else {
                                           (*gd).sx.wrapping_sub(1i32 as
                                                                     libc::c_uint)
                                       },
                                       if 0 != direction {
                                           0i32 as libc::c_uint
                                       } else {
                                           (*gd).hsize.wrapping_add((*gd).sy).wrapping_sub(1i32
                                                                                               as
                                                                                               libc::c_uint)
                                       }, fy, cis, 0i32, direction)
    } else { return 0i32 };
}
unsafe extern "C" fn window_copy_scroll_to(mut wp: *mut window_pane,
                                           mut px: u_int, mut py: u_int)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut gd: *mut grid = (*(*data).backing).grid;
    let mut offset: u_int = 0;
    let mut gap: u_int = 0;
    (*data).cx = px;
    if py >= (*gd).hsize.wrapping_sub((*data).oy) &&
           py < (*gd).hsize.wrapping_sub((*data).oy).wrapping_add((*gd).sy) {
        (*data).cy = py.wrapping_sub((*gd).hsize.wrapping_sub((*data).oy))
    } else {
        gap = (*gd).sy.wrapping_div(4i32 as libc::c_uint);
        if py < (*gd).sy {
            offset = 0i32 as u_int;
            (*data).cy = py
        } else if py > (*gd).hsize.wrapping_add((*gd).sy).wrapping_sub(gap) {
            offset = (*gd).hsize;
            (*data).cy = py.wrapping_sub((*gd).hsize)
        } else {
            offset = py.wrapping_add(gap).wrapping_sub((*gd).sy);
            (*data).cy = py.wrapping_sub(offset)
        }
        (*data).oy = (*gd).hsize.wrapping_sub(offset)
    }
    window_copy_update_selection(wp, 1i32);
    window_copy_redraw_screen(wp);
}
unsafe extern "C" fn window_copy_update_selection(mut wp: *mut window_pane,
                                                  mut may_redraw: libc::c_int)
 -> libc::c_int {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut oo: *mut options = (*(*wp).window).options;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut cy: u_int = 0;
    let mut endsx: u_int = 0;
    let mut endsy: u_int = 0;
    let mut startrelpos: libc::c_int = 0;
    let mut endrelpos: libc::c_int = 0;
    if 0 == (*s).sel.flag &&
           (*s).sel.lineflag as libc::c_uint ==
               LINE_SEL_NONE as libc::c_int as libc::c_uint {
        return 0i32
    } else {
        window_copy_synchronize_cursor(wp);
        sx = (*data).selx;
        sy = (*data).sely;
        startrelpos =
            window_copy_adjust_selection(wp, &mut sx as *mut u_int,
                                         &mut sy as *mut u_int);
        endsx = (*data).endselx;
        endsy = (*data).endsely;
        endrelpos =
            window_copy_adjust_selection(wp, &mut endsx as *mut u_int,
                                         &mut endsy as *mut u_int);
        if startrelpos == endrelpos &&
               startrelpos != WINDOW_COPY_REL_POS_ON_SCREEN as libc::c_int {
            screen_hide_selection(s);
            return 0i32
        } else {
            style_apply(&mut gc as *mut grid_cell, oo,
                        b"mode-style\x00" as *const u8 as
                            *const libc::c_char);
            gc.flags = (gc.flags as libc::c_int | 32i32) as u_char;
            screen_set_selection(s, sx, sy, endsx, endsy,
                                 (*data).rectflag as u_int,
                                 &mut gc as *mut grid_cell);
            if 0 != (*data).rectflag && 0 != may_redraw {
                cy = (*data).cy;
                if (*data).cursordrag as libc::c_uint ==
                       CURSORDRAG_ENDSEL as libc::c_int as libc::c_uint {
                    if sy < cy {
                        window_copy_redraw_lines(wp, sy,
                                                 cy.wrapping_sub(sy).wrapping_add(1i32
                                                                                      as
                                                                                      libc::c_uint));
                    } else {
                        window_copy_redraw_lines(wp, cy,
                                                 sy.wrapping_sub(cy).wrapping_add(1i32
                                                                                      as
                                                                                      libc::c_uint));
                    }
                } else if endsy < cy {
                    window_copy_redraw_lines(wp, endsy,
                                             cy.wrapping_sub(endsy).wrapping_add(1i32
                                                                                     as
                                                                                     libc::c_uint));
                } else {
                    window_copy_redraw_lines(wp, cy,
                                             endsy.wrapping_sub(cy).wrapping_add(1i32
                                                                                     as
                                                                                     libc::c_uint));
                }
            }
            return 1i32
        }
    };
}
unsafe extern "C" fn window_copy_adjust_selection(mut wp: *mut window_pane,
                                                  mut selx: *mut u_int,
                                                  mut sely: *mut u_int)
 -> libc::c_int {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut ty: u_int = 0;
    let mut relpos: libc::c_int = 0;
    sx = *selx;
    sy = *sely;
    ty = (*(*(*data).backing).grid).hsize.wrapping_sub((*data).oy);
    if sy < ty {
        relpos = WINDOW_COPY_REL_POS_ABOVE as libc::c_int;
        if 0 == (*data).rectflag { sx = 0i32 as u_int }
        sy = 0i32 as u_int
    } else if sy >
                  ty.wrapping_add((*(*s).grid).sy).wrapping_sub(1i32 as
                                                                    libc::c_uint)
     {
        relpos = WINDOW_COPY_REL_POS_BELOW as libc::c_int;
        if 0 == (*data).rectflag {
            sx = (*(*s).grid).sx.wrapping_sub(1i32 as libc::c_uint)
        }
        sy = (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint)
    } else {
        relpos = WINDOW_COPY_REL_POS_ON_SCREEN as libc::c_int;
        sy = (sy as libc::c_uint).wrapping_sub(ty) as u_int as u_int
    }
    *selx = sx;
    *sely = sy;
    return relpos;
}
unsafe extern "C" fn window_copy_synchronize_cursor(mut wp: *mut window_pane)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    xx = (*data).cx;
    yy =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    match (*data).cursordrag as libc::c_uint {
        1 => { (*data).endselx = xx; (*data).endsely = yy }
        2 => { (*data).selx = xx; (*data).sely = yy }
        0 | _ => { }
    };
}
unsafe extern "C" fn window_copy_search_rl(mut gd: *mut grid,
                                           mut sgd: *mut grid,
                                           mut ppx: *mut u_int, mut py: u_int,
                                           mut first: u_int, mut last: u_int,
                                           mut cis: libc::c_int)
 -> libc::c_int {
    let mut ax: u_int = 0;
    let mut bx: u_int = 0;
    let mut px: u_int = 0;
    let mut matched: libc::c_int = 0;
    ax = last.wrapping_add(1i32 as libc::c_uint);
    loop  {
        if ax > first {
            if !((*gd).sx.wrapping_sub(ax.wrapping_sub(1i32 as libc::c_uint))
                     < (*sgd).sx) {
                bx = 0i32 as u_int;
                while bx < (*sgd).sx {
                    px =
                        ax.wrapping_sub(1i32 as
                                            libc::c_uint).wrapping_add(bx);
                    matched =
                        window_copy_search_compare(gd, px, py, sgd, bx, cis);
                    if 0 == matched { break ; }
                    bx = bx.wrapping_add(1)
                }
                if bx == (*sgd).sx {
                    *ppx = ax.wrapping_sub(1i32 as libc::c_uint);
                    return 1i32
                }
            }
            ax = ax.wrapping_sub(1)
        } else { return 0i32 }
    };
}
unsafe extern "C" fn window_copy_clear_selection(mut wp: *mut window_pane)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    screen_clear_selection(&mut (*data).screen as *mut screen);
    (*data).cursordrag = CURSORDRAG_NONE;
    py =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    px = window_copy_find_length(wp, py);
    if (*data).cx > px { window_copy_update_cursor(wp, px, (*data).cy); };
}
unsafe extern "C" fn window_copy_update_cursor(mut wp: *mut window_pane,
                                               mut cx: u_int, mut cy: u_int)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut ctx: screen_write_ctx =
        screen_write_ctx{wp: 0 as *mut window_pane,
                         s: 0 as *mut screen,
                         item: 0 as *mut screen_write_collect_item,
                         list: 0 as *mut screen_write_collect_line,
                         scrolled: 0,
                         bg: 0,
                         cells: 0,
                         written: 0,
                         skipped: 0,};
    let mut old_cx: u_int = 0;
    let mut old_cy: u_int = 0;
    old_cx = (*data).cx;
    old_cy = (*data).cy;
    (*data).cx = cx;
    (*data).cy = cy;
    if old_cx == (*(*s).grid).sx {
        window_copy_redraw_lines(wp, old_cy, 1i32 as u_int);
    }
    if (*data).cx == (*(*s).grid).sx {
        window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
    } else {
        screen_write_start(&mut ctx as *mut screen_write_ctx, wp,
                           0 as *mut screen);
        screen_write_cursormove(&mut ctx as *mut screen_write_ctx, (*data).cx,
                                (*data).cy);
        screen_write_stop(&mut ctx as *mut screen_write_ctx);
    };
}
unsafe extern "C" fn window_copy_find_length(mut wp: *mut window_pane,
                                             mut py: u_int) -> u_int {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = (*data).backing;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut px: u_int = 0;
    px = (*(*(*s).grid).linedata.offset(py as isize)).cellsize;
    if px > (*(*s).grid).sx { px = (*(*s).grid).sx }
    while px > 0i32 as libc::c_uint {
        grid_get_cell((*s).grid, px.wrapping_sub(1i32 as libc::c_uint), py,
                      &mut gc as *mut grid_cell);
        if gc.data.size as libc::c_int != 1i32 ||
               *gc.data.data.as_mut_ptr() as libc::c_int != 32 {
            break ;
        }
        px = px.wrapping_sub(1)
    }
    return px;
}
unsafe extern "C" fn window_copy_move_left(mut s: *mut screen,
                                           mut fx: *mut u_int,
                                           mut fy: *mut u_int) -> () {
    if *fx == 0i32 as libc::c_uint {
        if *fy == 0i32 as libc::c_uint {
            return
        } else {
            *fx = (*(*s).grid).sx.wrapping_sub(1i32 as libc::c_uint);
            *fy = (*fy).wrapping_sub(1i32 as libc::c_uint)
        }
    } else { *fx = (*fx).wrapping_sub(1i32 as libc::c_uint) };
}
unsafe extern "C" fn window_copy_move_right(mut s: *mut screen,
                                            mut fx: *mut u_int,
                                            mut fy: *mut u_int) -> () {
    if *fx == (*(*s).grid).sx.wrapping_sub(1i32 as libc::c_uint) {
        if *fy == (*(*s).grid).hsize.wrapping_add((*(*s).grid).sy) {
            return
        } else {
            *fx = 0i32 as u_int;
            *fy = (*fy).wrapping_add(1i32 as libc::c_uint)
        }
    } else { *fx = (*fx).wrapping_add(1i32 as libc::c_uint) };
}
unsafe extern "C" fn window_copy_search_down(mut wp: *mut window_pane)
 -> libc::c_int {
    return window_copy_search(wp, 1i32);
}
unsafe extern "C" fn window_copy_cursor_jump_to(mut wp: *mut window_pane)
 -> () {
    let mut current_block: u64;
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    px = (*data).cx.wrapping_add(2i32 as libc::c_uint);
    py =
        (*(*back_s).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    xx = window_copy_find_length(wp, py);
    loop  {
        if px < xx {
            grid_get_cell((*back_s).grid, px, py, &mut gc as *mut grid_cell);
            if 0 == gc.flags as libc::c_int & 4i32 &&
                   gc.data.size as libc::c_int == 1i32 &&
                   *gc.data.data.as_mut_ptr() as libc::c_int ==
                       (*data).jumpchar as libc::c_int {
                window_copy_update_cursor(wp,
                                          px.wrapping_sub(1i32 as
                                                              libc::c_uint),
                                          (*data).cy);
                if 0 != window_copy_update_selection(wp, 1i32) {
                    current_block = 17179679302217393232;
                    break ;
                } else { current_block = 17778012151635330486; break ; }
            } else { px = px.wrapping_add(1) }
        } else { return; }
    }
    match current_block {
        17179679302217393232 => {
            window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
        }
        _ => { }
    };
}
unsafe extern "C" fn window_copy_cursor_jump_to_back(mut wp: *mut window_pane)
 -> () {
    let mut current_block: u64;
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    px = (*data).cx;
    py =
        (*(*back_s).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    if px > 0i32 as libc::c_uint { px = px.wrapping_sub(1) }
    if px > 0i32 as libc::c_uint { px = px.wrapping_sub(1) }
    loop  {
        grid_get_cell((*back_s).grid, px, py, &mut gc as *mut grid_cell);
        if 0 == gc.flags as libc::c_int & 4i32 &&
               gc.data.size as libc::c_int == 1i32 &&
               *gc.data.data.as_mut_ptr() as libc::c_int ==
                   (*data).jumpchar as libc::c_int {
            window_copy_update_cursor(wp,
                                      px.wrapping_add(1i32 as libc::c_uint),
                                      (*data).cy);
            if 0 != window_copy_update_selection(wp, 1i32) {
                current_block = 15427931788582360902;
                break ;
            } else { current_block = 15240798224410183470; break ; }
        } else if px == 0i32 as libc::c_uint {
            return;
        } else { px = px.wrapping_sub(1) }
    }
    match current_block {
        15427931788582360902 => {
            window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
        }
        _ => { }
    };
}
unsafe extern "C" fn window_copy_cursor_jump(mut wp: *mut window_pane) -> () {
    let mut current_block: u64;
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    px = (*data).cx.wrapping_add(1i32 as libc::c_uint);
    py =
        (*(*back_s).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    xx = window_copy_find_length(wp, py);
    loop  {
        if px < xx {
            grid_get_cell((*back_s).grid, px, py, &mut gc as *mut grid_cell);
            if 0 == gc.flags as libc::c_int & 4i32 &&
                   gc.data.size as libc::c_int == 1i32 &&
                   *gc.data.data.as_mut_ptr() as libc::c_int ==
                       (*data).jumpchar as libc::c_int {
                window_copy_update_cursor(wp, px, (*data).cy);
                if 0 != window_copy_update_selection(wp, 1i32) {
                    current_block = 17179679302217393232;
                    break ;
                } else { current_block = 17778012151635330486; break ; }
            } else { px = px.wrapping_add(1) }
        } else { return; }
    }
    match current_block {
        17179679302217393232 => {
            window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
        }
        _ => { }
    };
}
unsafe extern "C" fn window_copy_cursor_jump_back(mut wp: *mut window_pane)
 -> () {
    let mut current_block: u64;
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    px = (*data).cx;
    py =
        (*(*back_s).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    if px > 0i32 as libc::c_uint { px = px.wrapping_sub(1) }
    loop  {
        grid_get_cell((*back_s).grid, px, py, &mut gc as *mut grid_cell);
        if 0 == gc.flags as libc::c_int & 4i32 &&
               gc.data.size as libc::c_int == 1i32 &&
               *gc.data.data.as_mut_ptr() as libc::c_int ==
                   (*data).jumpchar as libc::c_int {
            window_copy_update_cursor(wp, px, (*data).cy);
            if 0 != window_copy_update_selection(wp, 1i32) {
                current_block = 820271813250567934;
                break ;
            } else { current_block = 7502529970979898288; break ; }
        } else if px == 0i32 as libc::c_uint {
            return;
        } else { px = px.wrapping_sub(1) }
    }
    match current_block {
        820271813250567934 => {
            window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
        }
        _ => { }
    };
}
unsafe extern "C" fn window_copy_goto_line(mut wp: *mut window_pane,
                                           mut linestr: *const libc::c_char)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut lineno: u_int = 0;
    lineno =
        strtonum(linestr, 0i32 as libc::c_longlong,
                 (*(*(*data).backing).grid).hsize as libc::c_longlong,
                 &mut errstr as *mut *const libc::c_char) as u_int;
    if errstr != 0 as *mut libc::c_void as *const libc::c_char {
        return
    } else {
        (*data).oy = lineno;
        window_copy_update_selection(wp, 1i32);
        window_copy_redraw_screen(wp);
        return;
    };
}
unsafe extern "C" fn window_copy_copy_pipe(mut wp: *mut window_pane,
                                           mut s: *mut session,
                                           mut bufname: *const libc::c_char,
                                           mut arg: *const libc::c_char)
 -> () {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut len: size_t = 0;
    let mut job: *mut job = 0 as *mut job;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    buf = window_copy_get_selection(wp, &mut len as *mut size_t);
    if buf == 0 as *mut libc::c_void {
        return
    } else {
        expanded =
            format_single(0 as *mut cmdq_item, arg, 0 as *mut client, s,
                          0 as *mut winlink, wp);
        job =
            job_run(expanded, s, 0 as *const libc::c_char, None, None, None,
                    0 as *mut libc::c_void, 1i32);
        bufferevent_write((*job).event, buf, len);
        free(expanded as *mut libc::c_void);
        window_copy_copy_buffer(wp, bufname, buf, len);
        return;
    };
}
unsafe extern "C" fn window_copy_copy_buffer(mut wp: *mut window_pane,
                                             mut bufname: *const libc::c_char,
                                             mut buf: *mut libc::c_void,
                                             mut len: size_t) -> () {
    let mut ctx: screen_write_ctx =
        screen_write_ctx{wp: 0 as *mut window_pane,
                         s: 0 as *mut screen,
                         item: 0 as *mut screen_write_collect_item,
                         list: 0 as *mut screen_write_collect_line,
                         scrolled: 0,
                         bg: 0,
                         cells: 0,
                         written: 0,
                         skipped: 0,};
    if options_get_number(global_options,
                          b"set-clipboard\x00" as *const u8 as
                              *const libc::c_char) != 0i32 as libc::c_longlong
       {
        screen_write_start(&mut ctx as *mut screen_write_ctx, wp,
                           0 as *mut screen);
        screen_write_setselection(&mut ctx as *mut screen_write_ctx,
                                  buf as *mut u_char, len as u_int);
        screen_write_stop(&mut ctx as *mut screen_write_ctx);
        notify_pane(b"pane-set-clipboard\x00" as *const u8 as
                        *const libc::c_char, wp);
    }
    if paste_set(buf as *mut libc::c_char, len, bufname,
                 0 as *mut *mut libc::c_char) != 0i32 {
        free(buf);
    };
}
unsafe extern "C" fn window_copy_get_selection(mut wp: *mut window_pane,
                                               mut len: *mut size_t)
 -> *mut libc::c_void {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut off: size_t = 0;
    let mut i: u_int = 0;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut ex: u_int = 0;
    let mut ey: u_int = 0;
    let mut ey_last: u_int = 0;
    let mut firstsx: u_int = 0;
    let mut lastex: u_int = 0;
    let mut restex: u_int = 0;
    let mut restsx: u_int = 0;
    let mut selx: u_int = 0;
    let mut keys: libc::c_int = 0;
    if 0 == (*s).sel.flag &&
           (*s).sel.lineflag as libc::c_uint ==
               LINE_SEL_NONE as libc::c_int as libc::c_uint {
        return 0 as *mut libc::c_void
    } else {
        buf = xmalloc(1i32 as size_t) as *mut libc::c_char;
        off = 0i32 as size_t;
        *buf = 0 as libc::c_char;
        xx = (*data).endselx;
        yy = (*data).endsely;
        if yy < (*data).sely || yy == (*data).sely && xx < (*data).selx {
            sx = xx;
            sy = yy;
            ex = (*data).selx;
            ey = (*data).sely
        } else { sx = (*data).selx; sy = (*data).sely; ex = xx; ey = yy }
        ey_last = window_copy_find_length(wp, ey);
        if ex > ey_last { ex = ey_last }
        xx = (*(*s).grid).sx;
        keys =
            options_get_number((*(*wp).window).options,
                               b"mode-keys\x00" as *const u8 as
                                   *const libc::c_char) as libc::c_int;
        if 0 != (*data).rectflag {
            if (*data).cursordrag as libc::c_uint ==
                   CURSORDRAG_ENDSEL as libc::c_int as libc::c_uint {
                selx = (*data).selx
            } else { selx = (*data).endselx }
            if selx < (*data).cx {
                if keys == 0i32 {
                    lastex = (*data).cx;
                    restex = (*data).cx
                } else {
                    lastex = (*data).cx.wrapping_add(1i32 as libc::c_uint);
                    restex = (*data).cx.wrapping_add(1i32 as libc::c_uint)
                }
                firstsx = selx;
                restsx = selx
            } else {
                lastex = selx.wrapping_add(1i32 as libc::c_uint);
                restex = selx.wrapping_add(1i32 as libc::c_uint);
                firstsx = (*data).cx;
                restsx = (*data).cx
            }
        } else {
            if keys == 0i32 {
                lastex = ex
            } else { lastex = ex.wrapping_add(1i32 as libc::c_uint) }
            restex = xx;
            firstsx = sx;
            restsx = 0i32 as u_int
        }
        i = sy;
        while i <= ey {
            window_copy_copy_line(wp, &mut buf as *mut *mut libc::c_char,
                                  &mut off as *mut size_t, i,
                                  if i == sy { firstsx } else { restsx },
                                  if i == ey { lastex } else { restex });
            i = i.wrapping_add(1)
        }
        if off == 0i32 as libc::c_ulong {
            free(buf as *mut libc::c_void);
            return 0 as *mut libc::c_void
        } else {
            if keys == 0i32 || lastex <= ey_last {
                off =
                    (off as libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong)
                        as size_t as size_t
            }
            *len = off;
            return buf as *mut libc::c_void
        }
    };
}
unsafe extern "C" fn window_copy_copy_line(mut wp: *mut window_pane,
                                           mut buf: *mut *mut libc::c_char,
                                           mut off: *mut size_t,
                                           mut sy: u_int, mut sx: u_int,
                                           mut ex: u_int) -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut gd: *mut grid = (*(*data).backing).grid;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut ud: utf8_data =
        utf8_data{data: [0; 9], have: 0, size: 0, width: 0,};
    let mut i: u_int = 0;
    let mut xx: u_int = 0;
    let mut wrapped: u_int = 0i32 as u_int;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if sx > ex {
        return
    } else {
        gl = &mut *(*gd).linedata.offset(sy as isize) as *mut grid_line;
        if 0 != (*gl).flags & 1i32 && (*gl).cellsize <= (*gd).sx {
            wrapped = 1i32 as u_int
        }
        if 0 != wrapped {
            xx = (*gl).cellsize
        } else { xx = window_copy_find_length(wp, sy) }
        if ex > xx { ex = xx }
        if sx > xx { sx = xx }
        if sx < ex {
            i = sx;
            while i < ex {
                grid_get_cell(gd, i, sy, &mut gc as *mut grid_cell);
                if !(0 != gc.flags as libc::c_int & 4i32) {
                    utf8_copy(&mut ud as *mut utf8_data,
                              &mut gc.data as *mut utf8_data);
                    if ud.size as libc::c_int == 1i32 &&
                           0 != gc.attr as libc::c_int & 128i32 {
                        s = tty_acs_get(0 as *mut tty, ud.data[0usize]);
                        if s != 0 as *mut libc::c_void as *const libc::c_char
                               &&
                               strlen(s) <=
                                   ::std::mem::size_of::<[u_char; 9]>() as
                                       libc::c_ulong {
                            ud.size = strlen(s) as u_char;
                            memcpy(ud.data.as_mut_ptr() as *mut libc::c_void,
                                   s as *const libc::c_void,
                                   ud.size as libc::c_ulong);
                        }
                    }
                    *buf =
                        xrealloc(*buf as *mut libc::c_void,
                                 (*off).wrapping_add(ud.size as
                                                         libc::c_ulong)) as
                            *mut libc::c_char;
                    memcpy((*buf).offset(*off as isize) as *mut libc::c_void,
                           ud.data.as_mut_ptr() as *const libc::c_void,
                           ud.size as libc::c_ulong);
                    *off =
                        (*off as
                             libc::c_ulong).wrapping_add(ud.size as
                                                             libc::c_ulong) as
                            size_t as size_t
                }
                i = i.wrapping_add(1)
            }
        }
        if 0 == wrapped || ex != xx {
            *buf =
                xrealloc(*buf as *mut libc::c_void,
                         (*off).wrapping_add(1i32 as libc::c_ulong)) as
                    *mut libc::c_char;
            let fresh3 = *off;
            *off = (*off).wrapping_add(1);
            *(*buf).offset(fresh3 as isize) = 10 as libc::c_char
        }
        return;
    };
}
unsafe extern "C" fn window_copy_cursor_start_of_line(mut wp:
                                                          *mut window_pane)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut gd: *mut grid = (*back_s).grid;
    let mut py: u_int = 0;
    if (*data).cx == 0i32 as libc::c_uint &&
           (*s).sel.lineflag as libc::c_uint ==
               LINE_SEL_NONE as libc::c_int as libc::c_uint {
        py =
            (*(*back_s).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
        while py > 0i32 as libc::c_uint &&
                  0 !=
                      (*(*gd).linedata.offset(py.wrapping_sub(1i32 as
                                                                  libc::c_uint)
                                                  as isize)).flags & 1i32 {
            window_copy_cursor_up(wp, 0i32);
            py =
                (*(*back_s).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy)
        }
    }
    window_copy_update_cursor(wp, 0i32 as u_int, (*data).cy);
    if 0 != window_copy_update_selection(wp, 1i32) {
        window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
    };
}
unsafe extern "C" fn window_copy_cursor_up(mut wp: *mut window_pane,
                                           mut scroll_only: libc::c_int)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    oy =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    ox = window_copy_find_length(wp, oy);
    if (*data).cx != ox { (*data).lastcx = (*data).cx; (*data).lastsx = ox }
    if (*s).sel.lineflag as libc::c_uint ==
           LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint &&
           oy == (*data).sely {
        window_copy_other_end(wp);
    }
    (*data).cx = (*data).lastcx;
    if 0 != scroll_only || (*data).cy == 0i32 as libc::c_uint {
        window_copy_scroll_down(wp, 1i32 as u_int);
        if 0 != scroll_only {
            if (*data).cy ==
                   (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint) {
                window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
            } else {
                window_copy_redraw_lines(wp, (*data).cy, 2i32 as u_int);
            }
        }
    } else {
        window_copy_update_cursor(wp, (*data).cx,
                                  (*data).cy.wrapping_sub(1i32 as
                                                              libc::c_uint));
        if 0 != window_copy_update_selection(wp, 1i32) {
            if (*data).cy ==
                   (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint) {
                window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
            } else {
                window_copy_redraw_lines(wp, (*data).cy, 2i32 as u_int);
            }
        }
    }
    if 0 == (*data).screen.sel.flag || 0 == (*data).rectflag {
        py =
            (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
        px = window_copy_find_length(wp, py);
        if (*data).cx >= (*data).lastsx && (*data).cx != px || (*data).cx > px
           {
            window_copy_cursor_end_of_line(wp);
        }
    }
    if (*s).sel.lineflag as libc::c_uint ==
           LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint {
        window_copy_cursor_end_of_line(wp);
    } else if (*s).sel.lineflag as libc::c_uint ==
                  LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint {
        window_copy_cursor_start_of_line(wp);
    };
}
unsafe extern "C" fn window_copy_cursor_end_of_line(mut wp: *mut window_pane)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut gd: *mut grid = (*back_s).grid;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    py =
        (*(*back_s).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    px = window_copy_find_length(wp, py);
    if (*data).cx == px &&
           (*s).sel.lineflag as libc::c_uint ==
               LINE_SEL_NONE as libc::c_int as libc::c_uint {
        if 0 != (*data).screen.sel.flag && 0 != (*data).rectflag {
            px = (*(*back_s).grid).sx
        }
        if 0 != (*(*gd).linedata.offset(py as isize)).flags & 1i32 {
            while py < (*gd).sy.wrapping_add((*gd).hsize) &&
                      0 != (*(*gd).linedata.offset(py as isize)).flags & 1i32
                  {
                window_copy_cursor_down(wp, 0i32);
                py =
                    (*(*back_s).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy)
            }
            px = window_copy_find_length(wp, py)
        }
    }
    window_copy_update_cursor(wp, px, (*data).cy);
    if 0 != window_copy_update_selection(wp, 1i32) {
        window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
    };
}
unsafe extern "C" fn window_copy_cursor_down(mut wp: *mut window_pane,
                                             mut scroll_only: libc::c_int)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    oy =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    ox = window_copy_find_length(wp, oy);
    if (*data).cx != ox { (*data).lastcx = (*data).cx; (*data).lastsx = ox }
    if (*s).sel.lineflag as libc::c_uint ==
           LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint &&
           oy == (*data).endsely {
        window_copy_other_end(wp);
    }
    (*data).cx = (*data).lastcx;
    if 0 != scroll_only ||
           (*data).cy == (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint) {
        window_copy_scroll_up(wp, 1i32 as u_int);
        if 0 != scroll_only && (*data).cy > 0i32 as libc::c_uint {
            window_copy_redraw_lines(wp,
                                     (*data).cy.wrapping_sub(1i32 as
                                                                 libc::c_uint),
                                     2i32 as u_int);
        }
    } else {
        window_copy_update_cursor(wp, (*data).cx,
                                  (*data).cy.wrapping_add(1i32 as
                                                              libc::c_uint));
        if 0 != window_copy_update_selection(wp, 1i32) {
            window_copy_redraw_lines(wp,
                                     (*data).cy.wrapping_sub(1i32 as
                                                                 libc::c_uint),
                                     2i32 as u_int);
        }
    }
    if 0 == (*data).screen.sel.flag || 0 == (*data).rectflag {
        py =
            (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
        px = window_copy_find_length(wp, py);
        if (*data).cx >= (*data).lastsx && (*data).cx != px || (*data).cx > px
           {
            window_copy_cursor_end_of_line(wp);
        }
    }
    if (*s).sel.lineflag as libc::c_uint ==
           LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint {
        window_copy_cursor_end_of_line(wp);
    } else if (*s).sel.lineflag as libc::c_uint ==
                  LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint {
        window_copy_cursor_start_of_line(wp);
    };
}
unsafe extern "C" fn window_copy_scroll_up(mut wp: *mut window_pane,
                                           mut ny: u_int) -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut ctx: screen_write_ctx =
        screen_write_ctx{wp: 0 as *mut window_pane,
                         s: 0 as *mut screen,
                         item: 0 as *mut screen_write_collect_item,
                         list: 0 as *mut screen_write_collect_line,
                         scrolled: 0,
                         bg: 0,
                         cells: 0,
                         written: 0,
                         skipped: 0,};
    if (*data).oy < ny { ny = (*data).oy }
    if ny == 0i32 as libc::c_uint {
        return
    } else {
        (*data).oy =
            ((*data).oy as libc::c_uint).wrapping_sub(ny) as u_int as u_int;
        window_copy_update_selection(wp, 0i32);
        screen_write_start(&mut ctx as *mut screen_write_ctx, wp,
                           0 as *mut screen);
        screen_write_cursormove(&mut ctx as *mut screen_write_ctx,
                                0i32 as u_int, 0i32 as u_int);
        screen_write_deleteline(&mut ctx as *mut screen_write_ctx, ny,
                                8i32 as u_int);
        window_copy_write_lines(wp, &mut ctx as *mut screen_write_ctx,
                                (*(*s).grid).sy.wrapping_sub(ny), ny);
        window_copy_write_line(wp, &mut ctx as *mut screen_write_ctx,
                               0i32 as u_int);
        if (*(*s).grid).sy > 1i32 as libc::c_uint {
            window_copy_write_line(wp, &mut ctx as *mut screen_write_ctx,
                                   1i32 as u_int);
        }
        if (*(*s).grid).sy > 3i32 as libc::c_uint {
            window_copy_write_line(wp, &mut ctx as *mut screen_write_ctx,
                                   (*(*s).grid).sy.wrapping_sub(2i32 as
                                                                    libc::c_uint));
        }
        if 0 != (*s).sel.flag && (*(*s).grid).sy > ny {
            window_copy_write_line(wp, &mut ctx as *mut screen_write_ctx,
                                   (*(*s).grid).sy.wrapping_sub(ny).wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_uint));
        }
        screen_write_cursormove(&mut ctx as *mut screen_write_ctx, (*data).cx,
                                (*data).cy);
        screen_write_stop(&mut ctx as *mut screen_write_ctx);
        return;
    };
}
unsafe extern "C" fn window_copy_write_lines(mut wp: *mut window_pane,
                                             mut ctx: *mut screen_write_ctx,
                                             mut py: u_int, mut ny: u_int)
 -> () {
    let mut yy: u_int = 0;
    yy = py;
    while yy < py.wrapping_add(ny) {
        window_copy_write_line(wp, ctx, py);
        yy = yy.wrapping_add(1)
    };
}
unsafe extern "C" fn window_copy_other_end(mut wp: *mut window_pane) -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut selx: u_int = 0;
    let mut sely: u_int = 0;
    let mut cy: u_int = 0;
    let mut yy: u_int = 0;
    let mut hsize: u_int = 0;
    if 0 == (*s).sel.flag &&
           (*s).sel.lineflag as libc::c_uint ==
               LINE_SEL_NONE as libc::c_int as libc::c_uint {
        return
    } else {
        if (*s).sel.lineflag as libc::c_uint ==
               LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint {
            (*s).sel.lineflag = LINE_SEL_RIGHT_LEFT
        } else if (*s).sel.lineflag as libc::c_uint ==
                      LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint {
            (*s).sel.lineflag = LINE_SEL_LEFT_RIGHT
        }
        match (*data).cursordrag as libc::c_uint {
            0 | 2 => { (*data).cursordrag = CURSORDRAG_ENDSEL }
            1 => { (*data).cursordrag = CURSORDRAG_SEL }
            _ => { }
        }
        selx = (*data).endselx;
        sely = (*data).endsely;
        if (*data).cursordrag as libc::c_uint ==
               CURSORDRAG_SEL as libc::c_int as libc::c_uint {
            selx = (*data).selx;
            sely = (*data).sely
        }
        cy = (*data).cy;
        yy =
            (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
        (*data).cx = selx;
        hsize = (*(*(*data).backing).grid).hsize;
        if sely < hsize.wrapping_sub((*data).oy) {
            (*data).oy = hsize.wrapping_sub(sely);
            (*data).cy = 0i32 as u_int
        } else if sely >
                      hsize.wrapping_sub((*data).oy).wrapping_add((*(*s).grid).sy)
         {
            (*data).oy =
                hsize.wrapping_sub(sely).wrapping_add((*(*s).grid).sy).wrapping_sub(1i32
                                                                                        as
                                                                                        libc::c_uint);
            (*data).cy = (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint)
        } else { (*data).cy = cy.wrapping_add(sely).wrapping_sub(yy) }
        window_copy_update_selection(wp, 1i32);
        window_copy_redraw_screen(wp);
        return;
    };
}
unsafe extern "C" fn window_copy_scroll_down(mut wp: *mut window_pane,
                                             mut ny: u_int) -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut ctx: screen_write_ctx =
        screen_write_ctx{wp: 0 as *mut window_pane,
                         s: 0 as *mut screen,
                         item: 0 as *mut screen_write_collect_item,
                         list: 0 as *mut screen_write_collect_line,
                         scrolled: 0,
                         bg: 0,
                         cells: 0,
                         written: 0,
                         skipped: 0,};
    if ny > (*(*(*data).backing).grid).hsize {
        return
    } else {
        if (*data).oy > (*(*(*data).backing).grid).hsize.wrapping_sub(ny) {
            ny = (*(*(*data).backing).grid).hsize.wrapping_sub((*data).oy)
        }
        if ny == 0i32 as libc::c_uint {
            return
        } else {
            (*data).oy =
                ((*data).oy as libc::c_uint).wrapping_add(ny) as u_int as
                    u_int;
            window_copy_update_selection(wp, 0i32);
            screen_write_start(&mut ctx as *mut screen_write_ctx, wp,
                               0 as *mut screen);
            screen_write_cursormove(&mut ctx as *mut screen_write_ctx,
                                    0i32 as u_int, 0i32 as u_int);
            screen_write_insertline(&mut ctx as *mut screen_write_ctx, ny,
                                    8i32 as u_int);
            window_copy_write_lines(wp, &mut ctx as *mut screen_write_ctx,
                                    0i32 as u_int, ny);
            if 0 != (*s).sel.flag && (*(*s).grid).sy > ny {
                window_copy_write_line(wp, &mut ctx as *mut screen_write_ctx,
                                       ny);
            } else if ny == 1i32 as libc::c_uint {
                window_copy_write_line(wp, &mut ctx as *mut screen_write_ctx,
                                       1i32 as u_int);
            }
            screen_write_cursormove(&mut ctx as *mut screen_write_ctx,
                                    (*data).cx, (*data).cy);
            screen_write_stop(&mut ctx as *mut screen_write_ctx);
            return;
        }
    };
}
unsafe extern "C" fn window_copy_cursor_next_word_end(mut wp:
                                                          *mut window_pane,
                                                      mut separators:
                                                          *const libc::c_char)
 -> () {
    let mut current_block: u64;
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut oo: *mut options = (*(*wp).window).options;
    let mut back_s: *mut screen = (*data).backing;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut keys: libc::c_int = 0;
    let mut expected: libc::c_int = 1i32;
    px = (*data).cx;
    py =
        (*(*back_s).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    xx = window_copy_find_length(wp, py);
    yy =
        (*(*back_s).grid).hsize.wrapping_add((*(*back_s).grid).sy).wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_uint);
    keys =
        options_get_number(oo,
                           b"mode-keys\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    if keys == 1i32 && 0 == window_copy_in_set(wp, px, py, separators) {
        px = px.wrapping_add(1)
    }
    loop  {
        if px > xx || window_copy_in_set(wp, px, py, separators) == expected {
            if px > xx {
                if py == yy {
                    return
                } else {
                    window_copy_cursor_down(wp, 0i32);
                    px = 0i32 as u_int;
                    py =
                        (*(*back_s).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
                    xx = window_copy_find_length(wp, py)
                }
            } else { px = px.wrapping_add(1) }
        } else {
            expected = (0 == expected) as libc::c_int;
            if expected == 0i32 { continue ; }
            if keys == 1i32 && px != 0i32 as libc::c_uint {
                current_block = 17216689946888361452;
                break ;
            } else { current_block = 14523784380283086299; break ; }
        }
    }
    match current_block {
        17216689946888361452 => { px = px.wrapping_sub(1) }
        _ => { }
    }
    window_copy_update_cursor(wp, px, (*data).cy);
    if 0 != window_copy_update_selection(wp, 1i32) {
        window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
    };
}
unsafe extern "C" fn window_copy_in_set(mut wp: *mut window_pane,
                                        mut px: u_int, mut py: u_int,
                                        mut set: *const libc::c_char)
 -> libc::c_int {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut ud: *const utf8_data = 0 as *const utf8_data;
    grid_get_cell((*(*data).backing).grid, px, py, &mut gc as *mut grid_cell);
    ud = &mut gc.data as *mut utf8_data;
    if (*ud).size as libc::c_int != 1i32 ||
           0 != gc.flags as libc::c_int & 4i32 {
        return 0i32
    } else if *(*ud).data.as_ptr() as libc::c_int == 0i32 ||
                  *(*ud).data.as_ptr() as libc::c_int == 127i32 {
        return 0i32
    } else {
        return (strchr(set, *(*ud).data.as_ptr() as libc::c_int) !=
                    0 as *mut libc::c_void as *mut libc::c_char) as
                   libc::c_int
    };
}
unsafe extern "C" fn window_copy_start_selection(mut wp: *mut window_pane)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    (*data).selx = (*data).cx;
    (*data).sely =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    (*data).endselx = (*data).selx;
    (*data).endsely = (*data).sely;
    (*data).cursordrag = CURSORDRAG_ENDSEL;
    (*s).sel.flag = 1i32;
    window_copy_update_selection(wp, 1i32);
}
unsafe extern "C" fn window_copy_cursor_previous_word(mut wp:
                                                          *mut window_pane,
                                                      mut separators:
                                                          *const libc::c_char)
 -> () {
    let mut current_block: u64;
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    px = (*data).cx;
    py =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    loop  {
        if px > 0i32 as libc::c_uint {
            px = px.wrapping_sub(1);
            if 0 == window_copy_in_set(wp, px, py, separators) {
                current_block = 6483416627284290920;
                break ;
            }
        } else {
            if (*data).cy == 0i32 as libc::c_uint &&
                   ((*(*(*data).backing).grid).hsize == 0i32 as libc::c_uint
                        ||
                        (*data).oy >=
                            (*(*(*data).backing).grid).hsize.wrapping_sub(1i32
                                                                              as
                                                                              libc::c_uint))
               {
                current_block = 10485851455608404399;
                break ;
            }
            window_copy_cursor_up(wp, 0i32);
            py =
                (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
            px = window_copy_find_length(wp, py)
        }
    }
    loop  {
        match current_block {
            6483416627284290920 => {
                if !(px > 0i32 as libc::c_uint &&
                         0 ==
                             window_copy_in_set(wp,
                                                px.wrapping_sub(1i32 as
                                                                    libc::c_uint),
                                                py, separators)) {
                    current_block = 10485851455608404399;
                    continue ;
                }
                px = px.wrapping_sub(1);
                current_block = 6483416627284290920;
            }
            _ => {
                window_copy_update_cursor(wp, px, (*data).cy);
                if !(0 != window_copy_update_selection(wp, 1i32)) { break ; }
                window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
                break ;
            }
        }
    };
}
unsafe extern "C" fn window_copy_rectangle_toggle(mut wp: *mut window_pane)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    (*data).rectflag = (0 == (*data).rectflag) as libc::c_int;
    py =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    px = window_copy_find_length(wp, py);
    if (*data).cx > px { window_copy_update_cursor(wp, px, (*data).cy); }
    window_copy_update_selection(wp, 1i32);
    window_copy_redraw_screen(wp);
}
unsafe extern "C" fn window_copy_previous_paragraph(mut wp: *mut window_pane)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut oy: u_int = 0;
    oy =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    while oy > 0i32 as libc::c_uint &&
              window_copy_find_length(wp, oy) == 0i32 as libc::c_uint {
        oy = oy.wrapping_sub(1)
    }
    while oy > 0i32 as libc::c_uint &&
              window_copy_find_length(wp, oy) > 0i32 as libc::c_uint {
        oy = oy.wrapping_sub(1)
    }
    window_copy_scroll_to(wp, 0i32 as u_int, oy);
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_pageup(mut wp: *mut window_pane,
                                            mut half_page: libc::c_int)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut n: u_int = 0;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    oy =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    ox = window_copy_find_length(wp, oy);
    if (*s).sel.lineflag as libc::c_uint ==
           LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint &&
           oy == (*data).sely {
        window_copy_other_end(wp);
    }
    if (*data).cx != ox { (*data).lastcx = (*data).cx; (*data).lastsx = ox }
    (*data).cx = (*data).lastcx;
    n = 1i32 as u_int;
    if (*(*s).grid).sy > 2i32 as libc::c_uint {
        if 0 != half_page {
            n = (*(*s).grid).sy.wrapping_div(2i32 as libc::c_uint)
        } else { n = (*(*s).grid).sy.wrapping_sub(2i32 as libc::c_uint) }
    }
    if (*data).oy.wrapping_add(n) > (*(*(*data).backing).grid).hsize {
        (*data).oy = (*(*(*data).backing).grid).hsize
    } else {
        (*data).oy =
            ((*data).oy as libc::c_uint).wrapping_add(n) as u_int as u_int
    }
    if 0 == (*data).screen.sel.flag || 0 == (*data).rectflag {
        py =
            (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
        px = window_copy_find_length(wp, py);
        if (*data).cx >= (*data).lastsx && (*data).cx != px || (*data).cx > px
           {
            window_copy_cursor_end_of_line(wp);
        }
    }
    window_copy_update_selection(wp, 1i32);
    window_copy_redraw_screen(wp);
}
unsafe extern "C" fn window_copy_pagedown(mut wp: *mut window_pane,
                                          mut half_page: libc::c_int,
                                          mut scroll_exit: libc::c_int)
 -> libc::c_int {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut n: u_int = 0;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    oy =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    ox = window_copy_find_length(wp, oy);
    if (*s).sel.lineflag as libc::c_uint ==
           LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint &&
           oy == (*data).sely {
        window_copy_other_end(wp);
    }
    if (*data).cx != ox { (*data).lastcx = (*data).cx; (*data).lastsx = ox }
    (*data).cx = (*data).lastcx;
    n = 1i32 as u_int;
    if (*(*s).grid).sy > 2i32 as libc::c_uint {
        if 0 != half_page {
            n = (*(*s).grid).sy.wrapping_div(2i32 as libc::c_uint)
        } else { n = (*(*s).grid).sy.wrapping_sub(2i32 as libc::c_uint) }
    }
    if (*data).oy < n {
        (*data).oy = 0i32 as u_int
    } else {
        (*data).oy =
            ((*data).oy as libc::c_uint).wrapping_sub(n) as u_int as u_int
    }
    if 0 == (*data).screen.sel.flag || 0 == (*data).rectflag {
        py =
            (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
        px = window_copy_find_length(wp, py);
        if (*data).cx >= (*data).lastsx && (*data).cx != px || (*data).cx > px
           {
            window_copy_cursor_end_of_line(wp);
        }
    }
    if 0 != scroll_exit && (*data).oy == 0i32 as libc::c_uint {
        return 1i32
    } else {
        window_copy_update_selection(wp, 1i32);
        window_copy_redraw_screen(wp);
        return 0i32
    };
}
unsafe extern "C" fn window_copy_cursor_next_word(mut wp: *mut window_pane,
                                                  mut separators:
                                                      *const libc::c_char)
 -> () {
    let mut current_block: u64;
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut expected: libc::c_int = 0i32;
    px = (*data).cx;
    py =
        (*(*back_s).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    xx = window_copy_find_length(wp, py);
    yy =
        (*(*back_s).grid).hsize.wrapping_add((*(*back_s).grid).sy).wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_uint);
    loop  {
        if px > xx || window_copy_in_set(wp, px, py, separators) == expected {
            if px > xx {
                if py == yy {
                    return
                } else {
                    window_copy_cursor_down(wp, 0i32);
                    px = 0i32 as u_int;
                    py =
                        (*(*back_s).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
                    xx = window_copy_find_length(wp, py)
                }
            } else { px = px.wrapping_add(1) }
        } else {
            expected = (0 == expected) as libc::c_int;
            if expected == 1i32 { continue ; }
            window_copy_update_cursor(wp, px, (*data).cy);
            if 0 != window_copy_update_selection(wp, 1i32) {
                current_block = 7815301370352969686;
                break ;
            } else { current_block = 6937071982253665452; break ; }
        }
    }
    match current_block {
        7815301370352969686 => {
            window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
        }
        _ => { }
    };
}
unsafe extern "C" fn window_copy_next_paragraph(mut wp: *mut window_pane)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut maxy: u_int = 0;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    oy =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    maxy =
        (*(*(*data).backing).grid).hsize.wrapping_add((*(*s).grid).sy).wrapping_sub(1i32
                                                                                        as
                                                                                        libc::c_uint);
    while oy < maxy && window_copy_find_length(wp, oy) == 0i32 as libc::c_uint
          {
        oy = oy.wrapping_add(1)
    }
    while oy < maxy && window_copy_find_length(wp, oy) > 0i32 as libc::c_uint
          {
        oy = oy.wrapping_add(1)
    }
    ox = window_copy_find_length(wp, oy);
    window_copy_scroll_to(wp, ox, oy);
}
unsafe extern "C" fn window_copy_cursor_right(mut wp: *mut window_pane)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut yy: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    py =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    yy =
        (*(*(*data).backing).grid).hsize.wrapping_add((*(*(*data).backing).grid).sy).wrapping_sub(1i32
                                                                                                      as
                                                                                                      libc::c_uint);
    if 0 != (*data).screen.sel.flag && 0 != (*data).rectflag {
        px = (*(*(&mut (*data).screen as *mut screen)).grid).sx
    } else { px = window_copy_find_length(wp, py) }
    if (*data).cx >= px && py < yy {
        window_copy_cursor_start_of_line(wp);
        window_copy_cursor_down(wp, 0i32);
    } else if (*data).cx < px {
        cx = (*data).cx.wrapping_add(1i32 as libc::c_uint);
        cy =
            (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
        while cx < px {
            grid_get_cell((*(*data).backing).grid, cx, cy,
                          &mut gc as *mut grid_cell);
            if 0 != !(gc.flags as libc::c_int) & 4i32 { break ; }
            cx = cx.wrapping_add(1)
        }
        window_copy_update_cursor(wp, cx, (*data).cy);
        if 0 != window_copy_update_selection(wp, 1i32) {
            window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
        }
    };
}
unsafe extern "C" fn window_copy_cursor_left(mut wp: *mut window_pane) -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut py: u_int = 0;
    let mut cx: u_int = 0;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    py =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    cx = (*data).cx;
    while cx > 0i32 as libc::c_uint {
        grid_get_cell((*(*data).backing).grid, cx, py,
                      &mut gc as *mut grid_cell);
        if 0 != !(gc.flags as libc::c_int) & 4i32 { break ; }
        cx = cx.wrapping_sub(1)
    }
    if cx == 0i32 as libc::c_uint && py > 0i32 as libc::c_uint {
        window_copy_cursor_up(wp, 0i32);
        window_copy_cursor_end_of_line(wp);
    } else if cx > 0i32 as libc::c_uint {
        window_copy_update_cursor(wp, cx.wrapping_sub(1i32 as libc::c_uint),
                                  (*data).cy);
        if 0 != window_copy_update_selection(wp, 1i32) {
            window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
        }
    };
}
unsafe extern "C" fn window_copy_copy_selection(mut wp: *mut window_pane,
                                                mut bufname:
                                                    *const libc::c_char)
 -> () {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut len: size_t = 0;
    buf = window_copy_get_selection(wp, &mut len as *mut size_t);
    if buf == 0 as *mut libc::c_void {
        return
    } else { window_copy_copy_buffer(wp, bufname, buf, len); return; };
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_start_drag(mut c: *mut client,
                                                mut m: *mut mouse_event)
 -> () {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    if c == 0 as *mut libc::c_void as *mut client {
        return
    } else {
        wp =
            cmd_mouse_pane(m, 0 as *mut *mut session, 0 as *mut *mut winlink);
        if wp == 0 as *mut libc::c_void as *mut window_pane ||
               (*wp).mode != &window_copy_mode as *const window_mode {
            return
        } else if cmd_mouse_at(wp, m, &mut x as *mut u_int,
                               &mut y as *mut u_int, 1i32) != 0i32 {
            return
        } else {
            (*c).tty.mouse_drag_update = Some(window_copy_drag_update);
            (*c).tty.mouse_drag_release = None;
            window_copy_update_cursor(wp, x, y);
            window_copy_start_selection(wp);
            window_copy_redraw_screen(wp);
            return;
        }
    };
}
unsafe extern "C" fn window_copy_drag_update(mut c: *mut client,
                                             mut m: *mut mouse_event) -> () {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut data: *mut window_copy_mode_data =
        0 as *mut window_copy_mode_data;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut old_cy: u_int = 0;
    wp = cmd_mouse_pane(m, 0 as *mut *mut session, 0 as *mut *mut winlink);
    if wp == 0 as *mut libc::c_void as *mut window_pane ||
           (*wp).mode != &window_copy_mode as *const window_mode {
        return
    } else {
        data = (*wp).modedata as *mut window_copy_mode_data;
        if cmd_mouse_at(wp, m, &mut x as *mut u_int, &mut y as *mut u_int,
                        0i32) != 0i32 {
            return
        } else {
            old_cy = (*data).cy;
            window_copy_update_cursor(wp, x, y);
            if 0 != window_copy_update_selection(wp, 1i32) {
                window_copy_redraw_selection(wp, old_cy);
            }
            return;
        }
    };
}
unsafe extern "C" fn window_copy_redraw_selection(mut wp: *mut window_pane,
                                                  mut old_y: u_int) -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut new_y: u_int = 0;
    let mut start: u_int = 0;
    let mut end: u_int = 0;
    new_y = (*data).cy;
    if old_y <= new_y {
        start = old_y;
        end = new_y
    } else { start = new_y; end = old_y }
    window_copy_redraw_lines(wp, start,
                             end.wrapping_sub(start).wrapping_add(1i32 as
                                                                      libc::c_uint));
}
unsafe extern "C" fn window_copy_cursor_back_to_indentation(mut wp:
                                                                *mut window_pane)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    px = 0i32 as u_int;
    py =
        (*(*(*data).backing).grid).hsize.wrapping_add((*data).cy).wrapping_sub((*data).oy);
    xx = window_copy_find_length(wp, py);
    while px < xx {
        grid_get_cell((*(*data).backing).grid, px, py,
                      &mut gc as *mut grid_cell);
        if gc.data.size as libc::c_int != 1i32 ||
               *gc.data.data.as_mut_ptr() as libc::c_int != 32 {
            break ;
        }
        px = px.wrapping_add(1)
    }
    window_copy_update_cursor(wp, px, (*data).cy);
    if 0 != window_copy_update_selection(wp, 1i32) {
        window_copy_redraw_lines(wp, (*data).cy, 1i32 as u_int);
    };
}
unsafe extern "C" fn window_copy_append_selection(mut wp: *mut window_pane,
                                                  mut bufname:
                                                      *const libc::c_char)
 -> () {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut bufdata: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut bufsize: size_t = 0;
    let mut ctx: screen_write_ctx =
        screen_write_ctx{wp: 0 as *mut window_pane,
                         s: 0 as *mut screen,
                         item: 0 as *mut screen_write_collect_item,
                         list: 0 as *mut screen_write_collect_line,
                         scrolled: 0,
                         bg: 0,
                         cells: 0,
                         written: 0,
                         skipped: 0,};
    buf =
        window_copy_get_selection(wp, &mut len as *mut size_t) as
            *mut libc::c_char;
    if buf == 0 as *mut libc::c_void as *mut libc::c_char {
        return
    } else {
        if options_get_number(global_options,
                              b"set-clipboard\x00" as *const u8 as
                                  *const libc::c_char) !=
               0i32 as libc::c_longlong {
            screen_write_start(&mut ctx as *mut screen_write_ctx, wp,
                               0 as *mut screen);
            screen_write_setselection(&mut ctx as *mut screen_write_ctx,
                                      buf as *mut u_char, len as u_int);
            screen_write_stop(&mut ctx as *mut screen_write_ctx);
            notify_pane(b"pane-set-clipboard\x00" as *const u8 as
                            *const libc::c_char, wp);
        }
        if bufname == 0 as *mut libc::c_void as *const libc::c_char ||
               *bufname as libc::c_int == 0 {
            pb = paste_get_top(&mut bufname as *mut *const libc::c_char)
        } else { pb = paste_get_name(bufname) }
        if pb != 0 as *mut libc::c_void as *mut paste_buffer {
            bufdata = paste_buffer_data(pb, &mut bufsize as *mut size_t);
            buf =
                xrealloc(buf as *mut libc::c_void, len.wrapping_add(bufsize))
                    as *mut libc::c_char;
            memmove(buf.offset(bufsize as isize) as *mut libc::c_void,
                    buf as *const libc::c_void, len);
            memcpy(buf as *mut libc::c_void, bufdata as *const libc::c_void,
                   bufsize);
            len =
                (len as libc::c_ulong).wrapping_add(bufsize) as size_t as
                    size_t
        }
        if paste_set(buf, len, bufname, 0 as *mut *mut libc::c_char) != 0i32 {
            free(buf as *mut libc::c_void);
        }
        return;
    };
}
unsafe extern "C" fn window_copy_move_mouse(mut m: *mut mouse_event) -> () {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    wp = cmd_mouse_pane(m, 0 as *mut *mut session, 0 as *mut *mut winlink);
    if wp == 0 as *mut libc::c_void as *mut window_pane ||
           (*wp).mode != &window_copy_mode as *const window_mode {
        return
    } else if cmd_mouse_at(wp, m, &mut x as *mut u_int, &mut y as *mut u_int,
                           0i32) != 0i32 {
        return
    } else { window_copy_update_cursor(wp, x, y); return; };
}
unsafe extern "C" fn window_copy_key_table(mut wp: *mut window_pane)
 -> *const libc::c_char {
    if options_get_number((*(*wp).window).options,
                          b"mode-keys\x00" as *const u8 as
                              *const libc::c_char) == 1i32 as libc::c_longlong
       {
        return b"copy-mode-vi\x00" as *const u8 as *const libc::c_char
    } else { return b"copy-mode\x00" as *const u8 as *const libc::c_char };
}
unsafe extern "C" fn window_copy_resize(mut wp: *mut window_pane,
                                        mut sx: u_int, mut sy: u_int) -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut ctx: screen_write_ctx =
        screen_write_ctx{wp: 0 as *mut window_pane,
                         s: 0 as *mut screen,
                         item: 0 as *mut screen_write_collect_item,
                         list: 0 as *mut screen_write_collect_line,
                         scrolled: 0,
                         bg: 0,
                         cells: 0,
                         written: 0,
                         skipped: 0,};
    screen_resize(s, sx, sy, 1i32);
    if (*data).backing != &mut (*wp).base as *mut screen {
        screen_resize((*data).backing, sx, sy, 1i32);
    }
    if (*data).cy > sy.wrapping_sub(1i32 as libc::c_uint) {
        (*data).cy = sy.wrapping_sub(1i32 as libc::c_uint)
    }
    if (*data).cx > sx { (*data).cx = sx }
    if (*data).oy > (*(*(*data).backing).grid).hsize {
        (*data).oy = (*(*(*data).backing).grid).hsize
    }
    window_copy_clear_selection(wp);
    screen_write_start(&mut ctx as *mut screen_write_ctx,
                       0 as *mut window_pane, s);
    window_copy_write_lines(wp, &mut ctx as *mut screen_write_ctx,
                            0i32 as u_int,
                            (*(*s).grid).sy.wrapping_sub(1i32 as
                                                             libc::c_uint));
    screen_write_stop(&mut ctx as *mut screen_write_ctx);
    if (*data).searchmark != 0 as *mut libc::c_void as *mut bitstr_t {
        window_copy_search_marks(wp, 0 as *mut screen);
    }
    (*data).searchx = (*data).cx as libc::c_int;
    (*data).searchy = (*data).cy as libc::c_int;
    (*data).searcho = (*data).oy as libc::c_int;
    window_copy_redraw_screen(wp);
}
unsafe extern "C" fn window_copy_free(mut wp: *mut window_pane) -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    if (*wp).fd != 1i32.wrapping_neg() {
        bufferevent_enable((*wp).event, (2i32 | 4i32) as libc::c_short);
    }
    free((*data).searchmark as *mut libc::c_void);
    free((*data).searchstr as *mut libc::c_void);
    if (*data).backing != &mut (*wp).base as *mut screen {
        screen_free((*data).backing);
        free((*data).backing as *mut libc::c_void);
    }
    screen_free(&mut (*data).screen as *mut screen);
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn window_copy_init(mut wp: *mut window_pane,
                                      mut fs: *mut cmd_find_state,
                                      mut args: *mut args) -> *mut screen {
    let mut data: *mut window_copy_mode_data =
        0 as *mut window_copy_mode_data;
    let mut s: *mut screen = 0 as *mut screen;
    data =
        xmalloc(::std::mem::size_of::<window_copy_mode_data>() as
                    libc::c_ulong) as *mut window_copy_mode_data;
    (*wp).modedata = data as *mut libc::c_void;
    (*data).oy = 0i32 as u_int;
    (*data).cx = 0i32 as u_int;
    (*data).cy = 0i32 as u_int;
    (*data).cursordrag = CURSORDRAG_NONE;
    (*data).lastcx = 0i32 as u_int;
    (*data).lastsx = 0i32 as u_int;
    (*data).backing_written = 0i32;
    (*data).rectflag = 0i32;
    (*data).scroll_exit = 0i32;
    if (*wp).searchstr != 0 as *mut libc::c_void as *mut libc::c_char {
        (*data).searchtype = WINDOW_COPY_SEARCHUP as libc::c_int;
        (*data).searchstr = xstrdup((*wp).searchstr)
    } else {
        (*data).searchtype = WINDOW_COPY_OFF as libc::c_int;
        (*data).searchstr = 0 as *mut libc::c_char
    }
    (*data).searchmark = 0 as *mut bitstr_t;
    (*data).searcho = 1i32.wrapping_neg();
    (*data).searchy = (*data).searcho;
    (*data).searchx = (*data).searchy;
    if (*wp).fd != 1i32.wrapping_neg() {
        bufferevent_disable((*wp).event, (2i32 | 4i32) as libc::c_short);
    }
    (*data).jumptype = WINDOW_COPY_OFF as libc::c_int;
    (*data).jumpchar = 0 as libc::c_char;
    s = &mut (*data).screen as *mut screen;
    screen_init(s, (*(*(&mut (*wp).base as *mut screen)).grid).sx,
                (*(*(&mut (*wp).base as *mut screen)).grid).sy,
                0i32 as u_int);
    (*s).sel.modekeys =
        options_get_number((*(*wp).window).options,
                           b"mode-keys\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    (*data).backing = 0 as *mut screen;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_init_from_pane(mut wp: *mut window_pane,
                                                    mut scroll_exit:
                                                        libc::c_int) -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    let mut ctx: screen_write_ctx =
        screen_write_ctx{wp: 0 as *mut window_pane,
                         s: 0 as *mut screen,
                         item: 0 as *mut screen_write_collect_item,
                         list: 0 as *mut screen_write_collect_line,
                         scrolled: 0,
                         bg: 0,
                         cells: 0,
                         written: 0,
                         skipped: 0,};
    let mut i: u_int = 0;
    if (*wp).mode != &window_copy_mode as *const window_mode {
        fatalx(b"not in copy mode\x00" as *const u8 as *const libc::c_char);
    } else {
        (*data).backing = &mut (*wp).base as *mut screen;
        (*data).cx = (*(*data).backing).cx;
        (*data).cy = (*(*data).backing).cy;
        (*data).scroll_exit = scroll_exit;
        (*s).cx = (*data).cx;
        (*s).cy = (*data).cy;
        screen_write_start(&mut ctx as *mut screen_write_ctx,
                           0 as *mut window_pane, s);
        i = 0i32 as u_int;
        while i < (*(*s).grid).sy {
            window_copy_write_line(wp, &mut ctx as *mut screen_write_ctx, i);
            i = i.wrapping_add(1)
        }
        screen_write_cursormove(&mut ctx as *mut screen_write_ctx, (*data).cx,
                                (*data).cy);
        screen_write_stop(&mut ctx as *mut screen_write_ctx);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_init_for_output(mut wp: *mut window_pane)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    (*data).backing =
        xmalloc(::std::mem::size_of::<screen>() as libc::c_ulong) as
            *mut screen;
    screen_init((*data).backing,
                (*(*(&mut (*wp).base as *mut screen)).grid).sx,
                (*(*(&mut (*wp).base as *mut screen)).grid).sy,
                (2147483647i32 as
                     libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32));
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_vadd(mut wp: *mut window_pane,
                                          mut fmt: *const libc::c_char,
                                          mut ap: *mut __va_list_tag) -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut backing: *mut screen = (*data).backing;
    let mut back_ctx: screen_write_ctx =
        screen_write_ctx{wp: 0 as *mut window_pane,
                         s: 0 as *mut screen,
                         item: 0 as *mut screen_write_collect_item,
                         list: 0 as *mut screen_write_collect_line,
                         scrolled: 0,
                         bg: 0,
                         cells: 0,
                         written: 0,
                         skipped: 0,};
    let mut ctx: screen_write_ctx =
        screen_write_ctx{wp: 0 as *mut window_pane,
                         s: 0 as *mut screen,
                         item: 0 as *mut screen_write_collect_item,
                         list: 0 as *mut screen_write_collect_line,
                         scrolled: 0,
                         bg: 0,
                         cells: 0,
                         written: 0,
                         skipped: 0,};
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut old_hsize: u_int = 0;
    let mut old_cy: u_int = 0;
    if backing == &mut (*wp).base as *mut screen {
        return
    } else {
        memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
               &grid_default_cell as *const grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        old_hsize = (*(*(*data).backing).grid).hsize;
        screen_write_start(&mut back_ctx as *mut screen_write_ctx,
                           0 as *mut window_pane, backing);
        if 0 != (*data).backing_written {
            screen_write_carriagereturn(&mut back_ctx as
                                            *mut screen_write_ctx);
            screen_write_linefeed(&mut back_ctx as *mut screen_write_ctx,
                                  0i32, 8i32 as u_int);
        } else { (*data).backing_written = 1i32 }
        old_cy = (*backing).cy;
        screen_write_vnputs(&mut back_ctx as *mut screen_write_ctx,
                            0i32 as ssize_t, &mut gc as *mut grid_cell, fmt,
                            ap);
        screen_write_stop(&mut back_ctx as *mut screen_write_ctx);
        (*data).oy =
            ((*data).oy as
                 libc::c_uint).wrapping_add((*(*(*data).backing).grid).hsize.wrapping_sub(old_hsize))
                as u_int as u_int;
        screen_write_start(&mut ctx as *mut screen_write_ctx, wp,
                           &mut (*data).screen as *mut screen);
        if 0 != (*(*(*data).backing).grid).hsize {
            window_copy_redraw_lines(wp, 0i32 as u_int, 1i32 as u_int);
        }
        window_copy_redraw_lines(wp, old_cy,
                                 (*backing).cy.wrapping_sub(old_cy).wrapping_add(1i32
                                                                                     as
                                                                                     libc::c_uint));
        screen_write_stop(&mut ctx as *mut screen_write_ctx);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_add_formats(mut wp: *mut window_pane,
                                                 mut ft: *mut format_tree)
 -> () {
    let mut data: *mut window_copy_mode_data =
        (*wp).modedata as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen as *mut screen;
    if (*wp).mode != &window_copy_mode as *const window_mode {
        return
    } else {
        format_add(ft,
                   b"selection_present\x00" as *const u8 as
                       *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   (*s).sel.flag);
        format_add(ft,
                   b"scroll_position\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char, (*data).oy);
        return;
    };
}

