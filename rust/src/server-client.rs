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
    pub type args_entry;
    pub type screen_titles;
    pub type hooks;
    pub type tmuxpeer;
    pub type tty_code;
    pub type tmuxproc;
    pub type event_base;
    pub type input_ctx;
    pub type bufferevent_ops;
    pub type format_tree;
    pub type evbuffer;
    pub type _IO_FILE_plus;
    pub type environ;
    pub type options;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, ...) -> libc::c_int;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
     -> libc::c_int;
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
    fn event_pending(ev: *const event, events: libc::c_short,
                     tv: *mut timeval) -> libc::c_int;
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
    fn evbuffer_new() -> *mut evbuffer;
    #[no_mangle]
    fn evbuffer_free(buf: *mut evbuffer) -> ();
    #[no_mangle]
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn evbuffer_add(buf: *mut evbuffer, data: *const libc::c_void,
                    datlen: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_add_printf(buf: *mut evbuffer, fmt: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t)
     -> *mut libc::c_uchar;
    #[no_mangle]
    fn bufferevent_write(bufev: *mut bufferevent, data: *const libc::c_void,
                         size: size_t) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn realpath(__name: *const libc::c_char, __resolved: *mut libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
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
    fn access(__name: *const libc::c_char, __type: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
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
    fn setblocking(_: libc::c_int, _: libc::c_int) -> ();
    #[no_mangle]
    fn find_home() -> *const libc::c_char;
    #[no_mangle]
    fn proc_send(_: *mut tmuxpeer, _: msgtype, _: libc::c_int,
                 _: *const libc::c_void, _: size_t) -> libc::c_int;
    #[no_mangle]
    fn proc_add_peer(_: *mut tmuxproc, _: libc::c_int,
                     _:
                         Option<unsafe extern "C" fn(_: *mut imsg,
                                                     _: *mut libc::c_void)
                                    -> ()>, _: *mut libc::c_void)
     -> *mut tmuxpeer;
    #[no_mangle]
    fn proc_remove_peer(_: *mut tmuxpeer) -> ();
    #[no_mangle]
    fn proc_kill_peer(_: *mut tmuxpeer) -> ();
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn format_create(_: *mut client, _: *mut cmdq_item, _: libc::c_int,
                     _: libc::c_int) -> *mut format_tree;
    #[no_mangle]
    fn format_free(_: *mut format_tree) -> ();
    #[no_mangle]
    fn format_expand_time(_: *mut format_tree, _: *const libc::c_char,
                          _: time_t) -> *mut libc::c_char;
    #[no_mangle]
    fn format_defaults(_: *mut format_tree, _: *mut client, _: *mut session,
                       _: *mut winlink, _: *mut window_pane) -> ();
    #[no_mangle]
    fn format_lost_client(_: *mut client) -> ();
    #[no_mangle]
    fn notify_client(_: *const libc::c_char, _: *mut client) -> ();
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
    fn environ_find(_: *mut environ, _: *const libc::c_char)
     -> *mut environ_entry;
    #[no_mangle]
    fn environ_put(_: *mut environ, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn tty_reset(_: *mut tty) -> ();
    #[no_mangle]
    fn tty_region_off(_: *mut tty) -> ();
    #[no_mangle]
    fn tty_margin_off(_: *mut tty) -> ();
    #[no_mangle]
    fn tty_cursor(_: *mut tty, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn tty_init(_: *mut tty, _: *mut client, _: libc::c_int,
                _: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tty_resize(_: *mut tty) -> ();
    #[no_mangle]
    fn tty_start_tty(_: *mut tty) -> ();
    #[no_mangle]
    fn tty_stop_tty(_: *mut tty) -> ();
    #[no_mangle]
    fn tty_set_title(_: *mut tty, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn tty_update_mode(_: *mut tty, _: libc::c_int, _: *mut screen) -> ();
    #[no_mangle]
    fn tty_open(_: *mut tty, _: *mut *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tty_close(_: *mut tty) -> ();
    #[no_mangle]
    fn tty_free(_: *mut tty) -> ();
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn cmd_find_from_session(_: *mut cmd_find_state, _: *mut session,
                             _: libc::c_int) -> ();
    #[no_mangle]
    fn cmd_find_from_mouse(_: *mut cmd_find_state, _: *mut mouse_event,
                           _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cmd_unpack_argv(_: *mut libc::c_char, _: size_t, _: libc::c_int,
                       _: *mut *mut *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn cmd_free_argv(_: libc::c_int, _: *mut *mut libc::c_char) -> ();
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn cmd_list_parse(_: libc::c_int, _: *mut *mut libc::c_char,
                      _: *const libc::c_char, _: u_int,
                      _: *mut *mut libc::c_char) -> *mut cmd_list;
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
    fn cmdq_append(_: *mut client, _: *mut cmdq_item) -> ();
    #[no_mangle]
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn key_bindings_RB_FIND(_: *mut key_bindings, _: *mut key_binding)
     -> *mut key_binding;
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    fn key_bindings_get_table(_: *const libc::c_char, _: libc::c_int)
     -> *mut key_table;
    #[no_mangle]
    fn key_bindings_unref_table(_: *mut key_table) -> ();
    #[no_mangle]
    fn key_bindings_dispatch(_: *mut key_binding, _: *mut cmdq_item,
                             _: *mut client, _: *mut mouse_event,
                             _: *mut cmd_find_state) -> ();
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    fn server_update_socket() -> ();
    #[no_mangle]
    fn server_add_accept(_: libc::c_int) -> ();
    #[no_mangle]
    fn server_redraw_client(_: *mut client) -> ();
    #[no_mangle]
    fn window_pane_tree_RB_NEXT(_: *mut window_pane) -> *mut window_pane;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn window_pane_tree_RB_MINMAX(_: *mut window_pane_tree, _: libc::c_int)
     -> *mut window_pane;
    #[no_mangle]
    fn window_pane_key(_: *mut window_pane, _: *mut client, _: *mut session,
                       _: key_code, _: *mut mouse_event) -> ();
    #[no_mangle]
    fn server_status_client(_: *mut client) -> ();
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn window_get_active_at(_: *mut window, _: u_int, _: u_int)
     -> *mut window_pane;
    #[no_mangle]
    fn status_get_window_at(_: *mut client, _: u_int) -> *mut window;
    #[no_mangle]
    fn status_at_line(_: *mut client) -> libc::c_int;
    #[no_mangle]
    fn status_prompt_key(_: *mut client, _: key_code) -> libc::c_int;
    #[no_mangle]
    fn status_message_clear(_: *mut client) -> ();
    #[no_mangle]
    fn window_pane_visible(_: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn window_pane_at_index(_: *mut window, _: u_int) -> *mut window_pane;
    #[no_mangle]
    fn window_unzoom(_: *mut window) -> libc::c_int;
    #[no_mangle]
    fn session_update_activity(_: *mut session, _: *mut timeval) -> ();
    #[no_mangle]
    fn fatal(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int) -> ();
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn recalculate_sizes() -> ();
    #[no_mangle]
    fn control_callback(_: *mut client, _: libc::c_int, _: *mut libc::c_void)
     -> ();
    #[no_mangle]
    fn server_check_unattached() -> ();
    #[no_mangle]
    fn screen_free(_: *mut screen) -> ();
    #[no_mangle]
    fn status_prompt_clear(_: *mut client) -> ();
    #[no_mangle]
    fn check_window_name(_: *mut window) -> ();
    #[no_mangle]
    fn windows_RB_NEXT(_: *mut window) -> *mut window;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    fn windows_RB_MINMAX(_: *mut windows, _: libc::c_int) -> *mut window;
    #[no_mangle]
    fn status_line_size(_: *mut session) -> u_int;
    #[no_mangle]
    fn screen_redraw_screen(_: *mut client, _: libc::c_int, _: libc::c_int,
                            _: libc::c_int) -> ();
    #[no_mangle]
    fn screen_redraw_pane(_: *mut client, _: *mut window_pane) -> ();
    #[no_mangle]
    fn screen_redraw_update(_: *mut client) -> ();
    #[no_mangle]
    static grid_default_cell: grid_cell;
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
pub const KEYC_MOUSEDOWN3_STATUS: unnamed_38 = 268435472;
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
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const KEYC_KP_STAR: unnamed_38 = 268435550;
pub const KEYC_MOUSEDOWN1_STATUS: unnamed_38 = 268435466;
pub const KEYC_MOUSEDRAGEND2_BORDER: unnamed_38 = 268435497;
pub type tcflag_t = libc::c_uint;
pub const KEYC_MOUSEDRAG1_STATUS: unnamed_38 = 268435484;
pub type unnamed = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const MOVE: unnamed_5 = 1;
pub type unnamed_0 = libc::c_uint;
pub const JOB_CLOSED: unnamed_0 = 2;
pub const CMDQ_COMMAND: cmdq_type = 0;
pub const MSG_COMMAND: msgtype = 200;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const STATUS: unnamed_42 = 2;
pub const KEYC_DOWN: unnamed_38 = 268435546;
pub const KEYC_KP_SEVEN: unnamed_38 = 268435552;
pub const KEYC_F4: unnamed_38 = 268435529;
pub const MSG_STDERR: msgtype = 211;
pub const KEYC_MOUSEDOWN3_PANE: unnamed_38 = 268435471;
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const KEYC_WHEELUP_STATUS: unnamed_38 = 268435502;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type layout_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const MSG_IDENTIFY_TERM: msgtype = 101;
pub type __time_t = libc::c_long;
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
    pub alerts_entry: unnamed_26,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_22,
    pub entry: unnamed_2,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub const MSG_RESIZE: msgtype = 208;
pub const KEYC_F9: unnamed_38 = 268435534;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
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
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct msg_stdout_data {
    pub size: ssize_t,
    pub data: [libc::c_char; 8192],
}
pub const MSG_STDIN: msgtype = 212;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_41,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const KEYC_MOUSEDOWN1_PANE: unnamed_38 = 268435465;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const MSG_IDENTIFY_CWD: msgtype = 108;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const KEYC_MOUSEDRAGEND2_STATUS: unnamed_38 = 268435496;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_8,
    pub ev_next: unnamed_36,
    pub ev_timeout_pos: unnamed_33,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_32,
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
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type unnamed_5 = libc::c_uint;
pub const KEYC_KP_FOUR: unnamed_38 = 268435556;
pub const KEYC_MOUSEUP3_STATUS: unnamed_38 = 268435481;
pub const MSG_SUSPEND: msgtype = 214;
pub const KEYC_DC: unnamed_38 = 268435539;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type uint32_t = libc::c_uint;
pub const TTY_VT102: unnamed_29 = 2;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const KEYC_WHEELUP_BORDER: unnamed_38 = 268435503;
pub const KEYC_TRIPLECLICK3_PANE: unnamed_38 = 268435522;
pub const WHEEL: unnamed_5 = 5;
pub const KEYC_UP: unnamed_38 = 268435545;
pub const KEYC_KP_SLASH: unnamed_38 = 268435549;
pub const KEYC_FOCUS_IN: unnamed_38 = 268435456;
pub type __ssize_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_16,
}
pub const KEYC_MOUSEDRAGEND3_BORDER: unnamed_38 = 268435500;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const KEYC_F8: unnamed_38 = 268435533;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub ev_signal_next: unnamed_1,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const KEYC_DOUBLECLICK1_BORDER: unnamed_38 = 268435509;
pub const KEYC_TRIPLECLICK1_PANE: unnamed_38 = 268435516;
pub const KEYC_MOUSEDRAGEND1_BORDER: unnamed_38 = 268435494;
pub const DOWN: unnamed_5 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_0,
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
    pub entry: unnamed_27,
}
pub const KEYC_F11: unnamed_38 = 268435536;
pub const MSG_IDENTIFY_DONE: msgtype = 106;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const MSG_DETACHKILL: msgtype = 202;
pub const LINE_SEL_LEFT_RIGHT: unnamed = 1;
pub const KEYC_KP_TWO: unnamed_38 = 268435560;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
}
pub const KEYC_KP_PERIOD: unnamed_38 = 268435564;
pub type cmdq_type = libc::c_uint;
pub type __u_char = libc::c_uchar;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const TRIPLE: unnamed_5 = 7;
pub const KEYC_TRIPLECLICK3_STATUS: unnamed_38 = 268435523;
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const MSG_LOCK: msgtype = 206;
pub const KEYC_MOUSEUP2_PANE: unnamed_38 = 268435477;
pub const MSG_IDENTIFY_FLAGS: msgtype = 100;
pub const KEYC_RIGHT: unnamed_38 = 268435548;
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
    pub message_log: unnamed_11,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_25,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_28,
}
pub const KEYC_WHEELUP_PANE: unnamed_38 = 268435501;
pub const KEYC_DOUBLECLICK2_PANE: unnamed_38 = 268435510;
pub const KEYC_KP_EIGHT: unnamed_38 = 268435553;
pub const MSG_UNLOCK: msgtype = 215;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct imsg_hdr {
    pub type_0: uint32_t,
    pub len: uint16_t,
    pub flags: uint16_t,
    pub peerid: uint32_t,
    pub pid: uint32_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_7,
}
pub const KEYC_KP_MINUS: unnamed_38 = 268435551;
pub const KEYC_BSPACE: unnamed_38 = 268435525;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_16 {
    offset: u_int,
    data: unnamed_20,
}
pub const KEYC_MOUSEDRAG3_BORDER: unnamed_38 = 268435491;
pub const TTY_VT320: unnamed_29 = 4;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_15,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct imsg {
    pub hdr: imsg_hdr,
    pub fd: libc::c_int,
    pub data: *mut libc::c_void,
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
pub const KEYC_F10: unnamed_38 = 268435535;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub const MSG_IDENTIFY_CLIENTPID: msgtype = 107;
pub type uint8_t = libc::c_uchar;
pub const UP: unnamed_5 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const KEYC_IC: unnamed_38 = 268435538;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
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
pub struct unnamed_20 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub const KEYC_MOUSEDRAG2_STATUS: unnamed_38 = 268435487;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub const KEYC_MOUSEDOWN2_PANE: unnamed_38 = 268435468;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const KEYC_DOUBLECLICK2_STATUS: unnamed_38 = 268435511;
pub type key_code = libc::c_ulonglong;
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
pub const DOUBLE: unnamed_5 = 6;
pub const KEYC_F1: unnamed_38 = 268435526;
pub const LINE_SEL_RIGHT_LEFT: unnamed = 2;
pub const DRAG: unnamed_5 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_40,
}
pub const KEYC_MOUSEDRAGEND3_STATUS: unnamed_38 = 268435499;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const KEYC_F5: unnamed_38 = 268435530;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const TTY_UNKNOWN: unnamed_29 = 6;
pub const KEYC_MOUSEMOVE_BORDER: unnamed_38 = 268435464;
pub const KEYC_MOUSEUP3_BORDER: unnamed_38 = 268435482;
pub const MSG_IDENTIFY_OLDCWD: msgtype = 103;
pub const KEYC_TRIPLECLICK2_BORDER: unnamed_38 = 268435521;
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const BORDER: unnamed_42 = 3;
pub type u_char = __u_char;
pub const NOWHERE: unnamed_42 = 0;
pub const MSG_WAKEUP: msgtype = 216;
pub const KEYC_MOUSEMOVE_PANE: unnamed_38 = 268435462;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct environ_entry {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub entry: unnamed_13,
}
pub const KEYC_MOUSEDRAG1_PANE: unnamed_38 = 268435483;
pub const KEYC_MOUSEDRAG3_PANE: unnamed_38 = 268435489;
pub const KEYC_END: unnamed_38 = 268435541;
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
pub const MSG_VERSION: msgtype = 12;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
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
    pub entry: unnamed_35,
    pub wentry: unnamed_6,
    pub sentry: unnamed_12,
}
pub type options_table_scope = libc::c_uint;
pub const KEYC_F2: unnamed_38 = 268435527;
pub const KEYC_MOUSEUP3_PANE: unnamed_38 = 268435480;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub type unnamed_25 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const KEYC_KP_ONE: unnamed_38 = 268435559;
pub const KEYC_PASTE_END: unnamed_38 = 268435459;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const KEYC_MOUSEDRAGEND2_PANE: unnamed_38 = 268435495;
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
    pub entry: unnamed_21,
    pub tree_entry: unnamed_39,
}
pub const NOTYPE: unnamed_5 = 0;
pub const PROMPT_COMMAND: unnamed_25 = 1;
pub const KEYC_MOUSEDRAGEND1_PANE: unnamed_38 = 268435492;
pub const KEYC_MOUSEDRAG3_STATUS: unnamed_38 = 268435490;
pub type _IO_lock_t = ();
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
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
pub const KEYC_LEFT: unnamed_38 = 268435547;
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
pub struct unnamed_28 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub const KEYC_PPAGE: unnamed_38 = 268435543;
pub const KEYC_DOUBLECLICK1_PANE: unnamed_38 = 268435507;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const PROMPT_ENTRY: unnamed_25 = 0;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const KEYC_F12: unnamed_38 = 268435537;
pub const KEYC_KP_ZERO: unnamed_38 = 268435563;
pub const KEYC_KP_THREE: unnamed_38 = 268435561;
pub const KEYC_MOUSEDRAG2_BORDER: unnamed_38 = 268435488;
pub const KEYC_MOUSEUP1_BORDER: unnamed_38 = 268435476;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
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
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const KEYC_MOUSEDOWN2_STATUS: unnamed_38 = 268435469;
pub type size_t = libc::c_ulong;
pub const MSG_SHUTDOWN: msgtype = 210;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_18,
}
pub const KEYC_TRIPLECLICK1_BORDER: unnamed_38 = 268435518;
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
    pub entry: unnamed_19,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_4,
}
pub type unnamed_29 = libc::c_uint;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub ev_io_next: unnamed_10,
    pub ev_timeout: timeval,
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
pub const KEYC_BTAB: unnamed_38 = 268435544;
pub const KEYC_FOCUS_OUT: unnamed_38 = 268435457;
pub const MSG_EXITING: msgtype = 205;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_34,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const KEYC_MOUSEDRAG2_PANE: unnamed_38 = 268435486;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct msg_stdin_data {
    pub size: ssize_t,
    pub data: [libc::c_char; 8192],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub type __u_short = libc::c_ushort;
pub const KEYC_MOUSE: unnamed_38 = 268435460;
pub type __timezone_ptr_t = *mut timezone;
pub type cmd_find_type = libc::c_uint;
pub const TTY_VT420: unnamed_29 = 5;
pub const MSG_EXEC: msgtype = 217;
pub const KEYC_PASTE_START: unnamed_38 = 268435458;
pub const KEYC_DOUBLECLICK1_STATUS: unnamed_38 = 268435508;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub const MSG_READY: msgtype = 207;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub const MSG_IDENTIFY_ENVIRON: msgtype = 105;
pub const KEYC_DRAGGING: unnamed_38 = 268435461;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_32 {
    ev_io: unnamed_30,
    ev_signal: unnamed_9,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_33 {
    ev_next_with_common_timeout: unnamed_17,
    min_heap_idx: libc::c_int,
}
pub const MSG_EXIT: msgtype = 203;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub const KEYC_KP_ENTER: unnamed_38 = 268435562;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const KEYC_MOUSEDRAG1_BORDER: unnamed_38 = 268435485;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const MSG_EXITED: msgtype = 204;
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const KEYC_KP_PLUS: unnamed_38 = 268435555;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const MSG_IDENTIFY_TTYNAME: msgtype = 102;
pub const LINE_SEL_NONE: unnamed = 0;
pub const KEYC_TRIPLECLICK2_PANE: unnamed_38 = 268435519;
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
pub struct unnamed_37 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const KEYC_TRIPLECLICK2_STATUS: unnamed_38 = 268435520;
pub const KEYC_HOME: unnamed_38 = 268435540;
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
    pub entry: unnamed_37,
}
pub const KEYC_MOUSEDOWN3_BORDER: unnamed_38 = 268435473;
pub const KEYC_MOUSEDOWN1_BORDER: unnamed_38 = 268435467;
pub const KEYC_DOUBLECLICK3_BORDER: unnamed_38 = 268435515;
pub const KEYC_KP_NINE: unnamed_38 = 268435554;
pub const KEYC_MOUSEUP1_PANE: unnamed_38 = 268435474;
pub type speed_t = libc::c_uint;
pub type unnamed_38 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const KEYC_MOUSEUP2_STATUS: unnamed_38 = 268435478;
pub const MSG_DETACH: msgtype = 201;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub type bitstr_t = libc::c_uchar;
pub const KEYC_F7: unnamed_38 = 268435532;
pub const KEYC_MOUSEDRAGEND3_PANE: unnamed_38 = 268435498;
pub const JOB_RUNNING: unnamed_0 = 0;
pub const KEYC_DOUBLECLICK3_PANE: unnamed_38 = 268435513;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const KEYC_F6: unnamed_38 = 268435531;
pub const KEYC_MOUSEMOVE_STATUS: unnamed_38 = 268435463;
pub const KEYC_KP_FIVE: unnamed_38 = 268435557;
pub type u_int = __u_int;
pub const KEYC_WHEELDOWN_PANE: unnamed_38 = 268435504;
pub const MSG_SHELL: msgtype = 209;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const JOB_DEAD: unnamed_0 = 1;
pub const KEYC_MOUSEDRAGEND1_STATUS: unnamed_38 = 268435493;
pub const KEYC_MOUSEUP1_STATUS: unnamed_38 = 268435475;
pub const KEYC_KP_SIX: unnamed_38 = 268435558;
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_40 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const KEYC_F3: unnamed_38 = 268435528;
pub const MSG_IDENTIFY_STDIN: msgtype = 104;
pub const KEYC_TRIPLECLICK1_STATUS: unnamed_38 = 268435517;
pub const KEYC_MOUSEDOWN2_BORDER: unnamed_38 = 268435470;
pub const MSG_STDOUT: msgtype = 213;
pub const TTY_VT100: unnamed_29 = 0;
pub const TTY_VT220: unnamed_29 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const PANE: unnamed_42 = 1;
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
    pub gentry: unnamed_24,
    pub entry: unnamed_31,
}
pub const KEYC_MOUSEUP2_BORDER: unnamed_38 = 268435479;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_41 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const KEYC_DOUBLECLICK2_BORDER: unnamed_38 = 268435512;
pub type ssize_t = __ssize_t;
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
    pub term_type: unnamed_29,
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
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub type options_table_type = libc::c_uint;
pub type msgtype = libc::c_uint;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct msg_command_data {
    pub argc: libc::c_int,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const TTY_VT101: unnamed_29 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const KEYC_WHEELDOWN_BORDER: unnamed_38 = 268435506;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct msg_stderr_data {
    pub size: ssize_t,
    pub data: [libc::c_char; 8192],
}
pub const KEYC_WHEELDOWN_STATUS: unnamed_38 = 268435505;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
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
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_3,
    pub entry: unnamed_23,
}
pub const KEYC_DOUBLECLICK3_STATUS: unnamed_38 = 268435514;
pub const KEYC_TRIPLECLICK3_BORDER: unnamed_38 = 268435524;
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
pub type unnamed_42 = libc::c_uint;
pub const KEYC_NPAGE: unnamed_38 = 268435542;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn server_client_how_many() -> u_int {
    let mut c: *mut client = 0 as *mut client;
    let mut n: u_int = 0;
    n = 0i32 as u_int;
    c = (*(&mut clients as *mut clients)).tqh_first;
    while c != 0 as *mut libc::c_void as *mut client {
        if (*c).session != 0 as *mut libc::c_void as *mut session &&
               0 != !(*c).flags & 4096i32 {
            n = n.wrapping_add(1)
        }
        c = (*c).entry.tqe_next
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn server_client_set_identify(mut c: *mut client,
                                                    mut delay: u_int) -> () {
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    tv.tv_sec = delay.wrapping_div(1000i32 as libc::c_uint) as __time_t;
    tv.tv_usec =
        delay.wrapping_rem(1000i32 as libc::c_uint) as libc::c_long * 1000i64;
    if 0 != event_initialized(&mut (*c).identify_timer as *mut event) {
        event_del(&mut (*c).identify_timer as *mut event);
    }
    event_set(&mut (*c).identify_timer as *mut event, 1i32.wrapping_neg(),
              0i32 as libc::c_short, Some(server_client_callback_identify),
              c as *mut libc::c_void);
    if delay != 0i32 as libc::c_uint {
        event_add(&mut (*c).identify_timer as *mut event,
                  &mut tv as *mut timeval);
    }
    (*c).flags |= 256i32;
    (*c).tty.flags |= 2i32 | 1i32;
    server_redraw_client(c);
}
unsafe extern "C" fn server_client_callback_identify(mut fd: libc::c_int,
                                                     mut events:
                                                         libc::c_short,
                                                     mut data:
                                                         *mut libc::c_void)
 -> () {
    server_client_clear_identify(data as *mut client, 0 as *mut window_pane);
}
#[no_mangle]
pub unsafe extern "C" fn server_client_clear_identify(mut c: *mut client,
                                                      mut wp:
                                                          *mut window_pane)
 -> () {
    if 0 != !(*c).flags & 256i32 {
        return
    } else {
        (*c).flags &= !256i32;
        if (*c).identify_callback !=
               ::std::mem::transmute::<*mut libc::c_void,
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut client,
                                                                   _:
                                                                       *mut window_pane)
                                                  ->
                                                      ()>>(0 as
                                                               *mut libc::c_void)
           {
            (*c).identify_callback.expect("non-null function pointer")(c, wp);
        }
        (*c).tty.flags &= !(2i32 | 1i32);
        server_redraw_client(c);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_client_set_key_table(mut c: *mut client,
                                                     mut name:
                                                         *const libc::c_char)
 -> () {
    if name == 0 as *mut libc::c_void as *const libc::c_char {
        name = server_client_get_key_table(c)
    }
    key_bindings_unref_table((*c).keytable);
    (*c).keytable = key_bindings_get_table(name, 1i32);
    (*(*c).keytable).references = (*(*c).keytable).references.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn server_client_get_key_table(mut c: *mut client)
 -> *const libc::c_char {
    let mut s: *mut session = (*c).session;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if s == 0 as *mut libc::c_void as *mut session {
        return b"root\x00" as *const u8 as *const libc::c_char
    } else {
        name =
            options_get_string((*s).options,
                               b"key-table\x00" as *const u8 as
                                   *const libc::c_char);
        if *name as libc::c_int == 0 {
            return b"root\x00" as *const u8 as *const libc::c_char
        } else { return name }
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_client_check_nested(mut c: *mut client)
 -> libc::c_int {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    envent =
        environ_find((*c).environ,
                     b"TMUX\x00" as *const u8 as *const libc::c_char);
    if envent == 0 as *mut libc::c_void as *mut environ_entry ||
           *(*envent).value as libc::c_int == 0 {
        return 0i32
    } else {
        wp =
            window_pane_tree_RB_MINMAX(&mut all_window_panes as
                                           *mut window_pane_tree,
                                       1i32.wrapping_neg());
        loop  {
            if wp != 0 as *mut libc::c_void as *mut window_pane {
                if strcmp((*wp).tty.as_mut_ptr(), (*c).ttyname) == 0i32 {
                    return 1i32
                } else { wp = window_pane_tree_RB_NEXT(wp) }
            } else { return 0i32 }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_client_handle_key(mut c: *mut client,
                                                  mut key: key_code) -> () {
    let mut current_block: u64;
    let mut m: *mut mouse_event = &mut (*c).tty.mouse as *mut mouse_event;
    let mut s: *mut session = (*c).session;
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut first: *mut key_table = 0 as *mut key_table;
    let mut bd_find: key_binding =
        key_binding{key: 0,
                    cmdlist: 0 as *mut cmd_list,
                    flags: 0,
                    entry:
                        unnamed_7{rbe_left: 0 as *mut key_binding,
                                  rbe_right: 0 as *mut key_binding,
                                  rbe_parent: 0 as *mut key_binding,
                                  rbe_color: 0,},};
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut xtimeout: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut fs: cmd_find_state =
        cmd_find_state{flags: 0,
                       current:
                           0 as *const cmd_find_state as *mut cmd_find_state,
                       s: 0 as *const session as *mut session,
                       wl: 0 as *const winlink as *mut winlink,
                       w: 0 as *const window as *mut window,
                       wp: 0 as *const window_pane as *mut window_pane,
                       idx: 0,};
    let mut key0: key_code = 0;
    if s == 0 as *mut libc::c_void as *mut session ||
           (*c).flags & (512i32 | 64i32) != 0i32 {
        return
    } else {
        w = (*(*s).curw).window;
        if gettimeofday(&mut (*c).activity_time as *mut timeval,
                        0 as *mut timezone) != 0i32 {
            fatal(b"gettimeofday failed\x00" as *const u8 as
                      *const libc::c_char);
        } else {
            session_update_activity(s,
                                    &mut (*c).activity_time as *mut timeval);
            if 0 != (*c).flags & 256i32 && key >= 48 as libc::c_ulonglong &&
                   key <= 57 as libc::c_ulonglong {
                if 0 != (*c).flags & 2048i32 {
                    return
                } else {
                    window_unzoom(w);
                    wp =
                        window_pane_at_index(w,
                                             key.wrapping_sub(48 as
                                                                  libc::c_ulonglong)
                                                 as u_int);
                    if wp != 0 as *mut libc::c_void as *mut window_pane &&
                           0 == window_pane_visible(wp) {
                        wp = 0 as *mut window_pane
                    }
                    server_client_clear_identify(c, wp);
                    return
                }
            } else {
                if 0 == (*c).flags & 2048i32 {
                    status_message_clear(c);
                    server_client_clear_identify(c, 0 as *mut window_pane);
                }
                if (*c).prompt_string !=
                       0 as *mut libc::c_void as *mut libc::c_char {
                    if 0 != (*c).flags & 2048i32 {
                        return
                    } else if status_prompt_key(c, key) == 0i32 { return }
                }
                (*m).valid = 0i32;
                if key == KEYC_MOUSE as libc::c_int as libc::c_ulonglong {
                    if 0 != (*c).flags & 2048i32 {
                        return
                    } else {
                        key = server_client_check_mouse(c);
                        if key == 281466386776064u64 {
                            return
                        } else {
                            (*m).valid = 1i32;
                            (*m).key = key;
                            if key ==
                                   KEYC_DRAGGING as libc::c_int as
                                       libc::c_ulonglong {
                                (*c).tty.mouse_drag_update.expect("non-null function pointer")(c,
                                                                                               m);
                                return
                            }
                        }
                    }
                } else { (*m).valid = 0i32 }
                if 0 !=
                       !(key &
                             !(35184372088832u64 | 70368744177664u64 |
                                   140737488355328u64 | 281474976710656u64) >=
                             KEYC_MOUSE as libc::c_int as libc::c_ulonglong &&
                             key &
                                 !(35184372088832u64 | 70368744177664u64 |
                                       140737488355328u64 |
                                       281474976710656u64) <
                                 KEYC_BSPACE as libc::c_int as
                                     libc::c_ulonglong) as libc::c_int ||
                       cmd_find_from_mouse(&mut fs as *mut cmd_find_state, m,
                                           0i32) != 0i32 {
                    cmd_find_from_session(&mut fs as *mut cmd_find_state, s,
                                          0i32);
                }
                wp = fs.wp;
                if !(key &
                         !(35184372088832u64 | 70368744177664u64 |
                               140737488355328u64 | 281474976710656u64) >=
                         KEYC_MOUSE as libc::c_int as libc::c_ulonglong &&
                         key &
                             !(35184372088832u64 | 70368744177664u64 |
                                   140737488355328u64 | 281474976710656u64) <
                             KEYC_BSPACE as libc::c_int as libc::c_ulonglong
                         &&
                         0 ==
                             options_get_number((*s).options,
                                                b"mouse\x00" as *const u8 as
                                                    *const libc::c_char)) {
                    if !(0 !=
                             !(key &
                                   !(35184372088832u64 | 70368744177664u64 |
                                         140737488355328u64 |
                                         281474976710656u64) >=
                                   KEYC_MOUSE as libc::c_int as
                                       libc::c_ulonglong &&
                                   key &
                                       !(35184372088832u64 | 70368744177664u64
                                             | 140737488355328u64 |
                                             281474976710656u64) <
                                       KEYC_BSPACE as libc::c_int as
                                           libc::c_ulonglong) as libc::c_int
                             && 0 != server_client_assume_paste(s)) {
                        if 0 !=
                               server_client_is_default_key_table(c,
                                                                  (*c).keytable)
                               &&
                               wp !=
                                   0 as *mut libc::c_void as *mut window_pane
                               &&
                               (*wp).mode !=
                                   0 as *mut libc::c_void as
                                       *const window_mode &&
                               (*(*wp).mode).key_table !=
                                   ::std::mem::transmute::<*mut libc::c_void,
                                                           Option<unsafe extern "C" fn(_:
                                                                                           *mut window_pane)
                                                                      ->
                                                                          *const libc::c_char>>(0
                                                                                                    as
                                                                                                    *mut libc::c_void)
                           {
                            table =
                                key_bindings_get_table((*(*wp).mode).key_table.expect("non-null function pointer")(wp),
                                                       1i32)
                        } else { table = (*c).keytable }
                        first = table;
                        loop  {
                            key0 = key & !281474976710656u64;
                            if (key0 ==
                                    options_get_number((*s).options,
                                                       b"prefix\x00" as
                                                           *const u8 as
                                                           *const libc::c_char)
                                        as key_code ||
                                    key0 ==
                                        options_get_number((*s).options,
                                                           b"prefix2\x00" as
                                                               *const u8 as
                                                               *const libc::c_char)
                                            as key_code) &&
                                   strcmp((*table).name,
                                          b"prefix\x00" as *const u8 as
                                              *const libc::c_char) != 0i32 {
                                server_client_set_key_table(c,
                                                            b"prefix\x00" as
                                                                *const u8 as
                                                                *const libc::c_char);
                                server_status_client(c);
                                return
                            } else {
                                flags = (*c).flags;
                                if wp ==
                                       0 as *mut libc::c_void as
                                           *mut window_pane {
                                    log_debug(b"key table %s (no pane)\x00" as
                                                  *const u8 as
                                                  *const libc::c_char,
                                              (*table).name);
                                } else {
                                    log_debug(b"key table %s (pane %%%u)\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              (*table).name, (*wp).id);
                                }
                                if 0 != (*c).flags & 32i32 {
                                    log_debug(b"currently repeating\x00" as
                                                  *const u8 as
                                                  *const libc::c_char);
                                }
                                bd_find.key = key0;
                                bd =
                                    key_bindings_RB_FIND(&mut (*table).key_bindings
                                                             as
                                                             *mut key_bindings,
                                                         &mut bd_find as
                                                             *mut key_binding);
                                if bd !=
                                       0 as *mut libc::c_void as
                                           *mut key_binding {
                                    if 0 != (*c).flags & 32i32 &&
                                           0 != !(*bd).flags & 1i32 {
                                        server_client_set_key_table(c,
                                                                    0 as
                                                                        *const libc::c_char);
                                        (*c).flags &= !32i32;
                                        server_status_client(c);
                                        table = (*c).keytable
                                    } else {
                                        log_debug(b"found in key table %s\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  (*table).name);
                                        (*table).references =
                                            (*table).references.wrapping_add(1);
                                        xtimeout =
                                            options_get_number((*s).options,
                                                               b"repeat-time\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char)
                                                as libc::c_int;
                                        if xtimeout != 0i32 &&
                                               0 != (*bd).flags & 1i32 {
                                            current_block =
                                                9828876828309294594;
                                            break ;
                                        } else {
                                            current_block =
                                                7056779235015430508;
                                            break ;
                                        }
                                    }
                                } else {
                                    log_debug(b"not found in key table %s\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              (*table).name);
                                    if 0 ==
                                           server_client_is_default_key_table(c,
                                                                              table)
                                           || 0 != (*c).flags & 32i32 {
                                        server_client_set_key_table(c,
                                                                    0 as
                                                                        *const libc::c_char);
                                        (*c).flags &= !32i32;
                                        server_status_client(c);
                                        table = (*c).keytable
                                    } else if first != table &&
                                                  0 != !flags & 32i32 {
                                        current_block = 11459959175219260272;
                                        break ;
                                    } else {
                                        current_block = 18048823125822211551;
                                        break ;
                                    }
                                }
                            }
                        }
                        match current_block {
                            18048823125822211551 => { }
                            _ => {
                                match current_block {
                                    7056779235015430508 => {
                                        (*c).flags &= !32i32;
                                        server_client_set_key_table(c,
                                                                    0 as
                                                                        *const libc::c_char);
                                    }
                                    9828876828309294594 => {
                                        (*c).flags |= 32i32;
                                        tv.tv_sec =
                                            (xtimeout / 1000i32) as __time_t;
                                        tv.tv_usec =
                                            (xtimeout % 1000i32) as
                                                libc::c_long * 1000i64;
                                        event_del(&mut (*c).repeat_timer as
                                                      *mut event);
                                        event_add(&mut (*c).repeat_timer as
                                                      *mut event,
                                                  &mut tv as *mut timeval);
                                    }
                                    _ => {
                                        server_client_set_key_table(c,
                                                                    0 as
                                                                        *const libc::c_char);
                                        server_status_client(c);
                                        return
                                    }
                                }
                                server_status_client(c);
                                key_bindings_dispatch(bd, 0 as *mut cmdq_item,
                                                      c, m,
                                                      &mut fs as
                                                          *mut cmd_find_state);
                                key_bindings_unref_table(table);
                                return
                            }
                        }
                    }
                }
                if 0 != (*c).flags & 2048i32 {
                    return
                } else {
                    if wp != 0 as *mut libc::c_void as *mut window_pane {
                        window_pane_key(wp, c, s, key, m);
                    }
                    return;
                }
            }
        }
    };
}
unsafe extern "C" fn server_client_is_default_key_table(mut c: *mut client,
                                                        mut table:
                                                            *mut key_table)
 -> libc::c_int {
    return (strcmp((*table).name, server_client_get_key_table(c)) == 0i32) as
               libc::c_int;
}
unsafe extern "C" fn server_client_assume_paste(mut s: *mut session)
 -> libc::c_int {
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut t: libc::c_int = 0;
    t =
        options_get_number((*s).options,
                           b"assume-paste-time\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    if t == 0i32 {
        return 0i32
    } else {
        loop  {
            (*(&mut tv as *mut timeval)).tv_sec =
                (*(&mut (*s).activity_time as *mut timeval)).tv_sec -
                    (*(&mut (*s).last_activity_time as *mut timeval)).tv_sec;
            (*(&mut tv as *mut timeval)).tv_usec =
                (*(&mut (*s).activity_time as *mut timeval)).tv_usec -
                    (*(&mut (*s).last_activity_time as *mut timeval)).tv_usec;
            if (*(&mut tv as *mut timeval)).tv_usec < 0i32 as libc::c_long {
                let ref mut fresh0 = (*(&mut tv as *mut timeval)).tv_sec;
                *fresh0 -= 1;
                let ref mut fresh1 = (*(&mut tv as *mut timeval)).tv_usec;
                *fresh1 += 1000000i32 as libc::c_long
            }
            if !(0 != 0i32) { break ; }
        }
        if tv.tv_sec == 0i32 as libc::c_long &&
               tv.tv_usec < (t * 1000i32) as libc::c_long {
            log_debug(b"session %s pasting (flag %d)\x00" as *const u8 as
                          *const libc::c_char, (*s).name,
                      !(0 == (*s).flags & 2i32) as libc::c_int);
            if 0 != (*s).flags & 2i32 {
                return 1i32
            } else { (*s).flags |= 2i32; return 0i32 }
        } else {
            log_debug(b"session %s not pasting\x00" as *const u8 as
                          *const libc::c_char, (*s).name);
            (*s).flags &= !2i32;
            return 0i32
        }
    };
}
unsafe extern "C" fn server_client_check_mouse(mut c: *mut client)
 -> key_code {
    let mut current_block: u64;
    let mut s: *mut session = (*c).session;
    let mut m: *mut mouse_event = &mut (*c).tty.mouse as *mut mouse_event;
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut b: u_int = 0;
    let mut flag: libc::c_int = 0;
    let mut key: key_code = 0;
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut type_0: unnamed_5 = NOTYPE;
    let mut where_0: unnamed_42 = NOWHERE;
    type_0 = NOTYPE;
    where_0 = NOWHERE;
    log_debug(b"mouse %02x at %u,%u (last %u,%u) (%d)\x00" as *const u8 as
                  *const libc::c_char, (*m).b, (*m).x, (*m).y, (*m).lx,
              (*m).ly, (*c).tty.mouse_drag_flag);
    if (*m).sgr_type != 32 as libc::c_uint &&
           0 != (*m).sgr_b & 32i32 as libc::c_uint &&
           (*m).sgr_b & 3i32 as libc::c_uint == 3i32 as libc::c_uint ||
           (*m).sgr_type == 32 as libc::c_uint &&
               0 != (*m).b & 32i32 as libc::c_uint &&
               (*m).b & 3i32 as libc::c_uint == 3i32 as libc::c_uint &&
               (*m).lb & 3i32 as libc::c_uint == 3i32 as libc::c_uint {
        type_0 = MOVE;
        x = (*m).x;
        y = (*m).y;
        b = 0i32 as u_int;
        log_debug(b"move at %u,%u\x00" as *const u8 as *const libc::c_char, x,
                  y);
    } else if 0 != (*m).b & 32i32 as libc::c_uint {
        type_0 = DRAG;
        if 0 != (*c).tty.mouse_drag_flag {
            x = (*m).x;
            y = (*m).y;
            b = (*m).b;
            log_debug(b"drag update at %u,%u\x00" as *const u8 as
                          *const libc::c_char, x, y);
        } else {
            x = (*m).lx;
            y = (*m).ly;
            b = (*m).lb;
            log_debug(b"drag start at %u,%u\x00" as *const u8 as
                          *const libc::c_char, x, y);
        }
    } else if 0 != (*m).b & 64i32 as libc::c_uint {
        type_0 = WHEEL;
        x = (*m).x;
        y = (*m).y;
        b = (*m).b;
        log_debug(b"wheel at %u,%u\x00" as *const u8 as *const libc::c_char,
                  x, y);
    } else if (*m).b & 3i32 as libc::c_uint == 3i32 as libc::c_uint {
        type_0 = UP;
        x = (*m).x;
        y = (*m).y;
        b = (*m).lb;
        log_debug(b"up at %u,%u\x00" as *const u8 as *const libc::c_char, x,
                  y);
    } else {
        if 0 != (*c).flags & 1048576i32 {
            event_del(&mut (*c).click_timer as *mut event);
            (*c).flags &= !1048576i32;
            if (*m).b == (*c).click_button {
                type_0 = DOUBLE;
                x = (*m).x;
                y = (*m).y;
                b = (*m).b;
                log_debug(b"double-click at %u,%u\x00" as *const u8 as
                              *const libc::c_char, x, y);
                flag = 2097152i32;
                current_block = 10312677879954137429;
            } else { current_block = 12209867499936983673; }
        } else if 0 != (*c).flags & 2097152i32 {
            event_del(&mut (*c).click_timer as *mut event);
            (*c).flags &= !2097152i32;
            if (*m).b == (*c).click_button {
                type_0 = TRIPLE;
                x = (*m).x;
                y = (*m).y;
                b = (*m).b;
                log_debug(b"triple-click at %u,%u\x00" as *const u8 as
                              *const libc::c_char, x, y);
                current_block = 811591568610607247;
            } else { current_block = 12209867499936983673; }
        } else { current_block = 12209867499936983673; }
        match current_block {
            811591568610607247 => { }
            _ => {
                match current_block {
                    12209867499936983673 => {
                        type_0 = DOWN;
                        x = (*m).x;
                        y = (*m).y;
                        b = (*m).b;
                        log_debug(b"down at %u,%u\x00" as *const u8 as
                                      *const libc::c_char, x, y);
                        flag = 1048576i32
                    }
                    _ => { }
                }
                if 300i32 != 0i32 {
                    (*c).flags |= flag;
                    (*c).click_button = (*m).b;
                    tv.tv_sec = (300i32 / 1000i32) as __time_t;
                    tv.tv_usec = (300i32 % 1000i32) as libc::c_long * 1000i64;
                    event_del(&mut (*c).click_timer as *mut event);
                    event_add(&mut (*c).click_timer as *mut event,
                              &mut tv as *mut timeval);
                }
            }
        }
    }
    if type_0 as libc::c_uint == NOTYPE as libc::c_int as libc::c_uint {
        return 281466386776064u64
    } else {
        (*m).s = (*s).id as libc::c_int;
        (*m).statusat = status_at_line(c);
        if (*m).statusat != 1i32.wrapping_neg() && y == (*m).statusat as u_int
           {
            w = status_get_window_at(c, x);
            if w == 0 as *mut libc::c_void as *mut window {
                return 281466386776064u64
            } else { (*m).w = (*w).id as libc::c_int; where_0 = STATUS }
        } else { (*m).w = 1i32.wrapping_neg() }
        if where_0 as libc::c_uint == NOWHERE as libc::c_int as libc::c_uint {
            if (*m).statusat == 0i32 && y > 0i32 as libc::c_uint {
                y = y.wrapping_sub(1)
            } else if (*m).statusat > 0i32 && y >= (*m).statusat as u_int {
                y = ((*m).statusat - 1i32) as u_int
            }
            wp =
                (*(&mut (*(*(*s).curw).window).panes as
                       *mut window_panes)).tqh_first;
            while wp != 0 as *mut libc::c_void as *mut window_pane {
                if (*wp).xoff.wrapping_add((*wp).sx) == x &&
                       (*wp).yoff <= (1i32 as libc::c_uint).wrapping_add(y) &&
                       (*wp).yoff.wrapping_add((*wp).sy) >= y ||
                       (*wp).yoff.wrapping_add((*wp).sy) == y &&
                           (*wp).xoff <=
                               (1i32 as libc::c_uint).wrapping_add(x) &&
                           (*wp).xoff.wrapping_add((*wp).sx) >= x {
                    break ;
                }
                wp = (*wp).entry.tqe_next
            }
            if wp != 0 as *mut libc::c_void as *mut window_pane {
                where_0 = BORDER
            } else {
                wp = window_get_active_at((*(*s).curw).window, x, y);
                if wp != 0 as *mut libc::c_void as *mut window_pane {
                    where_0 = PANE;
                    log_debug(b"mouse at %u,%u is on pane %%%u\x00" as
                                  *const u8 as *const libc::c_char, x, y,
                              (*wp).id);
                }
            }
            if where_0 as libc::c_uint ==
                   NOWHERE as libc::c_int as libc::c_uint {
                return 281466386776064u64
            } else {
                (*m).wp = (*wp).id as libc::c_int;
                (*m).w = (*(*wp).window).id as libc::c_int
            }
        } else { (*m).wp = 1i32.wrapping_neg() }
        if type_0 as libc::c_uint != DRAG as libc::c_int as libc::c_uint &&
               type_0 as libc::c_uint != WHEEL as libc::c_int as libc::c_uint
               && 0 != (*c).tty.mouse_drag_flag {
            if (*c).tty.mouse_drag_release !=
                   ::std::mem::transmute::<*mut libc::c_void,
                                           Option<unsafe extern "C" fn(_:
                                                                           *mut client,
                                                                       _:
                                                                           *mut mouse_event)
                                                      ->
                                                          ()>>(0 as
                                                                   *mut libc::c_void)
               {
                (*c).tty.mouse_drag_release.expect("non-null function pointer")(c,
                                                                                m);
            }
            (*c).tty.mouse_drag_update = None;
            (*c).tty.mouse_drag_release = None;
            match (*c).tty.mouse_drag_flag {
                1 => {
                    if where_0 as libc::c_uint ==
                           PANE as libc::c_int as libc::c_uint {
                        key =
                            KEYC_MOUSEDRAGEND1_PANE as libc::c_int as key_code
                    }
                    if where_0 as libc::c_uint ==
                           STATUS as libc::c_int as libc::c_uint {
                        key =
                            KEYC_MOUSEDRAGEND1_STATUS as libc::c_int as
                                key_code
                    }
                    if where_0 as libc::c_uint ==
                           BORDER as libc::c_int as libc::c_uint {
                        key =
                            KEYC_MOUSEDRAGEND1_BORDER as libc::c_int as
                                key_code
                    }
                }
                2 => {
                    if where_0 as libc::c_uint ==
                           PANE as libc::c_int as libc::c_uint {
                        key =
                            KEYC_MOUSEDRAGEND2_PANE as libc::c_int as key_code
                    }
                    if where_0 as libc::c_uint ==
                           STATUS as libc::c_int as libc::c_uint {
                        key =
                            KEYC_MOUSEDRAGEND2_STATUS as libc::c_int as
                                key_code
                    }
                    if where_0 as libc::c_uint ==
                           BORDER as libc::c_int as libc::c_uint {
                        key =
                            KEYC_MOUSEDRAGEND2_BORDER as libc::c_int as
                                key_code
                    }
                }
                3 => {
                    if where_0 as libc::c_uint ==
                           PANE as libc::c_int as libc::c_uint {
                        key =
                            KEYC_MOUSEDRAGEND3_PANE as libc::c_int as key_code
                    }
                    if where_0 as libc::c_uint ==
                           STATUS as libc::c_int as libc::c_uint {
                        key =
                            KEYC_MOUSEDRAGEND3_STATUS as libc::c_int as
                                key_code
                    }
                    if where_0 as libc::c_uint ==
                           BORDER as libc::c_int as libc::c_uint {
                        key =
                            KEYC_MOUSEDRAGEND3_BORDER as libc::c_int as
                                key_code
                    }
                }
                _ => { key = KEYC_MOUSE as libc::c_int as key_code }
            }
            (*c).tty.mouse_drag_flag = 0i32;
            return key
        } else {
            key = 281466386776064u64;
            match type_0 as libc::c_uint {
                1 => {
                    if where_0 as libc::c_uint ==
                           PANE as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEMOVE_PANE as libc::c_int as key_code
                    }
                    if where_0 as libc::c_uint ==
                           STATUS as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEMOVE_STATUS as libc::c_int as key_code
                    }
                    if where_0 as libc::c_uint ==
                           BORDER as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEMOVE_BORDER as libc::c_int as key_code
                    }
                }
                4 => {
                    if (*c).tty.mouse_drag_update !=
                           ::std::mem::transmute::<*mut libc::c_void,
                                                   Option<unsafe extern "C" fn(_:
                                                                                   *mut client,
                                                                               _:
                                                                                   *mut mouse_event)
                                                              ->
                                                                  ()>>(0 as
                                                                           *mut libc::c_void)
                       {
                        key = KEYC_DRAGGING as libc::c_int as key_code
                    } else {
                        match b & 3i32 as libc::c_uint {
                            0 => {
                                current_block = 1741327384316483099;
                                match current_block {
                                    17579185928145363449 => {
                                        if where_0 as libc::c_uint ==
                                               PANE as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG2_PANE as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               STATUS as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG2_STATUS as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               BORDER as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG2_BORDER as
                                                    libc::c_int as key_code
                                        }
                                    }
                                    15701127765566801219 => {
                                        if where_0 as libc::c_uint ==
                                               PANE as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG3_PANE as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               STATUS as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG3_STATUS as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               BORDER as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG3_BORDER as
                                                    libc::c_int as key_code
                                        }
                                    }
                                    _ => {
                                        if where_0 as libc::c_uint ==
                                               PANE as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG1_PANE as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               STATUS as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG1_STATUS as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               BORDER as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG1_BORDER as
                                                    libc::c_int as key_code
                                        }
                                    }
                                }
                            }
                            1 => {
                                current_block = 17579185928145363449;
                                match current_block {
                                    17579185928145363449 => {
                                        if where_0 as libc::c_uint ==
                                               PANE as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG2_PANE as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               STATUS as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG2_STATUS as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               BORDER as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG2_BORDER as
                                                    libc::c_int as key_code
                                        }
                                    }
                                    15701127765566801219 => {
                                        if where_0 as libc::c_uint ==
                                               PANE as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG3_PANE as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               STATUS as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG3_STATUS as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               BORDER as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG3_BORDER as
                                                    libc::c_int as key_code
                                        }
                                    }
                                    _ => {
                                        if where_0 as libc::c_uint ==
                                               PANE as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG1_PANE as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               STATUS as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG1_STATUS as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               BORDER as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG1_BORDER as
                                                    libc::c_int as key_code
                                        }
                                    }
                                }
                            }
                            2 => {
                                current_block = 15701127765566801219;
                                match current_block {
                                    17579185928145363449 => {
                                        if where_0 as libc::c_uint ==
                                               PANE as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG2_PANE as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               STATUS as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG2_STATUS as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               BORDER as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG2_BORDER as
                                                    libc::c_int as key_code
                                        }
                                    }
                                    15701127765566801219 => {
                                        if where_0 as libc::c_uint ==
                                               PANE as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG3_PANE as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               STATUS as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG3_STATUS as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               BORDER as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG3_BORDER as
                                                    libc::c_int as key_code
                                        }
                                    }
                                    _ => {
                                        if where_0 as libc::c_uint ==
                                               PANE as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG1_PANE as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               STATUS as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG1_STATUS as
                                                    libc::c_int as key_code
                                        }
                                        if where_0 as libc::c_uint ==
                                               BORDER as libc::c_int as
                                                   libc::c_uint {
                                            key =
                                                KEYC_MOUSEDRAG1_BORDER as
                                                    libc::c_int as key_code
                                        }
                                    }
                                }
                            }
                            _ => { }
                        }
                    }
                    (*c).tty.mouse_drag_flag =
                        (b &
                             3i32 as
                                 libc::c_uint).wrapping_add(1i32 as
                                                                libc::c_uint)
                            as libc::c_int
                }
                5 => {
                    if b & 3i32 as libc::c_uint == 0i32 as libc::c_uint {
                        if where_0 as libc::c_uint ==
                               PANE as libc::c_int as libc::c_uint {
                            key = KEYC_WHEELUP_PANE as libc::c_int as key_code
                        }
                        if where_0 as libc::c_uint ==
                               STATUS as libc::c_int as libc::c_uint {
                            key =
                                KEYC_WHEELUP_STATUS as libc::c_int as key_code
                        }
                        if where_0 as libc::c_uint ==
                               BORDER as libc::c_int as libc::c_uint {
                            key =
                                KEYC_WHEELUP_BORDER as libc::c_int as key_code
                        }
                    } else {
                        if where_0 as libc::c_uint ==
                               PANE as libc::c_int as libc::c_uint {
                            key =
                                KEYC_WHEELDOWN_PANE as libc::c_int as key_code
                        }
                        if where_0 as libc::c_uint ==
                               STATUS as libc::c_int as libc::c_uint {
                            key =
                                KEYC_WHEELDOWN_STATUS as libc::c_int as
                                    key_code
                        }
                        if where_0 as libc::c_uint ==
                               BORDER as libc::c_int as libc::c_uint {
                            key =
                                KEYC_WHEELDOWN_BORDER as libc::c_int as
                                    key_code
                        }
                    }
                }
                3 => {
                    match b & 3i32 as libc::c_uint {
                        0 => {
                            current_block = 7011446367585754027;
                            match current_block {
                                2318432664459945252 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEUP2_PANE as libc::c_int
                                                as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP2_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP2_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                2905775416588883895 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEUP3_PANE as libc::c_int
                                                as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP3_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP3_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                _ => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEUP1_PANE as libc::c_int
                                                as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP1_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP1_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                            }
                        }
                        1 => {
                            current_block = 2318432664459945252;
                            match current_block {
                                2318432664459945252 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEUP2_PANE as libc::c_int
                                                as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP2_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP2_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                2905775416588883895 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEUP3_PANE as libc::c_int
                                                as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP3_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP3_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                _ => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEUP1_PANE as libc::c_int
                                                as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP1_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP1_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                            }
                        }
                        2 => {
                            current_block = 2905775416588883895;
                            match current_block {
                                2318432664459945252 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEUP2_PANE as libc::c_int
                                                as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP2_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP2_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                2905775416588883895 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEUP3_PANE as libc::c_int
                                                as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP3_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP3_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                _ => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEUP1_PANE as libc::c_int
                                                as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP1_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEUP1_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                            }
                        }
                        _ => { }
                    }
                }
                2 => {
                    match b & 3i32 as libc::c_uint {
                        0 => {
                            current_block = 8613272643215299965;
                            match current_block {
                                9784205294207992806 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEDOWN2_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN2_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN2_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                8613272643215299965 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEDOWN1_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN1_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN1_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                _ => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEDOWN3_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN3_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN3_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                            }
                        }
                        1 => {
                            current_block = 9784205294207992806;
                            match current_block {
                                9784205294207992806 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEDOWN2_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN2_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN2_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                8613272643215299965 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEDOWN1_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN1_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN1_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                _ => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEDOWN3_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN3_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN3_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                            }
                        }
                        2 => {
                            current_block = 5590603150538338923;
                            match current_block {
                                9784205294207992806 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEDOWN2_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN2_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN2_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                8613272643215299965 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEDOWN1_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN1_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN1_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                _ => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_MOUSEDOWN3_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN3_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_MOUSEDOWN3_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                            }
                        }
                        _ => { }
                    }
                }
                6 => {
                    match b & 3i32 as libc::c_uint {
                        0 => {
                            current_block = 3571746747088904883;
                            match current_block {
                                3571746747088904883 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_DOUBLECLICK1_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK1_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK1_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                249406591968161332 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_DOUBLECLICK3_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK3_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK3_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                _ => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_DOUBLECLICK2_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK2_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK2_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                            }
                        }
                        1 => {
                            current_block = 2828603588737439084;
                            match current_block {
                                3571746747088904883 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_DOUBLECLICK1_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK1_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK1_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                249406591968161332 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_DOUBLECLICK3_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK3_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK3_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                _ => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_DOUBLECLICK2_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK2_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK2_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                            }
                        }
                        2 => {
                            current_block = 249406591968161332;
                            match current_block {
                                3571746747088904883 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_DOUBLECLICK1_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK1_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK1_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                249406591968161332 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_DOUBLECLICK3_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK3_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK3_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                _ => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_DOUBLECLICK2_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK2_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_DOUBLECLICK2_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                            }
                        }
                        _ => { }
                    }
                }
                7 => {
                    match b & 3i32 as libc::c_uint {
                        0 => {
                            current_block = 15418940902936924983;
                            match current_block {
                                3680501547795761175 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_TRIPLECLICK2_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK2_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK2_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                5574021865528513969 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_TRIPLECLICK3_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK3_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK3_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                _ => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_TRIPLECLICK1_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK1_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK1_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                            }
                        }
                        1 => {
                            current_block = 3680501547795761175;
                            match current_block {
                                3680501547795761175 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_TRIPLECLICK2_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK2_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK2_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                5574021865528513969 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_TRIPLECLICK3_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK3_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK3_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                _ => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_TRIPLECLICK1_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK1_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK1_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                            }
                        }
                        2 => {
                            current_block = 5574021865528513969;
                            match current_block {
                                3680501547795761175 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_TRIPLECLICK2_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK2_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK2_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                5574021865528513969 => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_TRIPLECLICK3_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK3_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK3_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                                _ => {
                                    if where_0 as libc::c_uint ==
                                           PANE as libc::c_int as libc::c_uint
                                       {
                                        key =
                                            KEYC_TRIPLECLICK1_PANE as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           STATUS as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK1_STATUS as
                                                libc::c_int as key_code
                                    }
                                    if where_0 as libc::c_uint ==
                                           BORDER as libc::c_int as
                                               libc::c_uint {
                                        key =
                                            KEYC_TRIPLECLICK1_BORDER as
                                                libc::c_int as key_code
                                    }
                                }
                            }
                        }
                        _ => { }
                    }
                }
                0 | _ => { }
            }
            if key == 281466386776064u64 {
                return 281466386776064u64
            } else {
                if 0 != b & 8i32 as libc::c_uint { key |= 35184372088832u64 }
                if 0 != b & 16i32 as libc::c_uint { key |= 70368744177664u64 }
                if 0 != b & 4i32 as libc::c_uint { key |= 140737488355328u64 }
                return key
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_client_create(mut fd: libc::c_int)
 -> *mut client {
    let mut c: *mut client = 0 as *mut client;
    setblocking(fd, 0i32);
    c =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<client>() as libc::c_ulong) as
            *mut client;
    (*c).references = 1i32;
    (*c).peer =
        proc_add_peer(server_proc, fd, Some(server_client_dispatch),
                      c as *mut libc::c_void);
    if gettimeofday(&mut (*c).creation_time as *mut timeval,
                    0 as *mut timezone) != 0i32 {
        fatal(b"gettimeofday failed\x00" as *const u8 as *const libc::c_char);
    } else {
        memcpy(&mut (*c).activity_time as *mut timeval as *mut libc::c_void,
               &mut (*c).creation_time as *mut timeval as *const libc::c_void,
               ::std::mem::size_of::<timeval>() as libc::c_ulong);
        (*c).environ = environ_create();
        (*c).fd = 1i32.wrapping_neg();
        (*c).cwd = 0 as *const libc::c_char;
        loop  {
            let ref mut fresh2 =
                (*(&mut (*c).queue as *mut cmdq_list)).tqh_first;
            *fresh2 = 0 as *mut cmdq_item;
            let ref mut fresh3 =
                (*(&mut (*c).queue as *mut cmdq_list)).tqh_last;
            *fresh3 =
                &mut (*(&mut (*c).queue as *mut cmdq_list)).tqh_first as
                    *mut *mut cmdq_item;
            if !(0 != 0i32) { break ; }
        }
        (*c).stdin_data = evbuffer_new();
        (*c).stdout_data = evbuffer_new();
        (*c).stderr_data = evbuffer_new();
        (*c).tty.fd = 1i32.wrapping_neg();
        (*c).title = 0 as *mut libc::c_char;
        (*c).session = 0 as *mut session;
        (*c).last_session = 0 as *mut session;
        (*c).tty.sx = 80i32 as u_int;
        (*c).tty.sy = 24i32 as u_int;
        screen_init(&mut (*c).status.status as *mut screen, (*c).tty.sx,
                    1i32 as u_int, 0i32 as u_int);
        (*c).message_string = 0 as *mut libc::c_char;
        loop  {
            let ref mut fresh4 =
                (*(&mut (*c).message_log as *mut unnamed_11)).tqh_first;
            *fresh4 = 0 as *mut message_entry;
            let ref mut fresh5 =
                (*(&mut (*c).message_log as *mut unnamed_11)).tqh_last;
            *fresh5 =
                &mut (*(&mut (*c).message_log as *mut unnamed_11)).tqh_first
                    as *mut *mut message_entry;
            if !(0 != 0i32) { break ; }
        }
        (*c).prompt_string = 0 as *mut libc::c_char;
        (*c).prompt_buffer = 0 as *mut utf8_data;
        (*c).prompt_index = 0i32 as size_t;
        (*c).flags |= 32768i32;
        (*c).keytable =
            key_bindings_get_table(b"root\x00" as *const u8 as
                                       *const libc::c_char, 1i32);
        (*(*c).keytable).references =
            (*(*c).keytable).references.wrapping_add(1);
        event_set(&mut (*c).repeat_timer as *mut event, 1i32.wrapping_neg(),
                  0i32 as libc::c_short, Some(server_client_repeat_timer),
                  c as *mut libc::c_void);
        event_set(&mut (*c).click_timer as *mut event, 1i32.wrapping_neg(),
                  0i32 as libc::c_short, Some(server_client_click_timer),
                  c as *mut libc::c_void);
        loop  {
            (*c).entry.tqe_next = 0 as *mut client;
            (*c).entry.tqe_prev = (*(&mut clients as *mut clients)).tqh_last;
            let ref mut fresh6 = *(*(&mut clients as *mut clients)).tqh_last;
            *fresh6 = c;
            let ref mut fresh7 = (*(&mut clients as *mut clients)).tqh_last;
            *fresh7 = &mut (*c).entry.tqe_next as *mut *mut client;
            if !(0 != 0i32) { break ; }
        }
        log_debug(b"new client %p\x00" as *const u8 as *const libc::c_char,
                  c);
        return c
    };
}
unsafe extern "C" fn server_client_click_timer(mut fd: libc::c_int,
                                               mut events: libc::c_short,
                                               mut data: *mut libc::c_void)
 -> () {
    let mut c: *mut client = data as *mut client;
    (*c).flags &= !(1048576i32 | 2097152i32);
}
unsafe extern "C" fn server_client_repeat_timer(mut fd: libc::c_int,
                                                mut events: libc::c_short,
                                                mut data: *mut libc::c_void)
 -> () {
    let mut c: *mut client = data as *mut client;
    if 0 != (*c).flags & 32i32 {
        server_client_set_key_table(c, 0 as *const libc::c_char);
        (*c).flags &= !32i32;
        server_status_client(c);
    };
}
unsafe extern "C" fn server_client_dispatch(mut imsg: *mut imsg,
                                            mut arg: *mut libc::c_void)
 -> () {
    let mut c: *mut client = arg as *mut client;
    let mut stdindata: msg_stdin_data =
        msg_stdin_data{size: 0, data: [0; 8192],};
    let mut data: *const libc::c_char = 0 as *const libc::c_char;
    let mut datalen: ssize_t = 0;
    let mut s: *mut session = 0 as *mut session;
    if 0 != (*c).flags & 512i32 {
        return
    } else if imsg == 0 as *mut libc::c_void as *mut imsg {
        server_client_lost(c);
        return
    } else {
        data = (*imsg).data as *const libc::c_char;
        datalen =
            ((*imsg).hdr.len as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<imsg_hdr>()
                                                 as libc::c_ulong) as ssize_t;
        match (*imsg).hdr.type_0 {
            100 | 101 | 102 | 108 | 104 | 105 | 107 | 106 => {
                server_client_dispatch_identify(c, imsg);
            }
            200 => { server_client_dispatch_command(c, imsg); }
            212 => {
                if datalen as libc::c_ulong !=
                       ::std::mem::size_of::<msg_stdin_data>() as
                           libc::c_ulong {
                    fatalx(b"bad MSG_STDIN size\x00" as *const u8 as
                               *const libc::c_char);
                } else {
                    memcpy(&mut stdindata as *mut msg_stdin_data as
                               *mut libc::c_void, data as *const libc::c_void,
                           ::std::mem::size_of::<msg_stdin_data>() as
                               libc::c_ulong);
                    if !((*c).stdin_callback ==
                             ::std::mem::transmute::<*mut libc::c_void,
                                                     Option<unsafe extern "C" fn(_:
                                                                                     *mut client,
                                                                                 _:
                                                                                     libc::c_int,
                                                                                 _:
                                                                                     *mut libc::c_void)
                                                                ->
                                                                    ()>>(0 as
                                                                             *mut libc::c_void))
                       {
                        if stdindata.size <= 0i32 as libc::c_long {
                            (*c).stdin_closed = 1i32
                        } else {
                            evbuffer_add((*c).stdin_data,
                                         stdindata.data.as_mut_ptr() as
                                             *const libc::c_void,
                                         stdindata.size as size_t);
                        }
                        (*c).stdin_callback.expect("non-null function pointer")(c,
                                                                                (*c).stdin_closed,
                                                                                (*c).stdin_callback_data);
                    }
                }
            }
            208 => {
                if datalen != 0i32 as libc::c_long {
                    fatalx(b"bad MSG_RESIZE size\x00" as *const u8 as
                               *const libc::c_char);
                } else if !(0 != (*c).flags & 8192i32) {
                    tty_resize(&mut (*c).tty as *mut tty);
                    recalculate_sizes();
                    server_redraw_client(c);
                    if (*c).session != 0 as *mut libc::c_void as *mut session
                       {
                        notify_client(b"client-resized\x00" as *const u8 as
                                          *const libc::c_char, c);
                    }
                }
            }
            205 => {
                if datalen != 0i32 as libc::c_long {
                    fatalx(b"bad MSG_EXITING size\x00" as *const u8 as
                               *const libc::c_char);
                } else {
                    (*c).session = 0 as *mut session;
                    tty_close(&mut (*c).tty as *mut tty);
                    proc_send((*c).peer, MSG_EXITED, 1i32.wrapping_neg(),
                              0 as *const libc::c_void, 0i32 as size_t);
                }
            }
            216 | 215 => {
                if datalen != 0i32 as libc::c_long {
                    fatalx(b"bad MSG_WAKEUP size\x00" as *const u8 as
                               *const libc::c_char);
                } else if !(0 == (*c).flags & 64i32) {
                    (*c).flags &= !64i32;
                    if !((*c).tty.fd == 1i32.wrapping_neg()) {
                        s = (*c).session;
                        if gettimeofday(&mut (*c).activity_time as
                                            *mut timeval, 0 as *mut timezone)
                               != 0i32 {
                            fatal(b"gettimeofday failed\x00" as *const u8 as
                                      *const libc::c_char);
                        } else {
                            tty_start_tty(&mut (*c).tty as *mut tty);
                            server_redraw_client(c);
                            recalculate_sizes();
                            if s != 0 as *mut libc::c_void as *mut session {
                                session_update_activity(s,
                                                        &mut (*c).activity_time
                                                            as *mut timeval);
                            }
                        }
                    }
                }
            }
            209 => {
                if datalen != 0i32 as libc::c_long {
                    fatalx(b"bad MSG_SHELL size\x00" as *const u8 as
                               *const libc::c_char);
                } else { server_client_dispatch_shell(c); }
            }
            _ => { }
        }
        return;
    };
}
unsafe extern "C" fn server_client_dispatch_shell(mut c: *mut client) -> () {
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    shell =
        options_get_string(global_s_options,
                           b"default-shell\x00" as *const u8 as
                               *const libc::c_char);
    if *shell as libc::c_int == 0 || 0 != areshell(shell) {
        shell = b"/bin/sh\x00" as *const u8 as *const libc::c_char
    }
    proc_send((*c).peer, MSG_SHELL, 1i32.wrapping_neg(),
              shell as *const libc::c_void,
              strlen(shell).wrapping_add(1i32 as libc::c_ulong));
    proc_kill_peer((*c).peer);
}
unsafe extern "C" fn server_client_dispatch_command(mut c: *mut client,
                                                    mut imsg: *mut imsg)
 -> () {
    let mut data: msg_command_data = msg_command_data{argc: 0,};
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    let mut argc: libc::c_int = 0;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 != (*c).flags & 4i32 {
        return
    } else if ((*imsg).hdr.len as
                   libc::c_ulong).wrapping_sub(::std::mem::size_of::<imsg_hdr>()
                                                   as libc::c_ulong) <
                  ::std::mem::size_of::<msg_command_data>() as libc::c_ulong {
        fatalx(b"bad MSG_COMMAND size\x00" as *const u8 as
                   *const libc::c_char);
    } else {
        memcpy(&mut data as *mut msg_command_data as *mut libc::c_void,
               (*imsg).data,
               ::std::mem::size_of::<msg_command_data>() as libc::c_ulong);
        buf =
            ((*imsg).data as
                 *mut libc::c_char).offset(::std::mem::size_of::<msg_command_data>()
                                               as libc::c_ulong as isize);
        len =
            ((*imsg).hdr.len as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<imsg_hdr>()
                                                 as
                                                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<msg_command_data>()
                                                                                 as
                                                                                 libc::c_ulong);
        if len > 0i32 as libc::c_ulong &&
               *buf.offset(len.wrapping_sub(1i32 as libc::c_ulong) as isize)
                   as libc::c_int != 0 {
            fatalx(b"bad MSG_COMMAND string\x00" as *const u8 as
                       *const libc::c_char);
        } else {
            argc = data.argc;
            if cmd_unpack_argv(buf, len, argc,
                               &mut argv as *mut *mut *mut libc::c_char) !=
                   0i32 {
                cause =
                    xstrdup(b"command too long\x00" as *const u8 as
                                *const libc::c_char)
            } else {
                if argc == 0i32 {
                    argc = 1i32;
                    argv =
                        xcalloc(1i32 as size_t,
                                ::std::mem::size_of::<*mut libc::c_char>() as
                                    libc::c_ulong) as *mut *mut libc::c_char;
                    *argv =
                        xstrdup(b"new-session\x00" as *const u8 as
                                    *const libc::c_char)
                }
                cmdlist =
                    cmd_list_parse(argc, argv, 0 as *const libc::c_char,
                                   0i32 as u_int,
                                   &mut cause as *mut *mut libc::c_char);
                if cmdlist == 0 as *mut libc::c_void as *mut cmd_list {
                    cmd_free_argv(argc, argv);
                } else {
                    cmd_free_argv(argc, argv);
                    cmdq_append(c,
                                cmdq_get_command(cmdlist,
                                                 0 as *mut cmd_find_state,
                                                 0 as *mut mouse_event,
                                                 0i32));
                    cmdq_append(c,
                                cmdq_get_callback1(b"server_client_command_done\x00"
                                                       as *const u8 as
                                                       *const libc::c_char,
                                                   Some(server_client_command_done),
                                                   0 as *mut libc::c_void));
                    cmd_list_free(cmdlist);
                    return
                }
            }
            cmdq_append(c,
                        cmdq_get_callback1(b"server_client_command_error\x00"
                                               as *const u8 as
                                               *const libc::c_char,
                                           Some(server_client_command_error),
                                           cause as *mut libc::c_void));
            if cmdlist != 0 as *mut libc::c_void as *mut cmd_list {
                cmd_list_free(cmdlist);
            }
            (*c).flags |= 4i32;
            return;
        }
    };
}
unsafe extern "C" fn server_client_command_error(mut item: *mut cmdq_item,
                                                 mut data: *mut libc::c_void)
 -> cmd_retval {
    let mut error: *mut libc::c_char = data as *mut libc::c_char;
    cmdq_error(item, b"%s\x00" as *const u8 as *const libc::c_char, error);
    free(error as *mut libc::c_void);
    return CMD_RETURN_NORMAL;
}
unsafe extern "C" fn server_client_command_done(mut item: *mut cmdq_item,
                                                mut data: *mut libc::c_void)
 -> cmd_retval {
    let mut c: *mut client = (*item).client;
    if 0 != !(*c).flags & 128i32 { (*c).flags |= 4i32 }
    return CMD_RETURN_NORMAL;
}
unsafe extern "C" fn server_client_dispatch_identify(mut c: *mut client,
                                                     mut imsg: *mut imsg)
 -> () {
    let mut data: *const libc::c_char = 0 as *const libc::c_char;
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    let mut datalen: size_t = 0;
    let mut flags: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 != (*c).flags & 262144i32 {
        fatalx(b"out-of-order identify message\x00" as *const u8 as
                   *const libc::c_char);
    } else {
        data = (*imsg).data as *const libc::c_char;
        datalen =
            ((*imsg).hdr.len as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<imsg_hdr>()
                                                 as libc::c_ulong);
        match (*imsg).hdr.type_0 {
            100 => {
                if datalen !=
                       ::std::mem::size_of::<libc::c_int>() as libc::c_ulong {
                    fatalx(b"bad MSG_IDENTIFY_FLAGS size\x00" as *const u8 as
                               *const libc::c_char);
                } else {
                    memcpy(&mut flags as *mut libc::c_int as
                               *mut libc::c_void, data as *const libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong);
                    (*c).flags |= flags;
                    log_debug(b"client %p IDENTIFY_FLAGS %#x\x00" as *const u8
                                  as *const libc::c_char, c, flags);
                }
            }
            101 => {
                if datalen == 0i32 as libc::c_ulong ||
                       *data.offset(datalen.wrapping_sub(1i32 as
                                                             libc::c_ulong) as
                                        isize) as libc::c_int != 0 {
                    fatalx(b"bad MSG_IDENTIFY_TERM string\x00" as *const u8 as
                               *const libc::c_char);
                } else {
                    (*c).term = xstrdup(data);
                    log_debug(b"client %p IDENTIFY_TERM %s\x00" as *const u8
                                  as *const libc::c_char, c, data);
                }
            }
            102 => {
                if datalen == 0i32 as libc::c_ulong ||
                       *data.offset(datalen.wrapping_sub(1i32 as
                                                             libc::c_ulong) as
                                        isize) as libc::c_int != 0 {
                    fatalx(b"bad MSG_IDENTIFY_TTYNAME string\x00" as *const u8
                               as *const libc::c_char);
                } else {
                    (*c).ttyname = xstrdup(data);
                    log_debug(b"client %p IDENTIFY_TTYNAME %s\x00" as
                                  *const u8 as *const libc::c_char, c, data);
                }
            }
            108 => {
                if datalen == 0i32 as libc::c_ulong ||
                       *data.offset(datalen.wrapping_sub(1i32 as
                                                             libc::c_ulong) as
                                        isize) as libc::c_int != 0 {
                    fatalx(b"bad MSG_IDENTIFY_CWD string\x00" as *const u8 as
                               *const libc::c_char);
                } else {
                    if access(data, 1i32) == 0i32 {
                        (*c).cwd = xstrdup(data)
                    } else {
                        home = find_home();
                        if home !=
                               0 as *mut libc::c_void as *const libc::c_char {
                            (*c).cwd = xstrdup(home)
                        } else {
                            (*c).cwd =
                                xstrdup(b"/\x00" as *const u8 as
                                            *const libc::c_char)
                        }
                    }
                    log_debug(b"client %p IDENTIFY_CWD %s\x00" as *const u8 as
                                  *const libc::c_char, c, data);
                }
            }
            104 => {
                if datalen != 0i32 as libc::c_ulong {
                    fatalx(b"bad MSG_IDENTIFY_STDIN size\x00" as *const u8 as
                               *const libc::c_char);
                } else {
                    (*c).fd = (*imsg).fd;
                    log_debug(b"client %p IDENTIFY_STDIN %d\x00" as *const u8
                                  as *const libc::c_char, c, (*imsg).fd);
                }
            }
            105 => {
                if datalen == 0i32 as libc::c_ulong ||
                       *data.offset(datalen.wrapping_sub(1i32 as
                                                             libc::c_ulong) as
                                        isize) as libc::c_int != 0 {
                    fatalx(b"bad MSG_IDENTIFY_ENVIRON string\x00" as *const u8
                               as *const libc::c_char);
                } else {
                    if strchr(data, 61) !=
                           0 as *mut libc::c_void as *mut libc::c_char {
                        environ_put((*c).environ, data);
                    }
                    log_debug(b"client %p IDENTIFY_ENVIRON %s\x00" as
                                  *const u8 as *const libc::c_char, c, data);
                }
            }
            107 => {
                if datalen != ::std::mem::size_of::<pid_t>() as libc::c_ulong
                   {
                    fatalx(b"bad MSG_IDENTIFY_CLIENTPID size\x00" as *const u8
                               as *const libc::c_char);
                } else {
                    memcpy(&mut (*c).pid as *mut pid_t as *mut libc::c_void,
                           data as *const libc::c_void,
                           ::std::mem::size_of::<pid_t>() as libc::c_ulong);
                    log_debug(b"client %p IDENTIFY_CLIENTPID %ld\x00" as
                                  *const u8 as *const libc::c_char, c,
                              (*c).pid as libc::c_long);
                }
            }
            _ => { }
        }
        if (*imsg).hdr.type_0 !=
               MSG_IDENTIFY_DONE as libc::c_int as libc::c_uint {
            return
        } else {
            (*c).flags |= 262144i32;
            if *(*c).ttyname as libc::c_int != 0 {
                name = xstrdup((*c).ttyname)
            } else {
                xasprintf(&mut name as *mut *mut libc::c_char,
                          b"client-%ld\x00" as *const u8 as
                              *const libc::c_char, (*c).pid as libc::c_long);
            }
            (*c).name = name;
            log_debug(b"client %p name is %s\x00" as *const u8 as
                          *const libc::c_char, c, (*c).name);
            if 0 != (*c).flags & 8192i32 {
                (*c).stdin_callback = Some(control_callback);
                evbuffer_free((*c).stderr_data);
                (*c).stderr_data = (*c).stdout_data;
                if 0 != (*c).flags & 16384i32 {
                    evbuffer_add_printf((*c).stdout_data,
                                        b"\x1bP1000p\x00" as *const u8 as
                                            *const libc::c_char);
                }
                proc_send((*c).peer, MSG_STDIN, 1i32.wrapping_neg(),
                          0 as *const libc::c_void, 0i32 as size_t);
                (*c).tty.fd = 1i32.wrapping_neg();
                close((*c).fd);
                (*c).fd = 1i32.wrapping_neg();
                return
            } else if (*c).fd == 1i32.wrapping_neg() {
                return
            } else if tty_init(&mut (*c).tty as *mut tty, c, (*c).fd,
                               (*c).term) != 0i32 {
                close((*c).fd);
                (*c).fd = 1i32.wrapping_neg();
                return
            } else {
                if 0 != (*c).flags & 65536i32 { (*c).tty.flags |= 8i32 }
                if 0 != (*c).flags & 131072i32 { (*c).tty.term_flags |= 1i32 }
                tty_resize(&mut (*c).tty as *mut tty);
                if 0 == (*c).flags & 8192i32 { (*c).flags |= 1i32 }
                return;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_client_lost(mut c: *mut client) -> () {
    let mut msg: *mut message_entry = 0 as *mut message_entry;
    let mut msg1: *mut message_entry = 0 as *mut message_entry;
    (*c).flags |= 512i32;
    server_client_clear_identify(c, 0 as *mut window_pane);
    status_prompt_clear(c);
    status_message_clear(c);
    if (*c).stdin_callback !=
           ::std::mem::transmute::<*mut libc::c_void,
                                   Option<unsafe extern "C" fn(_: *mut client,
                                                               _: libc::c_int,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> ()>>(0 as *mut libc::c_void)
       {
        (*c).stdin_callback.expect("non-null function pointer")(c, 1i32,
                                                                (*c).stdin_callback_data);
    }
    loop  {
        if (*c).entry.tqe_next != 0 as *mut libc::c_void as *mut client {
            (*(*c).entry.tqe_next).entry.tqe_prev = (*c).entry.tqe_prev
        } else {
            let ref mut fresh8 = (*(&mut clients as *mut clients)).tqh_last;
            *fresh8 = (*c).entry.tqe_prev
        }
        *(*c).entry.tqe_prev = (*c).entry.tqe_next;
        if !(0 != 0i32) { break ; }
    }
    log_debug(b"lost client %p\x00" as *const u8 as *const libc::c_char, c);
    if 0 != (*c).flags & 1i32 { tty_free(&mut (*c).tty as *mut tty); }
    free((*c).ttyname as *mut libc::c_void);
    free((*c).term as *mut libc::c_void);
    evbuffer_free((*c).stdin_data);
    evbuffer_free((*c).stdout_data);
    if (*c).stderr_data != (*c).stdout_data {
        evbuffer_free((*c).stderr_data);
    }
    if 0 != event_initialized(&mut (*c).status.timer as *mut event) {
        event_del(&mut (*c).status.timer as *mut event);
    }
    screen_free(&mut (*c).status.status as *mut screen);
    if (*c).status.old_status != 0 as *mut libc::c_void as *mut screen {
        screen_free((*c).status.old_status);
        free((*c).status.old_status as *mut libc::c_void);
    }
    free((*c).title as *mut libc::c_void);
    free((*c).cwd as *mut libc::c_void);
    event_del(&mut (*c).repeat_timer as *mut event);
    event_del(&mut (*c).click_timer as *mut event);
    key_bindings_unref_table((*c).keytable);
    if 0 != event_initialized(&mut (*c).identify_timer as *mut event) {
        event_del(&mut (*c).identify_timer as *mut event);
    }
    free((*c).message_string as *mut libc::c_void);
    if 0 != event_initialized(&mut (*c).message_timer as *mut event) {
        event_del(&mut (*c).message_timer as *mut event);
    }
    msg = (*(&mut (*c).message_log as *mut unnamed_11)).tqh_first;
    while msg != 0 as *mut libc::c_void as *mut message_entry &&
              { msg1 = (*msg).entry.tqe_next; 0 != 1i32 } {
        free((*msg).msg as *mut libc::c_void);
        loop  {
            if (*msg).entry.tqe_next !=
                   0 as *mut libc::c_void as *mut message_entry {
                (*(*msg).entry.tqe_next).entry.tqe_prev =
                    (*msg).entry.tqe_prev
            } else {
                let ref mut fresh9 =
                    (*(&mut (*c).message_log as *mut unnamed_11)).tqh_last;
                *fresh9 = (*msg).entry.tqe_prev
            }
            *(*msg).entry.tqe_prev = (*msg).entry.tqe_next;
            if !(0 != 0i32) { break ; }
        }
        free(msg as *mut libc::c_void);
        msg = msg1
    }
    free((*c).prompt_string as *mut libc::c_void);
    free((*c).prompt_buffer as *mut libc::c_void);
    format_lost_client(c);
    environ_free((*c).environ);
    proc_remove_peer((*c).peer);
    (*c).peer = 0 as *mut tmuxpeer;
    server_client_unref(c);
    server_add_accept(0i32);
    recalculate_sizes();
    server_check_unattached();
    server_update_socket();
}
#[no_mangle]
pub unsafe extern "C" fn server_client_unref(mut c: *mut client) -> () {
    log_debug(b"unref client %p (%d references)\x00" as *const u8 as
                  *const libc::c_char, c, (*c).references);
    (*c).references -= 1;
    if (*c).references == 0i32 {
        event_once(1i32.wrapping_neg(), 1i32 as libc::c_short,
                   Some(server_client_free), c as *mut libc::c_void,
                   0 as *const timeval);
    };
}
unsafe extern "C" fn server_client_free(mut fd: libc::c_int,
                                        mut events: libc::c_short,
                                        mut arg: *mut libc::c_void) -> () {
    let mut c: *mut client = arg as *mut client;
    log_debug(b"free client %p (%d references)\x00" as *const u8 as
                  *const libc::c_char, c, (*c).references);
    if 0 !=
           !((*(&mut (*c).queue as *mut cmdq_list)).tqh_first ==
                 0 as *mut libc::c_void as *mut cmdq_item) as libc::c_int {
        fatalx(b"queue not empty\x00" as *const u8 as *const libc::c_char);
    } else {
        if (*c).references == 0i32 {
            free((*c).name as *mut libc::c_void);
            free(c as *mut libc::c_void);
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_client_open(mut c: *mut client,
                                            mut cause: *mut *mut libc::c_char)
 -> libc::c_int {
    if 0 != (*c).flags & 8192i32 {
        return 0i32
    } else if strcmp((*c).ttyname,
                     b"/dev/tty\x00" as *const u8 as *const libc::c_char) ==
                  0i32 {
        *cause =
            xstrdup(b"can\'t use /dev/tty\x00" as *const u8 as
                        *const libc::c_char);
        return 1i32.wrapping_neg()
    } else if 0 == (*c).flags & 1i32 {
        *cause =
            xstrdup(b"not a terminal\x00" as *const u8 as
                        *const libc::c_char);
        return 1i32.wrapping_neg()
    } else if tty_open(&mut (*c).tty as *mut tty, cause) != 0i32 {
        return 1i32.wrapping_neg()
    } else { return 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn server_client_suspend(mut c: *mut client) -> () {
    let mut s: *mut session = (*c).session;
    if s == 0 as *mut libc::c_void as *mut session ||
           0 != (*c).flags & 4096i32 {
        return
    } else {
        tty_stop_tty(&mut (*c).tty as *mut tty);
        (*c).flags |= 64i32;
        proc_send((*c).peer, MSG_SUSPEND, 1i32.wrapping_neg(),
                  0 as *const libc::c_void, 0i32 as size_t);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_client_detach(mut c: *mut client,
                                              mut msgtype: msgtype) -> () {
    let mut s: *mut session = (*c).session;
    if s == 0 as *mut libc::c_void as *mut session ||
           0 != (*c).flags & 4096i32 {
        return
    } else {
        (*c).flags |= 4096i32;
        notify_client(b"client-detached\x00" as *const u8 as
                          *const libc::c_char, c);
        proc_send((*c).peer, msgtype, 1i32.wrapping_neg(),
                  (*s).name as *const libc::c_void,
                  strlen((*s).name).wrapping_add(1i32 as libc::c_ulong));
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_client_exec(mut c: *mut client,
                                            mut cmd: *const libc::c_char)
 -> () {
    let mut s: *mut session = (*c).session;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmdsize: size_t = 0;
    let mut shellsize: size_t = 0;
    if *cmd as libc::c_int == 0 {
        return
    } else {
        cmdsize = strlen(cmd).wrapping_add(1i32 as libc::c_ulong);
        if s != 0 as *mut libc::c_void as *mut session {
            shell =
                options_get_string((*s).options,
                                   b"default-shell\x00" as *const u8 as
                                       *const libc::c_char)
        } else {
            shell =
                options_get_string(global_s_options,
                                   b"default-shell\x00" as *const u8 as
                                       *const libc::c_char)
        }
        shellsize = strlen(shell).wrapping_add(1i32 as libc::c_ulong);
        msg = xmalloc(cmdsize.wrapping_add(shellsize)) as *mut libc::c_char;
        memcpy(msg as *mut libc::c_void, cmd as *const libc::c_void, cmdsize);
        memcpy(msg.offset(cmdsize as isize) as *mut libc::c_void,
               shell as *const libc::c_void, shellsize);
        proc_send((*c).peer, MSG_EXEC, 1i32.wrapping_neg(),
                  msg as *const libc::c_void,
                  cmdsize.wrapping_add(shellsize));
        free(msg as *mut libc::c_void);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_client_loop() -> () {
    let mut c: *mut client = 0 as *mut client;
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut focus: libc::c_int = 0;
    c = (*(&mut clients as *mut clients)).tqh_first;
    while c != 0 as *mut libc::c_void as *mut client {
        server_client_check_exit(c);
        if (*c).session != 0 as *mut libc::c_void as *mut session {
            server_client_check_redraw(c);
            server_client_reset_state(c);
        }
        c = (*c).entry.tqe_next
    }
    focus =
        options_get_number(global_options,
                           b"focus-events\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    w = windows_RB_MINMAX(&mut windows as *mut windows, 1i32.wrapping_neg());
    while w != 0 as *mut libc::c_void as *mut window {
        wp = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
        while wp != 0 as *mut libc::c_void as *mut window_pane {
            if (*wp).fd != 1i32.wrapping_neg() {
                if 0 != focus { server_client_check_focus(wp); }
                server_client_check_resize(wp);
            }
            (*wp).flags &= !1i32;
            wp = (*wp).entry.tqe_next
        }
        check_window_name(w);
        w = windows_RB_NEXT(w)
    };
}
unsafe extern "C" fn server_client_check_resize(mut wp: *mut window_pane)
 -> () {
    let mut tv: timeval =
        timeval{tv_sec: 0, tv_usec: 250000i32 as __suseconds_t,};
    if 0 == (*wp).flags & 8i32 {
        return
    } else {
        log_debug(b"%s: %%%u resize to %u,%u\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"server_client_check_resize\x00")).as_ptr(),
                  (*wp).id, (*wp).sx, (*wp).sy);
        if 0 == event_initialized(&mut (*wp).resize_timer as *mut event) {
            event_set(&mut (*wp).resize_timer as *mut event,
                      1i32.wrapping_neg(), 0i32 as libc::c_short,
                      Some(server_client_resize_event),
                      wp as *mut libc::c_void);
        }
        if 0 ==
               event_pending(&mut (*wp).resize_timer as *mut event,
                             1i32 as libc::c_short, 0 as *mut timeval) {
            server_client_resize_event(1i32.wrapping_neg(),
                                       0i32 as libc::c_short,
                                       wp as *mut libc::c_void);
        }
        if (*wp).saved_grid != 0 as *mut libc::c_void as *mut grid &&
               0 !=
                   event_pending(&mut (*wp).resize_timer as *mut event,
                                 1i32 as libc::c_short, 0 as *mut timeval) {
            return
        } else {
            event_del(&mut (*wp).resize_timer as *mut event);
            event_add(&mut (*wp).resize_timer as *mut event,
                      &mut tv as *mut timeval);
            return;
        }
    };
}
unsafe extern "C" fn server_client_resize_event(mut fd: libc::c_int,
                                                mut events: libc::c_short,
                                                mut data: *mut libc::c_void)
 -> () {
    let mut wp: *mut window_pane = data as *mut window_pane;
    let mut ws: winsize =
        winsize{ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0,};
    event_del(&mut (*wp).resize_timer as *mut event);
    if 0 == (*wp).flags & 8i32 {
        return
    } else if 0 != server_client_resize_force(wp) {
        return
    } else {
        memset(&mut ws as *mut winsize as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<winsize>() as libc::c_ulong);
        ws.ws_col = (*wp).sx as libc::c_ushort;
        ws.ws_row = (*wp).sy as libc::c_ushort;
        if (*wp).fd != 1i32.wrapping_neg() &&
               ioctl((*wp).fd, 21524i32 as libc::c_ulong,
                     &mut ws as *mut winsize) == 1i32.wrapping_neg() {
            fatal(b"ioctl failed\x00" as *const u8 as *const libc::c_char);
        } else {
            log_debug(b"%s: %%%u resize to %u,%u\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"server_client_resize_event\x00")).as_ptr(),
                      (*wp).id, (*wp).sx, (*wp).sy);
            (*wp).flags &= !8i32;
            (*wp).osx = (*wp).sx;
            (*wp).osy = (*wp).sy;
            return;
        }
    };
}
unsafe extern "C" fn server_client_resize_force(mut wp: *mut window_pane)
 -> libc::c_int {
    let mut tv: timeval =
        timeval{tv_sec: 0, tv_usec: 100000i32 as __suseconds_t,};
    let mut ws: winsize =
        winsize{ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0,};
    if 0 != (*wp).flags & 16i32 {
        (*wp).flags &= !16i32;
        return 0i32
    } else if (*wp).sx != (*wp).osx || (*wp).sy != (*wp).osy ||
                  (*wp).sx <= 1i32 as libc::c_uint ||
                  (*wp).sy <= 1i32 as libc::c_uint {
        return 0i32
    } else {
        memset(&mut ws as *mut winsize as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<winsize>() as libc::c_ulong);
        ws.ws_col = (*wp).sx as libc::c_ushort;
        ws.ws_row =
            (*wp).sy.wrapping_sub(1i32 as libc::c_uint) as libc::c_ushort;
        if (*wp).fd != 1i32.wrapping_neg() &&
               ioctl((*wp).fd, 21524i32 as libc::c_ulong,
                     &mut ws as *mut winsize) == 1i32.wrapping_neg() {
            fatal(b"ioctl failed\x00" as *const u8 as *const libc::c_char);
        } else {
            log_debug(b"%s: %%%u forcing resize\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"server_client_resize_force\x00")).as_ptr(),
                      (*wp).id);
            event_add(&mut (*wp).resize_timer as *mut event,
                      &mut tv as *mut timeval);
            (*wp).flags |= 16i32;
            return 1i32
        }
    };
}
unsafe extern "C" fn server_client_check_focus(mut wp: *mut window_pane)
 -> () {
    let mut current_block: u64;
    let mut c: *mut client = 0 as *mut client;
    let mut push: libc::c_int = 0;
    push = (*wp).flags & 32i32;
    (*wp).flags &= !32i32;
    if 0 == (*wp).base.mode & 2048i32 {
        return
    } else {
        if !((*(*wp).window).active != wp) {
            if !((*wp).screen != &mut (*wp).base as *mut screen) {
                c = (*(&mut clients as *mut clients)).tqh_first;
                loop  {
                    if !(c != 0 as *mut libc::c_void as *mut client) {
                        current_block = 160336299305500981;
                        break ;
                    }
                    if !((*c).session ==
                             0 as *mut libc::c_void as *mut session ||
                             0 == (*c).flags & 32768i32) {
                        if !(0 != (*(*c).session).flags & 1i32) {
                            if (*(*(*c).session).curw).window == (*wp).window
                               {
                                if 0 != push || 0 == (*wp).flags & 4i32 {
                                    current_block = 7815301370352969686;
                                    break ;
                                } else {
                                    current_block = 7351195479953500246;
                                    break ;
                                }
                            }
                        }
                    }
                    c = (*c).entry.tqe_next
                }
                match current_block {
                    160336299305500981 => { }
                    _ => {
                        match current_block {
                            7815301370352969686 => {
                                bufferevent_write((*wp).event,
                                                  b"\x1b[I\x00" as *const u8
                                                      as *const libc::c_char
                                                      as *const libc::c_void,
                                                  3i32 as size_t);
                            }
                            _ => { }
                        }
                        (*wp).flags |= 4i32;
                        return;
                    }
                }
            }
        }
        if 0 != push || 0 != (*wp).flags & 4i32 {
            bufferevent_write((*wp).event,
                              b"\x1b[O\x00" as *const u8 as
                                  *const libc::c_char as *const libc::c_void,
                              3i32 as size_t);
        }
        (*wp).flags &= !4i32;
        return
    };
}
unsafe extern "C" fn server_client_reset_state(mut c: *mut client) -> () {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = (*w).active;
    let mut loop_0: *mut window_pane = 0 as *mut window_pane;
    let mut s: *mut screen = (*wp).screen;
    let mut oo: *mut options = (*(*c).session).options;
    let mut lines: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    if 0 != (*c).flags & (8192i32 | 64i32) {
        return
    } else {
        tty_region_off(&mut (*c).tty as *mut tty);
        tty_margin_off(&mut (*c).tty as *mut tty);
        if status_at_line(c) != 0i32 {
            lines = 0i32
        } else { lines = status_line_size((*c).session) as libc::c_int }
        if 0 == window_pane_visible(wp) ||
               (*wp).yoff.wrapping_add((*s).cy) >=
                   (*c).tty.sy.wrapping_sub(lines as libc::c_uint) {
            tty_cursor(&mut (*c).tty as *mut tty, 0i32 as u_int,
                       0i32 as u_int);
        } else {
            tty_cursor(&mut (*c).tty as *mut tty,
                       (*wp).xoff.wrapping_add((*s).cx),
                       (lines as
                            libc::c_uint).wrapping_add((*wp).yoff).wrapping_add((*s).cy));
        }
        mode = (*s).mode;
        if 0 !=
               options_get_number(oo,
                                  b"mouse\x00" as *const u8 as
                                      *const libc::c_char) {
            mode &= !(32i32 | 64i32 | 4096i32);
            loop_0 = (*(&mut (*w).panes as *mut window_panes)).tqh_first;
            while loop_0 != 0 as *mut libc::c_void as *mut window_pane {
                if 0 != (*(*loop_0).screen).mode & 4096i32 { mode |= 4096i32 }
                loop_0 = (*loop_0).entry.tqe_next
            }
            if 0 != !mode & 4096i32 { mode |= 64i32 }
        }
        if (*c).prompt_string != 0 as *mut libc::c_void as *mut libc::c_char {
            mode &= !1024i32
        }
        tty_update_mode(&mut (*c).tty as *mut tty, mode, s);
        tty_reset(&mut (*c).tty as *mut tty);
        return;
    };
}
unsafe extern "C" fn server_client_check_redraw(mut c: *mut client) -> () {
    let mut s: *mut session = (*c).session;
    let mut tty: *mut tty = &mut (*c).tty as *mut tty;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut needed: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut masked: libc::c_int = 0;
    let mut tv: timeval =
        timeval{tv_sec: 0, tv_usec: 1000i32 as __suseconds_t,};
    static mut ev: event =
        unsafe {
            event{ev_active_next:
                      unnamed_8{tqe_next: 0 as *const event as *mut event,
                                tqe_prev:
                                    0 as *const *mut event as
                                        *mut *mut event,},
                  ev_next:
                      unnamed_36{tqe_next: 0 as *const event as *mut event,
                                 tqe_prev:
                                     0 as *const *mut event as
                                         *mut *mut event,},
                  ev_timeout_pos:
                      unnamed_33{ev_next_with_common_timeout:
                                     unnamed_17{tqe_next:
                                                    0 as *const event as
                                                        *mut event,
                                                tqe_prev:
                                                    0 as *const *mut event as
                                                        *mut *mut event,},},
                  ev_fd: 0,
                  ev_base: 0 as *const event_base as *mut event_base,
                  _ev:
                      unnamed_32{ev_io:
                                     unnamed_30{ev_io_next:
                                                    unnamed_10{tqe_next:
                                                                   0 as
                                                                       *const event
                                                                       as
                                                                       *mut event,
                                                               tqe_prev:
                                                                   0 as
                                                                       *const *mut event
                                                                       as
                                                                       *mut *mut event,},
                                                ev_timeout:
                                                    timeval{tv_sec: 0,
                                                            tv_usec: 0,},},},
                  ev_events: 0,
                  ev_res: 0,
                  ev_flags: 0,
                  ev_pri: 0,
                  ev_closure: 0,
                  ev_timeout: timeval{tv_sec: 0, tv_usec: 0,},
                  ev_callback: None,
                  ev_arg: 0 as *const libc::c_void as *mut libc::c_void,}
        };
    let mut left: size_t = 0;
    if 0 != (*c).flags & (8192i32 | 64i32) {
        return
    } else {
        needed = 0i32;
        if 0 != (*c).flags & 8i32 {
            needed = 1i32
        } else {
            wp =
                (*(&mut (*(*(*(*c).session).curw).window).panes as
                       *mut window_panes)).tqh_first;
            while wp != 0 as *mut libc::c_void as *mut window_pane {
                if 0 != (*wp).flags & 1i32 {
                    needed = 1i32;
                    break ;
                } else { wp = (*wp).entry.tqe_next }
            }
        }
        if 0 != needed &&
               {
                   left = evbuffer_get_length((*tty).out);
                   left != 0i32 as libc::c_ulong
               } {
            log_debug(b"%s: redraw deferred (%zu left)\x00" as *const u8 as
                          *const libc::c_char, (*c).name, left);
            if 0 == event_initialized(&mut ev as *mut event) {
                event_set(&mut ev as *mut event, 1i32.wrapping_neg(),
                          0i32 as libc::c_short,
                          Some(server_client_redraw_timer),
                          0 as *mut libc::c_void);
            }
            if 0 ==
                   event_pending(&mut ev as *mut event, 1i32 as libc::c_short,
                                 0 as *mut timeval) {
                log_debug(b"redraw timer started\x00" as *const u8 as
                              *const libc::c_char);
                event_add(&mut ev as *mut event, &mut tv as *mut timeval);
            }
            (*c).flags |= 8i32;
            return
        } else {
            if 0 != needed {
                log_debug(b"%s: redraw needed\x00" as *const u8 as
                              *const libc::c_char, (*c).name);
            }
            if 0 != (*c).flags & (8i32 | 16i32) {
                if 0 !=
                       options_get_number((*s).options,
                                          b"set-titles\x00" as *const u8 as
                                              *const libc::c_char) {
                    server_client_set_title(c);
                }
                screen_redraw_update(c);
            }
            flags = (*tty).flags & (128i32 | 2i32 | 1i32);
            (*tty).flags = (*tty).flags & !(128i32 | 2i32) | 1i32;
            if 0 != (*c).flags & 8i32 {
                tty_update_mode(tty, (*tty).mode, 0 as *mut screen);
                screen_redraw_screen(c, 1i32, 1i32, 1i32);
                (*c).flags &= !(16i32 | 1024i32)
            } else {
                wp =
                    (*(&mut (*(*(*(*c).session).curw).window).panes as
                           *mut window_panes)).tqh_first;
                while wp != 0 as *mut libc::c_void as *mut window_pane {
                    if 0 != (*wp).flags & 1i32 {
                        tty_update_mode(tty, (*tty).mode, 0 as *mut screen);
                        screen_redraw_pane(c, wp);
                    }
                    wp = (*wp).entry.tqe_next
                }
            }
            masked = (*c).flags & (1024i32 | 16i32);
            if masked != 0i32 {
                tty_update_mode(tty, (*tty).mode, 0 as *mut screen);
            }
            if masked == 1024i32 {
                screen_redraw_screen(c, 0i32, 0i32, 1i32);
            } else if masked == 16i32 {
                screen_redraw_screen(c, 0i32, 1i32, 0i32);
            } else if masked != 0i32 {
                screen_redraw_screen(c, 0i32, 1i32, 1i32);
            }
            (*tty).flags = (*tty).flags & !(2i32 | 1i32) | flags;
            tty_update_mode(tty, (*tty).mode, 0 as *mut screen);
            (*c).flags &= !(8i32 | 1024i32 | 16i32 | 524288i32);
            if 0 != needed {
                (*c).redraw = evbuffer_get_length((*tty).out);
                log_debug(b"%s: redraw added %zu bytes\x00" as *const u8 as
                              *const libc::c_char, (*c).name, (*c).redraw);
            }
            return;
        }
    };
}
unsafe extern "C" fn server_client_set_title(mut c: *mut client) -> () {
    let mut s: *mut session = (*c).session;
    let mut template: *const libc::c_char = 0 as *const libc::c_char;
    let mut title: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    template =
        options_get_string((*s).options,
                           b"set-titles-string\x00" as *const u8 as
                               *const libc::c_char);
    ft = format_create(c, 0 as *mut cmdq_item, 0i32, 0i32);
    format_defaults(ft, c, 0 as *mut session, 0 as *mut winlink,
                    0 as *mut window_pane);
    title = format_expand_time(ft, template, time(0 as *mut time_t));
    if (*c).title == 0 as *mut libc::c_void as *mut libc::c_char ||
           strcmp(title, (*c).title) != 0i32 {
        free((*c).title as *mut libc::c_void);
        (*c).title = xstrdup(title);
        tty_set_title(&mut (*c).tty as *mut tty, (*c).title);
    }
    free(title as *mut libc::c_void);
    format_free(ft);
}
unsafe extern "C" fn server_client_redraw_timer(mut fd: libc::c_int,
                                                mut events: libc::c_short,
                                                mut data: *mut libc::c_void)
 -> () {
    log_debug(b"redraw timer fired\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn server_client_check_exit(mut c: *mut client) -> () {
    if 0 == (*c).flags & 4i32 {
        return
    } else if evbuffer_get_length((*c).stdin_data) != 0i32 as libc::c_ulong {
        return
    } else if evbuffer_get_length((*c).stdout_data) != 0i32 as libc::c_ulong {
        return
    } else if evbuffer_get_length((*c).stderr_data) != 0i32 as libc::c_ulong {
        return
    } else {
        if 0 != (*c).flags & 128i32 {
            notify_client(b"client-detached\x00" as *const u8 as
                              *const libc::c_char, c);
        }
        proc_send((*c).peer, MSG_EXIT, 1i32.wrapping_neg(),
                  &mut (*c).retval as *mut libc::c_int as *const libc::c_void,
                  ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
        (*c).flags &= !4i32;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_client_push_stdout(mut c: *mut client) -> () {
    let mut data: msg_stdout_data =
        msg_stdout_data{size: 0, data: [0; 8192],};
    let mut sent: size_t = 0;
    let mut left: size_t = 0;
    left = evbuffer_get_length((*c).stdout_data);
    while left != 0i32 as libc::c_ulong {
        sent = left;
        if sent >
               ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
           {
            sent =
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
        }
        memcpy(data.data.as_mut_ptr() as *mut libc::c_void,
               evbuffer_pullup((*c).stdout_data,
                               1i32.wrapping_neg() as ssize_t) as
                   *const libc::c_void, sent);
        data.size = sent as ssize_t;
        if proc_send((*c).peer, MSG_STDOUT, 1i32.wrapping_neg(),
                     &mut data as *mut msg_stdout_data as *const libc::c_void,
                     ::std::mem::size_of::<msg_stdout_data>() as
                         libc::c_ulong) != 0i32 {
            break ;
        }
        evbuffer_drain((*c).stdout_data, sent);
        left = evbuffer_get_length((*c).stdout_data);
        log_debug(b"%s: client %p, sent %zu, left %zu\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 26],
                                            &[libc::c_char; 26]>(b"server_client_push_stdout\x00")).as_ptr(),
                  c, sent, left);
    }
    if left != 0i32 as libc::c_ulong {
        (*c).references += 1;
        event_once(1i32.wrapping_neg(), 1i32 as libc::c_short,
                   Some(server_client_stdout_cb), c as *mut libc::c_void,
                   0 as *const timeval);
        log_debug(b"%s: client %p, queued\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 26],
                                            &[libc::c_char; 26]>(b"server_client_push_stdout\x00")).as_ptr(),
                  c);
    };
}
unsafe extern "C" fn server_client_stdout_cb(mut fd: libc::c_int,
                                             mut events: libc::c_short,
                                             mut arg: *mut libc::c_void)
 -> () {
    let mut c: *mut client = arg as *mut client;
    if 0 != !(*c).flags & 512i32 { server_client_push_stdout(c); }
    server_client_unref(c);
}
#[no_mangle]
pub unsafe extern "C" fn server_client_push_stderr(mut c: *mut client) -> () {
    let mut data: msg_stderr_data =
        msg_stderr_data{size: 0, data: [0; 8192],};
    let mut sent: size_t = 0;
    let mut left: size_t = 0;
    if (*c).stderr_data == (*c).stdout_data {
        server_client_push_stdout(c);
        return
    } else {
        left = evbuffer_get_length((*c).stderr_data);
        while left != 0i32 as libc::c_ulong {
            sent = left;
            if sent >
                   ::std::mem::size_of::<[libc::c_char; 8192]>() as
                       libc::c_ulong {
                sent =
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as
                        libc::c_ulong
            }
            memcpy(data.data.as_mut_ptr() as *mut libc::c_void,
                   evbuffer_pullup((*c).stderr_data,
                                   1i32.wrapping_neg() as ssize_t) as
                       *const libc::c_void, sent);
            data.size = sent as ssize_t;
            if proc_send((*c).peer, MSG_STDERR, 1i32.wrapping_neg(),
                         &mut data as *mut msg_stderr_data as
                             *const libc::c_void,
                         ::std::mem::size_of::<msg_stderr_data>() as
                             libc::c_ulong) != 0i32 {
                break ;
            }
            evbuffer_drain((*c).stderr_data, sent);
            left = evbuffer_get_length((*c).stderr_data);
            log_debug(b"%s: client %p, sent %zu, left %zu\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"server_client_push_stderr\x00")).as_ptr(),
                      c, sent, left);
        }
        if left != 0i32 as libc::c_ulong {
            (*c).references += 1;
            event_once(1i32.wrapping_neg(), 1i32 as libc::c_short,
                       Some(server_client_stderr_cb), c as *mut libc::c_void,
                       0 as *const timeval);
            log_debug(b"%s: client %p, queued\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"server_client_push_stderr\x00")).as_ptr(),
                      c);
        }
        return;
    };
}
unsafe extern "C" fn server_client_stderr_cb(mut fd: libc::c_int,
                                             mut events: libc::c_short,
                                             mut arg: *mut libc::c_void)
 -> () {
    let mut c: *mut client = arg as *mut client;
    if 0 != !(*c).flags & 512i32 { server_client_push_stderr(c); }
    server_client_unref(c);
}
#[no_mangle]
pub unsafe extern "C" fn server_client_get_path(mut c: *mut client,
                                                mut file: *const libc::c_char)
 -> *mut libc::c_char {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut resolved: [libc::c_char; 4096] = [0; 4096];
    if *file as libc::c_int == 47 {
        path = xstrdup(file)
    } else {
        xasprintf(&mut path as *mut *mut libc::c_char,
                  b"%s/%s\x00" as *const u8 as *const libc::c_char,
                  server_client_get_cwd(c), file);
    }
    if realpath(path, resolved.as_mut_ptr()) ==
           0 as *mut libc::c_void as *mut libc::c_char {
        return path
    } else {
        free(path as *mut libc::c_void);
        return xstrdup(resolved.as_mut_ptr())
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_client_get_cwd(mut c: *mut client)
 -> *const libc::c_char {
    let mut s: *mut session = 0 as *mut session;
    if c != 0 as *mut libc::c_void as *mut client &&
           (*c).session == 0 as *mut libc::c_void as *mut session &&
           (*c).cwd != 0 as *mut libc::c_void as *const libc::c_char {
        return (*c).cwd
    } else if c != 0 as *mut libc::c_void as *mut client &&
                  {
                      s = (*c).session;
                      s != 0 as *mut libc::c_void as *mut session
                  } &&
                  (*s).cwd != 0 as *mut libc::c_void as *const libc::c_char {
        return (*s).cwd
    } else { return b".\x00" as *const u8 as *const libc::c_char };
}

