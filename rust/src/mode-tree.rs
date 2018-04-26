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
    pub type format_job_tree;
    pub type tmuxproc;
    pub type hooks;
    pub type bufferevent_ops;
    pub type environ;
    pub type args_entry;
    pub type event_base;
    pub type screen_write_collect_item;
    pub type tty_code;
    pub type evbuffer;
    pub type format_tree;
    pub type _IO_FILE_plus;
    pub type screen_write_collect_line;
    pub type tmuxpeer;
    pub type options;
    pub type input_ctx;
    pub type screen_titles;
    #[no_mangle]
    fn toupper(_: libc::c_int) -> libc::c_int;
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
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    static in6addr_any: in6_addr;
    #[no_mangle]
    static in6addr_loopback: in6_addr;
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
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
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
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
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_mouse_at(_: *mut window_pane, _: *mut mouse_event, _: *mut u_int,
                    _: *mut u_int, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cmd_template_replace(_: *const libc::c_char, _: *const libc::c_char,
                            _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn cmd_list_free(_: *mut cmd_list) -> ();
    #[no_mangle]
    fn cmdq_get_command(_: *mut cmd_list, _: *mut cmd_find_state,
                        _: *mut mouse_event, _: libc::c_int)
     -> *mut cmdq_item;
    #[no_mangle]
    fn cmdq_append(_: *mut client, _: *mut cmdq_item) -> ();
    #[no_mangle]
    fn cmd_string_parse(_: *const libc::c_char, _: *const libc::c_char,
                        _: u_int, _: *mut *mut libc::c_char) -> *mut cmd_list;
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    fn server_redraw_window(_: *mut window) -> ();
    #[no_mangle]
    fn server_unzoom_window(_: *mut window) -> ();
    #[no_mangle]
    fn status_message_set(_: *mut client, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn status_prompt_set(_: *mut client, _: *const libc::c_char,
                         _: *const libc::c_char, _: prompt_input_cb,
                         _: prompt_free_cb, _: *mut libc::c_void,
                         _: libc::c_int) -> ();
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut window_pane,
                          _: *mut screen) -> ();
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx) -> ();
    #[no_mangle]
    fn screen_write_puts(_: *mut screen_write_ctx, _: *const grid_cell,
                         _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn screen_write_nputs(_: *mut screen_write_ctx, _: ssize_t,
                          _: *const grid_cell, _: *const libc::c_char, ...)
     -> ();
    #[no_mangle]
    fn screen_write_box(_: *mut screen_write_ctx, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn screen_write_clearendofline(_: *mut screen_write_ctx, _: u_int) -> ();
    #[no_mangle]
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn screen_write_clearscreen(_: *mut screen_write_ctx, _: u_int) -> ();
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn screen_free(_: *mut screen) -> ();
    #[no_mangle]
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int)
     -> ();
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn window_zoom(_: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn style_apply(_: *mut grid_cell, _: *mut options, _: *const libc::c_char)
     -> ();
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
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
pub const KEYC_MOUSEDRAG2_PANE: unnamed_14 = 268435486;
pub const KEYC_DOUBLECLICK2_STATUS: unnamed_14 = 268435511;
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
    pub gentry: unnamed_33,
    pub entry: unnamed_24,
}
pub const KEYC_KP_MINUS: unnamed_14 = 268435551;
pub type mode_tree_draw_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: *mut screen_write_ctx, _: u_int, _: u_int)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const KEYC_MOUSEDRAGEND3_STATUS: unnamed_14 = 268435499;
pub const KEYC_DC: unnamed_14 = 268435539;
pub type _IO_lock_t = ();
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_10,
    pub entry: unnamed_23,
}
pub const KEYC_WHEELDOWN_BORDER: unnamed_14 = 268435506;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
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
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub ev_signal_next: unnamed_25,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const KEYC_TRIPLECLICK1_STATUS: unnamed_14 = 268435517;
pub const KEYC_DOUBLECLICK1_STATUS: unnamed_14 = 268435508;
pub const KEYC_F5: unnamed_14 = 268435530;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
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
pub const KEYC_MOUSEUP2_PANE: unnamed_14 = 268435477;
pub const KEYC_MOUSEDOWN2_PANE: unnamed_14 = 268435468;
pub const CMDQ_COMMAND: cmdq_type = 0;
pub const KEYC_KP_SIX: unnamed_14 = 268435558;
pub const KEYC_KP_FIVE: unnamed_14 = 268435557;
pub const KEYC_DOUBLECLICK3_PANE: unnamed_14 = 268435513;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub const KEYC_KP_ONE: unnamed_14 = 268435559;
pub type uint32_t = libc::c_uint;
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub type uint8_t = libc::c_uchar;
pub const PROMPT_ENTRY: unnamed_22 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct mode_tree_data {
    pub dead: libc::c_int,
    pub references: u_int,
    pub zoomed: libc::c_int,
    pub wp: *mut window_pane,
    pub modedata: *mut libc::c_void,
    pub sort_list: *mut *const libc::c_char,
    pub sort_size: u_int,
    pub sort_type: u_int,
    pub buildcb: mode_tree_build_cb,
    pub drawcb: mode_tree_draw_cb,
    pub searchcb: mode_tree_search_cb,
    pub children: mode_tree_list,
    pub saved: mode_tree_list,
    pub line_list: *mut mode_tree_line,
    pub line_size: u_int,
    pub depth: u_int,
    pub width: u_int,
    pub height: u_int,
    pub offset: u_int,
    pub current: u_int,
    pub screen: screen,
    pub preview: libc::c_int,
    pub search: *mut libc::c_char,
    pub filter: *mut libc::c_char,
    pub no_matches: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_6 {
    offset: u_int,
    data: unnamed_38,
}
pub const KEYC_TRIPLECLICK2_PANE: unnamed_14 = 268435519;
pub const KEYC_F11: unnamed_14 = 268435536;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const KEYC_MOUSEUP1_BORDER: unnamed_14 = 268435476;
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
pub type cmd_retval = libc::c_int;
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
pub struct unnamed_8 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub const KEYC_DOUBLECLICK2_BORDER: unnamed_14 = 268435512;
pub const KEYC_TRIPLECLICK2_STATUS: unnamed_14 = 268435520;
pub const KEYC_BSPACE: unnamed_14 = 268435525;
pub const TTY_VT101: unnamed_30 = 1;
pub const KEYC_MOUSEUP2_BORDER: unnamed_14 = 268435479;
pub const KEYC_DOUBLECLICK2_PANE: unnamed_14 = 268435510;
pub type __time_t = libc::c_long;
pub const KEYC_F1: unnamed_14 = 268435526;
pub const KEYC_TRIPLECLICK3_STATUS: unnamed_14 = 268435523;
pub const KEYC_KP_THREE: unnamed_14 = 268435561;
pub const KEYC_MOUSEUP1_PANE: unnamed_14 = 268435474;
pub const JOB_DEAD: unnamed_20 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub ev_io_next: unnamed_15,
    pub ev_timeout: timeval,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct mode_tree_line {
    pub item: *mut mode_tree_item,
    pub depth: u_int,
    pub last: libc::c_int,
    pub flat: libc::c_int,
}
pub const KEYC_WHEELDOWN_STATUS: unnamed_14 = 268435505;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const JOB_RUNNING: unnamed_20 = 0;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const KEYC_MOUSEDRAG1_STATUS: unnamed_14 = 268435484;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const TTY_UNKNOWN: unnamed_30 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const KEYC_MOUSEDRAGEND3_BORDER: unnamed_14 = 268435500;
pub const KEYC_DOUBLECLICK3_STATUS: unnamed_14 = 268435514;
pub type u_int = __u_int;
pub type key_code = libc::c_ulonglong;
pub const KEYC_FOCUS_OUT: unnamed_14 = 268435457;
pub const KEYC_MOUSEMOVE_STATUS: unnamed_14 = 268435463;
pub type options_table_type = libc::c_uint;
pub const KEYC_F12: unnamed_14 = 268435537;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const KEYC_HOME: unnamed_14 = 268435540;
pub const KEYC_MOUSEDRAG3_BORDER: unnamed_14 = 268435491;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const KEYC_MOUSEUP3_STATUS: unnamed_14 = 268435481;
pub const KEYC_WHEELDOWN_PANE: unnamed_14 = 268435504;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub const TTY_VT100: unnamed_30 = 0;
pub const KEYC_F7: unnamed_14 = 268435532;
pub type __suseconds_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub const KEYC_F9: unnamed_14 = 268435534;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const KEYC_MOUSEDRAG1_PANE: unnamed_14 = 268435483;
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
    pub alerts_entry: unnamed_4,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_32,
    pub entry: unnamed_1,
}
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
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
    pub entry: unnamed_0,
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
    pub term_type: unnamed_30,
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
pub type mode_tree_search_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: *const libc::c_char) -> libc::c_int>;
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut mode_tree_item,
    pub tqe_prev: *mut *mut mode_tree_item,
}
pub const KEYC_UP: unnamed_14 = 268435545;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_2,
}
pub const KEYC_DOUBLECLICK1_PANE: unnamed_14 = 268435507;
pub const KEYC_MOUSEDOWN3_BORDER: unnamed_14 = 268435473;
pub const KEYC_MOUSEDRAGEND1_PANE: unnamed_14 = 268435492;
pub const KEYC_MOUSEUP3_PANE: unnamed_14 = 268435480;
pub const KEYC_MOUSEDOWN2_STATUS: unnamed_14 = 268435469;
pub const KEYC_KP_ENTER: unnamed_14 = 268435562;
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
pub type unnamed_14 = libc::c_uint;
pub type uint64_t = libc::c_ulong;
pub const LINE_SEL_RIGHT_LEFT: unnamed_36 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const TTY_VT220: unnamed_30 = 3;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const KEYC_KP_NINE: unnamed_14 = 268435554;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const KEYC_WHEELUP_BORDER: unnamed_14 = 268435503;
pub const KEYC_PPAGE: unnamed_14 = 268435543;
pub const KEYC_KP_PERIOD: unnamed_14 = 268435564;
pub const JOB_CLOSED: unnamed_20 = 2;
pub const KEYC_PASTE_START: unnamed_14 = 268435458;
pub const KEYC_RIGHT: unnamed_14 = 268435548;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct mode_tree_item {
    pub parent: *mut mode_tree_item,
    pub itemdata: *mut libc::c_void,
    pub line: u_int,
    pub tag: uint64_t,
    pub name: *const libc::c_char,
    pub text: *const libc::c_char,
    pub expanded: libc::c_int,
    pub tagged: libc::c_int,
    pub children: mode_tree_list,
    pub entry: unnamed_13,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct mode_tree_list {
    pub tqh_first: *mut mode_tree_item,
    pub tqh_last: *mut *mut mode_tree_item,
}
pub const KEYC_MOUSEDRAGEND1_BORDER: unnamed_14 = 268435494;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTY_VT420: unnamed_30 = 5;
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
pub const KEYC_TRIPLECLICK3_PANE: unnamed_14 = 268435522;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
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
    pub ev_active_next: unnamed_37,
    pub ev_next: unnamed_7,
    pub ev_timeout_pos: unnamed_17,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_31,
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
pub struct unnamed_16 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type bitstr_t = libc::c_uchar;
pub const LINE_SEL_LEFT_RIGHT: unnamed_36 = 1;
pub const KEYC_KP_STAR: unnamed_14 = 268435550;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub type __ssize_t = libc::c_long;
pub const KEYC_MOUSEUP1_STATUS: unnamed_14 = 268435475;
pub const KEYC_F6: unnamed_14 = 268435531;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_17 {
    ev_next_with_common_timeout: unnamed_34,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_36,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type layout_type = libc::c_uint;
pub const TTY_VT102: unnamed_30 = 2;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const KEYC_KP_EIGHT: unnamed_14 = 268435553;
pub const KEYC_MOUSEDRAGEND3_PANE: unnamed_14 = 268435498;
pub const KEYC_MOUSEDOWN3_PANE: unnamed_14 = 268435471;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const KEYC_LEFT: unnamed_14 = 268435547;
pub const TTY_VT320: unnamed_30 = 4;
pub const KEYC_KP_TWO: unnamed_14 = 268435560;
pub type __u_char = libc::c_uchar;
pub const KEYC_F3: unnamed_14 = 268435528;
pub const KEYC_MOUSEMOVE_PANE: unnamed_14 = 268435462;
pub type u_char = __u_char;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub type unnamed_20 = libc::c_uint;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const KEYC_KP_SEVEN: unnamed_14 = 268435552;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_27,
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
    pub entry: unnamed_21,
}
pub const KEYC_END: unnamed_14 = 268435541;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub type unnamed_22 = libc::c_uint;
pub const KEYC_DRAGGING: unnamed_14 = 268435461;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub const KEYC_DOUBLECLICK3_BORDER: unnamed_14 = 268435515;
pub type __off_t = libc::c_long;
pub const KEYC_TRIPLECLICK1_PANE: unnamed_14 = 268435516;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const KEYC_KP_FOUR: unnamed_14 = 268435556;
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
    pub entry: unnamed_26,
    pub tree_entry: unnamed_12,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_29,
}
pub const KEYC_TRIPLECLICK1_BORDER: unnamed_14 = 268435518;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const KEYC_MOUSEDRAGEND2_BORDER: unnamed_14 = 268435497;
pub type pid_t = __pid_t;
pub const KEYC_MOUSE: unnamed_14 = 268435460;
pub type tcflag_t = libc::c_uint;
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
pub struct unnamed_28 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const KEYC_MOUSEDRAGEND1_STATUS: unnamed_14 = 268435493;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_5,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const KEYC_F8: unnamed_14 = 268435533;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type cc_t = libc::c_uchar;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
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
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const KEYC_MOUSEDRAGEND2_PANE: unnamed_14 = 268435495;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub type unnamed_30 = libc::c_uint;
pub const KEYC_KP_PLUS: unnamed_14 = 268435555;
pub const KEYC_MOUSEMOVE_BORDER: unnamed_14 = 268435464;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
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
    pub entry: unnamed_40,
    pub wentry: unnamed_39,
    pub sentry: unnamed_35,
}
pub const KEYC_BTAB: unnamed_14 = 268435544;
pub type size_t = libc::c_ulong;
pub const KEYC_MOUSEDRAGEND2_STATUS: unnamed_14 = 268435496;
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_18,
}
pub const KEYC_F4: unnamed_14 = 268435529;
pub const KEYC_PASTE_END: unnamed_14 = 268435459;
pub const KEYC_MOUSEDOWN1_STATUS: unnamed_14 = 268435466;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_31 {
    ev_io: unnamed_9,
    ev_signal: unnamed_3,
}
pub const KEYC_IC: unnamed_14 = 268435538;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub const KEYC_MOUSEDRAG3_STATUS: unnamed_14 = 268435490;
pub const LINE_SEL_NONE: unnamed_36 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type mode_tree_each_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: *mut client, _: key_code) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_16,
}
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
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
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_6,
}
pub const KEYC_MOUSEDOWN3_STATUS: unnamed_14 = 268435472;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const KEYC_WHEELUP_PANE: unnamed_14 = 268435501;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
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
pub const KEYC_TRIPLECLICK2_BORDER: unnamed_14 = 268435521;
pub const KEYC_MOUSEUP2_STATUS: unnamed_14 = 268435478;
pub const KEYC_MOUSEDRAG2_STATUS: unnamed_14 = 268435487;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
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
pub const KEYC_MOUSEDRAG3_PANE: unnamed_14 = 268435489;
pub const KEYC_NPAGE: unnamed_14 = 268435542;
pub const KEYC_KP_ZERO: unnamed_14 = 268435563;
pub const KEYC_DOWN: unnamed_14 = 268435546;
pub const KEYC_F2: unnamed_14 = 268435527;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub type unnamed_36 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const KEYC_MOUSEDRAG1_BORDER: unnamed_14 = 268435485;
pub const KEYC_FOCUS_IN: unnamed_14 = 268435456;
pub type ssize_t = __ssize_t;
pub const KEYC_TRIPLECLICK3_BORDER: unnamed_14 = 268435524;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const KEYC_WHEELUP_STATUS: unnamed_14 = 268435502;
pub const KEYC_MOUSEDOWN1_BORDER: unnamed_14 = 268435467;
pub const KEYC_MOUSEDRAG2_BORDER: unnamed_14 = 268435488;
pub const KEYC_MOUSEDOWN2_BORDER: unnamed_14 = 268435470;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const KEYC_KP_SLASH: unnamed_14 = 268435549;
pub const KEYC_MOUSEDOWN1_PANE: unnamed_14 = 268435465;
pub const PROMPT_COMMAND: unnamed_22 = 1;
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
pub struct layout_cell {
    pub type_0: layout_type,
    pub parent: *mut layout_cell,
    pub sx: u_int,
    pub sy: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub wp: *mut window_pane,
    pub cells: layout_cells,
    pub entry: unnamed_8,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const KEYC_MOUSEUP3_BORDER: unnamed_14 = 268435482;
pub type __u_int = libc::c_uint;
pub const KEYC_F10: unnamed_14 = 268435535;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_40 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
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
    pub message_log: unnamed_19,
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
    pub entry: unnamed_28,
}
pub const KEYC_DOUBLECLICK1_BORDER: unnamed_14 = 268435509;
pub type mode_tree_build_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: u_int,
                                _: *mut uint64_t, _: *const libc::c_char)
               -> ()>;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
#[no_mangle]
pub unsafe extern "C" fn mode_tree_count_tagged(mut mtd: *mut mode_tree_data)
 -> u_int {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut i: u_int = 0;
    let mut tagged: u_int = 0;
    tagged = 0i32 as u_int;
    i = 0i32 as u_int;
    while i < (*mtd).line_size {
        mti = (*(*mtd).line_list.offset(i as isize)).item;
        if 0 != (*mti).tagged { tagged = tagged.wrapping_add(1) }
        i = i.wrapping_add(1)
    }
    return tagged;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_get_current(mut mtd: *mut mode_tree_data)
 -> *mut libc::c_void {
    return (*(*(*mtd).line_list.offset((*mtd).current as
                                           isize)).item).itemdata;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_expand_current(mut mtd:
                                                      *mut mode_tree_data)
 -> () {
    if 0 ==
           (*(*(*mtd).line_list.offset((*mtd).current as
                                           isize)).item).expanded {
        (*(*(*mtd).line_list.offset((*mtd).current as isize)).item).expanded =
            1i32;
        mode_tree_build(mtd);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_build(mut mtd: *mut mode_tree_data) -> () {
    let mut s: *mut screen = &mut (*mtd).screen as *mut screen;
    let mut tag: uint64_t = 0;
    if (*mtd).line_list != 0 as *mut libc::c_void as *mut mode_tree_line {
        tag = (*(*(*mtd).line_list.offset((*mtd).current as isize)).item).tag
    } else { tag = 0i32 as uint64_t }
    loop  {
        if 0 !=
               !((*(&mut (*mtd).children as *mut mode_tree_list)).tqh_first ==
                     0 as *mut libc::c_void as *mut mode_tree_item) as
                   libc::c_int {
            let ref mut fresh0 =
                *(*(&mut (*mtd).saved as *mut mode_tree_list)).tqh_last;
            *fresh0 =
                (*(&mut (*mtd).children as *mut mode_tree_list)).tqh_first;
            let ref mut fresh1 =
                (*(*(&mut (*mtd).children as
                         *mut mode_tree_list)).tqh_first).entry.tqe_prev;
            *fresh1 = (*(&mut (*mtd).saved as *mut mode_tree_list)).tqh_last;
            let ref mut fresh2 =
                (*(&mut (*mtd).saved as *mut mode_tree_list)).tqh_last;
            *fresh2 =
                (*(&mut (*mtd).children as *mut mode_tree_list)).tqh_last;
            loop  {
                let ref mut fresh3 =
                    (*(&mut (*mtd).children as
                           *mut mode_tree_list)).tqh_first;
                *fresh3 = 0 as *mut mode_tree_item;
                let ref mut fresh4 =
                    (*(&mut (*mtd).children as *mut mode_tree_list)).tqh_last;
                *fresh4 =
                    &mut (*(&mut (*mtd).children as
                                *mut mode_tree_list)).tqh_first as
                        *mut *mut mode_tree_item;
                if !(0 != 0i32) { break ; }
            }
        }
        if !(0 != 0i32) { break ; }
    }
    loop  {
        let ref mut fresh5 =
            (*(&mut (*mtd).children as *mut mode_tree_list)).tqh_first;
        *fresh5 = 0 as *mut mode_tree_item;
        let ref mut fresh6 =
            (*(&mut (*mtd).children as *mut mode_tree_list)).tqh_last;
        *fresh6 =
            &mut (*(&mut (*mtd).children as *mut mode_tree_list)).tqh_first as
                *mut *mut mode_tree_item;
        if !(0 != 0i32) { break ; }
    }
    (*mtd).buildcb.expect("non-null function pointer")((*mtd).modedata,
                                                       (*mtd).sort_type,
                                                       &mut tag as
                                                           *mut uint64_t,
                                                       (*mtd).filter);
    (*mtd).no_matches =
        ((*(&mut (*mtd).children as *mut mode_tree_list)).tqh_first ==
             0 as *mut libc::c_void as *mut mode_tree_item) as libc::c_int;
    if 0 != (*mtd).no_matches {
        (*mtd).buildcb.expect("non-null function pointer")((*mtd).modedata,
                                                           (*mtd).sort_type,
                                                           &mut tag as
                                                               *mut uint64_t,
                                                           0 as
                                                               *const libc::c_char);
    }
    mode_tree_free_items(&mut (*mtd).saved as *mut mode_tree_list);
    loop  {
        let ref mut fresh7 =
            (*(&mut (*mtd).saved as *mut mode_tree_list)).tqh_first;
        *fresh7 = 0 as *mut mode_tree_item;
        let ref mut fresh8 =
            (*(&mut (*mtd).saved as *mut mode_tree_list)).tqh_last;
        *fresh8 =
            &mut (*(&mut (*mtd).saved as *mut mode_tree_list)).tqh_first as
                *mut *mut mode_tree_item;
        if !(0 != 0i32) { break ; }
    }
    mode_tree_clear_lines(mtd);
    mode_tree_build_lines(mtd, &mut (*mtd).children as *mut mode_tree_list,
                          0i32 as u_int);
    mode_tree_set_current(mtd, tag);
    (*mtd).width = (*(*s).grid).sx;
    if 0 != (*mtd).preview {
        (*mtd).height =
            (*(*s).grid).sy.wrapping_div(3i32 as
                                             libc::c_uint).wrapping_mul(2i32
                                                                            as
                                                                            libc::c_uint);
        if (*mtd).height > (*mtd).line_size {
            (*mtd).height = (*(*s).grid).sy.wrapping_div(2i32 as libc::c_uint)
        }
        if (*mtd).height < 10i32 as libc::c_uint {
            (*mtd).height = (*(*s).grid).sy
        }
        if (*(*s).grid).sy.wrapping_sub((*mtd).height) < 2i32 as libc::c_uint
           {
            (*mtd).height = (*(*s).grid).sy
        }
    } else { (*mtd).height = (*(*s).grid).sy }
    mode_tree_check_selected(mtd);
}
unsafe extern "C" fn mode_tree_check_selected(mut mtd: *mut mode_tree_data)
 -> () {
    if (*mtd).current > (*mtd).height.wrapping_sub(1i32 as libc::c_uint) {
        (*mtd).offset =
            (*mtd).current.wrapping_sub((*mtd).height).wrapping_add(1i32 as
                                                                        libc::c_uint)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_set_current(mut mtd: *mut mode_tree_data,
                                               mut tag: uint64_t) -> () {
    let mut i: u_int = 0;
    i = 0i32 as u_int;
    while i < (*mtd).line_size {
        if (*(*(*mtd).line_list.offset(i as isize)).item).tag == tag {
            break ;
        }
        i = i.wrapping_add(1)
    }
    if i != (*mtd).line_size {
        (*mtd).current = i;
        if (*mtd).current > (*mtd).height.wrapping_sub(1i32 as libc::c_uint) {
            (*mtd).offset =
                (*mtd).current.wrapping_sub((*mtd).height).wrapping_add(1i32
                                                                            as
                                                                            libc::c_uint)
        } else { (*mtd).offset = 0i32 as u_int }
    } else { (*mtd).current = 0i32 as u_int; (*mtd).offset = 0i32 as u_int };
}
unsafe extern "C" fn mode_tree_build_lines(mut mtd: *mut mode_tree_data,
                                           mut mtl: *mut mode_tree_list,
                                           mut depth: u_int) -> () {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut line: *mut mode_tree_line = 0 as *mut mode_tree_line;
    let mut i: u_int = 0;
    let mut flat: libc::c_int = 1i32;
    (*mtd).depth = depth;
    mti = (*mtl).tqh_first;
    while mti != 0 as *mut libc::c_void as *mut mode_tree_item {
        (*mtd).line_list =
            xreallocarray((*mtd).line_list as *mut libc::c_void,
                          (*mtd).line_size.wrapping_add(1i32 as libc::c_uint)
                              as size_t,
                          ::std::mem::size_of::<mode_tree_line>() as
                              libc::c_ulong) as *mut mode_tree_line;
        let fresh9 = (*mtd).line_size;
        (*mtd).line_size = (*mtd).line_size.wrapping_add(1);
        line =
            &mut *(*mtd).line_list.offset(fresh9 as isize) as
                *mut mode_tree_line;
        (*line).item = mti;
        (*line).depth = depth;
        (*line).last =
            (mti == *(*((*mtl).tqh_last as *mut mode_tree_list)).tqh_last) as
                libc::c_int;
        (*mti).line = (*mtd).line_size.wrapping_sub(1i32 as libc::c_uint);
        if 0 !=
               !((*(&mut (*mti).children as *mut mode_tree_list)).tqh_first ==
                     0 as *mut libc::c_void as *mut mode_tree_item) as
                   libc::c_int {
            flat = 0i32
        }
        if 0 != (*mti).expanded {
            mode_tree_build_lines(mtd,
                                  &mut (*mti).children as *mut mode_tree_list,
                                  depth.wrapping_add(1i32 as libc::c_uint));
        }
        mti = (*mti).entry.tqe_next
    }
    mti = (*mtl).tqh_first;
    while mti != 0 as *mut libc::c_void as *mut mode_tree_item {
        i = 0i32 as u_int;
        while i < (*mtd).line_size {
            line =
                &mut *(*mtd).line_list.offset(i as isize) as
                    *mut mode_tree_line;
            if (*line).item == mti { (*line).flat = flat }
            i = i.wrapping_add(1)
        }
        mti = (*mti).entry.tqe_next
    };
}
unsafe extern "C" fn mode_tree_clear_lines(mut mtd: *mut mode_tree_data)
 -> () {
    free((*mtd).line_list as *mut libc::c_void);
    (*mtd).line_list = 0 as *mut mode_tree_line;
    (*mtd).line_size = 0i32 as u_int;
}
unsafe extern "C" fn mode_tree_free_items(mut mtl: *mut mode_tree_list)
 -> () {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut mti1: *mut mode_tree_item = 0 as *mut mode_tree_item;
    mti = (*mtl).tqh_first;
    while mti != 0 as *mut libc::c_void as *mut mode_tree_item &&
              { mti1 = (*mti).entry.tqe_next; 0 != 1i32 } {
        loop  {
            if (*mti).entry.tqe_next !=
                   0 as *mut libc::c_void as *mut mode_tree_item {
                (*(*mti).entry.tqe_next).entry.tqe_prev =
                    (*mti).entry.tqe_prev
            } else { (*mtl).tqh_last = (*mti).entry.tqe_prev }
            *(*mti).entry.tqe_prev = (*mti).entry.tqe_next;
            if !(0 != 0i32) { break ; }
        }
        mode_tree_free_item(mti);
        mti = mti1
    };
}
unsafe extern "C" fn mode_tree_free_item(mut mti: *mut mode_tree_item) -> () {
    mode_tree_free_items(&mut (*mti).children as *mut mode_tree_list);
    free((*mti).name as *mut libc::c_void);
    free((*mti).text as *mut libc::c_void);
    free(mti as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_each_tagged(mut mtd: *mut mode_tree_data,
                                               mut cb: mode_tree_each_cb,
                                               mut c: *mut client,
                                               mut key: key_code,
                                               mut current: libc::c_int)
 -> () {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut i: u_int = 0;
    let mut fired: libc::c_int = 0;
    fired = 0i32;
    i = 0i32 as u_int;
    while i < (*mtd).line_size {
        mti = (*(*mtd).line_list.offset(i as isize)).item;
        if 0 != (*mti).tagged {
            fired = 1i32;
            cb.expect("non-null function pointer")((*mtd).modedata,
                                                   (*mti).itemdata, c, key);
        }
        i = i.wrapping_add(1)
    }
    if 0 == fired && 0 != current {
        mti = (*(*mtd).line_list.offset((*mtd).current as isize)).item;
        cb.expect("non-null function pointer")((*mtd).modedata,
                                               (*mti).itemdata, c, key);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_up(mut mtd: *mut mode_tree_data,
                                      mut wrap: libc::c_int) -> () {
    if (*mtd).current == 0i32 as libc::c_uint {
        if 0 != wrap {
            (*mtd).current =
                (*mtd).line_size.wrapping_sub(1i32 as libc::c_uint);
            if (*mtd).line_size >= (*mtd).height {
                (*mtd).offset = (*mtd).line_size.wrapping_sub((*mtd).height)
            }
        }
    } else {
        (*mtd).current = (*mtd).current.wrapping_sub(1);
        if (*mtd).current < (*mtd).offset {
            (*mtd).offset = (*mtd).offset.wrapping_sub(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_down(mut mtd: *mut mode_tree_data,
                                        mut wrap: libc::c_int) -> () {
    if (*mtd).current == (*mtd).line_size.wrapping_sub(1i32 as libc::c_uint) {
        if 0 != wrap {
            (*mtd).current = 0i32 as u_int;
            (*mtd).offset = 0i32 as u_int
        }
    } else {
        (*mtd).current = (*mtd).current.wrapping_add(1);
        if (*mtd).current >
               (*mtd).offset.wrapping_add((*mtd).height).wrapping_sub(1i32 as
                                                                          libc::c_uint)
           {
            (*mtd).offset = (*mtd).offset.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_start(mut wp: *mut window_pane,
                                         mut args: *mut args,
                                         mut buildcb: mode_tree_build_cb,
                                         mut drawcb: mode_tree_draw_cb,
                                         mut searchcb: mode_tree_search_cb,
                                         mut modedata: *mut libc::c_void,
                                         mut sort_list:
                                             *mut *const libc::c_char,
                                         mut sort_size: u_int,
                                         mut s: *mut *mut screen)
 -> *mut mode_tree_data {
    let mut mtd: *mut mode_tree_data = 0 as *mut mode_tree_data;
    let mut sort: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: u_int = 0;
    mtd =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<mode_tree_data>() as libc::c_ulong) as
            *mut mode_tree_data;
    (*mtd).references = 1i32 as u_int;
    (*mtd).wp = wp;
    (*mtd).modedata = modedata;
    (*mtd).sort_list = sort_list;
    (*mtd).sort_size = sort_size;
    (*mtd).sort_type = 0i32 as u_int;
    (*mtd).preview = (0 == args_has(args, 78 as u_char)) as libc::c_int;
    sort = args_get(args, 79 as u_char);
    if sort != 0 as *mut libc::c_void as *const libc::c_char {
        i = 0i32 as u_int;
        while i < sort_size {
            if strcasecmp(sort, *sort_list.offset(i as isize)) == 0i32 {
                (*mtd).sort_type = i
            }
            i = i.wrapping_add(1)
        }
    }
    if 0 != args_has(args, 102 as u_char) {
        (*mtd).filter = xstrdup(args_get(args, 102 as u_char))
    } else { (*mtd).filter = 0 as *mut libc::c_char }
    (*mtd).buildcb = buildcb;
    (*mtd).drawcb = drawcb;
    (*mtd).searchcb = searchcb;
    loop  {
        let ref mut fresh10 =
            (*(&mut (*mtd).children as *mut mode_tree_list)).tqh_first;
        *fresh10 = 0 as *mut mode_tree_item;
        let ref mut fresh11 =
            (*(&mut (*mtd).children as *mut mode_tree_list)).tqh_last;
        *fresh11 =
            &mut (*(&mut (*mtd).children as *mut mode_tree_list)).tqh_first as
                *mut *mut mode_tree_item;
        if !(0 != 0i32) { break ; }
    }
    *s = &mut (*mtd).screen as *mut screen;
    screen_init(*s, (*(*(&mut (*wp).base as *mut screen)).grid).sx,
                (*(*(&mut (*wp).base as *mut screen)).grid).sy,
                0i32 as u_int);
    (**s).mode &= !1i32;
    return mtd;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_zoom(mut mtd: *mut mode_tree_data,
                                        mut args: *mut args) -> () {
    let mut wp: *mut window_pane = (*mtd).wp;
    if 0 != args_has(args, 90 as u_char) {
        (*mtd).zoomed = (*(*wp).window).flags & 4096i32;
        if 0 == (*mtd).zoomed && window_zoom(wp) == 0i32 {
            server_redraw_window((*wp).window);
        }
    } else { (*mtd).zoomed = 1i32.wrapping_neg() };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_free(mut mtd: *mut mode_tree_data) -> () {
    let mut wp: *mut window_pane = (*mtd).wp;
    if (*mtd).zoomed == 0i32 { server_unzoom_window((*wp).window); }
    mode_tree_free_items(&mut (*mtd).children as *mut mode_tree_list);
    mode_tree_clear_lines(mtd);
    screen_free(&mut (*mtd).screen as *mut screen);
    free((*mtd).search as *mut libc::c_void);
    free((*mtd).filter as *mut libc::c_void);
    (*mtd).dead = 1i32;
    mode_tree_remove_ref(mtd);
}
unsafe extern "C" fn mode_tree_remove_ref(mut mtd: *mut mode_tree_data)
 -> () {
    (*mtd).references = (*mtd).references.wrapping_sub(1);
    if (*mtd).references == 0i32 as libc::c_uint {
        free(mtd as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_resize(mut mtd: *mut mode_tree_data,
                                          mut sx: u_int, mut sy: u_int)
 -> () {
    let mut s: *mut screen = &mut (*mtd).screen as *mut screen;
    screen_resize(s, sx, sy, 0i32);
    mode_tree_build(mtd);
    mode_tree_draw(mtd);
    (*(*mtd).wp).flags |= 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_draw(mut mtd: *mut mode_tree_data) -> () {
    let mut wp: *mut window_pane = (*mtd).wp;
    let mut s: *mut screen = &mut (*mtd).screen as *mut screen;
    let mut line: *mut mode_tree_line = 0 as *mut mode_tree_line;
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut oo: *mut options = (*(*wp).window).options;
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
    let mut gc0: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut w: u_int = 0;
    let mut h: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut sy: u_int = 0;
    let mut box_x: u_int = 0;
    let mut box_y: u_int = 0;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: [libc::c_char; 7] = [0; 7];
    let mut tag: *const libc::c_char = 0 as *const libc::c_char;
    let mut symbol: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: size_t = 0;
    let mut n: size_t = 0;
    let mut keylen: libc::c_int = 0;
    if (*mtd).line_size == 0i32 as libc::c_uint {
        return
    } else {
        memcpy(&mut gc0 as *mut grid_cell as *mut libc::c_void,
               &grid_default_cell as *const grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
               &grid_default_cell as *const grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        style_apply(&mut gc as *mut grid_cell, oo,
                    b"mode-style\x00" as *const u8 as *const libc::c_char);
        w = (*mtd).width;
        h = (*mtd).height;
        screen_write_start(&mut ctx as *mut screen_write_ctx,
                           0 as *mut window_pane, s);
        screen_write_clearscreen(&mut ctx as *mut screen_write_ctx,
                                 8i32 as u_int);
        if (*mtd).line_size > 10i32 as libc::c_uint {
            keylen = 6i32
        } else { keylen = 4i32 }
        i = 0i32 as u_int;
        while i < (*mtd).line_size {
            if !(i < (*mtd).offset) {
                if i >
                       (*mtd).offset.wrapping_add(h).wrapping_sub(1i32 as
                                                                      libc::c_uint)
                   {
                    break ;
                }
                line =
                    &mut *(*mtd).line_list.offset(i as isize) as
                        *mut mode_tree_line;
                mti = (*line).item;
                screen_write_cursormove(&mut ctx as *mut screen_write_ctx,
                                        0i32 as u_int,
                                        i.wrapping_sub((*mtd).offset));
                if i < 10i32 as libc::c_uint {
                    snprintf(key.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 7]>() as
                                 libc::c_ulong,
                             b"(%c)  \x00" as *const u8 as
                                 *const libc::c_char,
                             (48 as libc::c_uint).wrapping_add(i));
                } else if i < 36i32 as libc::c_uint {
                    snprintf(key.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 7]>() as
                                 libc::c_ulong,
                             b"(M-%c)\x00" as *const u8 as
                                 *const libc::c_char,
                             (97 as
                                  libc::c_uint).wrapping_add(i.wrapping_sub(10i32
                                                                                as
                                                                                libc::c_uint)));
                } else { *key.as_mut_ptr() = 0 as libc::c_char }
                if 0 != (*line).flat {
                    symbol = b"\x00" as *const u8 as *const libc::c_char
                } else if (*(&mut (*mti).children as
                                 *mut mode_tree_list)).tqh_first ==
                              0 as *mut libc::c_void as *mut mode_tree_item {
                    symbol = b"  \x00" as *const u8 as *const libc::c_char
                } else if 0 != (*mti).expanded {
                    symbol = b"- \x00" as *const u8 as *const libc::c_char
                } else {
                    symbol = b"+ \x00" as *const u8 as *const libc::c_char
                }
                if (*line).depth == 0i32 as libc::c_uint {
                    start = xstrdup(symbol)
                } else {
                    size =
                        (4i32 as
                             libc::c_uint).wrapping_mul((*line).depth).wrapping_add(32i32
                                                                                        as
                                                                                        libc::c_uint)
                            as size_t;
                    start =
                        xcalloc(1i32 as size_t, size) as *mut libc::c_char;
                    j = 1i32 as u_int;
                    while j < (*line).depth {
                        if (*mti).parent !=
                               0 as *mut libc::c_void as *mut mode_tree_item
                               &&
                               0 !=
                                   (*(*mtd).line_list.offset((*(*mti).parent).line
                                                                 as
                                                                 isize)).last
                           {
                            strlcat(start,
                                    b"    \x00" as *const u8 as
                                        *const libc::c_char, size);
                        } else {
                            strlcat(start,
                                    b"\x01x\x01   \x00" as *const u8 as
                                        *const libc::c_char, size);
                        }
                        j = j.wrapping_add(1)
                    }
                    if 0 != (*line).last {
                        strlcat(start,
                                b"\x01mq\x01> \x00" as *const u8 as
                                    *const libc::c_char, size);
                    } else {
                        strlcat(start,
                                b"\x01tq\x01> \x00" as *const u8 as
                                    *const libc::c_char, size);
                    }
                    strlcat(start, symbol, size);
                }
                if 0 != (*mti).tagged {
                    tag = b"*\x00" as *const u8 as *const libc::c_char
                } else { tag = b"\x00" as *const u8 as *const libc::c_char }
                xasprintf(&mut text as *mut *mut libc::c_char,
                          b"%-*s%s%s%s: %s\x00" as *const u8 as
                              *const libc::c_char, keylen, key.as_mut_ptr(),
                          start, (*mti).name, tag, (*mti).text);
                free(start as *mut libc::c_void);
                if 0 != (*mti).tagged {
                    gc.attr = (gc.attr as libc::c_int ^ 1i32) as u_short;
                    gc0.attr = (gc0.attr as libc::c_int ^ 1i32) as u_short
                }
                if i != (*mtd).current {
                    screen_write_nputs(&mut ctx as *mut screen_write_ctx,
                                       w as ssize_t,
                                       &mut gc0 as *mut grid_cell,
                                       b"%s\x00" as *const u8 as
                                           *const libc::c_char, text);
                    screen_write_clearendofline(&mut ctx as
                                                    *mut screen_write_ctx,
                                                8i32 as u_int);
                } else {
                    screen_write_nputs(&mut ctx as *mut screen_write_ctx,
                                       w as ssize_t,
                                       &mut gc as *mut grid_cell,
                                       b"%s\x00" as *const u8 as
                                           *const libc::c_char, text);
                    screen_write_clearendofline(&mut ctx as
                                                    *mut screen_write_ctx,
                                                gc.bg as u_int);
                }
                free(text as *mut libc::c_void);
                if 0 != (*mti).tagged {
                    gc.attr = (gc.attr as libc::c_int ^ 1i32) as u_short;
                    gc0.attr = (gc0.attr as libc::c_int ^ 1i32) as u_short
                }
            }
            i = i.wrapping_add(1)
        }
        sy = (*(*s).grid).sy;
        if 0 == (*mtd).preview || sy <= 4i32 as libc::c_uint ||
               h <= 4i32 as libc::c_uint ||
               sy.wrapping_sub(h) <= 4i32 as libc::c_uint ||
               w <= 4i32 as libc::c_uint {
            screen_write_stop(&mut ctx as *mut screen_write_ctx);
            return
        } else {
            line =
                &mut *(*mtd).line_list.offset((*mtd).current as isize) as
                    *mut mode_tree_line;
            mti = (*line).item;
            screen_write_cursormove(&mut ctx as *mut screen_write_ctx,
                                    0i32 as u_int, h);
            screen_write_box(&mut ctx as *mut screen_write_ctx, w,
                             sy.wrapping_sub(h));
            xasprintf(&mut text as *mut *mut libc::c_char,
                      b" %s (sort: %s)\x00" as *const u8 as
                          *const libc::c_char, (*mti).name,
                      *(*mtd).sort_list.offset((*mtd).sort_type as isize));
            if w.wrapping_sub(2i32 as libc::c_uint) as libc::c_ulong >=
                   strlen(text) {
                screen_write_cursormove(&mut ctx as *mut screen_write_ctx,
                                        1i32 as u_int, h);
                screen_write_puts(&mut ctx as *mut screen_write_ctx,
                                  &mut gc0 as *mut grid_cell,
                                  b"%s\x00" as *const u8 as
                                      *const libc::c_char, text);
                if 0 != (*mtd).no_matches {
                    n =
                        (::std::mem::size_of::<[libc::c_char; 11]>() as
                             libc::c_ulong).wrapping_sub(1i32 as
                                                             libc::c_ulong)
                } else {
                    n =
                        (::std::mem::size_of::<[libc::c_char; 7]>() as
                             libc::c_ulong).wrapping_sub(1i32 as
                                                             libc::c_ulong)
                }
                if (*mtd).filter !=
                       0 as *mut libc::c_void as *mut libc::c_char &&
                       w.wrapping_sub(2i32 as libc::c_uint) as libc::c_ulong
                           >=
                           strlen(text).wrapping_add(10i32 as
                                                         libc::c_ulong).wrapping_add(n).wrapping_add(2i32
                                                                                                         as
                                                                                                         libc::c_ulong)
                   {
                    screen_write_puts(&mut ctx as *mut screen_write_ctx,
                                      &mut gc0 as *mut grid_cell,
                                      b" (filter: \x00" as *const u8 as
                                          *const libc::c_char);
                    if 0 != (*mtd).no_matches {
                        screen_write_puts(&mut ctx as *mut screen_write_ctx,
                                          &mut gc as *mut grid_cell,
                                          b"no matches\x00" as *const u8 as
                                              *const libc::c_char);
                    } else {
                        screen_write_puts(&mut ctx as *mut screen_write_ctx,
                                          &mut gc0 as *mut grid_cell,
                                          b"active\x00" as *const u8 as
                                              *const libc::c_char);
                    }
                    screen_write_puts(&mut ctx as *mut screen_write_ctx,
                                      &mut gc0 as *mut grid_cell,
                                      b") \x00" as *const u8 as
                                          *const libc::c_char);
                }
            }
            free(text as *mut libc::c_void);
            box_x = w.wrapping_sub(4i32 as libc::c_uint);
            box_y = sy.wrapping_sub(h).wrapping_sub(2i32 as libc::c_uint);
            if box_x != 0i32 as libc::c_uint && box_y != 0i32 as libc::c_uint
               {
                screen_write_cursormove(&mut ctx as *mut screen_write_ctx,
                                        2i32 as u_int,
                                        h.wrapping_add(1i32 as libc::c_uint));
                (*mtd).drawcb.expect("non-null function pointer")((*mtd).modedata,
                                                                  (*mti).itemdata,
                                                                  &mut ctx as
                                                                      *mut screen_write_ctx,
                                                                  box_x,
                                                                  box_y);
            }
            screen_write_stop(&mut ctx as *mut screen_write_ctx);
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_add(mut mtd: *mut mode_tree_data,
                                       mut parent: *mut mode_tree_item,
                                       mut itemdata: *mut libc::c_void,
                                       mut tag: uint64_t,
                                       mut name: *const libc::c_char,
                                       mut text: *const libc::c_char,
                                       mut expanded: libc::c_int)
 -> *mut mode_tree_item {
    let mut current_block: u64;
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut saved: *mut mode_tree_item = 0 as *mut mode_tree_item;
    log_debug(b"%s: %llu, %s %s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"mode_tree_add\x00")).as_ptr(),
              tag as libc::c_ulonglong, name, text);
    mti =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<mode_tree_item>() as libc::c_ulong) as
            *mut mode_tree_item;
    (*mti).parent = parent;
    (*mti).itemdata = itemdata;
    (*mti).tag = tag;
    (*mti).name = xstrdup(name);
    (*mti).text = xstrdup(text);
    saved =
        mode_tree_find_item(&mut (*mtd).saved as *mut mode_tree_list, tag);
    if saved != 0 as *mut libc::c_void as *mut mode_tree_item {
        if parent == 0 as *mut libc::c_void as *mut mode_tree_item ||
               parent != 0 as *mut libc::c_void as *mut mode_tree_item &&
                   0 != (*parent).expanded {
            (*mti).tagged = (*saved).tagged
        }
        (*mti).expanded = (*saved).expanded
    } else if expanded == 1i32.wrapping_neg() {
        (*mti).expanded = 1i32
    } else { (*mti).expanded = expanded }
    loop  {
        let ref mut fresh12 =
            (*(&mut (*mti).children as *mut mode_tree_list)).tqh_first;
        *fresh12 = 0 as *mut mode_tree_item;
        let ref mut fresh13 =
            (*(&mut (*mti).children as *mut mode_tree_list)).tqh_last;
        *fresh13 =
            &mut (*(&mut (*mti).children as *mut mode_tree_list)).tqh_first as
                *mut *mut mode_tree_item;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut mode_tree_item {
        current_block = 7815301370352969686;
    } else { current_block = 13183875560443969876; }
    loop  {
        match current_block {
            13183875560443969876 => {
                (*mti).entry.tqe_next = 0 as *mut mode_tree_item;
                (*mti).entry.tqe_prev =
                    (*(&mut (*mtd).children as *mut mode_tree_list)).tqh_last;
                let ref mut fresh16 =
                    *(*(&mut (*mtd).children as
                            *mut mode_tree_list)).tqh_last;
                *fresh16 = mti;
                let ref mut fresh17 =
                    (*(&mut (*mtd).children as *mut mode_tree_list)).tqh_last;
                *fresh17 =
                    &mut (*mti).entry.tqe_next as *mut *mut mode_tree_item;
                if 0 != 0i32 {
                    current_block = 13183875560443969876;
                } else { break ; }
            }
            _ => {
                (*mti).entry.tqe_next = 0 as *mut mode_tree_item;
                (*mti).entry.tqe_prev =
                    (*(&mut (*parent).children as
                           *mut mode_tree_list)).tqh_last;
                let ref mut fresh14 =
                    *(*(&mut (*parent).children as
                            *mut mode_tree_list)).tqh_last;
                *fresh14 = mti;
                let ref mut fresh15 =
                    (*(&mut (*parent).children as
                           *mut mode_tree_list)).tqh_last;
                *fresh15 =
                    &mut (*mti).entry.tqe_next as *mut *mut mode_tree_item;
                if 0 != 0i32 {
                    current_block = 7815301370352969686;
                } else { break ; }
            }
        }
    }
    return mti;
}
unsafe extern "C" fn mode_tree_find_item(mut mtl: *mut mode_tree_list,
                                         mut tag: uint64_t)
 -> *mut mode_tree_item {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut child: *mut mode_tree_item = 0 as *mut mode_tree_item;
    mti = (*mtl).tqh_first;
    loop  {
        if mti != 0 as *mut libc::c_void as *mut mode_tree_item {
            if (*mti).tag == tag {
                return mti
            } else {
                child =
                    mode_tree_find_item(&mut (*mti).children as
                                            *mut mode_tree_list, tag);
                if child != 0 as *mut libc::c_void as *mut mode_tree_item {
                    return child
                } else { mti = (*mti).entry.tqe_next }
            }
        } else { return 0 as *mut mode_tree_item }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_remove(mut mtd: *mut mode_tree_data,
                                          mut mti: *mut mode_tree_item)
 -> () {
    let mut current_block: u64;
    let mut parent: *mut mode_tree_item = (*mti).parent;
    if parent != 0 as *mut libc::c_void as *mut mode_tree_item {
        current_block = 6239978542346980191;
    } else { current_block = 17179679302217393232; }
    loop  {
        match current_block {
            17179679302217393232 => {
                if (*mti).entry.tqe_next !=
                       0 as *mut libc::c_void as *mut mode_tree_item {
                    (*(*mti).entry.tqe_next).entry.tqe_prev =
                        (*mti).entry.tqe_prev
                } else {
                    let ref mut fresh19 =
                        (*(&mut (*mtd).children as
                               *mut mode_tree_list)).tqh_last;
                    *fresh19 = (*mti).entry.tqe_prev
                }
                *(*mti).entry.tqe_prev = (*mti).entry.tqe_next;
                if 0 != 0i32 {
                    current_block = 17179679302217393232;
                } else { break ; }
            }
            _ => {
                if (*mti).entry.tqe_next !=
                       0 as *mut libc::c_void as *mut mode_tree_item {
                    (*(*mti).entry.tqe_next).entry.tqe_prev =
                        (*mti).entry.tqe_prev
                } else {
                    let ref mut fresh18 =
                        (*(&mut (*parent).children as
                               *mut mode_tree_list)).tqh_last;
                    *fresh18 = (*mti).entry.tqe_prev
                }
                *(*mti).entry.tqe_prev = (*mti).entry.tqe_next;
                if 0 != 0i32 {
                    current_block = 6239978542346980191;
                } else { break ; }
            }
        }
    }
    mode_tree_free_item(mti);
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_key(mut mtd: *mut mode_tree_data,
                                       mut c: *mut client,
                                       mut key: *mut key_code,
                                       mut m: *mut mouse_event,
                                       mut xp: *mut u_int, mut yp: *mut u_int)
 -> libc::c_int {
    let mut line: *mut mode_tree_line = 0 as *mut mode_tree_line;
    let mut current: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut parent: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut i: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut choice: libc::c_int = 0;
    let mut tmp: key_code = 0;
    if *key &
           !(35184372088832u64 | 70368744177664u64 | 140737488355328u64 |
                 281474976710656u64) >=
           KEYC_MOUSE as libc::c_int as libc::c_ulonglong &&
           *key &
               !(35184372088832u64 | 70368744177664u64 | 140737488355328u64 |
                     281474976710656u64) <
               KEYC_BSPACE as libc::c_int as libc::c_ulonglong {
        if cmd_mouse_at((*mtd).wp, m, &mut x as *mut u_int,
                        &mut y as *mut u_int, 0i32) != 0i32 {
            *key = 281470681743360u64;
            return 0i32
        } else {
            if xp != 0 as *mut libc::c_void as *mut u_int { *xp = x }
            if yp != 0 as *mut libc::c_void as *mut u_int { *yp = y }
            if x > (*mtd).width || y > (*mtd).height {
                if 0 == (*mtd).preview { *key = 281470681743360u64 }
                return 0i32
            } else {
                if (*mtd).offset.wrapping_add(y) < (*mtd).line_size {
                    if *key ==
                           KEYC_MOUSEDOWN1_PANE as libc::c_int as
                               libc::c_ulonglong ||
                           *key ==
                               KEYC_DOUBLECLICK1_PANE as libc::c_int as
                                   libc::c_ulonglong {
                        (*mtd).current = (*mtd).offset.wrapping_add(y)
                    }
                    if *key ==
                           KEYC_DOUBLECLICK1_PANE as libc::c_int as
                               libc::c_ulonglong {
                        *key = 13 as key_code
                    } else { *key = 281470681743360u64 }
                } else { *key = 281470681743360u64 }
                return 0i32
            }
        }
    } else {
        line =
            &mut *(*mtd).line_list.offset((*mtd).current as isize) as
                *mut mode_tree_line;
        current = (*line).item;
        choice = 1i32.wrapping_neg();
        if *key >= 48 as libc::c_ulonglong && *key <= 57 as libc::c_ulonglong
           {
            choice =
                (*key).wrapping_sub(48 as libc::c_ulonglong) as libc::c_int
        } else if *key &
                      (35184372088832u64 | 70368744177664u64 |
                           140737488355328u64 | 281474976710656u64) ==
                      35184372088832u64 {
            tmp =
                *key &
                    !(35184372088832u64 | 70368744177664u64 |
                          140737488355328u64 | 281474976710656u64);
            if tmp >= 97 as libc::c_ulonglong &&
                   tmp <= 122 as libc::c_ulonglong {
                choice =
                    (10i32 as
                         libc::c_ulonglong).wrapping_add(tmp.wrapping_sub(97
                                                                              as
                                                                              libc::c_ulonglong))
                        as libc::c_int
            }
        }
        if choice != 1i32.wrapping_neg() {
            if choice as u_int >
                   (*mtd).line_size.wrapping_sub(1i32 as libc::c_uint) {
                *key = 281470681743360u64;
                return 0i32
            } else {
                (*mtd).current = choice as u_int;
                *key = 13 as key_code;
                return 0i32
            }
        } else {
            match *key {
                113 | 27 | 7 => { return 1i32 }
                268435545 | 107 | 268435501 | 16 => {
                    mode_tree_up(mtd, 1i32);
                }
                268435546 | 106 | 268435504 | 14 => {
                    mode_tree_down(mtd, 1i32);
                }
                268435543 | 2 => {
                    i = 0i32 as u_int;
                    while i < (*mtd).height {
                        if (*mtd).current == 0i32 as libc::c_uint { break ; }
                        mode_tree_up(mtd, 1i32);
                        i = i.wrapping_add(1)
                    }
                }
                268435542 | 6 => {
                    i = 0i32 as u_int;
                    while i < (*mtd).height {
                        if (*mtd).current ==
                               (*mtd).line_size.wrapping_sub(1i32 as
                                                                 libc::c_uint)
                           {
                            break ;
                        }
                        mode_tree_down(mtd, 1i32);
                        i = i.wrapping_add(1)
                    }
                }
                268435540 => {
                    (*mtd).current = 0i32 as u_int;
                    (*mtd).offset = 0i32 as u_int
                }
                268435541 => {
                    (*mtd).current =
                        (*mtd).line_size.wrapping_sub(1i32 as libc::c_uint);
                    if (*mtd).current >
                           (*mtd).height.wrapping_sub(1i32 as libc::c_uint) {
                        (*mtd).offset =
                            (*mtd).current.wrapping_sub((*mtd).height).wrapping_add(1i32
                                                                                        as
                                                                                        libc::c_uint)
                    } else { (*mtd).offset = 0i32 as u_int }
                }
                116 => {
                    if 0 == (*current).tagged {
                        parent = (*current).parent;
                        while parent !=
                                  0 as *mut libc::c_void as
                                      *mut mode_tree_item {
                            (*parent).tagged = 0i32;
                            parent = (*parent).parent
                        }
                        mode_tree_clear_tagged(&mut (*current).children as
                                                   *mut mode_tree_list);
                        (*current).tagged = 1i32
                    } else { (*current).tagged = 0i32 }
                    mode_tree_down(mtd, 0i32);
                }
                84 => {
                    i = 0i32 as u_int;
                    while i < (*mtd).line_size {
                        (*(*(*mtd).line_list.offset(i as isize)).item).tagged
                            = 0i32;
                        i = i.wrapping_add(1)
                    }
                }
                20 => {
                    i = 0i32 as u_int;
                    while i < (*mtd).line_size {
                        if (*(*(*mtd).line_list.offset(i as
                                                           isize)).item).parent
                               ==
                               0 as *mut libc::c_void as *mut mode_tree_item {
                            (*(*(*mtd).line_list.offset(i as
                                                            isize)).item).tagged
                                = 1i32
                        } else {
                            (*(*(*mtd).line_list.offset(i as
                                                            isize)).item).tagged
                                = 0i32
                        }
                        i = i.wrapping_add(1)
                    }
                }
                79 => {
                    (*mtd).sort_type = (*mtd).sort_type.wrapping_add(1);
                    if (*mtd).sort_type == (*mtd).sort_size {
                        (*mtd).sort_type = 0i32 as u_int
                    }
                    mode_tree_build(mtd);
                }
                268435547 | 104 | 45 => {
                    if 0 != (*line).flat || 0 == (*current).expanded {
                        current = (*current).parent
                    }
                    if current ==
                           0 as *mut libc::c_void as *mut mode_tree_item {
                        mode_tree_up(mtd, 0i32);
                    } else {
                        (*current).expanded = 0i32;
                        (*mtd).current = (*current).line;
                        mode_tree_build(mtd);
                    }
                }
                268435548 | 108 | 43 => {
                    if 0 != (*line).flat || 0 != (*current).expanded {
                        mode_tree_down(mtd, 0i32);
                    } else if 0 == (*line).flat {
                        (*current).expanded = 1i32;
                        mode_tree_build(mtd);
                    }
                }
                19 => {
                    (*mtd).references = (*mtd).references.wrapping_add(1);
                    status_prompt_set(c,
                                      b"(search) \x00" as *const u8 as
                                          *const libc::c_char,
                                      b"\x00" as *const u8 as
                                          *const libc::c_char,
                                      Some(mode_tree_search_callback),
                                      Some(mode_tree_search_free),
                                      mtd as *mut libc::c_void, 8i32);
                }
                110 => { mode_tree_search_set(mtd); }
                102 => {
                    (*mtd).references = (*mtd).references.wrapping_add(1);
                    status_prompt_set(c,
                                      b"(filter) \x00" as *const u8 as
                                          *const libc::c_char, (*mtd).filter,
                                      Some(mode_tree_filter_callback),
                                      Some(mode_tree_filter_free),
                                      mtd as *mut libc::c_void, 8i32);
                }
                118 => {
                    (*mtd).preview = (0 == (*mtd).preview) as libc::c_int;
                    mode_tree_build(mtd);
                    if 0 != (*mtd).preview { mode_tree_check_selected(mtd); }
                }
                _ => { }
            }
            return 0i32
        }
    };
}
unsafe extern "C" fn mode_tree_filter_free(mut data: *mut libc::c_void)
 -> () {
    mode_tree_remove_ref(data as *mut mode_tree_data);
}
unsafe extern "C" fn mode_tree_filter_callback(mut c: *mut client,
                                               mut data: *mut libc::c_void,
                                               mut s: *const libc::c_char,
                                               mut done: libc::c_int)
 -> libc::c_int {
    let mut mtd: *mut mode_tree_data = data as *mut mode_tree_data;
    if 0 != (*mtd).dead {
        return 0i32
    } else {
        if (*mtd).filter != 0 as *mut libc::c_void as *mut libc::c_char {
            free((*mtd).filter as *mut libc::c_void);
        }
        if s == 0 as *mut libc::c_void as *const libc::c_char ||
               *s as libc::c_int == 0 {
            (*mtd).filter = 0 as *mut libc::c_char
        } else { (*mtd).filter = xstrdup(s) }
        mode_tree_build(mtd);
        mode_tree_draw(mtd);
        (*(*mtd).wp).flags |= 1i32;
        return 0i32
    };
}
unsafe extern "C" fn mode_tree_search_set(mut mtd: *mut mode_tree_data)
 -> () {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut loop_0: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut tag: uint64_t = 0;
    mti = mode_tree_search_for(mtd);
    if mti == 0 as *mut libc::c_void as *mut mode_tree_item {
        return
    } else {
        tag = (*mti).tag;
        loop_0 = (*mti).parent;
        while loop_0 != 0 as *mut libc::c_void as *mut mode_tree_item {
            (*loop_0).expanded = 1i32;
            loop_0 = (*loop_0).parent
        }
        mode_tree_build(mtd);
        mode_tree_set_current(mtd, tag);
        mode_tree_draw(mtd);
        (*(*mtd).wp).flags |= 1i32;
        return;
    };
}
unsafe extern "C" fn mode_tree_search_for(mut mtd: *mut mode_tree_data)
 -> *mut mode_tree_item {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut last: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut next: *mut mode_tree_item = 0 as *mut mode_tree_item;
    if (*mtd).search == 0 as *mut libc::c_void as *mut libc::c_char {
        return 0 as *mut mode_tree_item
    } else {
        last = (*(*mtd).line_list.offset((*mtd).current as isize)).item;
        mti = last;
        loop  {
            if 0 !=
                   !((*(&mut (*mti).children as
                            *mut mode_tree_list)).tqh_first ==
                         0 as *mut libc::c_void as *mut mode_tree_item) as
                       libc::c_int {
                mti =
                    (*(&mut (*mti).children as *mut mode_tree_list)).tqh_first
            } else {
                next = (*mti).entry.tqe_next;
                if next != 0 as *mut libc::c_void as *mut mode_tree_item {
                    mti = next
                } else {
                    loop  {
                        mti = (*mti).parent;
                        if mti ==
                               0 as *mut libc::c_void as *mut mode_tree_item {
                            break ;
                        }
                        next = (*mti).entry.tqe_next;
                        if !(next !=
                                 0 as *mut libc::c_void as
                                     *mut mode_tree_item) {
                            continue ;
                        }
                        mti = next;
                        break ;
                    }
                }
            }
            if mti == 0 as *mut libc::c_void as *mut mode_tree_item {
                mti =
                    (*(&mut (*mtd).children as *mut mode_tree_list)).tqh_first
            }
            if mti == last {
                return 0 as *mut mode_tree_item
            } else if (*mtd).searchcb ==
                          ::std::mem::transmute::<*mut libc::c_void,
                                                  mode_tree_search_cb>(0 as
                                                                           *mut libc::c_void)
             {
                if !(strstr((*mti).name, (*mtd).search) !=
                         0 as *mut libc::c_void as *mut libc::c_char) {
                    continue ;
                }
                return mti
            } else {
                if !(0 !=
                         (*mtd).searchcb.expect("non-null function pointer")((*mtd).modedata,
                                                                             (*mti).itemdata,
                                                                             (*mtd).search))
                   {
                    continue ;
                }
                return mti
            }
        }
    };
}
unsafe extern "C" fn mode_tree_search_free(mut data: *mut libc::c_void)
 -> () {
    mode_tree_remove_ref(data as *mut mode_tree_data);
}
unsafe extern "C" fn mode_tree_search_callback(mut c: *mut client,
                                               mut data: *mut libc::c_void,
                                               mut s: *const libc::c_char,
                                               mut done: libc::c_int)
 -> libc::c_int {
    let mut mtd: *mut mode_tree_data = data as *mut mode_tree_data;
    if 0 != (*mtd).dead {
        return 0i32
    } else {
        free((*mtd).search as *mut libc::c_void);
        if s == 0 as *mut libc::c_void as *const libc::c_char ||
               *s as libc::c_int == 0 {
            (*mtd).search = 0 as *mut libc::c_char;
            return 0i32
        } else {
            (*mtd).search = xstrdup(s);
            mode_tree_search_set(mtd);
            return 0i32
        }
    };
}
unsafe extern "C" fn mode_tree_clear_tagged(mut mtl: *mut mode_tree_list)
 -> () {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    mti = (*mtl).tqh_first;
    while mti != 0 as *mut libc::c_void as *mut mode_tree_item {
        (*mti).tagged = 0i32;
        mode_tree_clear_tagged(&mut (*mti).children as *mut mode_tree_list);
        mti = (*mti).entry.tqe_next
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_run_command(mut c: *mut client,
                                               mut fs: *mut cmd_find_state,
                                               mut template:
                                                   *const libc::c_char,
                                               mut name: *const libc::c_char)
 -> () {
    let mut new_item: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    command = cmd_template_replace(template, name, 1i32);
    if command == 0 as *mut libc::c_void as *mut libc::c_char ||
           *command as libc::c_int == 0 {
        free(command as *mut libc::c_void);
        return
    } else {
        cmdlist =
            cmd_string_parse(command, 0 as *const libc::c_char, 0i32 as u_int,
                             &mut cause as *mut *mut libc::c_char);
        if cmdlist == 0 as *mut libc::c_void as *mut cmd_list {
            if cause != 0 as *mut libc::c_void as *mut libc::c_char &&
                   c != 0 as *mut libc::c_void as *mut client {
                *cause =
                    toupper(*cause as u_char as libc::c_int) as libc::c_char;
                status_message_set(c,
                                   b"%s\x00" as *const u8 as
                                       *const libc::c_char, cause);
            }
            free(cause as *mut libc::c_void);
        } else {
            new_item =
                cmdq_get_command(cmdlist, fs, 0 as *mut mouse_event, 0i32);
            cmdq_append(c, new_item);
            cmd_list_free(cmdlist);
        }
        free(command as *mut libc::c_void);
        return;
    };
}

