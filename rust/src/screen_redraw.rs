extern crate libc;
extern "C" {
    pub type screen_write_collect_line;
    pub type tty_code;
    pub type event_base;
    pub type environ;
    pub type tmuxproc;
    pub type options;
    pub type screen_write_collect_item;
    pub type screen_titles;
    pub type format_job_tree;
    pub type hooks;
    pub type _IO_FILE_plus;
    pub type format_tree;
    pub type input_ctx;
    pub type tmuxpeer;
    pub type bufferevent_ops;
    pub type args_entry;
    pub type evbuffer;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
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
    fn format_create(_: *mut client, _: *mut cmdq_item, _: libc::c_int,
                     _: libc::c_int) -> *mut format_tree;
    #[no_mangle]
    fn format_free(_: *mut format_tree) -> ();
    #[no_mangle]
    fn format_expand(_: *mut format_tree, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn format_defaults(_: *mut format_tree, _: *mut client, _: *mut session,
                       _: *mut winlink, _: *mut window_pane) -> ();
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
    fn tty_attributes(_: *mut tty, _: *const grid_cell, _: *const window_pane)
     -> ();
    #[no_mangle]
    fn tty_reset(_: *mut tty) -> ();
    #[no_mangle]
    fn tty_cursor(_: *mut tty, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn tty_puts(_: *mut tty, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn tty_putc(_: *mut tty, _: u_char) -> ();
    #[no_mangle]
    fn tty_draw_pane(_: *mut tty, _: *const window_pane, _: u_int, _: u_int,
                     _: u_int) -> ();
    #[no_mangle]
    fn tty_draw_line(_: *mut tty, _: *const window_pane, _: *mut screen,
                     _: u_int, _: u_int, _: u_int) -> ();
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
    fn server_is_marked(_: *mut session, _: *mut winlink, _: *mut window_pane)
     -> libc::c_int;
    #[no_mangle]
    fn status_at_line(_: *mut client) -> libc::c_int;
    #[no_mangle]
    fn status_line_size(_: *mut session) -> u_int;
    #[no_mangle]
    fn status_redraw(_: *mut client) -> libc::c_int;
    #[no_mangle]
    fn status_message_redraw(_: *mut client) -> libc::c_int;
    #[no_mangle]
    fn status_prompt_redraw(_: *mut client) -> libc::c_int;
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    fn grid_compare(_: *mut grid, _: *mut grid) -> libc::c_int;
    #[no_mangle]
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut window_pane,
                          _: *mut screen) -> ();
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx) -> ();
    #[no_mangle]
    fn screen_write_cstrlen(_: *const libc::c_char, ...) -> size_t;
    #[no_mangle]
    fn screen_write_cnputs(_: *mut screen_write_ctx, _: ssize_t,
                           _: *const grid_cell, _: *const libc::c_char, ...)
     -> ();
    #[no_mangle]
    fn screen_write_clearline(_: *mut screen_write_ctx, _: u_int) -> ();
    #[no_mangle]
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn screen_free(_: *mut screen) -> ();
    #[no_mangle]
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int)
     -> ();
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn style_apply(_: *mut grid_cell, _: *mut options, _: *const libc::c_char)
     -> ();
    #[no_mangle]
    static window_clock_table: [[[libc::c_char; 5]; 5]; 14];
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn window_pane_index(_: *mut window_pane, _: *mut u_int) -> libc::c_int;
    #[no_mangle]
    fn window_pane_visible(_: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn window_count_panes(_: *mut window) -> u_int;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
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
    static window_client_mode: window_mode;
    #[no_mangle]
    static window_copy_mode: window_mode;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    static mut session_groups: session_groups;
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub type __u_short = libc::c_ushort;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_14,
}
pub type layout_type = libc::c_uint;
pub type key_code = libc::c_ulonglong;
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
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_22,
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
pub struct unnamed {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub ev_io_next: unnamed_36,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_3 {
    offset: u_int,
    data: unnamed_37,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub type cmd_retval = libc::c_int;
pub type unnamed_5 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
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
pub struct unnamed_6 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type pid_t = __pid_t;
pub const JOB_DEAD: unnamed_5 = 1;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
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
    pub entry: unnamed_15,
    pub tree_entry: unnamed_2,
}
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
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub const TTY_VT100: unnamed_10 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type u_char = __u_char;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub type unnamed_10 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_29,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_18,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_16,
    pub ev_next: unnamed_4,
    pub ev_timeout_pos: unnamed_28,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_11,
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
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_11 {
    ev_io: unnamed_0,
    ev_signal: unnamed_33,
}
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_32,
}
pub type __u_int = libc::c_uint;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub type options_table_type = libc::c_uint;
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
pub const TTY_VT320: unnamed_10 = 4;
pub type ssize_t = __ssize_t;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
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
pub struct unnamed_12 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type unnamed_13 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_5,
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
    pub entry: unnamed_24,
}
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type u_int = __u_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const PROMPT_ENTRY: unnamed_13 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
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
    pub entry: unnamed_35,
    pub wentry: unnamed,
    pub sentry: unnamed_12,
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
    pub winlinks: unnamed_25,
    pub entry: unnamed_38,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_26,
}
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
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
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type __off64_t = libc::c_long;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTY_UNKNOWN: unnamed_10 = 6;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub type tcflag_t = libc::c_uint;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub type bitstr_t = libc::c_uchar;
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
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_1,
    pub entry: unnamed_8,
}
pub type cmdq_type = libc::c_uint;
pub const PROMPT_COMMAND: unnamed_13 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub type __ssize_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_29 = 1;
pub const LINE_SEL_RIGHT_LEFT: unnamed_29 = 2;
pub const TTY_VT101: unnamed_10 = 1;
pub type uint16_t = libc::c_ushort;
pub type cc_t = libc::c_uchar;
pub type __suseconds_t = libc::c_long;
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
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const TTY_VT420: unnamed_10 = 5;
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
pub struct unnamed_20 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type size_t = libc::c_ulong;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_23,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub const TTY_VT220: unnamed_10 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const LINE_SEL_NONE: unnamed_29 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_6,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const TTY_VT102: unnamed_10 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_28 {
    ev_next_with_common_timeout: unnamed_20,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
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
pub type uint8_t = libc::c_uchar;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
pub type unnamed_29 = libc::c_uint;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
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
    pub term_type: unnamed_10,
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
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_3,
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
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_32 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub type __u_char = libc::c_uchar;
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
pub struct unnamed_33 {
    pub ev_signal_next: unnamed_31,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
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
    pub prompt_mode: unnamed_13,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_30,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const JOB_RUNNING: unnamed_5 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub type uint32_t = libc::c_uint;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const JOB_CLOSED: unnamed_5 = 2;
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
    pub entry: unnamed_27,
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
    pub gentry: unnamed_9,
    pub entry: unnamed_17,
}
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
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
pub struct unnamed_38 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type options_table_scope = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn screen_redraw_update(mut c: *mut client) -> () {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut wo: *mut options = (*w).options;
    let mut redraw: libc::c_int = 0;
    if (*c).message_string != 0 as *mut libc::c_void as *mut libc::c_char {
        redraw = status_message_redraw(c)
    } else if (*c).prompt_string !=
                  0 as *mut libc::c_void as *mut libc::c_char {
        redraw = status_prompt_redraw(c)
    } else { redraw = status_redraw(c) }
    if 0 == redraw { (*c).flags &= !16i32 }
    if options_get_number(wo,
                          b"pane-border-status\x00" as *const u8 as
                              *const libc::c_char) != 0i32 as libc::c_longlong
       {
        redraw = 0i32;
        wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
        while wp != 0 as *mut libc::c_void as *mut window_pane {
            if 0 != screen_redraw_make_pane_status(c, w, wp) { redraw = 1i32 }
            wp = (*wp).entry.tqe_next
        }
        if 0 != redraw { (*c).flags |= 1024i32 }
    };
}
unsafe extern "C" fn screen_redraw_make_pane_status(mut c: *mut client,
                                                    mut w: *mut window,
                                                    mut wp: *mut window_pane)
 -> libc::c_int {
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outlen: size_t = 0;
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
    let mut old: screen =
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
    if wp == (*w).active {
        style_apply(&mut gc as *mut grid_cell, (*w).options,
                    b"pane-active-border-style\x00" as *const u8 as
                        *const libc::c_char);
    } else {
        style_apply(&mut gc as *mut grid_cell, (*w).options,
                    b"pane-border-style\x00" as *const u8 as
                        *const libc::c_char);
    }
    fmt =
        options_get_string((*w).options,
                           b"pane-border-format\x00" as *const u8 as
                               *const libc::c_char);
    ft =
        format_create(c, 0 as *mut cmdq_item,
                      (2147483648u32 | (*wp).id) as libc::c_int, 0i32);
    format_defaults(ft, c, 0 as *mut session, 0 as *mut winlink, wp);
    memcpy(&mut old as *mut screen as *mut libc::c_void,
           &mut (*wp).status_screen as *mut screen as *const libc::c_void,
           ::std::mem::size_of::<screen>() as libc::c_ulong);
    screen_init(&mut (*wp).status_screen as *mut screen, (*wp).sx,
                1i32 as u_int, 0i32 as u_int);
    (*wp).status_screen.mode = 0i32;
    out = format_expand(ft, fmt);
    outlen =
        screen_write_cstrlen(b"%s\x00" as *const u8 as *const libc::c_char,
                             out);
    if outlen > (*wp).sx.wrapping_sub(4i32 as libc::c_uint) as libc::c_ulong {
        outlen = (*wp).sx.wrapping_sub(4i32 as libc::c_uint) as size_t
    }
    screen_resize(&mut (*wp).status_screen as *mut screen, outlen as u_int,
                  1i32 as u_int, 0i32);
    screen_write_start(&mut ctx as *mut screen_write_ctx,
                       0 as *mut window_pane,
                       &mut (*wp).status_screen as *mut screen);
    screen_write_cursormove(&mut ctx as *mut screen_write_ctx, 0i32 as u_int,
                            0i32 as u_int);
    screen_write_clearline(&mut ctx as *mut screen_write_ctx, 8i32 as u_int);
    screen_write_cnputs(&mut ctx as *mut screen_write_ctx, outlen as ssize_t,
                        &mut gc as *mut grid_cell,
                        b"%s\x00" as *const u8 as *const libc::c_char, out);
    screen_write_stop(&mut ctx as *mut screen_write_ctx);
    free(out as *mut libc::c_void);
    format_free(ft);
    (*wp).status_size = outlen;
    if grid_compare((*wp).status_screen.grid, old.grid) == 0i32 {
        screen_free(&mut old as *mut screen);
        return 0i32
    } else { screen_free(&mut old as *mut screen); return 1i32 };
}
#[no_mangle]
pub unsafe extern "C" fn screen_redraw_screen(mut c: *mut client,
                                              mut draw_panes: libc::c_int,
                                              mut draw_status: libc::c_int,
                                              mut draw_borders: libc::c_int)
 -> () {
    let mut oo: *mut options = (*(*c).session).options;
    let mut tty: *mut tty = &mut (*c).tty as *mut tty;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wo: *mut options = (*w).options;
    let mut top: u_int = 0;
    let mut lines: u_int = 0;
    let mut position: libc::c_int = 0;
    let mut pane_status: libc::c_int = 0;
    if 0 != (*c).flags & 64i32 {
        return
    } else {
        if 0 != (*c).flags & 8388608i32 {
            lines = 0i32 as u_int
        } else { lines = status_line_size((*c).session) }
        if (*c).message_string != 0 as *mut libc::c_void as *mut libc::c_char
               ||
               (*c).prompt_string !=
                   0 as *mut libc::c_void as *mut libc::c_char {
            lines =
                if lines == 0i32 as libc::c_uint {
                    1i32 as libc::c_uint
                } else { lines }
        }
        position =
            options_get_number(oo,
                               b"status-position\x00" as *const u8 as
                                   *const libc::c_char) as libc::c_int;
        if lines != 0i32 as libc::c_uint && position == 0i32 {
            top = 1i32 as u_int
        } else { top = 0i32 as u_int }
        if lines == 0i32 as libc::c_uint { draw_status = 0i32 }
        if 0 != draw_borders {
            pane_status =
                options_get_number(wo,
                                   b"pane-border-status\x00" as *const u8 as
                                       *const libc::c_char) as libc::c_int;
            screen_redraw_draw_borders(c, pane_status, lines, top);
            if pane_status != 0i32 {
                screen_redraw_draw_pane_status(c, pane_status);
            }
        }
        if 0 != draw_panes { screen_redraw_draw_panes(c, lines, top); }
        if 0 != draw_status { screen_redraw_draw_status(c, lines, top); }
        tty_reset(tty);
        return;
    };
}
unsafe extern "C" fn screen_redraw_draw_status(mut c: *mut client,
                                               mut lines: u_int,
                                               mut top: u_int) -> () {
    let mut tty: *mut tty = &mut (*c).tty as *mut tty;
    let mut i: u_int = 0;
    let mut y: u_int = 0;
    if 0 != top {
        y = 0i32 as u_int
    } else { y = (*tty).sy.wrapping_sub(lines) }
    i = 0i32 as u_int;
    while i < lines {
        tty_draw_line(tty, 0 as *const window_pane,
                      &mut (*c).status.status as *mut screen, i,
                      0i32 as u_int, y);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn screen_redraw_draw_panes(mut c: *mut client,
                                              mut lines: u_int,
                                              mut top: u_int) -> () {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut tty: *mut tty = &mut (*c).tty as *mut tty;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut i: u_int = 0;
    let mut y: u_int = 0;
    if 0 != top { y = lines } else { y = 0i32 as u_int }
    wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
    while wp != 0 as *mut libc::c_void as *mut window_pane {
        if !(0 == window_pane_visible(wp)) {
            i = 0i32 as u_int;
            while i < (*wp).sy {
                tty_draw_pane(tty, wp, i, (*wp).xoff,
                              y.wrapping_add((*wp).yoff));
                i = i.wrapping_add(1)
            }
            if 0 != (*c).flags & 256i32 {
                screen_redraw_draw_number(c, wp, lines, top);
            }
        }
        wp = (*wp).entry.tqe_next
    };
}
unsafe extern "C" fn screen_redraw_draw_number(mut c: *mut client,
                                               mut wp: *mut window_pane,
                                               mut lines: u_int,
                                               mut top: u_int) -> () {
    let mut tty: *mut tty = &mut (*c).tty as *mut tty;
    let mut s: *mut session = (*c).session;
    let mut oo: *mut options = (*s).options;
    let mut w: *mut window = (*wp).window;
    let mut gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut idx: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut xoff: u_int = 0;
    let mut yoff: u_int = 0;
    let mut colour: libc::c_int = 0;
    let mut active_colour: libc::c_int = 0;
    let mut buf: [libc::c_char; 16] = [0; 16];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if window_pane_index(wp, &mut idx as *mut u_int) != 0i32 {
        fatalx(b"index not found\x00" as *const u8 as *const libc::c_char);
    } else {
        len =
            xsnprintf(buf.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 16]>() as
                          libc::c_ulong,
                      b"%u\x00" as *const u8 as *const libc::c_char, idx) as
                size_t;
        if ((*wp).sx as libc::c_ulong) < len {
            return
        } else {
            colour =
                options_get_number(oo,
                                   b"display-panes-colour\x00" as *const u8 as
                                       *const libc::c_char) as libc::c_int;
            active_colour =
                options_get_number(oo,
                                   b"display-panes-active-colour\x00" as
                                       *const u8 as *const libc::c_char) as
                    libc::c_int;
            px = (*wp).sx.wrapping_div(2i32 as libc::c_uint);
            py = (*wp).sy.wrapping_div(2i32 as libc::c_uint);
            xoff = (*wp).xoff;
            yoff = (*wp).yoff;
            if 0 != top {
                yoff =
                    (yoff as libc::c_uint).wrapping_add(lines) as u_int as
                        u_int
            }
            if ((*wp).sx as libc::c_ulong) <
                   len.wrapping_mul(6i32 as libc::c_ulong) ||
                   (*wp).sy < 5i32 as libc::c_uint {
                tty_cursor(tty,
                           (xoff.wrapping_add(px) as
                                libc::c_ulong).wrapping_sub(len.wrapping_div(2i32
                                                                                 as
                                                                                 libc::c_ulong))
                               as u_int, yoff.wrapping_add(py));
            } else {
                px =
                    (px as
                         libc::c_ulong).wrapping_sub(len.wrapping_mul(3i32 as
                                                                          libc::c_ulong))
                        as u_int as u_int;
                py =
                    (py as libc::c_uint).wrapping_sub(2i32 as libc::c_uint) as
                        u_int as u_int;
                memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
                       &grid_default_cell as *const grid_cell as
                           *const libc::c_void,
                       ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
                if (*w).active == wp {
                    gc.bg = active_colour
                } else { gc.bg = colour }
                gc.flags = (gc.flags as libc::c_int | 32i32) as u_char;
                tty_attributes(tty, &mut gc as *mut grid_cell, wp);
                ptr = buf.as_mut_ptr();
                while *ptr as libc::c_int != 0 {
                    if !((*ptr as libc::c_int) < 48 ||
                             *ptr as libc::c_int > 57) {
                        idx = (*ptr as libc::c_int - 48) as u_int;
                        j = 0i32 as u_int;
                        while j < 5i32 as libc::c_uint {
                            i = px;
                            while i < px.wrapping_add(5i32 as libc::c_uint) {
                                tty_cursor(tty, xoff.wrapping_add(i),
                                           yoff.wrapping_add(py).wrapping_add(j));
                                if 0 !=
                                       window_clock_table[idx as
                                                              usize][j as
                                                                         usize][i.wrapping_sub(px)
                                                                                    as
                                                                                    usize]
                                   {
                                    tty_putc(tty, 32 as u_char);
                                }
                                i = i.wrapping_add(1)
                            }
                            j = j.wrapping_add(1)
                        }
                        px =
                            (px as
                                 libc::c_uint).wrapping_add(6i32 as
                                                                libc::c_uint)
                                as u_int as u_int
                    }
                    ptr = ptr.offset(1isize)
                }
                len =
                    xsnprintf(buf.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 16]>() as
                                  libc::c_ulong,
                              b"%ux%u\x00" as *const u8 as
                                  *const libc::c_char, (*wp).sx, (*wp).sy) as
                        size_t;
                if ((*wp).sx as libc::c_ulong) < len ||
                       (*wp).sy < 6i32 as libc::c_uint {
                    return
                } else {
                    tty_cursor(tty,
                               (xoff.wrapping_add((*wp).sx) as
                                    libc::c_ulong).wrapping_sub(len) as u_int,
                               yoff);
                }
            }
            memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
                   &grid_default_cell as *const grid_cell as
                       *const libc::c_void,
                   ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
            if (*w).active == wp {
                gc.fg = active_colour
            } else { gc.fg = colour }
            gc.flags = (gc.flags as libc::c_int | 32i32) as u_char;
            tty_attributes(tty, &mut gc as *mut grid_cell, wp);
            tty_puts(tty, buf.as_mut_ptr());
            tty_cursor(tty, 0i32 as u_int, 0i32 as u_int);
            return;
        }
    };
}
unsafe extern "C" fn screen_redraw_draw_pane_status(mut c: *mut client,
                                                    mut pane_status:
                                                        libc::c_int) -> () {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut oo: *mut options = (*(*c).session).options;
    let mut tty: *mut tty = &mut (*c).tty as *mut tty;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut spos: libc::c_int = 0;
    let mut yoff: u_int = 0;
    spos =
        options_get_number(oo,
                           b"status-position\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
    while wp != 0 as *mut libc::c_void as *mut window_pane {
        if !(0 == window_pane_visible(wp)) {
            if pane_status == 1i32 {
                yoff = (*wp).yoff.wrapping_sub(1i32 as libc::c_uint)
            } else { yoff = (*wp).yoff.wrapping_add((*wp).sy) }
            if spos == 0i32 {
                yoff =
                    (yoff as libc::c_uint).wrapping_add(1i32 as libc::c_uint)
                        as u_int as u_int
            }
            tty_draw_line(tty, 0 as *const window_pane,
                          &mut (*wp).status_screen as *mut screen,
                          0i32 as u_int,
                          (*wp).xoff.wrapping_add(2i32 as libc::c_uint),
                          yoff);
        }
        wp = (*wp).entry.tqe_next
    }
    tty_cursor(tty, 0i32 as u_int, 0i32 as u_int);
}
unsafe extern "C" fn screen_redraw_draw_borders(mut c: *mut client,
                                                mut pane_status: libc::c_int,
                                                mut lines: u_int,
                                                mut top: u_int) -> () {
    let mut s: *mut session = (*c).session;
    let mut w: *mut window = (*(*s).curw).window;
    let mut oo: *mut options = (*w).options;
    let mut tty: *mut tty = &mut (*c).tty as *mut tty;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut m_active_gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut active_gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut m_other_gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut other_gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut msg_gc: grid_cell =
        grid_cell{flags: 0,
                  attr: 0,
                  fg: 0,
                  bg: 0,
                  data:
                      utf8_data{data: [0; 9], have: 0, size: 0, width: 0,},};
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut type_0: u_int = 0;
    let mut msgx: u_int = 0i32 as u_int;
    let mut msgy: u_int = 0i32 as u_int;
    let mut active: libc::c_int = 0;
    let mut small: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut msg: [libc::c_char; 256] = [0; 256];
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut msglen: size_t = 0i32 as size_t;
    small =
        ((*tty).sy.wrapping_sub(lines).wrapping_add(top) > (*w).sy ||
             (*tty).sx > (*w).sx) as libc::c_int;
    if 0 != small {
        flags = (*w).flags & (8192i32 | 16384i32);
        if flags == 8192i32 | 16384i32 {
            tmp =
                b"force-width, force-height\x00" as *const u8 as
                    *const libc::c_char
        } else if flags == 8192i32 {
            tmp = b"force-width\x00" as *const u8 as *const libc::c_char
        } else if flags == 16384i32 {
            tmp = b"force-height\x00" as *const u8 as *const libc::c_char
        } else if 0 != (*c).flags & 8388608i32 {
            tmp = b"status line\x00" as *const u8 as *const libc::c_char
        } else {
            tmp = b"a smaller client\x00" as *const u8 as *const libc::c_char
        }
        xsnprintf(msg.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong,
                  b"(size %ux%u from %s)\x00" as *const u8 as
                      *const libc::c_char, (*w).sx, (*w).sy, tmp);
        msglen = strlen(msg.as_mut_ptr());
        if (*tty).sy.wrapping_sub(1i32 as
                                      libc::c_uint).wrapping_sub(lines).wrapping_add(top)
               > (*w).sy && (*tty).sx as libc::c_ulong >= msglen {
            msgx = ((*tty).sx as libc::c_ulong).wrapping_sub(msglen) as u_int;
            msgy =
                (*tty).sy.wrapping_sub(1i32 as
                                           libc::c_uint).wrapping_sub(lines).wrapping_add(top)
        } else if (*tty).sx.wrapping_sub((*w).sx) as libc::c_ulong > msglen {
            msgx = ((*tty).sx as libc::c_ulong).wrapping_sub(msglen) as u_int;
            msgy =
                (*tty).sy.wrapping_sub(1i32 as
                                           libc::c_uint).wrapping_sub(lines).wrapping_add(top)
        } else { small = 0i32 }
    }
    style_apply(&mut other_gc as *mut grid_cell, oo,
                b"pane-border-style\x00" as *const u8 as *const libc::c_char);
    style_apply(&mut active_gc as *mut grid_cell, oo,
                b"pane-active-border-style\x00" as *const u8 as
                    *const libc::c_char);
    other_gc.attr = 128i32 as u_short;
    active_gc.attr = other_gc.attr;
    memcpy(&mut m_other_gc as *mut grid_cell as *mut libc::c_void,
           &mut other_gc as *mut grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    m_other_gc.attr = (m_other_gc.attr as libc::c_int ^ 16i32) as u_short;
    memcpy(&mut m_active_gc as *mut grid_cell as *mut libc::c_void,
           &mut active_gc as *mut grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    m_active_gc.attr = (m_active_gc.attr as libc::c_int ^ 16i32) as u_short;
    j = 0i32 as u_int;
    while j < (*tty).sy.wrapping_sub(lines) {
        i = 0i32 as u_int;
        while i < (*tty).sx {
            type_0 =
                screen_redraw_check_cell(c, i, j, pane_status,
                                         &mut wp as *mut *mut window_pane) as
                    u_int;
            if !(type_0 == 0i32 as libc::c_uint) {
                if !(type_0 == 12i32 as libc::c_uint && 0 != small && i > msgx
                         && j == msgy) {
                    active =
                        screen_redraw_check_is(i, j, type_0 as libc::c_int,
                                               pane_status, w, (*w).active,
                                               wp);
                    if 0 != server_is_marked(s, (*s).curw, marked_pane.wp) &&
                           0 !=
                               screen_redraw_check_is(i, j,
                                                      type_0 as libc::c_int,
                                                      pane_status, w,
                                                      marked_pane.wp, wp) {
                        if 0 != active {
                            tty_attributes(tty,
                                           &mut m_active_gc as *mut grid_cell,
                                           0 as *const window_pane);
                        } else {
                            tty_attributes(tty,
                                           &mut m_other_gc as *mut grid_cell,
                                           0 as *const window_pane);
                        }
                    } else if 0 != active {
                        tty_attributes(tty, &mut active_gc as *mut grid_cell,
                                       0 as *const window_pane);
                    } else {
                        tty_attributes(tty, &mut other_gc as *mut grid_cell,
                                       0 as *const window_pane);
                    }
                    if 0 != top {
                        tty_cursor(tty, i, lines.wrapping_add(j));
                    } else { tty_cursor(tty, i, j); }
                    tty_putc(tty,
                             (*::std::mem::transmute::<&[u8; 14],
                                                       &[libc::c_char; 14]>(b" xqlkmjwvtun~\x00"))[type_0
                                                                                                       as
                                                                                                       usize]
                                 as u_char);
                }
            }
            i = i.wrapping_add(1)
        }
        j = j.wrapping_add(1)
    }
    if 0 != small {
        memcpy(&mut msg_gc as *mut grid_cell as *mut libc::c_void,
               &grid_default_cell as *const grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        tty_attributes(tty, &mut msg_gc as *mut grid_cell,
                       0 as *const window_pane);
        tty_cursor(tty, msgx, msgy);
        tty_puts(tty, msg.as_mut_ptr());
    };
}
unsafe extern "C" fn screen_redraw_check_is(mut px: u_int, mut py: u_int,
                                            mut type_0: libc::c_int,
                                            mut pane_status: libc::c_int,
                                            mut w: *mut window,
                                            mut wantwp: *mut window_pane,
                                            mut wp: *mut window_pane)
 -> libc::c_int {
    let mut border: libc::c_int = 0;
    border = screen_redraw_cell_border1(wantwp, px, py);
    if border == 0i32 || border == 1i32.wrapping_neg() {
        return 0i32
    } else if pane_status == 1i32 && border == 4i32 {
        return 0i32
    } else if pane_status == 2i32 && border == 3i32 {
        return 0i32
    } else if window_count_panes(w) != 2i32 as libc::c_uint {
        return 1i32
    } else if wp == 0 as *mut libc::c_void as *mut window_pane ||
                  (type_0 == 12i32 || type_0 == 0i32) {
        return 1i32
    } else if pane_status != 0i32 {
        return 1i32
    } else if (*wp).xoff == 0i32 as libc::c_uint && (*wp).sx == (*w).sx {
        if (*wp).yoff == 0i32 as libc::c_uint {
            if wp == wantwp {
                return (px <= (*wp).sx.wrapping_div(2i32 as libc::c_uint)) as
                           libc::c_int
            } else {
                return (px > (*wp).sx.wrapping_div(2i32 as libc::c_uint)) as
                           libc::c_int
            }
        } else { return 0i32 }
    } else if (*wp).yoff == 0i32 as libc::c_uint && (*wp).sy == (*w).sy {
        if (*wp).xoff == 0i32 as libc::c_uint {
            if wp == wantwp {
                return (py <= (*wp).sy.wrapping_div(2i32 as libc::c_uint)) as
                           libc::c_int
            } else {
                return (py > (*wp).sy.wrapping_div(2i32 as libc::c_uint)) as
                           libc::c_int
            }
        } else { return 0i32 }
    } else { return 1i32 };
}
unsafe extern "C" fn screen_redraw_cell_border1(mut wp: *mut window_pane,
                                                mut px: u_int, mut py: u_int)
 -> libc::c_int {
    if px >= (*wp).xoff && px < (*wp).xoff.wrapping_add((*wp).sx) &&
           py >= (*wp).yoff && py < (*wp).yoff.wrapping_add((*wp).sy) {
        return 0i32
    } else {
        if ((*wp).yoff == 0i32 as libc::c_uint ||
                py >= (*wp).yoff.wrapping_sub(1i32 as libc::c_uint)) &&
               py <= (*wp).yoff.wrapping_add((*wp).sy) {
            if (*wp).xoff != 0i32 as libc::c_uint &&
                   px == (*wp).xoff.wrapping_sub(1i32 as libc::c_uint) {
                return 1i32
            } else if px == (*wp).xoff.wrapping_add((*wp).sx) { return 2i32 }
        }
        if ((*wp).xoff == 0i32 as libc::c_uint ||
                px >= (*wp).xoff.wrapping_sub(1i32 as libc::c_uint)) &&
               px <= (*wp).xoff.wrapping_add((*wp).sx) {
            if (*wp).yoff != 0i32 as libc::c_uint &&
                   py == (*wp).yoff.wrapping_sub(1i32 as libc::c_uint) {
                return 3i32
            } else if py == (*wp).yoff.wrapping_add((*wp).sy) { return 4i32 }
        }
        return 1i32.wrapping_neg()
    };
}
unsafe extern "C" fn screen_redraw_check_cell(mut c: *mut client,
                                              mut px: u_int, mut py: u_int,
                                              mut pane_status: libc::c_int,
                                              mut wpp: *mut *mut window_pane)
 -> libc::c_int {
    let mut current_block: u64;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut borders: libc::c_int = 0;
    let mut right: u_int = 0;
    let mut line: u_int = 0;
    *wpp = 0 as *mut window_pane;
    if px > (*w).sx || py > (*w).sy {
        return 12i32
    } else {
        if pane_status != 0i32 {
            wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
            while wp != 0 as *mut libc::c_void as *mut window_pane {
                if !(0 == window_pane_visible(wp)) {
                    if pane_status == 1i32 {
                        line = (*wp).yoff.wrapping_sub(1i32 as libc::c_uint)
                    } else { line = (*wp).yoff.wrapping_add((*wp).sy) }
                    right =
                        ((*wp).xoff.wrapping_add(2i32 as libc::c_uint) as
                             libc::c_ulong).wrapping_add((*wp).status_size).wrapping_sub(1i32
                                                                                             as
                                                                                             libc::c_ulong)
                            as u_int;
                    if py == line &&
                           px >= (*wp).xoff.wrapping_add(2i32 as libc::c_uint)
                           && px <= right {
                        return 0i32
                    }
                }
                wp = (*wp).entry.tqe_next
            }
        }
        wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
        loop  {
            if wp != 0 as *mut libc::c_void as *mut window_pane {
                if !(0 == window_pane_visible(wp)) {
                    *wpp = wp;
                    if !((*wp).xoff != 0i32 as libc::c_uint &&
                             px <
                                 (*wp).xoff.wrapping_sub(1i32 as libc::c_uint)
                             || px > (*wp).xoff.wrapping_add((*wp).sx) ||
                             (*wp).yoff != 0i32 as libc::c_uint &&
                                 py <
                                     (*wp).yoff.wrapping_sub(1i32 as
                                                                 libc::c_uint)
                             || py > (*wp).yoff.wrapping_add((*wp).sy)) {
                        if 0 == screen_redraw_cell_border(c, px, py) {
                            return 0i32
                        } else {
                            borders = 0i32;
                            if px == 0i32 as libc::c_uint ||
                                   0 !=
                                       screen_redraw_cell_border(c,
                                                                 px.wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_uint),
                                                                 py) {
                                borders |= 8i32
                            }
                            if px <= (*w).sx &&
                                   0 !=
                                       screen_redraw_cell_border(c,
                                                                 px.wrapping_add(1i32
                                                                                     as
                                                                                     libc::c_uint),
                                                                 py) {
                                borders |= 4i32
                            }
                            if pane_status == 1i32 {
                                if py != 0i32 as libc::c_uint &&
                                       0 !=
                                           screen_redraw_cell_border(c, px,
                                                                     py.wrapping_sub(1i32
                                                                                         as
                                                                                         libc::c_uint))
                                   {
                                    borders |= 2i32
                                }
                            } else if py == 0i32 as libc::c_uint ||
                                          0 !=
                                              screen_redraw_cell_border(c, px,
                                                                        py.wrapping_sub(1i32
                                                                                            as
                                                                                            libc::c_uint))
                             {
                                borders |= 2i32
                            }
                            if py <= (*w).sy &&
                                   0 !=
                                       screen_redraw_cell_border(c, px,
                                                                 py.wrapping_add(1i32
                                                                                     as
                                                                                     libc::c_uint))
                               {
                                borders |= 1i32
                            }
                            match borders {
                                15 => {
                                    current_block = 1741198717206098829;
                                    match current_block {
                                        14250624762762127536 => {
                                            return 7i32
                                        }
                                        11972837138215406864 => {
                                            return 5i32
                                        }
                                        15157255837846000548 => {
                                            return 2i32
                                        }
                                        5242414451524867936 => { return 1i32 }
                                        15665623852012128862 => {
                                            return 4i32
                                        }
                                        8165433048226719820 => { return 6i32 }
                                        15458570239431914930 => {
                                            return 8i32
                                        }
                                        1741198717206098829 => {
                                            return 11i32
                                        }
                                        8953692784526113934 => {
                                            return 10i32
                                        }
                                        15561448070103519713 => {
                                            return 9i32
                                        }
                                        _ => { return 3i32 }
                                    }
                                }
                                14 => {
                                    current_block = 15458570239431914930;
                                    match current_block {
                                        14250624762762127536 => {
                                            return 7i32
                                        }
                                        11972837138215406864 => {
                                            return 5i32
                                        }
                                        15157255837846000548 => {
                                            return 2i32
                                        }
                                        5242414451524867936 => { return 1i32 }
                                        15665623852012128862 => {
                                            return 4i32
                                        }
                                        8165433048226719820 => { return 6i32 }
                                        15458570239431914930 => {
                                            return 8i32
                                        }
                                        1741198717206098829 => {
                                            return 11i32
                                        }
                                        8953692784526113934 => {
                                            return 10i32
                                        }
                                        15561448070103519713 => {
                                            return 9i32
                                        }
                                        _ => { return 3i32 }
                                    }
                                }
                                13 => {
                                    current_block = 14250624762762127536;
                                    match current_block {
                                        14250624762762127536 => {
                                            return 7i32
                                        }
                                        11972837138215406864 => {
                                            return 5i32
                                        }
                                        15157255837846000548 => {
                                            return 2i32
                                        }
                                        5242414451524867936 => { return 1i32 }
                                        15665623852012128862 => {
                                            return 4i32
                                        }
                                        8165433048226719820 => { return 6i32 }
                                        15458570239431914930 => {
                                            return 8i32
                                        }
                                        1741198717206098829 => {
                                            return 11i32
                                        }
                                        8953692784526113934 => {
                                            return 10i32
                                        }
                                        15561448070103519713 => {
                                            return 9i32
                                        }
                                        _ => { return 3i32 }
                                    }
                                }
                                12 => {
                                    current_block = 15157255837846000548;
                                    match current_block {
                                        14250624762762127536 => {
                                            return 7i32
                                        }
                                        11972837138215406864 => {
                                            return 5i32
                                        }
                                        15157255837846000548 => {
                                            return 2i32
                                        }
                                        5242414451524867936 => { return 1i32 }
                                        15665623852012128862 => {
                                            return 4i32
                                        }
                                        8165433048226719820 => { return 6i32 }
                                        15458570239431914930 => {
                                            return 8i32
                                        }
                                        1741198717206098829 => {
                                            return 11i32
                                        }
                                        8953692784526113934 => {
                                            return 10i32
                                        }
                                        15561448070103519713 => {
                                            return 9i32
                                        }
                                        _ => { return 3i32 }
                                    }
                                }
                                11 => {
                                    current_block = 8953692784526113934;
                                    match current_block {
                                        14250624762762127536 => {
                                            return 7i32
                                        }
                                        11972837138215406864 => {
                                            return 5i32
                                        }
                                        15157255837846000548 => {
                                            return 2i32
                                        }
                                        5242414451524867936 => { return 1i32 }
                                        15665623852012128862 => {
                                            return 4i32
                                        }
                                        8165433048226719820 => { return 6i32 }
                                        15458570239431914930 => {
                                            return 8i32
                                        }
                                        1741198717206098829 => {
                                            return 11i32
                                        }
                                        8953692784526113934 => {
                                            return 10i32
                                        }
                                        15561448070103519713 => {
                                            return 9i32
                                        }
                                        _ => { return 3i32 }
                                    }
                                }
                                10 => {
                                    current_block = 8165433048226719820;
                                    match current_block {
                                        14250624762762127536 => {
                                            return 7i32
                                        }
                                        11972837138215406864 => {
                                            return 5i32
                                        }
                                        15157255837846000548 => {
                                            return 2i32
                                        }
                                        5242414451524867936 => { return 1i32 }
                                        15665623852012128862 => {
                                            return 4i32
                                        }
                                        8165433048226719820 => { return 6i32 }
                                        15458570239431914930 => {
                                            return 8i32
                                        }
                                        1741198717206098829 => {
                                            return 11i32
                                        }
                                        8953692784526113934 => {
                                            return 10i32
                                        }
                                        15561448070103519713 => {
                                            return 9i32
                                        }
                                        _ => { return 3i32 }
                                    }
                                }
                                9 => {
                                    current_block = 15665623852012128862;
                                    match current_block {
                                        14250624762762127536 => {
                                            return 7i32
                                        }
                                        11972837138215406864 => {
                                            return 5i32
                                        }
                                        15157255837846000548 => {
                                            return 2i32
                                        }
                                        5242414451524867936 => { return 1i32 }
                                        15665623852012128862 => {
                                            return 4i32
                                        }
                                        8165433048226719820 => { return 6i32 }
                                        15458570239431914930 => {
                                            return 8i32
                                        }
                                        1741198717206098829 => {
                                            return 11i32
                                        }
                                        8953692784526113934 => {
                                            return 10i32
                                        }
                                        15561448070103519713 => {
                                            return 9i32
                                        }
                                        _ => { return 3i32 }
                                    }
                                }
                                7 => {
                                    current_block = 15561448070103519713;
                                    match current_block {
                                        14250624762762127536 => {
                                            return 7i32
                                        }
                                        11972837138215406864 => {
                                            return 5i32
                                        }
                                        15157255837846000548 => {
                                            return 2i32
                                        }
                                        5242414451524867936 => { return 1i32 }
                                        15665623852012128862 => {
                                            return 4i32
                                        }
                                        8165433048226719820 => { return 6i32 }
                                        15458570239431914930 => {
                                            return 8i32
                                        }
                                        1741198717206098829 => {
                                            return 11i32
                                        }
                                        8953692784526113934 => {
                                            return 10i32
                                        }
                                        15561448070103519713 => {
                                            return 9i32
                                        }
                                        _ => { return 3i32 }
                                    }
                                }
                                6 => {
                                    current_block = 11972837138215406864;
                                    match current_block {
                                        14250624762762127536 => {
                                            return 7i32
                                        }
                                        11972837138215406864 => {
                                            return 5i32
                                        }
                                        15157255837846000548 => {
                                            return 2i32
                                        }
                                        5242414451524867936 => { return 1i32 }
                                        15665623852012128862 => {
                                            return 4i32
                                        }
                                        8165433048226719820 => { return 6i32 }
                                        15458570239431914930 => {
                                            return 8i32
                                        }
                                        1741198717206098829 => {
                                            return 11i32
                                        }
                                        8953692784526113934 => {
                                            return 10i32
                                        }
                                        15561448070103519713 => {
                                            return 9i32
                                        }
                                        _ => { return 3i32 }
                                    }
                                }
                                5 => {
                                    current_block = 17616954722366564836;
                                    match current_block {
                                        14250624762762127536 => {
                                            return 7i32
                                        }
                                        11972837138215406864 => {
                                            return 5i32
                                        }
                                        15157255837846000548 => {
                                            return 2i32
                                        }
                                        5242414451524867936 => { return 1i32 }
                                        15665623852012128862 => {
                                            return 4i32
                                        }
                                        8165433048226719820 => { return 6i32 }
                                        15458570239431914930 => {
                                            return 8i32
                                        }
                                        1741198717206098829 => {
                                            return 11i32
                                        }
                                        8953692784526113934 => {
                                            return 10i32
                                        }
                                        15561448070103519713 => {
                                            return 9i32
                                        }
                                        _ => { return 3i32 }
                                    }
                                }
                                3 => {
                                    current_block = 5242414451524867936;
                                    match current_block {
                                        14250624762762127536 => {
                                            return 7i32
                                        }
                                        11972837138215406864 => {
                                            return 5i32
                                        }
                                        15157255837846000548 => {
                                            return 2i32
                                        }
                                        5242414451524867936 => { return 1i32 }
                                        15665623852012128862 => {
                                            return 4i32
                                        }
                                        8165433048226719820 => { return 6i32 }
                                        15458570239431914930 => {
                                            return 8i32
                                        }
                                        1741198717206098829 => {
                                            return 11i32
                                        }
                                        8953692784526113934 => {
                                            return 10i32
                                        }
                                        15561448070103519713 => {
                                            return 9i32
                                        }
                                        _ => { return 3i32 }
                                    }
                                }
                                _ => { }
                            }
                        }
                    }
                }
                wp = (*wp).entry.tqe_next
            } else { return 12i32 }
        }
    };
}
unsafe extern "C" fn screen_redraw_cell_border(mut c: *mut client,
                                               mut px: u_int, mut py: u_int)
 -> libc::c_int {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut retval: libc::c_int = 0;
    wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
    loop  {
        if wp != 0 as *mut libc::c_void as *mut window_pane {
            if !(0 == window_pane_visible(wp)) {
                retval = screen_redraw_cell_border1(wp, px, py);
                if retval != 1i32.wrapping_neg() {
                    return !(0 == retval) as libc::c_int
                }
            }
            wp = (*wp).entry.tqe_next
        } else { return 0i32 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_redraw_pane(mut c: *mut client,
                                            mut wp: *mut window_pane) -> () {
    let mut i: u_int = 0;
    let mut yoff: u_int = 0;
    if 0 == window_pane_visible(wp) {
        return
    } else {
        yoff = (*wp).yoff;
        if status_at_line(c) == 0i32 {
            yoff =
                (yoff as
                     libc::c_uint).wrapping_add(status_line_size((*c).session))
                    as u_int as u_int
        }
        log_debug(b"%s: redraw pane %%%u (at %u,%u)\x00" as *const u8 as
                      *const libc::c_char, (*c).name, (*wp).id, (*wp).xoff,
                  yoff);
        i = 0i32 as u_int;
        while i < (*wp).sy {
            tty_draw_pane(&mut (*c).tty as *mut tty, wp, i, (*wp).xoff, yoff);
            i = i.wrapping_add(1)
        }
        tty_reset(&mut (*c).tty as *mut tty);
        return;
    };
}

