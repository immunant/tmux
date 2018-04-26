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
    pub type format_tree;
    pub type environ;
    pub type format_job_tree;
    pub type mode_tree_item;
    pub type screen_write_collect_item;
    pub type screen_titles;
    pub type event_base;
    pub type tmuxpeer;
    pub type options;
    pub type hooks;
    pub type evbuffer;
    pub type input_ctx;
    pub type screen_write_collect_line;
    pub type tty_code;
    pub type tmuxproc;
    pub type args_entry;
    pub type _IO_FILE_plus;
    pub type mode_tree_data;
    pub type bufferevent_ops;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
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
    fn format_true(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn format_single(_: *mut cmdq_item, _: *const libc::c_char,
                     _: *mut client, _: *mut session, _: *mut winlink,
                     _: *mut window_pane) -> *mut libc::c_char;
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
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
    fn cmd_find_clear_state(_: *mut cmd_find_state, _: libc::c_int) -> ();
    #[no_mangle]
    fn cmd_find_from_winlink_pane(_: *mut cmd_find_state, _: *mut winlink,
                                  _: *mut window_pane, _: libc::c_int) -> ();
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn cmdq_get_callback1(_: *const libc::c_char, _: cmdq_cb,
                          _: *mut libc::c_void) -> *mut cmdq_item;
    #[no_mangle]
    fn cmdq_append(_: *mut client, _: *mut cmdq_item) -> ();
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    fn server_kill_pane(_: *mut window_pane) -> ();
    #[no_mangle]
    fn server_kill_window(_: *mut window) -> ();
    #[no_mangle]
    fn server_destroy_session(_: *mut session) -> ();
    #[no_mangle]
    fn status_prompt_set(_: *mut client, _: *const libc::c_char,
                         _: *const libc::c_char, _: prompt_input_cb,
                         _: prompt_free_cb, _: *mut libc::c_void,
                         _: libc::c_int) -> ();
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    fn screen_write_puts(_: *mut screen_write_ctx, _: *const grid_cell,
                         _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn screen_write_vline(_: *mut screen_write_ctx, _: u_int, _: libc::c_int,
                          _: libc::c_int) -> ();
    #[no_mangle]
    fn screen_write_box(_: *mut screen_write_ctx, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn screen_write_preview(_: *mut screen_write_ctx, _: *mut screen,
                            _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn winlink_find_by_index(_: *mut winlinks, _: libc::c_int)
     -> *mut winlink;
    #[no_mangle]
    fn winlink_count(_: *mut winlinks) -> u_int;
    #[no_mangle]
    fn window_has_pane(_: *mut window, _: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn window_pane_index(_: *mut window_pane, _: *mut u_int) -> libc::c_int;
    #[no_mangle]
    fn window_count_panes(_: *mut window) -> u_int;
    #[no_mangle]
    fn window_pane_find_by_id(_: u_int) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_reset_mode(_: *mut window_pane) -> ();
    #[no_mangle]
    fn mode_tree_count_tagged(_: *mut mode_tree_data) -> u_int;
    #[no_mangle]
    fn mode_tree_get_current(_: *mut mode_tree_data) -> *mut libc::c_void;
    #[no_mangle]
    fn mode_tree_expand_current(_: *mut mode_tree_data) -> ();
    #[no_mangle]
    fn mode_tree_set_current(_: *mut mode_tree_data, _: uint64_t) -> ();
    #[no_mangle]
    fn mode_tree_each_tagged(_: *mut mode_tree_data, _: mode_tree_each_cb,
                             _: *mut client, _: key_code, _: libc::c_int)
     -> ();
    #[no_mangle]
    fn mode_tree_start(_: *mut window_pane, _: *mut args,
                       _: mode_tree_build_cb, _: mode_tree_draw_cb,
                       _: mode_tree_search_cb, _: *mut libc::c_void,
                       _: *mut *const libc::c_char, _: u_int,
                       _: *mut *mut screen) -> *mut mode_tree_data;
    #[no_mangle]
    fn mode_tree_zoom(_: *mut mode_tree_data, _: *mut args) -> ();
    #[no_mangle]
    fn mode_tree_build(_: *mut mode_tree_data) -> ();
    #[no_mangle]
    fn mode_tree_free(_: *mut mode_tree_data) -> ();
    #[no_mangle]
    fn mode_tree_resize(_: *mut mode_tree_data, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn mode_tree_add(_: *mut mode_tree_data, _: *mut mode_tree_item,
                     _: *mut libc::c_void, _: uint64_t,
                     _: *const libc::c_char, _: *const libc::c_char,
                     _: libc::c_int) -> *mut mode_tree_item;
    #[no_mangle]
    fn mode_tree_remove(_: *mut mode_tree_data, _: *mut mode_tree_item) -> ();
    #[no_mangle]
    fn mode_tree_draw(_: *mut mode_tree_data) -> ();
    #[no_mangle]
    fn mode_tree_key(_: *mut mode_tree_data, _: *mut client, _: *mut key_code,
                     _: *mut mouse_event, _: *mut u_int, _: *mut u_int)
     -> libc::c_int;
    #[no_mangle]
    fn mode_tree_run_command(_: *mut client, _: *mut cmd_find_state,
                             _: *const libc::c_char, _: *const libc::c_char)
     -> ();
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
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    #[no_mangle]
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    #[no_mangle]
    fn session_find_by_id(_: u_int) -> *mut session;
    #[no_mangle]
    fn session_destroy(_: *mut session, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn session_group_contains(_: *mut session) -> *mut session_group;
    #[no_mangle]
    fn osdep_get_name(_: libc::c_int, _: *mut libc::c_char)
     -> *mut libc::c_char;
}
pub const KEYC_DOUBLECLICK3_BORDER: unnamed = 268435515;
pub const LINE_SEL_NONE: unnamed_8 = 0;
pub const KEYC_PASTE_START: unnamed = 268435458;
pub type unnamed = libc::c_uint;
pub const KEYC_F8: unnamed = 268435533;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const KEYC_MOUSEUP1_STATUS: unnamed = 268435475;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const KEYC_MOUSEDRAGEND2_PANE: unnamed = 268435495;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const KEYC_F7: unnamed = 268435532;
pub const WINDOW_TREE_BY_INDEX: window_tree_sort_type = 0;
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub const KEYC_MOUSEDRAGEND2_BORDER: unnamed = 268435497;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type window_tree_type = libc::c_uint;
pub const KEYC_HOME: unnamed = 268435540;
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
    pub tree_entry: unnamed_35,
}
pub type unnamed_3 = libc::c_uint;
pub const KEYC_IC: unnamed = 268435538;
pub const KEYC_KP_MINUS: unnamed = 268435551;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_19,
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
    pub entry: unnamed_10,
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
pub const KEYC_MOUSEDRAGEND1_PANE: unnamed = 268435492;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const TTY_VT100: unnamed_3 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_18,
}
pub const KEYC_TRIPLECLICK1_BORDER: unnamed = 268435518;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const KEYC_MOUSEDRAGEND3_STATUS: unnamed = 268435499;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub type u_int = __u_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const KEYC_RIGHT: unnamed = 268435548;
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
pub const CMDQ_COMMAND: cmdq_type = 0;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub type __u_char = libc::c_uchar;
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
    pub list: unnamed_31,
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
    pub entry: unnamed_13,
    pub wentry: unnamed_5,
    pub sentry: unnamed_39,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub const KEYC_BSPACE: unnamed = 268435525;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const KEYC_TRIPLECLICK2_BORDER: unnamed = 268435521;
pub const KEYC_MOUSEUP2_PANE: unnamed = 268435477;
pub const KEYC_MOUSEUP3_STATUS: unnamed = 268435481;
pub const JOB_RUNNING: unnamed_19 = 0;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const KEYC_WHEELUP_STATUS: unnamed = 268435502;
pub const KEYC_DOUBLECLICK2_BORDER: unnamed = 268435512;
pub const KEYC_F10: unnamed = 268435535;
pub const KEYC_MOUSEMOVE_BORDER: unnamed = 268435464;
pub type speed_t = libc::c_uint;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const KEYC_WHEELUP_BORDER: unnamed = 268435503;
pub const KEYC_UP: unnamed = 268435545;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const KEYC_MOUSE: unnamed = 268435460;
pub const KEYC_MOUSEDOWN2_STATUS: unnamed = 268435469;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const KEYC_TRIPLECLICK3_BORDER: unnamed = 268435524;
pub const KEYC_KP_SLASH: unnamed = 268435549;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub const TTY_UNKNOWN: unnamed_3 = 6;
pub const KEYC_MOUSEDRAGEND1_BORDER: unnamed = 268435494;
pub const KEYC_WHEELUP_PANE: unnamed = 268435501;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
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
pub struct unnamed_7 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub type unnamed_8 = libc::c_uint;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const KEYC_MOUSEDRAGEND3_BORDER: unnamed = 268435500;
pub const KEYC_MOUSEDRAG2_PANE: unnamed = 268435486;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const JOB_CLOSED: unnamed_19 = 2;
pub const WINDOW_TREE_SESSION: window_tree_type = 1;
pub const KEYC_MOUSEUP1_PANE: unnamed = 268435474;
pub const KEYC_PASTE_END: unnamed = 268435459;
pub type uint32_t = libc::c_uint;
pub const KEYC_F6: unnamed = 268435531;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
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
pub const KEYC_TRIPLECLICK2_STATUS: unnamed = 268435520;
pub const KEYC_KP_PLUS: unnamed = 268435555;
pub const KEYC_MOUSEDOWN3_PANE: unnamed = 268435471;
pub type __u_short = libc::c_ushort;
pub type mode_tree_build_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: u_int,
                                _: *mut uint64_t, _: *const libc::c_char)
               -> ()>;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub ev_io_next: unnamed_2,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const KEYC_DOWN: unnamed = 268435546;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const KEYC_KP_SEVEN: unnamed = 268435552;
pub const KEYC_KP_TWO: unnamed = 268435560;
pub const KEYC_KP_PERIOD: unnamed = 268435564;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const KEYC_KP_ONE: unnamed = 268435559;
pub const TTY_VT320: unnamed_3 = 4;
pub const KEYC_BTAB: unnamed = 268435544;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type pid_t = __pid_t;
pub const WINDOW_TREE_BY_TIME: window_tree_sort_type = 2;
pub const KEYC_F1: unnamed = 268435526;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub type uint8_t = libc::c_uchar;
pub const KEYC_KP_FIVE: unnamed = 268435557;
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_8,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type unnamed_16 = libc::c_uint;
pub const KEYC_TRIPLECLICK1_STATUS: unnamed = 268435517;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_18 {
    offset: u_int,
    data: unnamed_34,
}
pub type key_code = libc::c_ulonglong;
pub type unnamed_19 = libc::c_uint;
pub const TTY_VT101: unnamed_3 = 1;
pub const KEYC_MOUSEDRAGEND3_PANE: unnamed = 268435498;
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
pub const KEYC_TRIPLECLICK3_STATUS: unnamed = 268435523;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_tree_modedata {
    pub wp: *mut window_pane,
    pub dead: libc::c_int,
    pub references: libc::c_int,
    pub data: *mut mode_tree_data,
    pub format: *mut libc::c_char,
    pub command: *mut libc::c_char,
    pub squash_groups: libc::c_int,
    pub item_list: *mut *mut window_tree_itemdata,
    pub item_size: u_int,
    pub entered: *const libc::c_char,
    pub fs: cmd_find_state,
    pub type_0: window_tree_type,
    pub offset: libc::c_int,
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub start: u_int,
    pub end: u_int,
    pub each: u_int,
}
pub type __time_t = libc::c_long;
pub const KEYC_MOUSEDOWN1_STATUS: unnamed = 268435466;
pub const KEYC_F5: unnamed = 268435530;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_21 {
    ev_next_with_common_timeout: unnamed_38,
    min_heap_idx: libc::c_int,
}
pub const KEYC_WHEELDOWN_PANE: unnamed = 268435504;
pub type __u_int = libc::c_uint;
pub type u_char = __u_char;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const KEYC_PPAGE: unnamed = 268435543;
pub const KEYC_FOCUS_OUT: unnamed = 268435457;
pub const KEYC_DOUBLECLICK1_PANE: unnamed = 268435507;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const KEYC_LEFT: unnamed = 268435547;
pub const KEYC_DOUBLECLICK3_STATUS: unnamed = 268435514;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_22 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const KEYC_END: unnamed = 268435541;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const KEYC_WHEELDOWN_BORDER: unnamed = 268435506;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
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
    pub entry: unnamed_9,
}
pub const KEYC_TRIPLECLICK3_PANE: unnamed = 268435522;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub ev_signal_next: unnamed_7,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub type window_tree_sort_type = libc::c_uint;
pub type options_table_type = libc::c_uint;
pub const KEYC_MOUSEDRAGEND1_STATUS: unnamed = 268435493;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_tree_itemdata {
    pub type_0: window_tree_type,
    pub session: libc::c_int,
    pub winlink: libc::c_int,
    pub pane: libc::c_int,
}
pub const KEYC_TRIPLECLICK2_PANE: unnamed = 268435519;
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
    pub prompt_mode: unnamed_16,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_17,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
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
    pub alerts_entry: unnamed_11,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_28,
    pub entry: unnamed_37,
}
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const KEYC_MOUSEDOWN2_PANE: unnamed = 268435468;
pub const KEYC_F11: unnamed = 268435536;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub const KEYC_MOUSEDRAG3_STATUS: unnamed = 268435490;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
pub const PROMPT_ENTRY: unnamed_16 = 0;
pub type cmd_find_type = libc::c_uint;
pub const KEYC_WHEELDOWN_STATUS: unnamed = 268435505;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_14,
}
pub const WINDOW_TREE_NONE: window_tree_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const KEYC_MOUSEDOWN3_BORDER: unnamed = 268435473;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_30,
}
pub const KEYC_F12: unnamed = 268435537;
pub const KEYC_F4: unnamed = 268435529;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_20,
}
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub const KEYC_TRIPLECLICK1_PANE: unnamed = 268435516;
pub const KEYC_DOUBLECLICK3_PANE: unnamed = 268435513;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const KEYC_MOUSEUP2_BORDER: unnamed = 268435479;
pub const KEYC_KP_NINE: unnamed = 268435554;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
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
    pub entry: unnamed_29,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const KEYC_KP_ZERO: unnamed = 268435563;
pub const KEYC_MOUSEDOWN2_BORDER: unnamed = 268435470;
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
pub struct unnamed_30 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type __off_t = libc::c_long;
pub type layout_type = libc::c_uint;
pub const KEYC_MOUSEDRAG2_BORDER: unnamed = 268435488;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_25,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const WINDOW_TREE_BY_NAME: window_tree_sort_type = 1;
pub const TTY_VT220: unnamed_3 = 3;
pub const KEYC_MOUSEDRAG1_STATUS: unnamed = 268435484;
pub const KEYC_FOCUS_IN: unnamed = 268435456;
pub const KEYC_MOUSEMOVE_STATUS: unnamed = 268435463;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const KEYC_MOUSEDOWN1_BORDER: unnamed = 268435467;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub const WINDOW_TREE_WINDOW: window_tree_type = 2;
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_22,
}
pub const KEYC_NPAGE: unnamed = 268435542;
pub const KEYC_F3: unnamed = 268435528;
pub const KEYC_F9: unnamed = 268435534;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type cmdq_type = libc::c_uint;
pub const KEYC_MOUSEDRAG3_PANE: unnamed = 268435489;
pub const TTY_VT420: unnamed_3 = 5;
pub const KEYC_MOUSEUP3_PANE: unnamed = 268435480;
pub const KEYC_KP_THREE: unnamed = 268435561;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_15,
    pub entry: unnamed_24,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const LINE_SEL_RIGHT_LEFT: unnamed_8 = 2;
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
pub const KEYC_DOUBLECLICK1_STATUS: unnamed = 268435508;
pub const KEYC_MOUSEDRAGEND2_STATUS: unnamed = 268435496;
pub const KEYC_MOUSEDOWN1_PANE: unnamed = 268435465;
pub const KEYC_DOUBLECLICK2_PANE: unnamed = 268435510;
pub const KEYC_F2: unnamed = 268435527;
pub type __off64_t = libc::c_long;
pub const KEYC_KP_STAR: unnamed = 268435550;
pub const KEYC_MOUSEMOVE_PANE: unnamed = 268435462;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const KEYC_KP_FOUR: unnamed = 268435556;
pub type bitstr_t = libc::c_uchar;
pub const KEYC_KP_EIGHT: unnamed = 268435553;
pub const KEYC_MOUSEDOWN3_STATUS: unnamed = 268435472;
pub const KEYC_MOUSEDRAG2_STATUS: unnamed = 268435487;
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
    pub term_type: unnamed_3,
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
pub const LINE_SEL_LEFT_RIGHT: unnamed_8 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_27,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const KEYC_KP_SIX: unnamed = 268435558;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const KEYC_DRAGGING: unnamed = 268435461;
pub const KEYC_MOUSEUP1_BORDER: unnamed = 268435476;
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_4,
    pub ev_next: unnamed_0,
    pub ev_timeout_pos: unnamed_21,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_36,
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
pub const WINDOW_TREE_PANE: window_tree_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_36 {
    ev_io: unnamed_12,
    ev_signal: unnamed_23,
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
pub type __suseconds_t = libc::c_long;
pub const KEYC_MOUSEUP2_STATUS: unnamed = 268435478;
pub type tcflag_t = libc::c_uint;
pub const PROMPT_COMMAND: unnamed_16 = 1;
pub const KEYC_MOUSEDRAG3_BORDER: unnamed = 268435491;
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
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type mode_tree_each_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: *mut client, _: key_code) -> ()>;
pub type mode_tree_draw_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: *mut screen_write_ctx, _: u_int, _: u_int)
               -> ()>;
pub type __pid_t = libc::c_int;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const KEYC_DC: unnamed = 268435539;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
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
    pub gentry: unnamed_1,
    pub entry: unnamed_6,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type uint64_t = libc::c_ulong;
pub const KEYC_DOUBLECLICK1_BORDER: unnamed = 268435509;
pub const TTY_VT102: unnamed_3 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub const KEYC_MOUSEDRAG1_PANE: unnamed = 268435483;
pub const KEYC_DOUBLECLICK2_STATUS: unnamed = 268435511;
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
pub const JOB_DEAD: unnamed_19 = 1;
pub type mode_tree_search_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: *const libc::c_char) -> libc::c_int>;
pub const KEYC_MOUSEDRAG1_BORDER: unnamed = 268435485;
pub const KEYC_MOUSEUP3_BORDER: unnamed = 268435482;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub const KEYC_KP_ENTER: unnamed = 268435562;
unsafe extern "C" fn window_tree_init(mut wp: *mut window_pane,
                                      mut fs: *mut cmd_find_state,
                                      mut args: *mut args) -> *mut screen {
    let mut data: *mut window_tree_modedata = 0 as *mut window_tree_modedata;
    let mut s: *mut screen = 0 as *mut screen;
    data =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<window_tree_modedata>() as
                    libc::c_ulong) as *mut window_tree_modedata;
    (*wp).modedata = data as *mut libc::c_void;
    if 0 != args_has(args, 115 as u_char) {
        (*data).type_0 = WINDOW_TREE_SESSION
    } else if 0 != args_has(args, 119 as u_char) {
        (*data).type_0 = WINDOW_TREE_WINDOW
    } else { (*data).type_0 = WINDOW_TREE_PANE }
    memcpy(&mut (*data).fs as *mut cmd_find_state as *mut libc::c_void,
           fs as *const libc::c_void,
           ::std::mem::size_of::<cmd_find_state>() as libc::c_ulong);
    (*data).wp = wp;
    (*data).references = 1i32;
    if args == 0 as *mut libc::c_void as *mut args ||
           0 == args_has(args, 70 as u_char) {
        (*data).format =
            xstrdup(b"#{?pane_format,#{pane_current_command} \"#{pane_title}\",#{?window_format,#{window_name}#{window_flags} (#{window_panes} panes)#{?#{==:#{window_panes},1}, \"#{pane_title}\",},#{session_windows} windows#{?session_grouped, (group #{session_group}: #{session_group_list}),}#{?session_attached, (attached),}}}\x00"
                        as *const u8 as *const libc::c_char)
    } else { (*data).format = xstrdup(args_get(args, 70 as u_char)) }
    if args == 0 as *mut libc::c_void as *mut args || (*args).argc == 0i32 {
        (*data).command =
            xstrdup(b"switch-client -t \'%%\'\x00" as *const u8 as
                        *const libc::c_char)
    } else { (*data).command = xstrdup(*(*args).argv.offset(0isize)) }
    (*data).squash_groups =
        (0 == args_has(args, 71 as u_char)) as libc::c_int;
    (*data).data =
        mode_tree_start(wp, args, Some(window_tree_build),
                        Some(window_tree_draw), Some(window_tree_search),
                        data as *mut libc::c_void,
                        window_tree_sort_list.as_mut_ptr(),
                        (::std::mem::size_of::<[*const libc::c_char; 3]>() as
                             libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                             as libc::c_ulong)
                            as u_int, &mut s as *mut *mut screen);
    mode_tree_zoom((*data).data, args);
    mode_tree_build((*data).data);
    mode_tree_draw((*data).data);
    (*data).type_0 = WINDOW_TREE_NONE;
    return s;
}
static mut window_tree_sort_list: [*const libc::c_char; 3] =
    unsafe {
        [b"index\x00" as *const u8 as *const libc::c_char,
         b"name\x00" as *const u8 as *const libc::c_char,
         b"time\x00" as *const u8 as *const libc::c_char]
    };
unsafe extern "C" fn window_tree_search(mut modedata: *mut libc::c_void,
                                        mut itemdata: *mut libc::c_void,
                                        mut ss: *const libc::c_char)
 -> libc::c_int {
    let mut item: *mut window_tree_itemdata =
        itemdata as *mut window_tree_itemdata;
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    window_tree_pull_item(item, &mut s as *mut *mut session,
                          &mut wl as *mut *mut winlink,
                          &mut wp as *mut *mut window_pane);
    match (*item).type_0 as libc::c_uint {
        0 => { return 0i32 }
        1 => {
            if s == 0 as *mut libc::c_void as *mut session {
                return 0i32
            } else {
                return (strstr((*s).name, ss) !=
                            0 as *mut libc::c_void as *mut libc::c_char) as
                           libc::c_int
            }
        }
        2 => {
            if s == 0 as *mut libc::c_void as *mut session ||
                   wl == 0 as *mut libc::c_void as *mut winlink {
                return 0i32
            } else {
                return (strstr((*(*wl).window).name, ss) !=
                            0 as *mut libc::c_void as *mut libc::c_char) as
                           libc::c_int
            }
        }
        3 => {
            if !(s == 0 as *mut libc::c_void as *mut session ||
                     wl == 0 as *mut libc::c_void as *mut winlink ||
                     wp == 0 as *mut libc::c_void as *mut window_pane) {
                cmd = osdep_get_name((*wp).fd, (*wp).tty.as_mut_ptr());
                if cmd == 0 as *mut libc::c_void as *const libc::c_char ||
                       *cmd as libc::c_int == 0 {
                    return 0i32
                } else {
                    return (strstr(cmd, ss) !=
                                0 as *mut libc::c_void as *mut libc::c_char)
                               as libc::c_int
                }
            }
        }
        _ => { }
    }
    return 0i32;
}
unsafe extern "C" fn window_tree_pull_item(mut item:
                                               *mut window_tree_itemdata,
                                           mut sp: *mut *mut session,
                                           mut wlp: *mut *mut winlink,
                                           mut wp: *mut *mut window_pane)
 -> () {
    *wp = 0 as *mut window_pane;
    *wlp = 0 as *mut winlink;
    *sp = session_find_by_id((*item).session as u_int);
    if *sp == 0 as *mut libc::c_void as *mut session {
        return
    } else if (*item).type_0 as libc::c_uint ==
                  WINDOW_TREE_SESSION as libc::c_int as libc::c_uint {
        *wlp = (**sp).curw;
        *wp = (*(**wlp).window).active;
        return
    } else {
        *wlp =
            winlink_find_by_index(&mut (**sp).windows as *mut winlinks,
                                  (*item).winlink);
        if *wlp == 0 as *mut libc::c_void as *mut winlink {
            *sp = 0 as *mut session;
            return
        } else if (*item).type_0 as libc::c_uint ==
                      WINDOW_TREE_WINDOW as libc::c_int as libc::c_uint {
            *wp = (*(**wlp).window).active;
            return
        } else {
            *wp = window_pane_find_by_id((*item).pane as u_int);
            if 0 == window_has_pane((**wlp).window, *wp) {
                *wp = 0 as *mut window_pane
            }
            if *wp == 0 as *mut libc::c_void as *mut window_pane {
                *sp = 0 as *mut session;
                *wlp = 0 as *mut winlink;
                return
            } else { return; }
        }
    };
}
unsafe extern "C" fn window_tree_draw(mut modedata: *mut libc::c_void,
                                      mut itemdata: *mut libc::c_void,
                                      mut ctx: *mut screen_write_ctx,
                                      mut sx: u_int, mut sy: u_int) -> () {
    let mut item: *mut window_tree_itemdata =
        itemdata as *mut window_tree_itemdata;
    let mut sp: *mut session = 0 as *mut session;
    let mut wlp: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    window_tree_pull_item(item, &mut sp as *mut *mut session,
                          &mut wlp as *mut *mut winlink,
                          &mut wp as *mut *mut window_pane);
    if wp == 0 as *mut libc::c_void as *mut window_pane {
        return
    } else {
        match (*item).type_0 as libc::c_uint {
            1 => {
                window_tree_draw_session(modedata as
                                             *mut window_tree_modedata, sp,
                                         ctx, sx, sy);
            }
            2 => {
                window_tree_draw_window(modedata as *mut window_tree_modedata,
                                        sp, (*wlp).window, ctx, sx, sy);
            }
            3 => {
                screen_write_preview(ctx, &mut (*wp).base as *mut screen, sx,
                                     sy);
            }
            0 | _ => { }
        }
        return;
    };
}
unsafe extern "C" fn window_tree_draw_window(mut data:
                                                 *mut window_tree_modedata,
                                             mut s: *mut session,
                                             mut w: *mut window,
                                             mut ctx: *mut screen_write_ctx,
                                             mut sx: u_int, mut sy: u_int)
 -> () {
    let mut oo: *mut options = (*s).options;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut cx: u_int = (*(*ctx).s).cx;
    let mut cy: u_int = (*(*ctx).s).cy;
    let mut loop_0: u_int = 0;
    let mut total: u_int = 0;
    let mut visible: u_int = 0;
    let mut each: u_int = 0;
    let mut width: u_int = 0;
    let mut offset: u_int = 0;
    let mut current: u_int = 0;
    let mut start: u_int = 0;
    let mut end: u_int = 0;
    let mut remaining: u_int = 0;
    let mut i: u_int = 0;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut colour: libc::c_int = 0;
    let mut active_colour: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut pane_idx: libc::c_int = 0;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    total = window_count_panes(w);
    memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    colour =
        options_get_number(oo,
                           b"display-panes-colour\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    active_colour =
        options_get_number(oo,
                           b"display-panes-active-colour\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    if sx.wrapping_div(total) < 24i32 as libc::c_uint {
        visible = sx.wrapping_div(24i32 as libc::c_uint);
        if visible == 0i32 as libc::c_uint { visible = 1i32 as u_int }
    } else { visible = total }
    current = 0i32 as u_int;
    wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
    while wp != 0 as *mut libc::c_void as *mut window_pane {
        if wp == (*w).active { break ; }
        current = current.wrapping_add(1);
        wp = (*wp).entry.tqe_next
    }
    if current < visible {
        start = 0i32 as u_int;
        end = visible
    } else if current >= total.wrapping_sub(visible) {
        start = total.wrapping_sub(visible);
        end = total
    } else {
        start =
            current.wrapping_sub(visible.wrapping_div(2i32 as libc::c_uint));
        end = start.wrapping_add(visible)
    }
    if (*data).offset < (start as libc::c_int).wrapping_neg() {
        (*data).offset = (start as libc::c_int).wrapping_neg()
    }
    if (*data).offset > total.wrapping_sub(end) as libc::c_int {
        (*data).offset = total.wrapping_sub(end) as libc::c_int
    }
    start =
        (start as libc::c_uint).wrapping_add((*data).offset as libc::c_uint)
            as u_int as u_int;
    end =
        (end as libc::c_uint).wrapping_add((*data).offset as libc::c_uint) as
            u_int as u_int;
    left = (start != 0i32 as libc::c_uint) as libc::c_int;
    right = (end != total) as libc::c_int;
    if 0 != left && 0 != right && sx <= 6i32 as libc::c_uint ||
           (0 != left || 0 != right) && sx <= 3i32 as libc::c_uint {
        right = 0i32;
        left = right
    }
    if 0 != left && 0 != right {
        each = sx.wrapping_sub(6i32 as libc::c_uint).wrapping_div(visible);
        remaining =
            sx.wrapping_sub(6i32 as
                                libc::c_uint).wrapping_sub(visible.wrapping_mul(each))
    } else if 0 != left || 0 != right {
        each = sx.wrapping_sub(3i32 as libc::c_uint).wrapping_div(visible);
        remaining =
            sx.wrapping_sub(3i32 as
                                libc::c_uint).wrapping_sub(visible.wrapping_mul(each))
    } else {
        each = sx.wrapping_div(visible);
        remaining = sx.wrapping_sub(visible.wrapping_mul(each))
    }
    if each == 0i32 as libc::c_uint {
        return
    } else {
        if 0 != left {
            (*data).left =
                cx.wrapping_add(2i32 as libc::c_uint) as libc::c_int;
            screen_write_cursormove(ctx,
                                    cx.wrapping_add(2i32 as libc::c_uint),
                                    cy);
            screen_write_vline(ctx, sy, 0i32, 0i32);
            screen_write_cursormove(ctx, cx,
                                    cy.wrapping_add(sy.wrapping_div(2i32 as
                                                                        libc::c_uint)));
            screen_write_puts(ctx, &grid_default_cell as *const grid_cell,
                              b"<\x00" as *const u8 as *const libc::c_char);
        } else { (*data).left = 1i32.wrapping_neg() }
        if 0 != right {
            (*data).right =
                cx.wrapping_add(sx).wrapping_sub(3i32 as libc::c_uint) as
                    libc::c_int;
            screen_write_cursormove(ctx,
                                    cx.wrapping_add(sx).wrapping_sub(3i32 as
                                                                         libc::c_uint),
                                    cy);
            screen_write_vline(ctx, sy, 0i32, 0i32);
            screen_write_cursormove(ctx,
                                    cx.wrapping_add(sx).wrapping_sub(1i32 as
                                                                         libc::c_uint),
                                    cy.wrapping_add(sy.wrapping_div(2i32 as
                                                                        libc::c_uint)));
            screen_write_puts(ctx, &grid_default_cell as *const grid_cell,
                              b">\x00" as *const u8 as *const libc::c_char);
        } else { (*data).right = 1i32.wrapping_neg() }
        (*data).start = start;
        (*data).end = end;
        (*data).each = each;
        loop_0 = 0i32 as u_int;
        i = loop_0;
        wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
        while wp != 0 as *mut libc::c_void as *mut window_pane {
            if loop_0 == end { break ; }
            if loop_0 < start {
                loop_0 = loop_0.wrapping_add(1)
            } else {
                if wp == (*w).active {
                    gc.fg = active_colour
                } else { gc.fg = colour }
                if 0 != left {
                    offset =
                        (3i32 as
                             libc::c_uint).wrapping_add(i.wrapping_mul(each))
                } else { offset = i.wrapping_mul(each) }
                if loop_0 == end.wrapping_sub(1i32 as libc::c_uint) {
                    width = each.wrapping_add(remaining)
                } else { width = each.wrapping_sub(1i32 as libc::c_uint) }
                screen_write_cursormove(ctx, cx.wrapping_add(offset), cy);
                screen_write_preview(ctx, &mut (*wp).base as *mut screen,
                                     width, sy);
                if window_pane_index(wp,
                                     &mut pane_idx as *mut libc::c_int as
                                         *mut u_int) != 0i32 {
                    pane_idx = loop_0 as libc::c_int
                }
                xasprintf(&mut label as *mut *mut libc::c_char,
                          b" %u \x00" as *const u8 as *const libc::c_char,
                          pane_idx);
                window_tree_draw_label(ctx, cx.wrapping_add(offset), cy, each,
                                       sy, &mut gc as *mut grid_cell, label);
                free(label as *mut libc::c_void);
                if loop_0 != end.wrapping_sub(1i32 as libc::c_uint) {
                    screen_write_cursormove(ctx,
                                            cx.wrapping_add(offset).wrapping_add(width),
                                            cy);
                    screen_write_vline(ctx, sy, 0i32, 0i32);
                }
                loop_0 = loop_0.wrapping_add(1);
                i = i.wrapping_add(1)
            }
            wp = (*wp).entry.tqe_next
        }
        return;
    };
}
unsafe extern "C" fn window_tree_draw_label(mut ctx: *mut screen_write_ctx,
                                            mut px: u_int, mut py: u_int,
                                            mut sx: u_int, mut sy: u_int,
                                            mut gc: *const grid_cell,
                                            mut label: *const libc::c_char)
 -> () {
    let mut len: size_t = 0;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    len = strlen(label);
    if sx == 0i32 as libc::c_uint || sy == 1i32 as libc::c_uint ||
           len > sx as libc::c_ulong {
        return
    } else {
        ox =
            (sx as
                 libc::c_ulong).wrapping_sub(len).wrapping_add(1i32 as
                                                                   libc::c_ulong).wrapping_div(2i32
                                                                                                   as
                                                                                                   libc::c_ulong)
                as u_int;
        oy =
            sy.wrapping_add(1i32 as
                                libc::c_uint).wrapping_div(2i32 as
                                                               libc::c_uint);
        if ox > 1i32 as libc::c_uint &&
               (ox as libc::c_ulong).wrapping_add(len) <
                   sx.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong &&
               sy >= 3i32 as libc::c_uint {
            screen_write_cursormove(ctx,
                                    px.wrapping_add(ox).wrapping_sub(1i32 as
                                                                         libc::c_uint),
                                    py.wrapping_add(oy).wrapping_sub(1i32 as
                                                                         libc::c_uint));
            screen_write_box(ctx,
                             len.wrapping_add(2i32 as libc::c_ulong) as u_int,
                             3i32 as u_int);
        }
        screen_write_cursormove(ctx, px.wrapping_add(ox),
                                py.wrapping_add(oy));
        screen_write_puts(ctx, gc,
                          b"%s\x00" as *const u8 as *const libc::c_char,
                          label);
        return;
    };
}
unsafe extern "C" fn window_tree_draw_session(mut data:
                                                  *mut window_tree_modedata,
                                              mut s: *mut session,
                                              mut ctx: *mut screen_write_ctx,
                                              mut sx: u_int, mut sy: u_int)
 -> () {
    let mut oo: *mut options = (*s).options;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut w: *mut window = 0 as *mut window;
    let mut cx: u_int = (*(*ctx).s).cx;
    let mut cy: u_int = (*(*ctx).s).cy;
    let mut loop_0: u_int = 0;
    let mut total: u_int = 0;
    let mut visible: u_int = 0;
    let mut each: u_int = 0;
    let mut width: u_int = 0;
    let mut offset: u_int = 0;
    let mut current: u_int = 0;
    let mut start: u_int = 0;
    let mut end: u_int = 0;
    let mut remaining: u_int = 0;
    let mut i: u_int = 0;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut colour: libc::c_int = 0;
    let mut active_colour: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    total = winlink_count(&mut (*s).windows as *mut winlinks);
    memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    colour =
        options_get_number(oo,
                           b"display-panes-colour\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    active_colour =
        options_get_number(oo,
                           b"display-panes-active-colour\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    if sx.wrapping_div(total) < 24i32 as libc::c_uint {
        visible = sx.wrapping_div(24i32 as libc::c_uint);
        if visible == 0i32 as libc::c_uint { visible = 1i32 as u_int }
    } else { visible = total }
    current = 0i32 as u_int;
    wl =
        winlinks_RB_MINMAX(&mut (*s).windows as *mut winlinks,
                           1i32.wrapping_neg());
    while wl != 0 as *mut libc::c_void as *mut winlink {
        if wl == (*s).curw { break ; }
        current = current.wrapping_add(1);
        wl = winlinks_RB_NEXT(wl)
    }
    if current < visible {
        start = 0i32 as u_int;
        end = visible
    } else if current >= total.wrapping_sub(visible) {
        start = total.wrapping_sub(visible);
        end = total
    } else {
        start =
            current.wrapping_sub(visible.wrapping_div(2i32 as libc::c_uint));
        end = start.wrapping_add(visible)
    }
    if (*data).offset < (start as libc::c_int).wrapping_neg() {
        (*data).offset = (start as libc::c_int).wrapping_neg()
    }
    if (*data).offset > total.wrapping_sub(end) as libc::c_int {
        (*data).offset = total.wrapping_sub(end) as libc::c_int
    }
    start =
        (start as libc::c_uint).wrapping_add((*data).offset as libc::c_uint)
            as u_int as u_int;
    end =
        (end as libc::c_uint).wrapping_add((*data).offset as libc::c_uint) as
            u_int as u_int;
    left = (start != 0i32 as libc::c_uint) as libc::c_int;
    right = (end != total) as libc::c_int;
    if 0 != left && 0 != right && sx <= 6i32 as libc::c_uint ||
           (0 != left || 0 != right) && sx <= 3i32 as libc::c_uint {
        right = 0i32;
        left = right
    }
    if 0 != left && 0 != right {
        each = sx.wrapping_sub(6i32 as libc::c_uint).wrapping_div(visible);
        remaining =
            sx.wrapping_sub(6i32 as
                                libc::c_uint).wrapping_sub(visible.wrapping_mul(each))
    } else if 0 != left || 0 != right {
        each = sx.wrapping_sub(3i32 as libc::c_uint).wrapping_div(visible);
        remaining =
            sx.wrapping_sub(3i32 as
                                libc::c_uint).wrapping_sub(visible.wrapping_mul(each))
    } else {
        each = sx.wrapping_div(visible);
        remaining = sx.wrapping_sub(visible.wrapping_mul(each))
    }
    if each == 0i32 as libc::c_uint {
        return
    } else {
        if 0 != left {
            (*data).left =
                cx.wrapping_add(2i32 as libc::c_uint) as libc::c_int;
            screen_write_cursormove(ctx,
                                    cx.wrapping_add(2i32 as libc::c_uint),
                                    cy);
            screen_write_vline(ctx, sy, 0i32, 0i32);
            screen_write_cursormove(ctx, cx,
                                    cy.wrapping_add(sy.wrapping_div(2i32 as
                                                                        libc::c_uint)));
            screen_write_puts(ctx, &grid_default_cell as *const grid_cell,
                              b"<\x00" as *const u8 as *const libc::c_char);
        } else { (*data).left = 1i32.wrapping_neg() }
        if 0 != right {
            (*data).right =
                cx.wrapping_add(sx).wrapping_sub(3i32 as libc::c_uint) as
                    libc::c_int;
            screen_write_cursormove(ctx,
                                    cx.wrapping_add(sx).wrapping_sub(3i32 as
                                                                         libc::c_uint),
                                    cy);
            screen_write_vline(ctx, sy, 0i32, 0i32);
            screen_write_cursormove(ctx,
                                    cx.wrapping_add(sx).wrapping_sub(1i32 as
                                                                         libc::c_uint),
                                    cy.wrapping_add(sy.wrapping_div(2i32 as
                                                                        libc::c_uint)));
            screen_write_puts(ctx, &grid_default_cell as *const grid_cell,
                              b">\x00" as *const u8 as *const libc::c_char);
        } else { (*data).right = 1i32.wrapping_neg() }
        (*data).start = start;
        (*data).end = end;
        (*data).each = each;
        loop_0 = 0i32 as u_int;
        i = loop_0;
        wl =
            winlinks_RB_MINMAX(&mut (*s).windows as *mut winlinks,
                               1i32.wrapping_neg());
        while wl != 0 as *mut libc::c_void as *mut winlink {
            if loop_0 == end { break ; }
            if loop_0 < start {
                loop_0 = loop_0.wrapping_add(1)
            } else {
                w = (*wl).window;
                if wl == (*s).curw {
                    gc.fg = active_colour
                } else { gc.fg = colour }
                if 0 != left {
                    offset =
                        (3i32 as
                             libc::c_uint).wrapping_add(i.wrapping_mul(each))
                } else { offset = i.wrapping_mul(each) }
                if loop_0 == end.wrapping_sub(1i32 as libc::c_uint) {
                    width = each.wrapping_add(remaining)
                } else { width = each.wrapping_sub(1i32 as libc::c_uint) }
                screen_write_cursormove(ctx, cx.wrapping_add(offset), cy);
                screen_write_preview(ctx,
                                     &mut (*(*w).active).base as *mut screen,
                                     width, sy);
                xasprintf(&mut label as *mut *mut libc::c_char,
                          b" %u:%s \x00" as *const u8 as *const libc::c_char,
                          (*wl).idx, (*w).name);
                if strlen(label) > width as libc::c_ulong {
                    xasprintf(&mut label as *mut *mut libc::c_char,
                              b" %u \x00" as *const u8 as *const libc::c_char,
                              (*wl).idx);
                }
                window_tree_draw_label(ctx, cx.wrapping_add(offset), cy,
                                       width, sy, &mut gc as *mut grid_cell,
                                       label);
                free(label as *mut libc::c_void);
                if loop_0 != end.wrapping_sub(1i32 as libc::c_uint) {
                    screen_write_cursormove(ctx,
                                            cx.wrapping_add(offset).wrapping_add(width),
                                            cy);
                    screen_write_vline(ctx, sy, 0i32, 0i32);
                }
                loop_0 = loop_0.wrapping_add(1);
                i = i.wrapping_add(1)
            }
            wl = winlinks_RB_NEXT(wl)
        }
        return;
    };
}
unsafe extern "C" fn window_tree_build(mut modedata: *mut libc::c_void,
                                       mut sort_type: u_int,
                                       mut tag: *mut uint64_t,
                                       mut filter: *const libc::c_char)
 -> () {
    let mut current_block: u64;
    let mut data: *mut window_tree_modedata =
        modedata as *mut window_tree_modedata;
    let mut s: *mut session = 0 as *mut session;
    let mut l: *mut *mut session = 0 as *mut *mut session;
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut current: *mut session_group = 0 as *mut session_group;
    let mut n: u_int = 0;
    let mut i: u_int = 0;
    current = session_group_contains((*data).fs.s);
    i = 0i32 as u_int;
    while i < (*data).item_size {
        window_tree_free_item(*(*data).item_list.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free((*data).item_list as *mut libc::c_void);
    (*data).item_list = 0 as *mut *mut window_tree_itemdata;
    (*data).item_size = 0i32 as u_int;
    l = 0 as *mut *mut session;
    n = 0i32 as u_int;
    s =
        sessions_RB_MINMAX(&mut sessions as *mut sessions,
                           1i32.wrapping_neg());
    while s != 0 as *mut libc::c_void as *mut session {
        if 0 != (*data).squash_groups &&
               {
                   sg = session_group_contains(s);
                   sg != 0 as *mut libc::c_void as *mut session_group
               } {
            if sg == current && s != (*data).fs.s ||
                   sg != current &&
                       s !=
                           (*(&mut (*sg).sessions as
                                  *mut unnamed_15)).tqh_first {
                current_block = 17179679302217393232;
            } else { current_block = 820271813250567934; }
        } else { current_block = 820271813250567934; }
        match current_block {
            820271813250567934 => {
                l =
                    xreallocarray(l as *mut libc::c_void,
                                  n.wrapping_add(1i32 as libc::c_uint) as
                                      size_t,
                                  ::std::mem::size_of::<*mut session>() as
                                      libc::c_ulong) as *mut *mut session;
                let fresh0 = n;
                n = n.wrapping_add(1);
                let ref mut fresh1 = *l.offset(fresh0 as isize);
                *fresh1 = s
            }
            _ => { }
        }
        s = sessions_RB_NEXT(s)
    }
    match sort_type {
        1 => {
            qsort(l as *mut libc::c_void, n as size_t,
                  ::std::mem::size_of::<*mut session>() as libc::c_ulong,
                  Some(window_tree_cmp_session_name));
        }
        2 => {
            qsort(l as *mut libc::c_void, n as size_t,
                  ::std::mem::size_of::<*mut session>() as libc::c_ulong,
                  Some(window_tree_cmp_session_time));
        }
        0 | _ => { }
    }
    i = 0i32 as u_int;
    while i < n {
        window_tree_build_session(*l.offset(i as isize), modedata, sort_type,
                                  filter);
        i = i.wrapping_add(1)
    }
    free(l as *mut libc::c_void);
    match (*data).type_0 as libc::c_uint {
        1 => { *tag = (*data).fs.s as uint64_t }
        2 => { *tag = (*data).fs.wl as uint64_t }
        3 => {
            if window_count_panes((*(*data).fs.wl).window) ==
                   1i32 as libc::c_uint {
                *tag = (*data).fs.wl as uint64_t
            } else { *tag = (*data).fs.wp as uint64_t }
        }
        0 | _ => { }
    };
}
unsafe extern "C" fn window_tree_build_session(mut s: *mut session,
                                               mut modedata:
                                                   *mut libc::c_void,
                                               mut sort_type: u_int,
                                               mut filter:
                                                   *const libc::c_char)
 -> () {
    let mut data: *mut window_tree_modedata =
        modedata as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut l: *mut *mut winlink = 0 as *mut *mut winlink;
    let mut n: u_int = 0;
    let mut i: u_int = 0;
    let mut empty: u_int = 0;
    let mut expanded: libc::c_int = 0;
    item = window_tree_add_item(data);
    (*item).type_0 = WINDOW_TREE_SESSION;
    (*item).session = (*s).id as libc::c_int;
    (*item).winlink = 1i32.wrapping_neg();
    (*item).pane = 1i32.wrapping_neg();
    text =
        format_single(0 as *mut cmdq_item, (*data).format, 0 as *mut client,
                      s, 0 as *mut winlink, 0 as *mut window_pane);
    if (*data).type_0 as libc::c_uint ==
           WINDOW_TREE_SESSION as libc::c_int as libc::c_uint {
        expanded = 0i32
    } else { expanded = 1i32 }
    mti =
        mode_tree_add((*data).data, 0 as *mut mode_tree_item,
                      item as *mut libc::c_void, s as uint64_t, (*s).name,
                      text, expanded);
    free(text as *mut libc::c_void);
    l = 0 as *mut *mut winlink;
    n = 0i32 as u_int;
    wl =
        winlinks_RB_MINMAX(&mut (*s).windows as *mut winlinks,
                           1i32.wrapping_neg());
    while wl != 0 as *mut libc::c_void as *mut winlink {
        l =
            xreallocarray(l as *mut libc::c_void,
                          n.wrapping_add(1i32 as libc::c_uint) as size_t,
                          ::std::mem::size_of::<*mut winlink>() as
                              libc::c_ulong) as *mut *mut winlink;
        let fresh2 = n;
        n = n.wrapping_add(1);
        let ref mut fresh3 = *l.offset(fresh2 as isize);
        *fresh3 = wl;
        wl = winlinks_RB_NEXT(wl)
    }
    match sort_type {
        1 => {
            qsort(l as *mut libc::c_void, n as size_t,
                  ::std::mem::size_of::<*mut winlink>() as libc::c_ulong,
                  Some(window_tree_cmp_window_name));
        }
        2 => {
            qsort(l as *mut libc::c_void, n as size_t,
                  ::std::mem::size_of::<*mut winlink>() as libc::c_ulong,
                  Some(window_tree_cmp_window_time));
        }
        0 | _ => { }
    }
    empty = 0i32 as u_int;
    i = 0i32 as u_int;
    while i < n {
        if 0 ==
               window_tree_build_window(s, *l.offset(i as isize), modedata,
                                        sort_type, mti, filter) {
            empty = empty.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    if empty == n {
        window_tree_free_item(item);
        (*data).item_size = (*data).item_size.wrapping_sub(1);
        mode_tree_remove((*data).data, mti);
    }
    free(l as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_free_item(mut item:
                                               *mut window_tree_itemdata)
 -> () {
    free(item as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_build_window(mut s: *mut session,
                                              mut wl: *mut winlink,
                                              mut modedata: *mut libc::c_void,
                                              mut sort_type: u_int,
                                              mut parent: *mut mode_tree_item,
                                              mut filter: *const libc::c_char)
 -> libc::c_int {
    let mut data: *mut window_tree_modedata =
        modedata as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut l: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut n: u_int = 0;
    let mut i: u_int = 0;
    let mut expanded: libc::c_int = 0;
    item = window_tree_add_item(data);
    (*item).type_0 = WINDOW_TREE_WINDOW;
    (*item).session = (*s).id as libc::c_int;
    (*item).winlink = (*wl).idx;
    (*item).pane = 1i32.wrapping_neg();
    text =
        format_single(0 as *mut cmdq_item, (*data).format, 0 as *mut client,
                      s, wl, 0 as *mut window_pane);
    xasprintf(&mut name as *mut *mut libc::c_char,
              b"%u\x00" as *const u8 as *const libc::c_char, (*wl).idx);
    if (*data).type_0 as libc::c_uint ==
           WINDOW_TREE_SESSION as libc::c_int as libc::c_uint ||
           (*data).type_0 as libc::c_uint ==
               WINDOW_TREE_WINDOW as libc::c_int as libc::c_uint {
        expanded = 0i32
    } else { expanded = 1i32 }
    mti =
        mode_tree_add((*data).data, parent, item as *mut libc::c_void,
                      wl as uint64_t, name, text, expanded);
    free(text as *mut libc::c_void);
    free(name as *mut libc::c_void);
    wp = (*(&mut (*(*wl).window).panes as *mut window_panes)).tqh_first;
    if !(wp == 0 as *mut libc::c_void as *mut window_pane) {
        if (*wp).entry.tqe_next == 0 as *mut libc::c_void as *mut window_pane
           {
            if !(0 == window_tree_filter_pane(s, wl, wp, filter)) {
                return 1i32
            }
        } else {
            l = 0 as *mut *mut window_pane;
            n = 0i32 as u_int;
            wp =
                (*(&mut (*(*wl).window).panes as
                       *mut window_panes)).tqh_first;
            while wp != 0 as *mut libc::c_void as *mut window_pane {
                if !(0 == window_tree_filter_pane(s, wl, wp, filter)) {
                    l =
                        xreallocarray(l as *mut libc::c_void,
                                      n.wrapping_add(1i32 as libc::c_uint) as
                                          size_t,
                                      ::std::mem::size_of::<*mut window_pane>()
                                          as libc::c_ulong) as
                            *mut *mut window_pane;
                    let fresh4 = n;
                    n = n.wrapping_add(1);
                    let ref mut fresh5 = *l.offset(fresh4 as isize);
                    *fresh5 = wp
                }
                wp = (*wp).entry.tqe_next
            }
            if !(n == 0i32 as libc::c_uint) {
                match sort_type {
                    2 => {
                        qsort(l as *mut libc::c_void, n as size_t,
                              ::std::mem::size_of::<*mut window_pane>() as
                                  libc::c_ulong,
                              Some(window_tree_cmp_pane_time));
                    }
                    0 | 1 | _ => { }
                }
                i = 0i32 as u_int;
                while i < n {
                    window_tree_build_pane(s, wl, *l.offset(i as isize),
                                           modedata, mti);
                    i = i.wrapping_add(1)
                }
                free(l as *mut libc::c_void);
                return 1i32
            }
        }
    }
    window_tree_free_item(item);
    (*data).item_size = (*data).item_size.wrapping_sub(1);
    mode_tree_remove((*data).data, mti);
    return 0i32;
}
unsafe extern "C" fn window_tree_build_pane(mut s: *mut session,
                                            mut wl: *mut winlink,
                                            mut wp: *mut window_pane,
                                            mut modedata: *mut libc::c_void,
                                            mut parent: *mut mode_tree_item)
 -> () {
    let mut data: *mut window_tree_modedata =
        modedata as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: u_int = 0;
    window_pane_index(wp, &mut idx as *mut u_int);
    item = window_tree_add_item(data);
    (*item).type_0 = WINDOW_TREE_PANE;
    (*item).session = (*s).id as libc::c_int;
    (*item).winlink = (*wl).idx;
    (*item).pane = (*wp).id as libc::c_int;
    text =
        format_single(0 as *mut cmdq_item, (*data).format, 0 as *mut client,
                      s, wl, wp);
    xasprintf(&mut name as *mut *mut libc::c_char,
              b"%u\x00" as *const u8 as *const libc::c_char, idx);
    mode_tree_add((*data).data, parent, item as *mut libc::c_void,
                  wp as uint64_t, name, text, 1i32.wrapping_neg());
    free(text as *mut libc::c_void);
    free(name as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_add_item(mut data: *mut window_tree_modedata)
 -> *mut window_tree_itemdata {
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    (*data).item_list =
        xreallocarray((*data).item_list as *mut libc::c_void,
                      (*data).item_size.wrapping_add(1i32 as libc::c_uint) as
                          size_t,
                      ::std::mem::size_of::<*mut window_tree_itemdata>() as
                          libc::c_ulong) as *mut *mut window_tree_itemdata;
    let fresh6 = (*data).item_size;
    (*data).item_size = (*data).item_size.wrapping_add(1);
    let ref mut fresh7 = *(*data).item_list.offset(fresh6 as isize);
    *fresh7 =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<window_tree_itemdata>() as
                    libc::c_ulong) as *mut window_tree_itemdata;
    item = *fresh7;
    return item;
}
unsafe extern "C" fn window_tree_cmp_pane_time(mut a0: *const libc::c_void,
                                               mut b0: *const libc::c_void)
 -> libc::c_int {
    let mut a: *const *const window_pane = a0 as *const *const window_pane;
    let mut b: *const *const window_pane = b0 as *const *const window_pane;
    if (**a).active_point < (**b).active_point {
        return 1i32.wrapping_neg()
    } else if (**a).active_point > (**b).active_point {
        return 1i32
    } else { return 0i32 };
}
unsafe extern "C" fn window_tree_filter_pane(mut s: *mut session,
                                             mut wl: *mut winlink,
                                             mut wp: *mut window_pane,
                                             mut filter: *const libc::c_char)
 -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: libc::c_int = 0;
    if filter == 0 as *mut libc::c_void as *const libc::c_char {
        return 1i32
    } else {
        cp =
            format_single(0 as *mut cmdq_item, filter, 0 as *mut client, s,
                          wl, wp);
        result = format_true(cp);
        free(cp as *mut libc::c_void);
        return result
    };
}
unsafe extern "C" fn window_tree_cmp_window_time(mut a0: *const libc::c_void,
                                                 mut b0: *const libc::c_void)
 -> libc::c_int {
    let mut a: *const *const winlink = a0 as *const *const winlink;
    let mut b: *const *const winlink = b0 as *const *const winlink;
    if 0 !=
           if (*(&mut (*(**a).window).activity_time as *mut timeval)).tv_sec
                  ==
                  (*(&mut (*(**b).window).activity_time as
                         *mut timeval)).tv_sec {
               ((*(&mut (*(**a).window).activity_time as
                       *mut timeval)).tv_usec >
                    (*(&mut (*(**b).window).activity_time as
                           *mut timeval)).tv_usec) as libc::c_int
           } else {
               ((*(&mut (*(**a).window).activity_time as *mut timeval)).tv_sec
                    >
                    (*(&mut (*(**b).window).activity_time as
                           *mut timeval)).tv_sec) as libc::c_int
           } {
        return 1i32.wrapping_neg()
    } else if 0 !=
                  if (*(&mut (*(**a).window).activity_time as
                            *mut timeval)).tv_sec ==
                         (*(&mut (*(**b).window).activity_time as
                                *mut timeval)).tv_sec {
                      ((*(&mut (*(**a).window).activity_time as
                              *mut timeval)).tv_usec <
                           (*(&mut (*(**b).window).activity_time as
                                  *mut timeval)).tv_usec) as libc::c_int
                  } else {
                      ((*(&mut (*(**a).window).activity_time as
                              *mut timeval)).tv_sec <
                           (*(&mut (*(**b).window).activity_time as
                                  *mut timeval)).tv_sec) as libc::c_int
                  } {
        return 1i32
    } else { return strcmp((*(**a).window).name, (*(**b).window).name) };
}
unsafe extern "C" fn window_tree_cmp_window_name(mut a0: *const libc::c_void,
                                                 mut b0: *const libc::c_void)
 -> libc::c_int {
    let mut a: *const *const winlink = a0 as *const *const winlink;
    let mut b: *const *const winlink = b0 as *const *const winlink;
    return strcmp((*(**a).window).name, (*(**b).window).name);
}
unsafe extern "C" fn window_tree_cmp_session_time(mut a0: *const libc::c_void,
                                                  mut b0: *const libc::c_void)
 -> libc::c_int {
    let mut a: *const *const session = a0 as *const *const session;
    let mut b: *const *const session = b0 as *const *const session;
    if 0 !=
           if (*(&(**a).activity_time as *const timeval)).tv_sec ==
                  (*(&(**b).activity_time as *const timeval)).tv_sec {
               ((*(&(**a).activity_time as *const timeval)).tv_usec >
                    (*(&(**b).activity_time as *const timeval)).tv_usec) as
                   libc::c_int
           } else {
               ((*(&(**a).activity_time as *const timeval)).tv_sec >
                    (*(&(**b).activity_time as *const timeval)).tv_sec) as
                   libc::c_int
           } {
        return 1i32.wrapping_neg()
    } else if 0 !=
                  if (*(&(**a).activity_time as *const timeval)).tv_sec ==
                         (*(&(**b).activity_time as *const timeval)).tv_sec {
                      ((*(&(**a).activity_time as *const timeval)).tv_usec <
                           (*(&(**b).activity_time as
                                  *const timeval)).tv_usec) as libc::c_int
                  } else {
                      ((*(&(**a).activity_time as *const timeval)).tv_sec <
                           (*(&(**b).activity_time as *const timeval)).tv_sec)
                          as libc::c_int
                  } {
        return 1i32
    } else { return strcmp((**a).name, (**b).name) };
}
unsafe extern "C" fn window_tree_cmp_session_name(mut a0: *const libc::c_void,
                                                  mut b0: *const libc::c_void)
 -> libc::c_int {
    let mut a: *const *const session = a0 as *const *const session;
    let mut b: *const *const session = b0 as *const *const session;
    return strcmp((**a).name, (**b).name);
}
unsafe extern "C" fn window_tree_free(mut wp: *mut window_pane) -> () {
    let mut data: *mut window_tree_modedata =
        (*wp).modedata as *mut window_tree_modedata;
    if data == 0 as *mut libc::c_void as *mut window_tree_modedata {
        return
    } else {
        (*data).dead = 1i32;
        mode_tree_free((*data).data);
        window_tree_destroy(data);
        return;
    };
}
unsafe extern "C" fn window_tree_destroy(mut data: *mut window_tree_modedata)
 -> () {
    let mut i: u_int = 0;
    (*data).references -= 1;
    if (*data).references != 0i32 {
        return
    } else {
        i = 0i32 as u_int;
        while i < (*data).item_size {
            window_tree_free_item(*(*data).item_list.offset(i as isize));
            i = i.wrapping_add(1)
        }
        free((*data).item_list as *mut libc::c_void);
        free((*data).format as *mut libc::c_void);
        free((*data).command as *mut libc::c_void);
        free(data as *mut libc::c_void);
        return;
    };
}
unsafe extern "C" fn window_tree_resize(mut wp: *mut window_pane,
                                        mut sx: u_int, mut sy: u_int) -> () {
    let mut data: *mut window_tree_modedata =
        (*wp).modedata as *mut window_tree_modedata;
    mode_tree_resize((*data).data, sx, sy);
}
unsafe extern "C" fn window_tree_key(mut wp: *mut window_pane,
                                     mut c: *mut client, mut s: *mut session,
                                     mut key: key_code,
                                     mut m: *mut mouse_event) -> () {
    let mut data: *mut window_tree_modedata =
        (*wp).modedata as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut new_item: *mut window_tree_itemdata =
        0 as *mut window_tree_itemdata;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fs: cmd_find_state =
        cmd_find_state{flags: 0,
                       current:
                           0 as *const cmd_find_state as *mut cmd_find_state,
                       s: 0 as *const session as *mut session,
                       wl: 0 as *const winlink as *mut winlink,
                       w: 0 as *const window as *mut window,
                       wp: 0 as *const window_pane as *mut window_pane,
                       idx: 0,};
    let mut finished: libc::c_int = 0;
    let mut tagged: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut idx: u_int = 0;
    let mut ns: *mut session = 0 as *mut session;
    let mut nwl: *mut winlink = 0 as *mut winlink;
    let mut nwp: *mut window_pane = 0 as *mut window_pane;
    item = mode_tree_get_current((*data).data) as *mut window_tree_itemdata;
    finished =
        mode_tree_key((*data).data, c, &mut key as *mut key_code, m,
                      &mut x as *mut u_int, &mut y as *mut u_int);
    new_item =
        mode_tree_get_current((*data).data) as *mut window_tree_itemdata;
    if item != new_item { item = new_item; (*data).offset = 0i32 }
    if key &
           !(35184372088832u64 | 70368744177664u64 | 140737488355328u64 |
                 281474976710656u64) >=
           KEYC_MOUSE as libc::c_int as libc::c_ulonglong &&
           key &
               !(35184372088832u64 | 70368744177664u64 | 140737488355328u64 |
                     281474976710656u64) <
               KEYC_BSPACE as libc::c_int as libc::c_ulonglong {
        key = window_tree_mouse(data, key, x, item)
    }
    match key {
        60 => { (*data).offset -= 1 }
        62 => { (*data).offset += 1 }
        120 => {
            window_tree_pull_item(item, &mut ns as *mut *mut session,
                                  &mut nwl as *mut *mut winlink,
                                  &mut nwp as *mut *mut window_pane);
            match (*item).type_0 as libc::c_uint {
                1 => {
                    if !(ns == 0 as *mut libc::c_void as *mut session) {
                        xasprintf(&mut prompt as *mut *mut libc::c_char,
                                  b"Kill session %s? \x00" as *const u8 as
                                      *const libc::c_char, (*ns).name);
                    }
                }
                2 => {
                    if !(nwl == 0 as *mut libc::c_void as *mut winlink) {
                        xasprintf(&mut prompt as *mut *mut libc::c_char,
                                  b"Kill window %u? \x00" as *const u8 as
                                      *const libc::c_char, (*nwl).idx);
                    }
                }
                3 => {
                    if !(nwp == 0 as *mut libc::c_void as *mut window_pane ||
                             window_pane_index(nwp, &mut idx as *mut u_int) !=
                                 0i32) {
                        xasprintf(&mut prompt as *mut *mut libc::c_char,
                                  b"Kill pane %u? \x00" as *const u8 as
                                      *const libc::c_char, idx);
                    }
                }
                0 | _ => { }
            }
            if !(prompt == 0 as *mut libc::c_void as *mut libc::c_char) {
                (*data).references += 1;
                status_prompt_set(c, prompt,
                                  b"\x00" as *const u8 as *const libc::c_char,
                                  Some(window_tree_kill_current_callback),
                                  Some(window_tree_command_free),
                                  data as *mut libc::c_void, 1i32 | 8i32);
                free(prompt as *mut libc::c_void);
            }
        }
        88 => {
            tagged = mode_tree_count_tagged((*data).data);
            if !(tagged == 0i32 as libc::c_uint) {
                xasprintf(&mut prompt as *mut *mut libc::c_char,
                          b"Kill %u tagged? \x00" as *const u8 as
                              *const libc::c_char, tagged);
                (*data).references += 1;
                status_prompt_set(c, prompt,
                                  b"\x00" as *const u8 as *const libc::c_char,
                                  Some(window_tree_kill_tagged_callback),
                                  Some(window_tree_command_free),
                                  data as *mut libc::c_void, 1i32 | 8i32);
                free(prompt as *mut libc::c_void);
            }
        }
        58 => {
            tagged = mode_tree_count_tagged((*data).data);
            if tagged != 0i32 as libc::c_uint {
                xasprintf(&mut prompt as *mut *mut libc::c_char,
                          b"(%u tagged) \x00" as *const u8 as
                              *const libc::c_char, tagged);
            } else {
                xasprintf(&mut prompt as *mut *mut libc::c_char,
                          b"(current) \x00" as *const u8 as
                              *const libc::c_char);
            }
            (*data).references += 1;
            status_prompt_set(c, prompt,
                              b"\x00" as *const u8 as *const libc::c_char,
                              Some(window_tree_command_callback),
                              Some(window_tree_command_free),
                              data as *mut libc::c_void, 8i32);
            free(prompt as *mut libc::c_void);
        }
        13 => {
            item =
                mode_tree_get_current((*data).data) as
                    *mut window_tree_itemdata;
            name =
                window_tree_get_target(item, &mut fs as *mut cmd_find_state);
            if name != 0 as *mut libc::c_void as *mut libc::c_char {
                mode_tree_run_command(c, 0 as *mut cmd_find_state,
                                      (*data).command, name);
            }
            finished = 1i32;
            free(name as *mut libc::c_void);
        }
        _ => { }
    }
    if 0 != finished {
        window_pane_reset_mode(wp);
    } else { mode_tree_draw((*data).data); (*wp).flags |= 1i32 };
}
unsafe extern "C" fn window_tree_get_target(mut item:
                                                *mut window_tree_itemdata,
                                            mut fs: *mut cmd_find_state)
 -> *mut libc::c_char {
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    window_tree_pull_item(item, &mut s as *mut *mut session,
                          &mut wl as *mut *mut winlink,
                          &mut wp as *mut *mut window_pane);
    target = 0 as *mut libc::c_char;
    match (*item).type_0 as libc::c_uint {
        1 => {
            if !(s == 0 as *mut libc::c_void as *mut session) {
                xasprintf(&mut target as *mut *mut libc::c_char,
                          b"=%s:\x00" as *const u8 as *const libc::c_char,
                          (*s).name);
            }
        }
        2 => {
            if !(s == 0 as *mut libc::c_void as *mut session ||
                     wl == 0 as *mut libc::c_void as *mut winlink) {
                xasprintf(&mut target as *mut *mut libc::c_char,
                          b"=%s:%u.\x00" as *const u8 as *const libc::c_char,
                          (*s).name, (*wl).idx);
            }
        }
        3 => {
            if !(s == 0 as *mut libc::c_void as *mut session ||
                     wl == 0 as *mut libc::c_void as *mut winlink ||
                     wp == 0 as *mut libc::c_void as *mut window_pane) {
                xasprintf(&mut target as *mut *mut libc::c_char,
                          b"=%s:%u.%%%u\x00" as *const u8 as
                              *const libc::c_char, (*s).name, (*wl).idx,
                          (*wp).id);
            }
        }
        0 | _ => { }
    }
    if target == 0 as *mut libc::c_void as *mut libc::c_char {
        cmd_find_clear_state(fs, 0i32);
    } else { cmd_find_from_winlink_pane(fs, wl, wp, 0i32); }
    return target;
}
unsafe extern "C" fn window_tree_command_free(mut modedata: *mut libc::c_void)
 -> () {
    let mut data: *mut window_tree_modedata =
        modedata as *mut window_tree_modedata;
    window_tree_destroy(data);
}
unsafe extern "C" fn window_tree_command_callback(mut c: *mut client,
                                                  mut modedata:
                                                      *mut libc::c_void,
                                                  mut s: *const libc::c_char,
                                                  mut done: libc::c_int)
 -> libc::c_int {
    let mut data: *mut window_tree_modedata =
        modedata as *mut window_tree_modedata;
    if s == 0 as *mut libc::c_void as *const libc::c_char ||
           *s as libc::c_int == 0 || 0 != (*data).dead {
        return 0i32
    } else {
        (*data).entered = s;
        mode_tree_each_tagged((*data).data, Some(window_tree_command_each), c,
                              281470681743360u64, 1i32);
        (*data).entered = 0 as *const libc::c_char;
        (*data).references += 1;
        cmdq_append(c,
                    cmdq_get_callback1(b"window_tree_command_done\x00" as
                                           *const u8 as *const libc::c_char,
                                       Some(window_tree_command_done),
                                       data as *mut libc::c_void));
        return 0i32
    };
}
unsafe extern "C" fn window_tree_command_done(mut item: *mut cmdq_item,
                                              mut modedata: *mut libc::c_void)
 -> cmd_retval {
    let mut data: *mut window_tree_modedata =
        modedata as *mut window_tree_modedata;
    if 0 == (*data).dead {
        mode_tree_build((*data).data);
        mode_tree_draw((*data).data);
        (*(*data).wp).flags |= 1i32
    }
    window_tree_destroy(data);
    return CMD_RETURN_NORMAL;
}
unsafe extern "C" fn window_tree_command_each(mut modedata: *mut libc::c_void,
                                              mut itemdata: *mut libc::c_void,
                                              mut c: *mut client,
                                              mut key: key_code) -> () {
    let mut data: *mut window_tree_modedata =
        modedata as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata =
        itemdata as *mut window_tree_itemdata;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fs: cmd_find_state =
        cmd_find_state{flags: 0,
                       current:
                           0 as *const cmd_find_state as *mut cmd_find_state,
                       s: 0 as *const session as *mut session,
                       wl: 0 as *const winlink as *mut winlink,
                       w: 0 as *const window as *mut window,
                       wp: 0 as *const window_pane as *mut window_pane,
                       idx: 0,};
    name = window_tree_get_target(item, &mut fs as *mut cmd_find_state);
    if name != 0 as *mut libc::c_void as *mut libc::c_char {
        mode_tree_run_command(c, &mut fs as *mut cmd_find_state,
                              (*data).entered, name);
    }
    free(name as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_kill_tagged_callback(mut c: *mut client,
                                                      mut modedata:
                                                          *mut libc::c_void,
                                                      mut s:
                                                          *const libc::c_char,
                                                      mut done: libc::c_int)
 -> libc::c_int {
    let mut data: *mut window_tree_modedata =
        modedata as *mut window_tree_modedata;
    let mut mtd: *mut mode_tree_data = (*data).data;
    if s == 0 as *mut libc::c_void as *const libc::c_char ||
           *s as libc::c_int == 0 || 0 != (*data).dead {
        return 0i32
    } else if tolower(*s.offset(0isize) as u_char as libc::c_int) != 121 ||
                  *s.offset(1isize) as libc::c_int != 0 {
        return 0i32
    } else {
        mode_tree_each_tagged(mtd, Some(window_tree_kill_each), c,
                              281470681743360u64, 1i32);
        (*data).references += 1;
        cmdq_append(c,
                    cmdq_get_callback1(b"window_tree_command_done\x00" as
                                           *const u8 as *const libc::c_char,
                                       Some(window_tree_command_done),
                                       data as *mut libc::c_void));
        return 0i32
    };
}
unsafe extern "C" fn window_tree_kill_each(mut modedata: *mut libc::c_void,
                                           mut itemdata: *mut libc::c_void,
                                           mut c: *mut client,
                                           mut key: key_code) -> () {
    let mut item: *mut window_tree_itemdata =
        itemdata as *mut window_tree_itemdata;
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    window_tree_pull_item(item, &mut s as *mut *mut session,
                          &mut wl as *mut *mut winlink,
                          &mut wp as *mut *mut window_pane);
    match (*item).type_0 as libc::c_uint {
        1 => {
            if s != 0 as *mut libc::c_void as *mut session {
                server_destroy_session(s);
                session_destroy(s,
                                (*::std::mem::transmute::<&[u8; 22],
                                                          &[libc::c_char; 22]>(b"window_tree_kill_each\x00")).as_ptr());
            }
        }
        2 => {
            if wl != 0 as *mut libc::c_void as *mut winlink {
                server_kill_window((*wl).window);
            }
        }
        3 => {
            if wp != 0 as *mut libc::c_void as *mut window_pane {
                server_kill_pane(wp);
            }
        }
        0 | _ => { }
    };
}
unsafe extern "C" fn window_tree_kill_current_callback(mut c: *mut client,
                                                       mut modedata:
                                                           *mut libc::c_void,
                                                       mut s:
                                                           *const libc::c_char,
                                                       mut done: libc::c_int)
 -> libc::c_int {
    let mut data: *mut window_tree_modedata =
        modedata as *mut window_tree_modedata;
    let mut mtd: *mut mode_tree_data = (*data).data;
    if s == 0 as *mut libc::c_void as *const libc::c_char ||
           *s as libc::c_int == 0 || 0 != (*data).dead {
        return 0i32
    } else if tolower(*s.offset(0isize) as u_char as libc::c_int) != 121 ||
                  *s.offset(1isize) as libc::c_int != 0 {
        return 0i32
    } else {
        window_tree_kill_each(data as *mut libc::c_void,
                              mode_tree_get_current(mtd), c,
                              281470681743360u64);
        (*data).references += 1;
        cmdq_append(c,
                    cmdq_get_callback1(b"window_tree_command_done\x00" as
                                           *const u8 as *const libc::c_char,
                                       Some(window_tree_command_done),
                                       data as *mut libc::c_void));
        return 0i32
    };
}
unsafe extern "C" fn window_tree_mouse(mut data: *mut window_tree_modedata,
                                       mut key: key_code, mut x: u_int,
                                       mut item: *mut window_tree_itemdata)
 -> key_code {
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut loop_0: u_int = 0;
    if key != KEYC_MOUSEDOWN1_PANE as libc::c_int as libc::c_ulonglong {
        return 281470681743360u64
    } else if (*data).left != 1i32.wrapping_neg() &&
                  x <= (*data).left as u_int {
        return 60 as key_code
    } else if (*data).right != 1i32.wrapping_neg() &&
                  x >= (*data).right as u_int {
        return 62 as key_code
    } else {
        if (*data).left != 1i32.wrapping_neg() {
            x =
                (x as libc::c_uint).wrapping_sub((*data).left as libc::c_uint)
                    as u_int as u_int
        } else if x != 0i32 as libc::c_uint { x = x.wrapping_sub(1) }
        if x == 0i32 as libc::c_uint || (*data).end == 0i32 as libc::c_uint {
            x = 0i32 as u_int
        } else {
            x = x.wrapping_div((*data).each);
            if (*data).start.wrapping_add(x) >= (*data).end {
                x = (*data).end.wrapping_sub(1i32 as libc::c_uint)
            }
        }
        window_tree_pull_item(item, &mut s as *mut *mut session,
                              &mut wl as *mut *mut winlink,
                              &mut wp as *mut *mut window_pane);
        if (*item).type_0 as libc::c_uint ==
               WINDOW_TREE_SESSION as libc::c_int as libc::c_uint {
            if s == 0 as *mut libc::c_void as *mut session {
                return 281470681743360u64
            } else {
                mode_tree_expand_current((*data).data);
                loop_0 = 0i32 as u_int;
                wl =
                    winlinks_RB_MINMAX(&mut (*s).windows as *mut winlinks,
                                       1i32.wrapping_neg());
                while wl != 0 as *mut libc::c_void as *mut winlink {
                    if loop_0 == (*data).start.wrapping_add(x) { break ; }
                    loop_0 = loop_0.wrapping_add(1);
                    wl = winlinks_RB_NEXT(wl)
                }
                if wl != 0 as *mut libc::c_void as *mut winlink {
                    mode_tree_set_current((*data).data, wl as uint64_t);
                }
                return 13 as key_code
            }
        } else if (*item).type_0 as libc::c_uint ==
                      WINDOW_TREE_WINDOW as libc::c_int as libc::c_uint {
            if wl == 0 as *mut libc::c_void as *mut winlink {
                return 281470681743360u64
            } else {
                mode_tree_expand_current((*data).data);
                loop_0 = 0i32 as u_int;
                wp =
                    (*(&mut (*(*wl).window).panes as
                           *mut window_panes)).tqh_first;
                while wp != 0 as *mut libc::c_void as *mut window_pane {
                    if loop_0 == (*data).start.wrapping_add(x) { break ; }
                    loop_0 = loop_0.wrapping_add(1);
                    wp = (*wp).entry.tqe_next
                }
                if wp != 0 as *mut libc::c_void as *mut window_pane {
                    mode_tree_set_current((*data).data, wp as uint64_t);
                }
                return 13 as key_code
            }
        } else { return 281470681743360u64 }
    };
}

