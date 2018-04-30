extern crate libc;
extern "C" {
    pub type tty_code;
    pub type format_job_tree;
    pub type options_entry;
    pub type screen_titles;
    pub type evbuffer;
    pub type environ;
    pub type input_ctx;
    pub type tmuxpeer;
    pub type _IO_FILE_plus;
    pub type format_tree;
    pub type hooks;
    pub type options;
    pub type bufferevent_ops;
    pub type tmuxproc;
    pub type event_base;
    pub type args_entry;
    #[no_mangle]
    fn strtoul(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
               __base: libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
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
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
    #[no_mangle]
    fn event_pending(ev: *const event, events: libc::c_short,
                     tv: *mut timeval) -> libc::c_int;
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
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t)
     -> *mut libc::c_uchar;
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
    fn options_get(_: *mut options, _: *const libc::c_char)
     -> *mut options_entry;
    #[no_mangle]
    fn options_array_get(_: *mut options_entry, _: u_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn options_array_size(_: *mut options_entry, _: *mut u_int)
     -> libc::c_int;
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn tty_set_type(_: *mut tty, _: libc::c_int) -> ();
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn tty_term_string(_: *mut tty_term, _: tty_code_code)
     -> *const libc::c_char;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn key_string_lookup_key(_: key_code) -> *const libc::c_char;
    #[no_mangle]
    fn server_client_handle_key(_: *mut client, _: key_code) -> ();
    #[no_mangle]
    fn utf8_combine(_: *const utf8_data, _: *mut wchar_t) -> utf8_state;
    #[no_mangle]
    fn utf8_append(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn utf8_open(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn xterm_keys_find(_: *const libc::c_char, _: size_t, _: *mut size_t,
                       _: *mut key_code) -> libc::c_int;
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
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const TTYC_KUP3: tty_code_code = 175;
pub const TTYC_KF34: tty_code_code = 95;
pub const KEYC_TRIPLECLICK1_BORDER: unnamed_30 = 268435518;
pub const KEYC_MOUSEDRAGEND3_BORDER: unnamed_30 = 268435500;
pub const KEYC_DOUBLECLICK3_STATUS: unnamed_30 = 268435514;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const TTYC_EL1: tty_code_code = 32;
pub const TTYC_KUP6: tty_code_code = 178;
pub type size_t = libc::c_ulong;
pub const TTYC_KF55: tty_code_code = 118;
pub const TTYC_KHOM7: tty_code_code = 136;
pub const KEYC_MOUSEUP2_BORDER: unnamed_30 = 268435479;
pub const TTYC_KF16: tty_code_code = 75;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    offset: u_int,
    data: unnamed_24,
}
pub const TTYC_KLFT5: tty_code_code = 149;
pub const TTYC_SMCUP: tty_code_code = 196;
pub const TTYC_KDN2: tty_code_code = 55;
pub const KEYC_F1: unnamed_30 = 268435526;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const TTYC_KUP5: tty_code_code = 177;
pub const KEYC_HOME: unnamed_30 = 268435540;
pub const TTYC_SMUL: tty_code_code = 199;
pub const KEYC_KP_ENTER: unnamed_30 = 268435562;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const JOB_RUNNING: unnamed_6 = 0;
pub const TTYC_KF12: tty_code_code = 71;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub const TTYC_KF46: tty_code_code = 108;
pub const TTYC_KICH1: tty_code_code = 144;
pub const KEYC_KP_SLASH: unnamed_30 = 268435549;
pub const TTYC_ICH: tty_code_code = 37;
pub const TTYC_KPP: tty_code_code = 160;
pub const KEYC_F12: unnamed_30 = 268435537;
pub const KEYC_FOCUS_IN: unnamed_30 = 268435456;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub const TTYC_KF37: tty_code_code = 98;
pub const TTYC_INVIS: tty_code_code = 42;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const KEYC_FOCUS_OUT: unnamed_30 = 268435457;
pub const TTYC_KCUF1: tty_code_code = 46;
pub const TTYC_KF29: tty_code_code = 89;
pub const KEYC_MOUSEDRAGEND1_BORDER: unnamed_30 = 268435494;
pub type cc_t = libc::c_uchar;
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
    pub entry: unnamed_0,
    pub wentry: unnamed_4,
    pub sentry: unnamed_23,
}
pub type ssize_t = __ssize_t;
pub const KEYC_MOUSEMOVE_STATUS: unnamed_30 = 268435463;
pub type __pid_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub type wchar_t = libc::c_int;
pub type tcflag_t = libc::c_uint;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const KEYC_KP_ZERO: unnamed_30 = 268435563;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const TTYC_BOLD: tty_code_code = 5;
pub const KEYC_WHEELDOWN_BORDER: unnamed_30 = 268435506;
pub const KEYC_MOUSEDRAG3_STATUS: unnamed_30 = 268435490;
pub const TTY_VT101: unnamed_35 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const TTYC_KF1: tty_code_code = 68;
pub const KEYC_MOUSEDRAG3_BORDER: unnamed_30 = 268435491;
pub type unnamed_6 = libc::c_uint;
pub const TTY_VT420: unnamed_35 = 5;
pub const TTYC_KNXT6: tty_code_code = 158;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_KDN7: tty_code_code = 60;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_KPRV2: tty_code_code = 161;
pub const TTYC_KEND2: tty_code_code = 62;
pub const TTYC_KHOME: tty_code_code = 137;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub const KEYC_KP_EIGHT: unnamed_30 = 268435553;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_6,
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
    pub entry: unnamed_16,
}
pub const TTYC_KHOM6: tty_code_code = 135;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_15,
}
pub type uint32_t = libc::c_uint;
pub const KEYC_MOUSEDRAG1_STATUS: unnamed_30 = 268435484;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub const UTF8_MORE: utf8_state = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
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
pub const TTYC_KEND: tty_code_code = 61;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
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
pub type cmd_retval = libc::c_int;
pub const KEYC_DC: unnamed_30 = 268435539;
pub const TTYC_KF45: tty_code_code = 107;
pub const TTYC_KEND4: tty_code_code = 64;
pub const TTYC_KF24: tty_code_code = 84;
pub const TTYC_KF63: tty_code_code = 127;
pub const TTYC_KF2: tty_code_code = 79;
pub const TTYC_DL1: tty_code_code = 27;
pub const KEYC_WHEELDOWN_STATUS: unnamed_30 = 268435505;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_20,
}
pub const TTYC_KNXT3: tty_code_code = 155;
pub const TTYC_RMKX: tty_code_code = 187;
pub const TTYC_KF44: tty_code_code = 106;
pub const TTYC_E3: tty_code_code = 28;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_17,
    pub entry: unnamed_34,
}
pub const TTYC_KF27: tty_code_code = 87;
pub const TTYC_KF19: tty_code_code = 78;
pub const TTYC_KRI: tty_code_code = 167;
pub const TTYC_KF51: tty_code_code = 114;
pub const KEYC_KP_NINE: unnamed_30 = 268435554;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub const TTYC_RMCUP: tty_code_code = 186;
pub const KEYC_KP_PLUS: unnamed_30 = 268435555;
pub const KEYC_MOUSEUP2_PANE: unnamed_30 = 268435477;
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
pub struct unnamed_9 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const KEYC_F2: unnamed_30 = 268435527;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const KEYC_TRIPLECLICK3_BORDER: unnamed_30 = 268435524;
pub const TTYC_CLEAR: tty_code_code = 7;
pub const KEYC_F3: unnamed_30 = 268435528;
pub const TTYC_KF59: tty_code_code = 122;
pub type u_short = __u_short;
pub const TTYC_ACSC: tty_code_code = 1;
pub const TTYC_DCH: tty_code_code = 23;
pub type layout_type = libc::c_uint;
pub const TTYC_TSL: tty_code_code = 203;
pub const TTYC_SS: tty_code_code = 201;
pub const TTYC_KF22: tty_code_code = 82;
pub const KEYC_BSPACE: unnamed_30 = 268435525;
pub const TTYC_EL: tty_code_code = 31;
pub const TTYC_KF28: tty_code_code = 88;
pub type __suseconds_t = libc::c_long;
pub const KEYC_TRIPLECLICK2_STATUS: unnamed_30 = 268435520;
pub const TTYC_SITM: tty_code_code = 194;
pub const KEYC_TRIPLECLICK3_STATUS: unnamed_30 = 268435523;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub const KEYC_TRIPLECLICK3_PANE: unnamed_30 = 268435522;
pub const TTYC_VPA: tty_code_code = 205;
pub const TTYC_SMSO: tty_code_code = 198;
pub const TTYC_KDC2: tty_code_code = 48;
pub const KEYC_TRIPLECLICK2_PANE: unnamed_30 = 268435519;
pub const KEYC_WHEELDOWN_PANE: unnamed_30 = 268435504;
pub const LINE_SEL_RIGHT_LEFT: unnamed_12 = 2;
pub const TTYC_ECH: tty_code_code = 29;
pub const CMD_RETURN_STOP: cmd_retval = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const TTYC_KIC2: tty_code_code = 138;
pub const KEYC_KP_STAR: unnamed_30 = 268435550;
pub const KEYC_DOUBLECLICK2_STATUS: unnamed_30 = 268435511;
pub const TTYC_KF18: tty_code_code = 77;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const TTYC_KF11: tty_code_code = 70;
pub const TTYC_FSL: tty_code_code = 34;
pub const TTYC_KF54: tty_code_code = 117;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const TTYC_KF7: tty_code_code = 128;
pub const TTYC_KIC6: tty_code_code = 142;
pub const TTYC_KHOM4: tty_code_code = 133;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const TTYC_KDC5: tty_code_code = 51;
pub type unnamed_12 = libc::c_uint;
pub const LINE_SEL_NONE: unnamed_12 = 0;
pub const TTYC_ENACS: tty_code_code = 33;
pub const TTYC_IL1: tty_code_code = 40;
pub type u_char = __u_char;
pub const TTYC_KF17: tty_code_code = 76;
pub const TTYC_KF49: tty_code_code = 111;
pub const TTYC_BEL: tty_code_code = 3;
pub const TTYC_KF4: tty_code_code = 101;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
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
pub const KEYC_KP_ONE: unnamed_30 = 268435559;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed,
}
pub const TTYC_KRIT3: tty_code_code = 169;
pub const TTYC_ED: tty_code_code = 30;
pub const KEYC_F4: unnamed_30 = 268435529;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const TTYC_KIC3: tty_code_code = 139;
pub type uint8_t = libc::c_uchar;
pub const KEYC_WHEELUP_PANE: unnamed_30 = 268435501;
pub const KEYC_MOUSEUP3_PANE: unnamed_30 = 268435480;
pub const TTYC_KF43: tty_code_code = 105;
pub type __ssize_t = libc::c_long;
pub const TTYC_KPRV7: tty_code_code = 166;
pub type unnamed_14 = libc::c_uint;
pub const KEYC_DOUBLECLICK3_PANE: unnamed_30 = 268435513;
pub const TTYC_HOME: tty_code_code = 35;
pub const KEYC_MOUSEUP3_STATUS: unnamed_30 = 268435481;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub type __time_t = libc::c_long;
pub const KEYC_NPAGE: unnamed_30 = 268435542;
pub const KEYC_MOUSEDRAGEND3_PANE: unnamed_30 = 268435498;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const TTYC_CUF: tty_code_code = 17;
pub const TTYC_KF30: tty_code_code = 91;
pub type bitstr_t = libc::c_uchar;
pub const KEYC_DOUBLECLICK2_PANE: unnamed_30 = 268435510;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub const TTYC_KF3: tty_code_code = 90;
pub const TTYC_KIC7: tty_code_code = 143;
pub const TTYC_CNORM: tty_code_code = 8;
pub const TTYC_KNXT2: tty_code_code = 154;
pub const TTYC_SMACS: tty_code_code = 195;
pub const TTYC_KLFT3: tty_code_code = 147;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const TTYC_KHOM2: tty_code_code = 131;
pub const KEYC_MOUSEUP3_BORDER: unnamed_30 = 268435482;
pub const TTYC_KLFT6: tty_code_code = 150;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const TTYC_AX: tty_code_code = 0;
pub const TTYC_KDCH1: tty_code_code = 54;
pub const TTYC_CUB: tty_code_code = 13;
pub const TTYC_SETAB: tty_code_code = 189;
pub const TTYC_KDN3: tty_code_code = 56;
pub const KEYC_MOUSEDOWN3_PANE: unnamed_30 = 268435471;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub const KEYC_DOUBLECLICK1_BORDER: unnamed_30 = 268435509;
pub const TTYC_KF48: tty_code_code = 110;
pub const KEYC_F6: unnamed_30 = 268435531;
pub const TTYC_CS: tty_code_code = 11;
pub const TTYC_KDN6: tty_code_code = 59;
pub const KEYC_MOUSEDRAG2_STATUS: unnamed_30 = 268435487;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const KEYC_MOUSEDRAG1_BORDER: unnamed_30 = 268435485;
pub const TTYC_KF50: tty_code_code = 113;
pub const TTYC_KCUD1: tty_code_code = 45;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_18 {
    ev_next_with_common_timeout: unnamed_7,
    min_heap_idx: libc::c_int,
}
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const TTYC_KRIT2: tty_code_code = 168;
pub const TTYC_SE: tty_code_code = 188;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_31,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_19 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const TTY_VT102: unnamed_35 = 2;
pub type options_table_scope = libc::c_uint;
pub const TTYC_KF8: tty_code_code = 129;
pub const TTYC_KF31: tty_code_code = 92;
pub const TTYC_KHOM5: tty_code_code = 134;
pub const TTYC_KF35: tty_code_code = 96;
pub const UTF8_ERROR: utf8_state = 2;
pub const TTYC_KCUU1: tty_code_code = 47;
pub const KEYC_DOWN: unnamed_30 = 268435546;
pub const TTYC_KDN5: tty_code_code = 58;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_11,
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
    pub term_type: unnamed_35,
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
pub const TTYC_KRIT6: tty_code_code = 172;
pub const TTYC_KRIT4: tty_code_code = 170;
pub const KEYC_MOUSEDOWN3_STATUS: unnamed_30 = 268435472;
pub const TTYC_SETAF: tty_code_code = 190;
pub const TTYC_BCE: tty_code_code = 2;
pub const TTYC_KF26: tty_code_code = 86;
pub const KEYC_KP_MINUS: unnamed_30 = 268435551;
pub const TTYC_CR: tty_code_code = 10;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub const KEYC_DOUBLECLICK1_PANE: unnamed_30 = 268435507;
pub const KEYC_DOUBLECLICK2_BORDER: unnamed_30 = 268435512;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const KEYC_MOUSEDOWN1_PANE: unnamed_30 = 268435465;
pub const TTYC_KF52: tty_code_code = 115;
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
    pub message_log: unnamed_3,
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
    pub entry: unnamed_39,
}
pub const TTYC_KUP2: tty_code_code = 174;
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
    pub gentry: unnamed_5,
    pub entry: unnamed_21,
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
pub const KEYC_KP_PERIOD: unnamed_30 = 268435564;
pub const KEYC_MOUSEDRAG3_PANE: unnamed_30 = 268435489;
pub const KEYC_MOUSEMOVE_PANE: unnamed_30 = 268435462;
pub const TTYC_DL: tty_code_code = 26;
pub type __u_short = libc::c_ushort;
pub const KEYC_MOUSEDRAGEND2_STATUS: unnamed_30 = 268435496;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const TTYC_BLINK: tty_code_code = 4;
pub const TTYC_KF41: tty_code_code = 103;
pub const TTYC_TC: tty_code_code = 202;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const TTYC_KF56: tty_code_code = 119;
pub const TTYC_KLFT4: tty_code_code = 148;
pub const KEYC_MOUSEDRAGEND3_STATUS: unnamed_30 = 268435499;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const KEYC_MOUSEDOWN1_BORDER: unnamed_30 = 268435467;
pub const TTYC_CIVIS: tty_code_code = 6;
pub const TTYC_SETRGBF: tty_code_code = 192;
pub const TTYC_RGB: tty_code_code = 183;
pub const KEYC_MOUSEUP2_STATUS: unnamed_30 = 268435478;
pub const KEYC_UP: unnamed_30 = 268435545;
pub const KEYC_IC: unnamed_30 = 268435538;
pub const KEYC_KP_SIX: unnamed_30 = 268435558;
pub const KEYC_MOUSEDOWN1_STATUS: unnamed_30 = 268435466;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub ev_io_next: unnamed_8,
    pub ev_timeout: timeval,
}
pub const KEYC_MOUSEDRAGEND1_STATUS: unnamed_30 = 268435493;
pub const TTYC_KF13: tty_code_code = 72;
pub type __u_int = libc::c_uint;
pub const KEYC_KP_FIVE: unnamed_30 = 268435557;
pub const TTYC_KF57: tty_code_code = 120;
pub const TTYC_KF61: tty_code_code = 125;
pub const TTYC_SGR0: tty_code_code = 193;
pub const TTYC_KPRV5: tty_code_code = 164;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const TTYC_KDC4: tty_code_code = 50;
pub const TTYC_KF36: tty_code_code = 97;
pub const CMDQ_COMMAND: cmdq_type = 0;
pub type _IO_lock_t = ();
pub const TTYC_KF42: tty_code_code = 104;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_27 {
    ev_io: unnamed_25,
    ev_signal: unnamed_37,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const TTYC_HPA: tty_code_code = 36;
pub const TTYC_CUD1: tty_code_code = 16;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_default_key_code {
    pub code: tty_code_code,
    pub key: key_code,
}
pub const TTYC_KF39: tty_code_code = 100;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const TTY_UNKNOWN: unnamed_35 = 6;
pub const KEYC_MOUSE: unnamed_30 = 268435460;
pub const TTYC_KF6: tty_code_code = 123;
pub const LINE_SEL_LEFT_RIGHT: unnamed_12 = 1;
pub const TTYC_KF21: tty_code_code = 81;
pub type __u_char = libc::c_uchar;
pub const KEYC_TRIPLECLICK2_BORDER: unnamed_30 = 268435521;
pub const TTYC_KIC4: tty_code_code = 140;
pub const KEYC_MOUSEDRAGEND1_PANE: unnamed_30 = 268435492;
pub const TTYC_KCUB1: tty_code_code = 44;
pub const TTYC_KF14: tty_code_code = 73;
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const TTYC_KF62: tty_code_code = 126;
pub const KEYC_F7: unnamed_30 = 268435532;
pub const KEYC_PASTE_START: unnamed_30 = 268435458;
pub const TTY_VT100: unnamed_35 = 0;
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_22,
}
pub const KEYC_F9: unnamed_30 = 268435534;
pub const TTYC_KF5: tty_code_code = 112;
pub type __off64_t = libc::c_long;
pub const TTYC_OP: tty_code_code = 181;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const KEYC_F5: unnamed_30 = 268435530;
pub const TTYC_KRIT7: tty_code_code = 173;
pub const TTYC_KPRV4: tty_code_code = 163;
pub type cmdq_type = libc::c_uint;
pub const TTYC_U8: tty_code_code = 204;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const KEYC_F11: unnamed_30 = 268435536;
pub const KEYC_MOUSEUP1_STATUS: unnamed_30 = 268435475;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const TTYC_KF33: tty_code_code = 94;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_38,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const TTYC_KCBT: tty_code_code = 43;
pub const KEYC_WHEELUP_STATUS: unnamed_30 = 268435502;
pub const TTYC_CSR: tty_code_code = 12;
pub type unnamed_30 = libc::c_uint;
pub const PROMPT_ENTRY: unnamed_14 = 0;
pub const KEYC_MOUSEDRAGEND2_BORDER: unnamed_30 = 268435497;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub const TTYC_CUD: tty_code_code = 15;
pub const TTYC_KF23: tty_code_code = 83;
pub const TTYC_KDC3: tty_code_code = 49;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub const TTYC_KDC7: tty_code_code = 53;
pub const TTYC_KEND7: tty_code_code = 67;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const TTYC_KPRV6: tty_code_code = 165;
pub const KEYC_MOUSEDRAG1_PANE: unnamed_30 = 268435483;
pub const TTYC_KF10: tty_code_code = 69;
pub const TTYC_KUP4: tty_code_code = 176;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub type __off_t = libc::c_long;
pub const TTYC_KUP7: tty_code_code = 179;
pub const TTYC_REV: tty_code_code = 182;
pub const TTYC_DCH1: tty_code_code = 24;
pub const TTYC_KHOM3: tty_code_code = 132;
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
    pub alerts_entry: unnamed_10,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_32,
    pub entry: unnamed_26,
}
pub const TTYC_CUF1: tty_code_code = 18;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const KEYC_KP_TWO: unnamed_30 = 268435560;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const KEYC_RIGHT: unnamed_30 = 268435548;
pub const TTYC_MS: tty_code_code = 180;
pub const KEYC_KP_FOUR: unnamed_30 = 268435556;
pub const TTYC_KF15: tty_code_code = 74;
pub const KEYC_MOUSEDOWN2_STATUS: unnamed_30 = 268435469;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const TTYC_COLORS: tty_code_code = 9;
pub const TTYC_INDN: tty_code_code = 41;
pub const TTYC_KNP: tty_code_code = 153;
pub const KEYC_F10: unnamed_30 = 268435535;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_19,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const TTYC_DIM: tty_code_code = 25;
pub const KEYC_MOUSEDOWN2_PANE: unnamed_30 = 268435468;
pub const KEYC_MOUSEUP1_BORDER: unnamed_30 = 268435476;
pub const KEYC_BTAB: unnamed_30 = 268435544;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const TTYC_KF9: tty_code_code = 130;
pub const KEYC_F8: unnamed_30 = 268435533;
pub const TTYC_KNXT5: tty_code_code = 157;
pub const TTYC_KPRV3: tty_code_code = 162;
pub const JOB_DEAD: unnamed_6 = 1;
pub const KEYC_MOUSEMOVE_BORDER: unnamed_30 = 268435464;
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
pub const KEYC_END: unnamed_30 = 268435541;
pub const UTF8_DONE: utf8_state = 1;
pub const TTYC_KIND: tty_code_code = 145;
pub type utf8_state = libc::c_uint;
pub const TTYC_CUP: tty_code_code = 19;
pub const TTYC_CVVIS: tty_code_code = 22;
pub const TTYC_SMKX: tty_code_code = 197;
pub const KEYC_MOUSEDOWN2_BORDER: unnamed_30 = 268435470;
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
pub struct tty_default_key_raw {
    pub string: *const libc::c_char,
    pub key: key_code,
}
pub const KEYC_DOUBLECLICK1_STATUS: unnamed_30 = 268435508;
pub const TTYC_SETRGBB: tty_code_code = 191;
pub const TTYC_KF40: tty_code_code = 102;
pub const TTYC_KMOUS: tty_code_code = 152;
pub const TTYC_KF25: tty_code_code = 85;
pub const TTY_VT220: unnamed_35 = 3;
pub const TTYC_KNXT7: tty_code_code = 159;
pub const TTYC_KF38: tty_code_code = 99;
pub const TTYC_KEND5: tty_code_code = 65;
pub const KEYC_TRIPLECLICK1_PANE: unnamed_30 = 268435516;
pub const KEYC_KP_THREE: unnamed_30 = 268435561;
pub const TTYC_CUU1: tty_code_code = 21;
pub const KEYC_LEFT: unnamed_30 = 268435547;
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const TTYC_KRIT5: tty_code_code = 171;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const TTYC_KF20: tty_code_code = 80;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_KIC5: tty_code_code = 141;
pub type time_t = __time_t;
pub const TTYC_KDN4: tty_code_code = 57;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_1,
}
pub const KEYC_WHEELUP_BORDER: unnamed_30 = 268435503;
pub const TTYC_KNXT4: tty_code_code = 156;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type tty_code_code = libc::c_uint;
pub const TTYC_KEND3: tty_code_code = 63;
pub const TTYC_ICH1: tty_code_code = 38;
pub type options_table_type = libc::c_uint;
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
pub const TTYC_CUU: tty_code_code = 20;
pub type unnamed_35 = libc::c_uint;
pub const TTYC_KF53: tty_code_code = 116;
pub const TTYC_KLFT7: tty_code_code = 151;
pub const TTYC_XT: tty_code_code = 207;
pub const TTYC_KLFT2: tty_code_code = 146;
pub const TTYC_IL: tty_code_code = 39;
pub const TTYC_XENL: tty_code_code = 206;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const KEYC_DOUBLECLICK3_BORDER: unnamed_30 = 268435515;
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
    pub entry: unnamed_36,
    pub tree_entry: unnamed_28,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const TTYC_KF58: tty_code_code = 121;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const TTYC_CUB1: tty_code_code = 14;
pub type speed_t = libc::c_uint;
pub const TTYC_KF32: tty_code_code = 93;
pub type pid_t = __pid_t;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub ev_signal_next: unnamed_9,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const KEYC_PASTE_END: unnamed_30 = 268435459;
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
pub const TTYC_RMACS: tty_code_code = 185;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
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
pub const KEYC_MOUSEDRAGEND2_PANE: unnamed_30 = 268435495;
pub const KEYC_MOUSEDRAG2_BORDER: unnamed_30 = 268435488;
pub const TTYC_SMXX: tty_code_code = 200;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub const TTYC_KEND6: tty_code_code = 66;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const TTYC_RI: tty_code_code = 184;
pub const KEYC_MOUSEDRAG2_PANE: unnamed_30 = 268435486;
pub type u_int = __u_int;
pub const KEYC_MOUSEUP1_PANE: unnamed_30 = 268435474;
pub const KEYC_TRIPLECLICK1_STATUS: unnamed_30 = 268435517;
pub const TTYC_KDC6: tty_code_code = 52;
pub const TTYC_KF60: tty_code_code = 124;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const KEYC_DRAGGING: unnamed_30 = 268435461;
pub const PROMPT_COMMAND: unnamed_14 = 1;
pub const KEYC_PPAGE: unnamed_30 = 268435543;
pub const TTYC_KF47: tty_code_code = 109;
pub const KEYC_MOUSEDOWN3_BORDER: unnamed_30 = 268435473;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub const KEYC_KP_SEVEN: unnamed_30 = 268435552;
pub const TTY_VT320: unnamed_35 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_33,
    pub ev_next: unnamed_29,
    pub ev_timeout_pos: unnamed_18,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_27,
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
pub const JOB_CLOSED: unnamed_6 = 2;
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
    pub entry: unnamed_13,
}
#[no_mangle]
pub unsafe extern "C" fn tty_keys_build(mut tty: *mut tty) -> () {
    let mut tdkr: *const tty_default_key_raw =
        0 as *const tty_default_key_raw;
    let mut tdkc: *const tty_default_key_code =
        0 as *const tty_default_key_code;
    let mut i: u_int = 0;
    let mut size: u_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    if (*tty).key_tree != 0 as *mut libc::c_void as *mut tty_key {
        tty_keys_free(tty);
    }
    (*tty).key_tree = 0 as *mut tty_key;
    i = 0i32 as u_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[tty_default_key_raw; 94]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<tty_default_key_raw>()
                                                   as libc::c_ulong) {
        tdkr =
            &tty_default_raw_keys[i as usize] as *const tty_default_key_raw;
        s = (*tdkr).string;
        if *s as libc::c_int != 0 { tty_keys_add(tty, s, (*tdkr).key); }
        i = i.wrapping_add(1)
    }
    i = 0i32 as u_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[tty_default_key_code; 136]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<tty_default_key_code>()
                                                   as libc::c_ulong) {
        tdkc =
            &tty_default_code_keys[i as usize] as *const tty_default_key_code;
        s = tty_term_string((*tty).term, (*tdkc).code);
        if *s as libc::c_int != 0 { tty_keys_add(tty, s, (*tdkc).key); }
        i = i.wrapping_add(1)
    }
    o =
        options_get(global_options,
                    b"user-keys\x00" as *const u8 as *const libc::c_char);
    if o != 0 as *mut libc::c_void as *mut options_entry &&
           options_array_size(o, &mut size as *mut u_int) !=
               1i32.wrapping_neg() {
        i = 0i32 as u_int;
        while i < size {
            value = options_array_get(o, i);
            if value != 0 as *mut libc::c_void as *const libc::c_char {
                tty_keys_add(tty, value,
                             536870912u64.wrapping_add(i as
                                                           libc::c_ulonglong));
            }
            i = i.wrapping_add(1)
        }
    };
}
unsafe extern "C" fn tty_keys_add(mut tty: *mut tty,
                                  mut s: *const libc::c_char,
                                  mut key: key_code) -> () {
    let mut tk: *mut tty_key = 0 as *mut tty_key;
    let mut size: size_t = 0;
    let mut keystr: *const libc::c_char = 0 as *const libc::c_char;
    keystr = key_string_lookup_key(key);
    tk = tty_keys_find(tty, s, strlen(s), &mut size as *mut size_t);
    if tk == 0 as *mut libc::c_void as *mut tty_key {
        log_debug(b"new key %s: 0x%llx (%s)\x00" as *const u8 as
                      *const libc::c_char, s, key, keystr);
        tty_keys_add1(&mut (*tty).key_tree as *mut *mut tty_key, s, key);
    } else {
        log_debug(b"replacing key %s: 0x%llx (%s)\x00" as *const u8 as
                      *const libc::c_char, s, key, keystr);
        (*tk).key = key
    };
}
unsafe extern "C" fn tty_keys_add1(mut tkp: *mut *mut tty_key,
                                   mut s: *const libc::c_char,
                                   mut key: key_code) -> () {
    let mut tk: *mut tty_key = 0 as *mut tty_key;
    tk = *tkp;
    if tk == 0 as *mut libc::c_void as *mut tty_key {
        *tkp =
            xcalloc(1i32 as size_t,
                    ::std::mem::size_of::<tty_key>() as libc::c_ulong) as
                *mut tty_key;
        tk = *tkp;
        (*tk).ch = *s;
        (*tk).key = 281466386776064u64
    }
    if *s as libc::c_int == (*tk).ch as libc::c_int {
        s = s.offset(1isize);
        if *s as libc::c_int == 0 {
            (*tk).key = key;
            return
        } else { tkp = &mut (*tk).next as *mut *mut tty_key }
    } else if (*s as libc::c_int) < (*tk).ch as libc::c_int {
        tkp = &mut (*tk).left as *mut *mut tty_key
    } else if *s as libc::c_int > (*tk).ch as libc::c_int {
        tkp = &mut (*tk).right as *mut *mut tty_key
    }
    tty_keys_add1(tkp, s, key);
}
unsafe extern "C" fn tty_keys_find(mut tty: *mut tty,
                                   mut buf: *const libc::c_char,
                                   mut len: size_t, mut size: *mut size_t)
 -> *mut tty_key {
    *size = 0i32 as size_t;
    return tty_keys_find1((*tty).key_tree, buf, len, size);
}
unsafe extern "C" fn tty_keys_find1(mut tk: *mut tty_key,
                                    mut buf: *const libc::c_char,
                                    mut len: size_t, mut size: *mut size_t)
 -> *mut tty_key {
    if tk == 0 as *mut libc::c_void as *mut tty_key {
        return 0 as *mut tty_key
    } else {
        if (*tk).ch as libc::c_int == *buf as libc::c_int {
            buf = buf.offset(1isize);
            len = len.wrapping_sub(1);
            *size = (*size).wrapping_add(1);
            if len == 0i32 as libc::c_ulong ||
                   (*tk).next == 0 as *mut libc::c_void as *mut tty_key &&
                       (*tk).key != 281466386776064u64 {
                return tk
            } else { tk = (*tk).next }
        } else if (*buf as libc::c_int) < (*tk).ch as libc::c_int {
            tk = (*tk).left
        } else if *buf as libc::c_int > (*tk).ch as libc::c_int {
            tk = (*tk).right
        }
        return tty_keys_find1(tk, buf, len, size)
    };
}
static mut tty_default_code_keys: [tty_default_key_code; 136] =
    unsafe {
        [tty_default_key_code{code: TTYC_KF1,
                              key: KEYC_F1 as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KF2,
                              key: KEYC_F2 as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KF3,
                              key: KEYC_F3 as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KF4,
                              key: KEYC_F4 as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KF5,
                              key: KEYC_F5 as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KF6,
                              key: KEYC_F6 as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KF7,
                              key: KEYC_F7 as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KF8,
                              key: KEYC_F8 as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KF9,
                              key: KEYC_F9 as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KF10,
                              key: KEYC_F10 as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KF11,
                              key: KEYC_F11 as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KF12,
                              key: KEYC_F12 as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KF13,
                              key:
                                  KEYC_F1 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64,},
         tty_default_key_code{code: TTYC_KF14,
                              key:
                                  KEYC_F2 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64,},
         tty_default_key_code{code: TTYC_KF15,
                              key:
                                  KEYC_F3 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64,},
         tty_default_key_code{code: TTYC_KF16,
                              key:
                                  KEYC_F4 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64,},
         tty_default_key_code{code: TTYC_KF17,
                              key:
                                  KEYC_F5 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64,},
         tty_default_key_code{code: TTYC_KF18,
                              key:
                                  KEYC_F6 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64,},
         tty_default_key_code{code: TTYC_KF19,
                              key:
                                  KEYC_F7 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64,},
         tty_default_key_code{code: TTYC_KF20,
                              key:
                                  KEYC_F8 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64,},
         tty_default_key_code{code: TTYC_KF21,
                              key:
                                  KEYC_F9 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64,},
         tty_default_key_code{code: TTYC_KF22,
                              key:
                                  KEYC_F10 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64,},
         tty_default_key_code{code: TTYC_KF23,
                              key:
                                  KEYC_F11 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64,},
         tty_default_key_code{code: TTYC_KF24,
                              key:
                                  KEYC_F12 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64,},
         tty_default_key_code{code: TTYC_KF25,
                              key:
                                  KEYC_F1 as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64,},
         tty_default_key_code{code: TTYC_KF26,
                              key:
                                  KEYC_F2 as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64,},
         tty_default_key_code{code: TTYC_KF27,
                              key:
                                  KEYC_F3 as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64,},
         tty_default_key_code{code: TTYC_KF28,
                              key:
                                  KEYC_F4 as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64,},
         tty_default_key_code{code: TTYC_KF29,
                              key:
                                  KEYC_F5 as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64,},
         tty_default_key_code{code: TTYC_KF30,
                              key:
                                  KEYC_F6 as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64,},
         tty_default_key_code{code: TTYC_KF31,
                              key:
                                  KEYC_F7 as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64,},
         tty_default_key_code{code: TTYC_KF32,
                              key:
                                  KEYC_F8 as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64,},
         tty_default_key_code{code: TTYC_KF33,
                              key:
                                  KEYC_F9 as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64,},
         tty_default_key_code{code: TTYC_KF34,
                              key:
                                  KEYC_F10 as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64,},
         tty_default_key_code{code: TTYC_KF35,
                              key:
                                  KEYC_F11 as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64,},
         tty_default_key_code{code: TTYC_KF36,
                              key:
                                  KEYC_F12 as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64,},
         tty_default_key_code{code: TTYC_KF37,
                              key:
                                  KEYC_F1 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      70368744177664u64,},
         tty_default_key_code{code: TTYC_KF38,
                              key:
                                  KEYC_F2 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      70368744177664u64,},
         tty_default_key_code{code: TTYC_KF39,
                              key:
                                  KEYC_F3 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      70368744177664u64,},
         tty_default_key_code{code: TTYC_KF40,
                              key:
                                  KEYC_F4 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      70368744177664u64,},
         tty_default_key_code{code: TTYC_KF41,
                              key:
                                  KEYC_F5 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      70368744177664u64,},
         tty_default_key_code{code: TTYC_KF42,
                              key:
                                  KEYC_F6 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      70368744177664u64,},
         tty_default_key_code{code: TTYC_KF43,
                              key:
                                  KEYC_F7 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      70368744177664u64,},
         tty_default_key_code{code: TTYC_KF44,
                              key:
                                  KEYC_F8 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      70368744177664u64,},
         tty_default_key_code{code: TTYC_KF45,
                              key:
                                  KEYC_F9 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      70368744177664u64,},
         tty_default_key_code{code: TTYC_KF46,
                              key:
                                  KEYC_F10 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      70368744177664u64,},
         tty_default_key_code{code: TTYC_KF47,
                              key:
                                  KEYC_F11 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      70368744177664u64,},
         tty_default_key_code{code: TTYC_KF48,
                              key:
                                  KEYC_F12 as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      70368744177664u64,},
         tty_default_key_code{code: TTYC_KF49,
                              key:
                                  KEYC_F1 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64,},
         tty_default_key_code{code: TTYC_KF50,
                              key:
                                  KEYC_F2 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64,},
         tty_default_key_code{code: TTYC_KF51,
                              key:
                                  KEYC_F3 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64,},
         tty_default_key_code{code: TTYC_KF52,
                              key:
                                  KEYC_F4 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64,},
         tty_default_key_code{code: TTYC_KF53,
                              key:
                                  KEYC_F5 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64,},
         tty_default_key_code{code: TTYC_KF54,
                              key:
                                  KEYC_F6 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64,},
         tty_default_key_code{code: TTYC_KF55,
                              key:
                                  KEYC_F7 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64,},
         tty_default_key_code{code: TTYC_KF56,
                              key:
                                  KEYC_F8 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64,},
         tty_default_key_code{code: TTYC_KF57,
                              key:
                                  KEYC_F9 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64,},
         tty_default_key_code{code: TTYC_KF58,
                              key:
                                  KEYC_F10 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64,},
         tty_default_key_code{code: TTYC_KF59,
                              key:
                                  KEYC_F11 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64,},
         tty_default_key_code{code: TTYC_KF60,
                              key:
                                  KEYC_F12 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64,},
         tty_default_key_code{code: TTYC_KF61,
                              key:
                                  KEYC_F1 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64 |
                                      140737488355328u64,},
         tty_default_key_code{code: TTYC_KF62,
                              key:
                                  KEYC_F2 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64 |
                                      140737488355328u64,},
         tty_default_key_code{code: TTYC_KF63,
                              key:
                                  KEYC_F3 as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64 |
                                      140737488355328u64,},
         tty_default_key_code{code: TTYC_KICH1,
                              key: KEYC_IC as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KDCH1,
                              key: KEYC_DC as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KHOME,
                              key: KEYC_HOME as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KEND,
                              key: KEYC_END as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KNP,
                              key: KEYC_NPAGE as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KPP,
                              key: KEYC_PPAGE as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KCBT,
                              key: KEYC_BTAB as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KCUU1,
                              key: KEYC_UP as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KCUD1,
                              key: KEYC_DOWN as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KCUB1,
                              key: KEYC_LEFT as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KCUF1,
                              key: KEYC_RIGHT as libc::c_int as key_code,},
         tty_default_key_code{code: TTYC_KDC2,
                              key:
                                  KEYC_DC as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KDC3,
                              key:
                                  KEYC_DC as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KDC4,
                              key:
                                  KEYC_DC as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 | 35184372088832u64
                                      | 281474976710656u64,},
         tty_default_key_code{code: TTYC_KDC5,
                              key:
                                  KEYC_DC as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KDC6,
                              key:
                                  KEYC_DC as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 | 70368744177664u64
                                      | 281474976710656u64,},
         tty_default_key_code{code: TTYC_KDC7,
                              key:
                                  KEYC_DC as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64 | 70368744177664u64
                                      | 281474976710656u64,},
         tty_default_key_code{code: TTYC_KIND,
                              key:
                                  KEYC_DOWN as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KDN2,
                              key:
                                  KEYC_DOWN as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KDN3,
                              key:
                                  KEYC_DOWN as libc::c_int as
                                      libc::c_ulonglong | 35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KDN4,
                              key:
                                  KEYC_DOWN as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KDN5,
                              key:
                                  KEYC_DOWN as libc::c_int as
                                      libc::c_ulonglong | 70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KDN6,
                              key:
                                  KEYC_DOWN as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KDN7,
                              key:
                                  KEYC_DOWN as libc::c_int as
                                      libc::c_ulonglong | 35184372088832u64 |
                                      70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KEND2,
                              key:
                                  KEYC_END as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KEND3,
                              key:
                                  KEYC_END as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KEND4,
                              key:
                                  KEYC_END as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 | 35184372088832u64
                                      | 281474976710656u64,},
         tty_default_key_code{code: TTYC_KEND5,
                              key:
                                  KEYC_END as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KEND6,
                              key:
                                  KEYC_END as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 | 70368744177664u64
                                      | 281474976710656u64,},
         tty_default_key_code{code: TTYC_KEND7,
                              key:
                                  KEYC_END as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64 | 70368744177664u64
                                      | 281474976710656u64,},
         tty_default_key_code{code: TTYC_KHOM2,
                              key:
                                  KEYC_HOME as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KHOM3,
                              key:
                                  KEYC_HOME as libc::c_int as
                                      libc::c_ulonglong | 35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KHOM4,
                              key:
                                  KEYC_HOME as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KHOM5,
                              key:
                                  KEYC_HOME as libc::c_int as
                                      libc::c_ulonglong | 70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KHOM6,
                              key:
                                  KEYC_HOME as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KHOM7,
                              key:
                                  KEYC_HOME as libc::c_int as
                                      libc::c_ulonglong | 35184372088832u64 |
                                      70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KIC2,
                              key:
                                  KEYC_IC as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KIC3,
                              key:
                                  KEYC_IC as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KIC4,
                              key:
                                  KEYC_IC as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 | 35184372088832u64
                                      | 281474976710656u64,},
         tty_default_key_code{code: TTYC_KIC5,
                              key:
                                  KEYC_IC as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KIC6,
                              key:
                                  KEYC_IC as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 | 70368744177664u64
                                      | 281474976710656u64,},
         tty_default_key_code{code: TTYC_KIC7,
                              key:
                                  KEYC_IC as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64 | 70368744177664u64
                                      | 281474976710656u64,},
         tty_default_key_code{code: TTYC_KLFT2,
                              key:
                                  KEYC_LEFT as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KLFT3,
                              key:
                                  KEYC_LEFT as libc::c_int as
                                      libc::c_ulonglong | 35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KLFT4,
                              key:
                                  KEYC_LEFT as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KLFT5,
                              key:
                                  KEYC_LEFT as libc::c_int as
                                      libc::c_ulonglong | 70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KLFT6,
                              key:
                                  KEYC_LEFT as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KLFT7,
                              key:
                                  KEYC_LEFT as libc::c_int as
                                      libc::c_ulonglong | 35184372088832u64 |
                                      70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KNXT2,
                              key:
                                  KEYC_NPAGE as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KNXT3,
                              key:
                                  KEYC_NPAGE as libc::c_int as
                                      libc::c_ulonglong | 35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KNXT4,
                              key:
                                  KEYC_NPAGE as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KNXT5,
                              key:
                                  KEYC_NPAGE as libc::c_int as
                                      libc::c_ulonglong | 70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KNXT6,
                              key:
                                  KEYC_NPAGE as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KNXT7,
                              key:
                                  KEYC_NPAGE as libc::c_int as
                                      libc::c_ulonglong | 35184372088832u64 |
                                      70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KPRV2,
                              key:
                                  KEYC_PPAGE as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KPRV3,
                              key:
                                  KEYC_PPAGE as libc::c_int as
                                      libc::c_ulonglong | 35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KPRV4,
                              key:
                                  KEYC_PPAGE as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KPRV5,
                              key:
                                  KEYC_PPAGE as libc::c_int as
                                      libc::c_ulonglong | 70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KPRV6,
                              key:
                                  KEYC_PPAGE as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KPRV7,
                              key:
                                  KEYC_PPAGE as libc::c_int as
                                      libc::c_ulonglong | 35184372088832u64 |
                                      70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KRIT2,
                              key:
                                  KEYC_RIGHT as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KRIT3,
                              key:
                                  KEYC_RIGHT as libc::c_int as
                                      libc::c_ulonglong | 35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KRIT4,
                              key:
                                  KEYC_RIGHT as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KRIT5,
                              key:
                                  KEYC_RIGHT as libc::c_int as
                                      libc::c_ulonglong | 70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KRIT6,
                              key:
                                  KEYC_RIGHT as libc::c_int as
                                      libc::c_ulonglong | 140737488355328u64 |
                                      70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KRIT7,
                              key:
                                  KEYC_RIGHT as libc::c_int as
                                      libc::c_ulonglong | 35184372088832u64 |
                                      70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KRI,
                              key:
                                  KEYC_UP as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KUP2,
                              key:
                                  KEYC_UP as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KUP3,
                              key:
                                  KEYC_UP as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KUP4,
                              key:
                                  KEYC_UP as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 | 35184372088832u64
                                      | 281474976710656u64,},
         tty_default_key_code{code: TTYC_KUP5,
                              key:
                                  KEYC_UP as libc::c_int as libc::c_ulonglong
                                      | 70368744177664u64 |
                                      281474976710656u64,},
         tty_default_key_code{code: TTYC_KUP6,
                              key:
                                  KEYC_UP as libc::c_int as libc::c_ulonglong
                                      | 140737488355328u64 | 70368744177664u64
                                      | 281474976710656u64,},
         tty_default_key_code{code: TTYC_KUP7,
                              key:
                                  KEYC_UP as libc::c_int as libc::c_ulonglong
                                      | 35184372088832u64 | 70368744177664u64
                                      | 281474976710656u64,}]
    };
static mut tty_default_raw_keys: [tty_default_key_raw; 94] =
    unsafe {
        [tty_default_key_raw{string:
                                 b"\x1bOo\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_SLASH as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOj\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_STAR as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOm\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_MINUS as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOw\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_SEVEN as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOx\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_EIGHT as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOy\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_NINE as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOk\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_PLUS as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOt\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_FOUR as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOu\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_FIVE as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOv\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_SIX as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOq\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_ONE as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOr\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_TWO as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOs\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_THREE as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOM\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_ENTER as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOp\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_ZERO as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOn\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_KP_PERIOD as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOA\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_UP as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOB\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_DOWN as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOC\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_RIGHT as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOD\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_LEFT as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1b[A\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_UP as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1b[B\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_DOWN as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1b[C\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_RIGHT as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1b[D\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_LEFT as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOH\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_HOME as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOF\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_END as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1b[H\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_HOME as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1b[F\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_END as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1bOa\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_UP as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1bOb\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_DOWN as libc::c_int as libc::c_ulonglong
                                     | 70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1bOc\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_RIGHT as libc::c_int as
                                     libc::c_ulonglong | 70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1bOd\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_LEFT as libc::c_int as libc::c_ulonglong
                                     | 70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[a\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_UP as libc::c_int as libc::c_ulonglong |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[b\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_DOWN as libc::c_int as libc::c_ulonglong
                                     | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[c\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_RIGHT as libc::c_int as
                                     libc::c_ulonglong | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[d\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_LEFT as libc::c_int as libc::c_ulonglong
                                     | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[11^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F1 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[12^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F2 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[13^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F3 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[14^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F4 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[15^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F5 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[17^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F6 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[18^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F7 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[19^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F8 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[20^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F9 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[21^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F10 as libc::c_int as libc::c_ulonglong
                                     | 70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[23^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F11 as libc::c_int as libc::c_ulonglong
                                     | 70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[24^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F12 as libc::c_int as libc::c_ulonglong
                                     | 70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[2^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_IC as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[3^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_DC as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[7^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_HOME as libc::c_int as libc::c_ulonglong
                                     | 70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[8^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_END as libc::c_int as libc::c_ulonglong
                                     | 70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[6^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_NPAGE as libc::c_int as
                                     libc::c_ulonglong | 70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[5^\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_PPAGE as libc::c_int as
                                     libc::c_ulonglong | 70368744177664u64,},
         tty_default_key_raw{string:
                                 b"\x1b[11$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F1 as libc::c_int as libc::c_ulonglong |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[12$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F2 as libc::c_int as libc::c_ulonglong |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[13$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F3 as libc::c_int as libc::c_ulonglong |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[14$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F4 as libc::c_int as libc::c_ulonglong |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[15$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F5 as libc::c_int as libc::c_ulonglong |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[17$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F6 as libc::c_int as libc::c_ulonglong |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[18$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F7 as libc::c_int as libc::c_ulonglong |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[19$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F8 as libc::c_int as libc::c_ulonglong |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[20$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F9 as libc::c_int as libc::c_ulonglong |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[21$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F10 as libc::c_int as libc::c_ulonglong
                                     | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[23$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F11 as libc::c_int as libc::c_ulonglong
                                     | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[24$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F12 as libc::c_int as libc::c_ulonglong
                                     | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[2$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_IC as libc::c_int as libc::c_ulonglong |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[3$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_DC as libc::c_int as libc::c_ulonglong |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[7$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_HOME as libc::c_int as libc::c_ulonglong
                                     | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[8$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_END as libc::c_int as libc::c_ulonglong
                                     | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[6$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_NPAGE as libc::c_int as
                                     libc::c_ulonglong | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[5$\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_PPAGE as libc::c_int as
                                     libc::c_ulonglong | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[11@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F1 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64 | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[12@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F2 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64 | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[13@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F3 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64 | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[14@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F4 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64 | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[15@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F5 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64 | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[17@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F6 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64 | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[18@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F7 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64 | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[19@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F8 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64 | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[20@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F9 as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64 | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[21@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F10 as libc::c_int as libc::c_ulonglong
                                     | 70368744177664u64 |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[23@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F11 as libc::c_int as libc::c_ulonglong
                                     | 70368744177664u64 |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[24@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_F12 as libc::c_int as libc::c_ulonglong
                                     | 70368744177664u64 |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[2@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_IC as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64 | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[3@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_DC as libc::c_int as libc::c_ulonglong |
                                     70368744177664u64 | 140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[7@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_HOME as libc::c_int as libc::c_ulonglong
                                     | 70368744177664u64 |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[8@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_END as libc::c_int as libc::c_ulonglong
                                     | 70368744177664u64 |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[6@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_NPAGE as libc::c_int as
                                     libc::c_ulonglong | 70368744177664u64 |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[5@\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_PPAGE as libc::c_int as
                                     libc::c_ulonglong | 70368744177664u64 |
                                     140737488355328u64,},
         tty_default_key_raw{string:
                                 b"\x1b[I\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_FOCUS_IN as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1b[O\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_FOCUS_OUT as libc::c_int as key_code,},
         tty_default_key_raw{string:
                                 b"\x1b[200~\x00" as *const u8 as
                                     *const libc::c_char,
                             key:
                                 KEYC_PASTE_START as libc::c_int as
                                     key_code,},
         tty_default_key_raw{string:
                                 b"\x1b[201~\x00" as *const u8 as
                                     *const libc::c_char,
                             key: KEYC_PASTE_END as libc::c_int as key_code,}]
    };
#[no_mangle]
pub unsafe extern "C" fn tty_keys_free(mut tty: *mut tty) -> () {
    tty_keys_free1((*tty).key_tree);
}
unsafe extern "C" fn tty_keys_free1(mut tk: *mut tty_key) -> () {
    if (*tk).next != 0 as *mut libc::c_void as *mut tty_key {
        tty_keys_free1((*tk).next);
    }
    if (*tk).left != 0 as *mut libc::c_void as *mut tty_key {
        tty_keys_free1((*tk).left);
    }
    if (*tk).right != 0 as *mut libc::c_void as *mut tty_key {
        tty_keys_free1((*tk).right);
    }
    free(tk as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tty_keys_next(mut tty: *mut tty) -> key_code {
    let mut current_block: u64;
    let mut c: *mut client = (*tty).client;
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut buf: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut size: size_t = 0;
    let mut bspace: cc_t = 0;
    let mut delay: libc::c_int = 0;
    let mut expired: libc::c_int = 0i32;
    let mut n: libc::c_int = 0;
    let mut key: key_code = 0;
    buf =
        evbuffer_pullup((*tty).in_0, 1i32.wrapping_neg() as ssize_t) as
            *const libc::c_char;
    len = evbuffer_get_length((*tty).in_0);
    if len == 0i32 as libc::c_ulong {
        return 0i32 as key_code
    } else {
        log_debug(b"%s: keys are %zu (%.*s)\x00" as *const u8 as
                      *const libc::c_char, (*c).name, len, len as libc::c_int,
                  buf);
        match tty_keys_device_attributes(tty, buf, len,
                                         &mut size as *mut size_t) {
            0 => {
                key = 281466386776064u64;
                current_block = 4487172175363689660;
            }
            1 => { current_block = 5242414451524867936; }
            -1 | _ => {
                match tty_keys_mouse(tty, buf, len, &mut size as *mut size_t)
                    {
                    0 => {
                        current_block = 15906047958950595151;
                        match current_block {
                            12827556049091717404 => {
                                key = KEYC_MOUSE as libc::c_int as key_code;
                                log_debug(b"%s: discard key %.*s %#llx\x00" as
                                              *const u8 as
                                              *const libc::c_char, (*c).name,
                                          size as libc::c_int, buf, key);
                                evbuffer_drain((*tty).in_0, size);
                                return 1i32 as key_code
                            }
                            _ => {
                                key = KEYC_MOUSE as libc::c_int as key_code
                            }
                        }
                        current_block = 4487172175363689660;
                    }
                    -2 => {
                        current_block = 12827556049091717404;
                        match current_block {
                            12827556049091717404 => {
                                key = KEYC_MOUSE as libc::c_int as key_code;
                                log_debug(b"%s: discard key %.*s %#llx\x00" as
                                              *const u8 as
                                              *const libc::c_char, (*c).name,
                                          size as libc::c_int, buf, key);
                                evbuffer_drain((*tty).in_0, size);
                                return 1i32 as key_code
                            }
                            _ => {
                                key = KEYC_MOUSE as libc::c_int as key_code
                            }
                        }
                        current_block = 4487172175363689660;
                    }
                    1 => { current_block = 5242414451524867936; }
                    -1 | _ => { current_block = 8953692784526113934; }
                }
            }
        }
        loop  {
            match current_block {
                4487172175363689660 => {
                    log_debug(b"%s: complete key %.*s %#llx\x00" as *const u8
                                  as *const libc::c_char, (*c).name,
                              size as libc::c_int, buf, key);
                    bspace = (*tty).tio.c_cc[2usize];
                    if !(bspace as libc::c_int != 0 &&
                             key &
                                 !(35184372088832u64 | 70368744177664u64 |
                                       140737488355328u64 |
                                       281474976710656u64) ==
                                 bspace as libc::c_ulonglong) {
                        current_block = 13797916685926291137;
                        break ;
                    }
                    key =
                        key &
                            (35184372088832u64 | 70368744177664u64 |
                                 140737488355328u64 | 281474976710656u64) |
                            KEYC_BSPACE as libc::c_int as libc::c_ulonglong;
                    current_block = 13797916685926291137;
                    break ;
                }
                5242414451524867936 => {
                    log_debug(b"%s: partial key %.*s\x00" as *const u8 as
                                  *const libc::c_char, (*c).name,
                              len as libc::c_int, buf);
                    if 0 != (*tty).flags & 4i32 {
                        if 0 !=
                               event_initialized(&mut (*tty).key_timer as
                                                     *mut event) &&
                               0 ==
                                   event_pending(&mut (*tty).key_timer as
                                                     *mut event,
                                                 1i32 as libc::c_short,
                                                 0 as *mut timeval) {
                            expired = 1i32;
                            current_block = 8953692784526113934;
                        } else { return 0i32 as key_code }
                    } else {
                        delay =
                            options_get_number(global_options,
                                               b"escape-time\x00" as *const u8
                                                   as *const libc::c_char) as
                                libc::c_int;
                        tv.tv_sec = (delay / 1000i32) as __time_t;
                        tv.tv_usec =
                            (delay % 1000i32) as libc::c_long * 1000i64;
                        if 0 !=
                               event_initialized(&mut (*tty).key_timer as
                                                     *mut event) {
                            current_block = 5948590327928692120;
                            break ;
                        } else {
                            current_block = 5689001924483802034;
                            break ;
                        }
                    }
                }
                _ => {
                    n =
                        tty_keys_next1(tty, buf, len,
                                       &mut key as *mut key_code,
                                       &mut size as *mut size_t, expired);
                    if n == 0i32 {
                        current_block = 4487172175363689660;
                        continue ;
                    }
                    if n == 1i32 {
                        current_block = 5242414451524867936;
                        continue ;
                    }
                    if *buf as libc::c_int == 27 {
                        n =
                            tty_keys_next1(tty, buf.offset(1isize),
                                           len.wrapping_sub(1i32 as
                                                                libc::c_ulong),
                                           &mut key as *mut key_code,
                                           &mut size as *mut size_t, expired);
                        if n == 0i32 {
                            if 0 != key & 281474976710656u64 {
                                key = 27 as key_code;
                                size = 1i32 as size_t;
                                current_block = 4487172175363689660;
                                continue ;
                            } else {
                                key |= 35184372088832u64;
                                size = size.wrapping_add(1);
                                current_block = 4487172175363689660;
                                continue ;
                            }
                        } else if n == 1i32 {
                            current_block = 5242414451524867936;
                            continue ;
                        }
                    }
                    if *buf as libc::c_int == 27 &&
                           len >= 2i32 as libc::c_ulong {
                        key =
                            *buf.offset(1isize) as u_char as libc::c_ulonglong
                                | 35184372088832u64;
                        size = 2i32 as size_t;
                        current_block = 4487172175363689660;
                    } else {
                        key = *buf.offset(0isize) as u_char as key_code;
                        size = 1i32 as size_t;
                        current_block = 4487172175363689660;
                    }
                }
            }
        }
        match current_block {
            5948590327928692120 => {
                event_del(&mut (*tty).key_timer as *mut event);
            }
            13797916685926291137 => {
                evbuffer_drain((*tty).in_0, size);
                if 0 != event_initialized(&mut (*tty).key_timer as *mut event)
                   {
                    event_del(&mut (*tty).key_timer as *mut event);
                }
                (*tty).flags &= !4i32;
                if key == KEYC_FOCUS_OUT as libc::c_int as libc::c_ulonglong {
                    (*(*tty).client).flags &= !32768i32;
                    return 1i32 as key_code
                } else if key ==
                              KEYC_FOCUS_IN as libc::c_int as
                                  libc::c_ulonglong {
                    (*(*tty).client).flags |= 32768i32;
                    return 1i32 as key_code
                } else {
                    if key != 281466386776064u64 {
                        server_client_handle_key((*tty).client, key);
                    }
                    return 1i32 as key_code
                }
            }
            _ => { }
        }
        event_set(&mut (*tty).key_timer as *mut event, 1i32.wrapping_neg(),
                  0i32 as libc::c_short, Some(tty_keys_callback),
                  tty as *mut libc::c_void);
        event_add(&mut (*tty).key_timer as *mut event,
                  &mut tv as *mut timeval);
        (*tty).flags |= 4i32;
        return 0i32 as key_code
    };
}
unsafe extern "C" fn tty_keys_callback(mut fd: libc::c_int,
                                       mut events: libc::c_short,
                                       mut data: *mut libc::c_void) -> () {
    let mut current_block: u64;
    let mut tty: *mut tty = data as *mut tty;
    if 0 != (*tty).flags & 4i32 {
        current_block = 16668937799742929182;
    } else { current_block = 8258075665625361029; }
    loop  {
        match current_block {
            16668937799742929182 => {
                if 0 != tty_keys_next(tty) {
                    current_block = 16668937799742929182;
                } else { current_block = 8258075665625361029; }
            }
            _ => { return; }
        }
    };
}
unsafe extern "C" fn tty_keys_next1(mut tty: *mut tty,
                                    mut buf: *const libc::c_char,
                                    mut len: size_t, mut key: *mut key_code,
                                    mut size: *mut size_t,
                                    mut expired: libc::c_int) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut tk: *mut tty_key = 0 as *mut tty_key;
    let mut tk1: *mut tty_key = 0 as *mut tty_key;
    let mut ud: utf8_data =
        utf8_data{data: [0; 9], have: 0, size: 0, width: 0,};
    let mut more: utf8_state = UTF8_MORE;
    let mut i: u_int = 0;
    let mut wc: wchar_t = 0;
    let mut n: libc::c_int = 0;
    log_debug(b"%s: next key is %zu (%.*s) (expired=%d)\x00" as *const u8 as
                  *const libc::c_char, (*c).name, len, len as libc::c_int,
              buf, expired);
    tk = tty_keys_find(tty, buf, len, size);
    if tk != 0 as *mut libc::c_void as *mut tty_key &&
           (*tk).key != 281466386776064u64 {
        tk1 = tk;
        loop  {
            log_debug(b"%s: keys in list: %#llx\x00" as *const u8 as
                          *const libc::c_char, (*c).name, (*tk1).key);
            tk1 = (*tk1).next;
            if !(tk1 != 0 as *mut libc::c_void as *mut tty_key) { break ; }
        }
        if (*tk).next != 0 as *mut libc::c_void as *mut tty_key &&
               0 == expired {
            return 1i32
        } else { *key = (*tk).key; return 0i32 }
    } else {
        n = xterm_keys_find(buf, len, size, key);
        if n == 0i32 {
            return 0i32
        } else if n == 1i32 && 0 == expired {
            return 1i32
        } else {
            more = utf8_open(&mut ud as *mut utf8_data, *buf as u_char);
            if more as libc::c_uint ==
                   UTF8_MORE as libc::c_int as libc::c_uint {
                *size = ud.size as size_t;
                if len < ud.size as libc::c_ulong {
                    if 0 == expired {
                        return 1i32
                    } else { return 1i32.wrapping_neg() }
                } else {
                    i = 1i32 as u_int;
                    while i < ud.size as libc::c_uint {
                        more =
                            utf8_append(&mut ud as *mut utf8_data,
                                        *buf.offset(i as isize) as u_char);
                        i = i.wrapping_add(1)
                    }
                    if more as libc::c_uint !=
                           UTF8_DONE as libc::c_int as libc::c_uint {
                        return 1i32.wrapping_neg()
                    } else if utf8_combine(&mut ud as *mut utf8_data,
                                           &mut wc as *mut wchar_t) as
                                  libc::c_uint !=
                                  UTF8_DONE as libc::c_int as libc::c_uint {
                        return 1i32.wrapping_neg()
                    } else {
                        *key = wc as key_code;
                        log_debug(b"%s: UTF-8 key %.*s %#llx\x00" as *const u8
                                      as *const libc::c_char, (*c).name,
                                  ud.size as libc::c_int, buf, *key);
                        return 0i32
                    }
                }
            } else { return 1i32.wrapping_neg() }
        }
    };
}
unsafe extern "C" fn tty_keys_mouse(mut tty: *mut tty,
                                    mut buf: *const libc::c_char,
                                    mut len: size_t, mut size: *mut size_t)
 -> libc::c_int {
    let mut current_block: u64;
    let mut c: *mut client = (*tty).client;
    let mut m: *mut mouse_event = &mut (*tty).mouse as *mut mouse_event;
    let mut i: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut b: u_int = 0;
    let mut sgr_b: u_int = 0;
    let mut sgr_type: u_char = 0;
    let mut ch: u_char = 0;
    *size = 0i32 as size_t;
    sgr_b = 0i32 as u_int;
    b = sgr_b;
    y = b;
    x = y;
    sgr_type = 32 as u_char;
    if *buf.offset(0isize) as libc::c_int != 27 {
        return 1i32.wrapping_neg()
    } else if len == 1i32 as libc::c_ulong {
        return 1i32
    } else if *buf.offset(1isize) as libc::c_int != 91 {
        return 1i32.wrapping_neg()
    } else if len == 2i32 as libc::c_ulong {
        return 1i32
    } else {
        if *buf.offset(2isize) as libc::c_int == 77 {
            *size = 3i32 as size_t;
            i = 0i32 as u_int;
            loop  {
                if i < 3i32 as libc::c_uint {
                    if len <= *size {
                        return 1i32
                    } else {
                        let fresh0 = *size;
                        *size = (*size).wrapping_add(1);
                        ch = *buf.offset(fresh0 as isize) as u_char;
                        if i == 0i32 as libc::c_uint {
                            b = ch as u_int
                        } else if i == 1i32 as libc::c_uint {
                            x = ch as u_int
                        } else { y = ch as u_int }
                        i = i.wrapping_add(1)
                    }
                } else {
                    log_debug(b"%s: mouse input: %.*s\x00" as *const u8 as
                                  *const libc::c_char, (*c).name,
                              *size as libc::c_int, buf);
                    if b < 32i32 as libc::c_uint {
                        current_block = 1394248824506584008;
                        break ;
                    } else { current_block = 13183875560443969876; break ; }
                }
            }
            match current_block {
                1394248824506584008 => { return 1i32.wrapping_neg() }
                _ => {
                    b =
                        (b as
                             libc::c_uint).wrapping_sub(32i32 as libc::c_uint)
                            as u_int as u_int;
                    if x >= 33i32 as libc::c_uint {
                        x =
                            (x as
                                 libc::c_uint).wrapping_sub(33i32 as
                                                                libc::c_uint)
                                as u_int as u_int
                    } else { x = (256i32 as libc::c_uint).wrapping_sub(x) }
                    if y >= 33i32 as libc::c_uint {
                        y =
                            (y as
                                 libc::c_uint).wrapping_sub(33i32 as
                                                                libc::c_uint)
                                as u_int as u_int
                    } else { y = (256i32 as libc::c_uint).wrapping_sub(y) }
                }
            }
        } else if *buf.offset(2isize) as libc::c_int == 60 {
            *size = 3i32 as size_t;
            loop  {
                if len <= *size {
                    return 1i32
                } else {
                    let fresh1 = *size;
                    *size = (*size).wrapping_add(1);
                    ch = *buf.offset(fresh1 as isize) as u_char;
                    if ch as libc::c_int == 59 { break ; }
                    if (ch as libc::c_int) < 48 || ch as libc::c_int > 57 {
                        return 1i32.wrapping_neg()
                    } else {
                        sgr_b =
                            (10i32 as
                                 libc::c_uint).wrapping_mul(sgr_b).wrapping_add((ch
                                                                                     as
                                                                                     libc::c_int
                                                                                     -
                                                                                     48)
                                                                                    as
                                                                                    libc::c_uint)
                    }
                }
            }
            loop  {
                if len <= *size {
                    return 1i32
                } else {
                    let fresh2 = *size;
                    *size = (*size).wrapping_add(1);
                    ch = *buf.offset(fresh2 as isize) as u_char;
                    if ch as libc::c_int == 59 { break ; }
                    if (ch as libc::c_int) < 48 || ch as libc::c_int > 57 {
                        return 1i32.wrapping_neg()
                    } else {
                        x =
                            (10i32 as
                                 libc::c_uint).wrapping_mul(x).wrapping_add((ch
                                                                                 as
                                                                                 libc::c_int
                                                                                 -
                                                                                 48)
                                                                                as
                                                                                libc::c_uint)
                    }
                }
            }
            loop  {
                if len <= *size {
                    return 1i32
                } else {
                    let fresh3 = *size;
                    *size = (*size).wrapping_add(1);
                    ch = *buf.offset(fresh3 as isize) as u_char;
                    if ch as libc::c_int == 77 || ch as libc::c_int == 109 {
                        log_debug(b"%s: mouse input (SGR): %.*s\x00" as
                                      *const u8 as *const libc::c_char,
                                  (*c).name, *size as libc::c_int, buf);
                        if x < 1i32 as libc::c_uint ||
                               y < 1i32 as libc::c_uint {
                            current_block = 11298138898191919651;
                            break ;
                        } else {
                            current_block = 11307063007268554308;
                            break ;
                        }
                    } else if (ch as libc::c_int) < 48 ||
                                  ch as libc::c_int > 57 {
                        return 1i32.wrapping_neg()
                    } else {
                        y =
                            (10i32 as
                                 libc::c_uint).wrapping_mul(y).wrapping_add((ch
                                                                                 as
                                                                                 libc::c_int
                                                                                 -
                                                                                 48)
                                                                                as
                                                                                libc::c_uint)
                    }
                }
            }
            match current_block {
                11298138898191919651 => { return 1i32.wrapping_neg() }
                _ => {
                    x = x.wrapping_sub(1);
                    y = y.wrapping_sub(1);
                    b = sgr_b;
                    sgr_type = ch;
                    if sgr_type as libc::c_int == 109 {
                        b |= 3i32 as libc::c_uint
                    }
                    if sgr_type as libc::c_int == 109 &&
                           0 != sgr_b & 64i32 as libc::c_uint {
                        return 2i32.wrapping_neg()
                    }
                }
            }
        } else { return 1i32.wrapping_neg() }
        (*m).lx = (*m).x;
        (*m).x = x;
        (*m).ly = (*m).y;
        (*m).y = y;
        (*m).lb = (*m).b;
        (*m).b = b;
        (*m).sgr_type = sgr_type as u_int;
        (*m).sgr_b = sgr_b;
        return 0i32
    };
}
unsafe extern "C" fn tty_keys_device_attributes(mut tty: *mut tty,
                                                mut buf: *const libc::c_char,
                                                mut len: size_t,
                                                mut size: *mut size_t)
 -> libc::c_int {
    let mut current_block: u64;
    let mut c: *mut client = (*tty).client;
    let mut i: u_int = 0;
    let mut a: u_int = 0;
    let mut b: u_int = 0;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut types: [*const libc::c_char; 7] =
        unsafe {
            [b"VT100\x00" as *const u8 as *const libc::c_char,
             b"VT101\x00" as *const u8 as *const libc::c_char,
             b"VT102\x00" as *const u8 as *const libc::c_char,
             b"VT220\x00" as *const u8 as *const libc::c_char,
             b"VT320\x00" as *const u8 as *const libc::c_char,
             b"VT420\x00" as *const u8 as *const libc::c_char,
             b"Unknown\x00" as *const u8 as *const libc::c_char]
        };
    let mut type_0: libc::c_int = 0;
    *size = 0i32 as size_t;
    if *buf.offset(0isize) as libc::c_int != 27 {
        return 1i32.wrapping_neg()
    } else if len == 1i32 as libc::c_ulong {
        return 1i32
    } else if *buf.offset(1isize) as libc::c_int != 91 {
        return 1i32.wrapping_neg()
    } else if len == 2i32 as libc::c_ulong {
        return 1i32
    } else if *buf.offset(2isize) as libc::c_int != 63 {
        return 1i32.wrapping_neg()
    } else if len == 3i32 as libc::c_ulong {
        return 1i32
    } else {
        i = 0i32 as u_int;
        loop  {
            if (i as libc::c_ulong) <
                   (::std::mem::size_of::<[libc::c_char; 64]>() as
                        libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) &&
                   *buf.offset((3i32 as libc::c_uint).wrapping_add(i) as
                                   isize) as libc::c_int != 99 {
                if (3i32 as libc::c_uint).wrapping_add(i) as libc::c_ulong ==
                       len {
                    return 1i32
                } else {
                    tmp[i as usize] =
                        *buf.offset((3i32 as libc::c_uint).wrapping_add(i) as
                                        isize);
                    i = i.wrapping_add(1)
                }
            } else if i as libc::c_ulong ==
                          (::std::mem::size_of::<[libc::c_char; 64]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong)
             {
                current_block = 6937071982253665452;
                break ;
            } else { current_block = 7815301370352969686; break ; }
        }
        match current_block {
            6937071982253665452 => { return 1i32.wrapping_neg() }
            _ => {
                tmp[i as usize] = 0 as libc::c_char;
                *size = (4i32 as libc::c_uint).wrapping_add(i) as size_t;
                a =
                    strtoul(tmp.as_mut_ptr(),
                            &mut endptr as *mut *mut libc::c_char, 10i32) as
                        u_int;
                if *endptr as libc::c_int == 59 {
                    b =
                        strtoul(endptr.offset(1isize),
                                &mut endptr as *mut *mut libc::c_char, 10i32)
                            as u_int;
                    if *endptr as libc::c_int != 0 &&
                           *endptr as libc::c_int != 59 {
                        b = 0i32 as u_int
                    }
                } else { b = 0i32 as u_int; a = b }
                type_0 = TTY_UNKNOWN as libc::c_int;
                match a {
                    1 => {
                        if b == 2i32 as libc::c_uint {
                            type_0 = TTY_VT100 as libc::c_int
                        } else if b == 0i32 as libc::c_uint {
                            type_0 = TTY_VT101 as libc::c_int
                        }
                    }
                    6 => { type_0 = TTY_VT102 as libc::c_int }
                    62 => { type_0 = TTY_VT220 as libc::c_int }
                    63 => { type_0 = TTY_VT320 as libc::c_int }
                    64 => { type_0 = TTY_VT420 as libc::c_int }
                    _ => { }
                }
                tty_set_type(tty, type_0);
                log_debug(b"%s: received DA %.*s (%s)\x00" as *const u8 as
                              *const libc::c_char, (*c).name,
                          *size as libc::c_int, buf, types[type_0 as usize]);
                return 0i32
            }
        }
    };
}

