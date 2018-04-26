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
    pub type format_job_tree;
    pub type hooks;
    pub type _IO_FILE_plus;
    pub type tty_code;
    pub type format_tree;
    pub type evbuffer;
    pub type tmuxproc;
    pub type screen_titles;
    pub type input_ctx;
    pub type event_base;
    pub type args_entry;
    pub type environ;
    pub type tmuxpeer;
    pub type options;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
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
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, ...) -> libc::c_int;
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
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xvasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char,
                  _: *mut __va_list_tag) -> libc::c_int;
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
    fn tty_write(_:
                     Option<unsafe extern "C" fn(_: *mut tty,
                                                 _: *const tty_ctx) -> ()>,
                 _: *mut tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_alignmenttest(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_cell(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_cells(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_clearendofline(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_clearendofscreen(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_clearline(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_clearscreen(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_clearstartofline(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_clearstartofscreen(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_deletecharacter(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_clearcharacter(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_deleteline(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_insertcharacter(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_insertline(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_scrollup(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_reverseindex(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_setselection(_: *mut tty, _: *const tty_ctx) -> ();
    #[no_mangle]
    fn tty_cmd_rawstring(_: *mut tty, _: *const tty_ctx) -> ();
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
    fn grid_cells_equal(_: *const grid_cell, _: *const grid_cell)
     -> libc::c_int;
    #[no_mangle]
    fn grid_get_cell(_: *mut grid, _: u_int, _: u_int, _: *mut grid_cell)
     -> ();
    #[no_mangle]
    fn grid_move_lines(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn grid_view_get_cell(_: *mut grid, _: u_int, _: u_int, _: *mut grid_cell)
     -> ();
    #[no_mangle]
    fn grid_view_set_cell(_: *mut grid, _: u_int, _: u_int,
                          _: *const grid_cell) -> ();
    #[no_mangle]
    fn grid_view_set_cells(_: *mut grid, _: u_int, _: u_int,
                           _: *const grid_cell, _: *const libc::c_char,
                           _: size_t) -> ();
    #[no_mangle]
    fn grid_view_clear_history(_: *mut grid, _: u_int) -> ();
    #[no_mangle]
    fn grid_view_clear(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int,
                       _: u_int) -> ();
    #[no_mangle]
    fn grid_view_scroll_region_up(_: *mut grid, _: u_int, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn grid_view_scroll_region_down(_: *mut grid, _: u_int, _: u_int,
                                    _: u_int) -> ();
    #[no_mangle]
    fn grid_view_insert_lines(_: *mut grid, _: u_int, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn grid_view_insert_lines_region(_: *mut grid, _: u_int, _: u_int,
                                     _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn grid_view_delete_lines(_: *mut grid, _: u_int, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn grid_view_delete_lines_region(_: *mut grid, _: u_int, _: u_int,
                                     _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn grid_view_insert_cells(_: *mut grid, _: u_int, _: u_int, _: u_int,
                              _: u_int) -> ();
    #[no_mangle]
    fn grid_view_delete_cells(_: *mut grid, _: u_int, _: u_int, _: u_int,
                              _: u_int) -> ();
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn screen_reset_tabs(_: *mut screen) -> ();
    #[no_mangle]
    fn screen_select_cell(_: *mut screen, _: *mut grid_cell,
                          _: *const grid_cell) -> ();
    #[no_mangle]
    fn screen_check_selection(_: *mut screen, _: u_int, _: u_int)
     -> libc::c_int;
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn utf8_set(_: *mut utf8_data, _: u_char) -> ();
    #[no_mangle]
    fn utf8_append(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn utf8_open(_: *mut utf8_data, _: u_char) -> utf8_state;
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
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
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
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
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
pub struct unnamed_2 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub type u_char = __u_char;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const TTY_VT102: unnamed_15 = 2;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
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
    pub entry: unnamed_30,
    pub wentry: unnamed_35,
    pub sentry: unnamed_37,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
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
    pub tree_entry: unnamed_1,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type options_table_scope = libc::c_uint;
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_9 {
    ev_io: unnamed_16,
    ev_signal: unnamed_18,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const TTY_VT101: unnamed_15 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_39,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const TTY_VT100: unnamed_15 = 0;
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_3,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const JOB_DEAD: unnamed_27 = 1;
pub const LINE_SEL_LEFT_RIGHT: unnamed_38 = 1;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub const UTF8_MORE: utf8_state = 0;
pub const TTY_UNKNOWN: unnamed_15 = 6;
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
    pub message_log: unnamed_23,
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
    pub entry: unnamed_26,
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
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub type size_t = libc::c_ulong;
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
pub type unnamed_15 = libc::c_uint;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
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
pub struct unnamed_16 {
    pub ev_io_next: unnamed_20,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub type __ssize_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub ev_signal_next: unnamed_7,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqh_first: *mut screen_write_collect_item,
    pub tqh_last: *mut *mut screen_write_collect_item,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type options_table_type = libc::c_uint;
pub type __u_char = libc::c_uchar;
pub type cc_t = libc::c_uchar;
pub const JOB_CLOSED: unnamed_27 = 2;
pub type utf8_state = libc::c_uint;
pub type uint8_t = libc::c_uchar;
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
    pub alerts_entry: unnamed_0,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed,
    pub entry: unnamed_8,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_write_collect_line {
    pub items: unnamed_19,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_12,
    pub entry: unnamed_34,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_4,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_29,
    pub ev_next: unnamed_33,
    pub ev_timeout_pos: unnamed_28,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_9,
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
pub struct unnamed_22 {
    pub tqe_next: *mut screen_write_collect_item,
    pub tqe_prev: *mut *mut screen_write_collect_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
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
pub struct unnamed_24 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub const JOB_RUNNING: unnamed_27 = 0;
pub const LINE_SEL_RIGHT_LEFT: unnamed_38 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type pid_t = __pid_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub type __pid_t = libc::c_int;
pub type unnamed_27 = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub type __u_short = libc::c_ushort;
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
    pub entry: unnamed_21,
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
pub union unnamed_28 {
    ev_next_with_common_timeout: unnamed_25,
    min_heap_idx: libc::c_int,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_write_collect_item {
    pub x: u_int,
    pub wrapped: libc::c_int,
    pub used: u_int,
    pub data: [libc::c_char; 256],
    pub gc: grid_cell,
    pub entry: unnamed_22,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_2,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const PROMPT_ENTRY: unnamed_36 = 0;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
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
    pub term_type: unnamed_15,
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
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
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
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_24,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_ctx {
    pub wp: *mut window_pane,
    pub cell: *const grid_cell,
    pub wrapped: libc::c_int,
    pub num: u_int,
    pub ptr: *mut libc::c_void,
    pub ocx: u_int,
    pub ocy: u_int,
    pub orupper: u_int,
    pub orlower: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub bg: u_int,
}
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type tcflag_t = libc::c_uint;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type cmd_retval = libc::c_int;
pub type time_t = __time_t;
pub const TTY_VT420: unnamed_15 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub type __u_int = libc::c_uint;
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const TTY_VT220: unnamed_15 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_40,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub type speed_t = libc::c_uint;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
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
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub type unnamed_36 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const PROMPT_COMMAND: unnamed_36 = 1;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
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
pub type u_int = __u_int;
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const TTY_VT320: unnamed_15 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_27,
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
    pub entry: unnamed_31,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub const LINE_SEL_NONE: unnamed_38 = 0;
pub const UTF8_DONE: utf8_state = 1;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub type unnamed_38 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_38,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
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
pub type ssize_t = __ssize_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_5,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
    pub gentry: unnamed_17,
    pub entry: unnamed_11,
}
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_6,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_39 {
    offset: u_int,
    data: unnamed_10,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_32,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_40 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
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
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_start(mut ctx: *mut screen_write_ctx,
                                            mut wp: *mut window_pane,
                                            mut s: *mut screen) -> () {
    let mut tmp: [libc::c_char; 16] = [0; 16];
    let mut y: u_int = 0;
    memset(ctx as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<screen_write_ctx>() as libc::c_ulong);
    (*ctx).wp = wp;
    if wp != 0 as *mut libc::c_void as *mut window_pane &&
           s == 0 as *mut libc::c_void as *mut screen {
        (*ctx).s = (*wp).screen
    } else { (*ctx).s = s }
    (*ctx).list =
        xcalloc((*(*(*ctx).s).grid).sy as size_t,
                ::std::mem::size_of::<screen_write_collect_line>() as
                    libc::c_ulong) as *mut screen_write_collect_line;
    y = 0i32 as u_int;
    while y < (*(*(*ctx).s).grid).sy {
        loop  {
            let ref mut fresh0 =
                (*(&mut (*(*ctx).list.offset(y as isize)).items as
                       *mut unnamed_19)).tqh_first;
            *fresh0 = 0 as *mut screen_write_collect_item;
            let ref mut fresh1 =
                (*(&mut (*(*ctx).list.offset(y as isize)).items as
                       *mut unnamed_19)).tqh_last;
            *fresh1 =
                &mut (*(&mut (*(*ctx).list.offset(y as isize)).items as
                            *mut unnamed_19)).tqh_first as
                    *mut *mut screen_write_collect_item;
            if !(0 != 0i32) { break ; }
        }
        y = y.wrapping_add(1)
    }
    (*ctx).item =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<screen_write_collect_item>() as
                    libc::c_ulong) as *mut screen_write_collect_item;
    (*ctx).scrolled = 0i32 as u_int;
    (*ctx).bg = 8i32 as u_int;
    if wp != 0 as *mut libc::c_void as *mut window_pane {
        snprintf(tmp.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                 b"pane %%%u\x00" as *const u8 as *const libc::c_char,
                 (*wp).id);
    }
    log_debug(b"%s: size %ux%u, %s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"screen_write_start\x00")).as_ptr(),
              (*(*(*ctx).s).grid).sx, (*(*(*ctx).s).grid).sy,
              if wp == 0 as *mut libc::c_void as *mut window_pane {
                  b"no pane\x00" as *const u8 as *const libc::c_char
              } else { tmp.as_mut_ptr() });
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_stop(mut ctx: *mut screen_write_ctx)
 -> () {
    screen_write_collect_end(ctx);
    screen_write_collect_flush(ctx, 0i32);
    log_debug(b"%s: %u cells (%u written, %u skipped)\x00" as *const u8 as
                  *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"screen_write_stop\x00")).as_ptr(),
              (*ctx).cells, (*ctx).written, (*ctx).skipped);
    free((*ctx).item as *mut libc::c_void);
    free((*ctx).list as *mut libc::c_void);
}
unsafe extern "C" fn screen_write_collect_flush(mut ctx:
                                                    *mut screen_write_ctx,
                                                mut scroll_only: libc::c_int)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut ci: *mut screen_write_collect_item =
        0 as *mut screen_write_collect_item;
    let mut tmp: *mut screen_write_collect_item =
        0 as *mut screen_write_collect_item;
    let mut y: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut items: u_int = 0i32 as u_int;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    let mut written: size_t = 0i32 as size_t;
    if (*ctx).scrolled != 0i32 as libc::c_uint {
        log_debug(b"%s: scrolled %u (region %u-%u)\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"screen_write_collect_flush\x00")).as_ptr(),
                  (*ctx).scrolled, (*s).rupper, (*s).rlower);
        if (*ctx).scrolled >
               (*s).rlower.wrapping_sub((*s).rupper).wrapping_add(1i32 as
                                                                      libc::c_uint)
           {
            (*ctx).scrolled =
                (*s).rlower.wrapping_sub((*s).rupper).wrapping_add(1i32 as
                                                                       libc::c_uint)
        }
        screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
        ttyctx.num = (*ctx).scrolled;
        ttyctx.bg = (*ctx).bg;
        tty_write(Some(tty_cmd_scrollup), &mut ttyctx as *mut tty_ctx);
    }
    (*ctx).scrolled = 0i32 as u_int;
    (*ctx).bg = 8i32 as u_int;
    if 0 != scroll_only {
        return
    } else {
        cx = (*s).cx;
        cy = (*s).cy;
        y = 0i32 as u_int;
        while y < (*(*s).grid).sy {
            ci =
                (*(&mut (*(*ctx).list.offset(y as isize)).items as
                       *mut unnamed_19)).tqh_first;
            while ci !=
                      0 as *mut libc::c_void as *mut screen_write_collect_item
                      && { tmp = (*ci).entry.tqe_next; 0 != 1i32 } {
                screen_write_cursormove(ctx, (*ci).x, y);
                screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
                ttyctx.cell = &mut (*ci).gc as *mut grid_cell;
                ttyctx.wrapped = (*ci).wrapped;
                ttyctx.ptr = (*ci).data.as_mut_ptr() as *mut libc::c_void;
                ttyctx.num = (*ci).used;
                tty_write(Some(tty_cmd_cells), &mut ttyctx as *mut tty_ctx);
                items = items.wrapping_add(1);
                written =
                    (written as
                         libc::c_ulong).wrapping_add((*ci).used as
                                                         libc::c_ulong) as
                        size_t as size_t;
                loop  {
                    if (*ci).entry.tqe_next !=
                           0 as *mut libc::c_void as
                               *mut screen_write_collect_item {
                        (*(*ci).entry.tqe_next).entry.tqe_prev =
                            (*ci).entry.tqe_prev
                    } else {
                        let ref mut fresh2 =
                            (*(&mut (*(*ctx).list.offset(y as isize)).items as
                                   *mut unnamed_19)).tqh_last;
                        *fresh2 = (*ci).entry.tqe_prev
                    }
                    *(*ci).entry.tqe_prev = (*ci).entry.tqe_next;
                    if !(0 != 0i32) { break ; }
                }
                free(ci as *mut libc::c_void);
                ci = tmp
            }
            y = y.wrapping_add(1)
        }
        (*s).cx = cx;
        (*s).cy = cy;
        log_debug(b"%s: flushed %u items (%zu bytes)\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"screen_write_collect_flush\x00")).as_ptr(),
                  items, written);
        (*ctx).written =
            ((*ctx).written as libc::c_ulong).wrapping_add(written) as u_int
                as u_int;
        return;
    };
}
unsafe extern "C" fn screen_write_initctx(mut ctx: *mut screen_write_ctx,
                                          mut ttyctx: *mut tty_ctx) -> () {
    let mut s: *mut screen = (*ctx).s;
    memset(ttyctx as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<tty_ctx>() as libc::c_ulong);
    (*ttyctx).wp = (*ctx).wp;
    (*ttyctx).ocx = (*s).cx;
    (*ttyctx).ocy = (*s).cy;
    (*ttyctx).orlower = (*s).rlower;
    (*ttyctx).orupper = (*s).rupper;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursormove(mut ctx:
                                                     *mut screen_write_ctx,
                                                 mut px: u_int, mut py: u_int)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    if px > (*(*s).grid).sx.wrapping_sub(1i32 as libc::c_uint) {
        px = (*(*s).grid).sx.wrapping_sub(1i32 as libc::c_uint)
    }
    if py > (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint) {
        py = (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint)
    }
    (*s).cx = px;
    (*s).cy = py;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_collect_end(mut ctx:
                                                      *mut screen_write_ctx)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut ci: *mut screen_write_collect_item = (*ctx).item;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut xx: u_int = 0;
    if (*ci).used == 0i32 as libc::c_uint {
        return
    } else {
        (*ci).data[(*ci).used as usize] = 0 as libc::c_char;
        (*ci).x = (*s).cx;
        loop  {
            (*ci).entry.tqe_next = 0 as *mut screen_write_collect_item;
            (*ci).entry.tqe_prev =
                (*(&mut (*(*ctx).list.offset((*s).cy as isize)).items as
                       *mut unnamed_19)).tqh_last;
            let ref mut fresh3 =
                *(*(&mut (*(*ctx).list.offset((*s).cy as isize)).items as
                        *mut unnamed_19)).tqh_last;
            *fresh3 = ci;
            let ref mut fresh4 =
                (*(&mut (*(*ctx).list.offset((*s).cy as isize)).items as
                       *mut unnamed_19)).tqh_last;
            *fresh4 =
                &mut (*ci).entry.tqe_next as
                    *mut *mut screen_write_collect_item;
            if !(0 != 0i32) { break ; }
        }
        (*ctx).item =
            xcalloc(1i32 as size_t,
                    ::std::mem::size_of::<screen_write_collect_item>() as
                        libc::c_ulong) as *mut screen_write_collect_item;
        log_debug(b"%s: %u %s (at %u,%u)\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"screen_write_collect_end\x00")).as_ptr(),
                  (*ci).used, (*ci).data.as_mut_ptr(), (*s).cx, (*s).cy);
        if (*s).cx != 0i32 as libc::c_uint {
            xx = (*s).cx;
            while xx > 0i32 as libc::c_uint {
                grid_view_get_cell((*s).grid, xx, (*s).cy,
                                   &mut gc as *mut grid_cell);
                if 0 != !(gc.flags as libc::c_int) & 4i32 { break ; }
                grid_view_set_cell((*s).grid, xx, (*s).cy,
                                   &grid_default_cell as *const grid_cell);
                xx = xx.wrapping_sub(1)
            }
            if gc.data.width as libc::c_int > 1i32 {
                grid_view_set_cell((*s).grid, xx, (*s).cy,
                                   &grid_default_cell as *const grid_cell);
            }
        }
        memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
               &mut (*ci).gc as *mut grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        grid_view_set_cells((*s).grid, (*s).cx, (*s).cy,
                            &mut gc as *mut grid_cell,
                            (*ci).data.as_mut_ptr(), (*ci).used as size_t);
        (*s).cx =
            ((*s).cx as libc::c_uint).wrapping_add((*ci).used) as u_int as
                u_int;
        xx = (*s).cx;
        while xx < (*(*s).grid).sx {
            grid_view_get_cell((*s).grid, xx, (*s).cy,
                               &mut gc as *mut grid_cell);
            if 0 != !(gc.flags as libc::c_int) & 4i32 { break ; }
            grid_view_set_cell((*s).grid, xx, (*s).cy,
                               &grid_default_cell as *const grid_cell);
            xx = xx.wrapping_add(1)
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_reset(mut ctx: *mut screen_write_ctx)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    screen_reset_tabs(s);
    screen_write_scrollregion(ctx, 0i32 as u_int,
                              (*(*s).grid).sy.wrapping_sub(1i32 as
                                                               libc::c_uint));
    (*s).mode &= !(2i32 | 4i32 | 8i32 | 2048i32);
    (*s).mode &= !(32i32 | 64i32 | 4096i32 | 256i32 | 512i32);
    screen_write_clearscreen(ctx, 8i32 as u_int);
    screen_write_cursormove(ctx, 0i32 as u_int, 0i32 as u_int);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearscreen(mut ctx:
                                                      *mut screen_write_ctx,
                                                  mut bg: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    let mut sx: u_int = (*(*s).grid).sx;
    let mut sy: u_int = (*(*s).grid).sy;
    screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
    ttyctx.bg = bg;
    if 0 != (*(*s).grid).flags & 1i32 {
        grid_view_clear_history((*s).grid, bg);
    } else {
        grid_view_clear((*s).grid, 0i32 as u_int, 0i32 as u_int, sx, sy, bg);
    }
    screen_write_collect_clear(ctx, 0i32 as u_int, sy);
    tty_write(Some(tty_cmd_clearscreen), &mut ttyctx as *mut tty_ctx);
}
unsafe extern "C" fn screen_write_collect_clear(mut ctx:
                                                    *mut screen_write_ctx,
                                                mut y: u_int, mut n: u_int)
 -> () {
    let mut ci: *mut screen_write_collect_item =
        0 as *mut screen_write_collect_item;
    let mut tmp: *mut screen_write_collect_item =
        0 as *mut screen_write_collect_item;
    let mut i: u_int = 0;
    let mut size: size_t = 0;
    i = y;
    while i < y.wrapping_add(n) {
        if !((*(&mut (*(*ctx).list.offset(i as isize)).items as
                    *mut unnamed_19)).tqh_first ==
                 0 as *mut libc::c_void as *mut screen_write_collect_item) {
            size = 0i32 as size_t;
            ci =
                (*(&mut (*(*ctx).list.offset(i as isize)).items as
                       *mut unnamed_19)).tqh_first;
            while ci !=
                      0 as *mut libc::c_void as *mut screen_write_collect_item
                      && { tmp = (*ci).entry.tqe_next; 0 != 1i32 } {
                size =
                    (size as
                         libc::c_ulong).wrapping_add((*ci).used as
                                                         libc::c_ulong) as
                        size_t as size_t;
                loop  {
                    if (*ci).entry.tqe_next !=
                           0 as *mut libc::c_void as
                               *mut screen_write_collect_item {
                        (*(*ci).entry.tqe_next).entry.tqe_prev =
                            (*ci).entry.tqe_prev
                    } else {
                        let ref mut fresh5 =
                            (*(&mut (*(*ctx).list.offset(i as isize)).items as
                                   *mut unnamed_19)).tqh_last;
                        *fresh5 = (*ci).entry.tqe_prev
                    }
                    *(*ci).entry.tqe_prev = (*ci).entry.tqe_next;
                    if !(0 != 0i32) { break ; }
                }
                free(ci as *mut libc::c_void);
                ci = tmp
            }
            (*ctx).skipped =
                ((*ctx).skipped as libc::c_ulong).wrapping_add(size) as u_int
                    as u_int;
            log_debug(b"%s: dropped %zu bytes (line %u)\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"screen_write_collect_clear\x00")).as_ptr(),
                      size, i);
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_scrollregion(mut ctx:
                                                       *mut screen_write_ctx,
                                                   mut rupper: u_int,
                                                   mut rlower: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    if rupper > (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint) {
        rupper = (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint)
    }
    if rlower > (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint) {
        rlower = (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint)
    }
    if rupper >= rlower {
        return
    } else {
        screen_write_collect_flush(ctx, 0i32);
        (*s).cx = 0i32 as u_int;
        (*s).cy = 0i32 as u_int;
        (*s).rupper = rupper;
        (*s).rlower = rlower;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_vnputs(mut ctx: *mut screen_write_ctx,
                                             mut maxlen: ssize_t,
                                             mut gcp: *const grid_cell,
                                             mut fmt: *const libc::c_char,
                                             mut ap: *mut __va_list_tag)
 -> () {
    let mut current_block: u64;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut ud: *mut utf8_data = &mut gc.data as *mut utf8_data;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut left: size_t = 0;
    let mut size: size_t = 0i32 as size_t;
    let mut more: utf8_state = UTF8_MORE;
    memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
           gcp as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    xvasprintf(&mut msg as *mut *mut libc::c_char, fmt, ap);
    ptr = msg as *mut u_char;
    loop  {
        if !(*ptr as libc::c_int != 0) {
            current_block = 14916268686031723178;
            break ;
        }
        if *ptr as libc::c_int > 127i32 &&
               utf8_open(ud, *ptr) as libc::c_uint ==
                   UTF8_MORE as libc::c_int as libc::c_uint {
            ptr = ptr.offset(1isize);
            left = strlen(ptr as *const libc::c_char);
            if left <
                   ((*ud).size as size_t).wrapping_sub(1i32 as libc::c_ulong)
               {
                current_block = 14916268686031723178;
                break ;
            }
            loop  {
                more = utf8_append(ud, *ptr);
                if !(more as libc::c_uint ==
                         UTF8_MORE as libc::c_int as libc::c_uint) {
                    break ;
                }
                ptr = ptr.offset(1isize)
            }
            ptr = ptr.offset(1isize);
            if more as libc::c_uint !=
                   UTF8_DONE as libc::c_int as libc::c_uint {
                continue ;
            }
            if maxlen > 0i32 as libc::c_long &&
                   size.wrapping_add((*ud).width as libc::c_ulong) >
                       maxlen as size_t {
                current_block = 7815301370352969686;
                break ;
            }
            size =
                (size as
                     libc::c_ulong).wrapping_add((*ud).width as libc::c_ulong)
                    as size_t as size_t;
            screen_write_cell(ctx, &mut gc as *mut grid_cell);
        } else {
            if maxlen > 0i32 as libc::c_long &&
                   size.wrapping_add(1i32 as libc::c_ulong) > maxlen as size_t
               {
                current_block = 14916268686031723178;
                break ;
            }
            if *ptr as libc::c_int == 1 {
                gc.attr = (gc.attr as libc::c_int ^ 128i32) as u_short
            } else if *ptr as libc::c_int > 31i32 &&
                          (*ptr as libc::c_int) < 127i32 {
                size = size.wrapping_add(1);
                screen_write_putc(ctx, &mut gc as *mut grid_cell, *ptr);
            }
            ptr = ptr.offset(1isize)
        }
    }
    loop  {
        match current_block {
            14916268686031723178 => {
                free(msg as *mut libc::c_void);
                break ;
            }
            _ => {
                if !(size < maxlen as size_t) {
                    current_block = 14916268686031723178;
                    continue ;
                }
                screen_write_putc(ctx, &mut gc as *mut grid_cell,
                                  32 as u_char);
                size = size.wrapping_add(1);
                current_block = 7815301370352969686;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_putc(mut ctx: *mut screen_write_ctx,
                                           mut gcp: *const grid_cell,
                                           mut ch: u_char) -> () {
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
           gcp as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    utf8_set(&mut gc.data as *mut utf8_data, ch);
    screen_write_cell(ctx, &mut gc as *mut grid_cell);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cell(mut ctx: *mut screen_write_ctx,
                                           mut gc: *const grid_cell) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut gce: *mut grid_cell_entry = 0 as *mut grid_cell_entry;
    let mut tmp_gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut now_gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    let mut sx: u_int = (*(*s).grid).sx;
    let mut sy: u_int = (*(*s).grid).sy;
    let mut width: u_int = (*gc).data.width as u_int;
    let mut xx: u_int = 0;
    let mut last: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut selected: libc::c_int = 0;
    let mut skip: libc::c_int = 1i32;
    if 0 != (*gc).flags as libc::c_int & 4i32 {
        return
    } else {
        (*ctx).cells = (*ctx).cells.wrapping_add(1);
        if width == 0i32 as libc::c_uint {
            screen_write_collect_flush(ctx, 0i32);
            gc =
                screen_write_combine(ctx, &(*gc).data as *const utf8_data,
                                     &mut xx as *mut u_int);
            if gc != 0 as *const grid_cell {
                cx = (*s).cx;
                cy = (*s).cy;
                screen_write_cursormove(ctx, xx, (*s).cy);
                screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
                ttyctx.cell = gc;
                tty_write(Some(tty_cmd_cell), &mut ttyctx as *mut tty_ctx);
                (*s).cx = cx;
                (*s).cy = cy
            }
            return
        } else {
            screen_write_collect_flush(ctx, 1i32);
            if 0 != !(*s).mode & 16i32 && width > 1i32 as libc::c_uint &&
                   (width > sx ||
                        (*s).cx != sx && (*s).cx > sx.wrapping_sub(width)) {
                return
            } else {
                if 0 != (*s).mode & 2i32 {
                    grid_view_insert_cells((*s).grid, (*s).cx, (*s).cy, width,
                                           8i32 as u_int);
                    skip = 0i32
                }
                if 0 != (*s).mode & 16i32 && (*s).cx > sx.wrapping_sub(width)
                   {
                    log_debug(b"%s: wrapped at %u,%u\x00" as *const u8 as
                                  *const libc::c_char,
                              (*::std::mem::transmute::<&[u8; 18],
                                                        &[libc::c_char; 18]>(b"screen_write_cell\x00")).as_ptr(),
                              (*s).cx, (*s).cy);
                    screen_write_linefeed(ctx, 1i32, 8i32 as u_int);
                    (*s).cx = 0i32 as u_int;
                    screen_write_collect_flush(ctx, 1i32);
                }
                if (*s).cx > sx.wrapping_sub(width) ||
                       (*s).cy > sy.wrapping_sub(1i32 as libc::c_uint) {
                    return
                } else {
                    screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
                    gl =
                        &mut *(*(*s).grid).linedata.offset((*(*s).grid).hsize.wrapping_add((*s).cy)
                                                               as isize) as
                            *mut grid_line;
                    if 0 != (*gl).flags & 2i32 {
                        grid_view_get_cell(gd, (*s).cx, (*s).cy,
                                           &mut now_gc as *mut grid_cell);
                        if 0 !=
                               screen_write_overwrite(ctx,
                                                      &mut now_gc as
                                                          *mut grid_cell,
                                                      width) {
                            skip = 0i32
                        }
                    }
                    xx = (*s).cx.wrapping_add(1i32 as libc::c_uint);
                    while xx < (*s).cx.wrapping_add(width) {
                        log_debug(b"%s: new padding at %u,%u\x00" as *const u8
                                      as *const libc::c_char,
                                  (*::std::mem::transmute::<&[u8; 18],
                                                            &[libc::c_char; 18]>(b"screen_write_cell\x00")).as_ptr(),
                                  xx, (*s).cy);
                        grid_view_set_cell(gd, xx, (*s).cy,
                                           &screen_write_pad_cell as
                                               *const grid_cell);
                        skip = 0i32;
                        xx = xx.wrapping_add(1)
                    }
                    if 0 != skip {
                        if (*s).cx >= (*gl).cellsize {
                            skip =
                                grid_cells_equal(gc,
                                                 &grid_default_cell as
                                                     *const grid_cell)
                        } else {
                            gce =
                                &mut *(*gl).celldata.offset((*s).cx as isize)
                                    as *mut grid_cell_entry;
                            if 0 != (*gce).flags as libc::c_int & 8i32 {
                                skip = 0i32
                            } else if (*gc).flags as libc::c_int !=
                                          (*gce).flags as libc::c_int {
                                skip = 0i32
                            } else if (*gc).attr as libc::c_int !=
                                          (*gce).unnamed.data.attr as
                                              libc::c_int {
                                skip = 0i32
                            } else if (*gc).fg !=
                                          (*gce).unnamed.data.fg as
                                              libc::c_int {
                                skip = 0i32
                            } else if (*gc).bg !=
                                          (*gce).unnamed.data.bg as
                                              libc::c_int {
                                skip = 0i32
                            } else if (*gc).data.width as libc::c_int != 1i32
                             {
                                skip = 0i32
                            } else if (*gc).data.size as libc::c_int != 1i32 {
                                skip = 0i32
                            } else if (*gce).unnamed.data.data as libc::c_int
                                          !=
                                          (*gc).data.data[0usize] as
                                              libc::c_int {
                                skip = 0i32
                            }
                        }
                    }
                    selected = screen_check_selection(s, (*s).cx, (*s).cy);
                    if 0 != selected &&
                           0 != !((*gc).flags as libc::c_int) & 16i32 {
                        memcpy(&mut tmp_gc as *mut grid_cell as
                                   *mut libc::c_void,
                               gc as *const libc::c_void,
                               ::std::mem::size_of::<grid_cell>() as
                                   libc::c_ulong);
                        tmp_gc.flags =
                            (tmp_gc.flags as libc::c_int | 16i32) as u_char;
                        grid_view_set_cell(gd, (*s).cx, (*s).cy,
                                           &mut tmp_gc as *mut grid_cell);
                    } else if 0 == selected &&
                                  0 != (*gc).flags as libc::c_int & 16i32 {
                        memcpy(&mut tmp_gc as *mut grid_cell as
                                   *mut libc::c_void,
                               gc as *const libc::c_void,
                               ::std::mem::size_of::<grid_cell>() as
                                   libc::c_ulong);
                        tmp_gc.flags =
                            (tmp_gc.flags as libc::c_int & !16i32) as u_char;
                        grid_view_set_cell(gd, (*s).cx, (*s).cy,
                                           &mut tmp_gc as *mut grid_cell);
                    } else if 0 == skip {
                        grid_view_set_cell(gd, (*s).cx, (*s).cy, gc);
                    }
                    if 0 != selected { skip = 0i32 }
                    last = (0 == (*s).mode & 16i32) as libc::c_int as u_int;
                    if (*s).cx <= sx.wrapping_sub(last).wrapping_sub(width) {
                        (*s).cx =
                            ((*s).cx as libc::c_uint).wrapping_add(width) as
                                u_int as u_int
                    } else { (*s).cx = sx.wrapping_sub(last) }
                    if 0 != (*s).mode & 2i32 {
                        screen_write_collect_flush(ctx, 0i32);
                        ttyctx.num = width;
                        tty_write(Some(tty_cmd_insertcharacter),
                                  &mut ttyctx as *mut tty_ctx);
                    }
                    if 0 == skip {
                        if 0 != selected {
                            screen_select_cell(s,
                                               &mut tmp_gc as *mut grid_cell,
                                               gc);
                            ttyctx.cell = &mut tmp_gc as *mut grid_cell
                        } else { ttyctx.cell = gc }
                        tty_write(Some(tty_cmd_cell),
                                  &mut ttyctx as *mut tty_ctx);
                        (*ctx).written = (*ctx).written.wrapping_add(1)
                    } else { (*ctx).skipped = (*ctx).skipped.wrapping_add(1) }
                    return;
                }
            }
        }
    };
}
static mut screen_write_pad_cell: grid_cell =
    unsafe {
        grid_cell{flags: 4i32 as u_char,
                  attr: 0i32 as u_short,
                  fg: 8i32,
                  bg: 8i32,
                  data:
                      utf8_data{data:
                                    [0i32 as u_char, 0, 0, 0, 0, 0, 0, 0, 0],
                                have: 0i32 as u_char,
                                size: 0i32 as u_char,
                                width: 0i32 as u_char,},}
    };
unsafe extern "C" fn screen_write_overwrite(mut ctx: *mut screen_write_ctx,
                                            mut gc: *mut grid_cell,
                                            mut width: u_int) -> libc::c_int {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut tmp_gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut xx: u_int = 0;
    let mut done: libc::c_int = 0i32;
    if 0 != (*gc).flags as libc::c_int & 4i32 {
        xx = (*s).cx.wrapping_add(1i32 as libc::c_uint);
        loop  {
            xx = xx.wrapping_sub(1);
            if !(xx > 0i32 as libc::c_uint) { break ; }
            grid_view_get_cell(gd, xx, (*s).cy,
                               &mut tmp_gc as *mut grid_cell);
            if 0 != !(tmp_gc.flags as libc::c_int) & 4i32 { break ; }
            log_debug(b"%s: padding at %u,%u\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"screen_write_overwrite\x00")).as_ptr(),
                      xx, (*s).cy);
            grid_view_set_cell(gd, xx, (*s).cy,
                               &grid_default_cell as *const grid_cell);
        }
        log_debug(b"%s: character at %u,%u\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"screen_write_overwrite\x00")).as_ptr(),
                  xx, (*s).cy);
        grid_view_set_cell(gd, xx, (*s).cy,
                           &grid_default_cell as *const grid_cell);
        done = 1i32
    }
    if width != 1i32 as libc::c_uint ||
           (*gc).data.width as libc::c_int != 1i32 ||
           0 != (*gc).flags as libc::c_int & 4i32 {
        xx = (*s).cx.wrapping_add(width).wrapping_sub(1i32 as libc::c_uint);
        loop  {
            xx = xx.wrapping_add(1);
            if !(xx < (*(*s).grid).sx) { break ; }
            grid_view_get_cell(gd, xx, (*s).cy,
                               &mut tmp_gc as *mut grid_cell);
            if 0 != !(tmp_gc.flags as libc::c_int) & 4i32 { break ; }
            log_debug(b"%s: overwrite at %u,%u\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"screen_write_overwrite\x00")).as_ptr(),
                      xx, (*s).cy);
            grid_view_set_cell(gd, xx, (*s).cy,
                               &grid_default_cell as *const grid_cell);
            done = 1i32
        }
    }
    return done;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_linefeed(mut ctx: *mut screen_write_ctx,
                                               mut wrapped: libc::c_int,
                                               mut bg: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    gl =
        &mut *(*gd).linedata.offset((*gd).hsize.wrapping_add((*s).cy) as
                                        isize) as *mut grid_line;
    if 0 != wrapped { (*gl).flags |= 1i32 } else { (*gl).flags &= !1i32 }
    log_debug(b"%s: at %u,%u (region %u-%u)\x00" as *const u8 as
                  *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"screen_write_linefeed\x00")).as_ptr(),
              (*s).cx, (*s).cy, (*s).rupper, (*s).rlower);
    if bg != (*ctx).bg {
        screen_write_collect_flush(ctx, 1i32);
        (*ctx).bg = bg
    }
    if (*s).cy == (*s).rlower {
        grid_view_scroll_region_up(gd, (*s).rupper, (*s).rlower, bg);
        screen_write_collect_scroll(ctx);
        (*ctx).scrolled = (*ctx).scrolled.wrapping_add(1)
    } else if (*s).cy < (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint) {
        (*s).cy = (*s).cy.wrapping_add(1)
    };
}
unsafe extern "C" fn screen_write_collect_scroll(mut ctx:
                                                     *mut screen_write_ctx)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut cl: *mut screen_write_collect_line =
        0 as *mut screen_write_collect_line;
    let mut y: u_int = 0;
    log_debug(b"%s: at %u,%u (region %u-%u)\x00" as *const u8 as
                  *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 28],
                                        &[libc::c_char; 28]>(b"screen_write_collect_scroll\x00")).as_ptr(),
              (*s).cx, (*s).cy, (*s).rupper, (*s).rlower);
    screen_write_collect_clear(ctx, (*s).rupper, 1i32 as u_int);
    y = (*s).rupper;
    while y < (*s).rlower {
        cl =
            &mut *(*ctx).list.offset(y.wrapping_add(1i32 as libc::c_uint) as
                                         isize) as
                *mut screen_write_collect_line;
        loop  {
            if 0 !=
                   !((*(&mut (*cl).items as *mut unnamed_19)).tqh_first ==
                         0 as *mut libc::c_void as
                             *mut screen_write_collect_item) as libc::c_int {
                let ref mut fresh6 =
                    *(*(&mut (*(*ctx).list.offset(y as isize)).items as
                            *mut unnamed_19)).tqh_last;
                *fresh6 = (*(&mut (*cl).items as *mut unnamed_19)).tqh_first;
                let ref mut fresh7 =
                    (*(*(&mut (*cl).items as
                             *mut unnamed_19)).tqh_first).entry.tqe_prev;
                *fresh7 =
                    (*(&mut (*(*ctx).list.offset(y as isize)).items as
                           *mut unnamed_19)).tqh_last;
                let ref mut fresh8 =
                    (*(&mut (*(*ctx).list.offset(y as isize)).items as
                           *mut unnamed_19)).tqh_last;
                *fresh8 = (*(&mut (*cl).items as *mut unnamed_19)).tqh_last;
                loop  {
                    let ref mut fresh9 =
                        (*(&mut (*cl).items as *mut unnamed_19)).tqh_first;
                    *fresh9 = 0 as *mut screen_write_collect_item;
                    let ref mut fresh10 =
                        (*(&mut (*cl).items as *mut unnamed_19)).tqh_last;
                    *fresh10 =
                        &mut (*(&mut (*cl).items as
                                    *mut unnamed_19)).tqh_first as
                            *mut *mut screen_write_collect_item;
                    if !(0 != 0i32) { break ; }
                }
            }
            if !(0 != 0i32) { break ; }
        }
        loop  {
            let ref mut fresh11 =
                (*(&mut (*cl).items as *mut unnamed_19)).tqh_first;
            *fresh11 = 0 as *mut screen_write_collect_item;
            let ref mut fresh12 =
                (*(&mut (*cl).items as *mut unnamed_19)).tqh_last;
            *fresh12 =
                &mut (*(&mut (*cl).items as *mut unnamed_19)).tqh_first as
                    *mut *mut screen_write_collect_item;
            if !(0 != 0i32) { break ; }
        }
        y = y.wrapping_add(1)
    };
}
unsafe extern "C" fn screen_write_combine(mut ctx: *mut screen_write_ctx,
                                          mut ud: *const utf8_data,
                                          mut xx: *mut u_int)
 -> *const grid_cell {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    static mut gc: grid_cell =
        unsafe {
            grid_cell{flags: 0,
                      attr: 0,
                      fg: 0,
                      bg: 0,
                      data:
                          utf8_data{data: [0; 9],
                                    have: 0,
                                    size: 0,
                                    width: 0,},}
        };
    let mut n: u_int = 0;
    if (*s).cx == 0i32 as libc::c_uint {
        return 0 as *const grid_cell
    } else if (*ud).size as libc::c_int == 0i32 {
        fatalx(b"UTF-8 data empty\x00" as *const u8 as *const libc::c_char);
    } else {
        n = 1i32 as u_int;
        while n <= (*s).cx {
            grid_view_get_cell(gd, (*s).cx.wrapping_sub(n), (*s).cy,
                               &mut gc as *mut grid_cell);
            if 0 != !(gc.flags as libc::c_int) & 4i32 { break ; }
            n = n.wrapping_add(1)
        }
        if n > (*s).cx {
            return 0 as *const grid_cell
        } else {
            *xx = (*s).cx.wrapping_sub(n);
            if (gc.data.size as libc::c_int + (*ud).size as libc::c_int) as
                   libc::c_ulong >
                   ::std::mem::size_of::<[u_char; 9]>() as libc::c_ulong {
                return 0 as *const grid_cell
            } else {
                log_debug(b"%s: %.*s onto %.*s at %u,%u\x00" as *const u8 as
                              *const libc::c_char,
                          (*::std::mem::transmute::<&[u8; 21],
                                                    &[libc::c_char; 21]>(b"screen_write_combine\x00")).as_ptr(),
                          (*ud).size as libc::c_int, (*ud).data.as_ptr(),
                          gc.data.size as libc::c_int,
                          gc.data.data.as_mut_ptr(), *xx, (*s).cy);
                memcpy(gc.data.data.as_mut_ptr().offset(gc.data.size as
                                                            libc::c_int as
                                                            isize) as
                           *mut libc::c_void,
                       (*ud).data.as_ptr() as *const libc::c_void,
                       (*ud).size as libc::c_ulong);
                gc.data.size =
                    (gc.data.size as libc::c_int + (*ud).size as libc::c_int)
                        as u_char;
                grid_view_set_cell(gd, *xx, (*s).cy,
                                   &mut gc as *mut grid_cell);
                return &mut gc as *mut grid_cell
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_copy(mut ctx: *mut screen_write_ctx,
                                           mut src: *mut screen,
                                           mut px: u_int, mut py: u_int,
                                           mut nx: u_int, mut ny: u_int,
                                           mut mbs: *mut bitstr_t,
                                           mut mgc: *const grid_cell) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*src).grid;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut b: u_int = 0;
    if nx == 0i32 as libc::c_uint || ny == 0i32 as libc::c_uint {
        return
    } else {
        cx = (*s).cx;
        cy = (*s).cy;
        yy = py;
        while yy < py.wrapping_add(ny) {
            xx = px;
            while xx < px.wrapping_add(nx) {
                grid_get_cell(gd, xx, yy, &mut gc as *mut grid_cell);
                if mbs != 0 as *mut libc::c_void as *mut bitstr_t {
                    b = yy.wrapping_mul((*(*src).grid).sx).wrapping_add(xx);
                    if 0 !=
                           *mbs.offset((b >> 3i32) as isize) as libc::c_int &
                               1i32 << (b & 7i32 as libc::c_uint) {
                        gc.attr = (*mgc).attr;
                        gc.fg = (*mgc).fg;
                        gc.bg = (*mgc).bg
                    }
                }
                if xx.wrapping_add(gc.data.width as libc::c_uint) <=
                       px.wrapping_add(nx) {
                    screen_write_cell(ctx, &mut gc as *mut grid_cell);
                }
                xx = xx.wrapping_add(1)
            }
            cy = cy.wrapping_add(1);
            screen_write_cursormove(ctx, cx, cy);
            yy = yy.wrapping_add(1)
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_fast_copy(mut ctx:
                                                    *mut screen_write_ctx,
                                                mut src: *mut screen,
                                                mut px: u_int, mut py: u_int,
                                                mut nx: u_int, mut ny: u_int)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*src).grid;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    if nx == 0i32 as libc::c_uint || ny == 0i32 as libc::c_uint {
        return
    } else {
        cy = (*s).cy;
        yy = py;
        while yy < py.wrapping_add(ny) {
            if yy >= (*gd).hsize.wrapping_add((*gd).sy) { break ; }
            cx = (*s).cx;
            xx = px;
            while xx < px.wrapping_add(nx) {
                if xx >= (*(*gd).linedata.offset(yy as isize)).cellsize {
                    break ;
                }
                grid_get_cell(gd, xx, yy, &mut gc as *mut grid_cell);
                if xx.wrapping_add(gc.data.width as libc::c_uint) >
                       px.wrapping_add(nx) {
                    break ;
                }
                if 0 ==
                       grid_cells_equal(&mut gc as *mut grid_cell,
                                        &grid_default_cell as
                                            *const grid_cell) {
                    grid_view_set_cell((*(*ctx).s).grid, cx, cy,
                                       &mut gc as *mut grid_cell);
                }
                cx = cx.wrapping_add(1);
                xx = xx.wrapping_add(1)
            }
            cy = cy.wrapping_add(1);
            yy = yy.wrapping_add(1)
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_hline(mut ctx: *mut screen_write_ctx,
                                            mut nx: u_int,
                                            mut left: libc::c_int,
                                            mut right: libc::c_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut i: u_int = 0;
    cx = (*s).cx;
    cy = (*s).cy;
    memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    gc.attr = (gc.attr as libc::c_int | 128i32) as u_short;
    screen_write_putc(ctx, &mut gc as *mut grid_cell,
                      if 0 != left { 116 } else { 113 } as u_char);
    i = 1i32 as u_int;
    while i < nx.wrapping_sub(1i32 as libc::c_uint) {
        screen_write_putc(ctx, &mut gc as *mut grid_cell, 113 as u_char);
        i = i.wrapping_add(1)
    }
    screen_write_putc(ctx, &mut gc as *mut grid_cell,
                      if 0 != right { 117 } else { 113 } as u_char);
    screen_write_cursormove(ctx, cx, cy);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_vline(mut ctx: *mut screen_write_ctx,
                                            mut ny: u_int,
                                            mut top: libc::c_int,
                                            mut bottom: libc::c_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut i: u_int = 0;
    cx = (*s).cx;
    cy = (*s).cy;
    memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    gc.attr = (gc.attr as libc::c_int | 128i32) as u_short;
    screen_write_putc(ctx, &mut gc as *mut grid_cell,
                      if 0 != top { 119 } else { 120 } as u_char);
    i = 1i32 as u_int;
    while i < ny.wrapping_sub(1i32 as libc::c_uint) {
        screen_write_cursormove(ctx, cx, cy.wrapping_add(i));
        screen_write_putc(ctx, &mut gc as *mut grid_cell, 120 as u_char);
        i = i.wrapping_add(1)
    }
    screen_write_cursormove(ctx, cx,
                            cy.wrapping_add(ny).wrapping_sub(1i32 as
                                                                 libc::c_uint));
    screen_write_putc(ctx, &mut gc as *mut grid_cell,
                      if 0 != bottom { 118 } else { 120 } as u_char);
    screen_write_cursormove(ctx, cx, cy);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_box(mut ctx: *mut screen_write_ctx,
                                          mut nx: u_int, mut ny: u_int)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut i: u_int = 0;
    cx = (*s).cx;
    cy = (*s).cy;
    memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    gc.attr = (gc.attr as libc::c_int | 128i32) as u_short;
    screen_write_putc(ctx, &mut gc as *mut grid_cell, 108 as u_char);
    i = 1i32 as u_int;
    while i < nx.wrapping_sub(1i32 as libc::c_uint) {
        screen_write_putc(ctx, &mut gc as *mut grid_cell, 113 as u_char);
        i = i.wrapping_add(1)
    }
    screen_write_putc(ctx, &mut gc as *mut grid_cell, 107 as u_char);
    screen_write_cursormove(ctx, cx,
                            cy.wrapping_add(ny).wrapping_sub(1i32 as
                                                                 libc::c_uint));
    screen_write_putc(ctx, &mut gc as *mut grid_cell, 109 as u_char);
    i = 1i32 as u_int;
    while i < nx.wrapping_sub(1i32 as libc::c_uint) {
        screen_write_putc(ctx, &mut gc as *mut grid_cell, 113 as u_char);
        i = i.wrapping_add(1)
    }
    screen_write_putc(ctx, &mut gc as *mut grid_cell, 106 as u_char);
    i = 1i32 as u_int;
    while i < ny.wrapping_sub(1i32 as libc::c_uint) {
        screen_write_cursormove(ctx, cx, cy.wrapping_add(i));
        screen_write_putc(ctx, &mut gc as *mut grid_cell, 120 as u_char);
        i = i.wrapping_add(1)
    }
    i = 1i32 as u_int;
    while i < ny.wrapping_sub(1i32 as libc::c_uint) {
        screen_write_cursormove(ctx,
                                cx.wrapping_add(nx).wrapping_sub(1i32 as
                                                                     libc::c_uint),
                                cy.wrapping_add(i));
        screen_write_putc(ctx, &mut gc as *mut grid_cell, 120 as u_char);
        i = i.wrapping_add(1)
    }
    screen_write_cursormove(ctx, cx, cy);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_preview(mut ctx: *mut screen_write_ctx,
                                              mut src: *mut screen,
                                              mut nx: u_int, mut ny: u_int)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    cx = (*s).cx;
    cy = (*s).cy;
    if 0 != (*src).mode & 1i32 {
        px = (*src).cx;
        if px < nx.wrapping_div(3i32 as libc::c_uint) {
            px = 0i32 as u_int
        } else { px = px.wrapping_sub(nx.wrapping_div(3i32 as libc::c_uint)) }
        if px.wrapping_add(nx) > (*(*src).grid).sx {
            if nx > (*(*src).grid).sx {
                px = 0i32 as u_int
            } else { px = (*(*src).grid).sx.wrapping_sub(nx) }
        }
        py = (*src).cy;
        if py < ny.wrapping_div(3i32 as libc::c_uint) {
            py = 0i32 as u_int
        } else { py = py.wrapping_sub(ny.wrapping_div(3i32 as libc::c_uint)) }
        if py.wrapping_add(ny) > (*(*src).grid).sy {
            if ny > (*(*src).grid).sy {
                py = 0i32 as u_int
            } else { py = (*(*src).grid).sy.wrapping_sub(ny) }
        }
    } else { px = 0i32 as u_int; py = 0i32 as u_int }
    screen_write_fast_copy(ctx, src, px,
                           (*(*src).grid).hsize.wrapping_add(py), nx, ny);
    if 0 != (*src).mode & 1i32 {
        grid_view_get_cell((*src).grid, (*src).cx, (*src).cy,
                           &mut gc as *mut grid_cell);
        gc.attr = (gc.attr as libc::c_int | 16i32) as u_short;
        screen_write_cursormove(ctx,
                                cx.wrapping_add((*src).cx.wrapping_sub(px)),
                                cy.wrapping_add((*src).cy.wrapping_sub(py)));
        screen_write_cell(ctx, &mut gc as *mut grid_cell);
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_backspace(mut ctx:
                                                    *mut screen_write_ctx)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    if (*s).cx == 0i32 as libc::c_uint {
        if (*s).cy == 0i32 as libc::c_uint {
            return
        } else {
            gl =
                &mut *(*(*s).grid).linedata.offset((*(*s).grid).hsize.wrapping_add((*s).cy).wrapping_sub(1i32
                                                                                                             as
                                                                                                             libc::c_uint)
                                                       as isize) as
                    *mut grid_line;
            if 0 != (*gl).flags & 1i32 {
                (*s).cy = (*s).cy.wrapping_sub(1);
                (*s).cx = (*(*s).grid).sx.wrapping_sub(1i32 as libc::c_uint)
            }
        }
    } else { (*s).cx = (*s).cx.wrapping_sub(1) };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_mode_set(mut ctx: *mut screen_write_ctx,
                                               mut mode: libc::c_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    (*s).mode |= mode;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_mode_clear(mut ctx:
                                                     *mut screen_write_ctx,
                                                 mut mode: libc::c_int)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    (*s).mode &= !mode;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursorup(mut ctx: *mut screen_write_ctx,
                                               mut ny: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    if ny == 0i32 as libc::c_uint { ny = 1i32 as u_int }
    if (*s).cy < (*s).rupper {
        if ny > (*s).cy { ny = (*s).cy }
    } else if ny > (*s).cy.wrapping_sub((*s).rupper) {
        ny = (*s).cy.wrapping_sub((*s).rupper)
    }
    if (*s).cx == (*(*s).grid).sx { (*s).cx = (*s).cx.wrapping_sub(1) }
    if ny == 0i32 as libc::c_uint {
        return
    } else {
        (*s).cy =
            ((*s).cy as libc::c_uint).wrapping_sub(ny) as u_int as u_int;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursordown(mut ctx:
                                                     *mut screen_write_ctx,
                                                 mut ny: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    if ny == 0i32 as libc::c_uint { ny = 1i32 as u_int }
    if (*s).cy > (*s).rlower {
        if ny >
               (*(*s).grid).sy.wrapping_sub(1i32 as
                                                libc::c_uint).wrapping_sub((*s).cy)
           {
            ny =
                (*(*s).grid).sy.wrapping_sub(1i32 as
                                                 libc::c_uint).wrapping_sub((*s).cy)
        }
    } else if ny > (*s).rlower.wrapping_sub((*s).cy) {
        ny = (*s).rlower.wrapping_sub((*s).cy)
    }
    if (*s).cx == (*(*s).grid).sx { (*s).cx = (*s).cx.wrapping_sub(1) }
    if ny == 0i32 as libc::c_uint {
        return
    } else {
        (*s).cy =
            ((*s).cy as libc::c_uint).wrapping_add(ny) as u_int as u_int;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursorright(mut ctx:
                                                      *mut screen_write_ctx,
                                                  mut nx: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    if nx == 0i32 as libc::c_uint { nx = 1i32 as u_int }
    if nx >
           (*(*s).grid).sx.wrapping_sub(1i32 as
                                            libc::c_uint).wrapping_sub((*s).cx)
       {
        nx =
            (*(*s).grid).sx.wrapping_sub(1i32 as
                                             libc::c_uint).wrapping_sub((*s).cx)
    }
    if nx == 0i32 as libc::c_uint {
        return
    } else {
        (*s).cx =
            ((*s).cx as libc::c_uint).wrapping_add(nx) as u_int as u_int;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursorleft(mut ctx:
                                                     *mut screen_write_ctx,
                                                 mut nx: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    if nx == 0i32 as libc::c_uint { nx = 1i32 as u_int }
    if nx > (*s).cx { nx = (*s).cx }
    if nx == 0i32 as libc::c_uint {
        return
    } else {
        (*s).cx =
            ((*s).cx as libc::c_uint).wrapping_sub(nx) as u_int as u_int;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_alignmenttest(mut ctx:
                                                        *mut screen_write_ctx)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
    memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    utf8_set(&mut gc.data as *mut utf8_data, 69 as u_char);
    yy = 0i32 as u_int;
    while yy < (*(*s).grid).sy {
        xx = 0i32 as u_int;
        while xx < (*(*s).grid).sx {
            grid_view_set_cell((*s).grid, xx, yy, &mut gc as *mut grid_cell);
            xx = xx.wrapping_add(1)
        }
        yy = yy.wrapping_add(1)
    }
    (*s).cx = 0i32 as u_int;
    (*s).cy = 0i32 as u_int;
    (*s).rupper = 0i32 as u_int;
    (*s).rlower = (*(*s).grid).sy.wrapping_sub(1i32 as libc::c_uint);
    screen_write_collect_clear(ctx, 0i32 as u_int,
                               (*(*s).grid).sy.wrapping_sub(1i32 as
                                                                libc::c_uint));
    tty_write(Some(tty_cmd_alignmenttest), &mut ttyctx as *mut tty_ctx);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_insertcharacter(mut ctx:
                                                          *mut screen_write_ctx,
                                                      mut nx: u_int,
                                                      mut bg: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    if nx == 0i32 as libc::c_uint { nx = 1i32 as u_int }
    if nx > (*(*s).grid).sx.wrapping_sub((*s).cx) {
        nx = (*(*s).grid).sx.wrapping_sub((*s).cx)
    }
    if nx == 0i32 as libc::c_uint {
        return
    } else if (*s).cx > (*(*s).grid).sx.wrapping_sub(1i32 as libc::c_uint) {
        return
    } else {
        screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
        ttyctx.bg = bg;
        grid_view_insert_cells((*s).grid, (*s).cx, (*s).cy, nx, bg);
        screen_write_collect_flush(ctx, 0i32);
        ttyctx.num = nx;
        tty_write(Some(tty_cmd_insertcharacter), &mut ttyctx as *mut tty_ctx);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_deletecharacter(mut ctx:
                                                          *mut screen_write_ctx,
                                                      mut nx: u_int,
                                                      mut bg: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    if nx == 0i32 as libc::c_uint { nx = 1i32 as u_int }
    if nx > (*(*s).grid).sx.wrapping_sub((*s).cx) {
        nx = (*(*s).grid).sx.wrapping_sub((*s).cx)
    }
    if nx == 0i32 as libc::c_uint {
        return
    } else if (*s).cx > (*(*s).grid).sx.wrapping_sub(1i32 as libc::c_uint) {
        return
    } else {
        screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
        ttyctx.bg = bg;
        grid_view_delete_cells((*s).grid, (*s).cx, (*s).cy, nx, bg);
        screen_write_collect_flush(ctx, 0i32);
        ttyctx.num = nx;
        tty_write(Some(tty_cmd_deletecharacter), &mut ttyctx as *mut tty_ctx);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearcharacter(mut ctx:
                                                         *mut screen_write_ctx,
                                                     mut nx: u_int,
                                                     mut bg: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    if nx == 0i32 as libc::c_uint { nx = 1i32 as u_int }
    if nx > (*(*s).grid).sx.wrapping_sub((*s).cx) {
        nx = (*(*s).grid).sx.wrapping_sub((*s).cx)
    }
    if nx == 0i32 as libc::c_uint {
        return
    } else if (*s).cx > (*(*s).grid).sx.wrapping_sub(1i32 as libc::c_uint) {
        return
    } else {
        screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
        ttyctx.bg = bg;
        grid_view_clear((*s).grid, (*s).cx, (*s).cy, nx, 1i32 as u_int, bg);
        screen_write_collect_flush(ctx, 0i32);
        ttyctx.num = nx;
        tty_write(Some(tty_cmd_clearcharacter), &mut ttyctx as *mut tty_ctx);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_insertline(mut ctx:
                                                     *mut screen_write_ctx,
                                                 mut ny: u_int, mut bg: u_int)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    if ny == 0i32 as libc::c_uint { ny = 1i32 as u_int }
    if (*s).cy < (*s).rupper || (*s).cy > (*s).rlower {
        if ny > (*(*s).grid).sy.wrapping_sub((*s).cy) {
            ny = (*(*s).grid).sy.wrapping_sub((*s).cy)
        }
        if ny == 0i32 as libc::c_uint {
            return
        } else {
            screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
            ttyctx.bg = bg;
            grid_view_insert_lines(gd, (*s).cy, ny, bg);
            screen_write_collect_flush(ctx, 0i32);
            ttyctx.num = ny;
            tty_write(Some(tty_cmd_insertline), &mut ttyctx as *mut tty_ctx);
            return
        }
    } else {
        if ny >
               (*s).rlower.wrapping_add(1i32 as
                                            libc::c_uint).wrapping_sub((*s).cy)
           {
            ny =
                (*s).rlower.wrapping_add(1i32 as
                                             libc::c_uint).wrapping_sub((*s).cy)
        }
        if ny == 0i32 as libc::c_uint {
            return
        } else {
            screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
            ttyctx.bg = bg;
            if (*s).cy < (*s).rupper || (*s).cy > (*s).rlower {
                grid_view_insert_lines(gd, (*s).cy, ny, bg);
            } else {
                grid_view_insert_lines_region(gd, (*s).rlower, (*s).cy, ny,
                                              bg);
            }
            screen_write_collect_flush(ctx, 0i32);
            ttyctx.num = ny;
            tty_write(Some(tty_cmd_insertline), &mut ttyctx as *mut tty_ctx);
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_deleteline(mut ctx:
                                                     *mut screen_write_ctx,
                                                 mut ny: u_int, mut bg: u_int)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    if ny == 0i32 as libc::c_uint { ny = 1i32 as u_int }
    if (*s).cy < (*s).rupper || (*s).cy > (*s).rlower {
        if ny > (*(*s).grid).sy.wrapping_sub((*s).cy) {
            ny = (*(*s).grid).sy.wrapping_sub((*s).cy)
        }
        if ny == 0i32 as libc::c_uint {
            return
        } else {
            screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
            ttyctx.bg = bg;
            grid_view_delete_lines(gd, (*s).cy, ny, bg);
            screen_write_collect_flush(ctx, 0i32);
            ttyctx.num = ny;
            tty_write(Some(tty_cmd_deleteline), &mut ttyctx as *mut tty_ctx);
            return
        }
    } else {
        if ny >
               (*s).rlower.wrapping_add(1i32 as
                                            libc::c_uint).wrapping_sub((*s).cy)
           {
            ny =
                (*s).rlower.wrapping_add(1i32 as
                                             libc::c_uint).wrapping_sub((*s).cy)
        }
        if ny == 0i32 as libc::c_uint {
            return
        } else {
            screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
            ttyctx.bg = bg;
            if (*s).cy < (*s).rupper || (*s).cy > (*s).rlower {
                grid_view_delete_lines(gd, (*s).cy, ny, bg);
            } else {
                grid_view_delete_lines_region(gd, (*s).rlower, (*s).cy, ny,
                                              bg);
            }
            screen_write_collect_flush(ctx, 0i32);
            ttyctx.num = ny;
            tty_write(Some(tty_cmd_deleteline), &mut ttyctx as *mut tty_ctx);
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearline(mut ctx:
                                                    *mut screen_write_ctx,
                                                mut bg: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    let mut sx: u_int = (*(*s).grid).sx;
    gl =
        &mut *(*(*s).grid).linedata.offset((*(*s).grid).hsize.wrapping_add((*s).cy)
                                               as isize) as *mut grid_line;
    if (*gl).cellsize == 0i32 as libc::c_uint && bg == 8i32 as libc::c_uint {
        return
    } else {
        screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
        ttyctx.bg = bg;
        grid_view_clear((*s).grid, 0i32 as u_int, (*s).cy, sx, 1i32 as u_int,
                        bg);
        screen_write_collect_clear(ctx, (*s).cy, 1i32 as u_int);
        screen_write_collect_flush(ctx, 0i32);
        tty_write(Some(tty_cmd_clearline), &mut ttyctx as *mut tty_ctx);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearendofline(mut ctx:
                                                         *mut screen_write_ctx,
                                                     mut bg: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    let mut sx: u_int = (*(*s).grid).sx;
    gl =
        &mut *(*(*s).grid).linedata.offset((*(*s).grid).hsize.wrapping_add((*s).cy)
                                               as isize) as *mut grid_line;
    if (*s).cx > sx.wrapping_sub(1i32 as libc::c_uint) ||
           (*s).cx >= (*gl).cellsize && bg == 8i32 as libc::c_uint {
        return
    } else {
        screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
        ttyctx.bg = bg;
        grid_view_clear((*s).grid, (*s).cx, (*s).cy, sx.wrapping_sub((*s).cx),
                        1i32 as u_int, bg);
        if (*s).cx == 0i32 as libc::c_uint {
            screen_write_collect_clear(ctx, (*s).cy, 1i32 as u_int);
        }
        screen_write_collect_flush(ctx, 0i32);
        tty_write(Some(tty_cmd_clearendofline), &mut ttyctx as *mut tty_ctx);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearstartofline(mut ctx:
                                                           *mut screen_write_ctx,
                                                       mut bg: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    let mut sx: u_int = (*(*s).grid).sx;
    screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
    ttyctx.bg = bg;
    if (*s).cx > sx.wrapping_sub(1i32 as libc::c_uint) {
        grid_view_clear((*s).grid, 0i32 as u_int, (*s).cy, sx, 1i32 as u_int,
                        bg);
    } else {
        grid_view_clear((*s).grid, 0i32 as u_int, (*s).cy,
                        (*s).cx.wrapping_add(1i32 as libc::c_uint),
                        1i32 as u_int, bg);
    }
    if (*s).cx > sx.wrapping_sub(1i32 as libc::c_uint) {
        screen_write_collect_clear(ctx, (*s).cy, 1i32 as u_int);
    }
    screen_write_collect_flush(ctx, 0i32);
    tty_write(Some(tty_cmd_clearstartofline), &mut ttyctx as *mut tty_ctx);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_reverseindex(mut ctx:
                                                       *mut screen_write_ctx,
                                                   mut bg: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
    ttyctx.bg = bg;
    if (*s).cy == (*s).rupper {
        grid_view_scroll_region_down((*s).grid, (*s).rupper, (*s).rlower, bg);
    } else if (*s).cy > 0i32 as libc::c_uint {
        (*s).cy = (*s).cy.wrapping_sub(1)
    }
    screen_write_collect_flush(ctx, 0i32);
    tty_write(Some(tty_cmd_reverseindex), &mut ttyctx as *mut tty_ctx);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_scrollup(mut ctx: *mut screen_write_ctx,
                                               mut lines: u_int,
                                               mut bg: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut i: u_int = 0;
    if lines == 0i32 as libc::c_uint {
        lines = 1i32 as u_int
    } else if lines >
                  (*s).rlower.wrapping_sub((*s).rupper).wrapping_add(1i32 as
                                                                         libc::c_uint)
     {
        lines =
            (*s).rlower.wrapping_sub((*s).rupper).wrapping_add(1i32 as
                                                                   libc::c_uint)
    }
    if bg != (*ctx).bg {
        screen_write_collect_flush(ctx, 1i32);
        (*ctx).bg = bg
    }
    i = 0i32 as u_int;
    while i < lines {
        grid_view_scroll_region_up(gd, (*s).rupper, (*s).rlower, bg);
        screen_write_collect_scroll(ctx);
        i = i.wrapping_add(1)
    }
    (*ctx).scrolled =
        ((*ctx).scrolled as libc::c_uint).wrapping_add(lines) as u_int as
            u_int;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_carriagereturn(mut ctx:
                                                         *mut screen_write_ctx)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    (*s).cx = 0i32 as u_int;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearendofscreen(mut ctx:
                                                           *mut screen_write_ctx,
                                                       mut bg: u_int) -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    let mut sx: u_int = (*(*s).grid).sx;
    let mut sy: u_int = (*(*s).grid).sy;
    screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
    ttyctx.bg = bg;
    if (*s).cx == 0i32 as libc::c_uint && (*s).cy == 0i32 as libc::c_uint &&
           0 != (*gd).flags & 1i32 {
        grid_view_clear_history(gd, bg);
    } else {
        if (*s).cx <= sx.wrapping_sub(1i32 as libc::c_uint) {
            grid_view_clear(gd, (*s).cx, (*s).cy, sx.wrapping_sub((*s).cx),
                            1i32 as u_int, bg);
        }
        grid_view_clear(gd, 0i32 as u_int,
                        (*s).cy.wrapping_add(1i32 as libc::c_uint), sx,
                        sy.wrapping_sub((*s).cy.wrapping_add(1i32 as
                                                                 libc::c_uint)),
                        bg);
    }
    screen_write_collect_clear(ctx,
                               (*s).cy.wrapping_add(1i32 as libc::c_uint),
                               sy.wrapping_sub((*s).cy.wrapping_add(1i32 as
                                                                        libc::c_uint)));
    screen_write_collect_flush(ctx, 0i32);
    tty_write(Some(tty_cmd_clearendofscreen), &mut ttyctx as *mut tty_ctx);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearstartofscreen(mut ctx:
                                                             *mut screen_write_ctx,
                                                         mut bg: u_int)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    let mut sx: u_int = (*(*s).grid).sx;
    screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
    ttyctx.bg = bg;
    if (*s).cy > 0i32 as libc::c_uint {
        grid_view_clear((*s).grid, 0i32 as u_int, 0i32 as u_int, sx, (*s).cy,
                        bg);
    }
    if (*s).cx > sx.wrapping_sub(1i32 as libc::c_uint) {
        grid_view_clear((*s).grid, 0i32 as u_int, (*s).cy, sx, 1i32 as u_int,
                        bg);
    } else {
        grid_view_clear((*s).grid, 0i32 as u_int, (*s).cy,
                        (*s).cx.wrapping_add(1i32 as libc::c_uint),
                        1i32 as u_int, bg);
    }
    screen_write_collect_clear(ctx, 0i32 as u_int, (*s).cy);
    screen_write_collect_flush(ctx, 0i32);
    tty_write(Some(tty_cmd_clearstartofscreen), &mut ttyctx as *mut tty_ctx);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearhistory(mut ctx:
                                                       *mut screen_write_ctx)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    grid_move_lines(gd, 0i32 as u_int, (*gd).hsize, (*gd).sy, 8i32 as u_int);
    (*gd).hsize = 0i32 as u_int;
    (*gd).hscrolled = (*gd).hsize;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_collect_add(mut ctx:
                                                      *mut screen_write_ctx,
                                                  mut gc: *const grid_cell)
 -> () {
    let mut s: *mut screen = (*ctx).s;
    let mut ci: *mut screen_write_collect_item =
        0 as *mut screen_write_collect_item;
    let mut sx: u_int = (*(*s).grid).sx;
    let mut collect: libc::c_int = 0;
    collect = 1i32;
    if (*gc).data.width as libc::c_int != 1i32 ||
           (*gc).data.size as libc::c_int != 1i32 ||
           *(*gc).data.data.as_ptr() as libc::c_int >= 127i32 {
        collect = 0i32
    } else if 0 != (*gc).attr as libc::c_int & 128i32 {
        collect = 0i32
    } else if 0 != !(*s).mode & 16i32 {
        collect = 0i32
    } else if 0 != (*s).mode & 2i32 {
        collect = 0i32
    } else if 0 != (*s).sel.flag { collect = 0i32 }
    if 0 == collect {
        screen_write_collect_end(ctx);
        screen_write_collect_flush(ctx, 0i32);
        screen_write_cell(ctx, gc);
        return
    } else {
        (*ctx).cells = (*ctx).cells.wrapping_add(1);
        if (*s).cx > sx.wrapping_sub(1i32 as libc::c_uint) ||
               (*(*ctx).item).used >
                   sx.wrapping_sub(1i32 as libc::c_uint).wrapping_sub((*s).cx)
           {
            screen_write_collect_end(ctx);
        }
        ci = (*ctx).item;
        if (*s).cx > sx.wrapping_sub(1i32 as libc::c_uint) {
            log_debug(b"%s: wrapped at %u,%u\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"screen_write_collect_add\x00")).as_ptr(),
                      (*s).cx, (*s).cy);
            (*ci).wrapped = 1i32;
            screen_write_linefeed(ctx, 1i32, 8i32 as u_int);
            (*s).cx = 0i32 as u_int
        }
        if (*ci).used == 0i32 as libc::c_uint {
            memcpy(&mut (*ci).gc as *mut grid_cell as *mut libc::c_void,
                   gc as *const libc::c_void,
                   ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        }
        let fresh13 = (*ci).used;
        (*ci).used = (*ci).used.wrapping_add(1);
        (*ci).data[fresh13 as usize] =
            (*gc).data.data[0usize] as libc::c_char;
        if (*ci).used as libc::c_ulong ==
               (::std::mem::size_of::<[libc::c_char; 256]>() as
                    libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) {
            screen_write_collect_end(ctx);
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_setselection(mut ctx:
                                                       *mut screen_write_ctx,
                                                   mut str: *mut u_char,
                                                   mut len: u_int) -> () {
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
    ttyctx.ptr = str as *mut libc::c_void;
    ttyctx.num = len;
    tty_write(Some(tty_cmd_setselection), &mut ttyctx as *mut tty_ctx);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_rawstring(mut ctx:
                                                    *mut screen_write_ctx,
                                                mut str: *mut u_char,
                                                mut len: u_int) -> () {
    let mut ttyctx: tty_ctx =
        tty_ctx{wp: 0 as *mut window_pane,
                cell: 0 as *const grid_cell,
                wrapped: 0,
                num: 0,
                ptr: 0 as *mut libc::c_void,
                ocx: 0,
                ocy: 0,
                orupper: 0,
                orlower: 0,
                xoff: 0,
                yoff: 0,
                bg: 0,};
    screen_write_initctx(ctx, &mut ttyctx as *mut tty_ctx);
    ttyctx.ptr = str as *mut libc::c_void;
    ttyctx.num = len;
    tty_write(Some(tty_cmd_rawstring), &mut ttyctx as *mut tty_ctx);
}

