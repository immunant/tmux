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
    pub type _IO_FILE_plus;
    pub type tmuxproc;
    pub type tmuxpeer;
    pub type event_base;
    pub type environ;
    pub type hooks;
    pub type input_ctx;
    pub type tty_code;
    pub type evbuffer;
    pub type options;
    pub type paste_buffer;
    pub type screen_titles;
    pub type args_entry;
    pub type options_entry;
    pub type bufferevent_ops;
    #[no_mangle]
    static _sys_siglist: [*const libc::c_char; 65];
    #[no_mangle]
    static sys_siglist: [*const libc::c_char; 65];
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fnmatch(__pattern: *const libc::c_char, __name: *const libc::c_char,
               __flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn ctime_r(__timer: *const time_t, __buf: *mut libc::c_char)
     -> *mut libc::c_char;
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
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
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
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t)
     -> *mut libc::c_uchar;
    #[no_mangle]
    fn evbuffer_readline(buffer: *mut evbuffer) -> *mut libc::c_char;
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
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xstrndup(_: *const libc::c_char, _: size_t) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, ...)
     -> libc::c_int;
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
    fn paste_buffer_name(_: *mut paste_buffer) -> *const libc::c_char;
    #[no_mangle]
    fn paste_buffer_created(_: *mut paste_buffer) -> time_t;
    #[no_mangle]
    fn paste_buffer_data(_: *mut paste_buffer, _: *mut size_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn paste_make_sample(_: *mut paste_buffer) -> *mut libc::c_char;
    #[no_mangle]
    fn format_add(_: *mut format_tree, _: *const libc::c_char,
                  _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn job_free(_: *mut job) -> ();
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn server_client_unref(_: *mut client) -> ();
    #[no_mangle]
    fn utf8_rtrimcstr(_: *const libc::c_char, _: u_int) -> *mut libc::c_char;
    #[no_mangle]
    fn utf8_trimcstr(_: *const libc::c_char, _: u_int) -> *mut libc::c_char;
    #[no_mangle]
    fn environ_find(_: *mut environ, _: *const libc::c_char)
     -> *mut environ_entry;
    #[no_mangle]
    fn options_tostring(_: *mut options_entry, _: libc::c_int, _: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn options_parse_get(_: *mut options, _: *const libc::c_char,
                         _: *mut libc::c_int, _: libc::c_int)
     -> *mut options_entry;
    #[no_mangle]
    fn window_pane_search(_: *mut window_pane, _: *const libc::c_char)
     -> u_int;
    #[no_mangle]
    fn server_status_client(_: *mut client) -> ();
    #[no_mangle]
    fn job_run(_: *const libc::c_char, _: *mut session,
               _: *const libc::c_char, _: job_update_cb, _: job_complete_cb,
               _: job_free_cb, _: *mut libc::c_void, _: libc::c_int)
     -> *mut job;
    #[no_mangle]
    fn window_copy_add_formats(_: *mut window_pane, _: *mut format_tree)
     -> ();
    #[no_mangle]
    fn osdep_get_cwd(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn parse_window_name(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn cmd_stringify_argv(_: libc::c_int, _: *mut *mut libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn osdep_get_name(_: libc::c_int, _: *mut libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    fn window_pane_visible(_: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn window_pane_index(_: *mut window_pane, _: *mut u_int) -> libc::c_int;
    #[no_mangle]
    fn session_is_linked(_: *mut session, _: *mut window) -> libc::c_int;
    #[no_mangle]
    fn window_printable_flags(_: *mut winlink) -> *const libc::c_char;
    #[no_mangle]
    fn window_count_panes(_: *mut window) -> u_int;
    #[no_mangle]
    fn layout_dump(_: *mut layout_cell) -> *mut libc::c_char;
    #[no_mangle]
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn session_group_contains(_: *mut session) -> *mut session_group;
    #[no_mangle]
    fn session_group_count(_: *mut session_group) -> u_int;
    #[no_mangle]
    fn winlink_count(_: *mut winlinks) -> u_int;
    #[no_mangle]
    fn session_alive(_: *mut session) -> libc::c_int;
    #[no_mangle]
    fn server_client_get_key_table(_: *mut client) -> *const libc::c_char;
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
    static window_copy_mode: window_mode;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    static mut session_groups: session_groups;
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
pub const JOB_CLOSED: unnamed_41 = 2;
pub const JOB_DEAD: unnamed_41 = 1;
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
    pub entry: unnamed_2,
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
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub const TTY_VT100: unnamed_26 = 0;
pub type cc_t = libc::c_uchar;
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
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    ev_next_with_common_timeout: unnamed_33,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_29,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type format_cb =
    Option<unsafe extern "C" fn(_: *mut format_tree, _: *mut format_entry)
               -> ()>;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const LINE_SEL_LEFT_RIGHT: unnamed_21 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub const TTY_VT102: unnamed_26 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_43,
    pub entry: unnamed_30,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_13,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const PROMPT_COMMAND: unnamed_12 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
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
    pub entry: unnamed_42,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct format_entry_tree {
    pub rbh_root: *mut format_entry,
}
pub type cmd_retval = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
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
    pub term_type: unnamed_26,
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
pub const JOB_RUNNING: unnamed_41 = 0;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
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
pub struct event {
    pub ev_active_next: unnamed_1,
    pub ev_next: unnamed_36,
    pub ev_timeout_pos: unnamed,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_18,
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
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct format_tree {
    pub w: *mut window,
    pub wl: *mut winlink,
    pub s: *mut session,
    pub wp: *mut window_pane,
    pub client: *mut client,
    pub tag: u_int,
    pub flags: libc::c_int,
    pub tree: format_entry_tree,
}
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub rbe_left: *mut format_job,
    pub rbe_right: *mut format_job,
    pub rbe_parent: *mut format_job,
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
    pub entry: unnamed_3,
    pub wentry: unnamed_19,
    pub sentry: unnamed_16,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub ev_io_next: unnamed_14,
    pub ev_timeout: timeval,
}
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct format_job {
    pub client: *mut client,
    pub tag: u_int,
    pub cmd: *const libc::c_char,
    pub expanded: *const libc::c_char,
    pub last: time_t,
    pub out: *mut libc::c_char,
    pub updated: libc::c_int,
    pub job: *mut job,
    pub status: libc::c_int,
    pub entry: unnamed_5,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type __ssize_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
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
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_21,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_9 {
    __in: libc::c_int,
    __i: libc::c_int,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type cmdq_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_11 {
    offset: u_int,
    data: unnamed_25,
}
pub type unnamed_12 = libc::c_uint;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
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
pub type layout_type = libc::c_uint;
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
pub struct unnamed_15 {
    pub ev_signal_next: unnamed_23,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct format_entry {
    pub key: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub t: time_t,
    pub cb: format_cb,
    pub entry: unnamed_31,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const PROMPT_ENTRY: unnamed_12 = 0;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub type u_int = __u_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_18 {
    ev_io: unnamed_6,
    ev_signal: unnamed_15,
}
pub const LINE_SEL_NONE: unnamed_21 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_34,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const TTY_VT220: unnamed_26 = 3;
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub type unnamed_21 = libc::c_uint;
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
pub struct format_job_tree {
    pub rbh_root: *mut format_job,
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
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_28,
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
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type pid_t = __pid_t;
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
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub type ssize_t = __ssize_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const TTY_VT420: unnamed_26 = 5;
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
pub struct unnamed_23 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type tcflag_t = libc::c_uint;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_10,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_11,
}
pub type unnamed_26 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
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
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub rbe_left: *mut format_entry,
    pub rbe_right: *mut format_entry,
    pub rbe_parent: *mut format_entry,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_32 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub type __off_t = libc::c_long;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub type time_t = __time_t;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const TTY_UNKNOWN: unnamed_26 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub type u_char = __u_char;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
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
    pub entry: unnamed_7,
    pub tree_entry: unnamed_35,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type speed_t = libc::c_uint;
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub type __suseconds_t = libc::c_long;
pub const LINE_SEL_RIGHT_LEFT: unnamed_21 = 2;
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
    pub gentry: unnamed_39,
    pub entry: unnamed_0,
}
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct environ_entry {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub entry: unnamed_22,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
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
    pub message_log: unnamed_4,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_12,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_38,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_32,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub type bitstr_t = libc::c_uchar;
pub type options_table_type = libc::c_uint;
pub const TTY_VT320: unnamed_26 = 4;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_8,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_41,
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
    pub entry: unnamed_20,
}
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_40 {
    __in: libc::c_int,
    __i: libc::c_int,
}
pub type unnamed_41 = libc::c_uint;
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
    pub alerts_entry: unnamed_37,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_24,
    pub entry: unnamed_17,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_42 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_43 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const TTY_VT101: unnamed_26 = 1;
#[no_mangle]
pub unsafe extern "C" fn format_true(mut s: *const libc::c_char)
 -> libc::c_int {
    if s != 0 as *mut libc::c_void as *const libc::c_char &&
           *s as libc::c_int != 0 &&
           (*s.offset(0isize) as libc::c_int != 48 ||
                *s.offset(1isize) as libc::c_int != 0) {
        return 1i32
    } else { return 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn format_create(mut c: *mut client,
                                       mut item: *mut cmdq_item,
                                       mut tag: libc::c_int,
                                       mut flags: libc::c_int)
 -> *mut format_tree {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    if 0 == event_initialized(&mut format_job_event as *mut event) {
        event_set(&mut format_job_event as *mut event, 1i32.wrapping_neg(),
                  0i32 as libc::c_short, Some(format_job_timer),
                  0 as *mut libc::c_void);
        format_job_timer(1i32.wrapping_neg(), 0i32 as libc::c_short,
                         0 as *mut libc::c_void);
    }
    ft =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<format_tree>() as libc::c_ulong) as
            *mut format_tree;
    loop  {
        let ref mut fresh0 =
            (*(&mut (*ft).tree as *mut format_entry_tree)).rbh_root;
        *fresh0 = 0 as *mut format_entry;
        if !(0 != 0i32) { break ; }
    }
    if c != 0 as *mut libc::c_void as *mut client {
        (*ft).client = c;
        (*(*ft).client).references += 1
    }
    (*ft).tag = tag as u_int;
    (*ft).flags = flags;
    format_add(ft, b"version\x00" as *const u8 as *const libc::c_char,
               b"%s\x00" as *const u8 as *const libc::c_char,
               b"xmaster-rs\x00" as *const u8 as *const libc::c_char);
    format_add_cb(ft, b"host\x00" as *const u8 as *const libc::c_char,
                  Some(format_cb_host));
    format_add_cb(ft, b"host_short\x00" as *const u8 as *const libc::c_char,
                  Some(format_cb_host_short));
    format_add_cb(ft, b"pid\x00" as *const u8 as *const libc::c_char,
                  Some(format_cb_pid));
    format_add(ft, b"socket_path\x00" as *const u8 as *const libc::c_char,
               b"%s\x00" as *const u8 as *const libc::c_char, socket_path);
    format_add_tv(ft, b"start_time\x00" as *const u8 as *const libc::c_char,
                  &mut start_time as *mut timeval);
    if item != 0 as *mut libc::c_void as *mut cmdq_item {
        if (*item).cmd != 0 as *mut libc::c_void as *mut cmd {
            format_add(ft, b"command\x00" as *const u8 as *const libc::c_char,
                       b"%s\x00" as *const u8 as *const libc::c_char,
                       (*(*(*item).cmd).entry).name);
        }
        if (*item).shared != 0 as *mut libc::c_void as *mut cmdq_shared &&
               (*(*item).shared).formats !=
                   0 as *mut libc::c_void as *mut format_tree {
            format_merge(ft, (*(*item).shared).formats);
        }
    }
    return ft;
}
unsafe extern "C" fn format_merge(mut ft: *mut format_tree,
                                  mut from: *mut format_tree) -> () {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    fe =
        format_entry_tree_RB_MINMAX(&mut (*from).tree as
                                        *mut format_entry_tree,
                                    1i32.wrapping_neg());
    while fe != 0 as *mut libc::c_void as *mut format_entry {
        if (*fe).value != 0 as *mut libc::c_void as *mut libc::c_char {
            format_add(ft, (*fe).key,
                       b"%s\x00" as *const u8 as *const libc::c_char,
                       (*fe).value);
        }
        fe = format_entry_tree_RB_NEXT(fe)
    };
}
unsafe extern "C" fn format_entry_tree_RB_NEXT(mut elm: *mut format_entry)
 -> *mut format_entry {
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
unsafe extern "C" fn format_entry_tree_RB_MINMAX(mut head:
                                                     *mut format_entry_tree,
                                                 mut val: libc::c_int)
 -> *mut format_entry {
    let mut tmp: *mut format_entry = (*head).rbh_root;
    let mut parent: *mut format_entry = 0 as *mut format_entry;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else { tmp = (*tmp).entry.rbe_right }
    }
    return parent;
}
unsafe extern "C" fn format_add_tv(mut ft: *mut format_tree,
                                   mut key: *const libc::c_char,
                                   mut tv: *mut timeval) -> () {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe_now: *mut format_entry = 0 as *mut format_entry;
    fe =
        xmalloc(::std::mem::size_of::<format_entry>() as libc::c_ulong) as
            *mut format_entry;
    (*fe).key = xstrdup(key);
    fe_now =
        format_entry_tree_RB_INSERT(&mut (*ft).tree as *mut format_entry_tree,
                                    fe);
    if fe_now != 0 as *mut libc::c_void as *mut format_entry {
        free((*fe).key as *mut libc::c_void);
        free(fe as *mut libc::c_void);
        free((*fe_now).value as *mut libc::c_void);
        fe = fe_now
    }
    (*fe).cb = None;
    (*fe).t = (*tv).tv_sec;
    (*fe).value = 0 as *mut libc::c_char;
}
unsafe extern "C" fn format_entry_tree_RB_INSERT(mut head:
                                                     *mut format_entry_tree,
                                                 mut elm: *mut format_entry)
 -> *mut format_entry {
    let mut tmp: *mut format_entry = 0 as *mut format_entry;
    let mut parent: *mut format_entry = 0 as *mut format_entry;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = format_entry_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    loop  {
        (*elm).entry.rbe_parent = parent;
        (*elm).entry.rbe_right = 0 as *mut format_entry;
        (*elm).entry.rbe_left = (*elm).entry.rbe_right;
        (*elm).entry.rbe_color = 1i32;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut format_entry {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else { (*parent).entry.rbe_right = elm }
        while 0 != 0i32 { }
    } else { (*head).rbh_root = elm }
    format_entry_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut format_entry;
}
unsafe extern "C" fn format_entry_tree_RB_INSERT_COLOR(mut head:
                                                           *mut format_entry_tree,
                                                       mut elm:
                                                           *mut format_entry)
 -> () {
    let mut current_block: u64;
    let mut parent: *mut format_entry = 0 as *mut format_entry;
    let mut gparent: *mut format_entry = 0 as *mut format_entry;
    let mut tmp: *mut format_entry = 0 as *mut format_entry;
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
                's_38:
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
                                                continue 's_38 ;
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
unsafe extern "C" fn format_entry_cmp(mut fe1: *mut format_entry,
                                      mut fe2: *mut format_entry)
 -> libc::c_int {
    return strcmp((*fe1).key, (*fe2).key);
}
unsafe extern "C" fn format_cb_pid(mut ft: *mut format_tree,
                                   mut fe: *mut format_entry) -> () {
    xasprintf(&mut (*fe).value as *mut *mut libc::c_char,
              b"%ld\x00" as *const u8 as *const libc::c_char,
              getpid() as libc::c_long);
}
unsafe extern "C" fn format_add_cb(mut ft: *mut format_tree,
                                   mut key: *const libc::c_char,
                                   mut cb: format_cb) -> () {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe_now: *mut format_entry = 0 as *mut format_entry;
    fe =
        xmalloc(::std::mem::size_of::<format_entry>() as libc::c_ulong) as
            *mut format_entry;
    (*fe).key = xstrdup(key);
    fe_now =
        format_entry_tree_RB_INSERT(&mut (*ft).tree as *mut format_entry_tree,
                                    fe);
    if fe_now != 0 as *mut libc::c_void as *mut format_entry {
        free((*fe).key as *mut libc::c_void);
        free(fe as *mut libc::c_void);
        free((*fe_now).value as *mut libc::c_void);
        fe = fe_now
    }
    (*fe).cb = cb;
    (*fe).t = 0i32 as time_t;
    (*fe).value = 0 as *mut libc::c_char;
}
unsafe extern "C" fn format_cb_host_short(mut ft: *mut format_tree,
                                          mut fe: *mut format_entry) -> () {
    let mut host: [libc::c_char; 65] = [0; 65];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if gethostname(host.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 65]>() as
                       libc::c_ulong) != 0i32 {
        (*fe).value = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
    } else {
        cp = strchr(host.as_mut_ptr(), 46);
        if cp != 0 as *mut libc::c_void as *mut libc::c_char {
            *cp = 0 as libc::c_char
        }
        (*fe).value = xstrdup(host.as_mut_ptr())
    };
}
unsafe extern "C" fn format_cb_host(mut ft: *mut format_tree,
                                    mut fe: *mut format_entry) -> () {
    let mut host: [libc::c_char; 65] = [0; 65];
    if gethostname(host.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 65]>() as
                       libc::c_ulong) != 0i32 {
        (*fe).value = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
    } else { (*fe).value = xstrdup(host.as_mut_ptr()) };
}
unsafe extern "C" fn format_job_timer(mut fd: libc::c_int,
                                      mut events: libc::c_short,
                                      mut arg: *mut libc::c_void) -> () {
    let mut c: *mut client = 0 as *mut client;
    let mut tv: timeval = timeval{tv_sec: 60i32 as __time_t, tv_usec: 0,};
    format_job_tidy(&mut format_jobs as *mut format_job_tree, 0i32);
    c = (*(&mut clients as *mut clients)).tqh_first;
    while c != 0 as *mut libc::c_void as *mut client {
        if (*c).jobs != 0 as *mut libc::c_void as *mut format_job_tree {
            format_job_tidy((*c).jobs, 0i32);
        }
        c = (*c).entry.tqe_next
    }
    event_del(&mut format_job_event as *mut event);
    event_add(&mut format_job_event as *mut event, &mut tv as *mut timeval);
}
static mut format_job_event: event =
    unsafe {
        event{ev_active_next:
                  unnamed_1{tqe_next: 0 as *const event as *mut event,
                            tqe_prev:
                                0 as *const *mut event as *mut *mut event,},
              ev_next:
                  unnamed_36{tqe_next: 0 as *const event as *mut event,
                             tqe_prev:
                                 0 as *const *mut event as *mut *mut event,},
              ev_timeout_pos:
                  unnamed{ev_next_with_common_timeout:
                              unnamed_33{tqe_next:
                                             0 as *const event as *mut event,
                                         tqe_prev:
                                             0 as *const *mut event as
                                                 *mut *mut event,},},
              ev_fd: 0,
              ev_base: 0 as *const event_base as *mut event_base,
              _ev:
                  unnamed_18{ev_io:
                                 unnamed_6{ev_io_next:
                                               unnamed_14{tqe_next:
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
unsafe extern "C" fn format_job_tidy(mut jobs: *mut format_job_tree,
                                     mut force: libc::c_int) -> () {
    let mut fj: *mut format_job = 0 as *mut format_job;
    let mut fj1: *mut format_job = 0 as *mut format_job;
    let mut now: time_t = 0;
    now = time(0 as *mut time_t);
    fj = format_job_tree_RB_MINMAX(jobs, 1i32.wrapping_neg());
    while fj != 0 as *mut libc::c_void as *mut format_job &&
              { fj1 = format_job_tree_RB_NEXT(fj); 0 != 1i32 } {
        if !(0 == force &&
                 ((*fj).last > now ||
                      now - (*fj).last < 3600i32 as libc::c_long)) {
            format_job_tree_RB_REMOVE(jobs, fj);
            log_debug(b"%s: %s\x00" as *const u8 as *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"format_job_tidy\x00")).as_ptr(),
                      (*fj).cmd);
            if (*fj).job != 0 as *mut libc::c_void as *mut job {
                job_free((*fj).job);
            }
            free((*fj).expanded as *mut libc::c_void);
            free((*fj).cmd as *mut libc::c_void);
            free((*fj).out as *mut libc::c_void);
            free(fj as *mut libc::c_void);
        }
        fj = fj1
    };
}
unsafe extern "C" fn format_job_tree_RB_REMOVE(mut head: *mut format_job_tree,
                                               mut elm: *mut format_job)
 -> *mut format_job {
    let mut current_block: u64;
    let mut child: *mut format_job = 0 as *mut format_job;
    let mut parent: *mut format_job = 0 as *mut format_job;
    let mut old: *mut format_job = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left == 0 as *mut libc::c_void as *mut format_job {
        child = (*elm).entry.rbe_right;
        current_block = 9386390421034826751;
    } else if (*elm).entry.rbe_right ==
                  0 as *mut libc::c_void as *mut format_job {
        child = (*elm).entry.rbe_left;
        current_block = 9386390421034826751;
    } else {
        let mut left: *mut format_job = 0 as *mut format_job;
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
            current_block = 4783745246848806834;
        } else { current_block = 4783745246848806834; }
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
    if color == 0i32 { format_job_tree_RB_REMOVE_COLOR(head, parent, child); }
    return old;
}
unsafe extern "C" fn format_job_tree_RB_REMOVE_COLOR(mut head:
                                                         *mut format_job_tree,
                                                     mut parent:
                                                         *mut format_job,
                                                     mut elm: *mut format_job)
 -> () {
    let mut current_block: u64;
    let mut tmp: *mut format_job = 0 as *mut format_job;
    's_4:
        loop  {
            if !((elm == 0 as *mut libc::c_void as *mut format_job ||
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
                        14155750587950065367 => {
                            if ((*tmp).entry.rbe_left ==
                                    0 as *mut libc::c_void as *mut format_job
                                    ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut format_job ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_right ==
                                          0 as *mut libc::c_void as
                                              *mut format_job ||
                                          (*(*tmp).entry.rbe_right).entry.rbe_color
                                              == 0i32 {
                                current_block = 15976848397966268834;
                                break 's_4 ;
                            } else {
                                current_block = 7149356873433890176;
                                break 's_4 ;
                            }
                        }
                        _ => {
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
                                    0 as *mut libc::c_void as *mut format_job
                                    ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut format_job ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_left ==
                                          0 as *mut libc::c_void as
                                              *mut format_job ||
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
            let mut oleft: *mut format_job = 0 as *mut format_job;
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
        13826291924415791078 => {
            let mut oright: *mut format_job = 0 as *mut format_job;
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
unsafe extern "C" fn format_job_tree_RB_NEXT(mut elm: *mut format_job)
 -> *mut format_job {
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
unsafe extern "C" fn format_job_tree_RB_MINMAX(mut head: *mut format_job_tree,
                                               mut val: libc::c_int)
 -> *mut format_job {
    let mut tmp: *mut format_job = (*head).rbh_root;
    let mut parent: *mut format_job = 0 as *mut format_job;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else { tmp = (*tmp).entry.rbe_right }
    }
    return parent;
}
static mut format_jobs: format_job_tree =
    unsafe {
        format_job_tree{rbh_root: 0 as *const format_job as *mut format_job,}
    };
#[no_mangle]
pub unsafe extern "C" fn format_free(mut ft: *mut format_tree) -> () {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe1: *mut format_entry = 0 as *mut format_entry;
    fe =
        format_entry_tree_RB_MINMAX(&mut (*ft).tree as *mut format_entry_tree,
                                    1i32.wrapping_neg());
    while fe != 0 as *mut libc::c_void as *mut format_entry &&
              { fe1 = format_entry_tree_RB_NEXT(fe); 0 != 1i32 } {
        format_entry_tree_RB_REMOVE(&mut (*ft).tree as *mut format_entry_tree,
                                    fe);
        free((*fe).value as *mut libc::c_void);
        free((*fe).key as *mut libc::c_void);
        free(fe as *mut libc::c_void);
        fe = fe1
    }
    if (*ft).client != 0 as *mut libc::c_void as *mut client {
        server_client_unref((*ft).client);
    }
    free(ft as *mut libc::c_void);
}
unsafe extern "C" fn format_entry_tree_RB_REMOVE(mut head:
                                                     *mut format_entry_tree,
                                                 mut elm: *mut format_entry)
 -> *mut format_entry {
    let mut current_block: u64;
    let mut child: *mut format_entry = 0 as *mut format_entry;
    let mut parent: *mut format_entry = 0 as *mut format_entry;
    let mut old: *mut format_entry = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left == 0 as *mut libc::c_void as *mut format_entry {
        child = (*elm).entry.rbe_right;
        current_block = 9386390421034826751;
    } else if (*elm).entry.rbe_right ==
                  0 as *mut libc::c_void as *mut format_entry {
        child = (*elm).entry.rbe_left;
        current_block = 9386390421034826751;
    } else {
        let mut left: *mut format_entry = 0 as *mut format_entry;
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
            current_block = 9209585778364289773;
        } else { current_block = 9209585778364289773; }
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
    if color == 0i32 {
        format_entry_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn format_entry_tree_RB_REMOVE_COLOR(mut head:
                                                           *mut format_entry_tree,
                                                       mut parent:
                                                           *mut format_entry,
                                                       mut elm:
                                                           *mut format_entry)
 -> () {
    let mut current_block: u64;
    let mut tmp: *mut format_entry = 0 as *mut format_entry;
    's_4:
        loop  {
            if !((elm == 0 as *mut libc::c_void as *mut format_entry ||
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
                        14155750587950065367 => {
                            if ((*tmp).entry.rbe_left ==
                                    0 as *mut libc::c_void as
                                        *mut format_entry ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut format_entry ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_right ==
                                          0 as *mut libc::c_void as
                                              *mut format_entry ||
                                          (*(*tmp).entry.rbe_right).entry.rbe_color
                                              == 0i32 {
                                current_block = 15976848397966268834;
                                break 's_4 ;
                            } else {
                                current_block = 7149356873433890176;
                                break 's_4 ;
                            }
                        }
                        _ => {
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
                    }
                }
            } else {
                tmp = (*parent).entry.rbe_left;
                if (*tmp).entry.rbe_color == 1i32 {
                    current_block = 6450597802325118133;
                } else { current_block = 7746103178988627676; }
                loop  {
                    match current_block {
                        7746103178988627676 => {
                            if ((*tmp).entry.rbe_left ==
                                    0 as *mut libc::c_void as
                                        *mut format_entry ||
                                    (*(*tmp).entry.rbe_left).entry.rbe_color
                                        == 0i32) &&
                                   ((*tmp).entry.rbe_right ==
                                        0 as *mut libc::c_void as
                                            *mut format_entry ||
                                        (*(*tmp).entry.rbe_right).entry.rbe_color
                                            == 0i32) {
                                (*tmp).entry.rbe_color = 1i32;
                                elm = parent;
                                parent = (*elm).entry.rbe_parent;
                                break ;
                            } else if (*tmp).entry.rbe_left ==
                                          0 as *mut libc::c_void as
                                              *mut format_entry ||
                                          (*(*tmp).entry.rbe_left).entry.rbe_color
                                              == 0i32 {
                                current_block = 13826291924415791078;
                                break 's_4 ;
                            } else {
                                current_block = 5892776923941496671;
                                break 's_4 ;
                            }
                        }
                        _ => {
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
                    }
                }
            }
        }
    match current_block {
        15976848397966268834 => {
            let mut oleft: *mut format_entry = 0 as *mut format_entry;
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
            let mut oright: *mut format_entry = 0 as *mut format_entry;
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
        _ => { }
    }
    if !elm.is_null() { (*elm).entry.rbe_color = 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn format_expand_time(mut ft: *mut format_tree,
                                            mut fmt: *const libc::c_char,
                                            mut t: time_t)
 -> *mut libc::c_char {
    let mut tm: *mut tm = 0 as *mut tm;
    let mut s: [libc::c_char; 2048] = [0; 2048];
    if fmt == 0 as *mut libc::c_void as *const libc::c_char ||
           *fmt as libc::c_int == 0 {
        return xstrdup(b"\x00" as *const u8 as *const libc::c_char)
    } else {
        tm = localtime(&mut t as *mut time_t);
        if strftime(s.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 2048]>() as
                        libc::c_ulong, fmt, tm) == 0i32 as libc::c_ulong {
            return xstrdup(b"\x00" as *const u8 as *const libc::c_char)
        } else { return format_expand(ft, s.as_mut_ptr()) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn format_expand(mut ft: *mut format_tree,
                                       mut fmt: *const libc::c_char)
 -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut saved: *const libc::c_char = fmt;
    let mut off: size_t = 0;
    let mut len: size_t = 0;
    let mut n: size_t = 0;
    let mut outlen: size_t = 0;
    let mut ch: libc::c_int = 0;
    let mut brackets: libc::c_int = 0;
    if fmt == 0 as *mut libc::c_void as *const libc::c_char {
        return xstrdup(b"\x00" as *const u8 as *const libc::c_char)
    } else {
        len = 64i32 as size_t;
        buf = xmalloc(len) as *mut libc::c_char;
        off = 0i32 as size_t;
        while *fmt as libc::c_int != 0 {
            if *fmt as libc::c_int != 35 {
                while len.wrapping_sub(off) < 2i32 as libc::c_ulong {
                    buf =
                        xreallocarray(buf as *mut libc::c_void,
                                      2i32 as size_t, len) as
                            *mut libc::c_char;
                    len =
                        (len as
                             libc::c_ulong).wrapping_mul(2i32 as
                                                             libc::c_ulong) as
                            size_t as size_t
                }
                let fresh2 = off;
                off = off.wrapping_add(1);
                let fresh1 = fmt;
                fmt = fmt.offset(1);
                *buf.offset(fresh2 as isize) = *fresh1
            } else {
                fmt = fmt.offset(1isize);
                let fresh3 = fmt;
                fmt = fmt.offset(1);
                ch = *fresh3 as u_char as libc::c_int;
                match ch {
                    40 => {
                        brackets = 1i32;
                        ptr = fmt;
                        while *ptr as libc::c_int != 0 {
                            if *ptr as libc::c_int == 40 { brackets += 1 }
                            if *ptr as libc::c_int == 41 &&
                                   { brackets -= 1; brackets == 0i32 } {
                                break ;
                            }
                            ptr = ptr.offset(1isize)
                        }
                        if *ptr as libc::c_int != 41 || brackets != 0i32 {
                            break ;
                        }
                        n =
                            fmt.offset_to(ptr).expect("bad offset_to") as
                                libc::c_long as size_t;
                        if 0 != (*ft).flags & 4i32 {
                            out =
                                xstrdup(b"\x00" as *const u8 as
                                            *const libc::c_char)
                        } else {
                            name = xstrndup(fmt, n);
                            out = format_job_get(ft, name);
                            free(name as *mut libc::c_void);
                        }
                        outlen = strlen(out);
                        while len.wrapping_sub(off) <
                                  outlen.wrapping_add(1i32 as libc::c_ulong) {
                            buf =
                                xreallocarray(buf as *mut libc::c_void,
                                              2i32 as size_t, len) as
                                    *mut libc::c_char;
                            len =
                                (len as
                                     libc::c_ulong).wrapping_mul(2i32 as
                                                                     libc::c_ulong)
                                    as size_t as size_t
                        }
                        memcpy(buf.offset(off as isize) as *mut libc::c_void,
                               out as *const libc::c_void, outlen);
                        off =
                            (off as libc::c_ulong).wrapping_add(outlen) as
                                size_t as size_t;
                        free(out as *mut libc::c_void);
                        fmt =
                            fmt.offset(n.wrapping_add(1i32 as libc::c_ulong)
                                           as isize)
                    }
                    123 => {
                        brackets = 1i32;
                        ptr = fmt;
                        while *ptr as libc::c_int != 0 {
                            if *ptr as libc::c_int == 123 { brackets += 1 }
                            if *ptr as libc::c_int == 125 &&
                                   { brackets -= 1; brackets == 0i32 } {
                                break ;
                            }
                            ptr = ptr.offset(1isize)
                        }
                        if *ptr as libc::c_int != 125 || brackets != 0i32 {
                            break ;
                        }
                        n =
                            fmt.offset_to(ptr).expect("bad offset_to") as
                                libc::c_long as size_t;
                        if format_replace(ft, fmt, n,
                                          &mut buf as *mut *mut libc::c_char,
                                          &mut len as *mut size_t,
                                          &mut off as *mut size_t) != 0i32 {
                            break ;
                        }
                        fmt =
                            fmt.offset(n.wrapping_add(1i32 as libc::c_ulong)
                                           as isize)
                    }
                    35 => {
                        while len.wrapping_sub(off) < 2i32 as libc::c_ulong {
                            buf =
                                xreallocarray(buf as *mut libc::c_void,
                                              2i32 as size_t, len) as
                                    *mut libc::c_char;
                            len =
                                (len as
                                     libc::c_ulong).wrapping_mul(2i32 as
                                                                     libc::c_ulong)
                                    as size_t as size_t
                        }
                        let fresh4 = off;
                        off = off.wrapping_add(1);
                        *buf.offset(fresh4 as isize) = 35 as libc::c_char
                    }
                    _ => {
                        s = 0 as *const libc::c_char;
                        if ch >= 65 && ch <= 90 {
                            s = format_upper[(ch - 65) as usize]
                        } else if ch >= 97 && ch <= 122 {
                            s = format_lower[(ch - 97) as usize]
                        }
                        if s == 0 as *mut libc::c_void as *const libc::c_char
                           {
                            while len.wrapping_sub(off) <
                                      3i32 as libc::c_ulong {
                                buf =
                                    xreallocarray(buf as *mut libc::c_void,
                                                  2i32 as size_t, len) as
                                        *mut libc::c_char;
                                len =
                                    (len as
                                         libc::c_ulong).wrapping_mul(2i32 as
                                                                         libc::c_ulong)
                                        as size_t as size_t
                            }
                            let fresh5 = off;
                            off = off.wrapping_add(1);
                            *buf.offset(fresh5 as isize) = 35 as libc::c_char;
                            let fresh6 = off;
                            off = off.wrapping_add(1);
                            *buf.offset(fresh6 as isize) = ch as libc::c_char
                        } else {
                            n = strlen(s);
                            if format_replace(ft, s, n,
                                              &mut buf as
                                                  *mut *mut libc::c_char,
                                              &mut len as *mut size_t,
                                              &mut off as *mut size_t) != 0i32
                               {
                                break ;
                            }
                        }
                    }
                }
            }
        }
        *buf.offset(off as isize) = 0 as libc::c_char;
        log_debug(b"format \'%s\' -> \'%s\'\x00" as *const u8 as
                      *const libc::c_char, saved, buf);
        return buf
    };
}
unsafe extern "C" fn format_replace(mut ft: *mut format_tree,
                                    mut key: *const libc::c_char,
                                    mut keylen: size_t,
                                    mut buf: *mut *mut libc::c_char,
                                    mut len: *mut size_t,
                                    mut off: *mut size_t) -> libc::c_int {
    let mut current_block: u64;
    let mut wp: *mut window_pane = (*ft).wp;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut found: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut from: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut to: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut right: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut valuelen: size_t = 0;
    let mut newlen: size_t = 0;
    let mut fromlen: size_t = 0;
    let mut tolen: size_t = 0;
    let mut used: size_t = 0;
    let mut limit: libc::c_long = 0i32 as libc::c_long;
    let mut modifiers: libc::c_int = 0i32;
    let mut compare: libc::c_int = 0i32;
    let mut search: libc::c_int = 0i32;
    copy =
        xmalloc(keylen.wrapping_add(1i32 as libc::c_ulong)) as
            *mut libc::c_char;
    copy0 = copy;
    memcpy(copy as *mut libc::c_void, key as *const libc::c_void, keylen);
    *copy.offset(keylen as isize) = 0 as libc::c_char;
    match *copy.offset(0isize) as libc::c_int {
        109 => {
            if !(*copy.offset(1isize) as libc::c_int != 58) {
                compare = 2i32.wrapping_neg();
                copy = copy.offset(2isize)
            }
        }
        67 => {
            if !(*copy.offset(1isize) as libc::c_int != 58) {
                search = 1i32;
                copy = copy.offset(2isize)
            }
        }
        124 => {
            if !(*copy.offset(1isize) as libc::c_int != 124 ||
                     *copy.offset(2isize) as libc::c_int != 58) {
                compare = 3i32.wrapping_neg();
                copy = copy.offset(3isize)
            }
        }
        38 => {
            if !(*copy.offset(1isize) as libc::c_int != 38 ||
                     *copy.offset(2isize) as libc::c_int != 58) {
                compare = 4i32.wrapping_neg();
                copy = copy.offset(3isize)
            }
        }
        33 => {
            if *copy.offset(1isize) as libc::c_int == 61 &&
                   *copy.offset(2isize) as libc::c_int == 58 {
                compare = 1i32.wrapping_neg();
                copy = copy.offset(3isize)
            }
        }
        61 => {
            if *copy.offset(1isize) as libc::c_int == 61 &&
                   *copy.offset(2isize) as libc::c_int == 58 {
                compare = 1i32;
                copy = copy.offset(3isize)
            } else {
                *__errno_location() = 0i32;
                limit =
                    strtol(copy.offset(1isize),
                           &mut endptr as *mut *mut libc::c_char, 10i32);
                if !(*__errno_location() == 34i32 &&
                         (limit ==
                              9223372036854775807i64.wrapping_neg() - 1i64 ||
                              limit == 9223372036854775807i64)) {
                    if !(*endptr as libc::c_int != 58) {
                        copy = endptr.offset(1isize)
                    }
                }
            }
        }
        98 => {
            if !(*copy.offset(1isize) as libc::c_int != 58) {
                modifiers |= 2i32;
                copy = copy.offset(2isize)
            }
        }
        100 => {
            if !(*copy.offset(1isize) as libc::c_int != 58) {
                modifiers |= 4i32;
                copy = copy.offset(2isize)
            }
        }
        116 => {
            if !(*copy.offset(1isize) as libc::c_int != 58) {
                modifiers |= 1i32;
                copy = copy.offset(2isize)
            }
        }
        115 => {
            if !(*copy.offset(1isize) as libc::c_int != 47) {
                from = copy.offset(2isize);
                copy = from;
                while *copy as libc::c_int != 0 && *copy as libc::c_int != 47
                      {
                    copy = copy.offset(1isize)
                }
                if *copy.offset(0isize) as libc::c_int != 47 || copy == from {
                    copy = copy0
                } else {
                    *copy.offset(0isize) = 0 as libc::c_char;
                    to = copy.offset(1isize);
                    copy = to;
                    while *copy as libc::c_int != 0 &&
                              *copy as libc::c_int != 47 {
                        copy = copy.offset(1isize)
                    }
                    if *copy.offset(0isize) as libc::c_int != 47 ||
                           *copy.offset(1isize) as libc::c_int != 58 {
                        copy = copy0
                    } else {
                        *copy.offset(0isize) = 0 as libc::c_char;
                        modifiers |= 8i32;
                        copy = copy.offset(2isize)
                    }
                }
            }
        }
        _ => { }
    }
    if 0 != search {
        if wp == 0 as *mut libc::c_void as *mut window_pane {
            value = xstrdup(b"0\x00" as *const u8 as *const libc::c_char)
        } else {
            xasprintf(&mut value as *mut *mut libc::c_char,
                      b"%u\x00" as *const u8 as *const libc::c_char,
                      window_pane_search(wp, copy));
        }
    } else {
        if compare != 0i32 {
            if format_choose(copy, &mut left as *mut *mut libc::c_char,
                             &mut right as *mut *mut libc::c_char) != 0i32 {
                current_block = 17369695013924525517;
            } else {
                left = format_expand(ft, left);
                right = format_expand(ft, right);
                if compare == 3i32.wrapping_neg() &&
                       (0 != format_true(left) || 0 != format_true(right)) {
                    value =
                        xstrdup(b"1\x00" as *const u8 as *const libc::c_char)
                } else if compare == 4i32.wrapping_neg() &&
                              (0 != format_true(left) &&
                                   0 != format_true(right)) {
                    value =
                        xstrdup(b"1\x00" as *const u8 as *const libc::c_char)
                } else if compare == 1i32 && strcmp(left, right) == 0i32 {
                    value =
                        xstrdup(b"1\x00" as *const u8 as *const libc::c_char)
                } else if compare == 1i32.wrapping_neg() &&
                              strcmp(left, right) != 0i32 {
                    value =
                        xstrdup(b"1\x00" as *const u8 as *const libc::c_char)
                } else if compare == 2i32.wrapping_neg() &&
                              fnmatch(left, right, 0i32) == 0i32 {
                    value =
                        xstrdup(b"1\x00" as *const u8 as *const libc::c_char)
                } else {
                    value =
                        xstrdup(b"0\x00" as *const u8 as *const libc::c_char)
                }
                free(right as *mut libc::c_void);
                free(left as *mut libc::c_void);
                current_block = 5634871135123216486;
            }
        } else if *copy as libc::c_int == 63 {
            ptr = format_skip(copy);
            if ptr == 0 as *mut libc::c_void as *mut libc::c_char {
                current_block = 17369695013924525517;
            } else {
                *ptr = 0 as libc::c_char;
                found = format_find(ft, copy.offset(1isize), modifiers);
                if found == 0 as *mut libc::c_void as *mut libc::c_char {
                    found = format_expand(ft, copy.offset(1isize))
                }
                if format_choose(ptr.offset(1isize),
                                 &mut left as *mut *mut libc::c_char,
                                 &mut right as *mut *mut libc::c_char) != 0i32
                   {
                    current_block = 17369695013924525517;
                } else {
                    if 0 != format_true(found) {
                        value = format_expand(ft, left)
                    } else { value = format_expand(ft, right) }
                    free(found as *mut libc::c_void);
                    current_block = 5634871135123216486;
                }
            }
        } else {
            value = format_find(ft, copy, modifiers);
            if value == 0 as *mut libc::c_void as *mut libc::c_char {
                value = xstrdup(b"\x00" as *const u8 as *const libc::c_char);
                current_block = 5634871135123216486;
            } else { current_block = 5634871135123216486; }
        }
        match current_block {
            5634871135123216486 => { }
            _ => {
                free(copy0 as *mut libc::c_void);
                return 1i32.wrapping_neg()
            }
        }
    }
    if 0 != modifiers & 8i32 {
        fromlen = strlen(from);
        tolen = strlen(to);
        newlen = strlen(value).wrapping_add(1i32 as libc::c_ulong);
        new = xmalloc(newlen) as *mut libc::c_char;
        copy = new;
        ptr = value;
        while *ptr as libc::c_int != 0 {
            if strncmp(ptr, from, fromlen) != 0i32 {
                let fresh8 = new;
                new = new.offset(1);
                let fresh7 = ptr;
                ptr = ptr.offset(1);
                *fresh8 = *fresh7
            } else {
                used =
                    copy.offset_to(new).expect("bad offset_to") as
                        libc::c_long as size_t;
                newlen =
                    (newlen as libc::c_ulong).wrapping_add(tolen) as size_t as
                        size_t;
                copy =
                    xrealloc(copy as *mut libc::c_void, newlen) as
                        *mut libc::c_char;
                new = copy.offset(used as isize);
                memcpy(new as *mut libc::c_void, to as *const libc::c_void,
                       tolen);
                new = new.offset(tolen as isize);
                ptr = ptr.offset(fromlen as isize)
            }
        }
        *new = 0 as libc::c_char;
        free(value as *mut libc::c_void);
        value = copy
    }
    if limit > 0i32 as libc::c_long {
        new = utf8_trimcstr(value, limit as u_int);
        free(value as *mut libc::c_void);
        value = new
    } else if limit < 0i32 as libc::c_long {
        new = utf8_rtrimcstr(value, limit.wrapping_neg() as u_int);
        free(value as *mut libc::c_void);
        value = new
    }
    valuelen = strlen(value);
    while (*len).wrapping_sub(*off) <
              valuelen.wrapping_add(1i32 as libc::c_ulong) {
        *buf =
            xreallocarray(*buf as *mut libc::c_void, 2i32 as size_t, *len) as
                *mut libc::c_char;
        *len =
            (*len as libc::c_ulong).wrapping_mul(2i32 as libc::c_ulong) as
                size_t as size_t
    }
    memcpy((*buf).offset(*off as isize) as *mut libc::c_void,
           value as *const libc::c_void, valuelen);
    *off = (*off as libc::c_ulong).wrapping_add(valuelen) as size_t as size_t;
    free(value as *mut libc::c_void);
    free(copy0 as *mut libc::c_void);
    return 0i32;
}
unsafe extern "C" fn format_find(mut ft: *mut format_tree,
                                 mut key: *const libc::c_char,
                                 mut modifiers: libc::c_int)
 -> *mut libc::c_char {
    let mut current_block: u64;
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe_find: format_entry =
        format_entry{key: 0 as *mut libc::c_char,
                     value: 0 as *mut libc::c_char,
                     t: 0,
                     cb: None,
                     entry:
                         unnamed_31{rbe_left: 0 as *mut format_entry,
                                    rbe_right: 0 as *mut format_entry,
                                    rbe_parent: 0 as *mut format_entry,
                                    rbe_color: 0,},};
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    static mut s: [libc::c_char; 64] = unsafe { [0; 64] };
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut found: *const libc::c_char = 0 as *const libc::c_char;
    let mut idx: libc::c_int = 0;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut saved: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 != !modifiers & 1i32 {
        o =
            options_parse_get(global_options, key,
                              &mut idx as *mut libc::c_int, 0i32);
        if o == 0 as *mut libc::c_void as *mut options_entry &&
               (*ft).w != 0 as *mut libc::c_void as *mut window {
            o =
                options_parse_get((*(*ft).w).options, key,
                                  &mut idx as *mut libc::c_int, 0i32)
        }
        if o == 0 as *mut libc::c_void as *mut options_entry {
            o =
                options_parse_get(global_w_options, key,
                                  &mut idx as *mut libc::c_int, 0i32)
        }
        if o == 0 as *mut libc::c_void as *mut options_entry &&
               (*ft).s != 0 as *mut libc::c_void as *mut session {
            o =
                options_parse_get((*(*ft).s).options, key,
                                  &mut idx as *mut libc::c_int, 0i32)
        }
        if o == 0 as *mut libc::c_void as *mut options_entry {
            o =
                options_parse_get(global_s_options, key,
                                  &mut idx as *mut libc::c_int, 0i32)
        }
        if o != 0 as *mut libc::c_void as *mut options_entry {
            found = options_tostring(o, idx, 1i32);
            current_block = 8890551863119205552;
        } else { current_block = 6239978542346980191; }
    } else { current_block = 6239978542346980191; }
    match current_block {
        6239978542346980191 => {
            found = 0 as *const libc::c_char;
            fe_find.key = key as *mut libc::c_char;
            fe =
                format_entry_tree_RB_FIND(&mut (*ft).tree as
                                              *mut format_entry_tree,
                                          &mut fe_find as *mut format_entry);
            if fe != 0 as *mut libc::c_void as *mut format_entry {
                if 0 != modifiers & 1i32 {
                    if (*fe).t == 0i32 as libc::c_long {
                        return 0 as *mut libc::c_char
                    } else {
                        ctime_r(&mut (*fe).t as *mut time_t, s.as_mut_ptr());
                        s[strcspn(s.as_mut_ptr(),
                                  b"\n\x00" as *const u8 as
                                      *const libc::c_char) as usize] =
                            0 as libc::c_char;
                        found = s.as_mut_ptr()
                    }
                } else if (*fe).t != 0i32 as libc::c_long {
                    xsnprintf(s.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 64]>() as
                                  libc::c_ulong,
                              b"%lld\x00" as *const u8 as *const libc::c_char,
                              (*fe).t as libc::c_longlong);
                    found = s.as_mut_ptr()
                } else {
                    if (*fe).value ==
                           0 as *mut libc::c_void as *mut libc::c_char &&
                           (*fe).cb !=
                               ::std::mem::transmute::<*mut libc::c_void,
                                                       format_cb>(0 as
                                                                      *mut libc::c_void)
                       {
                        (*fe).cb.expect("non-null function pointer")(ft, fe);
                        if (*fe).value ==
                               0 as *mut libc::c_void as *mut libc::c_char {
                            (*fe).value =
                                xstrdup(b"\x00" as *const u8 as
                                            *const libc::c_char)
                        }
                    }
                    found = (*fe).value
                }
            } else {
                if 0 != !modifiers & 1i32 {
                    envent = 0 as *mut environ_entry;
                    if (*ft).s != 0 as *mut libc::c_void as *mut session {
                        envent = environ_find((*(*ft).s).environ, key)
                    }
                    if envent == 0 as *mut libc::c_void as *mut environ_entry
                       {
                        envent = environ_find(global_environ, key)
                    }
                    if envent != 0 as *mut libc::c_void as *mut environ_entry
                       {
                        found = (*envent).value;
                        current_block = 8890551863119205552;
                    } else { current_block = 4166486009154926805; }
                } else { current_block = 4166486009154926805; }
                match current_block {
                    8890551863119205552 => { }
                    _ => { return 0 as *mut libc::c_char }
                }
            }
        }
        _ => { }
    }
    if found == 0 as *mut libc::c_void as *const libc::c_char {
        return 0 as *mut libc::c_char
    } else {
        copy = xstrdup(found);
        if 0 != modifiers & 2i32 {
            saved = copy;
            copy = xstrdup(__xpg_basename(saved));
            free(saved as *mut libc::c_void);
        }
        if 0 != modifiers & 4i32 {
            saved = copy;
            copy = xstrdup(dirname(saved));
            free(saved as *mut libc::c_void);
        }
        return copy
    };
}
unsafe extern "C" fn format_entry_tree_RB_FIND(mut head:
                                                   *mut format_entry_tree,
                                               mut elm: *mut format_entry)
 -> *mut format_entry {
    let mut tmp: *mut format_entry = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = format_entry_cmp(elm, tmp);
            if comp < 0i32 {
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return 0 as *mut format_entry }
    };
}
unsafe extern "C" fn format_choose(mut s: *mut libc::c_char,
                                   mut left: *mut *mut libc::c_char,
                                   mut right: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = format_skip(s);
    if cp == 0 as *mut libc::c_void as *mut libc::c_char {
        return 1i32.wrapping_neg()
    } else {
        *cp = 0 as libc::c_char;
        *left = s;
        *right = cp.offset(1isize);
        return 0i32
    };
}
unsafe extern "C" fn format_skip(mut s: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut brackets: libc::c_int = 0i32;
    while *s as libc::c_int != 0 {
        if *s as libc::c_int == 123 { brackets += 1 }
        if *s as libc::c_int == 125 { brackets -= 1 }
        if *s as libc::c_int == 44 && brackets == 0i32 { break ; }
        s = s.offset(1isize)
    }
    if *s as libc::c_int == 0 {
        return 0 as *mut libc::c_char
    } else { return s };
}
static mut format_lower: [*const libc::c_char; 26] =
    unsafe {
        [0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char,
         b"host_short\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char]
    };
static mut format_upper: [*const libc::c_char; 26] =
    unsafe {
        [0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char,
         b"pane_id\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char,
         b"window_flags\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char,
         b"host\x00" as *const u8 as *const libc::c_char,
         b"window_index\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         b"pane_index\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         b"session_name\x00" as *const u8 as *const libc::c_char,
         b"pane_title\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         b"window_name\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char, 0 as *const libc::c_char,
         0 as *const libc::c_char]
    };
unsafe extern "C" fn format_job_get(mut ft: *mut format_tree,
                                    mut cmd: *const libc::c_char)
 -> *mut libc::c_char {
    let mut jobs: *mut format_job_tree = 0 as *mut format_job_tree;
    let mut fj0: format_job =
        format_job{client: 0 as *mut client,
                   tag: 0,
                   cmd: 0 as *const libc::c_char,
                   expanded: 0 as *const libc::c_char,
                   last: 0,
                   out: 0 as *mut libc::c_char,
                   updated: 0,
                   job: 0 as *mut job,
                   status: 0,
                   entry:
                       unnamed_5{rbe_left: 0 as *mut format_job,
                                 rbe_right: 0 as *mut format_job,
                                 rbe_parent: 0 as *mut format_job,
                                 rbe_color: 0,},};
    let mut fj: *mut format_job = 0 as *mut format_job;
    let mut t: time_t = 0;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut force: libc::c_int = 0;
    if (*ft).client == 0 as *mut libc::c_void as *mut client {
        jobs = &mut format_jobs as *mut format_job_tree
    } else if (*(*ft).client).jobs !=
                  0 as *mut libc::c_void as *mut format_job_tree {
        jobs = (*(*ft).client).jobs
    } else {
        (*(*ft).client).jobs =
            xmalloc(::std::mem::size_of::<format_job_tree>() as libc::c_ulong)
                as *mut format_job_tree;
        jobs = (*(*ft).client).jobs;
        loop  {
            (*jobs).rbh_root = 0 as *mut format_job;
            if !(0 != 0i32) { break ; }
        }
    }
    fj0.tag = (*ft).tag;
    fj0.cmd = cmd;
    fj = format_job_tree_RB_FIND(jobs, &mut fj0 as *mut format_job);
    if fj == 0 as *mut libc::c_void as *mut format_job {
        fj =
            xcalloc(1i32 as size_t,
                    ::std::mem::size_of::<format_job>() as libc::c_ulong) as
                *mut format_job;
        (*fj).client = (*ft).client;
        (*fj).tag = (*ft).tag;
        (*fj).cmd = xstrdup(cmd);
        (*fj).expanded = 0 as *const libc::c_char;
        xasprintf(&mut (*fj).out as *mut *mut libc::c_char,
                  b"<\'%s\' not ready>\x00" as *const u8 as
                      *const libc::c_char, (*fj).cmd);
        format_job_tree_RB_INSERT(jobs, fj);
    }
    expanded = format_expand(ft, cmd);
    if (*fj).expanded == 0 as *mut libc::c_void as *const libc::c_char ||
           strcmp(expanded, (*fj).expanded) != 0i32 {
        free((*fj).expanded as *mut libc::c_void);
        (*fj).expanded = xstrdup(expanded);
        force = 1i32
    } else { force = (*ft).flags & 2i32 }
    t = time(0 as *mut time_t);
    if (*fj).job == 0 as *mut libc::c_void as *mut job &&
           (0 != force || (*fj).last != t) {
        (*fj).job =
            job_run(expanded, 0 as *mut session, 0 as *const libc::c_char,
                    Some(format_job_update), Some(format_job_complete), None,
                    fj as *mut libc::c_void, 1i32);
        if (*fj).job == 0 as *mut libc::c_void as *mut job {
            free((*fj).out as *mut libc::c_void);
            xasprintf(&mut (*fj).out as *mut *mut libc::c_char,
                      b"<\'%s\' didn\'t start>\x00" as *const u8 as
                          *const libc::c_char, (*fj).cmd);
        }
        (*fj).last = t;
        (*fj).updated = 0i32
    }
    if 0 != (*ft).flags & 1i32 { (*fj).status = 1i32 }
    free(expanded as *mut libc::c_void);
    return format_expand(ft, (*fj).out);
}
unsafe extern "C" fn format_job_complete(mut job: *mut job) -> () {
    let mut fj: *mut format_job = (*job).data as *mut format_job;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    (*fj).job = 0 as *mut job;
    buf = 0 as *mut libc::c_char;
    line = evbuffer_readline((*(*job).event).input);
    if line == 0 as *mut libc::c_void as *mut libc::c_char {
        len = evbuffer_get_length((*(*job).event).input);
        buf =
            xmalloc(len.wrapping_add(1i32 as libc::c_ulong)) as
                *mut libc::c_char;
        if len != 0i32 as libc::c_ulong {
            memcpy(buf as *mut libc::c_void,
                   evbuffer_pullup((*(*job).event).input,
                                   1i32.wrapping_neg() as ssize_t) as
                       *const libc::c_void, len);
        }
        *buf.offset(len as isize) = 0 as libc::c_char
    } else { buf = line }
    log_debug(b"%s: %p %s: %s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"format_job_complete\x00")).as_ptr(),
              fj, (*fj).cmd, buf);
    if *buf as libc::c_int != 0 || 0 == (*fj).updated {
        free((*fj).out as *mut libc::c_void);
        (*fj).out = buf
    } else { free(buf as *mut libc::c_void); }
    if 0 != (*fj).status {
        if (*fj).client != 0 as *mut libc::c_void as *mut client {
            server_status_client((*fj).client);
        }
        (*fj).status = 0i32
    };
}
unsafe extern "C" fn format_job_update(mut job: *mut job) -> () {
    let mut fj: *mut format_job = (*job).data as *mut format_job;
    let mut evb: *mut evbuffer = (*(*job).event).input;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: time_t = 0;
    loop  {
        next = evbuffer_readline(evb);
        if !(next != 0 as *mut libc::c_void as *mut libc::c_char) { break ; }
        free(line as *mut libc::c_void);
        line = next
    }
    if line == 0 as *mut libc::c_void as *mut libc::c_char {
        return
    } else {
        (*fj).updated = 1i32;
        free((*fj).out as *mut libc::c_void);
        (*fj).out = line;
        log_debug(b"%s: %p %s: %s\x00" as *const u8 as *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"format_job_update\x00")).as_ptr(),
                  fj, (*fj).cmd, (*fj).out);
        t = time(0 as *mut time_t);
        if 0 != (*fj).status && (*fj).last != t {
            if (*fj).client != 0 as *mut libc::c_void as *mut client {
                server_status_client((*fj).client);
            }
            (*fj).last = t
        }
        return;
    };
}
unsafe extern "C" fn format_job_tree_RB_INSERT(mut head: *mut format_job_tree,
                                               mut elm: *mut format_job)
 -> *mut format_job {
    let mut tmp: *mut format_job = 0 as *mut format_job;
    let mut parent: *mut format_job = 0 as *mut format_job;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = format_job_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    loop  {
        (*elm).entry.rbe_parent = parent;
        (*elm).entry.rbe_right = 0 as *mut format_job;
        (*elm).entry.rbe_left = (*elm).entry.rbe_right;
        (*elm).entry.rbe_color = 1i32;
        if !(0 != 0i32) { break ; }
    }
    if parent != 0 as *mut libc::c_void as *mut format_job {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else { (*parent).entry.rbe_right = elm }
        while 0 != 0i32 { }
    } else { (*head).rbh_root = elm }
    format_job_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut format_job;
}
unsafe extern "C" fn format_job_tree_RB_INSERT_COLOR(mut head:
                                                         *mut format_job_tree,
                                                     mut elm: *mut format_job)
 -> () {
    let mut current_block: u64;
    let mut parent: *mut format_job = 0 as *mut format_job;
    let mut gparent: *mut format_job = 0 as *mut format_job;
    let mut tmp: *mut format_job = 0 as *mut format_job;
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
unsafe extern "C" fn format_job_cmp(mut fj1: *mut format_job,
                                    mut fj2: *mut format_job) -> libc::c_int {
    if (*fj1).tag < (*fj2).tag {
        return 1i32.wrapping_neg()
    } else if (*fj1).tag > (*fj2).tag {
        return 1i32
    } else { return strcmp((*fj1).cmd, (*fj2).cmd) };
}
unsafe extern "C" fn format_job_tree_RB_FIND(mut head: *mut format_job_tree,
                                             mut elm: *mut format_job)
 -> *mut format_job {
    let mut tmp: *mut format_job = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = format_job_cmp(elm, tmp);
            if comp < 0i32 {
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return 0 as *mut format_job }
    };
}
#[no_mangle]
pub unsafe extern "C" fn format_single(mut item: *mut cmdq_item,
                                       mut fmt: *const libc::c_char,
                                       mut c: *mut client,
                                       mut s: *mut session,
                                       mut wl: *mut winlink,
                                       mut wp: *mut window_pane)
 -> *mut libc::c_char {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    if item != 0 as *mut libc::c_void as *mut cmdq_item {
        ft = format_create((*item).client, item, 0i32, 0i32)
    } else { ft = format_create(0 as *mut client, item, 0i32, 0i32) }
    format_defaults(ft, c, s, wl, wp);
    expanded = format_expand(ft, fmt);
    format_free(ft);
    return expanded;
}
#[no_mangle]
pub unsafe extern "C" fn format_defaults(mut ft: *mut format_tree,
                                         mut c: *mut client,
                                         mut s: *mut session,
                                         mut wl: *mut winlink,
                                         mut wp: *mut window_pane) -> () {
    if c != 0 as *mut libc::c_void as *mut client &&
           s != 0 as *mut libc::c_void as *mut session && (*c).session != s {
        log_debug(b"%s: session does not match\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"format_defaults\x00")).as_ptr());
    }
    format_add(ft, b"session_format\x00" as *const u8 as *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char,
               (s != 0 as *mut libc::c_void as *mut session) as libc::c_int);
    format_add(ft, b"window_format\x00" as *const u8 as *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char,
               (wl != 0 as *mut libc::c_void as *mut winlink) as libc::c_int);
    format_add(ft, b"pane_format\x00" as *const u8 as *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char,
               (wp != 0 as *mut libc::c_void as *mut window_pane) as
                   libc::c_int);
    if s == 0 as *mut libc::c_void as *mut session &&
           c != 0 as *mut libc::c_void as *mut client {
        s = (*c).session
    }
    if wl == 0 as *mut libc::c_void as *mut winlink &&
           s != 0 as *mut libc::c_void as *mut session {
        wl = (*s).curw
    }
    if wp == 0 as *mut libc::c_void as *mut window_pane &&
           wl != 0 as *mut libc::c_void as *mut winlink {
        wp = (*(*wl).window).active
    }
    if c != 0 as *mut libc::c_void as *mut client {
        format_defaults_client(ft, c);
    }
    if s != 0 as *mut libc::c_void as *mut session {
        format_defaults_session(ft, s);
    }
    if wl != 0 as *mut libc::c_void as *mut winlink {
        format_defaults_winlink(ft, wl);
    }
    if wp != 0 as *mut libc::c_void as *mut window_pane {
        format_defaults_pane(ft, wp);
    };
}
#[no_mangle]
pub unsafe extern "C" fn format_defaults_pane(mut ft: *mut format_tree,
                                              mut wp: *mut window_pane)
 -> () {
    let mut gd: *mut grid = (*wp).base.grid;
    let mut status: libc::c_int = (*wp).status;
    let mut idx: u_int = 0;
    if (*ft).w == 0 as *mut libc::c_void as *mut window {
        (*ft).w = (*wp).window
    }
    (*ft).wp = wp;
    format_add(ft, b"history_size\x00" as *const u8 as *const libc::c_char,
               b"%u\x00" as *const u8 as *const libc::c_char, (*gd).hsize);
    format_add(ft, b"history_limit\x00" as *const u8 as *const libc::c_char,
               b"%u\x00" as *const u8 as *const libc::c_char, (*gd).hlimit);
    format_add_cb(ft,
                  b"history_bytes\x00" as *const u8 as *const libc::c_char,
                  Some(format_cb_history_bytes));
    if window_pane_index(wp, &mut idx as *mut u_int) != 0i32 {
        fatalx(b"index not found\x00" as *const u8 as *const libc::c_char);
    } else {
        format_add(ft, b"pane_index\x00" as *const u8 as *const libc::c_char,
                   b"%u\x00" as *const u8 as *const libc::c_char, idx);
        format_add(ft, b"pane_width\x00" as *const u8 as *const libc::c_char,
                   b"%u\x00" as *const u8 as *const libc::c_char, (*wp).sx);
        format_add(ft, b"pane_height\x00" as *const u8 as *const libc::c_char,
                   b"%u\x00" as *const u8 as *const libc::c_char, (*wp).sy);
        format_add(ft, b"pane_title\x00" as *const u8 as *const libc::c_char,
                   b"%s\x00" as *const u8 as *const libc::c_char,
                   (*wp).base.title);
        format_add(ft, b"pane_id\x00" as *const u8 as *const libc::c_char,
                   b"%%%u\x00" as *const u8 as *const libc::c_char, (*wp).id);
        format_add(ft, b"pane_active\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   (wp == (*(*wp).window).active) as libc::c_int);
        format_add(ft,
                   b"pane_input_off\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   !(0 == (*wp).flags & 64i32) as libc::c_int);
        format_add(ft, b"pane_pipe\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   ((*wp).pipe_fd != 1i32.wrapping_neg()) as libc::c_int);
        if (0 != (*wp).flags & 512i32 &&
                unnamed_40{__in: status,}.__i & 127i32 == 0i32) {
            format_add(ft,
                       b"pane_dead_status\x00" as *const u8 as
                           *const libc::c_char,
                       b"%d\x00" as *const u8 as *const libc::c_char,
                       (unnamed_9{__in: status,}.__i & 65280i32) >> 8i32);
        }
        format_add(ft, b"pane_dead\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   ((*wp).fd == 1i32.wrapping_neg()) as libc::c_int);
        if 0 != window_pane_visible(wp) {
            format_add(ft,
                       b"pane_left\x00" as *const u8 as *const libc::c_char,
                       b"%u\x00" as *const u8 as *const libc::c_char,
                       (*wp).xoff);
            format_add(ft,
                       b"pane_top\x00" as *const u8 as *const libc::c_char,
                       b"%u\x00" as *const u8 as *const libc::c_char,
                       (*wp).yoff);
            format_add(ft,
                       b"pane_right\x00" as *const u8 as *const libc::c_char,
                       b"%u\x00" as *const u8 as *const libc::c_char,
                       (*wp).xoff.wrapping_add((*wp).sx).wrapping_sub(1i32 as
                                                                          libc::c_uint));
            format_add(ft,
                       b"pane_bottom\x00" as *const u8 as *const libc::c_char,
                       b"%u\x00" as *const u8 as *const libc::c_char,
                       (*wp).yoff.wrapping_add((*wp).sy).wrapping_sub(1i32 as
                                                                          libc::c_uint));
            format_add(ft,
                       b"pane_at_left\x00" as *const u8 as
                           *const libc::c_char,
                       b"%d\x00" as *const u8 as *const libc::c_char,
                       ((*wp).xoff == 0i32 as libc::c_uint) as libc::c_int);
            format_add(ft,
                       b"pane_at_top\x00" as *const u8 as *const libc::c_char,
                       b"%d\x00" as *const u8 as *const libc::c_char,
                       ((*wp).yoff == 0i32 as libc::c_uint) as libc::c_int);
            format_add(ft,
                       b"pane_at_right\x00" as *const u8 as
                           *const libc::c_char,
                       b"%d\x00" as *const u8 as *const libc::c_char,
                       ((*wp).xoff.wrapping_add((*wp).sx) ==
                            (*(*wp).window).sx) as libc::c_int);
            format_add(ft,
                       b"pane_at_bottom\x00" as *const u8 as
                           *const libc::c_char,
                       b"%d\x00" as *const u8 as *const libc::c_char,
                       ((*wp).yoff.wrapping_add((*wp).sy) ==
                            (*(*wp).window).sy) as libc::c_int);
        }
        format_add(ft,
                   b"pane_in_mode\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   ((*wp).screen != &mut (*wp).base as *mut screen) as
                       libc::c_int);
        if (*wp).mode != 0 as *mut libc::c_void as *const window_mode {
            format_add(ft,
                       b"pane_mode\x00" as *const u8 as *const libc::c_char,
                       b"%s\x00" as *const u8 as *const libc::c_char,
                       (*(*wp).mode).name);
        }
        format_add(ft,
                   b"pane_synchronized\x00" as *const u8 as
                       *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   !(0 ==
                         options_get_number((*(*wp).window).options,
                                            b"synchronize-panes\x00" as
                                                *const u8 as
                                                *const libc::c_char)) as
                       libc::c_int);
        if (*wp).searchstr != 0 as *mut libc::c_void as *mut libc::c_char {
            format_add(ft,
                       b"pane_search_string\x00" as *const u8 as
                           *const libc::c_char,
                       b"%s\x00" as *const u8 as *const libc::c_char,
                       (*wp).searchstr);
        }
        format_add(ft, b"pane_tty\x00" as *const u8 as *const libc::c_char,
                   b"%s\x00" as *const u8 as *const libc::c_char,
                   (*wp).tty.as_mut_ptr());
        format_add(ft, b"pane_pid\x00" as *const u8 as *const libc::c_char,
                   b"%ld\x00" as *const u8 as *const libc::c_char,
                   (*wp).pid as libc::c_long);
        format_add_cb(ft,
                      b"pane_start_command\x00" as *const u8 as
                          *const libc::c_char, Some(format_cb_start_command));
        format_add_cb(ft,
                      b"pane_current_command\x00" as *const u8 as
                          *const libc::c_char,
                      Some(format_cb_current_command));
        format_add_cb(ft,
                      b"pane_current_path\x00" as *const u8 as
                          *const libc::c_char, Some(format_cb_current_path));
        format_add(ft, b"cursor_x\x00" as *const u8 as *const libc::c_char,
                   b"%u\x00" as *const u8 as *const libc::c_char,
                   (*wp).base.cx);
        format_add(ft, b"cursor_y\x00" as *const u8 as *const libc::c_char,
                   b"%u\x00" as *const u8 as *const libc::c_char,
                   (*wp).base.cy);
        format_add(ft,
                   b"scroll_region_upper\x00" as *const u8 as
                       *const libc::c_char,
                   b"%u\x00" as *const u8 as *const libc::c_char,
                   (*wp).base.rupper);
        format_add(ft,
                   b"scroll_region_lower\x00" as *const u8 as
                       *const libc::c_char,
                   b"%u\x00" as *const u8 as *const libc::c_char,
                   (*wp).base.rlower);
        window_copy_add_formats(wp, ft);
        format_add(ft,
                   b"alternate_on\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   if !(*wp).saved_grid.is_null() { 1i32 } else { 0i32 });
        format_add(ft,
                   b"alternate_saved_x\x00" as *const u8 as
                       *const libc::c_char,
                   b"%u\x00" as *const u8 as *const libc::c_char,
                   (*wp).saved_cx);
        format_add(ft,
                   b"alternate_saved_y\x00" as *const u8 as
                       *const libc::c_char,
                   b"%u\x00" as *const u8 as *const libc::c_char,
                   (*wp).saved_cy);
        format_add(ft, b"cursor_flag\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   !(0 == (*wp).base.mode & 1i32) as libc::c_int);
        format_add(ft, b"insert_flag\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   !(0 == (*wp).base.mode & 2i32) as libc::c_int);
        format_add(ft,
                   b"keypad_cursor_flag\x00" as *const u8 as
                       *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   !(0 == (*wp).base.mode & 4i32) as libc::c_int);
        format_add(ft, b"keypad_flag\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   !(0 == (*wp).base.mode & 8i32) as libc::c_int);
        format_add(ft, b"wrap_flag\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   !(0 == (*wp).base.mode & 16i32) as libc::c_int);
        format_add(ft,
                   b"mouse_any_flag\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   !(0 == (*wp).base.mode & (32i32 | 64i32 | 4096i32)) as
                       libc::c_int);
        format_add(ft,
                   b"mouse_standard_flag\x00" as *const u8 as
                       *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   !(0 == (*wp).base.mode & 32i32) as libc::c_int);
        format_add(ft,
                   b"mouse_button_flag\x00" as *const u8 as
                       *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   !(0 == (*wp).base.mode & 64i32) as libc::c_int);
        format_add(ft,
                   b"mouse_all_flag\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   !(0 == (*wp).base.mode & 4096i32) as libc::c_int);
        format_add_cb(ft,
                      b"pane_tabs\x00" as *const u8 as *const libc::c_char,
                      Some(format_cb_pane_tabs));
        return;
    };
}
unsafe extern "C" fn format_cb_pane_tabs(mut ft: *mut format_tree,
                                         mut fe: *mut format_entry) -> () {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut buffer: *mut evbuffer = 0 as *mut evbuffer;
    let mut i: u_int = 0;
    let mut size: libc::c_int = 0;
    if wp == 0 as *mut libc::c_void as *mut window_pane {
        return
    } else {
        buffer = evbuffer_new();
        i = 0i32 as u_int;
        while i < (*(*wp).base.grid).sx {
            if !(0 ==
                     *(*wp).base.tabs.offset((i >> 3i32) as isize) as
                         libc::c_int & 1i32 << (i & 7i32 as libc::c_uint)) {
                if evbuffer_get_length(buffer) > 0i32 as libc::c_ulong {
                    evbuffer_add(buffer,
                                 b",\x00" as *const u8 as *const libc::c_char
                                     as *const libc::c_void, 1i32 as size_t);
                }
                evbuffer_add_printf(buffer,
                                    b"%u\x00" as *const u8 as
                                        *const libc::c_char, i);
            }
            i = i.wrapping_add(1)
        }
        size = evbuffer_get_length(buffer) as libc::c_int;
        if size != 0i32 {
            xasprintf(&mut (*fe).value as *mut *mut libc::c_char,
                      b"%.*s\x00" as *const u8 as *const libc::c_char, size,
                      evbuffer_pullup(buffer,
                                      1i32.wrapping_neg() as ssize_t));
        }
        evbuffer_free(buffer);
        return;
    };
}
unsafe extern "C" fn format_cb_current_path(mut ft: *mut format_tree,
                                            mut fe: *mut format_entry) -> () {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut cwd: *mut libc::c_char = 0 as *mut libc::c_char;
    if wp == 0 as *mut libc::c_void as *mut window_pane {
        return
    } else {
        cwd = osdep_get_cwd((*wp).fd);
        if cwd != 0 as *mut libc::c_void as *mut libc::c_char {
            (*fe).value = xstrdup(cwd)
        }
        return;
    };
}
unsafe extern "C" fn format_cb_current_command(mut ft: *mut format_tree,
                                               mut fe: *mut format_entry)
 -> () {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    if wp == 0 as *mut libc::c_void as *mut window_pane {
        return
    } else {
        cmd = osdep_get_name((*wp).fd, (*wp).tty.as_mut_ptr());
        if cmd == 0 as *mut libc::c_void as *mut libc::c_char ||
               *cmd as libc::c_int == 0 {
            free(cmd as *mut libc::c_void);
            cmd = cmd_stringify_argv((*wp).argc, (*wp).argv);
            if cmd == 0 as *mut libc::c_void as *mut libc::c_char ||
                   *cmd as libc::c_int == 0 {
                free(cmd as *mut libc::c_void);
                cmd = xstrdup((*wp).shell)
            }
        }
        (*fe).value = parse_window_name(cmd);
        free(cmd as *mut libc::c_void);
        return;
    };
}
unsafe extern "C" fn format_cb_start_command(mut ft: *mut format_tree,
                                             mut fe: *mut format_entry)
 -> () {
    let mut wp: *mut window_pane = (*ft).wp;
    if wp == 0 as *mut libc::c_void as *mut window_pane {
        return
    } else {
        (*fe).value = cmd_stringify_argv((*wp).argc, (*wp).argv);
        return;
    };
}
unsafe extern "C" fn format_cb_history_bytes(mut ft: *mut format_tree,
                                             mut fe: *mut format_entry)
 -> () {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut gd: *mut grid = 0 as *mut grid;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut size: libc::c_ulonglong = 0;
    let mut i: u_int = 0;
    if wp == 0 as *mut libc::c_void as *mut window_pane {
        return
    } else {
        gd = (*wp).base.grid;
        size = 0i32 as libc::c_ulonglong;
        i = 0i32 as u_int;
        while i < (*gd).hsize {
            gl = &mut *(*gd).linedata.offset(i as isize) as *mut grid_line;
            size =
                size.wrapping_add(((*gl).cellsize as
                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<grid_cell_entry>()
                                                                       as
                                                                       libc::c_ulong)
                                      as libc::c_ulonglong);
            size =
                size.wrapping_add(((*gl).extdsize as
                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<grid_cell>()
                                                                       as
                                                                       libc::c_ulong)
                                      as libc::c_ulonglong);
            i = i.wrapping_add(1)
        }
        size =
            size.wrapping_add(((*gd).hsize as
                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<grid_line>()
                                                                   as
                                                                   libc::c_ulong)
                                  as libc::c_ulonglong);
        xasprintf(&mut (*fe).value as *mut *mut libc::c_char,
                  b"%llu\x00" as *const u8 as *const libc::c_char, size);
        return;
    };
}
unsafe extern "C" fn format_defaults_winlink(mut ft: *mut format_tree,
                                             mut wl: *mut winlink) -> () {
    let mut s: *mut session = (*wl).session;
    let mut w: *mut window = (*wl).window;
    if (*ft).w == 0 as *mut libc::c_void as *mut window {
        (*ft).w = (*wl).window
    }
    (*ft).wl = wl;
    format_defaults_window(ft, w);
    format_add(ft, b"window_index\x00" as *const u8 as *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char, (*wl).idx);
    format_add_cb(ft,
                  b"window_stack_index\x00" as *const u8 as
                      *const libc::c_char,
                  Some(format_cb_window_stack_index));
    format_add(ft, b"window_flags\x00" as *const u8 as *const libc::c_char,
               b"%s\x00" as *const u8 as *const libc::c_char,
               window_printable_flags(wl));
    format_add(ft, b"window_active\x00" as *const u8 as *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char,
               (wl == (*s).curw) as libc::c_int);
    format_add(ft,
               b"window_bell_flag\x00" as *const u8 as *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char,
               !(0 == (*wl).flags & 1i32) as libc::c_int);
    format_add(ft,
               b"window_activity_flag\x00" as *const u8 as
                   *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char,
               !(0 == (*wl).flags & 2i32) as libc::c_int);
    format_add(ft,
               b"window_silence_flag\x00" as *const u8 as *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char,
               !(0 == (*wl).flags & 4i32) as libc::c_int);
    format_add(ft,
               b"window_last_flag\x00" as *const u8 as *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char,
               (0 ==
                    !(wl ==
                          (*(&mut (*s).lastw as
                                 *mut winlink_stack)).tqh_first) as
                        libc::c_int) as libc::c_int);
    format_add(ft, b"window_linked\x00" as *const u8 as *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char,
               session_is_linked(s, (*wl).window));
}
unsafe extern "C" fn format_cb_window_stack_index(mut ft: *mut format_tree,
                                                  mut fe: *mut format_entry)
 -> () {
    let mut s: *mut session = (*(*ft).wl).session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut idx: u_int = 0;
    idx = 0i32 as u_int;
    wl = (*(&mut (*s).lastw as *mut winlink_stack)).tqh_first;
    while wl != 0 as *mut libc::c_void as *mut winlink {
        idx = idx.wrapping_add(1);
        if wl == (*ft).wl { break ; }
        wl = (*wl).sentry.tqe_next
    }
    if wl != 0 as *mut libc::c_void as *mut winlink {
        xasprintf(&mut (*fe).value as *mut *mut libc::c_char,
                  b"%u\x00" as *const u8 as *const libc::c_char, idx);
    } else {
        (*fe).value = xstrdup(b"0\x00" as *const u8 as *const libc::c_char)
    };
}
#[no_mangle]
pub unsafe extern "C" fn format_defaults_window(mut ft: *mut format_tree,
                                                mut w: *mut window) -> () {
    (*ft).w = w;
    format_add_tv(ft,
                  b"window_activity\x00" as *const u8 as *const libc::c_char,
                  &mut (*w).activity_time as *mut timeval);
    format_add(ft, b"window_id\x00" as *const u8 as *const libc::c_char,
               b"@%u\x00" as *const u8 as *const libc::c_char, (*w).id);
    format_add(ft, b"window_name\x00" as *const u8 as *const libc::c_char,
               b"%s\x00" as *const u8 as *const libc::c_char, (*w).name);
    format_add(ft, b"window_width\x00" as *const u8 as *const libc::c_char,
               b"%u\x00" as *const u8 as *const libc::c_char, (*w).sx);
    format_add(ft, b"window_height\x00" as *const u8 as *const libc::c_char,
               b"%u\x00" as *const u8 as *const libc::c_char, (*w).sy);
    format_add_cb(ft,
                  b"window_layout\x00" as *const u8 as *const libc::c_char,
                  Some(format_cb_window_layout));
    format_add_cb(ft,
                  b"window_visible_layout\x00" as *const u8 as
                      *const libc::c_char,
                  Some(format_cb_window_visible_layout));
    format_add(ft, b"window_panes\x00" as *const u8 as *const libc::c_char,
               b"%u\x00" as *const u8 as *const libc::c_char,
               window_count_panes(w));
    format_add(ft,
               b"window_zoomed_flag\x00" as *const u8 as *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char,
               !(0 == (*w).flags & 4096i32) as libc::c_int);
}
unsafe extern "C" fn format_cb_window_visible_layout(mut ft: *mut format_tree,
                                                     mut fe:
                                                         *mut format_entry)
 -> () {
    let mut w: *mut window = (*ft).w;
    if w == 0 as *mut libc::c_void as *mut window {
        return
    } else { (*fe).value = layout_dump((*w).layout_root); return; };
}
unsafe extern "C" fn format_cb_window_layout(mut ft: *mut format_tree,
                                             mut fe: *mut format_entry)
 -> () {
    let mut w: *mut window = (*ft).w;
    if w == 0 as *mut libc::c_void as *mut window {
        return
    } else {
        if (*w).saved_layout_root !=
               0 as *mut libc::c_void as *mut layout_cell {
            (*fe).value = layout_dump((*w).saved_layout_root)
        } else { (*fe).value = layout_dump((*w).layout_root) }
        return;
    };
}
unsafe extern "C" fn format_defaults_session(mut ft: *mut format_tree,
                                             mut s: *mut session) -> () {
    let mut sg: *mut session_group = 0 as *mut session_group;
    (*ft).s = s;
    format_add(ft, b"session_name\x00" as *const u8 as *const libc::c_char,
               b"%s\x00" as *const u8 as *const libc::c_char, (*s).name);
    format_add(ft, b"session_windows\x00" as *const u8 as *const libc::c_char,
               b"%u\x00" as *const u8 as *const libc::c_char,
               winlink_count(&mut (*s).windows as *mut winlinks));
    format_add(ft, b"session_width\x00" as *const u8 as *const libc::c_char,
               b"%u\x00" as *const u8 as *const libc::c_char, (*s).sx);
    format_add(ft, b"session_height\x00" as *const u8 as *const libc::c_char,
               b"%u\x00" as *const u8 as *const libc::c_char, (*s).sy);
    format_add(ft, b"session_id\x00" as *const u8 as *const libc::c_char,
               b"$%u\x00" as *const u8 as *const libc::c_char, (*s).id);
    sg = session_group_contains(s);
    format_add(ft, b"session_grouped\x00" as *const u8 as *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char,
               (sg != 0 as *mut libc::c_void as *mut session_group) as
                   libc::c_int);
    if sg != 0 as *mut libc::c_void as *mut session_group {
        format_add(ft,
                   b"session_group\x00" as *const u8 as *const libc::c_char,
                   b"%s\x00" as *const u8 as *const libc::c_char, (*sg).name);
        format_add(ft,
                   b"session_group_size\x00" as *const u8 as
                       *const libc::c_char,
                   b"%u\x00" as *const u8 as *const libc::c_char,
                   session_group_count(sg));
        format_add_cb(ft,
                      b"session_group_list\x00" as *const u8 as
                          *const libc::c_char,
                      Some(format_cb_session_group_list));
    }
    format_add_tv(ft,
                  b"session_created\x00" as *const u8 as *const libc::c_char,
                  &mut (*s).creation_time as *mut timeval);
    format_add_tv(ft,
                  b"session_last_attached\x00" as *const u8 as
                      *const libc::c_char,
                  &mut (*s).last_attached_time as *mut timeval);
    format_add_tv(ft,
                  b"session_activity\x00" as *const u8 as *const libc::c_char,
                  &mut (*s).activity_time as *mut timeval);
    format_add(ft,
               b"session_attached\x00" as *const u8 as *const libc::c_char,
               b"%u\x00" as *const u8 as *const libc::c_char, (*s).attached);
    format_add(ft,
               b"session_many_attached\x00" as *const u8 as
                   *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char,
               ((*s).attached > 1i32 as libc::c_uint) as libc::c_int);
    format_add_cb(ft,
                  b"session_alerts\x00" as *const u8 as *const libc::c_char,
                  Some(format_cb_session_alerts));
    format_add_cb(ft,
                  b"session_stack\x00" as *const u8 as *const libc::c_char,
                  Some(format_cb_session_stack));
}
unsafe extern "C" fn format_cb_session_stack(mut ft: *mut format_tree,
                                             mut fe: *mut format_entry)
 -> () {
    let mut s: *mut session = (*ft).s;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut result: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: [libc::c_char; 16] = [0; 16];
    if s == 0 as *mut libc::c_void as *mut session {
        return
    } else {
        xsnprintf(result.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 1024]>() as
                      libc::c_ulong,
                  b"%u\x00" as *const u8 as *const libc::c_char,
                  (*(*s).curw).idx);
        wl = (*(&mut (*s).lastw as *mut winlink_stack)).tqh_first;
        while wl != 0 as *mut libc::c_void as *mut winlink {
            xsnprintf(tmp.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 16]>() as
                          libc::c_ulong,
                      b"%u\x00" as *const u8 as *const libc::c_char,
                      (*wl).idx);
            if *result.as_mut_ptr() as libc::c_int != 0 {
                strlcat(result.as_mut_ptr(),
                        b",\x00" as *const u8 as *const libc::c_char,
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as
                            libc::c_ulong);
            }
            strlcat(result.as_mut_ptr(), tmp.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as
                        libc::c_ulong);
            wl = (*wl).sentry.tqe_next
        }
        (*fe).value = xstrdup(result.as_mut_ptr());
        return;
    };
}
unsafe extern "C" fn format_cb_session_alerts(mut ft: *mut format_tree,
                                              mut fe: *mut format_entry)
 -> () {
    let mut s: *mut session = (*ft).s;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut alerts: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: [libc::c_char; 16] = [0; 16];
    if s == 0 as *mut libc::c_void as *mut session {
        return
    } else {
        *alerts.as_mut_ptr() = 0 as libc::c_char;
        wl =
            winlinks_RB_MINMAX(&mut (*s).windows as *mut winlinks,
                               1i32.wrapping_neg());
        while wl != 0 as *mut libc::c_void as *mut winlink {
            if !((*wl).flags & (1i32 | 2i32 | 4i32) == 0i32) {
                xsnprintf(tmp.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 16]>() as
                              libc::c_ulong,
                          b"%u\x00" as *const u8 as *const libc::c_char,
                          (*wl).idx);
                if *alerts.as_mut_ptr() as libc::c_int != 0 {
                    strlcat(alerts.as_mut_ptr(),
                            b",\x00" as *const u8 as *const libc::c_char,
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                libc::c_ulong);
                }
                strlcat(alerts.as_mut_ptr(), tmp.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as
                            libc::c_ulong);
                if 0 != (*wl).flags & 2i32 {
                    strlcat(alerts.as_mut_ptr(),
                            b"#\x00" as *const u8 as *const libc::c_char,
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                libc::c_ulong);
                }
                if 0 != (*wl).flags & 1i32 {
                    strlcat(alerts.as_mut_ptr(),
                            b"!\x00" as *const u8 as *const libc::c_char,
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                libc::c_ulong);
                }
                if 0 != (*wl).flags & 4i32 {
                    strlcat(alerts.as_mut_ptr(),
                            b"~\x00" as *const u8 as *const libc::c_char,
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                libc::c_ulong);
                }
            }
            wl = winlinks_RB_NEXT(wl)
        }
        (*fe).value = xstrdup(alerts.as_mut_ptr());
        return;
    };
}
unsafe extern "C" fn format_cb_session_group_list(mut ft: *mut format_tree,
                                                  mut fe: *mut format_entry)
 -> () {
    let mut s: *mut session = (*ft).s;
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut loop_0: *mut session = 0 as *mut session;
    let mut buffer: *mut evbuffer = 0 as *mut evbuffer;
    let mut size: libc::c_int = 0;
    if s == 0 as *mut libc::c_void as *mut session {
        return
    } else {
        sg = session_group_contains(s);
        if sg == 0 as *mut libc::c_void as *mut session_group {
            return
        } else {
            buffer = evbuffer_new();
            loop_0 = (*(&mut (*sg).sessions as *mut unnamed_43)).tqh_first;
            while loop_0 != 0 as *mut libc::c_void as *mut session {
                if evbuffer_get_length(buffer) > 0i32 as libc::c_ulong {
                    evbuffer_add(buffer,
                                 b",\x00" as *const u8 as *const libc::c_char
                                     as *const libc::c_void, 1i32 as size_t);
                }
                evbuffer_add_printf(buffer,
                                    b"%s\x00" as *const u8 as
                                        *const libc::c_char, (*loop_0).name);
                loop_0 = (*loop_0).gentry.tqe_next
            }
            size = evbuffer_get_length(buffer) as libc::c_int;
            if size != 0i32 {
                xasprintf(&mut (*fe).value as *mut *mut libc::c_char,
                          b"%.*s\x00" as *const u8 as *const libc::c_char,
                          size,
                          evbuffer_pullup(buffer,
                                          1i32.wrapping_neg() as ssize_t));
            }
            evbuffer_free(buffer);
            return;
        }
    };
}
unsafe extern "C" fn format_defaults_client(mut ft: *mut format_tree,
                                            mut c: *mut client) -> () {
    let mut s: *mut session = 0 as *mut session;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut tty: *mut tty = &mut (*c).tty as *mut tty;
    let mut types: [*const libc::c_char; 7] =
        [b"VT100\x00" as *const u8 as *const libc::c_char,
         b"VT101\x00" as *const u8 as *const libc::c_char,
         b"VT102\x00" as *const u8 as *const libc::c_char,
         b"VT220\x00" as *const u8 as *const libc::c_char,
         b"VT320\x00" as *const u8 as *const libc::c_char,
         b"VT420\x00" as *const u8 as *const libc::c_char,
         b"Unknown\x00" as *const u8 as *const libc::c_char];
    if (*ft).s == 0 as *mut libc::c_void as *mut session {
        (*ft).s = (*c).session
    }
    format_add(ft, b"client_name\x00" as *const u8 as *const libc::c_char,
               b"%s\x00" as *const u8 as *const libc::c_char, (*c).name);
    format_add(ft, b"client_pid\x00" as *const u8 as *const libc::c_char,
               b"%ld\x00" as *const u8 as *const libc::c_char,
               (*c).pid as libc::c_long);
    format_add(ft, b"client_height\x00" as *const u8 as *const libc::c_char,
               b"%u\x00" as *const u8 as *const libc::c_char, (*tty).sy);
    format_add(ft, b"client_width\x00" as *const u8 as *const libc::c_char,
               b"%u\x00" as *const u8 as *const libc::c_char, (*tty).sx);
    format_add(ft, b"client_tty\x00" as *const u8 as *const libc::c_char,
               b"%s\x00" as *const u8 as *const libc::c_char, (*c).ttyname);
    format_add(ft,
               b"client_control_mode\x00" as *const u8 as *const libc::c_char,
               b"%d\x00" as *const u8 as *const libc::c_char,
               !(0 == (*c).flags & 8192i32) as libc::c_int);
    if (*tty).term_name != 0 as *mut libc::c_void as *mut libc::c_char {
        format_add(ft,
                   b"client_termname\x00" as *const u8 as *const libc::c_char,
                   b"%s\x00" as *const u8 as *const libc::c_char,
                   (*tty).term_name);
    }
    if (*tty).term_name != 0 as *mut libc::c_void as *mut libc::c_char {
        format_add(ft,
                   b"client_termtype\x00" as *const u8 as *const libc::c_char,
                   b"%s\x00" as *const u8 as *const libc::c_char,
                   types[(*tty).term_type as usize]);
    }
    format_add_tv(ft,
                  b"client_created\x00" as *const u8 as *const libc::c_char,
                  &mut (*c).creation_time as *mut timeval);
    format_add_tv(ft,
                  b"client_activity\x00" as *const u8 as *const libc::c_char,
                  &mut (*c).activity_time as *mut timeval);
    format_add(ft, b"client_written\x00" as *const u8 as *const libc::c_char,
               b"%zu\x00" as *const u8 as *const libc::c_char, (*c).written);
    format_add(ft,
               b"client_discarded\x00" as *const u8 as *const libc::c_char,
               b"%zu\x00" as *const u8 as *const libc::c_char,
               (*c).discarded);
    name = server_client_get_key_table(c);
    if strcmp((*(*c).keytable).name, name) == 0i32 {
        format_add(ft,
                   b"client_prefix\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char, 0i32);
    } else {
        format_add(ft,
                   b"client_prefix\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char, 1i32);
    }
    format_add(ft,
               b"client_key_table\x00" as *const u8 as *const libc::c_char,
               b"%s\x00" as *const u8 as *const libc::c_char,
               (*(*c).keytable).name);
    if 0 != (*tty).flags & 8i32 {
        format_add(ft, b"client_utf8\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char, 1i32);
    } else {
        format_add(ft, b"client_utf8\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char, 0i32);
    }
    if 0 != (*c).flags & 2048i32 {
        format_add(ft,
                   b"client_readonly\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char, 1i32);
    } else {
        format_add(ft,
                   b"client_readonly\x00" as *const u8 as *const libc::c_char,
                   b"%d\x00" as *const u8 as *const libc::c_char, 0i32);
    }
    s = (*c).session;
    if s != 0 as *mut libc::c_void as *mut session {
        format_add(ft,
                   b"client_session\x00" as *const u8 as *const libc::c_char,
                   b"%s\x00" as *const u8 as *const libc::c_char, (*s).name);
    }
    s = (*c).last_session;
    if s != 0 as *mut libc::c_void as *mut session && 0 != session_alive(s) {
        format_add(ft,
                   b"client_last_session\x00" as *const u8 as
                       *const libc::c_char,
                   b"%s\x00" as *const u8 as *const libc::c_char, (*s).name);
    };
}
#[no_mangle]
pub unsafe extern "C" fn format_defaults_paste_buffer(mut ft:
                                                          *mut format_tree,
                                                      mut pb:
                                                          *mut paste_buffer)
 -> () {
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut size: size_t = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let ref mut fresh9 = (*(&mut tv as *mut timeval)).tv_usec;
    *fresh9 = 0i32 as __suseconds_t;
    (*(&mut tv as *mut timeval)).tv_sec = *fresh9;
    tv.tv_sec = paste_buffer_created(pb);
    paste_buffer_data(pb, &mut size as *mut size_t);
    format_add(ft, b"buffer_size\x00" as *const u8 as *const libc::c_char,
               b"%zu\x00" as *const u8 as *const libc::c_char, size);
    format_add(ft, b"buffer_name\x00" as *const u8 as *const libc::c_char,
               b"%s\x00" as *const u8 as *const libc::c_char,
               paste_buffer_name(pb));
    format_add_tv(ft,
                  b"buffer_created\x00" as *const u8 as *const libc::c_char,
                  &mut tv as *mut timeval);
    s = paste_make_sample(pb);
    format_add(ft, b"buffer_sample\x00" as *const u8 as *const libc::c_char,
               b"%s\x00" as *const u8 as *const libc::c_char, s);
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn format_lost_client(mut c: *mut client) -> () {
    if (*c).jobs != 0 as *mut libc::c_void as *mut format_job_tree {
        format_job_tidy((*c).jobs, 1i32);
    }
    free((*c).jobs as *mut libc::c_void);
}
unsafe extern "C" fn format_job_tree_RB_NFIND(mut head: *mut format_job_tree,
                                              mut elm: *mut format_job)
 -> *mut format_job {
    let mut tmp: *mut format_job = (*head).rbh_root;
    let mut res: *mut format_job = 0 as *mut format_job;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = format_job_cmp(elm, tmp);
            if comp < 0i32 {
                res = tmp;
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return res }
    };
}
unsafe extern "C" fn format_job_tree_RB_PREV(mut elm: *mut format_job)
 -> *mut format_job {
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
unsafe extern "C" fn format_entry_tree_RB_NFIND(mut head:
                                                    *mut format_entry_tree,
                                                mut elm: *mut format_entry)
 -> *mut format_entry {
    let mut tmp: *mut format_entry = (*head).rbh_root;
    let mut res: *mut format_entry = 0 as *mut format_entry;
    let mut comp: libc::c_int = 0;
    loop  {
        if !tmp.is_null() {
            comp = format_entry_cmp(elm, tmp);
            if comp < 0i32 {
                res = tmp;
                tmp = (*tmp).entry.rbe_left
            } else if comp > 0i32 {
                tmp = (*tmp).entry.rbe_right
            } else { return tmp }
        } else { return res }
    };
}
unsafe extern "C" fn format_entry_tree_RB_PREV(mut elm: *mut format_entry)
 -> *mut format_entry {
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

