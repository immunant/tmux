extern crate libc;
extern "C" {
    pub type format_tree;
    pub type ldat;
    pub type tmuxpeer;
    pub type _IO_FILE_plus;
    pub type event_base;
    pub type environ;
    pub type args_entry;
    pub type tmuxproc;
    pub type screen_titles;
    pub type bufferevent_ops;
    pub type options_entry;
    pub type hooks;
    pub type options;
    pub type evbuffer;
    pub type input_ctx;
    pub type format_job_tree;
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
    static mut acs_map: [chtype; 0];
    #[no_mangle]
    fn tigetflag(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tigetnum(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tigetstr(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn tparm(_: *const libc::c_char, ...) -> *mut libc::c_char;
    #[no_mangle]
    static mut curscr: *mut WINDOW;
    #[no_mangle]
    static mut newscr: *mut WINDOW;
    #[no_mangle]
    static mut stdscr: *mut WINDOW;
    #[no_mangle]
    static mut ttytype: [libc::c_char; 0];
    #[no_mangle]
    static mut COLORS: libc::c_int;
    #[no_mangle]
    static mut COLOR_PAIRS: libc::c_int;
    #[no_mangle]
    static mut COLS: libc::c_int;
    #[no_mangle]
    static mut ESCDELAY: libc::c_int;
    #[no_mangle]
    static mut LINES: libc::c_int;
    #[no_mangle]
    static mut TABSIZE: libc::c_int;
    #[no_mangle]
    fn fnmatch(__pattern: *const libc::c_char, __name: *const libc::c_char,
               __flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
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
    fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    static mut cur_term: *mut TERMINAL;
    #[no_mangle]
    static boolnames: [*const libc::c_char; 0];
    #[no_mangle]
    static boolcodes: [*const libc::c_char; 0];
    #[no_mangle]
    static boolfnames: [*const libc::c_char; 0];
    #[no_mangle]
    static numnames: [*const libc::c_char; 0];
    #[no_mangle]
    static numcodes: [*const libc::c_char; 0];
    #[no_mangle]
    static numfnames: [*const libc::c_char; 0];
    #[no_mangle]
    static strnames: [*const libc::c_char; 0];
    #[no_mangle]
    static strcodes: [*const libc::c_char; 0];
    #[no_mangle]
    static strfnames: [*const libc::c_char; 0];
    #[no_mangle]
    fn del_curterm(_: *mut TERMINAL) -> libc::c_int;
    #[no_mangle]
    fn setupterm(_: *const libc::c_char, _: libc::c_int, _: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    static in6addr_any: in6_addr;
    #[no_mangle]
    static in6addr_loopback: in6_addr;
    #[no_mangle]
    fn strnvis(_: *mut libc::c_char, _: *const libc::c_char, _: size_t,
               _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strunvis(_: *mut libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn options_get_only(_: *mut options, _: *const libc::c_char)
     -> *mut options_entry;
    #[no_mangle]
    fn options_array_get(_: *mut options_entry, _: u_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn options_array_size(_: *mut options_entry, _: *mut u_int)
     -> libc::c_int;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
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
pub const TTYC_KF4: tty_code_code = 101;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub const TTYC_CVVIS: tty_code_code = 22;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const TTYC_KUP4: tty_code_code = 176;
pub const TTYC_KF35: tty_code_code = 96;
pub const TTYC_KPRV7: tty_code_code = 166;
pub const TTYC_SMACS: tty_code_code = 195;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const TTYC_SMKX: tty_code_code = 197;
pub const TTYC_SMSO: tty_code_code = 198;
pub const TTYC_CIVIS: tty_code_code = 6;
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
    pub entry: unnamed_10,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    ev_next_with_common_timeout: unnamed_30,
    min_heap_idx: libc::c_int,
}
pub type uint8_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_CUB: tty_code_code = 13;
pub const TTYC_KF55: tty_code_code = 118;
pub const TTYC_SGR0: tty_code_code = 193;
pub const LINE_SEL_RIGHT_LEFT: unnamed_32 = 2;
pub const TTYC_KF58: tty_code_code = 121;
pub const TTYC_KPP: tty_code_code = 160;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub const TTYC_KF12: tty_code_code = 71;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_3 {
    ev_io: unnamed_17,
    ev_signal: unnamed_23,
}
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const TTYC_ICH1: tty_code_code = 38;
pub const TTYC_SETAB: tty_code_code = 189;
pub const TTYC_TC: tty_code_code = 202;
pub const TTYC_KHOM5: tty_code_code = 134;
pub const TTYC_KHOME: tty_code_code = 137;
pub const TTYC_KRIT2: tty_code_code = 168;
pub const TTYC_XT: tty_code_code = 207;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_4,
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
    pub entry: unnamed_12,
}
pub type __off_t = libc::c_long;
pub const TTYC_KF59: tty_code_code = 122;
pub const TTYC_KHOM4: tty_code_code = 133;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const TTYC_XENL: tty_code_code = 206;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const TTYC_DL: tty_code_code = 26;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_33,
}
pub const TTYC_KF44: tty_code_code = 106;
pub type unnamed_4 = libc::c_uint;
pub const TTYCODE_STRING: tty_code_type = 1;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const TTYCODE_FLAG: tty_code_type = 3;
pub const TTYC_KRI: tty_code_code = 167;
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
pub const TTYC_KF63: tty_code_code = 127;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type WINDOW = _win_st;
pub type options_table_scope = libc::c_uint;
pub const TTYC_KNXT5: tty_code_code = 157;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_24,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const TTYC_KUP7: tty_code_code = 179;
pub const TTYC_KCUU1: tty_code_code = 47;
pub const TTYC_KF41: tty_code_code = 103;
pub const TTYC_KHOM7: tty_code_code = 136;
pub const TTYC_KF18: tty_code_code = 77;
pub const TTYC_KDN7: tty_code_code = 60;
pub const TTYC_KF7: tty_code_code = 128;
pub const TTYC_KF38: tty_code_code = 99;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const TTYC_KF20: tty_code_code = 80;
pub const TTYC_KF5: tty_code_code = 112;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const TTYC_KHOM3: tty_code_code = 132;
pub type TERMTYPE = termtype;
pub const TTYC_DCH1: tty_code_code = 24;
pub type options_table_type = libc::c_uint;
pub type __off64_t = libc::c_long;
pub const TTYC_KF9: tty_code_code = 130;
pub const TTYC_TSL: tty_code_code = 203;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const TTYC_KF40: tty_code_code = 102;
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub type u_short = __u_short;
pub const TTYC_KF14: tty_code_code = 73;
pub const TTYC_CUP: tty_code_code = 19;
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
pub const TTYC_BLINK: tty_code_code = 4;
pub const TTYC_KRIT5: tty_code_code = 171;
pub const TTYC_EL: tty_code_code = 31;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
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
pub const TTYC_KDN6: tty_code_code = 59;
pub const TTYC_KF56: tty_code_code = 119;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_34,
}
pub const TTY_UNKNOWN: unnamed_19 = 6;
pub const TTYC_KCUF1: tty_code_code = 46;
pub const TTYC_KDC7: tty_code_code = 53;
pub const TTYC_KF28: tty_code_code = 88;
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
    pub message_log: unnamed_8,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_28,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_25,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const TTYC_CSR: tty_code_code = 12;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const TTYC_ECH: tty_code_code = 29;
pub const TTYC_SETRGBB: tty_code_code = 191;
pub const TTYC_ED: tty_code_code = 30;
pub const TTYC_KNP: tty_code_code = 153;
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
pub const TTY_VT220: unnamed_19 = 3;
pub const TTYC_KF51: tty_code_code = 114;
pub type uint16_t = libc::c_ushort;
pub const TTYC_KF57: tty_code_code = 120;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const TTYC_DIM: tty_code_code = 25;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const TTYC_INVIS: tty_code_code = 42;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _win_st {
    pub _cury: libc::c_short,
    pub _curx: libc::c_short,
    pub _maxy: libc::c_short,
    pub _maxx: libc::c_short,
    pub _begy: libc::c_short,
    pub _begx: libc::c_short,
    pub _flags: libc::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: libc::c_int,
    pub _line: *mut ldat,
    pub _regtop: libc::c_short,
    pub _regbottom: libc::c_short,
    pub _parx: libc::c_int,
    pub _pary: libc::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: libc::c_short,
    pub _bkgrnd: cchar_t,
}
pub const TTYC_KLFT7: tty_code_code = 151;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const TTYC_OP: tty_code_code = 181;
pub const TTYC_SMCUP: tty_code_code = 196;
pub const TTYC_KF47: tty_code_code = 109;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const TTYC_KDN5: tty_code_code = 58;
pub const TTYC_AX: tty_code_code = 0;
pub const TTYC_KF53: tty_code_code = 116;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type cc_t = libc::c_uchar;
pub const TTYC_RI: tty_code_code = 184;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const TTYC_KIC7: tty_code_code = 143;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const TTYC_KF11: tty_code_code = 70;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_code {
    pub type_0: tty_code_type,
    pub value: unnamed_35,
}
pub const TTYC_KF54: tty_code_code = 117;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const TTYC_KF6: tty_code_code = 123;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const TTY_VT320: unnamed_19 = 4;
pub const TTYC_CUF: tty_code_code = 17;
pub const TTYC_KEND2: tty_code_code = 62;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_21,
    pub entry: unnamed_39,
}
pub type chtype = libc::c_ulong;
pub const TTYC_ICH: tty_code_code = 37;
pub const TTYC_KF16: tty_code_code = 75;
pub const TTYC_KCUD1: tty_code_code = 45;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const TTYC_KEND7: tty_code_code = 67;
pub const TTYC_HOME: tty_code_code = 35;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_KNXT6: tty_code_code = 158;
pub const TTYC_KF29: tty_code_code = 89;
pub const TTYC_KEND5: tty_code_code = 65;
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
pub const TTYC_KF34: tty_code_code = 95;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub const TTYC_CUF1: tty_code_code = 18;
pub type uint32_t = libc::c_uint;
pub const TTYC_KF61: tty_code_code = 125;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub const TTYC_KF21: tty_code_code = 81;
pub type tty_code_type = libc::c_uint;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const TTYC_KDC4: tty_code_code = 50;
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
    pub entry: unnamed_6,
    pub tree_entry: unnamed_5,
}
pub const TTYC_KLFT3: tty_code_code = 147;
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
    pub gentry: unnamed_7,
    pub entry: unnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const TTYC_IL: tty_code_code = 39;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
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
    pub entry: unnamed_36,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub const TTYC_BEL: tty_code_code = 3;
pub const TTYC_MS: tty_code_code = 180;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const TTYC_KDC3: tty_code_code = 49;
pub const TTYC_KF45: tty_code_code = 107;
pub const TTYC_RGB: tty_code_code = 183;
pub const TTYC_KF2: tty_code_code = 79;
pub const TTYC_KEND: tty_code_code = 61;
pub const TTYC_SETAF: tty_code_code = 190;
pub const TTYC_KF37: tty_code_code = 98;
pub const TTYC_SMUL: tty_code_code = 199;
pub const TTYC_KF22: tty_code_code = 82;
pub const TTYC_KMOUS: tty_code_code = 152;
pub type TERMINAL = term;
pub const TTYC_KUP5: tty_code_code = 177;
pub const TTY_VT102: unnamed_19 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_2,
    pub ev_next: unnamed_22,
    pub ev_timeout_pos: unnamed_1,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_3,
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
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cchar_t {
    pub attr: attr_t,
    pub chars: [wchar_t; 5],
}
pub const TTY_VT101: unnamed_19 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term_code_entry {
    pub type_0: tty_code_type,
    pub name: *const libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub ev_io_next: unnamed_14,
    pub ev_timeout: timeval,
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
    pub alerts_entry: unnamed_38,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_9,
    pub entry: unnamed_26,
}
pub const TTYC_KF49: tty_code_code = 111;
pub const TTYCODE_NUMBER: tty_code_type = 2;
pub const TTYC_KDN4: tty_code_code = 57;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
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
pub const TTYC_KF13: tty_code_code = 72;
pub const TTYC_CUD1: tty_code_code = 16;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_18 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub type tty_code_code = libc::c_uint;
pub const TTYC_SETRGBF: tty_code_code = 192;
pub const TTYC_KF52: tty_code_code = 115;
pub const TTYC_KPRV3: tty_code_code = 162;
pub type __u_int = libc::c_uint;
pub const TTYC_SMXX: tty_code_code = 200;
pub const TTYC_KF31: tty_code_code = 92;
pub const TTYC_CNORM: tty_code_code = 8;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const TTYC_IL1: tty_code_code = 40;
pub type key_code = libc::c_ulonglong;
pub const TTYC_KF62: tty_code_code = 126;
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
    pub entry: unnamed_11,
    pub wentry: unnamed_20,
    pub sentry: unnamed_15,
}
pub type u_char = __u_char;
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
pub const TTYC_KF25: tty_code_code = 85;
pub type unnamed_19 = libc::c_uint;
pub const TTYC_KF33: tty_code_code = 94;
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
pub const TTYC_KF27: tty_code_code = 87;
pub const TTYC_DL1: tty_code_code = 27;
pub const TTYC_KNXT2: tty_code_code = 154;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub type time_t = __time_t;
pub const TTYC_KRIT4: tty_code_code = 170;
pub const TTYC_EL1: tty_code_code = 32;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub type __u_char = libc::c_uchar;
pub const TTYC_E3: tty_code_code = 28;
pub const JOB_CLOSED: unnamed_4 = 2;
pub const TTYC_KNXT7: tty_code_code = 159;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type pid_t = __pid_t;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const TTYC_CUD: tty_code_code = 15;
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_32,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
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
pub const TTYC_U8: tty_code_code = 204;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
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
pub const TTYC_KF46: tty_code_code = 108;
pub type speed_t = libc::c_uint;
pub const TTYC_KLFT6: tty_code_code = 150;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_KRIT3: tty_code_code = 169;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub ev_signal_next: unnamed_13,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const TTYC_KF17: tty_code_code = 76;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const TTYC_HPA: tty_code_code = 36;
pub const TTYC_KLFT2: tty_code_code = 146;
pub const TTYC_KDCH1: tty_code_code = 54;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const TTYC_KDC2: tty_code_code = 48;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const TTYC_KF43: tty_code_code = 105;
pub const TTYC_RMCUP: tty_code_code = 186;
pub type tcflag_t = libc::c_uint;
pub const TTYC_KUP6: tty_code_code = 178;
pub const TTYC_KDN2: tty_code_code = 55;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub type _IO_lock_t = ();
pub type __suseconds_t = libc::c_long;
pub const TTYC_KIC2: tty_code_code = 138;
pub const TTYC_KPRV6: tty_code_code = 165;
pub const TTYC_KF23: tty_code_code = 83;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_16,
}
pub const TTYC_KF42: tty_code_code = 104;
pub const TTYC_KF15: tty_code_code = 74;
pub const TTYC_KICH1: tty_code_code = 144;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_31,
}
pub const TTYCODE_NONE: tty_code_type = 0;
pub const TTYC_KEND6: tty_code_code = 66;
pub const TTYC_FSL: tty_code_code = 34;
pub const TTYC_KPRV2: tty_code_code = 161;
pub const TTYC_ENACS: tty_code_code = 33;
pub const TTYC_KNXT3: tty_code_code = 155;
pub const TTYC_BOLD: tty_code_code = 5;
pub const LINE_SEL_NONE: unnamed_32 = 0;
pub const TTY_VT420: unnamed_19 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const TTYC_COLORS: tty_code_code = 9;
pub const TTY_VT100: unnamed_19 = 0;
pub type __u_short = libc::c_ushort;
pub type wchar_t = libc::c_int;
pub const TTYC_CS: tty_code_code = 11;
pub const TTYC_KF36: tty_code_code = 97;
pub const TTYC_KF39: tty_code_code = 100;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub type unnamed_28 = libc::c_uint;
pub const TTYC_KUP3: tty_code_code = 175;
pub const TTYC_KRIT6: tty_code_code = 172;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
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
pub struct unnamed_29 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const TTYC_ACSC: tty_code_code = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_KF50: tty_code_code = 113;
pub const TTYC_SS: tty_code_code = 201;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const TTYC_KPRV5: tty_code_code = 164;
pub const TTYC_CR: tty_code_code = 10;
pub const TTYC_KCUB1: tty_code_code = 44;
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
    pub term_type: unnamed_19,
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
pub struct termtype {
    pub term_names: *mut libc::c_char,
    pub str_table: *mut libc::c_char,
    pub Booleans: *mut libc::c_char,
    pub Numbers: *mut libc::c_short,
    pub Strings: *mut *mut libc::c_char,
    pub ext_str_table: *mut libc::c_char,
    pub ext_Names: *mut *mut libc::c_char,
    pub num_Booleans: libc::c_ushort,
    pub num_Numbers: libc::c_ushort,
    pub num_Strings: libc::c_ushort,
    pub ext_Booleans: libc::c_ushort,
    pub ext_Numbers: libc::c_ushort,
    pub ext_Strings: libc::c_ushort,
}
pub type cmd_find_type = libc::c_uint;
pub const TTYC_KIC5: tty_code_code = 141;
pub const TTYC_KIC3: tty_code_code = 139;
pub const TTYC_KHOM2: tty_code_code = 131;
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
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub type u_int = __u_int;
pub const TTYC_KF26: tty_code_code = 86;
pub type unnamed_32 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_37,
}
pub const TTYC_KIC4: tty_code_code = 140;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const TTYC_KF1: tty_code_code = 68;
pub const TTYC_CUU: tty_code_code = 20;
pub const TTYC_KF19: tty_code_code = 78;
pub const TTYC_CLEAR: tty_code_code = 7;
pub type attr_t = chtype;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub type cmdq_type = libc::c_uint;
pub type cmd_retval = libc::c_int;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const PROMPT_ENTRY: unnamed_28 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct pdat {
    pub _pad_y: libc::c_short,
    pub _pad_x: libc::c_short,
    pub _pad_top: libc::c_short,
    pub _pad_left: libc::c_short,
    pub _pad_bottom: libc::c_short,
    pub _pad_right: libc::c_short,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const TTYC_RMKX: tty_code_code = 187;
pub const TTYC_RMACS: tty_code_code = 185;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const TTYC_DCH: tty_code_code = 23;
pub const TTYC_VPA: tty_code_code = 205;
pub const TTYC_SE: tty_code_code = 188;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const TTYC_KF3: tty_code_code = 90;
pub const TTYC_KF10: tty_code_code = 69;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct term {
    pub type_0: TERMTYPE,
    pub Filedes: libc::c_short,
    pub Ottyb: termios,
    pub Nttyb: termios,
    pub _baudrate: libc::c_int,
    pub _termname: *mut libc::c_char,
}
pub const TTYC_KHOM6: tty_code_code = 135;
pub const TTYC_KF8: tty_code_code = 129;
pub const TTYC_KNXT4: tty_code_code = 156;
pub const TTYC_KEND3: tty_code_code = 63;
pub const TTYC_KF48: tty_code_code = 110;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_35 {
    string: *mut libc::c_char,
    number: libc::c_int,
    flag: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const TTYC_KIND: tty_code_code = 145;
pub const TTYC_KF32: tty_code_code = 93;
pub const PROMPT_COMMAND: unnamed_28 = 1;
pub const TTYC_KEND4: tty_code_code = 64;
pub const TTYC_CUB1: tty_code_code = 14;
pub const TTYC_KPRV4: tty_code_code = 163;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub const TTYC_REV: tty_code_code = 182;
pub const TTYC_KLFT4: tty_code_code = 148;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type layout_type = libc::c_uint;
pub const TTYC_INDN: tty_code_code = 41;
pub const TTYC_KCBT: tty_code_code = 43;
pub const TTYC_KRIT7: tty_code_code = 173;
pub const TTYC_KDC6: tty_code_code = 52;
pub const TTYC_KIC6: tty_code_code = 142;
pub const TTYC_KUP2: tty_code_code = 174;
pub const LINE_SEL_LEFT_RIGHT: unnamed_32 = 1;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const TTYC_KLFT5: tty_code_code = 149;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_18,
}
pub const TTYC_KF30: tty_code_code = 91;
pub const TTYC_SITM: tty_code_code = 194;
pub const TTYC_KDC5: tty_code_code = 51;
pub const CMDQ_COMMAND: cmdq_type = 0;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_37 {
    offset: u_int,
    data: unnamed_29,
}
pub const TTYC_BCE: tty_code_code = 2;
pub const JOB_DEAD: unnamed_4 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub const TTYC_CUU1: tty_code_code = 21;
pub const TTYC_KF24: tty_code_code = 84;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub const TTYC_KDN3: tty_code_code = 56;
pub const TTYC_KF60: tty_code_code = 124;
pub const JOB_RUNNING: unnamed_4 = 0;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[no_mangle]
pub static mut tty_terms: tty_terms =
    unsafe { tty_terms{lh_first: 0 as *const tty_term as *mut tty_term,} };
#[no_mangle]
pub unsafe extern "C" fn tty_term_ncodes() -> u_int {
    return (::std::mem::size_of::<[tty_term_code_entry; 208]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<tty_term_code_entry>()
                                                as libc::c_ulong) as u_int;
}
static mut tty_term_codes: [tty_term_code_entry; 208] =
    unsafe {
        [tty_term_code_entry{type_0: TTYCODE_FLAG,
                             name:
                                 b"AX\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"acsc\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_FLAG,
                             name:
                                 b"bce\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"bel\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"blink\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"bold\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"civis\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"clear\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"cnorm\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_NUMBER,
                             name:
                                 b"colors\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"Cr\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"Cs\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"csr\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"cub\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"cub1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"cud\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"cud1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"cuf\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"cuf1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"cup\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"cuu\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"cuu1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"cvvis\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"dch\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"dch1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"dim\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"dl\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"dl1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"E3\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"ech\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"ed\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"el\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"el1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"enacs\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"fsl\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"home\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"hpa\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"ich\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"ich1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"il\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"il1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"indn\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"invis\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kcbt\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kcub1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kcud1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kcuf1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kcuu1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kDC\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kDC3\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kDC4\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kDC5\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kDC6\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kDC7\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kdch1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kDN\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kDN3\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kDN4\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kDN5\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kDN6\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kDN7\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kend\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kEND\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kEND3\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kEND4\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kEND5\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kEND6\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kEND7\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf10\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf11\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf12\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf13\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf14\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf15\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf16\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf17\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf18\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf19\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf2\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf20\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf21\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf22\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf23\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf24\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf25\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf26\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf27\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf28\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf29\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf3\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf30\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf31\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf32\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf33\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf34\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf35\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf36\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf37\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf38\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf39\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf4\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf40\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf41\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf42\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf43\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf44\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf45\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf46\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf47\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf48\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf49\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf5\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf50\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf51\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf52\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf53\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf54\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf55\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf56\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf57\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf58\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf59\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf6\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf60\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf61\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf62\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf63\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf7\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf8\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kf9\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kHOM\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kHOM3\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kHOM4\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kHOM5\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kHOM6\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kHOM7\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"khome\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kIC\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kIC3\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kIC4\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kIC5\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kIC6\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kIC7\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kich1\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kind\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kLFT\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kLFT3\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kLFT4\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kLFT5\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kLFT6\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kLFT7\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kmous\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"knp\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kNXT\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kNXT3\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kNXT4\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kNXT5\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kNXT6\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kNXT7\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kpp\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kPRV\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kPRV3\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kPRV4\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kPRV5\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kPRV6\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kPRV7\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kri\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kRIT\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kRIT3\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kRIT4\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kRIT5\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kRIT6\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kRIT7\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kUP\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kUP3\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kUP4\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kUP5\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kUP6\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"kUP7\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"Ms\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"op\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"rev\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_FLAG,
                             name:
                                 b"RGB\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"ri\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"rmacs\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"rmcup\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"rmkx\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"Se\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"setab\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"setaf\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"setrgbb\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"setrgbf\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"sgr0\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"sitm\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"smacs\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"smcup\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"smkx\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"smso\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"smul\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"smxx\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"Ss\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_FLAG,
                             name:
                                 b"Tc\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"tsl\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_NUMBER,
                             name:
                                 b"U8\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_STRING,
                             name:
                                 b"vpa\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_FLAG,
                             name:
                                 b"xenl\x00" as *const u8 as
                                     *const libc::c_char,},
         tty_term_code_entry{type_0: TTYCODE_FLAG,
                             name:
                                 b"XT\x00" as *const u8 as
                                     *const libc::c_char,}]
    };
#[no_mangle]
pub unsafe extern "C" fn tty_term_find(mut name: *mut libc::c_char,
                                       mut fd: libc::c_int,
                                       mut cause: *mut *mut libc::c_char)
 -> *mut tty_term {
    let mut term: *mut tty_term = 0 as *mut tty_term;
    let mut ent: *const tty_term_code_entry = 0 as *const tty_term_code_entry;
    let mut code: *mut tty_code = 0 as *mut tty_code;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut size: u_int = 0;
    let mut i: u_int = 0;
    let mut n: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut acs: *const libc::c_char = 0 as *const libc::c_char;
    term = (*(&mut tty_terms as *mut tty_terms)).lh_first;
    loop  {
        if term != 0 as *mut libc::c_void as *mut tty_term {
            if strcmp((*term).name, name) == 0i32 {
                (*term).references = (*term).references.wrapping_add(1);
                return term
            } else { term = (*term).entry.le_next }
        } else {
            log_debug(b"new term: %s\x00" as *const u8 as *const libc::c_char,
                      name);
            term =
                xmalloc(::std::mem::size_of::<tty_term>() as libc::c_ulong) as
                    *mut tty_term;
            (*term).name = xstrdup(name);
            (*term).references = 1i32 as u_int;
            (*term).flags = 0i32;
            (*term).codes =
                xcalloc(tty_term_ncodes() as size_t,
                        ::std::mem::size_of::<tty_code>() as libc::c_ulong) as
                    *mut tty_code;
            break ;
        }
    }
    loop  {
        (*term).entry.le_next =
            (*(&mut tty_terms as *mut tty_terms)).lh_first;
        if (*term).entry.le_next != 0 as *mut libc::c_void as *mut tty_term {
            let ref mut fresh0 =
                (*(*(&mut tty_terms as
                         *mut tty_terms)).lh_first).entry.le_prev;
            *fresh0 = &mut (*term).entry.le_next as *mut *mut tty_term
        }
        let ref mut fresh1 = (*(&mut tty_terms as *mut tty_terms)).lh_first;
        *fresh1 = term;
        (*term).entry.le_prev =
            &mut (*(&mut tty_terms as *mut tty_terms)).lh_first as
                *mut *mut tty_term;
        if !(0 != 0i32) { break ; }
    }
    if setupterm(name, fd, &mut error as *mut libc::c_int) != 0i32 {
        match error {
            1 => {
                xasprintf(cause,
                          b"can\'t use hardcopy terminal: %s\x00" as *const u8
                              as *const libc::c_char, name);
            }
            0 => {
                xasprintf(cause,
                          b"missing or unsuitable terminal: %s\x00" as
                              *const u8 as *const libc::c_char, name);
            }
            -1 => {
                xasprintf(cause,
                          b"can\'t find terminfo database\x00" as *const u8 as
                              *const libc::c_char);
            }
            _ => {
                xasprintf(cause,
                          b"unknown error\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
    } else {
        i = 0i32 as u_int;
        while i < tty_term_ncodes() {
            ent = &tty_term_codes[i as usize] as *const tty_term_code_entry;
            code = &mut *(*term).codes.offset(i as isize) as *mut tty_code;
            (*code).type_0 = TTYCODE_NONE;
            match (*ent).type_0 as libc::c_uint {
                1 => {
                    s = tigetstr((*ent).name as *mut libc::c_char);
                    if !(s == 0 as *mut libc::c_void as *const libc::c_char ||
                             s == 1i32.wrapping_neg() as *mut libc::c_char) {
                        (*code).type_0 = TTYCODE_STRING;
                        (*code).value.string = tty_term_strip(s)
                    }
                }
                2 => {
                    n = tigetnum((*ent).name as *mut libc::c_char);
                    if !(n == 1i32.wrapping_neg() || n == 2i32.wrapping_neg())
                       {
                        (*code).type_0 = TTYCODE_NUMBER;
                        (*code).value.number = n
                    }
                }
                3 => {
                    n = tigetflag((*ent).name as *mut libc::c_char);
                    if !(n == 1i32.wrapping_neg()) {
                        (*code).type_0 = TTYCODE_FLAG;
                        (*code).value.flag = n
                    }
                }
                0 | _ => { }
            }
            i = i.wrapping_add(1)
        }
        o =
            options_get_only(global_options,
                             b"terminal-overrides\x00" as *const u8 as
                                 *const libc::c_char);
        if options_array_size(o, &mut size as *mut u_int) !=
               1i32.wrapping_neg() {
            i = 0i32 as u_int;
            while i < size {
                s = options_array_get(o, i);
                if s != 0 as *mut libc::c_void as *const libc::c_char {
                    tty_term_override(term, s);
                }
                i = i.wrapping_add(1)
            }
        }
        del_curterm(cur_term);
        if 0 == tty_term_has(term, TTYC_CLEAR) {
            xasprintf(cause,
                      b"terminal does not support clear\x00" as *const u8 as
                          *const libc::c_char);
        } else if 0 == tty_term_has(term, TTYC_CUP) {
            xasprintf(cause,
                      b"terminal does not support cup\x00" as *const u8 as
                          *const libc::c_char);
        } else if 0 == tty_term_has(term, TTYC_CUD1) &&
                      0 == tty_term_has(term, TTYC_CUD) {
            xasprintf(cause,
                      b"terminal does not support cud1 or cud\x00" as
                          *const u8 as *const libc::c_char);
        } else {
            if tty_term_number(term, TTYC_COLORS) == 256i32 {
                (*term).flags |= 1i32
            }
            if 0 == tty_term_flag(term, TTYC_XENL) { (*term).flags |= 2i32 }
            memset((*term).acs.as_mut_ptr() as *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<[[libc::c_char; 2]; 256]>() as
                       libc::c_ulong);
            if 0 != tty_term_has(term, TTYC_ACSC) {
                acs = tty_term_string(term, TTYC_ACSC)
            } else {
                acs =
                    b"a#j+k+l+m+n+o-p-q-r-s-t+u+v+w+x|y<z>~.\x00" as *const u8
                        as *const libc::c_char
            }
            while *acs.offset(0isize) as libc::c_int != 0 &&
                      *acs.offset(1isize) as libc::c_int != 0 {
                (*term).acs[*acs.offset(0isize) as u_char as usize][0usize] =
                    *acs.offset(1isize);
                acs = acs.offset(2isize)
            }
            if 0 != tty_term_flag(term, TTYC_XT) &&
                   0 == tty_term_has(term, TTYC_TSL) &&
                   0 == tty_term_has(term, TTYC_FSL) {
                code =
                    &mut *(*term).codes.offset(TTYC_TSL as libc::c_int as
                                                   isize) as *mut tty_code;
                (*code).value.string =
                    xstrdup(b"\x1b]0;\x00" as *const u8 as
                                *const libc::c_char);
                (*code).type_0 = TTYCODE_STRING;
                code =
                    &mut *(*term).codes.offset(TTYC_FSL as libc::c_int as
                                                   isize) as *mut tty_code;
                (*code).value.string =
                    xstrdup(b"\x07\x00" as *const u8 as *const libc::c_char);
                (*code).type_0 = TTYCODE_STRING
            }
            if (0 != tty_term_flag(term, TTYC_TC) ||
                    0 != tty_term_flag(term, TTYC_RGB)) &&
                   0 == tty_term_has(term, TTYC_SETRGBF) &&
                   0 == tty_term_has(term, TTYC_SETRGBB) {
                code =
                    &mut *(*term).codes.offset(TTYC_SETRGBF as libc::c_int as
                                                   isize) as *mut tty_code;
                (*code).value.string =
                    xstrdup(b"\x1b[38;2;%p1%d;%p2%d;%p3%dm\x00" as *const u8
                                as *const libc::c_char);
                (*code).type_0 = TTYCODE_STRING;
                code =
                    &mut *(*term).codes.offset(TTYC_SETRGBB as libc::c_int as
                                                   isize) as *mut tty_code;
                (*code).value.string =
                    xstrdup(b"\x1b[48;2;%p1%d;%p2%d;%p3%dm\x00" as *const u8
                                as *const libc::c_char);
                (*code).type_0 = TTYCODE_STRING
            }
            i = 0i32 as u_int;
            while i < tty_term_ncodes() {
                log_debug(b"%s%s\x00" as *const u8 as *const libc::c_char,
                          name, tty_term_describe(term, i as tty_code_code));
                i = i.wrapping_add(1)
            }
            return term
        }
    }
    tty_term_free(term);
    return 0 as *mut tty_term;
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_free(mut term: *mut tty_term) -> () {
    let mut i: u_int = 0;
    (*term).references = (*term).references.wrapping_sub(1);
    if (*term).references != 0i32 as libc::c_uint {
        return
    } else {
        loop  {
            if (*term).entry.le_next !=
                   0 as *mut libc::c_void as *mut tty_term {
                (*(*term).entry.le_next).entry.le_prev = (*term).entry.le_prev
            }
            *(*term).entry.le_prev = (*term).entry.le_next;
            if !(0 != 0i32) { break ; }
        }
        i = 0i32 as u_int;
        while i < tty_term_ncodes() {
            if (*(*term).codes.offset(i as isize)).type_0 as libc::c_uint ==
                   TTYCODE_STRING as libc::c_int as libc::c_uint {
                free((*(*term).codes.offset(i as isize)).value.string as
                         *mut libc::c_void);
            }
            i = i.wrapping_add(1)
        }
        free((*term).codes as *mut libc::c_void);
        free((*term).name as *mut libc::c_void);
        free(term as *mut libc::c_void);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_describe(mut term: *mut tty_term,
                                           mut code: tty_code_code)
 -> *const libc::c_char {
    static mut s: [libc::c_char; 256] = unsafe { [0; 256] };
    let mut out: [libc::c_char; 128] = [0; 128];
    match (*(*term).codes.offset(code as isize)).type_0 as libc::c_uint {
        0 => {
            xsnprintf(s.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 256]>() as
                          libc::c_ulong,
                      b"%4u: %s: [missing]\x00" as *const u8 as
                          *const libc::c_char, code as libc::c_uint,
                      tty_term_codes[code as usize].name);
        }
        1 => {
            strnvis(out.as_mut_ptr(),
                    (*(*term).codes.offset(code as isize)).value.string,
                    ::std::mem::size_of::<[libc::c_char; 128]>() as
                        libc::c_ulong, 1i32 | 8i32 | 16i32);
            xsnprintf(s.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 256]>() as
                          libc::c_ulong,
                      b"%4u: %s: (string) %s\x00" as *const u8 as
                          *const libc::c_char, code as libc::c_uint,
                      tty_term_codes[code as usize].name, out.as_mut_ptr());
        }
        2 => {
            xsnprintf(s.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 256]>() as
                          libc::c_ulong,
                      b"%4u: %s: (number) %d\x00" as *const u8 as
                          *const libc::c_char, code as libc::c_uint,
                      tty_term_codes[code as usize].name,
                      (*(*term).codes.offset(code as isize)).value.number);
        }
        3 => {
            xsnprintf(s.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 256]>() as
                          libc::c_ulong,
                      b"%4u: %s: (flag) %s\x00" as *const u8 as
                          *const libc::c_char, code as libc::c_uint,
                      tty_term_codes[code as usize].name,
                      if 0 !=
                             (*(*term).codes.offset(code as isize)).value.flag
                         {
                          b"true\x00" as *const u8 as *const libc::c_char
                      } else {
                          b"false\x00" as *const u8 as *const libc::c_char
                      });
        }
        _ => { }
    }
    return s.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_has(mut term: *mut tty_term,
                                      mut code: tty_code_code)
 -> libc::c_int {
    return ((*(*term).codes.offset(code as isize)).type_0 as libc::c_uint !=
                TTYCODE_NONE as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_flag(mut term: *mut tty_term,
                                       mut code: tty_code_code)
 -> libc::c_int {
    if 0 == tty_term_has(term, code) {
        return 0i32
    } else if (*(*term).codes.offset(code as isize)).type_0 as libc::c_uint !=
                  TTYCODE_FLAG as libc::c_int as libc::c_uint {
        fatalx(b"not a flag: %d\x00" as *const u8 as *const libc::c_char,
               code as libc::c_uint);
    } else { return (*(*term).codes.offset(code as isize)).value.flag };
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_string(mut term: *mut tty_term,
                                         mut code: tty_code_code)
 -> *const libc::c_char {
    if 0 == tty_term_has(term, code) {
        return b"\x00" as *const u8 as *const libc::c_char
    } else if (*(*term).codes.offset(code as isize)).type_0 as libc::c_uint !=
                  TTYCODE_STRING as libc::c_int as libc::c_uint {
        fatalx(b"not a string: %d\x00" as *const u8 as *const libc::c_char,
               code as libc::c_uint);
    } else { return (*(*term).codes.offset(code as isize)).value.string };
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_number(mut term: *mut tty_term,
                                         mut code: tty_code_code)
 -> libc::c_int {
    if 0 == tty_term_has(term, code) {
        return 0i32
    } else if (*(*term).codes.offset(code as isize)).type_0 as libc::c_uint !=
                  TTYCODE_NUMBER as libc::c_int as libc::c_uint {
        fatalx(b"not a number: %d\x00" as *const u8 as *const libc::c_char,
               code as libc::c_uint);
    } else { return (*(*term).codes.offset(code as isize)).value.number };
}
unsafe extern "C" fn tty_term_override(mut term: *mut tty_term,
                                       mut override_0: *const libc::c_char)
 -> () {
    let mut current_block: u64;
    let mut ent: *const tty_term_code_entry = 0 as *const tty_term_code_entry;
    let mut code: *mut tty_code = 0 as *mut tty_code;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: u_int = 0;
    let mut n: libc::c_int = 0;
    let mut remove: libc::c_int = 0;
    next = xstrdup(override_0);
    copy = next;
    s =
        strsep(&mut next as *mut *mut libc::c_char,
               b":\x00" as *const u8 as *const libc::c_char);
    if s == 0 as *mut libc::c_void as *mut libc::c_char ||
           next == 0 as *mut libc::c_void as *mut libc::c_char ||
           fnmatch(s, (*term).name, 0i32) != 0i32 {
        free(copy as *mut libc::c_void);
        return
    } else {
        loop  {
            s =
                strsep(&mut next as *mut *mut libc::c_char,
                       b":\x00" as *const u8 as *const libc::c_char);
            if !(s != 0 as *mut libc::c_void as *mut libc::c_char) { break ; }
            if *s as libc::c_int == 0 { continue ; }
            value = 0 as *mut libc::c_char;
            remove = 0i32;
            cp = strchr(s, 61);
            if cp != 0 as *mut libc::c_void as *mut libc::c_char {
                let fresh2 = cp;
                cp = cp.offset(1);
                *fresh2 = 0 as libc::c_char;
                value = xstrdup(cp);
                if strunvis(value, cp) == 1i32.wrapping_neg() {
                    free(value as *mut libc::c_void);
                    value = xstrdup(cp)
                }
            } else if *s.offset(strlen(s).wrapping_sub(1i32 as libc::c_ulong)
                                    as isize) as libc::c_int == 64 {
                *s.offset(strlen(s).wrapping_sub(1i32 as libc::c_ulong) as
                              isize) = 0 as libc::c_char;
                remove = 1i32
            } else {
                value = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
            }
            if 0 != remove {
                log_debug(b"%s override: %s@\x00" as *const u8 as
                              *const libc::c_char, (*term).name, s);
            } else {
                log_debug(b"%s override: %s=%s\x00" as *const u8 as
                              *const libc::c_char, (*term).name, s, value);
            }
            i = 0i32 as u_int;
            while i < tty_term_ncodes() {
                ent =
                    &tty_term_codes[i as usize] as *const tty_term_code_entry;
                if !(strcmp(s, (*ent).name) != 0i32) {
                    code =
                        &mut *(*term).codes.offset(i as isize) as
                            *mut tty_code;
                    if 0 != remove {
                        (*code).type_0 = TTYCODE_NONE
                    } else {
                        match (*ent).type_0 as libc::c_uint {
                            1 => {
                                current_block = 2279516045080240537;
                                match current_block {
                                    4494227436833987707 => {
                                        (*code).value.flag = 1i32;
                                        (*code).type_0 = (*ent).type_0
                                    }
                                    2279516045080240537 => {
                                        if (*code).type_0 as libc::c_uint ==
                                               TTYCODE_STRING as libc::c_int
                                                   as libc::c_uint {
                                            free((*code).value.string as
                                                     *mut libc::c_void);
                                        }
                                        (*code).value.string = xstrdup(value);
                                        (*code).type_0 = (*ent).type_0
                                    }
                                    _ => {
                                        n =
                                            strtonum(value,
                                                     0i32 as libc::c_longlong,
                                                     2147483647i32 as
                                                         libc::c_longlong,
                                                     &mut errstr as
                                                         *mut *const libc::c_char)
                                                as libc::c_int;
                                        if !(errstr !=
                                                 0 as *mut libc::c_void as
                                                     *const libc::c_char) {
                                            (*code).value.number = n;
                                            (*code).type_0 = (*ent).type_0
                                        }
                                    }
                                }
                            }
                            2 => {
                                current_block = 7398230123754905173;
                                match current_block {
                                    4494227436833987707 => {
                                        (*code).value.flag = 1i32;
                                        (*code).type_0 = (*ent).type_0
                                    }
                                    2279516045080240537 => {
                                        if (*code).type_0 as libc::c_uint ==
                                               TTYCODE_STRING as libc::c_int
                                                   as libc::c_uint {
                                            free((*code).value.string as
                                                     *mut libc::c_void);
                                        }
                                        (*code).value.string = xstrdup(value);
                                        (*code).type_0 = (*ent).type_0
                                    }
                                    _ => {
                                        n =
                                            strtonum(value,
                                                     0i32 as libc::c_longlong,
                                                     2147483647i32 as
                                                         libc::c_longlong,
                                                     &mut errstr as
                                                         *mut *const libc::c_char)
                                                as libc::c_int;
                                        if !(errstr !=
                                                 0 as *mut libc::c_void as
                                                     *const libc::c_char) {
                                            (*code).value.number = n;
                                            (*code).type_0 = (*ent).type_0
                                        }
                                    }
                                }
                            }
                            3 => {
                                current_block = 4494227436833987707;
                                match current_block {
                                    4494227436833987707 => {
                                        (*code).value.flag = 1i32;
                                        (*code).type_0 = (*ent).type_0
                                    }
                                    2279516045080240537 => {
                                        if (*code).type_0 as libc::c_uint ==
                                               TTYCODE_STRING as libc::c_int
                                                   as libc::c_uint {
                                            free((*code).value.string as
                                                     *mut libc::c_void);
                                        }
                                        (*code).value.string = xstrdup(value);
                                        (*code).type_0 = (*ent).type_0
                                    }
                                    _ => {
                                        n =
                                            strtonum(value,
                                                     0i32 as libc::c_longlong,
                                                     2147483647i32 as
                                                         libc::c_longlong,
                                                     &mut errstr as
                                                         *mut *const libc::c_char)
                                                as libc::c_int;
                                        if !(errstr !=
                                                 0 as *mut libc::c_void as
                                                     *const libc::c_char) {
                                            (*code).value.number = n;
                                            (*code).type_0 = (*ent).type_0
                                        }
                                    }
                                }
                            }
                            0 | _ => { }
                        }
                    }
                }
                i = i.wrapping_add(1)
            }
            free(value as *mut libc::c_void);
        }
        free(s as *mut libc::c_void);
        return;
    };
}
unsafe extern "C" fn tty_term_strip(mut s: *const libc::c_char)
 -> *mut libc::c_char {
    let mut current_block: u64;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    static mut buf: [libc::c_char; 8192] = unsafe { [0; 8192] };
    let mut len: size_t = 0;
    if strchr(s, 36) == 0 as *mut libc::c_void as *mut libc::c_char {
        return xstrdup(s)
    } else {
        len = 0i32 as size_t;
        ptr = s;
        's_11:
            while *ptr as libc::c_int != 0 {
                if *ptr as libc::c_int == 36 &&
                       *ptr.offset(1isize) as libc::c_int == 60 {
                    current_block = 17778012151635330486;
                } else { current_block = 12675440807659640239; }
                loop  {
                    match current_block {
                        12675440807659640239 => {
                            let fresh3 = len;
                            len = len.wrapping_add(1);
                            buf[fresh3 as usize] = *ptr;
                            if len ==
                                   (::std::mem::size_of::<[libc::c_char; 8192]>()
                                        as
                                        libc::c_ulong).wrapping_sub(1i32 as
                                                                        libc::c_ulong)
                               {
                                break 's_11 ;
                            } else { break ; }
                        }
                        _ => {
                            if *ptr as libc::c_int != 0 &&
                                   *ptr as libc::c_int != 62 {
                                ptr = ptr.offset(1isize);
                                current_block = 17778012151635330486;
                            } else {
                                if !(*ptr as libc::c_int == 62) {
                                    current_block = 12675440807659640239;
                                    continue ;
                                }
                                ptr = ptr.offset(1isize);
                                current_block = 12675440807659640239;
                            }
                        }
                    }
                }
                ptr = ptr.offset(1isize)
            }
        buf[len as usize] = 0 as libc::c_char;
        return xstrdup(buf.as_mut_ptr())
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_string1(mut term: *mut tty_term,
                                          mut code: tty_code_code,
                                          mut a: libc::c_int)
 -> *const libc::c_char {
    return tparm(tty_term_string(term, code) as *mut libc::c_char, a, 0i32,
                 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_string2(mut term: *mut tty_term,
                                          mut code: tty_code_code,
                                          mut a: libc::c_int,
                                          mut b: libc::c_int)
 -> *const libc::c_char {
    return tparm(tty_term_string(term, code) as *mut libc::c_char, a, b, 0i32,
                 0i32, 0i32, 0i32, 0i32, 0i32, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_string3(mut term: *mut tty_term,
                                          mut code: tty_code_code,
                                          mut a: libc::c_int,
                                          mut b: libc::c_int,
                                          mut c: libc::c_int)
 -> *const libc::c_char {
    return tparm(tty_term_string(term, code) as *mut libc::c_char, a, b, c,
                 0i32, 0i32, 0i32, 0i32, 0i32, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_ptr1(mut term: *mut tty_term,
                                       mut code: tty_code_code,
                                       mut a: *const libc::c_void)
 -> *const libc::c_char {
    return tparm(tty_term_string(term, code) as *mut libc::c_char, a, 0i32,
                 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_ptr2(mut term: *mut tty_term,
                                       mut code: tty_code_code,
                                       mut a: *const libc::c_void,
                                       mut b: *const libc::c_void)
 -> *const libc::c_char {
    return tparm(tty_term_string(term, code) as *mut libc::c_char, a, b, 0i32,
                 0i32, 0i32, 0i32, 0i32, 0i32, 0i32);
}

