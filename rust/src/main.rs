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
// FIXME: The following allows are just here because
// lots of spam is being outputted. They should not be
// required
#![allow(unused_unsafe)]
#![allow(unused_parens)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
extern crate libc;

pub mod alerts;
pub mod arguments;
pub mod attributes;
pub mod cfg;
pub mod client_;
pub mod cmd_;
pub mod cmd_attach_session;
pub mod cmd_bind_key;
pub mod cmd_break_pane;
pub mod cmd_capture_pane;
pub mod cmd_choose_tree;
pub mod cmd_command_prompt;
pub mod cmd_confirm_before;
pub mod cmd_copy_mode;
pub mod cmd_detach_client;
pub mod cmd_display_message;
pub mod cmd_display_panes;
pub mod cmd_find_window;
pub mod cmd_find;
pub mod cmd_if_shell;
pub mod cmd_join_pane;
pub mod cmd_kill_pane;
pub mod cmd_kill_server;
pub mod cmd_kill_session;
pub mod cmd_kill_window;
pub mod cmd_list_;
pub mod cmd_list_buffers;
pub mod cmd_list_clients;
pub mod cmd_list_keys;
pub mod cmd_list_panes;
pub mod cmd_list_sessions;
pub mod cmd_list_windows;
pub mod cmd_load_buffer;
pub mod cmd_lock_server;
pub mod cmd_move_window;
pub mod cmd_new_session;
pub mod cmd_new_window;
pub mod cmd_paste_buffer;
pub mod cmd_pipe_pane;
pub mod cmd_queue;
pub mod cmd_refresh_client;
pub mod cmd_rename_session;
pub mod cmd_rename_window;
pub mod cmd_resize_pane;
pub mod cmd_respawn_pane;
pub mod cmd_respawn_window;
pub mod cmd_rotate_window;
pub mod cmd_run_shell;
pub mod cmd_save_buffer;
pub mod cmd_select_layout;
pub mod cmd_select_pane;
pub mod cmd_select_window;
pub mod cmd_send_keys;
pub mod cmd_set_buffer;
pub mod cmd_set_environment;
pub mod cmd_set_hook;
pub mod cmd_set_option;
pub mod cmd_show_environment;
pub mod cmd_show_messages;
pub mod cmd_show_options;
pub mod cmd_source_files;
pub mod cmd_split_window;
pub mod cmd_string;
pub mod cmd_swap_pane;
pub mod cmd_swap_window;
pub mod cmd_switch_client;
pub mod cmd_unbind_key;
pub mod cmd_wait_for;
pub mod colour;
pub mod compat;
pub mod control_notify;
pub mod control;
pub mod environ_;
pub mod format;
pub mod grid_;
pub mod grid_view;
pub mod hooks_;
pub mod input;
pub mod input_keys;
pub mod job_;
pub mod key_bindings_;
pub mod key_string;
pub mod layout_custom;
pub mod layout_set;
pub mod layout;
pub mod log;
pub mod mode_tree;
pub mod names;
pub mod notify;
pub mod options_;
pub mod osdep;
// FIXME: .as_mut_ptr() in statics. Maybe we can initialize in main..
pub mod options_table;
pub mod paste;
pub mod proc_;
pub mod resize;
pub mod screen_;
pub mod screen_redraw;
pub mod screen_write;
pub mod server_client;
pub mod server_fn;
pub mod server;
pub mod session_;
pub mod status;
pub mod style;
pub mod tty_;
pub mod tty_acs;
pub mod tty_keys;
pub mod tty_term_;
pub mod utf8;
pub mod window_;
pub mod window_buffer;
pub mod window_client;
pub mod window_clock;
pub mod window_copy;
pub mod window_tree;
pub mod xmalloc;
pub mod xterm_keys;

extern "C" {
    pub type options_entry;
    pub type args_entry;
    pub type bufferevent_ops;
    pub type environ;
    pub type input_ctx;
    pub type format_job_tree;
    pub type options;
    pub type event_base;
    pub type format_tree;
    pub type tty_code;
    pub type _IO_FILE_plus;
    pub type screen_titles;
    pub type hooks;
    pub type tmuxproc;
    pub type tmuxpeer;
    pub type evbuffer;
    #[no_mangle]
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut program_invocation_name: *mut libc::c_char;
    #[no_mangle]
    static mut program_invocation_short_name: *mut libc::c_char;
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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    static mut _sys_nerr: libc::c_int;
    #[no_mangle]
    static _sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, ...) -> libc::c_int;
    #[no_mangle]
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
    #[no_mangle]
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn realpath(__name: *const libc::c_char, __resolved: *mut libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcasestr(__haystack: *const libc::c_char,
                  __needle: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    static mut __tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut __daylight: libc::c_int;
    #[no_mangle]
    static mut __timezone: libc::c_long;
    #[no_mangle]
    static mut tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    fn tzset() -> ();
    #[no_mangle]
    static mut daylight: libc::c_int;
    #[no_mangle]
    static mut timezone: libc::c_long;
    #[no_mangle]
    static mut getdate_err: libc::c_int;
    #[no_mangle]
    fn access(__name: *const libc::c_char, __type: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn getuid() -> __uid_t;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    fn err(_: libc::c_int, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn errx(_: libc::c_int, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> libc::c_ulong;
    #[no_mangle]
    fn getprogname() -> *const libc::c_char;
    #[no_mangle]
    fn getptmfd() -> libc::c_int;
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
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn set_cfg_file(_: *const libc::c_char) -> ();
    #[no_mangle]
    fn hooks_create(_: *mut hooks) -> *mut hooks;
    #[no_mangle]
    fn options_create(_: *mut options) -> *mut options;
    #[no_mangle]
    fn options_default(_: *mut options, _: *const options_table_entry)
     -> *mut options_entry;
    #[no_mangle]
    fn options_set_string(_: *mut options, _: *const libc::c_char,
                          _: libc::c_int, _: *const libc::c_char, ...)
     -> *mut options_entry;
    #[no_mangle]
    fn options_set_number(_: *mut options, _: *const libc::c_char,
                          _: libc::c_longlong) -> *mut options_entry;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn environ_create() -> *mut environ;
    #[no_mangle]
    fn environ_set(_: *mut environ, _: *const libc::c_char,
                   _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn environ_put(_: *mut environ, _: *const libc::c_char) -> ();
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn client_main(_: *mut event_base, _: libc::c_int,
                   _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_int;
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
    #[no_mangle]
    fn osdep_event_init() -> *mut event_base;
    #[no_mangle]
    fn log_add_level() -> ();
}
pub const _NL_WABMON_9: unnamed_9 = 131146;
pub const _NL_CTYPE_MAP_TO_NONASCII: unnamed_9 = 70;
pub const _NL_CTYPE_EXTRA_MAP_2: unnamed_9 = 73;
pub const _NL_CTYPE_EXTRA_MAP_6: unnamed_9 = 77;
pub const _NL_CTYPE_OUTDIGIT8_WC: unnamed_9 = 59;
pub const _NL_CTYPE_INDIGITS1_MB: unnamed_9 = 21;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const __MON_GROUPING: unnamed_9 = 262148;
pub const _NL_WABMON_11: unnamed_9 = 131148;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_20,
}
pub const MON_6: unnamed_9 = 131103;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_37,
    pub ev_next: unnamed_27,
    pub ev_timeout_pos: unnamed_21,
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
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: unnamed_9 = 262180;
pub const _NL_TELEPHONE_TEL_DOM_FMT: unnamed_9 = 655361;
pub const TTY_UNKNOWN: unnamed_6 = 6;
pub const _NL_WMON_9: unnamed_9 = 131158;
pub const _NL_CTYPE_OUTDIGIT6_MB: unnamed_9 = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: unnamed_9 = 46;
pub const _NL_CTYPE_OUTDIGIT3_WC: unnamed_9 = 54;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const _NL_WMON_10: unnamed_9 = 131159;
pub type size_t = libc::c_ulong;
pub const _NL_IDENTIFICATION_LANGUAGE: unnamed_9 = 786439;
pub const __NEGATIVE_SIGN: unnamed_9 = 262150;
pub const _NL_NAME_CODESET: unnamed_9 = 524294;
pub const _NL_CTYPE_TRANSLIT_IGNORE: unnamed_9 = 69;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: unnamed_9 = 262173;
pub const _NL_MONETARY_CRNCYSTR: unnamed_9 = 262159;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_8,
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
pub const _NL_WDAY_3: unnamed_9 = 131133;
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
pub const MON_7: unnamed_9 = 131104;
pub const _NL_PAPER_CODESET: unnamed_9 = 458754;
pub const _NL_MESSAGES_CODESET: unnamed_9 = 327684;
pub const _NL_WMON_11: unnamed_9 = 131160;
pub const _NL_WDAY_6: unnamed_9 = 131136;
pub const _NL_MONETARY_DUO_VALID_TO: unnamed_9 = 262185;
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_5,
}
pub const _NL_CTYPE_EXTRA_MAP_11: unnamed_9 = 82;
pub type cmdq_type = libc::c_uint;
pub const TTY_VT100: unnamed_6 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const _NL_ADDRESS_COUNTRY_ISBN: unnamed_9 = 589831;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const _NL_TIME_ERA_NUM_ENTRIES: unnamed_9 = 131122;
pub const _NL_IDENTIFICATION_TERRITORY: unnamed_9 = 786440;
pub const _NL_CTYPE_INDIGITS3_MB: unnamed_9 = 23;
pub type __u_short = libc::c_ushort;
pub const _NL_CTYPE_CLASS_NAMES: unnamed_9 = 10;
pub const _NL_PAPER_HEIGHT: unnamed_9 = 458752;
pub const ABMON_9: unnamed_9 = 131094;
pub const _NL_CTYPE_GAP3: unnamed_9 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const __GROUPING: unnamed_9 = 65538;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const _NL_WMON_5: unnamed_9 = 131154;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: unnamed_9 = 64;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const _NL_NUM_LC_TELEPHONE: unnamed_9 = 655365;
pub const _NL_CTYPE_INDIGITS9_WC: unnamed_9 = 40;
pub type __off64_t = libc::c_long;
pub const _NL_MONETARY_UNO_VALID_TO: unnamed_9 = 262183;
pub const __FRAC_DIGITS: unnamed_9 = 262152;
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
pub struct windows {
    pub rbh_root: *mut window,
}
pub type uint32_t = libc::c_uint;
pub const _NL_CTYPE_EXTRA_MAP_4: unnamed_9 = 75;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub const _NL_COLLATE_CODESET: unnamed_9 = 196626;
pub const _NL_TIME_FIRST_WORKDAY: unnamed_9 = 131177;
pub const __INT_CURR_SYMBOL: unnamed_9 = 262144;
pub const _NL_WMON_4: unnamed_9 = 131153;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_4 {
    offset: u_int,
    data: unnamed,
}
pub const _NL_CTYPE_INDIGITS0_MB: unnamed_9 = 20;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type unnamed_6 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const _NL_CTYPE_OUTDIGIT0_MB: unnamed_9 = 41;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub ev_io_next: unnamed_34,
    pub ev_timeout: timeval,
}
pub type cmd_retval = libc::c_int;
pub const ABMON_2: unnamed_9 = 131087;
pub const __INT_N_SIGN_POSN: unnamed_9 = 262165;
pub type __ino_t = libc::c_ulong;
pub const _NL_WABDAY_1: unnamed_9 = 131124;
pub const _NL_WABMON_2: unnamed_9 = 131139;
pub const _NL_TIME_FIRST_WEEKDAY: unnamed_9 = 131176;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type unnamed_8 = libc::c_uint;
pub const _NL_WMON_1: unnamed_9 = 131150;
pub const ERA_D_T_FMT: unnamed_9 = 131120;
pub const __THOUSANDS_SEP: unnamed_9 = 65537;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: unnamed_9 = 262178;
pub const _NL_CTYPE_OUTDIGIT4_MB: unnamed_9 = 45;
pub const _NL_MONETARY_CODESET: unnamed_9 = 262189;
pub const MON_5: unnamed_9 = 131102;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: unnamed_9 = 262171;
pub const _NL_CTYPE_GAP2: unnamed_9 = 4;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: unnamed_9 = 196621;
pub type unnamed_9 = libc::c_uint;
pub const _NL_ADDRESS_COUNTRY_AB3: unnamed_9 = 589828;
pub const _NL_WERA_T_FMT: unnamed_9 = 131172;
pub const ABDAY_4: unnamed_9 = 131075;
pub const TTY_VT320: unnamed_6 = 4;
pub const MON_4: unnamed_9 = 131101;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const _NL_COLLATE_TABLEWC: unnamed_9 = 196617;
pub const _NL_WABMON_12: unnamed_9 = 131149;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const _NL_CTYPE_EXTRA_MAP_7: unnamed_9 = 78;
pub const __POSITIVE_SIGN: unnamed_9 = 262149;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: unnamed_9 = 262175;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: unnamed_9 = 262188;
pub const _NL_NUM: unnamed_9 = 786449;
pub const _NL_CTYPE_OUTDIGIT2_WC: unnamed_9 = 53;
pub const _NL_NAME_NAME_GEN: unnamed_9 = 524289;
pub type uint8_t = libc::c_uchar;
pub const _NL_CTYPE_CLASS_OFFSET: unnamed_9 = 17;
pub const TTY_VT102: unnamed_6 = 2;
pub const _NL_WABMON_8: unnamed_9 = 131145;
pub const _NL_COLLATE_SYMB_EXTRAMB: unnamed_9 = 196623;
pub const ERA_D_FMT: unnamed_9 = 131118;
pub const MON_1: unnamed_9 = 131098;
pub type key_code = libc::c_ulonglong;
pub type __u_char = libc::c_uchar;
pub const __MON_THOUSANDS_SEP: unnamed_9 = 262147;
pub const _NL_COLLATE_TABLEMB: unnamed_9 = 196610;
pub const __NOSTR: unnamed_9 = 327683;
pub const ABMON_4: unnamed_9 = 131089;
pub const _NL_CTYPE_GAP4: unnamed_9 = 7;
pub type __u_int = libc::c_uint;
pub const _NL_MONETARY_CONVERSION_RATE: unnamed_9 = 262186;
pub const _NL_COLLATE_EXTRAWC: unnamed_9 = 196619;
pub const _NL_CTYPE_OUTDIGIT2_MB: unnamed_9 = 43;
pub const _NL_CTYPE_INDIGITS2_MB: unnamed_9 = 22;
pub const _NL_CTYPE_OUTDIGIT5_WC: unnamed_9 = 56;
pub const _NL_CTYPE_MB_CUR_MAX: unnamed_9 = 13;
pub type __gid_t = libc::c_uint;
pub const _NL_IDENTIFICATION_EMAIL: unnamed_9 = 786436;
pub type _IO_lock_t = ();
pub type unnamed_12 = libc::c_uint;
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
    pub entry: unnamed_31,
    pub tree_entry: unnamed_11,
}
pub const _NL_IDENTIFICATION_CONTACT: unnamed_9 = 786435;
pub const _NL_CTYPE_OUTDIGIT1_WC: unnamed_9 = 52;
pub const _NL_NUM_LC_CTYPE: unnamed_9 = 86;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const _NL_CTYPE_OUTDIGIT1_MB: unnamed_9 = 42;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: unnamed_9 = 262177;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_25,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const _NL_WABMON_5: unnamed_9 = 131142;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const RADIXCHAR: unnamed_9 = 65536;
pub const MON_9: unnamed_9 = 131106;
pub const _NL_TELEPHONE_INT_PREFIX: unnamed_9 = 655363;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_12,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub const _NL_ADDRESS_LANG_NAME: unnamed_9 = 589832;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const PROMPT_COMMAND: unnamed_14 = 1;
pub type u_short = __u_short;
pub type __uid_t = libc::c_uint;
pub const _NL_NUMERIC_CODESET: unnamed_9 = 65541;
pub const _NL_TIME_WEEK_NDAYS: unnamed_9 = 131173;
pub const PROMPT_ENTRY: unnamed_14 = 0;
pub const _NL_MONETARY_UNO_VALID_FROM: unnamed_9 = 262182;
pub const ABDAY_1: unnamed_9 = 131072;
pub const _NL_CTYPE_INDIGITS9_MB: unnamed_9 = 29;
pub const JOB_DEAD: unnamed_8 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: unnamed_9 = 66;
pub type __suseconds_t = libc::c_long;
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
pub const _NL_W_DATE_FMT: unnamed_9 = 131181;
pub const _NL_TELEPHONE_TEL_INT_FMT: unnamed_9 = 655360;
pub const _NL_CTYPE_INDIGITS5_WC: unnamed_9 = 36;
pub type nl_item = libc::c_int;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const _NL_WT_FMT_AMPM: unnamed_9 = 131167;
pub type layout_type = libc::c_uint;
pub const _DATE_FMT: unnamed_9 = 131180;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_18,
}
pub const DAY_7: unnamed_9 = 131085;
pub const __ERA_YEAR: unnamed_9 = 131117;
pub const MON_11: unnamed_9 = 131108;
pub const _NL_COLLATE_COLLSEQMB: unnamed_9 = 196624;
pub const _NL_IDENTIFICATION_REVISION: unnamed_9 = 786444;
pub const _NL_NAME_NAME_FMT: unnamed_9 = 524288;
pub type time_t = __time_t;
pub const _NL_ADDRESS_LANG_LIB: unnamed_9 = 589835;
pub const MON_3: unnamed_9 = 131100;
pub const THOUSEP: unnamed_9 = 65537;
pub const __INT_FRAC_DIGITS: unnamed_9 = 262151;
pub const _NL_CTYPE_INDIGITS3_WC: unnamed_9 = 34;
pub const _NL_NAME_NAME_MS: unnamed_9 = 524293;
pub const __INT_N_CS_PRECEDES: unnamed_9 = 262162;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const __INT_P_SIGN_POSN: unnamed_9 = 262164;
pub type unnamed_14 = libc::c_uint;
pub type uid_t = __uid_t;
pub const _NL_WABMON_7: unnamed_9 = 131144;
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
    pub entry: unnamed_16,
    pub wentry: unnamed_0,
    pub sentry: unnamed_29,
}
pub const _NL_MEASUREMENT_MEASUREMENT: unnamed_9 = 720896;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const _NL_CTYPE_WIDTH: unnamed_9 = 12;
pub const _NL_ADDRESS_COUNTRY_NAME: unnamed_9 = 589825;
pub const _NL_WMON_12: unnamed_9 = 131161;
pub const _NL_CTYPE_INDIGITS4_WC: unnamed_9 = 35;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const _NL_ADDRESS_CODESET: unnamed_9 = 589836;
pub const _NL_CTYPE_EXTRA_MAP_12: unnamed_9 = 83;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_4,
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
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub const __P_CS_PRECEDES: unnamed_9 = 262153;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: unnamed_9 = 262181;
pub const _NL_CTYPE_INDIGITS7_MB: unnamed_9 = 27;
pub const _NL_WMON_3: unnamed_9 = 131152;
pub const ABMON_10: unnamed_9 = 131095;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_39,
}
pub const _NL_CTYPE_EXTRA_MAP_13: unnamed_9 = 84;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const ABMON_6: unnamed_9 = 131091;
pub const _NL_CTYPE_GAP5: unnamed_9 = 8;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: unnamed_9 = 67;
pub const _NL_MONETARY_DUO_VALID_FROM: unnamed_9 = 262184;
pub const __MON_DECIMAL_POINT: unnamed_9 = 262146;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const _NL_NUM_LC_COLLATE: unnamed_9 = 196627;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: unnamed_9 = 68;
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
pub const _NL_ADDRESS_POSTAL_FMT: unnamed_9 = 589824;
pub const _NL_WABDAY_5: unnamed_9 = 131128;
pub const _NL_NAME_NAME_MISS: unnamed_9 = 524292;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const _NL_COLLATE_COLLSEQWC: unnamed_9 = 196625;
pub const __YESSTR: unnamed_9 = 327682;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const _NL_CTYPE_INDIGITS2_WC: unnamed_9 = 33;
pub const _NL_COLLATE_INDIRECTMB: unnamed_9 = 196613;
pub const LINE_SEL_LEFT_RIGHT: unnamed_12 = 1;
pub const _NL_NUM_LC_TIME: unnamed_9 = 131183;
pub const _NL_IDENTIFICATION_ABBREVIATION: unnamed_9 = 786443;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const DAY_4: unnamed_9 = 131082;
pub const LINE_SEL_RIGHT_LEFT: unnamed_12 = 2;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: unnamed_9 = 262172;
pub const _NL_WABMON_6: unnamed_9 = 131143;
pub const _NL_WMON_7: unnamed_9 = 131156;
pub const ABDAY_7: unnamed_9 = 131078;
pub const _NL_CTYPE_INDIGITS0_WC: unnamed_9 = 31;
pub const T_FMT_AMPM: unnamed_9 = 131115;
pub const _NL_IDENTIFICATION_ADDRESS: unnamed_9 = 786434;
pub const _NL_WABMON_4: unnamed_9 = 131141;
pub const _NL_ADDRESS_COUNTRY_AB2: unnamed_9 = 589827;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: unnamed_9 = 262168;
pub const __P_SIGN_POSN: unnamed_9 = 262157;
pub const _NL_CTYPE_EXTRA_MAP_8: unnamed_9 = 79;
pub const _NL_WDAY_7: unnamed_9 = 131137;
pub const _NL_CTYPE_OUTDIGIT7_MB: unnamed_9 = 48;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: unnamed_9 = 65540;
pub const _NL_CTYPE_CLASS32: unnamed_9 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub const _NL_WMON_2: unnamed_9 = 131151;
pub const _NL_CTYPE_INDIGITS6_MB: unnamed_9 = 26;
pub const _NL_NUM_LC_PAPER: unnamed_9 = 458755;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: unnamed_9 = 62;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const _NL_CTYPE_CODESET_NAME: unnamed_9 = 14;
pub const _NL_ADDRESS_LANG_AB: unnamed_9 = 589833;
pub const ABMON_12: unnamed_9 = 131097;
pub const _NL_CTYPE_MAP_NAMES: unnamed_9 = 11;
pub const _NL_IDENTIFICATION_TEL: unnamed_9 = 786437;
pub const ALT_DIGITS: unnamed_9 = 131119;
pub const __N_SIGN_POSN: unnamed_9 = 262158;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const _NL_WMON_6: unnamed_9 = 131155;
pub type tcflag_t = libc::c_uint;
pub const _NL_PAPER_WIDTH: unnamed_9 = 458753;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_21 {
    ev_next_with_common_timeout: unnamed_3,
    min_heap_idx: libc::c_int,
}
pub const DAY_3: unnamed_9 = 131081;
pub const _NL_ADDRESS_COUNTRY_CAR: unnamed_9 = 589829;
pub const _NL_WABMON_1: unnamed_9 = 131138;
pub const _NL_CTYPE_INDIGITS7_WC: unnamed_9 = 38;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub ev_signal_next: unnamed_19,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const _NL_COLLATE_RULESETS: unnamed_9 = 196609;
pub const _NL_IDENTIFICATION_FAX: unnamed_9 = 786438;
pub const DAY_5: unnamed_9 = 131083;
pub const _NL_IDENTIFICATION_SOURCE: unnamed_9 = 786433;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: unnamed_9 = 63;
pub const _NL_CTYPE_GAP1: unnamed_9 = 2;
pub const _NL_NUM_LC_MONETARY: unnamed_9 = 262190;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_30,
}
pub const __INT_N_SEP_BY_SPACE: unnamed_9 = 262163;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: unnamed_9 = 262179;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: unnamed_9 = 262166;
pub const _NL_COLLATE_INDIRECTWC: unnamed_9 = 196620;
pub const _NL_TELEPHONE_INT_SELECT: unnamed_9 = 655362;
pub const _NL_IDENTIFICATION_APPLICATION: unnamed_9 = 786442;
pub const _NL_NUM_LC_ADDRESS: unnamed_9 = 589837;
pub const __INT_P_CS_PRECEDES: unnamed_9 = 262160;
pub const _NL_TELEPHONE_CODESET: unnamed_9 = 655364;
pub const ABDAY_6: unnamed_9 = 131077;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub const __N_CS_PRECEDES: unnamed_9 = 262155;
pub const ABMON_7: unnamed_9 = 131092;
pub const ABDAY_3: unnamed_9 = 131074;
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const __N_SEP_BY_SPACE: unnamed_9 = 262156;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const _NL_WAM_STR: unnamed_9 = 131162;
pub const MON_2: unnamed_9 = 131099;
pub const AM_STR: unnamed_9 = 131110;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: unnamed_9 = 61;
pub const _NL_WABMON_10: unnamed_9 = 131147;
pub const __DECIMAL_POINT: unnamed_9 = 65536;
pub const _NL_NUM_LC_NAME: unnamed_9 = 524295;
pub const _NL_CTYPE_OUTDIGIT6_WC: unnamed_9 = 57;
pub const _NL_WDAY_4: unnamed_9 = 131134;
pub const _NL_CTYPE_TOUPPER: unnamed_9 = 1;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const ERA_T_FMT: unnamed_9 = 131121;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: unnamed_9 = 65539;
pub const _NL_CTYPE_EXTRA_MAP_5: unnamed_9 = 76;
pub const _NL_CTYPE_GAP6: unnamed_9 = 9;
pub const JOB_CLOSED: unnamed_8 = 2;
pub const ABDAY_5: unnamed_9 = 131076;
pub const __INT_P_SEP_BY_SPACE: unnamed_9 = 262161;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub type __pid_t = libc::c_int;
pub const DAY_1: unnamed_9 = 131079;
pub const CODESET: unnamed_9 = 14;
pub const _NL_WABMON_3: unnamed_9 = 131140;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: unnamed_9 = 262169;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub const _NL_WT_FMT: unnamed_9 = 131166;
pub const __YESEXPR: unnamed_9 = 327680;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_10,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_35,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub const _NL_CTYPE_INDIGITS8_MB: unnamed_9 = 28;
pub const _NL_CTYPE_EXTRA_MAP_1: unnamed_9 = 72;
pub const TTY_VT420: unnamed_6 = 5;
pub const _NL_CTYPE_OUTDIGIT7_WC: unnamed_9 = 58;
pub const TTY_VT220: unnamed_6 = 3;
pub const _NL_NAME_NAME_MR: unnamed_9 = 524290;
pub const _NL_WD_FMT: unnamed_9 = 131165;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const MON_8: unnamed_9 = 131105;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub const _NL_CTYPE_INDIGITS1_WC: unnamed_9 = 32;
pub const _NL_CTYPE_OUTDIGIT4_WC: unnamed_9 = 55;
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
pub const _NL_CTYPE_OUTDIGIT8_MB: unnamed_9 = 49;
pub const _NL_COLLATE_NRULES: unnamed_9 = 196608;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const _NL_MONETARY_DECIMAL_POINT_WC: unnamed_9 = 262187;
pub const _NL_WDAY_5: unnamed_9 = 131135;
pub const _NL_CTYPE_EXTRA_MAP_3: unnamed_9 = 74;
pub const _NL_WERA_YEAR: unnamed_9 = 131168;
pub const _NL_WABDAY_7: unnamed_9 = 131130;
pub const DAY_2: unnamed_9 = 131080;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: unnamed_9 = 262170;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const _NL_ADDRESS_COUNTRY_POST: unnamed_9 = 589826;
pub const _NL_TIME_CODESET: unnamed_9 = 131182;
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
    pub winlinks: unnamed_15,
    pub entry: unnamed_1,
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
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_33,
    pub entry: unnamed_23,
}
pub const _NL_WABDAY_4: unnamed_9 = 131127;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_30 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const _NL_CTYPE_NONASCII_CASE: unnamed_9 = 71;
pub const ABMON_1: unnamed_9 = 131086;
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
pub type __blksize_t = libc::c_long;
pub const _NL_CTYPE_EXTRA_MAP_14: unnamed_9 = 85;
pub const _NL_CTYPE_TOLOWER32: unnamed_9 = 16;
pub const _NL_NUM_LC_IDENTIFICATION: unnamed_9 = 786448;
pub const ABMON_5: unnamed_9 = 131090;
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
    pub entry: unnamed_24,
}
pub const _NL_NUM_LC_NUMERIC: unnamed_9 = 65542;
pub type cc_t = libc::c_uchar;
pub const D_FMT: unnamed_9 = 131113;
pub const PM_STR: unnamed_9 = 131111;
pub const _NL_CTYPE_INDIGITS8_WC: unnamed_9 = 39;
pub const LINE_SEL_NONE: unnamed_12 = 0;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: unnamed_9 = 65;
pub const _NL_CTYPE_INDIGITS6_WC: unnamed_9 = 37;
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
    pub entry: unnamed_38,
}
pub type bitstr_t = libc::c_uchar;
pub const _NL_NUM_LC_MESSAGES: unnamed_9 = 327685;
pub type FILE = _IO_FILE;
pub const _NL_WABDAY_2: unnamed_9 = 131125;
pub const _NL_IDENTIFICATION_AUDIENCE: unnamed_9 = 786441;
pub const _NL_CTYPE_OUTDIGIT9_MB: unnamed_9 = 50;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const _NL_CTYPE_MAP_OFFSET: unnamed_9 = 18;
pub const D_T_FMT: unnamed_9 = 131112;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: unnamed_9 = 262176;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const __P_SEP_BY_SPACE: unnamed_9 = 262154;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub type __blkcnt_t = libc::c_long;
pub const ERA: unnamed_9 = 131116;
pub type __dev_t = libc::c_ulong;
pub const _NL_CTYPE_OUTDIGIT9_WC: unnamed_9 = 60;
pub const _NL_CTYPE_CLASS: unnamed_9 = 0;
pub const _NL_CTYPE_INDIGITS4_MB: unnamed_9 = 24;
pub type __nlink_t = libc::c_ulong;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_32 {
    ev_io: unnamed_7,
    ev_signal: unnamed_22,
}
pub const _NL_CTYPE_OUTDIGIT0_WC: unnamed_9 = 51;
pub const _NL_ADDRESS_COUNTRY_NUM: unnamed_9 = 589830;
pub const _NL_WDAY_1: unnamed_9 = 131131;
pub const ABMON_8: unnamed_9 = 131093;
pub const ABDAY_2: unnamed_9 = 131073;
pub const _NL_WPM_STR: unnamed_9 = 131163;
pub const _NL_IDENTIFICATION_DATE: unnamed_9 = 786445;
pub const _NL_WERA_D_T_FMT: unnamed_9 = 131171;
pub const TTY_VT101: unnamed_6 = 1;
pub const _NL_COLLATE_GAP1: unnamed_9 = 196614;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const JOB_RUNNING: unnamed_8 = 0;
pub const _NL_NUM_LC_MEASUREMENT: unnamed_9 = 720898;
pub type speed_t = libc::c_uint;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: unnamed_9 = 262167;
pub const _NL_CTYPE_TOLOWER: unnamed_9 = 3;
pub const _NL_WABDAY_3: unnamed_9 = 131126;
pub const ABMON_11: unnamed_9 = 131096;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const _NL_WABDAY_6: unnamed_9 = 131129;
pub const _NL_COLLATE_SYMB_TABLEMB: unnamed_9 = 196622;
pub type __syscall_slong_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const _NL_IDENTIFICATION_TITLE: unnamed_9 = 786432;
pub const _NL_IDENTIFICATION_CODESET: unnamed_9 = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: unnamed_9 = 786446;
pub type uint16_t = libc::c_ushort;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const _NL_TIME_WEEK_1STWEEK: unnamed_9 = 131175;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type cmd_find_type = libc::c_uint;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const MON_12: unnamed_9 = 131109;
pub const _NL_TIME_ERA_ENTRIES: unnamed_9 = 131123;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const _NL_CTYPE_INDIGITS_WC_LEN: unnamed_9 = 30;
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
pub type __time_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const MON_10: unnamed_9 = 131107;
pub const _NL_WERA_D_FMT: unnamed_9 = 131169;
pub const _NL_CTYPE_INDIGITS5_MB: unnamed_9 = 25;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub type u_int = __u_int;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const DAY_6: unnamed_9 = 131084;
pub const _NL_MEASUREMENT_CODESET: unnamed_9 = 720897;
pub const _NL_WD_T_FMT: unnamed_9 = 131164;
pub const _NL_WDAY_2: unnamed_9 = 131132;
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
    pub prompt_mode: unnamed_14,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_28,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
pub const _NL_COLLATE_GAP3: unnamed_9 = 196616;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const _NL_ADDRESS_LANG_TERM: unnamed_9 = 589834;
pub const _NL_COLLATE_GAP2: unnamed_9 = 196615;
pub type __mode_t = libc::c_uint;
pub const _NL_NAME_NAME_MRS: unnamed_9 = 524291;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const _NL_TIME_WEEK_1STDAY: unnamed_9 = 131174;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub type options_table_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub const _NL_COLLATE_EXTRAMB: unnamed_9 = 196612;
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
pub const _NL_WALT_DIGITS: unnamed_9 = 131170;
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
    pub term_type: unnamed_6,
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
pub const _NL_TIME_CAL_DIRECTION: unnamed_9 = 131178;
pub const _NL_CTYPE_INDIGITS_MB_LEN: unnamed_9 = 19;
pub const _NL_WMON_8: unnamed_9 = 131157;
pub const _NL_COLLATE_WEIGHTWC: unnamed_9 = 196618;
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
pub const __NOEXPR: unnamed_9 = 327681;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
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
pub const _NL_COLLATE_WEIGHTMB: unnamed_9 = 196611;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub const _NL_CTYPE_EXTRA_MAP_10: unnamed_9 = 81;
pub const ABMON_3: unnamed_9 = 131088;
pub const _NL_TIME_TIMEZONE: unnamed_9 = 131179;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: unnamed_9 = 262174;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const __CURRENCY_SYMBOL: unnamed_9 = 262145;
pub const T_FMT: unnamed_9 = 131114;
pub const _NL_CTYPE_TOUPPER32: unnamed_9 = 15;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pid_t = __pid_t;
pub const _NL_CTYPE_OUTDIGIT3_MB: unnamed_9 = 44;
pub type u_char = __u_char;
pub const _NL_CTYPE_EXTRA_MAP_9: unnamed_9 = 80;
#[no_mangle]
pub static mut global_hooks: *mut hooks =
    unsafe { 0 as *const hooks as *mut hooks };
#[no_mangle]
pub static mut global_options: *mut options =
    unsafe { 0 as *const options as *mut options };
#[no_mangle]
pub static mut global_s_options: *mut options =
    unsafe { 0 as *const options as *mut options };
#[no_mangle]
pub static mut global_w_options: *mut options =
    unsafe { 0 as *const options as *mut options };
#[no_mangle]
pub static mut global_environ: *mut environ =
    unsafe { 0 as *const environ as *mut environ };
#[no_mangle]
pub static mut start_time: timeval =
    unsafe { timeval{tv_sec: 0, tv_usec: 0,} };
#[no_mangle]
pub static mut socket_path: *const libc::c_char =
    unsafe { 0 as *const libc::c_char };
#[no_mangle]
pub static mut shell_command: *const libc::c_char =
    unsafe { 0 as *const libc::c_char };
#[no_mangle]
pub static mut ptm_fd: libc::c_int = -1i32;
#[no_mangle]
pub unsafe extern "C" fn areshell(mut shell: *const libc::c_char)
 -> libc::c_int {
    let mut progname: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    ptr = strrchr(shell, 47);
    if ptr != 0 as *mut libc::c_void as *const libc::c_char {
        ptr = ptr.offset(1isize)
    } else { ptr = shell }
    progname = getprogname();
    if *progname as libc::c_int == 45 { progname = progname.offset(1isize) }
    if strcmp(ptr, progname) == 0i32 { return 1i32 } else { return 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn setblocking(mut fd: libc::c_int,
                                     mut state: libc::c_int) -> () {
    let mut mode: libc::c_int = 0;
    mode = fcntl(fd, 3i32);
    if mode != 1i32.wrapping_neg() {
        if 0 == state { mode |= 2048i32 } else { mode &= !2048i32 }
        fcntl(fd, 4i32, mode);
    };
}
#[no_mangle]
pub unsafe extern "C" fn find_home() -> *const libc::c_char {
    let mut pw: *mut passwd = 0 as *mut passwd;
    static mut home: *const libc::c_char =
        unsafe { 0 as *const libc::c_char };
    if home != 0 as *mut libc::c_void as *const libc::c_char {
        return home
    } else {
        home = getenv(b"HOME\x00" as *const u8 as *const libc::c_char);
        if home == 0 as *mut libc::c_void as *const libc::c_char ||
               *home as libc::c_int == 0 {
            pw = getpwuid(getuid());
            if pw != 0 as *mut libc::c_void as *mut passwd {
                home = (*pw).pw_dir
            } else { home = 0 as *const libc::c_char }
        }
        return home
    };
}
unsafe extern "C" fn usage() -> ! {
    fprintf(stderr,
            b"usage: %s [-2CluvV] [-c shell-command] [-f file] [-L socket-name]\n            [-S socket-path] [command [flags]]\n\x00"
                as *const u8 as *const libc::c_char, getprogname());
    exit(1i32);
}
unsafe extern "C" fn make_label(mut label: *const libc::c_char,
                                mut cause: *mut *mut libc::c_char)
 -> *mut libc::c_char {
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut resolved: [libc::c_char; 4096] = [0; 4096];
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sb: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut uid: uid_t = 0;
    *cause = 0 as *mut libc::c_char;
    if label == 0 as *mut libc::c_void as *const libc::c_char {
        label = b"default\x00" as *const u8 as *const libc::c_char
    }
    uid = getuid();
    s = getenv(b"TMUX_TMPDIR\x00" as *const u8 as *const libc::c_char);
    if s != 0 as *mut libc::c_void as *mut libc::c_char &&
           *s as libc::c_int != 0 {
        xasprintf(&mut base as *mut *mut libc::c_char,
                  b"%s/tmux-%ld\x00" as *const u8 as *const libc::c_char, s,
                  uid as libc::c_long);
    } else {
        xasprintf(&mut base as *mut *mut libc::c_char,
                  b"%s/tmux-%ld\x00" as *const u8 as *const libc::c_char,
                  b"/tmp/\x00" as *const u8 as *const libc::c_char,
                  uid as libc::c_long);
    }
    if realpath(base, resolved.as_mut_ptr()) ==
           0 as *mut libc::c_void as *mut libc::c_char &&
           strlcpy(resolved.as_mut_ptr(), base,
                   ::std::mem::size_of::<[libc::c_char; 4096]>() as
                       libc::c_ulong) >=
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
       {
        *__errno_location() = 34i32;
        free(base as *mut libc::c_void);
    } else if !(mkdir(resolved.as_mut_ptr(),
                      (256i32 | 128i32 | 64i32) as __mode_t) != 0i32 &&
                    *__errno_location() != 17i32) {
        if !(lstat(resolved.as_mut_ptr(), &mut sb as *mut stat) != 0i32) {
            if !(sb.st_mode & 61440i32 as libc::c_uint ==
                     16384i32 as libc::c_uint) {
                *__errno_location() = 20i32
            } else if sb.st_uid != uid ||
                          sb.st_mode &
                              ((256i32 | 128i32 | 64i32) >> 3i32 >> 3i32) as
                                  libc::c_uint != 0i32 as libc::c_uint {
                *__errno_location() = 13i32
            } else {
                xasprintf(&mut path as *mut *mut libc::c_char,
                          b"%s/%s\x00" as *const u8 as *const libc::c_char,
                          resolved.as_mut_ptr(), label);
                return path
            }
        }
    }
    xasprintf(cause,
              b"error creating %s (%s)\x00" as *const u8 as
                  *const libc::c_char, resolved.as_mut_ptr(),
              strerror(*__errno_location()));
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn getshell() -> *const libc::c_char {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    shell = getenv(b"SHELL\x00" as *const u8 as *const libc::c_char);
    if 0 != checkshell(shell) {
        return shell
    } else {
        pw = getpwuid(getuid());
        if pw != 0 as *mut libc::c_void as *mut passwd &&
               0 != checkshell((*pw).pw_shell) {
            return (*pw).pw_shell
        } else { return b"/bin/sh\x00" as *const u8 as *const libc::c_char }
    };
}
unsafe extern "C" fn checkshell(mut shell: *const libc::c_char)
 -> libc::c_int {
    if shell == 0 as *mut libc::c_void as *const libc::c_char ||
           *shell as libc::c_int != 47 {
        return 0i32
    } else if 0 != areshell(shell) {
        return 0i32
    } else if access(shell, 1i32) != 0i32 {
        return 0i32
    } else { return 1i32 };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    use alerts::alerts_list;
    use cmd_queue::global_queue;
    use input::{input_state_ground, input_state_ground_table, input_state_esc_intermediate, input_state_esc_intermediate_table};
    use input::{input_state_dcs_intermediate, input_state_dcs_intermediate_table, input_state_dcs_ignore, input_state_dcs_ignore_table};
    use input::{input_state_dcs_parameter, input_state_dcs_parameter_table, input_state_dcs_handler, input_state_dcs_handler_table};
    use input::{input_state_csi_parameter, input_state_csi_parameter_table, input_state_dcs_escape, input_state_dcs_escape_table};
    use input::{input_state_csi_intermediate, input_state_csi_intermediate_table, input_state_csi_ignore, input_state_csi_ignore_table};
    use input::{input_state_csi_enter, input_state_csi_enter_table, input_state_osc_string, input_state_osc_string_table};
    use input::{input_state_apc_string, input_state_apc_string_table, input_state_esc_enter, input_state_esc_enter_table};
    use input::{input_state_rename_string, input_state_rename_string_table, input_state_consume_st, input_state_consume_st_table};
    use input::{input_state_dcs_enter, input_state_dcs_enter_table};
    use options_table::{options_table as opt_table, options_table_set_clipboard_list, options_table_bell_action_list};
    use options_table::{options_table_status_justify_list, options_table_status_keys_list, options_table_status_position_list};
    use options_table::{options_table_visual_bell_list, options_table_clock_mode_style_list, options_table_mode_keys_list};
    use options_table::{options_table_pane_status_list};
    use std::ffi::CStr;

    // Initialize global statics pointer fields (self-referential and otherwise)
    // any asserts are just sanity checks, to ensure the correct index was assigned
    alerts_list.tqh_last = &alerts_list.tqh_first as *const *mut _ as *mut *mut _;
    global_queue.tqh_last = &global_queue.tqh_first as *const *mut _ as *mut *mut _;
    input_state_ground.transitions = input_state_ground_table.as_ptr();
    input_state_esc_intermediate.transitions = input_state_esc_intermediate_table.as_ptr();
    input_state_dcs_intermediate.transitions = input_state_dcs_intermediate_table.as_ptr();
    input_state_dcs_ignore.transitions = input_state_dcs_ignore_table.as_ptr();
    input_state_dcs_parameter.transitions = input_state_dcs_parameter_table.as_ptr();
    input_state_dcs_handler.transitions = input_state_dcs_handler_table.as_ptr();
    input_state_csi_parameter.transitions = input_state_csi_parameter_table.as_ptr();
    input_state_dcs_escape.transitions = input_state_dcs_escape_table.as_ptr();
    input_state_csi_intermediate.transitions = input_state_csi_intermediate_table.as_ptr();
    input_state_csi_ignore.transitions = input_state_csi_ignore_table.as_ptr();
    input_state_csi_enter.transitions = input_state_csi_enter_table.as_ptr();
    input_state_apc_string.transitions = input_state_apc_string_table.as_ptr();
    input_state_osc_string.transitions = input_state_osc_string_table.as_ptr();
    input_state_esc_enter.transitions = input_state_esc_enter_table.as_ptr();
    input_state_rename_string.transitions = input_state_rename_string_table.as_ptr();
    input_state_consume_st.transitions = input_state_consume_st_table.as_ptr();
    input_state_dcs_enter.transitions = input_state_dcs_enter_table.as_ptr();
    assert_eq!(CStr::from_ptr(opt_table[9].name).to_bytes(), b"set-clipboard");
    opt_table[9].choices = options_table_set_clipboard_list.as_mut_ptr();
    assert_eq!(CStr::from_ptr(opt_table[12].name).to_bytes(), b"activity-action");
    opt_table[12].choices = options_table_bell_action_list.as_mut_ptr();
    assert_eq!(CStr::from_ptr(opt_table[15].name).to_bytes(), b"bell-action");
    opt_table[15].choices = options_table_bell_action_list.as_mut_ptr();
    assert_eq!(CStr::from_ptr(opt_table[43].name).to_bytes(), b"silence-action");
    opt_table[43].choices = options_table_bell_action_list.as_mut_ptr();
    assert_eq!(CStr::from_ptr(opt_table[49].name).to_bytes(), b"status-justify");
    opt_table[49].choices = options_table_status_justify_list.as_mut_ptr();
    assert_eq!(CStr::from_ptr(opt_table[50].name).to_bytes(), b"status-keys");
    opt_table[50].choices = options_table_status_keys_list.as_mut_ptr();
    assert_eq!(CStr::from_ptr(opt_table[57].name).to_bytes(), b"status-position");
    opt_table[57].choices = options_table_status_position_list.as_mut_ptr();
    assert_eq!(CStr::from_ptr(opt_table[66].name).to_bytes(), b"visual-activity");
    opt_table[66].choices = options_table_visual_bell_list.as_mut_ptr();
    assert_eq!(CStr::from_ptr(opt_table[67].name).to_bytes(), b"visual-bell");
    opt_table[67].choices = options_table_visual_bell_list.as_mut_ptr();
    assert_eq!(CStr::from_ptr(opt_table[68].name).to_bytes(), b"visual-silence");
    opt_table[68].choices = options_table_visual_bell_list.as_mut_ptr();
    assert_eq!(CStr::from_ptr(opt_table[76].name).to_bytes(), b"clock-mode-style");
    opt_table[76].choices = options_table_clock_mode_style_list.as_mut_ptr();
    assert_eq!(CStr::from_ptr(opt_table[84].name).to_bytes(), b"mode-keys");
    opt_table[84].choices = options_table_mode_keys_list.as_mut_ptr();
    assert_eq!(CStr::from_ptr(opt_table[98].name).to_bytes(), b"pane-border-status");
    opt_table[98].choices = options_table_pane_status_list.as_mut_ptr();

    let mut current_block: u64;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut var: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp: [libc::c_char; 4096] = [0; 4096];
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    let mut cwd: *const libc::c_char = 0 as *const libc::c_char;
    let mut opt: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut keys: libc::c_int = 0;
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
    if setlocale(0i32, b"en_US.UTF-8\x00" as *const u8 as *const libc::c_char)
           == 0 as *mut libc::c_void as *mut libc::c_char &&
           setlocale(0i32, b"C.UTF-8\x00" as *const u8 as *const libc::c_char)
               == 0 as *mut libc::c_void as *mut libc::c_char {
        if setlocale(0i32, b"\x00" as *const u8 as *const libc::c_char) ==
               0 as *mut libc::c_void as *mut libc::c_char {
            errx(1i32,
                 b"invalid LC_ALL, LC_CTYPE or LANG\x00" as *const u8 as
                     *const libc::c_char);
        }
        s = nl_langinfo(CODESET as libc::c_int);
        if strcasecmp(s, b"UTF-8\x00" as *const u8 as *const libc::c_char) !=
               0i32 &&
               strcasecmp(s, b"UTF8\x00" as *const u8 as *const libc::c_char)
                   != 0i32 {
            errx(1i32,
                 b"need UTF-8 locale (LC_CTYPE) but have %s\x00" as *const u8
                     as *const libc::c_char, s);
        }
    }
    setlocale(2i32, b"\x00" as *const u8 as *const libc::c_char);
    tzset();
    if **argv as libc::c_int == 45 { flags = 2i32 } else { flags = 0i32 }
    path = 0 as *mut libc::c_char;
    label = path;
    loop  {
        opt =
            BSDgetopt(argc, argv,
                      b"2c:Cdf:lL:qS:uUVv\x00" as *const u8 as
                          *const libc::c_char);
        if opt != 1i32.wrapping_neg() {
            match opt {
                50 => { flags |= 131072i32 }
                99 => { shell_command = BSDoptarg }
                67 => {
                    if 0 != flags & 8192i32 {
                        flags |= 16384i32
                    } else { flags |= 8192i32 }
                }
                86 => {
                    printf(b"%s %s\n\x00" as *const u8 as *const libc::c_char,
                           getprogname(),
                           b"xmaster-rs\x00" as *const u8 as
                               *const libc::c_char);
                    exit(0i32);
                }
                102 => { set_cfg_file(BSDoptarg); }
                108 => { flags |= 2i32 }
                76 => {
                    free(label as *mut libc::c_void);
                    label = xstrdup(BSDoptarg)
                }
                113 => { }
                83 => {
                    free(path as *mut libc::c_void);
                    path = xstrdup(BSDoptarg)
                }
                117 => { flags |= 65536i32 }
                118 => { log_add_level(); }
                _ => { usage(); }
            }
        } else {
            argc -= BSDoptind;
            argv = argv.offset(BSDoptind as isize);
            if shell_command != 0 as *mut libc::c_void as *const libc::c_char
                   && argc != 0i32 {
                current_block = 13586036798005543211;
                break ;
            } else { current_block = 11050875288958768710; break ; }
        }
    }
    match current_block {
        11050875288958768710 => {
            ptm_fd = getptmfd();
            if ptm_fd == 1i32.wrapping_neg() {
                err(1i32,
                    b"getptmfd\x00" as *const u8 as *const libc::c_char);
            }
            if 0i32 != 0i32 {
                err(1i32, b"pledge\x00" as *const u8 as *const libc::c_char);
            }
            if getenv(b"TMUX\x00" as *const u8 as *const libc::c_char) !=
                   0 as *mut libc::c_void as *mut libc::c_char {
                flags |= 65536i32
            } else {
                s = getenv(b"LC_ALL\x00" as *const u8 as *const libc::c_char);
                if s == 0 as *mut libc::c_void as *const libc::c_char ||
                       *s as libc::c_int == 0 {
                    s =
                        getenv(b"LC_CTYPE\x00" as *const u8 as
                                   *const libc::c_char)
                }
                if s == 0 as *mut libc::c_void as *const libc::c_char ||
                       *s as libc::c_int == 0 {
                    s =
                        getenv(b"LANG\x00" as *const u8 as
                                   *const libc::c_char)
                }
                if s == 0 as *mut libc::c_void as *const libc::c_char ||
                       *s as libc::c_int == 0 {
                    s = b"\x00" as *const u8 as *const libc::c_char
                }
                if strcasestr(s,
                              b"UTF-8\x00" as *const u8 as
                                  *const libc::c_char) !=
                       0 as *mut libc::c_void as *mut libc::c_char ||
                       strcasestr(s,
                                  b"UTF8\x00" as *const u8 as
                                      *const libc::c_char) !=
                           0 as *mut libc::c_void as *mut libc::c_char {
                    flags |= 65536i32
                }
            }
            global_hooks = hooks_create(0 as *mut hooks);
            global_environ = environ_create();
            var = environ;
            while *var != 0 as *mut libc::c_void as *mut libc::c_char {
                environ_put(global_environ, *var);
                var = var.offset(1isize)
            }
            cwd = getenv(b"PWD\x00" as *const u8 as *const libc::c_char);
            if cwd == 0 as *mut libc::c_void as *const libc::c_char &&
                   {
                       cwd =
                           getcwd(tmp.as_mut_ptr(),
                                  ::std::mem::size_of::<[libc::c_char; 4096]>()
                                      as libc::c_ulong);
                       cwd != 0 as *mut libc::c_void as *const libc::c_char
                   } {
                environ_set(global_environ,
                            b"PWD\x00" as *const u8 as *const libc::c_char,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            cwd);
            }
            global_options = options_create(0 as *mut options);
            global_s_options = options_create(0 as *mut options);
            global_w_options = options_create(0 as *mut options);
            oe = options_table.as_ptr();
            while (*oe).name != 0 as *mut libc::c_void as *const libc::c_char
                  {
                if (*oe).scope as libc::c_uint ==
                       OPTIONS_TABLE_SERVER as libc::c_int as libc::c_uint {
                    options_default(global_options, oe);
                }
                if (*oe).scope as libc::c_uint ==
                       OPTIONS_TABLE_SESSION as libc::c_int as libc::c_uint {
                    options_default(global_s_options, oe);
                }
                if (*oe).scope as libc::c_uint ==
                       OPTIONS_TABLE_WINDOW as libc::c_int as libc::c_uint {
                    options_default(global_w_options, oe);
                }
                oe = oe.offset(1isize)
            }
            shell = getshell();
            options_set_string(global_s_options,
                               b"default-shell\x00" as *const u8 as
                                   *const libc::c_char, 0i32,
                               b"%s\x00" as *const u8 as *const libc::c_char,
                               shell);
            s = getenv(b"VISUAL\x00" as *const u8 as *const libc::c_char);
            if s != 0 as *mut libc::c_void as *const libc::c_char ||
                   {
                       s =
                           getenv(b"EDITOR\x00" as *const u8 as
                                      *const libc::c_char);
                       s != 0 as *mut libc::c_void as *const libc::c_char
                   } {
                if strrchr(s, 47) !=
                       0 as *mut libc::c_void as *mut libc::c_char {
                    s = strrchr(s, 47).offset(1isize)
                }
                if strstr(s, b"vi\x00" as *const u8 as *const libc::c_char) !=
                       0 as *mut libc::c_void as *mut libc::c_char {
                    keys = 1i32
                } else { keys = 0i32 }
                options_set_number(global_s_options,
                                   b"status-keys\x00" as *const u8 as
                                       *const libc::c_char,
                                   keys as libc::c_longlong);
                options_set_number(global_w_options,
                                   b"mode-keys\x00" as *const u8 as
                                       *const libc::c_char,
                                   keys as libc::c_longlong);
            }
            if path == 0 as *mut libc::c_void as *mut libc::c_char &&
                   label == 0 as *mut libc::c_void as *mut libc::c_char {
                s = getenv(b"TMUX\x00" as *const u8 as *const libc::c_char);
                if s != 0 as *mut libc::c_void as *const libc::c_char &&
                       *s as libc::c_int != 0 && *s as libc::c_int != 44 {
                    path = xstrdup(s);
                    *path.offset(strcspn(path,
                                         b",\x00" as *const u8 as
                                             *const libc::c_char) as isize) =
                        0 as libc::c_char
                }
            }
            if path == 0 as *mut libc::c_void as *mut libc::c_char &&
                   {
                       path =
                           make_label(label,
                                      &mut cause as *mut *mut libc::c_char);
                       path == 0 as *mut libc::c_void as *mut libc::c_char
                   } {
                if cause != 0 as *mut libc::c_void as *mut libc::c_char {
                    fprintf(stderr,
                            b"%s\n\x00" as *const u8 as *const libc::c_char,
                            cause);
                    free(cause as *mut libc::c_void);
                }
                exit(1i32);
            } else {
                socket_path = path;
                free(label as *mut libc::c_void);
                exit(client_main(osdep_event_init(), argc, argv, flags));
            }
        }
        _ => { usage(); }
    };
}
fn main() -> () {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}

