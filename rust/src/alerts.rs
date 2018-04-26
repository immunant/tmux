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
    pub type evbuffer;
    pub type hooks;
    pub type event_base;
    pub type input_ctx;
    pub type options;
    pub type _IO_FILE_plus;
    pub type screen_titles;
    pub type environ;
    pub type bufferevent_ops;
    pub type args_entry;
    pub type format_tree;
    pub type tty_code;
    pub type format_job_tree;
    pub type tmuxpeer;
    pub type tmuxproc;
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
    fn notify_winlink(_: *const libc::c_char, _: *mut winlink) -> ();
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut all_jobs: joblist;
    #[no_mangle]
    fn tty_putcode(_: *mut tty, _: tty_code_code) -> ();
    #[no_mangle]
    static mut tty_terms: tty_terms;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    static mut key_tables: key_tables;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn window_remove_ref(_: *mut window, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn status_message_set(_: *mut client, _: *const libc::c_char, ...) -> ();
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn server_status_session(_: *mut session) -> ();
    #[no_mangle]
    fn window_add_ref(_: *mut window, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn windows_RB_NEXT(_: *mut window) -> *mut window;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    fn windows_RB_MINMAX(_: *mut windows, _: libc::c_int) -> *mut window;
    #[no_mangle]
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    static grid_default_cell: grid_cell;
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
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const TTYC_DL1: tty_code_code = 27;
pub const TTYC_KPRV7: tty_code_code = 166;
pub type tcflag_t = libc::c_uint;
pub const TTYC_SMUL: tty_code_code = 199;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const TTYC_KRIT7: tty_code_code = 173;
pub const TTYC_KF48: tty_code_code = 110;
pub const TTYC_KF6: tty_code_code = 123;
pub const TTYC_IL: tty_code_code = 39;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_23,
    pub ev_next: unnamed_28,
    pub ev_timeout_pos: unnamed_21,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_5,
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
pub struct unnamed {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub const TTYC_SE: tty_code_code = 188;
pub const TTYC_KEND6: tty_code_code = 66;
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
    pub entry: unnamed_26,
}
pub const TTYC_KF57: tty_code_code = 120;
pub type options_table_type = libc::c_uint;
pub const TTYC_KF55: tty_code_code = 118;
pub const TTYC_KF21: tty_code_code = 81;
pub const TTYC_KIC4: tty_code_code = 140;
pub const TTYC_KF47: tty_code_code = 109;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_18,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const TTYC_KF13: tty_code_code = 72;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_0,
    pub entry: unnamed_38,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const TTYC_KUP4: tty_code_code = 176;
pub type unnamed_1 = libc::c_uint;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const TTYC_KRIT6: tty_code_code = 172;
pub const TTYC_KF23: tty_code_code = 83;
pub const TTYC_KIND: tty_code_code = 145;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub const TTYC_EL: tty_code_code = 31;
pub const TTYC_KF28: tty_code_code = 88;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const TTYC_KF8: tty_code_code = 129;
pub const TTYC_KF56: tty_code_code = 119;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const TTYC_KF43: tty_code_code = 105;
pub const TTYC_KDN6: tty_code_code = 59;
pub const TTYC_E3: tty_code_code = 28;
pub const TTYC_KF54: tty_code_code = 117;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const TTYC_DCH: tty_code_code = 23;
pub const TTYC_TSL: tty_code_code = 203;
pub const TTY_VT320: unnamed_10 = 4;
pub const TTYC_KCUD1: tty_code_code = 45;
pub const TTYC_KF40: tty_code_code = 102;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_5 {
    ev_io: unnamed_15,
    ev_signal: unnamed_25,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_KF25: tty_code_code = 85;
pub const TTYC_BOLD: tty_code_code = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const TTYC_KF10: tty_code_code = 69;
pub const TTYC_CIVIS: tty_code_code = 6;
pub type cmdq_type = libc::c_uint;
pub const TTYC_KHOM3: tty_code_code = 132;
pub const TTYC_CR: tty_code_code = 10;
pub const TTYC_KEND5: tty_code_code = 65;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const TTYC_KF60: tty_code_code = 124;
pub const TTYC_KF7: tty_code_code = 128;
pub const TTYC_KF34: tty_code_code = 95;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
    pub entry: unnamed_4,
}
pub const TTYC_KCUF1: tty_code_code = 46;
pub type pid_t = __pid_t;
pub const TTYC_KPRV2: tty_code_code = 161;
pub const TTYC_RGB: tty_code_code = 183;
pub const JOB_DEAD: unnamed_34 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
pub const TTYC_TC: tty_code_code = 202;
pub const TTYC_KHOM5: tty_code_code = 134;
pub const TTY_VT220: unnamed_10 = 3;
pub const TTYC_KDC4: tty_code_code = 50;
pub type cmd_retval = libc::c_int;
pub const TTYC_OP: tty_code_code = 181;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const TTYC_KDC6: tty_code_code = 52;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_KHOM2: tty_code_code = 131;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub type __u_int = libc::c_uint;
pub const TTYC_KF5: tty_code_code = 112;
pub const TTYC_KDN2: tty_code_code = 55;
pub const TTYC_CSR: tty_code_code = 12;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const TTYC_ENACS: tty_code_code = 33;
pub const TTYC_KF1: tty_code_code = 68;
pub const TTYC_KF44: tty_code_code = 106;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub type unnamed_10 = libc::c_uint;
pub const TTYC_CUB1: tty_code_code = 14;
pub const JOB_CLOSED: unnamed_34 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub tqh_first: *mut window,
    pub tqh_last: *mut *mut window,
}
pub const TTYC_KDCH1: tty_code_code = 54;
pub const TTYC_SETAF: tty_code_code = 190;
pub const TTYC_CUB: tty_code_code = 13;
pub const TTYC_KDC5: tty_code_code = 51;
pub const TTYC_CUD1: tty_code_code = 16;
pub const TTYC_KPRV4: tty_code_code = 163;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub type tty_code_code = libc::c_uint;
pub const TTYC_XT: tty_code_code = 207;
pub const TTYC_KF31: tty_code_code = 92;
pub const TTY_VT420: unnamed_10 = 5;
pub const TTYC_KNXT6: tty_code_code = 158;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_31,
}
pub type cmd_find_type = libc::c_uint;
pub const TTYC_KRIT3: tty_code_code = 169;
pub const TTYC_KRI: tty_code_code = 167;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const TTYC_KF19: tty_code_code = 78;
pub const TTYC_KNXT2: tty_code_code = 154;
pub const TTYC_KHOM6: tty_code_code = 135;
pub const TTYC_KRIT5: tty_code_code = 171;
pub const TTYC_CVVIS: tty_code_code = 22;
pub const TTYC_CUP: tty_code_code = 19;
pub const TTYC_KICH1: tty_code_code = 144;
pub const TTYC_KNXT4: tty_code_code = 156;
pub const TTYC_KF39: tty_code_code = 100;
pub type __off64_t = libc::c_long;
pub const TTYC_KHOM7: tty_code_code = 136;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const TTYC_KF42: tty_code_code = 104;
pub const TTYC_KEND7: tty_code_code = 67;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const TTYC_KF63: tty_code_code = 127;
pub const TTYC_SMSO: tty_code_code = 198;
pub const TTYC_KF11: tty_code_code = 70;
pub const TTYC_KIC6: tty_code_code = 142;
pub const TTYC_KLFT7: tty_code_code = 151;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_33,
}
pub const TTYC_CUF1: tty_code_code = 18;
pub type u_short = __u_short;
pub const TTYC_KLFT6: tty_code_code = 150;
pub const TTYC_CUF: tty_code_code = 17;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const TTYC_ED: tty_code_code = 30;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_35,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const TTYC_KF26: tty_code_code = 86;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
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
pub type layout_type = libc::c_uint;
pub const TTYC_KF12: tty_code_code = 71;
pub const TTYC_RMACS: tty_code_code = 185;
pub const TTYC_BLINK: tty_code_code = 4;
pub const TTYC_KF24: tty_code_code = 84;
pub const TTYC_KRIT2: tty_code_code = 168;
pub const TTYC_KF15: tty_code_code = 74;
pub const TTYC_KF62: tty_code_code = 126;
pub type options_table_scope = libc::c_uint;
pub const TTYC_KEND4: tty_code_code = 64;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const TTYC_KIC3: tty_code_code = 139;
pub const TTYC_ECH: tty_code_code = 29;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
pub const TTYC_KF30: tty_code_code = 91;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub ev_io_next: unnamed_8,
    pub ev_timeout: timeval,
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
    pub entry: unnamed_9,
    pub tree_entry: unnamed_3,
}
pub const TTYC_KF27: tty_code_code = 87;
pub const TTYC_KF9: tty_code_code = 130;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
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
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const TTYC_REV: tty_code_code = 182;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const TTYC_ACSC: tty_code_code = 1;
pub const TTYC_KF51: tty_code_code = 114;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_18 {
    offset: u_int,
    data: unnamed,
}
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const TTYC_KF52: tty_code_code = 115;
pub const TTY_VT101: unnamed_10 = 1;
pub const TTYC_KDC2: tty_code_code = 48;
pub const TTYC_KF38: tty_code_code = 99;
pub const TTYC_BEL: tty_code_code = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const TTYC_DIM: tty_code_code = 25;
pub const TTYC_KF41: tty_code_code = 103;
pub const TTYC_KPRV5: tty_code_code = 164;
pub type uint8_t = libc::c_uchar;
pub const TTYC_INDN: tty_code_code = 41;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
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
    pub entry: unnamed_20,
    pub wentry: unnamed_27,
    pub sentry: unnamed_14,
}
pub const TTYC_KIC5: tty_code_code = 141;
pub const TTY_VT100: unnamed_10 = 0;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_21 {
    ev_next_with_common_timeout: unnamed_32,
    min_heap_idx: libc::c_int,
}
pub const TTYC_KF61: tty_code_code = 125;
pub const TTYC_KF16: tty_code_code = 75;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_KNXT5: tty_code_code = 157;
pub const TTYC_RMKX: tty_code_code = 187;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const TTYC_KCUU1: tty_code_code = 47;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const TTYC_KF46: tty_code_code = 108;
pub const TTYC_KHOM4: tty_code_code = 133;
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
pub const TTYC_KCBT: tty_code_code = 43;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const TTYC_KDC3: tty_code_code = 49;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub const TTYC_SS: tty_code_code = 201;
pub const TTYC_KHOME: tty_code_code = 137;
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
pub struct unnamed_24 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub const TTYC_EL1: tty_code_code = 32;
pub type __time_t = libc::c_long;
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
pub const TTYC_KDN5: tty_code_code = 58;
pub const TTYC_VPA: tty_code_code = 205;
pub const TTYC_KUP5: tty_code_code = 177;
pub const TTYC_KF29: tty_code_code = 89;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const TTYC_KF49: tty_code_code = 111;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_7,
}
pub const TTYC_SETRGBB: tty_code_code = 191;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub ev_signal_next: unnamed_6,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const TTYC_CUU: tty_code_code = 20;
pub const TTYC_SETAB: tty_code_code = 189;
pub const TTYC_KF32: tty_code_code = 93;
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
pub const TTYC_KF3: tty_code_code = 90;
pub const TTYC_KUP6: tty_code_code = 178;
pub const TTYC_KCUB1: tty_code_code = 44;
pub const TTYC_KNP: tty_code_code = 153;
pub const TTYC_KDN7: tty_code_code = 60;
pub const TTY_VT102: unnamed_10 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const TTYC_KF4: tty_code_code = 101;
pub const TTYC_CLEAR: tty_code_code = 7;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const LINE_SEL_NONE: unnamed_1 = 0;
pub const TTYC_KF35: tty_code_code = 96;
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
    pub message_log: unnamed_2,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_30,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_12,
}
pub const PROMPT_ENTRY: unnamed_30 = 0;
pub const TTYC_KMOUS: tty_code_code = 152;
pub const TTYC_MS: tty_code_code = 180;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const TTYC_SMACS: tty_code_code = 195;
pub const TTYC_KNXT3: tty_code_code = 155;
pub type uint16_t = libc::c_ushort;
pub const TTYC_KDN3: tty_code_code = 56;
pub const TTYC_COLORS: tty_code_code = 9;
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
pub const PROMPT_COMMAND: unnamed_30 = 1;
pub const TTYC_SGR0: tty_code_code = 193;
pub const TTYC_KLFT4: tty_code_code = 148;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_19,
}
pub const TTYC_INVIS: tty_code_code = 42;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const TTYC_HOME: tty_code_code = 35;
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
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_29 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const TTYC_KEND: tty_code_code = 61;
pub const TTYC_KF45: tty_code_code = 107;
pub const TTYC_KIC7: tty_code_code = 143;
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
pub const JOB_RUNNING: unnamed_34 = 0;
pub const TTYC_ICH: tty_code_code = 37;
pub const TTYC_KEND3: tty_code_code = 63;
pub const TTYC_KLFT5: tty_code_code = 149;
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
    pub entry: unnamed_22,
}
pub const TTYC_KF17: tty_code_code = 76;
pub const TTYC_CUU1: tty_code_code = 21;
pub const TTYC_KUP2: tty_code_code = 174;
pub const TTY_UNKNOWN: unnamed_10 = 6;
pub const TTYC_CUD: tty_code_code = 15;
pub type unnamed_30 = libc::c_uint;
pub type u_char = __u_char;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const TTYC_SMCUP: tty_code_code = 196;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
pub const TTYC_KPRV6: tty_code_code = 165;
pub const TTYC_CNORM: tty_code_code = 8;
pub const TTYC_KF2: tty_code_code = 79;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_13,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub type speed_t = libc::c_uint;
pub const TTYC_RMCUP: tty_code_code = 186;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const TTYC_KF18: tty_code_code = 77;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTYC_KNXT7: tty_code_code = 159;
pub const TTYC_KF59: tty_code_code = 122;
pub const TTYC_KEND2: tty_code_code = 62;
pub const TTYC_KDC7: tty_code_code = 53;
pub const LINE_SEL_RIGHT_LEFT: unnamed_1 = 2;
pub type bitstr_t = libc::c_uchar;
pub const TTYC_SITM: tty_code_code = 194;
pub const TTYC_SMKX: tty_code_code = 197;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_34,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub const TTYC_SETRGBF: tty_code_code = 192;
pub const LINE_SEL_LEFT_RIGHT: unnamed_1 = 1;
pub const TTYC_BCE: tty_code_code = 2;
pub const TTYC_KLFT2: tty_code_code = 146;
pub const TTYC_XENL: tty_code_code = 206;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type unnamed_34 = libc::c_uint;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_1,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub const TTYC_KF33: tty_code_code = 94;
pub const TTYC_KF50: tty_code_code = 113;
pub type __u_char = libc::c_uchar;
pub const TTYC_KRIT4: tty_code_code = 170;
pub const TTYC_KF37: tty_code_code = 98;
pub const TTYC_KF22: tty_code_code = 82;
pub const TTYC_KIC2: tty_code_code = 138;
pub const TTYC_KF58: tty_code_code = 121;
pub type __suseconds_t = libc::c_long;
pub const TTYC_RI: tty_code_code = 184;
pub const TTYC_CS: tty_code_code = 11;
pub const TTYC_U8: tty_code_code = 204;
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
    pub alerts_entry: unnamed_17,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_36,
    pub entry: unnamed_24,
}
pub const TTYC_DL: tty_code_code = 26;
pub const TTYC_KF20: tty_code_code = 80;
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const TTYC_KDN4: tty_code_code = 57;
pub const TTYC_KUP7: tty_code_code = 179;
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
pub struct unnamed_36 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const TTYC_KPP: tty_code_code = 160;
pub const TTYC_FSL: tty_code_code = 34;
pub const TTYC_HPA: tty_code_code = 36;
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const TTYC_SMXX: tty_code_code = 200;
pub const TTYC_KUP3: tty_code_code = 175;
pub const TTYC_KF36: tty_code_code = 97;
pub const TTYC_AX: tty_code_code = 0;
pub const TTYC_KLFT3: tty_code_code = 147;
pub const TTYC_KF53: tty_code_code = 116;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub const TTYC_IL1: tty_code_code = 40;
pub const TTYC_DCH1: tty_code_code = 24;
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
pub const TTYC_ICH1: tty_code_code = 38;
pub type uint32_t = libc::c_uint;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type __pid_t = libc::c_int;
pub const TTYC_KF14: tty_code_code = 73;
pub const CMDQ_CALLBACK: cmdq_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_29,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const TTYC_KPRV3: tty_code_code = 162;
pub type u_int = __u_int;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[no_mangle]
pub unsafe extern "C" fn alerts_reset_all() -> () {
    let mut w: *mut window = 0 as *mut window;
    w = windows_RB_MINMAX(&mut windows as *mut windows, 1i32.wrapping_neg());
    while w != 0 as *mut libc::c_void as *mut window {
        alerts_reset(w);
        w = windows_RB_NEXT(w)
    };
}
unsafe extern "C" fn alerts_reset(mut w: *mut window) -> () {
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    if 0 == event_initialized(&mut (*w).alerts_timer as *mut event) {
        event_set(&mut (*w).alerts_timer as *mut event, 1i32.wrapping_neg(),
                  0i32 as libc::c_short, Some(alerts_timer),
                  w as *mut libc::c_void);
    }
    (*w).flags &= !8i32;
    event_del(&mut (*w).alerts_timer as *mut event);
    let ref mut fresh0 = (*(&mut tv as *mut timeval)).tv_usec;
    *fresh0 = 0i32 as __suseconds_t;
    (*(&mut tv as *mut timeval)).tv_sec = *fresh0;
    tv.tv_sec =
        options_get_number((*w).options,
                           b"monitor-silence\x00" as *const u8 as
                               *const libc::c_char) as __time_t;
    log_debug(b"@%u alerts timer reset %u\x00" as *const u8 as
                  *const libc::c_char, (*w).id, tv.tv_sec as u_int);
    if tv.tv_sec != 0i32 as libc::c_long {
        event_add(&mut (*w).alerts_timer as *mut event,
                  &mut tv as *mut timeval);
    };
}
unsafe extern "C" fn alerts_timer(mut fd: libc::c_int,
                                  mut events: libc::c_short,
                                  mut arg: *mut libc::c_void) -> () {
    let mut w: *mut window = arg as *mut window;
    log_debug(b"@%u alerts timer expired\x00" as *const u8 as
                  *const libc::c_char, (*w).id);
    alerts_queue(w, 8i32);
}
#[no_mangle]
pub unsafe extern "C" fn alerts_queue(mut w: *mut window,
                                      mut flags: libc::c_int) -> () {
    alerts_reset(w);
    if (*w).flags & flags != flags {
        (*w).flags |= flags;
        log_debug(b"@%u alerts flags added %#x\x00" as *const u8 as
                      *const libc::c_char, (*w).id, flags);
    }
    if 0 != alerts_enabled(w, flags) {
        if 0 == (*w).alerts_queued {
            (*w).alerts_queued = 1i32;
            loop  {
                (*w).alerts_entry.tqe_next = 0 as *mut window;
                (*w).alerts_entry.tqe_prev =
                    (*(&mut alerts_list as *mut unnamed_11)).tqh_last;
                let ref mut fresh1 =
                    *(*(&mut alerts_list as *mut unnamed_11)).tqh_last;
                *fresh1 = w;
                let ref mut fresh2 =
                    (*(&mut alerts_list as *mut unnamed_11)).tqh_last;
                *fresh2 = &mut (*w).alerts_entry.tqe_next as *mut *mut window;
                if !(0 != 0i32) { break ; }
            }
            window_add_ref(w,
                           (*::std::mem::transmute::<&[u8; 13],
                                                     &[libc::c_char; 13]>(b"alerts_queue\x00")).as_ptr());
        }
        if 0 == alerts_fired {
            log_debug(b"alerts check queued (by @%u)\x00" as *const u8 as
                          *const libc::c_char, (*w).id);
            event_once(1i32.wrapping_neg(), 1i32 as libc::c_short,
                       Some(alerts_callback), 0 as *mut libc::c_void,
                       0 as *const timeval);
            alerts_fired = 1i32
        }
    };
}
static mut alerts_fired: libc::c_int = unsafe { 0 };
unsafe extern "C" fn alerts_callback(mut fd: libc::c_int,
                                     mut events: libc::c_short,
                                     mut arg: *mut libc::c_void) -> () {
    let mut w: *mut window = 0 as *mut window;
    let mut w1: *mut window = 0 as *mut window;
    let mut alerts: libc::c_int = 0;
    w = (*(&mut alerts_list as *mut unnamed_11)).tqh_first;
    while w != 0 as *mut libc::c_void as *mut window &&
              { w1 = (*w).alerts_entry.tqe_next; 0 != 1i32 } {
        alerts = alerts_check_all(w);
        log_debug(b"@%u alerts check, alerts %#x\x00" as *const u8 as
                      *const libc::c_char, (*w).id, alerts);
        (*w).alerts_queued = 0i32;
        loop  {
            if (*w).alerts_entry.tqe_next !=
                   0 as *mut libc::c_void as *mut window {
                (*(*w).alerts_entry.tqe_next).alerts_entry.tqe_prev =
                    (*w).alerts_entry.tqe_prev
            } else {
                let ref mut fresh3 =
                    (*(&mut alerts_list as *mut unnamed_11)).tqh_last;
                *fresh3 = (*w).alerts_entry.tqe_prev
            }
            *(*w).alerts_entry.tqe_prev = (*w).alerts_entry.tqe_next;
            if !(0 != 0i32) { break ; }
        }
        (*w).flags &= !(1i32 | 2i32 | 8i32);
        window_remove_ref(w,
                          (*::std::mem::transmute::<&[u8; 16],
                                                    &[libc::c_char; 16]>(b"alerts_callback\x00")).as_ptr());
        w = w1
    }
    alerts_fired = 0i32;
}
static mut alerts_list: unnamed_11 =
    unsafe {
        unnamed_11{tqh_first: 0 as *const window as *mut window,
                   tqh_last:
                       &alerts_list.tqh_first as *const *mut window as
                           *mut *mut window,}
    };
unsafe extern "C" fn alerts_check_all(mut w: *mut window) -> libc::c_int {
    let mut alerts: libc::c_int = 0;
    alerts = alerts_check_bell(w);
    alerts |= alerts_check_activity(w);
    alerts |= alerts_check_silence(w);
    return alerts;
}
unsafe extern "C" fn alerts_check_silence(mut w: *mut window) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut s: *mut session = 0 as *mut session;
    if 0 != !(*w).flags & 8i32 {
        return 0i32
    } else if options_get_number((*w).options,
                                 b"monitor-silence\x00" as *const u8 as
                                     *const libc::c_char) ==
                  0i32 as libc::c_longlong {
        return 0i32
    } else {
        wl = (*(&mut (*w).winlinks as *mut unnamed_36)).tqh_first;
        while wl != 0 as *mut libc::c_void as *mut winlink {
            (*(*wl).session).flags &= !4i32;
            wl = (*wl).wentry.tqe_next
        }
        wl = (*(&mut (*w).winlinks as *mut unnamed_36)).tqh_first;
        while wl != 0 as *mut libc::c_void as *mut winlink {
            if !(0 != (*wl).flags & 4i32) {
                s = (*wl).session;
                if (*s).curw != wl {
                    (*wl).flags |= 4i32;
                    server_status_session(s);
                }
                if !(0 ==
                         alerts_action_applies(wl,
                                               b"silence-action\x00" as
                                                   *const u8 as
                                                   *const libc::c_char)) {
                    notify_winlink(b"alert-silence\x00" as *const u8 as
                                       *const libc::c_char, wl);
                    if !(0 != (*s).flags & 4i32) {
                        (*s).flags |= 4i32;
                        alerts_set_message(wl,
                                           b"Silence\x00" as *const u8 as
                                               *const libc::c_char,
                                           b"visual-silence\x00" as *const u8
                                               as *const libc::c_char);
                    }
                }
            }
            wl = (*wl).wentry.tqe_next
        }
        return 8i32
    };
}
unsafe extern "C" fn alerts_set_message(mut wl: *mut winlink,
                                        mut type_0: *const libc::c_char,
                                        mut option: *const libc::c_char)
 -> () {
    let mut c: *mut client = 0 as *mut client;
    let mut visual: libc::c_int = 0;
    visual =
        options_get_number((*(*wl).session).options, option) as libc::c_int;
    c = (*(&mut clients as *mut clients)).tqh_first;
    while c != 0 as *mut libc::c_void as *mut client {
        if !((*c).session != (*wl).session || 0 != (*c).flags & 8192i32) {
            if visual == 0i32 || visual == 2i32 {
                tty_putcode(&mut (*c).tty as *mut tty, TTYC_BEL);
            }
            if !(visual == 0i32) {
                if (*(*c).session).curw == wl {
                    status_message_set(c,
                                       b"%s in current window\x00" as
                                           *const u8 as *const libc::c_char,
                                       type_0);
                } else {
                    status_message_set(c,
                                       b"%s in window %d\x00" as *const u8 as
                                           *const libc::c_char, type_0,
                                       (*wl).idx);
                }
            }
        }
        c = (*c).entry.tqe_next
    };
}
unsafe extern "C" fn alerts_action_applies(mut wl: *mut winlink,
                                           mut name: *const libc::c_char)
 -> libc::c_int {
    let mut action: libc::c_int = 0;
    action =
        options_get_number((*(*wl).session).options, name) as libc::c_int;
    if action == 1i32 {
        return 1i32
    } else if action == 2i32 {
        return (wl == (*(*wl).session).curw) as libc::c_int
    } else if action == 3i32 {
        return (wl != (*(*wl).session).curw) as libc::c_int
    } else { return 0i32 };
}
unsafe extern "C" fn alerts_check_activity(mut w: *mut window)
 -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut s: *mut session = 0 as *mut session;
    if 0 != !(*w).flags & 2i32 {
        return 0i32
    } else if 0 ==
                  options_get_number((*w).options,
                                     b"monitor-activity\x00" as *const u8 as
                                         *const libc::c_char) {
        return 0i32
    } else {
        wl = (*(&mut (*w).winlinks as *mut unnamed_36)).tqh_first;
        while wl != 0 as *mut libc::c_void as *mut winlink {
            (*(*wl).session).flags &= !4i32;
            wl = (*wl).wentry.tqe_next
        }
        wl = (*(&mut (*w).winlinks as *mut unnamed_36)).tqh_first;
        while wl != 0 as *mut libc::c_void as *mut winlink {
            if !(0 != (*wl).flags & 2i32) {
                s = (*wl).session;
                if (*s).curw != wl {
                    (*wl).flags |= 2i32;
                    server_status_session(s);
                }
                if !(0 ==
                         alerts_action_applies(wl,
                                               b"activity-action\x00" as
                                                   *const u8 as
                                                   *const libc::c_char)) {
                    notify_winlink(b"alert-activity\x00" as *const u8 as
                                       *const libc::c_char, wl);
                    if !(0 != (*s).flags & 4i32) {
                        (*s).flags |= 4i32;
                        alerts_set_message(wl,
                                           b"Activity\x00" as *const u8 as
                                               *const libc::c_char,
                                           b"visual-activity\x00" as *const u8
                                               as *const libc::c_char);
                    }
                }
            }
            wl = (*wl).wentry.tqe_next
        }
        return 2i32
    };
}
unsafe extern "C" fn alerts_check_bell(mut w: *mut window) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut s: *mut session = 0 as *mut session;
    if 0 != !(*w).flags & 1i32 {
        return 0i32
    } else if 0 ==
                  options_get_number((*w).options,
                                     b"monitor-bell\x00" as *const u8 as
                                         *const libc::c_char) {
        return 0i32
    } else {
        wl = (*(&mut (*w).winlinks as *mut unnamed_36)).tqh_first;
        while wl != 0 as *mut libc::c_void as *mut winlink {
            (*(*wl).session).flags &= !4i32;
            wl = (*wl).wentry.tqe_next
        }
        wl = (*(&mut (*w).winlinks as *mut unnamed_36)).tqh_first;
        while wl != 0 as *mut libc::c_void as *mut winlink {
            s = (*wl).session;
            if (*s).curw != wl {
                (*wl).flags |= 1i32;
                server_status_session(s);
            }
            if !(0 ==
                     alerts_action_applies(wl,
                                           b"bell-action\x00" as *const u8 as
                                               *const libc::c_char)) {
                notify_winlink(b"alert-bell\x00" as *const u8 as
                                   *const libc::c_char, wl);
                if !(0 != (*s).flags & 4i32) {
                    (*s).flags |= 4i32;
                    alerts_set_message(wl,
                                       b"Bell\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"visual-bell\x00" as *const u8 as
                                           *const libc::c_char);
                }
            }
            wl = (*wl).wentry.tqe_next
        }
        return 1i32
    };
}
unsafe extern "C" fn alerts_enabled(mut w: *mut window,
                                    mut flags: libc::c_int) -> libc::c_int {
    if 0 != flags & 1i32 {
        if 0 !=
               options_get_number((*w).options,
                                  b"monitor-bell\x00" as *const u8 as
                                      *const libc::c_char) {
            return 1i32
        }
    }
    if 0 != flags & 2i32 {
        if 0 !=
               options_get_number((*w).options,
                                  b"monitor-activity\x00" as *const u8 as
                                      *const libc::c_char) {
            return 1i32
        }
    }
    if 0 != flags & 8i32 {
        if options_get_number((*w).options,
                              b"monitor-silence\x00" as *const u8 as
                                  *const libc::c_char) !=
               0i32 as libc::c_longlong {
            return 1i32
        }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn alerts_check_session(mut s: *mut session) -> () {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl =
        winlinks_RB_MINMAX(&mut (*s).windows as *mut winlinks,
                           1i32.wrapping_neg());
    while wl != 0 as *mut libc::c_void as *mut winlink {
        alerts_check_all((*wl).window);
        wl = winlinks_RB_NEXT(wl)
    };
}

