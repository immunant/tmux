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
    pub type options;
    pub type tmuxproc;
    pub type args_entry;
    pub type tmuxpeer;
    pub type bufferevent_ops;
    pub type event_base;
    pub type _IO_FILE_plus;
    pub type environ;
    pub type tty_code;
    pub type format_tree;
    pub type hooks;
    pub type screen_titles;
    pub type evbuffer;
    pub type input_ctx;
    pub type format_job_tree;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
     -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
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
    static mut __tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut __daylight: libc::c_int;
    #[no_mangle]
    static mut __timezone: libc::c_long;
    #[no_mangle]
    static mut tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut daylight: libc::c_int;
    #[no_mangle]
    static mut timezone: libc::c_long;
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
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
    #[no_mangle]
    fn event_initialized(ev: *const event) -> libc::c_int;
    #[no_mangle]
    fn event_once(_: libc::c_int, _: libc::c_short,
                  _:
                      Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_short,
                                                  _: *mut libc::c_void)
                                 -> ()>, _: *mut libc::c_void,
                  _: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_set(_: *mut event, _: libc::c_int, _: libc::c_short,
                 _:
                     Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_short,
                                                 _: *mut libc::c_void) -> ()>,
                 _: *mut libc::c_void) -> ();
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
    fn areshell(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn hooks_create(_: *mut hooks) -> *mut hooks;
    #[no_mangle]
    fn hooks_free(_: *mut hooks) -> ();
    #[no_mangle]
    fn notify_session(_: *const libc::c_char, _: *mut session) -> ();
    #[no_mangle]
    fn notify_session_window(_: *const libc::c_char, _: *mut session,
                             _: *mut window) -> ();
    #[no_mangle]
    fn options_create(_: *mut options) -> *mut options;
    #[no_mangle]
    fn options_free(_: *mut options) -> ();
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
    fn environ_create() -> *mut environ;
    #[no_mangle]
    fn environ_free(_: *mut environ) -> ();
    #[no_mangle]
    fn environ_copy(_: *mut environ, _: *mut environ) -> ();
    #[no_mangle]
    fn environ_for_session(_: *mut session, _: libc::c_int) -> *mut environ;
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
    fn server_lock_session(_: *mut session) -> ();
    #[no_mangle]
    fn status_update_saved(s: *mut session) -> ();
    #[no_mangle]
    fn recalculate_sizes() -> ();
    #[no_mangle]
    static grid_default_cell: grid_cell;
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
    fn winlink_find_by_window(_: *mut winlinks, _: *mut window)
     -> *mut winlink;
    #[no_mangle]
    fn winlink_find_by_window_id(_: *mut winlinks, _: u_int) -> *mut winlink;
    #[no_mangle]
    fn winlink_add(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn winlink_set_window(_: *mut winlink, _: *mut window) -> ();
    #[no_mangle]
    fn winlink_remove(_: *mut winlinks, _: *mut winlink) -> ();
    #[no_mangle]
    fn winlink_next(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn winlink_previous(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn winlink_stack_push(_: *mut winlink_stack, _: *mut winlink) -> ();
    #[no_mangle]
    fn winlink_stack_remove(_: *mut winlink_stack, _: *mut winlink) -> ();
    #[no_mangle]
    fn window_update_activity(_: *mut window) -> ();
    #[no_mangle]
    fn window_create_spawn(_: *const libc::c_char, _: libc::c_int,
                           _: *mut *mut libc::c_char, _: *const libc::c_char,
                           _: *const libc::c_char, _: *const libc::c_char,
                           _: *mut environ, _: *mut termios, _: u_int,
                           _: u_int, _: u_int, _: *mut *mut libc::c_char)
     -> *mut window;
    #[no_mangle]
    fn winlink_clear_flags(_: *mut winlink) -> ();
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
    fn fatal(_: *const libc::c_char, ...) -> !;
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
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
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
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type __timezone_ptr_t = *mut timezone;
pub type options_table_scope = libc::c_uint;
pub type u_char = __u_char;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const TTY_VT320: unnamed_28 = 4;
pub type tcflag_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type cmd_find_type = libc::c_uint;
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
pub struct unnamed_3 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type key_code = libc::c_ulonglong;
pub const TTY_VT220: unnamed_28 = 3;
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
pub const TTY_VT100: unnamed_28 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_5 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
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
    pub alerts_entry: unnamed_33,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_34,
    pub entry: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_30,
    pub ev_next: unnamed_2,
    pub ev_timeout_pos: unnamed_24,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_21,
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
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_14,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_16,
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
    pub entry: unnamed_25,
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
pub struct unnamed_7 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub type uint16_t = libc::c_ushort;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const CMDQ_COMMAND: cmdq_type = 0;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
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
    pub prompt_mode: unnamed_17,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_13,
}
pub const TTY_UNKNOWN: unnamed_28 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
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
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type u_int = __u_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_12 {
    offset: u_int,
    data: unnamed_36,
}
pub const TTY_VT101: unnamed_28 = 1;
pub const LINE_SEL_NONE: unnamed_26 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
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
pub type uint8_t = libc::c_uchar;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type unnamed_16 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub type cmdq_type = libc::c_uint;
pub type pid_t = __pid_t;
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_5,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_12,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub const LINE_SEL_RIGHT_LEFT: unnamed_26 = 2;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type unnamed_17 = libc::c_uint;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
pub struct unnamed_18 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub type options_table_type = libc::c_uint;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed,
}
pub const PROMPT_ENTRY: unnamed_17 = 0;
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const JOB_RUNNING: unnamed_16 = 0;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
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
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_23,
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
    pub entry: unnamed_20,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_22,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const JOB_CLOSED: unnamed_16 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_21 {
    ev_io: unnamed_32,
    ev_signal: unnamed_35,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
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
    pub entry: unnamed_29,
    pub tree_entry: unnamed_4,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const TTY_VT102: unnamed_28 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_24 {
    ev_next_with_common_timeout: unnamed_38,
    min_heap_idx: libc::c_int,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
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
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_1,
}
pub type __off_t = libc::c_long;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type __suseconds_t = libc::c_long;
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
    pub gentry: unnamed_6,
    pub entry: unnamed_3,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub type unnamed_26 = libc::c_uint;
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
pub struct unnamed_27 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub type __u_char = libc::c_uchar;
pub const LINE_SEL_LEFT_RIGHT: unnamed_26 = 1;
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
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_18,
    pub entry: unnamed_10,
}
pub type speed_t = libc::c_uint;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub type cmd_retval = libc::c_int;
pub type unnamed_28 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
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
pub type u_short = __u_short;
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
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
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
    pub entry: unnamed_8,
    pub wentry: unnamed_31,
    pub sentry: unnamed_11,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub ev_io_next: unnamed_27,
    pub ev_timeout: timeval,
}
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
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
pub type __u_int = libc::c_uint;
pub type __pid_t = libc::c_int;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub ev_signal_next: unnamed_15,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub type __u_short = libc::c_ushort;
pub const TTY_VT420: unnamed_28 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub type size_t = libc::c_ulong;
pub const JOB_DEAD: unnamed_16 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
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
    pub entry: unnamed_37,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const PROMPT_COMMAND: unnamed_17 = 1;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[no_mangle]
pub unsafe extern "C" fn session_cmp(mut s1: *mut session,
                                     mut s2: *mut session) -> libc::c_int {
    return strcmp((*s1).name, (*s2).name);
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_INSERT_COLOR(mut head: *mut sessions,
                                                  mut elm: *mut session)
 -> () {
    let mut current_block: u64;
    let mut parent: *mut session = 0 as *mut session;
    let mut gparent: *mut session = 0 as *mut session;
    let mut tmp: *mut session = 0 as *mut session;
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
                            7351195479953500246 => {
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
                                        10048703153582371463 => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    10048703153582371463;
                                            } else {
                                                current_block =
                                                    10879442775620481940;
                                            }
                                        }
                                        _ => {
                                            if 0 != 0i32 {
                                                current_block =
                                                    7351195479953500246;
                                                continue 's_87 ;
                                            } else { break ; }
                                        }
                                    }
                                }
                                tmp = parent;
                                parent = elm;
                                elm = tmp;
                                current_block = 4956146061682418353;
                            }
                            _ => {
                                (*parent).entry.rbe_color = 0i32;
                                (*gparent).entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4956146061682418353;
                                } else { break ; }
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
                's_211:
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
                                                continue 's_211 ;
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
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_REMOVE_COLOR(mut head: *mut sessions,
                                                  mut parent: *mut session,
                                                  mut elm: *mut session)
 -> () {
    let mut current_block: u64;
    let mut tmp: *mut session = 0 as *mut session;
    's_4:
        loop  {
            if !((elm == 0 as *mut libc::c_void as *mut session ||
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
                                            11050875288958768710 => {
                                                if 0 != 0i32 {
                                                    current_block =
                                                        11050875288958768710;
                                                } else {
                                                    current_block =
                                                        15240798224410183470;
                                                }
                                            }
                                            _ => {
                                                if 0 != 0i32 {
                                                    break ;
                                                } else { break 's_30 ; }
                                            }
                                        }
                                    }
                                }
                            tmp = (*parent).entry.rbe_right;
                            current_block = 14155750587950065367;
                        }
                        _ => {
                            if ((*tmp).entry.rbe_left ==
                                    0 as *mut libc::c_void as *mut session ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as *mut session
                                        ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_right ==
                                          0 as *mut libc::c_void as
                                              *mut session ||
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
                                    0 as *mut libc::c_void as *mut session ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as *mut session
                                        ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_left ==
                                          0 as *mut libc::c_void as
                                              *mut session ||
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
        13826291924415791078 => {
            let mut oright: *mut session = 0 as *mut session;
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
        15976848397966268834 => {
            let mut oleft: *mut session = 0 as *mut session;
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
pub unsafe extern "C" fn sessions_RB_REMOVE(mut head: *mut sessions,
                                            mut elm: *mut session)
 -> *mut session {
    let mut current_block: u64;
    let mut child: *mut session = 0 as *mut session;
    let mut parent: *mut session = 0 as *mut session;
    let mut old: *mut session = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left == 0 as *mut libc::c_void as *mut session {
        child = (*elm).entry.rbe_right;
        current_block = 9386390421034826751;
    } else if (*elm).entry.rbe_right == 0 as *mut libc::c_void as *mut session
     {
        child = (*elm).entry.rbe_left;
        current_block = 9386390421034826751;
    } else {
        let mut left: *mut session = 0 as *mut session;
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
            current_block = 3164141441203627222;
        } else { current_block = 3164141441203627222; }
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
    if color == 0i32 { sessions_RB_REMOVE_COLOR(head, parent, child); }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_INSERT(mut head: *mut sessions,
                                            mut elm: *mut session)
 -> *mut session {
    let mut tmp: *mut session = 0 as *mut session;
    let mut parent: *mut session = 0 as *mut session;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = session_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    loop  {
        (*elm).entry.rbe_parent = parent;
        (*elm).entry.rbe_right = 0 as *mut session;
        (*elm).entry.rbe_left = (*elm).entry.rbe_right;
        (*elm).entry.rbe_color = 1i32;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut session {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else { (*parent).entry.rbe_right = elm }
        while 0 != 0i32 { }
    } else { (*head).rbh_root = elm }
    sessions_RB_INSERT_COLOR(head, elm);
    return 0 as *mut session;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_FIND(mut head: *mut sessions,
                                          mut elm: *mut session)
 -> *mut session {
    let mut tmp: *mut session = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = session_cmp(elm, tmp);
            if comp < 0i32 {
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return 0 as *mut session }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_NFIND(mut head: *mut sessions,
                                           mut elm: *mut session)
 -> *mut session {
    let mut tmp: *mut session = (*head).rbh_root;
    let mut res: *mut session = 0 as *mut session;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = session_cmp(elm, tmp);
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
pub unsafe extern "C" fn sessions_RB_NEXT(mut elm: *mut session)
 -> *mut session {
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
pub unsafe extern "C" fn sessions_RB_PREV(mut elm: *mut session)
 -> *mut session {
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
pub unsafe extern "C" fn sessions_RB_MINMAX(mut head: *mut sessions,
                                            mut val: libc::c_int)
 -> *mut session {
    let mut tmp: *mut session = (*head).rbh_root;
    let mut parent: *mut session = 0 as *mut session;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else { tmp = (*tmp).entry.rbe_right }
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn session_group_cmp(mut s1: *mut session_group,
                                           mut s2: *mut session_group)
 -> libc::c_int {
    return strcmp((*s1).name, (*s2).name);
}
#[no_mangle]
pub unsafe extern "C" fn session_groups_RB_INSERT_COLOR(mut head:
                                                            *mut session_groups,
                                                        mut elm:
                                                            *mut session_group)
 -> () {
    let mut current_block: u64;
    let mut parent: *mut session_group = 0 as *mut session_group;
    let mut gparent: *mut session_group = 0 as *mut session_group;
    let mut tmp: *mut session_group = 0 as *mut session_group;
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
                            7351195479953500246 => {
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
                            _ => {
                                (*parent).entry.rbe_color = 0i32;
                                (*gparent).entry.rbe_color = 1i32;
                                if 0 != 0i32 {
                                    current_block = 4956146061682418353;
                                } else { break ; }
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
pub unsafe extern "C" fn session_groups_RB_REMOVE_COLOR(mut head:
                                                            *mut session_groups,
                                                        mut parent:
                                                            *mut session_group,
                                                        mut elm:
                                                            *mut session_group)
 -> () {
    let mut current_block: u64;
    let mut tmp: *mut session_group = 0 as *mut session_group;
    's_4:
        loop  {
            if !((elm == 0 as *mut libc::c_void as *mut session_group ||
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
                                    0 as *mut libc::c_void as
                                        *mut session_group ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut session_group ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_right ==
                                          0 as *mut libc::c_void as
                                              *mut session_group ||
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
                                    0 as *mut libc::c_void as
                                        *mut session_group ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut session_group ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_left ==
                                          0 as *mut libc::c_void as
                                              *mut session_group ||
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
        13826291924415791078 => {
            let mut oright: *mut session_group = 0 as *mut session_group;
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
                            3392087639489470149 => {
                                if 0 != 0i32 {
                                    current_block = 3392087639489470149;
                                } else {
                                    current_block = 1854459640724737493;
                                }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_276 ; }
                            }
                        }
                    }
                }
            tmp = (*parent).entry.rbe_left;
            current_block = 5892776923941496671;
        }
        15976848397966268834 => {
            let mut oleft: *mut session_group = 0 as *mut session_group;
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
                            13707613154239713890 => {
                                if 0 != 0i32 {
                                    break ;
                                } else { break 's_328 ; }
                            }
                            _ => {
                                if 0 != 0i32 {
                                    current_block = 13910774313357589740;
                                } else {
                                    current_block = 13707613154239713890;
                                }
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
pub unsafe extern "C" fn session_groups_RB_REMOVE(mut head:
                                                      *mut session_groups,
                                                  mut elm: *mut session_group)
 -> *mut session_group {
    let mut current_block: u64;
    let mut child: *mut session_group = 0 as *mut session_group;
    let mut parent: *mut session_group = 0 as *mut session_group;
    let mut old: *mut session_group = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left == 0 as *mut libc::c_void as *mut session_group {
        child = (*elm).entry.rbe_right;
        current_block = 9386390421034826751;
    } else if (*elm).entry.rbe_right ==
                  0 as *mut libc::c_void as *mut session_group {
        child = (*elm).entry.rbe_left;
        current_block = 9386390421034826751;
    } else {
        let mut left: *mut session_group = 0 as *mut session_group;
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
            current_block = 16331546839105579257;
        } else { current_block = 16331546839105579257; }
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
    if color == 0i32 { session_groups_RB_REMOVE_COLOR(head, parent, child); }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn session_groups_RB_INSERT(mut head:
                                                      *mut session_groups,
                                                  mut elm: *mut session_group)
 -> *mut session_group {
    let mut tmp: *mut session_group = 0 as *mut session_group;
    let mut parent: *mut session_group = 0 as *mut session_group;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = session_group_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    loop  {
        (*elm).entry.rbe_parent = parent;
        (*elm).entry.rbe_right = 0 as *mut session_group;
        (*elm).entry.rbe_left = (*elm).entry.rbe_right;
        (*elm).entry.rbe_color = 1i32;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut session_group {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else { (*parent).entry.rbe_right = elm }
        while 0 != 0i32 { }
    } else { (*head).rbh_root = elm }
    session_groups_RB_INSERT_COLOR(head, elm);
    return 0 as *mut session_group;
}
#[no_mangle]
pub unsafe extern "C" fn session_groups_RB_FIND(mut head: *mut session_groups,
                                                mut elm: *mut session_group)
 -> *mut session_group {
    let mut tmp: *mut session_group = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = session_group_cmp(elm, tmp);
            if comp < 0i32 {
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return 0 as *mut session_group }
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_groups_RB_NFIND(mut head:
                                                     *mut session_groups,
                                                 mut elm: *mut session_group)
 -> *mut session_group {
    let mut tmp: *mut session_group = (*head).rbh_root;
    let mut res: *mut session_group = 0 as *mut session_group;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = session_group_cmp(elm, tmp);
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
pub unsafe extern "C" fn session_groups_RB_NEXT(mut elm: *mut session_group)
 -> *mut session_group {
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
pub unsafe extern "C" fn session_groups_RB_PREV(mut elm: *mut session_group)
 -> *mut session_group {
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
pub unsafe extern "C" fn session_groups_RB_MINMAX(mut head:
                                                      *mut session_groups,
                                                  mut val: libc::c_int)
 -> *mut session_group {
    let mut tmp: *mut session_group = (*head).rbh_root;
    let mut parent: *mut session_group = 0 as *mut session_group;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else { tmp = (*tmp).entry.rbe_right }
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn session_alive(mut s: *mut session) -> libc::c_int {
    let mut s_loop: *mut session = 0 as *mut session;
    s_loop =
        sessions_RB_MINMAX(&mut sessions as *mut sessions,
                           1i32.wrapping_neg());
    loop  {
        if s_loop != 0 as *mut libc::c_void as *mut session {
            if s_loop == s {
                return 1i32
            } else { s_loop = sessions_RB_NEXT(s_loop) }
        } else { return 0i32 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_find(mut name: *const libc::c_char)
 -> *mut session {
    let mut s: session =
        session{id: 0,
                name: 0 as *mut libc::c_char,
                cwd: 0 as *const libc::c_char,
                creation_time: timeval{tv_sec: 0, tv_usec: 0,},
                last_attached_time: timeval{tv_sec: 0, tv_usec: 0,},
                activity_time: timeval{tv_sec: 0, tv_usec: 0,},
                last_activity_time: timeval{tv_sec: 0, tv_usec: 0,},
                lock_timer:
                    event{ev_active_next:
                              unnamed_30{tqe_next: 0 as *mut event,
                                         tqe_prev: 0 as *mut *mut event,},
                          ev_next:
                              unnamed_2{tqe_next: 0 as *mut event,
                                        tqe_prev: 0 as *mut *mut event,},
                          ev_timeout_pos:
                              unnamed_24{ev_next_with_common_timeout:
                                             unnamed_38{tqe_next:
                                                            0 as *mut event,
                                                        tqe_prev:
                                                            0 as
                                                                *mut *mut event,},},
                          ev_fd: 0,
                          ev_base: 0 as *mut event_base,
                          _ev:
                              unnamed_21{ev_io:
                                             unnamed_32{ev_io_next:
                                                            unnamed_27{tqe_next:
                                                                           0
                                                                               as
                                                                               *mut event,
                                                                       tqe_prev:
                                                                           0
                                                                               as
                                                                               *mut *mut event,},
                                                        ev_timeout:
                                                            timeval{tv_sec: 0,
                                                                    tv_usec:
                                                                        0,},},},
                          ev_events: 0,
                          ev_res: 0,
                          ev_flags: 0,
                          ev_pri: 0,
                          ev_closure: 0,
                          ev_timeout: timeval{tv_sec: 0, tv_usec: 0,},
                          ev_callback: None,
                          ev_arg: 0 as *mut libc::c_void,},
                sx: 0,
                sy: 0,
                curw: 0 as *mut winlink,
                lastw:
                    winlink_stack{tqh_first: 0 as *mut winlink,
                                  tqh_last: 0 as *mut *mut winlink,},
                windows: winlinks{rbh_root: 0 as *mut winlink,},
                statusat: 0,
                hooks: 0 as *mut hooks,
                options: 0 as *mut options,
                flags: 0,
                attached: 0,
                tio: 0 as *mut termios,
                environ: 0 as *mut environ,
                references: 0,
                gentry:
                    unnamed_6{tqe_next: 0 as *mut session,
                              tqe_prev: 0 as *mut *mut session,},
                entry:
                    unnamed_3{rbe_left: 0 as *mut session,
                              rbe_right: 0 as *mut session,
                              rbe_parent: 0 as *mut session,
                              rbe_color: 0,},};
    s.name = name as *mut libc::c_char;
    return sessions_RB_FIND(&mut sessions as *mut sessions,
                            &mut s as *mut session);
}
#[no_mangle]
pub unsafe extern "C" fn session_find_by_id_str(mut s: *const libc::c_char)
 -> *mut session {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut id: u_int = 0;
    if *s as libc::c_int != 36 {
        return 0 as *mut session
    } else {
        id =
            strtonum(s.offset(1isize), 0i32 as libc::c_longlong,
                     (2147483647i32 as
                          libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                         as libc::c_longlong,
                     &mut errstr as *mut *const libc::c_char) as u_int;
        if errstr != 0 as *mut libc::c_void as *const libc::c_char {
            return 0 as *mut session
        } else { return session_find_by_id(id) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_find_by_id(mut id: u_int) -> *mut session {
    let mut s: *mut session = 0 as *mut session;
    s =
        sessions_RB_MINMAX(&mut sessions as *mut sessions,
                           1i32.wrapping_neg());
    loop  {
        if s != 0 as *mut libc::c_void as *mut session {
            if (*s).id == id { return s } else { s = sessions_RB_NEXT(s) }
        } else { return 0 as *mut session }
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_create(mut prefix: *const libc::c_char,
                                        mut name: *const libc::c_char,
                                        mut argc: libc::c_int,
                                        mut argv: *mut *mut libc::c_char,
                                        mut path: *const libc::c_char,
                                        mut cwd: *const libc::c_char,
                                        mut env: *mut environ,
                                        mut tio: *mut termios,
                                        mut idx: libc::c_int, mut sx: u_int,
                                        mut sy: u_int,
                                        mut cause: *mut *mut libc::c_char)
 -> *mut session {
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    s =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<session>() as libc::c_ulong) as
            *mut session;
    (*s).references = 1i32;
    (*s).flags = 0i32;
    (*s).cwd = xstrdup(cwd);
    (*s).curw = 0 as *mut winlink;
    loop  {
        let ref mut fresh0 =
            (*(&mut (*s).lastw as *mut winlink_stack)).tqh_first;
        *fresh0 = 0 as *mut winlink;
        let ref mut fresh1 =
            (*(&mut (*s).lastw as *mut winlink_stack)).tqh_last;
        *fresh1 =
            &mut (*(&mut (*s).lastw as *mut winlink_stack)).tqh_first as
                *mut *mut winlink;
        if !(0 != 0i32) { break ; }
    }
    loop  {
        let ref mut fresh2 = (*(&mut (*s).windows as *mut winlinks)).rbh_root;
        *fresh2 = 0 as *mut winlink;
        if !(0 != 0i32) { break ; }
    }
    (*s).environ = environ_create();
    if env != 0 as *mut libc::c_void as *mut environ {
        environ_copy(env, (*s).environ);
    }
    (*s).options = options_create(global_s_options);
    (*s).hooks = hooks_create(global_hooks);
    status_update_saved(s);
    (*s).tio = 0 as *mut termios;
    if tio != 0 as *mut libc::c_void as *mut termios {
        (*s).tio =
            xmalloc(::std::mem::size_of::<termios>() as libc::c_ulong) as
                *mut termios;
        memcpy((*s).tio as *mut libc::c_void, tio as *const libc::c_void,
               ::std::mem::size_of::<termios>() as libc::c_ulong);
    }
    (*s).sx = sx;
    (*s).sy = sy;
    if name != 0 as *mut libc::c_void as *const libc::c_char {
        (*s).name = xstrdup(name);
        let fresh3 = next_session_id;
        next_session_id = next_session_id.wrapping_add(1);
        (*s).id = fresh3
    } else {
        (*s).name = 0 as *mut libc::c_char;
        loop  {
            let fresh4 = next_session_id;
            next_session_id = next_session_id.wrapping_add(1);
            (*s).id = fresh4;
            free((*s).name as *mut libc::c_void);
            if prefix != 0 as *mut libc::c_void as *const libc::c_char {
                xasprintf(&mut (*s).name as *mut *mut libc::c_char,
                          b"%s-%u\x00" as *const u8 as *const libc::c_char,
                          prefix, (*s).id);
            } else {
                xasprintf(&mut (*s).name as *mut *mut libc::c_char,
                          b"%u\x00" as *const u8 as *const libc::c_char,
                          (*s).id);
            }
            if !(sessions_RB_FIND(&mut sessions as *mut sessions, s) !=
                     0 as *mut libc::c_void as *mut session) {
                break ;
            }
        }
    }
    sessions_RB_INSERT(&mut sessions as *mut sessions, s);
    log_debug(b"new session %s $%u\x00" as *const u8 as *const libc::c_char,
              (*s).name, (*s).id);
    if gettimeofday(&mut (*s).creation_time as *mut timeval,
                    0 as *mut timezone) != 0i32 {
        fatal(b"gettimeofday failed\x00" as *const u8 as *const libc::c_char);
    } else {
        session_update_activity(s, &mut (*s).creation_time as *mut timeval);
        if argc >= 0i32 {
            wl =
                session_new(s, 0 as *const libc::c_char, argc, argv, path,
                            cwd, idx, cause);
            if wl == 0 as *mut libc::c_void as *mut winlink {
                session_destroy(s,
                                (*::std::mem::transmute::<&[u8; 15],
                                                          &[libc::c_char; 15]>(b"session_create\x00")).as_ptr());
                return 0 as *mut session
            } else {
                session_select(s,
                               (*(*(&mut (*s).windows as
                                        *mut winlinks)).rbh_root).idx);
            }
        }
        log_debug(b"session %s created\x00" as *const u8 as
                      *const libc::c_char, (*s).name);
        return s
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_select(mut s: *mut session,
                                        mut idx: libc::c_int) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlink_find_by_index(&mut (*s).windows as *mut winlinks, idx);
    return session_set_current(s, wl);
}
#[no_mangle]
pub unsafe extern "C" fn session_set_current(mut s: *mut session,
                                             mut wl: *mut winlink)
 -> libc::c_int {
    if wl == 0 as *mut libc::c_void as *mut winlink {
        return 1i32.wrapping_neg()
    } else if wl == (*s).curw {
        return 1i32
    } else {
        winlink_stack_remove(&mut (*s).lastw as *mut winlink_stack, wl);
        winlink_stack_push(&mut (*s).lastw as *mut winlink_stack, (*s).curw);
        (*s).curw = wl;
        winlink_clear_flags(wl);
        window_update_activity((*wl).window);
        notify_session(b"session-window-changed\x00" as *const u8 as
                           *const libc::c_char, s);
        return 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_destroy(mut s: *mut session,
                                         mut from: *const libc::c_char)
 -> () {
    let mut wl: *mut winlink = 0 as *mut winlink;
    log_debug(b"session %s destroyed (%s)\x00" as *const u8 as
                  *const libc::c_char, (*s).name, from);
    (*s).curw = 0 as *mut winlink;
    sessions_RB_REMOVE(&mut sessions as *mut sessions, s);
    notify_session(b"session-closed\x00" as *const u8 as *const libc::c_char,
                   s);
    free((*s).tio as *mut libc::c_void);
    if 0 != event_initialized(&mut (*s).lock_timer as *mut event) {
        event_del(&mut (*s).lock_timer as *mut event);
    }
    session_group_remove(s);
    while 0 !=
              !((*(&mut (*s).lastw as *mut winlink_stack)).tqh_first ==
                    0 as *mut libc::c_void as *mut winlink) as libc::c_int {
        winlink_stack_remove(&mut (*s).lastw as *mut winlink_stack,
                             (*(&mut (*s).lastw as
                                    *mut winlink_stack)).tqh_first);
    }
    while 0 !=
              !((*(&mut (*s).windows as *mut winlinks)).rbh_root ==
                    0 as *mut libc::c_void as *mut winlink) as libc::c_int {
        wl = (*(&mut (*s).windows as *mut winlinks)).rbh_root;
        notify_session_window(b"window-unlinked\x00" as *const u8 as
                                  *const libc::c_char, s, (*wl).window);
        winlink_remove(&mut (*s).windows as *mut winlinks, wl);
    }
    free((*s).cwd as *mut libc::c_void);
    session_remove_ref(s,
                       (*::std::mem::transmute::<&[u8; 16],
                                                 &[libc::c_char; 16]>(b"session_destroy\x00")).as_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn session_remove_ref(mut s: *mut session,
                                            mut from: *const libc::c_char)
 -> () {
    (*s).references -= 1;
    log_debug(b"%s: %s %s, now %d\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"session_remove_ref\x00")).as_ptr(),
              (*s).name, from, (*s).references);
    if (*s).references == 0i32 {
        event_once(1i32.wrapping_neg(), 1i32 as libc::c_short,
                   Some(session_free), s as *mut libc::c_void,
                   0 as *const timeval);
    };
}
unsafe extern "C" fn session_free(mut fd: libc::c_int,
                                  mut events: libc::c_short,
                                  mut arg: *mut libc::c_void) -> () {
    let mut s: *mut session = arg as *mut session;
    log_debug(b"session %s freed (%d references)\x00" as *const u8 as
                  *const libc::c_char, (*s).name, (*s).references);
    if (*s).references == 0i32 {
        environ_free((*s).environ);
        options_free((*s).options);
        hooks_free((*s).hooks);
        free((*s).name as *mut libc::c_void);
        free(s as *mut libc::c_void);
    };
}
unsafe extern "C" fn session_group_remove(mut s: *mut session) -> () {
    let mut sg: *mut session_group = 0 as *mut session_group;
    sg = session_group_contains(s);
    if sg == 0 as *mut libc::c_void as *mut session_group {
        return
    } else {
        loop  {
            if (*s).gentry.tqe_next != 0 as *mut libc::c_void as *mut session
               {
                (*(*s).gentry.tqe_next).gentry.tqe_prev = (*s).gentry.tqe_prev
            } else {
                let ref mut fresh5 =
                    (*(&mut (*sg).sessions as *mut unnamed_18)).tqh_last;
                *fresh5 = (*s).gentry.tqe_prev
            }
            *(*s).gentry.tqe_prev = (*s).gentry.tqe_next;
            if !(0 != 0i32) { break ; }
        }
        if (*(&mut (*sg).sessions as *mut unnamed_18)).tqh_first ==
               0 as *mut libc::c_void as *mut session {
            session_groups_RB_REMOVE(&mut session_groups as
                                         *mut session_groups, sg);
            free(sg as *mut libc::c_void);
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_group_contains(mut target: *mut session)
 -> *mut session_group {
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut s: *mut session = 0 as *mut session;
    sg =
        session_groups_RB_MINMAX(&mut session_groups as *mut session_groups,
                                 1i32.wrapping_neg());
    loop  {
        if sg != 0 as *mut libc::c_void as *mut session_group {
            s = (*(&mut (*sg).sessions as *mut unnamed_18)).tqh_first;
            loop  {
                if s != 0 as *mut libc::c_void as *mut session {
                    if s == target {
                        return sg
                    } else { s = (*s).gentry.tqe_next }
                } else { sg = session_groups_RB_NEXT(sg); break ; }
            }
        } else { return 0 as *mut session_group }
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_new(mut s: *mut session,
                                     mut name: *const libc::c_char,
                                     mut argc: libc::c_int,
                                     mut argv: *mut *mut libc::c_char,
                                     mut path: *const libc::c_char,
                                     mut cwd: *const libc::c_char,
                                     mut idx: libc::c_int,
                                     mut cause: *mut *mut libc::c_char)
 -> *mut winlink {
    let mut w: *mut window = 0 as *mut window;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut env: *mut environ = 0 as *mut environ;
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    let mut hlimit: u_int = 0;
    wl = winlink_add(&mut (*s).windows as *mut winlinks, idx);
    if wl == 0 as *mut libc::c_void as *mut winlink {
        xasprintf(cause,
                  b"index in use: %d\x00" as *const u8 as *const libc::c_char,
                  idx);
        return 0 as *mut winlink
    } else {
        (*wl).session = s;
        shell =
            options_get_string((*s).options,
                               b"default-shell\x00" as *const u8 as
                                   *const libc::c_char);
        if *shell as libc::c_int == 0 || 0 != areshell(shell) {
            shell = b"/bin/sh\x00" as *const u8 as *const libc::c_char
        }
        hlimit =
            options_get_number((*s).options,
                               b"history-limit\x00" as *const u8 as
                                   *const libc::c_char) as u_int;
        env = environ_for_session(s, 0i32);
        w =
            window_create_spawn(name, argc, argv, path, shell, cwd, env,
                                (*s).tio, (*s).sx, (*s).sy, hlimit, cause);
        if w == 0 as *mut libc::c_void as *mut window {
            winlink_remove(&mut (*s).windows as *mut winlinks, wl);
            environ_free(env);
            return 0 as *mut winlink
        } else {
            winlink_set_window(wl, w);
            environ_free(env);
            notify_session_window(b"window-linked\x00" as *const u8 as
                                      *const libc::c_char, s, w);
            session_group_synchronize_from(s);
            return wl
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_group_synchronize_from(mut target:
                                                            *mut session)
 -> () {
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut s: *mut session = 0 as *mut session;
    sg = session_group_contains(target);
    if sg == 0 as *mut libc::c_void as *mut session_group {
        return
    } else {
        s = (*(&mut (*sg).sessions as *mut unnamed_18)).tqh_first;
        while s != 0 as *mut libc::c_void as *mut session {
            if s != target { session_group_synchronize1(target, s); }
            s = (*s).gentry.tqe_next
        }
        return;
    };
}
unsafe extern "C" fn session_group_synchronize1(mut target: *mut session,
                                                mut s: *mut session) -> () {
    let mut current_block: u64;
    let mut old_windows: winlinks = winlinks{rbh_root: 0 as *mut winlink,};
    let mut ww: *mut winlinks = 0 as *mut winlinks;
    let mut old_lastw: winlink_stack =
        winlink_stack{tqh_first: 0 as *mut winlink,
                      tqh_last: 0 as *mut *mut winlink,};
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wl2: *mut winlink = 0 as *mut winlink;
    ww = &mut (*target).windows as *mut winlinks;
    if (*ww).rbh_root == 0 as *mut libc::c_void as *mut winlink {
        return
    } else {
        if (*s).curw != 0 as *mut libc::c_void as *mut winlink &&
               winlink_find_by_index(ww, (*(*s).curw).idx) ==
                   0 as *mut libc::c_void as *mut winlink &&
               session_last(s) != 0i32 && session_previous(s, 0i32) != 0i32 {
            session_next(s, 0i32);
        }
        memcpy(&mut old_windows as *mut winlinks as *mut libc::c_void,
               &mut (*s).windows as *mut winlinks as *const libc::c_void,
               ::std::mem::size_of::<winlinks>() as libc::c_ulong);
        loop  {
            let ref mut fresh6 =
                (*(&mut (*s).windows as *mut winlinks)).rbh_root;
            *fresh6 = 0 as *mut winlink;
            if !(0 != 0i32) { break ; }
        }
        wl = winlinks_RB_MINMAX(ww, 1i32.wrapping_neg());
        while wl != 0 as *mut libc::c_void as *mut winlink {
            wl2 = winlink_add(&mut (*s).windows as *mut winlinks, (*wl).idx);
            (*wl2).session = s;
            winlink_set_window(wl2, (*wl).window);
            notify_session_window(b"window-linked\x00" as *const u8 as
                                      *const libc::c_char, s, (*wl2).window);
            (*wl2).flags |= (*wl).flags & (1i32 | 2i32 | 4i32);
            wl = winlinks_RB_NEXT(wl)
        }
        if (*s).curw != 0 as *mut libc::c_void as *mut winlink {
            (*s).curw =
                winlink_find_by_index(&mut (*s).windows as *mut winlinks,
                                      (*(*s).curw).idx)
        } else {
            (*s).curw =
                winlink_find_by_index(&mut (*s).windows as *mut winlinks,
                                      (*(*target).curw).idx)
        }
        memcpy(&mut old_lastw as *mut winlink_stack as *mut libc::c_void,
               &mut (*s).lastw as *mut winlink_stack as *const libc::c_void,
               ::std::mem::size_of::<winlink_stack>() as libc::c_ulong);
        loop  {
            let ref mut fresh7 =
                (*(&mut (*s).lastw as *mut winlink_stack)).tqh_first;
            *fresh7 = 0 as *mut winlink;
            let ref mut fresh8 =
                (*(&mut (*s).lastw as *mut winlink_stack)).tqh_last;
            *fresh8 =
                &mut (*(&mut (*s).lastw as *mut winlink_stack)).tqh_first as
                    *mut *mut winlink;
            if !(0 != 0i32) { break ; }
        }
        wl = (*(&mut old_lastw as *mut winlink_stack)).tqh_first;
        while wl != 0 as *mut libc::c_void as *mut winlink {
            wl2 =
                winlink_find_by_index(&mut (*s).windows as *mut winlinks,
                                      (*wl).idx);
            if wl2 != 0 as *mut libc::c_void as *mut winlink {
                current_block = 12209867499936983673;
            } else { current_block = 3640593987805443782; }
            loop  {
                match current_block {
                    3640593987805443782 => {
                        wl = (*wl).sentry.tqe_next;
                        break ;
                    }
                    _ => {
                        (*wl2).sentry.tqe_next = 0 as *mut winlink;
                        (*wl2).sentry.tqe_prev =
                            (*(&mut (*s).lastw as
                                   *mut winlink_stack)).tqh_last;
                        let ref mut fresh9 =
                            *(*(&mut (*s).lastw as
                                    *mut winlink_stack)).tqh_last;
                        *fresh9 = wl2;
                        let ref mut fresh10 =
                            (*(&mut (*s).lastw as
                                   *mut winlink_stack)).tqh_last;
                        *fresh10 =
                            &mut (*wl2).sentry.tqe_next as *mut *mut winlink;
                        if 0 != 0i32 {
                            current_block = 12209867499936983673;
                        } else { current_block = 3640593987805443782; }
                    }
                }
            }
        }
        while 0 !=
                  !((*(&mut old_windows as *mut winlinks)).rbh_root ==
                        0 as *mut libc::c_void as *mut winlink) as libc::c_int
              {
            wl = (*(&mut old_windows as *mut winlinks)).rbh_root;
            wl2 =
                winlink_find_by_window_id(&mut (*s).windows as *mut winlinks,
                                          (*(*wl).window).id);
            if wl2 == 0 as *mut libc::c_void as *mut winlink {
                notify_session_window(b"window-unlinked\x00" as *const u8 as
                                          *const libc::c_char, s,
                                      (*wl).window);
            }
            winlink_remove(&mut old_windows as *mut winlinks, wl);
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_next(mut s: *mut session,
                                      mut alert: libc::c_int) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    if (*s).curw == 0 as *mut libc::c_void as *mut winlink {
        return 1i32.wrapping_neg()
    } else {
        wl = winlink_next((*s).curw);
        if 0 != alert { wl = session_next_alert(wl) }
        if wl == 0 as *mut libc::c_void as *mut winlink {
            wl =
                winlinks_RB_MINMAX(&mut (*s).windows as *mut winlinks,
                                   1i32.wrapping_neg());
            if 0 != alert &&
                   {
                       wl = session_next_alert(wl);
                       wl == 0 as *mut libc::c_void as *mut winlink
                   } {
                return 1i32.wrapping_neg()
            }
        }
        return session_set_current(s, wl)
    };
}
unsafe extern "C" fn session_next_alert(mut wl: *mut winlink)
 -> *mut winlink {
    while wl != 0 as *mut libc::c_void as *mut winlink {
        if 0 != (*wl).flags & (1i32 | 2i32 | 4i32) { break ; }
        wl = winlink_next(wl)
    }
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn session_previous(mut s: *mut session,
                                          mut alert: libc::c_int)
 -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    if (*s).curw == 0 as *mut libc::c_void as *mut winlink {
        return 1i32.wrapping_neg()
    } else {
        wl = winlink_previous((*s).curw);
        if 0 != alert { wl = session_previous_alert(wl) }
        if wl == 0 as *mut libc::c_void as *mut winlink {
            wl = winlinks_RB_MINMAX(&mut (*s).windows as *mut winlinks, 1i32);
            if 0 != alert &&
                   {
                       wl = session_previous_alert(wl);
                       wl == 0 as *mut libc::c_void as *mut winlink
                   } {
                return 1i32.wrapping_neg()
            }
        }
        return session_set_current(s, wl)
    };
}
unsafe extern "C" fn session_previous_alert(mut wl: *mut winlink)
 -> *mut winlink {
    while wl != 0 as *mut libc::c_void as *mut winlink {
        if 0 != (*wl).flags & (1i32 | 2i32 | 4i32) { break ; }
        wl = winlink_previous(wl)
    }
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn session_last(mut s: *mut session) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = (*(&mut (*s).lastw as *mut winlink_stack)).tqh_first;
    if wl == 0 as *mut libc::c_void as *mut winlink {
        return 1i32.wrapping_neg()
    } else if wl == (*s).curw {
        return 1i32
    } else { return session_set_current(s, wl) };
}
#[no_mangle]
pub unsafe extern "C" fn session_update_activity(mut s: *mut session,
                                                 mut from: *mut timeval)
 -> () {
    let mut last: *mut timeval = &mut (*s).last_activity_time as *mut timeval;
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    memcpy(last as *mut libc::c_void,
           &mut (*s).activity_time as *mut timeval as *const libc::c_void,
           ::std::mem::size_of::<timeval>() as libc::c_ulong);
    if from == 0 as *mut libc::c_void as *mut timeval {
        gettimeofday(&mut (*s).activity_time as *mut timeval,
                     0 as *mut timezone);
    } else {
        memcpy(&mut (*s).activity_time as *mut timeval as *mut libc::c_void,
               from as *const libc::c_void,
               ::std::mem::size_of::<timeval>() as libc::c_ulong);
    }
    log_debug(b"session %s activity %lld.%06d (last %lld.%06d)\x00" as
                  *const u8 as *const libc::c_char, (*s).name,
              (*s).activity_time.tv_sec as libc::c_longlong,
              (*s).activity_time.tv_usec as libc::c_int,
              (*last).tv_sec as libc::c_longlong,
              (*last).tv_usec as libc::c_int);
    if 0 != event_initialized(&mut (*s).lock_timer as *mut event) {
        event_del(&mut (*s).lock_timer as *mut event);
    } else {
        event_set(&mut (*s).lock_timer as *mut event, 1i32.wrapping_neg(),
                  0i32 as libc::c_short, Some(session_lock_timer),
                  s as *mut libc::c_void);
    }
    if 0 != !(*s).flags & 1i32 {
        let ref mut fresh11 = (*(&mut tv as *mut timeval)).tv_usec;
        *fresh11 = 0i32 as __suseconds_t;
        (*(&mut tv as *mut timeval)).tv_sec = *fresh11;
        tv.tv_sec =
            options_get_number((*s).options,
                               b"lock-after-time\x00" as *const u8 as
                                   *const libc::c_char) as __time_t;
        if tv.tv_sec != 0i32 as libc::c_long {
            event_add(&mut (*s).lock_timer as *mut event,
                      &mut tv as *mut timeval);
        }
    };
}
unsafe extern "C" fn session_lock_timer(mut fd: libc::c_int,
                                        mut events: libc::c_short,
                                        mut arg: *mut libc::c_void) -> () {
    let mut s: *mut session = arg as *mut session;
    if 0 != (*s).flags & 1i32 {
        return
    } else {
        log_debug(b"session %s locked, activity time %lld\x00" as *const u8 as
                      *const libc::c_char, (*s).name,
                  (*s).activity_time.tv_sec as libc::c_longlong);
        server_lock_session(s);
        recalculate_sizes();
        return;
    };
}
static mut next_session_id: u_int = unsafe { 0 };
#[no_mangle]
pub unsafe extern "C" fn session_add_ref(mut s: *mut session,
                                         mut from: *const libc::c_char)
 -> () {
    (*s).references += 1;
    log_debug(b"%s: %s %s, now %d\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"session_add_ref\x00")).as_ptr(),
              (*s).name, from, (*s).references);
}
#[no_mangle]
pub unsafe extern "C" fn session_check_name(mut name: *const libc::c_char)
 -> libc::c_int {
    return (*name as libc::c_int != 0 &&
                *name.offset(strcspn(name,
                                     b":.\x00" as *const u8 as
                                         *const libc::c_char) as isize) as
                    libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn session_next_session(mut s: *mut session)
 -> *mut session {
    let mut s2: *mut session = 0 as *mut session;
    if (*(&mut sessions as *mut sessions)).rbh_root ==
           0 as *mut libc::c_void as *mut session || 0 == session_alive(s) {
        return 0 as *mut session
    } else {
        s2 = sessions_RB_NEXT(s);
        if s2 == 0 as *mut libc::c_void as *mut session {
            s2 =
                sessions_RB_MINMAX(&mut sessions as *mut sessions,
                                   1i32.wrapping_neg())
        }
        if s2 == s { return 0 as *mut session } else { return s2 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_previous_session(mut s: *mut session)
 -> *mut session {
    let mut s2: *mut session = 0 as *mut session;
    if (*(&mut sessions as *mut sessions)).rbh_root ==
           0 as *mut libc::c_void as *mut session || 0 == session_alive(s) {
        return 0 as *mut session
    } else {
        s2 = sessions_RB_PREV(s);
        if s2 == 0 as *mut libc::c_void as *mut session {
            s2 = sessions_RB_MINMAX(&mut sessions as *mut sessions, 1i32)
        }
        if s2 == s { return 0 as *mut session } else { return s2 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_attach(mut s: *mut session,
                                        mut w: *mut window,
                                        mut idx: libc::c_int,
                                        mut cause: *mut *mut libc::c_char)
 -> *mut winlink {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlink_add(&mut (*s).windows as *mut winlinks, idx);
    if wl == 0 as *mut libc::c_void as *mut winlink {
        xasprintf(cause,
                  b"index in use: %d\x00" as *const u8 as *const libc::c_char,
                  idx);
        return 0 as *mut winlink
    } else {
        (*wl).session = s;
        winlink_set_window(wl, w);
        notify_session_window(b"window-linked\x00" as *const u8 as
                                  *const libc::c_char, s, w);
        session_group_synchronize_from(s);
        return wl
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_detach(mut s: *mut session,
                                        mut wl: *mut winlink) -> libc::c_int {
    if (*s).curw == wl && session_last(s) != 0i32 &&
           session_previous(s, 0i32) != 0i32 {
        session_next(s, 0i32);
    }
    (*wl).flags &= !(1i32 | 2i32 | 4i32);
    notify_session_window(b"window-unlinked\x00" as *const u8 as
                              *const libc::c_char, s, (*wl).window);
    winlink_stack_remove(&mut (*s).lastw as *mut winlink_stack, wl);
    winlink_remove(&mut (*s).windows as *mut winlinks, wl);
    session_group_synchronize_from(s);
    if (*(&mut (*s).windows as *mut winlinks)).rbh_root ==
           0 as *mut libc::c_void as *mut winlink {
        session_destroy(s,
                        (*::std::mem::transmute::<&[u8; 15],
                                                  &[libc::c_char; 15]>(b"session_detach\x00")).as_ptr());
        return 1i32
    } else { return 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn session_has(mut s: *mut session, mut w: *mut window)
 -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = (*(&mut (*w).winlinks as *mut unnamed_34)).tqh_first;
    loop  {
        if wl != 0 as *mut libc::c_void as *mut winlink {
            if (*wl).session == s {
                return 1i32
            } else { wl = (*wl).wentry.tqe_next }
        } else { return 0i32 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_is_linked(mut s: *mut session,
                                           mut w: *mut window)
 -> libc::c_int {
    let mut sg: *mut session_group = 0 as *mut session_group;
    sg = session_group_contains(s);
    if sg != 0 as *mut libc::c_void as *mut session_group {
        return ((*w).references != session_group_count(sg)) as libc::c_int
    } else {
        return ((*w).references != 1i32 as libc::c_uint) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_group_count(mut sg: *mut session_group)
 -> u_int {
    let mut s: *mut session = 0 as *mut session;
    let mut n: u_int = 0;
    n = 0i32 as u_int;
    s = (*(&mut (*sg).sessions as *mut unnamed_18)).tqh_first;
    while s != 0 as *mut libc::c_void as *mut session {
        n = n.wrapping_add(1);
        s = (*s).gentry.tqe_next
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn session_group_find(mut name: *const libc::c_char)
 -> *mut session_group {
    let mut sg: session_group =
        session_group{name: 0 as *const libc::c_char,
                      sessions:
                          unnamed_18{tqh_first: 0 as *mut session,
                                     tqh_last: 0 as *mut *mut session,},
                      entry:
                          unnamed_10{rbe_left: 0 as *mut session_group,
                                     rbe_right: 0 as *mut session_group,
                                     rbe_parent: 0 as *mut session_group,
                                     rbe_color: 0,},};
    sg.name = name;
    return session_groups_RB_FIND(&mut session_groups as *mut session_groups,
                                  &mut sg as *mut session_group);
}
#[no_mangle]
pub unsafe extern "C" fn session_group_new(mut name: *const libc::c_char)
 -> *mut session_group {
    let mut sg: *mut session_group = 0 as *mut session_group;
    sg = session_group_find(name);
    if sg != 0 as *mut libc::c_void as *mut session_group {
        return sg
    } else {
        sg =
            xcalloc(1i32 as size_t,
                    ::std::mem::size_of::<session_group>() as libc::c_ulong)
                as *mut session_group;
        (*sg).name = xstrdup(name);
        loop  {
            let ref mut fresh12 =
                (*(&mut (*sg).sessions as *mut unnamed_18)).tqh_first;
            *fresh12 = 0 as *mut session;
            let ref mut fresh13 =
                (*(&mut (*sg).sessions as *mut unnamed_18)).tqh_last;
            *fresh13 =
                &mut (*(&mut (*sg).sessions as *mut unnamed_18)).tqh_first as
                    *mut *mut session;
            if !(0 != 0i32) { break ; }
        }
        session_groups_RB_INSERT(&mut session_groups as *mut session_groups,
                                 sg);
        return sg
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_group_add(mut sg: *mut session_group,
                                           mut s: *mut session) -> () {
    let mut current_block: u64;
    if session_group_contains(s) ==
           0 as *mut libc::c_void as *mut session_group {
        current_block = 16668937799742929182;
    } else { current_block = 14916268686031723178; }
    loop  {
        match current_block {
            16668937799742929182 => {
                (*s).gentry.tqe_next = 0 as *mut session;
                (*s).gentry.tqe_prev =
                    (*(&mut (*sg).sessions as *mut unnamed_18)).tqh_last;
                let ref mut fresh14 =
                    *(*(&mut (*sg).sessions as *mut unnamed_18)).tqh_last;
                *fresh14 = s;
                let ref mut fresh15 =
                    (*(&mut (*sg).sessions as *mut unnamed_18)).tqh_last;
                *fresh15 = &mut (*s).gentry.tqe_next as *mut *mut session;
                if 0 != 0i32 {
                    current_block = 16668937799742929182;
                } else { current_block = 14916268686031723178; }
            }
            _ => { return; }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_group_synchronize_to(mut s: *mut session)
 -> () {
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut target: *mut session = 0 as *mut session;
    sg = session_group_contains(s);
    if sg == 0 as *mut libc::c_void as *mut session_group {
        return
    } else {
        target = 0 as *mut session;
        target = (*(&mut (*sg).sessions as *mut unnamed_18)).tqh_first;
        while target != 0 as *mut libc::c_void as *mut session {
            if target != s { break ; }
            target = (*target).gentry.tqe_next
        }
        if target != 0 as *mut libc::c_void as *mut session {
            session_group_synchronize1(target, s);
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn session_renumber_windows(mut s: *mut session) -> () {
    let mut current_block: u64;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wl1: *mut winlink = 0 as *mut winlink;
    let mut wl_new: *mut winlink = 0 as *mut winlink;
    let mut old_wins: winlinks = winlinks{rbh_root: 0 as *mut winlink,};
    let mut old_lastw: winlink_stack =
        winlink_stack{tqh_first: 0 as *mut winlink,
                      tqh_last: 0 as *mut *mut winlink,};
    let mut new_idx: libc::c_int = 0;
    let mut new_curw_idx: libc::c_int = 0;
    memcpy(&mut old_wins as *mut winlinks as *mut libc::c_void,
           &mut (*s).windows as *mut winlinks as *const libc::c_void,
           ::std::mem::size_of::<winlinks>() as libc::c_ulong);
    loop  {
        let ref mut fresh16 =
            (*(&mut (*s).windows as *mut winlinks)).rbh_root;
        *fresh16 = 0 as *mut winlink;
        if !(0 != 0i32) { break ; }
    }
    new_idx =
        options_get_number((*s).options,
                           b"base-index\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    new_curw_idx = 0i32;
    wl =
        winlinks_RB_MINMAX(&mut old_wins as *mut winlinks,
                           1i32.wrapping_neg());
    while wl != 0 as *mut libc::c_void as *mut winlink {
        wl_new = winlink_add(&mut (*s).windows as *mut winlinks, new_idx);
        (*wl_new).session = s;
        winlink_set_window(wl_new, (*wl).window);
        (*wl_new).flags |= (*wl).flags & (1i32 | 2i32 | 4i32);
        if wl == (*s).curw { new_curw_idx = (*wl_new).idx }
        new_idx += 1;
        wl = winlinks_RB_NEXT(wl)
    }
    memcpy(&mut old_lastw as *mut winlink_stack as *mut libc::c_void,
           &mut (*s).lastw as *mut winlink_stack as *const libc::c_void,
           ::std::mem::size_of::<winlink_stack>() as libc::c_ulong);
    loop  {
        let ref mut fresh17 =
            (*(&mut (*s).lastw as *mut winlink_stack)).tqh_first;
        *fresh17 = 0 as *mut winlink;
        let ref mut fresh18 =
            (*(&mut (*s).lastw as *mut winlink_stack)).tqh_last;
        *fresh18 =
            &mut (*(&mut (*s).lastw as *mut winlink_stack)).tqh_first as
                *mut *mut winlink;
        if !(0 != 0i32) { break ; }
    }
    wl = (*(&mut old_lastw as *mut winlink_stack)).tqh_first;
    while wl != 0 as *mut libc::c_void as *mut winlink {
        wl_new =
            winlink_find_by_window(&mut (*s).windows as *mut winlinks,
                                   (*wl).window);
        if wl_new != 0 as *mut libc::c_void as *mut winlink {
            current_block = 11650488183268122163;
        } else { current_block = 13513818773234778473; }
        loop  {
            match current_block {
                13513818773234778473 => {
                    wl = (*wl).sentry.tqe_next;
                    break ;
                }
                _ => {
                    (*wl_new).sentry.tqe_next = 0 as *mut winlink;
                    (*wl_new).sentry.tqe_prev =
                        (*(&mut (*s).lastw as *mut winlink_stack)).tqh_last;
                    let ref mut fresh19 =
                        *(*(&mut (*s).lastw as *mut winlink_stack)).tqh_last;
                    *fresh19 = wl_new;
                    let ref mut fresh20 =
                        (*(&mut (*s).lastw as *mut winlink_stack)).tqh_last;
                    *fresh20 =
                        &mut (*wl_new).sentry.tqe_next as *mut *mut winlink;
                    if 0 != 0i32 {
                        current_block = 11650488183268122163;
                    } else { current_block = 13513818773234778473; }
                }
            }
        }
    }
    (*s).curw =
        winlink_find_by_index(&mut (*s).windows as *mut winlinks,
                              new_curw_idx);
    wl =
        winlinks_RB_MINMAX(&mut old_wins as *mut winlinks,
                           1i32.wrapping_neg());
    while wl != 0 as *mut libc::c_void as *mut winlink &&
              { wl1 = winlinks_RB_NEXT(wl); 0 != 1i32 } {
        winlink_remove(&mut old_wins as *mut winlinks, wl);
        wl = wl1
    };
}

