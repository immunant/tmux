extern crate libc;
extern "C" {
    pub type bufferevent_ops;
    pub type screen_titles;
    pub type format_tree;
    pub type event_base;
    pub type hooks;
    pub type tty_code;
    pub type evbuffer;
    pub type tmuxproc;
    pub type environ;
    pub type tmuxpeer;
    pub type args_entry;
    pub type input_ctx;
    pub type format_job_tree;
    pub type _IO_FILE_plus;
    pub type options;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    static mut _sys_nerr: libc::c_int;
    #[no_mangle]
    static _sys_errlist: [*const libc::c_char; 0];
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
    fn utf8_combine(_: *const utf8_data, _: *mut wchar_t) -> utf8_state;
    #[no_mangle]
    fn utf8_append(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn utf8_open(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn utf8_split(_: wchar_t, _: *mut utf8_data) -> utf8_state;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub type unnamed_0 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const KEYC_MOUSEDOWN3_BORDER: unnamed_0 = 268435473;
pub const KEYC_DOUBLECLICK1_STATUS: unnamed_0 = 268435508;
pub const KEYC_KP_MINUS: unnamed_0 = 268435551;
pub const KEYC_MOUSEDRAGEND1_PANE: unnamed_0 = 268435492;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const CMDQ_COMMAND: cmdq_type = 0;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
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
    pub entry: unnamed_1,
    pub wentry: unnamed_26,
    pub sentry: unnamed_40,
}
pub const KEYC_PPAGE: unnamed_0 = 268435543;
pub type layout_type = libc::c_uint;
pub type tcflag_t = libc::c_uint;
pub const KEYC_RIGHT: unnamed_0 = 268435548;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const KEYC_F5: unnamed_0 = 268435530;
pub const KEYC_WHEELUP_STATUS: unnamed_0 = 268435502;
pub const KEYC_MOUSEMOVE_BORDER: unnamed_0 = 268435464;
pub const KEYC_MOUSEDRAG1_STATUS: unnamed_0 = 268435484;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const KEYC_WHEELUP_PANE: unnamed_0 = 268435501;
pub const TTY_UNKNOWN: unnamed_33 = 6;
pub const KEYC_MOUSEDOWN1_BORDER: unnamed_0 = 268435467;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type uint8_t = libc::c_uchar;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub type cmdq_type = libc::c_uint;
pub const KEYC_MOUSEDOWN2_BORDER: unnamed_0 = 268435470;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_34,
}
pub const KEYC_F8: unnamed_0 = 268435533;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
pub const PROMPT_COMMAND: unnamed_4 = 1;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const KEYC_KP_SEVEN: unnamed_0 = 268435552;
pub const KEYC_TRIPLECLICK2_BORDER: unnamed_0 = 268435521;
pub type __u_int = libc::c_uint;
pub const KEYC_KP_SLASH: unnamed_0 = 268435549;
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
    pub entry: unnamed_17,
}
pub const PROMPT_ENTRY: unnamed_4 = 0;
pub const LINE_SEL_NONE: unnamed_12 = 0;
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
pub type __u_char = libc::c_uchar;
pub const KEYC_DOUBLECLICK3_STATUS: unnamed_0 = 268435514;
pub const KEYC_MOUSEUP1_STATUS: unnamed_0 = 268435475;
pub type unnamed_4 = libc::c_uint;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub const KEYC_KP_EIGHT: unnamed_0 = 268435553;
pub const KEYC_MOUSEUP1_PANE: unnamed_0 = 268435474;
pub const KEYC_MOUSEDRAG2_STATUS: unnamed_0 = 268435487;
pub const LINE_SEL_LEFT_RIGHT: unnamed_12 = 1;
pub type _IO_lock_t = ();
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const JOB_CLOSED: unnamed_27 = 2;
pub const KEYC_MOUSEDRAGEND1_BORDER: unnamed_0 = 268435494;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
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
pub type utf8_state = libc::c_uint;
pub const KEYC_MOUSEDRAG1_BORDER: unnamed_0 = 268435485;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const KEYC_MOUSEDRAG3_PANE: unnamed_0 = 268435489;
pub const KEYC_MOUSEDRAG2_PANE: unnamed_0 = 268435486;
pub const KEYC_MOUSEUP2_STATUS: unnamed_0 = 268435478;
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
    pub entry: unnamed_22,
}
pub const KEYC_F10: unnamed_0 = 268435535;
pub type options_table_scope = libc::c_uint;
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
pub const KEYC_MOUSEUP2_BORDER: unnamed_0 = 268435479;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub type cc_t = libc::c_uchar;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const KEYC_F6: unnamed_0 = 268435531;
pub const KEYC_LEFT: unnamed_0 = 268435547;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_7 {
    ev_io: unnamed_20,
    ev_signal: unnamed_32,
}
pub type u_short = __u_short;
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
    pub entry: unnamed_14,
    pub tree_entry: unnamed_24,
}
pub const KEYC_UP: unnamed_0 = 268435545;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_31,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type key_code = libc::c_ulonglong;
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
pub const JOB_DEAD: unnamed_27 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const KEYC_MOUSEUP3_STATUS: unnamed_0 = 268435481;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const KEYC_DOUBLECLICK2_STATUS: unnamed_0 = 268435511;
pub const KEYC_MOUSEUP3_BORDER: unnamed_0 = 268435482;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_11 {
    ev_next_with_common_timeout: unnamed_28,
    min_heap_idx: libc::c_int,
}
pub const UTF8_MORE: utf8_state = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_2,
    pub entry: unnamed_25,
}
pub const KEYC_TRIPLECLICK3_PANE: unnamed_0 = 268435522;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const KEYC_TRIPLECLICK1_STATUS: unnamed_0 = 268435517;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const KEYC_MOUSEUP2_PANE: unnamed_0 = 268435477;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type unnamed_12 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const KEYC_KP_THREE: unnamed_0 = 268435561;
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
    pub term_type: unnamed_33,
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
pub struct windows {
    pub rbh_root: *mut window,
}
pub const KEYC_MOUSEDOWN1_STATUS: unnamed_0 = 268435466;
pub const KEYC_MOUSEDRAGEND1_STATUS: unnamed_0 = 268435493;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const KEYC_F9: unnamed_0 = 268435534;
pub const KEYC_MOUSEDRAGEND2_PANE: unnamed_0 = 268435495;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const KEYC_KP_FIVE: unnamed_0 = 268435557;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const TTY_VT420: unnamed_33 = 5;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub const KEYC_KP_TWO: unnamed_0 = 268435560;
pub const UTF8_ERROR: utf8_state = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const KEYC_MOUSEDRAGEND3_BORDER: unnamed_0 = 268435500;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
pub const KEYC_DOUBLECLICK2_PANE: unnamed_0 = 268435510;
pub const KEYC_DRAGGING: unnamed_0 = 268435461;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_10,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub type __time_t = libc::c_long;
pub const KEYC_KP_NINE: unnamed_0 = 268435554;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub ev_io_next: unnamed_8,
    pub ev_timeout: timeval,
}
pub const KEYC_KP_ZERO: unnamed_0 = 268435563;
pub const KEYC_MOUSEDRAGEND3_PANE: unnamed_0 = 268435498;
pub type u_int = __u_int;
pub const KEYC_F11: unnamed_0 = 268435536;
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
    pub message_log: unnamed_39,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_4,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_36,
}
pub const KEYC_MOUSEMOVE_STATUS: unnamed_0 = 268435463;
pub const JOB_RUNNING: unnamed_27 = 0;
pub const KEYC_F1: unnamed_0 = 268435526;
pub const UTF8_DONE: utf8_state = 1;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
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
    pub alerts_entry: unnamed_30,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_29,
    pub entry: unnamed_21,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
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
pub const KEYC_TRIPLECLICK2_PANE: unnamed_0 = 268435519;
pub const KEYC_DOUBLECLICK1_BORDER: unnamed_0 = 268435509;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub type uint16_t = libc::c_ushort;
pub const KEYC_MOUSEDRAG1_PANE: unnamed_0 = 268435483;
pub const KEYC_FOCUS_IN: unnamed_0 = 268435456;
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const KEYC_DC: unnamed_0 = 268435539;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const KEYC_TRIPLECLICK3_STATUS: unnamed_0 = 268435523;
pub const TTY_VT100: unnamed_33 = 0;
pub const KEYC_FOCUS_OUT: unnamed_0 = 268435457;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const KEYC_KP_ENTER: unnamed_0 = 268435562;
pub const KEYC_MOUSEDRAG3_BORDER: unnamed_0 = 268435491;
pub const KEYC_MOUSEUP1_BORDER: unnamed_0 = 268435476;
pub const KEYC_BTAB: unnamed_0 = 268435544;
pub const KEYC_F12: unnamed_0 = 268435537;
pub type options_table_type = libc::c_uint;
pub const KEYC_WHEELUP_BORDER: unnamed_0 = 268435503;
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
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub const KEYC_MOUSEDRAG3_STATUS: unnamed_0 = 268435490;
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
pub struct unnamed_24 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const KEYC_TRIPLECLICK1_BORDER: unnamed_0 = 268435518;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
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
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const KEYC_MOUSE: unnamed_0 = 268435460;
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
    pub gentry: unnamed_23,
    pub entry: unnamed_16,
}
pub const KEYC_F4: unnamed_0 = 268435529;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const KEYC_WHEELDOWN_STATUS: unnamed_0 = 268435505;
pub const KEYC_F2: unnamed_0 = 268435527;
pub const KEYC_MOUSEDRAG2_BORDER: unnamed_0 = 268435488;
pub const KEYC_END: unnamed_0 = 268435541;
pub const KEYC_DOUBLECLICK2_BORDER: unnamed_0 = 268435512;
pub const KEYC_MOUSEDOWN2_PANE: unnamed_0 = 268435468;
pub type unnamed_27 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_9,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub type __u_short = libc::c_ushort;
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const KEYC_MOUSEMOVE_PANE: unnamed_0 = 268435462;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const KEYC_KP_FOUR: unnamed_0 = 268435556;
pub const KEYC_KP_SIX: unnamed_0 = 268435558;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_31 {
    offset: u_int,
    data: unnamed_37,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_12 = 2;
pub const KEYC_BSPACE: unnamed_0 = 268435525;
pub type uint32_t = libc::c_uint;
pub const TTY_VT220: unnamed_33 = 3;
pub const KEYC_PASTE_END: unnamed_0 = 268435459;
pub type __off64_t = libc::c_long;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const KEYC_KP_STAR: unnamed_0 = 268435550;
pub type __suseconds_t = libc::c_long;
pub const TTY_VT320: unnamed_33 = 4;
pub type pid_t = __pid_t;
pub const KEYC_F3: unnamed_0 = 268435528;
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const KEYC_KP_PERIOD: unnamed_0 = 268435564;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_19,
}
pub const KEYC_TRIPLECLICK3_BORDER: unnamed_0 = 268435524;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_18,
}
pub const TTY_VT101: unnamed_33 = 1;
pub const KEYC_MOUSEDRAGEND2_BORDER: unnamed_0 = 268435497;
pub const KEYC_MOUSEDOWN1_PANE: unnamed_0 = 268435465;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub ev_signal_next: unnamed_3,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_15,
    pub ev_next: unnamed_13,
    pub ev_timeout_pos: unnamed_11,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_7,
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
pub const KEYC_TRIPLECLICK2_STATUS: unnamed_0 = 268435520;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
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
pub const KEYC_WHEELDOWN_PANE: unnamed_0 = 268435504;
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_27,
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
    pub entry: unnamed_5,
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
pub type unnamed_33 = libc::c_uint;
pub const KEYC_KP_ONE: unnamed_0 = 268435559;
pub const KEYC_PASTE_START: unnamed_0 = 268435458;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub type cmd_retval = libc::c_int;
pub const KEYC_NPAGE: unnamed_0 = 268435542;
pub const KEYC_WHEELDOWN_BORDER: unnamed_0 = 268435506;
pub const KEYC_F7: unnamed_0 = 268435532;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
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
pub const KEYC_DOUBLECLICK3_PANE: unnamed_0 = 268435513;
pub type __pid_t = libc::c_int;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_34 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
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
pub const KEYC_MOUSEDOWN3_STATUS: unnamed_0 = 268435472;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub string: *const libc::c_char,
    pub key: key_code,
}
pub const KEYC_MOUSEDRAGEND2_STATUS: unnamed_0 = 268435496;
pub const TTY_VT102: unnamed_33 = 2;
pub const KEYC_IC: unnamed_0 = 268435538;
pub type u_char = __u_char;
pub const KEYC_DOUBLECLICK1_PANE: unnamed_0 = 268435507;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub const KEYC_KP_PLUS: unnamed_0 = 268435555;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_38,
}
pub const KEYC_MOUSEUP3_PANE: unnamed_0 = 268435480;
pub const KEYC_MOUSEDOWN3_PANE: unnamed_0 = 268435471;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type size_t = libc::c_ulong;
pub const KEYC_DOUBLECLICK3_BORDER: unnamed_0 = 268435515;
pub const KEYC_MOUSEDOWN2_STATUS: unnamed_0 = 268435469;
pub const KEYC_HOME: unnamed_0 = 268435540;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type wchar_t = libc::c_int;
pub const KEYC_DOWN: unnamed_0 = 268435546;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const KEYC_MOUSEDRAGEND3_STATUS: unnamed_0 = 268435499;
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
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub const KEYC_TRIPLECLICK1_PANE: unnamed_0 = 268435516;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_40 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[no_mangle]
pub unsafe extern "C" fn key_string_lookup_string(mut string:
                                                      *const libc::c_char)
 -> key_code {
    static mut other: *const libc::c_char =
        unsafe {
            b"!#()+,-.0123456789:;<=>?\'\r\t\x00" as *const u8 as
                *const libc::c_char
        };
    let mut key: key_code = 0;
    let mut u: u_int = 0;
    let mut modifiers: key_code = 0;
    let mut ud: utf8_data =
        utf8_data{data: [0; 9], have: 0, size: 0, width: 0,};
    let mut i: u_int = 0;
    let mut more: utf8_state = UTF8_MORE;
    let mut wc: wchar_t = 0;
    if strcasecmp(string, b"None\x00" as *const u8 as *const libc::c_char) ==
           0i32 {
        return 281470681743360u64
    } else if *string.offset(0isize) as libc::c_int == 48 &&
                  *string.offset(1isize) as libc::c_int == 120 {
        if sscanf(string.offset(2isize),
                  b"%x\x00" as *const u8 as *const libc::c_char,
                  &mut u as *mut u_int) != 1i32 {
            return 281466386776064u64
        } else if u > 2097151i32 as libc::c_uint {
            return 281466386776064u64
        } else { return u as key_code }
    } else {
        modifiers = 0i32 as key_code;
        if *string.offset(0isize) as libc::c_int == 94 &&
               *string.offset(1isize) as libc::c_int != 0 {
            modifiers |= 70368744177664u64;
            string = string.offset(1isize)
        }
        modifiers |=
            key_string_get_modifiers(&mut string as *mut *const libc::c_char);
        if string == 0 as *mut libc::c_void as *const libc::c_char ||
               *string.offset(0isize) as libc::c_int == 0 {
            return 281466386776064u64
        } else {
            if *string.offset(1isize) as libc::c_int == 0 &&
                   *string.offset(0isize) as u_char as libc::c_int <= 127i32 {
                key = *string.offset(0isize) as u_char as key_code;
                if key < 32i32 as libc::c_ulonglong ||
                       key == 127i32 as libc::c_ulonglong {
                    return 281466386776064u64
                }
            } else {
                more =
                    utf8_open(&mut ud as *mut utf8_data, *string as u_char);
                if more as libc::c_uint ==
                       UTF8_MORE as libc::c_int as libc::c_uint {
                    if strlen(string) != ud.size as libc::c_ulong {
                        return 281466386776064u64
                    } else {
                        i = 1i32 as u_int;
                        while i < ud.size as libc::c_uint {
                            more =
                                utf8_append(&mut ud as *mut utf8_data,
                                            *string.offset(i as isize) as
                                                u_char);
                            i = i.wrapping_add(1)
                        }
                        if more as libc::c_uint !=
                               UTF8_DONE as libc::c_int as libc::c_uint {
                            return 281466386776064u64
                        } else if utf8_combine(&mut ud as *mut utf8_data,
                                               &mut wc as *mut wchar_t) as
                                      libc::c_uint !=
                                      UTF8_DONE as libc::c_int as libc::c_uint
                         {
                            return 281466386776064u64
                        } else { return wc as libc::c_ulonglong | modifiers }
                    }
                } else {
                    key = key_string_search_table(string);
                    if key == 281466386776064u64 { return 281466386776064u64 }
                }
            }
            if key < 268435456u64 && 0 != modifiers & 70368744177664u64 &&
                   0 !=
                       strchr(other, key as libc::c_int).is_null() as
                           libc::c_int {
                if key >= 97i32 as libc::c_ulonglong &&
                       key <= 122i32 as libc::c_ulonglong {
                    key =
                        (key as
                             libc::c_ulonglong).wrapping_sub(96i32 as
                                                                 libc::c_ulonglong)
                            as key_code as key_code
                } else if key >= 64i32 as libc::c_ulonglong &&
                              key <= 95i32 as libc::c_ulonglong {
                    key =
                        (key as
                             libc::c_ulonglong).wrapping_sub(64i32 as
                                                                 libc::c_ulonglong)
                            as key_code as key_code
                } else if key == 32i32 as libc::c_ulonglong {
                    key = 0i32 as key_code
                } else if key == 63i32 as libc::c_ulonglong {
                    key = KEYC_BSPACE as libc::c_int as key_code
                } else { return 281466386776064u64 }
                modifiers &= !70368744177664u64
            }
            return key | modifiers
        }
    };
}
unsafe extern "C" fn key_string_search_table(mut string: *const libc::c_char)
 -> key_code {
    let mut current_block: u64;
    let mut i: u_int = 0;
    let mut user: u_int = 0;
    i = 0i32 as u_int;
    loop  {
        if (i as libc::c_ulong) <
               (::std::mem::size_of::<[unnamed_35; 108]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<unnamed_35>()
                                                    as libc::c_ulong) {
            if strcasecmp(string, key_string_table[i as usize].string) == 0i32
               {
                return key_string_table[i as usize].key
            } else { i = i.wrapping_add(1) }
        } else if sscanf(string,
                         b"User%u\x00" as *const u8 as *const libc::c_char,
                         &mut user as *mut u_int) == 1i32 &&
                      user < 1000i32 as libc::c_uint {
            current_block = 12517898123489920830;
            break ;
        } else { current_block = 14155750587950065367; break ; }
    }
    match current_block {
        12517898123489920830 => {
            return 536870912u64.wrapping_add(user as libc::c_ulonglong)
        }
        _ => { return 281466386776064u64 }
    };
}
static mut key_string_table: [unnamed_35; 108] =
    unsafe {
        [unnamed_35{string: b"F1\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_F1 as libc::c_int as key_code,},
         unnamed_35{string: b"F2\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_F2 as libc::c_int as key_code,},
         unnamed_35{string: b"F3\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_F3 as libc::c_int as key_code,},
         unnamed_35{string: b"F4\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_F4 as libc::c_int as key_code,},
         unnamed_35{string: b"F5\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_F5 as libc::c_int as key_code,},
         unnamed_35{string: b"F6\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_F6 as libc::c_int as key_code,},
         unnamed_35{string: b"F7\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_F7 as libc::c_int as key_code,},
         unnamed_35{string: b"F8\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_F8 as libc::c_int as key_code,},
         unnamed_35{string: b"F9\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_F9 as libc::c_int as key_code,},
         unnamed_35{string: b"F10\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_F10 as libc::c_int as key_code,},
         unnamed_35{string: b"F11\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_F11 as libc::c_int as key_code,},
         unnamed_35{string: b"F12\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_F12 as libc::c_int as key_code,},
         unnamed_35{string: b"IC\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_IC as libc::c_int as key_code,},
         unnamed_35{string: b"DC\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_DC as libc::c_int as key_code,},
         unnamed_35{string: b"Home\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_HOME as libc::c_int as key_code,},
         unnamed_35{string: b"End\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_END as libc::c_int as key_code,},
         unnamed_35{string: b"NPage\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_NPAGE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"PageDown\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_NPAGE as libc::c_int as key_code,},
         unnamed_35{string: b"PgDn\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_NPAGE as libc::c_int as key_code,},
         unnamed_35{string: b"PPage\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_PPAGE as libc::c_int as key_code,},
         unnamed_35{string: b"PageUp\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_PPAGE as libc::c_int as key_code,},
         unnamed_35{string: b"PgUp\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_PPAGE as libc::c_int as key_code,},
         unnamed_35{string: b"Tab\x00" as *const u8 as *const libc::c_char,
                    key: 9 as key_code,},
         unnamed_35{string: b"BTab\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_BTAB as libc::c_int as key_code,},
         unnamed_35{string: b"Space\x00" as *const u8 as *const libc::c_char,
                    key: 32 as key_code,},
         unnamed_35{string: b"BSpace\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_BSPACE as libc::c_int as key_code,},
         unnamed_35{string: b"Enter\x00" as *const u8 as *const libc::c_char,
                    key: 13 as key_code,},
         unnamed_35{string: b"Escape\x00" as *const u8 as *const libc::c_char,
                    key: 27 as key_code,},
         unnamed_35{string: b"Up\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_UP as libc::c_int as key_code,},
         unnamed_35{string: b"Down\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_DOWN as libc::c_int as key_code,},
         unnamed_35{string: b"Left\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_LEFT as libc::c_int as key_code,},
         unnamed_35{string: b"Right\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_RIGHT as libc::c_int as key_code,},
         unnamed_35{string: b"KP/\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_SLASH as libc::c_int as key_code,},
         unnamed_35{string: b"KP*\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_STAR as libc::c_int as key_code,},
         unnamed_35{string: b"KP-\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_MINUS as libc::c_int as key_code,},
         unnamed_35{string: b"KP7\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_SEVEN as libc::c_int as key_code,},
         unnamed_35{string: b"KP8\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_EIGHT as libc::c_int as key_code,},
         unnamed_35{string: b"KP9\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_NINE as libc::c_int as key_code,},
         unnamed_35{string: b"KP+\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_PLUS as libc::c_int as key_code,},
         unnamed_35{string: b"KP4\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_FOUR as libc::c_int as key_code,},
         unnamed_35{string: b"KP5\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_FIVE as libc::c_int as key_code,},
         unnamed_35{string: b"KP6\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_SIX as libc::c_int as key_code,},
         unnamed_35{string: b"KP1\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_ONE as libc::c_int as key_code,},
         unnamed_35{string: b"KP2\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_TWO as libc::c_int as key_code,},
         unnamed_35{string: b"KP3\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_THREE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"KPEnter\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_ENTER as libc::c_int as key_code,},
         unnamed_35{string: b"KP0\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_ZERO as libc::c_int as key_code,},
         unnamed_35{string: b"KP.\x00" as *const u8 as *const libc::c_char,
                    key: KEYC_KP_PERIOD as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDown1Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDOWN1_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDown1Status\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDOWN1_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDown1Border\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDOWN1_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDown2Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDOWN2_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDown2Status\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDOWN2_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDown2Border\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDOWN2_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDown3Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDOWN3_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDown3Status\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDOWN3_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDown3Border\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDOWN3_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseUp1Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEUP1_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseUp1Status\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEUP1_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseUp1Border\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEUP1_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseUp2Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEUP2_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseUp2Status\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEUP2_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseUp2Border\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEUP2_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseUp3Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEUP3_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseUp3Status\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEUP3_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseUp3Border\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEUP3_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDrag1Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDRAG1_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDrag1Status\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDRAG1_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDrag1Border\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDRAG1_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDrag2Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDRAG2_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDrag2Status\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDRAG2_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDrag2Border\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDRAG2_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDrag3Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDRAG3_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDrag3Status\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDRAG3_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDrag3Border\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDRAG3_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDragEnd1Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDRAGEND1_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDragEnd1Status\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_MOUSEDRAGEND1_STATUS as libc::c_int as
                            key_code,},
         unnamed_35{string:
                        b"MouseDragEnd1Border\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_MOUSEDRAGEND1_BORDER as libc::c_int as
                            key_code,},
         unnamed_35{string:
                        b"MouseDragEnd2Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDRAGEND2_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDragEnd2Status\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_MOUSEDRAGEND2_STATUS as libc::c_int as
                            key_code,},
         unnamed_35{string:
                        b"MouseDragEnd2Border\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_MOUSEDRAGEND2_BORDER as libc::c_int as
                            key_code,},
         unnamed_35{string:
                        b"MouseDragEnd3Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_MOUSEDRAGEND3_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"MouseDragEnd3Status\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_MOUSEDRAGEND3_STATUS as libc::c_int as
                            key_code,},
         unnamed_35{string:
                        b"MouseDragEnd3Border\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_MOUSEDRAGEND3_BORDER as libc::c_int as
                            key_code,},
         unnamed_35{string:
                        b"WheelUpPane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_WHEELUP_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"WheelUpStatus\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_WHEELUP_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"WheelUpBorder\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_WHEELUP_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"WheelDownPane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_WHEELDOWN_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"WheelDownStatus\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_WHEELDOWN_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"WheelDownBorder\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_WHEELDOWN_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"DoubleClick1Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_DOUBLECLICK1_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"DoubleClick1Status\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_DOUBLECLICK1_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"DoubleClick1Border\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_DOUBLECLICK1_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"DoubleClick2Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_DOUBLECLICK2_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"DoubleClick2Status\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_DOUBLECLICK2_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"DoubleClick2Border\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_DOUBLECLICK2_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"DoubleClick3Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_DOUBLECLICK3_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"DoubleClick3Status\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_DOUBLECLICK3_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"DoubleClick3Border\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_DOUBLECLICK3_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"TripleClick1Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_TRIPLECLICK1_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"TripleClick1Status\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_TRIPLECLICK1_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"TripleClick1Border\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_TRIPLECLICK1_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"TripleClick2Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_TRIPLECLICK2_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"TripleClick2Status\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_TRIPLECLICK2_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"TripleClick2Border\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_TRIPLECLICK2_BORDER as libc::c_int as key_code,},
         unnamed_35{string:
                        b"TripleClick3Pane\x00" as *const u8 as
                            *const libc::c_char,
                    key: KEYC_TRIPLECLICK3_PANE as libc::c_int as key_code,},
         unnamed_35{string:
                        b"TripleClick3Status\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_TRIPLECLICK3_STATUS as libc::c_int as key_code,},
         unnamed_35{string:
                        b"TripleClick3Border\x00" as *const u8 as
                            *const libc::c_char,
                    key:
                        KEYC_TRIPLECLICK3_BORDER as libc::c_int as key_code,}]
    };
unsafe extern "C" fn key_string_get_modifiers(mut string:
                                                  *mut *const libc::c_char)
 -> key_code {
    let mut modifiers: key_code = 0;
    modifiers = 0i32 as key_code;
    loop  {
        if *(*string).offset(0isize) as libc::c_int != 0 &&
               *(*string).offset(1isize) as libc::c_int == 45 {
            match *(*string).offset(0isize) as libc::c_int {
                67 | 99 => { modifiers |= 70368744177664u64 }
                77 | 109 => { modifiers |= 35184372088832u64 }
                83 | 115 => { modifiers |= 140737488355328u64 }
                _ => {
                    *string = 0 as *const libc::c_char;
                    return 0i32 as key_code
                }
            }
            *string = (*string).offset(2isize)
        } else { return modifiers }
    };
}
#[no_mangle]
pub unsafe extern "C" fn key_string_lookup_key(mut key: key_code)
 -> *const libc::c_char {
    static mut out: [libc::c_char; 32] = unsafe { [0; 32] };
    let mut tmp: [libc::c_char; 8] = [0; 8];
    let mut i: u_int = 0;
    let mut ud: utf8_data =
        utf8_data{data: [0; 9], have: 0, size: 0, width: 0,};
    let mut off: size_t = 0;
    *out.as_mut_ptr() = 0 as libc::c_char;
    if key == 281470681743360u64 {
        return b"None\x00" as *const u8 as *const libc::c_char
    } else if key == 281466386776064u64 {
        return b"Unknown\x00" as *const u8 as *const libc::c_char
    } else if key == KEYC_FOCUS_IN as libc::c_int as libc::c_ulonglong {
        return b"FocusIn\x00" as *const u8 as *const libc::c_char
    } else if key == KEYC_FOCUS_OUT as libc::c_int as libc::c_ulonglong {
        return b"FocusOut\x00" as *const u8 as *const libc::c_char
    } else if key == KEYC_PASTE_START as libc::c_int as libc::c_ulonglong {
        return b"PasteStart\x00" as *const u8 as *const libc::c_char
    } else if key == KEYC_PASTE_END as libc::c_int as libc::c_ulonglong {
        return b"PasteEnd\x00" as *const u8 as *const libc::c_char
    } else if key == KEYC_MOUSE as libc::c_int as libc::c_ulonglong {
        return b"Mouse\x00" as *const u8 as *const libc::c_char
    } else if key == KEYC_DRAGGING as libc::c_int as libc::c_ulonglong {
        return b"Dragging\x00" as *const u8 as *const libc::c_char
    } else if key == KEYC_MOUSEMOVE_PANE as libc::c_int as libc::c_ulonglong {
        return b"MouseMovePane\x00" as *const u8 as *const libc::c_char
    } else if key == KEYC_MOUSEMOVE_STATUS as libc::c_int as libc::c_ulonglong
     {
        return b"MouseMoveStatus\x00" as *const u8 as *const libc::c_char
    } else if key == KEYC_MOUSEMOVE_BORDER as libc::c_int as libc::c_ulonglong
     {
        return b"MouseMoveBorder\x00" as *const u8 as *const libc::c_char
    } else if key >= 536870912u64 &&
                  key <
                      536870912u64.wrapping_add(1000i32 as libc::c_ulonglong)
     {
        snprintf(out.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                 b"User%u\x00" as *const u8 as *const libc::c_char,
                 key.wrapping_sub(536870912u64) as u_int);
        return out.as_mut_ptr()
    } else {
        if key &
               !(35184372088832u64 | 70368744177664u64 | 140737488355328u64 |
                     281474976710656u64) == 0i32 as libc::c_ulonglong {
            key =
                32 as libc::c_ulonglong | 70368744177664u64 |
                    key &
                        (35184372088832u64 | 70368744177664u64 |
                             140737488355328u64 | 281474976710656u64)
        }
        if 0 != key & 70368744177664u64 {
            strlcat(out.as_mut_ptr(),
                    b"C-\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 32]>() as
                        libc::c_ulong);
        }
        if 0 != key & 35184372088832u64 {
            strlcat(out.as_mut_ptr(),
                    b"M-\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 32]>() as
                        libc::c_ulong);
        }
        if 0 != key & 140737488355328u64 {
            strlcat(out.as_mut_ptr(),
                    b"S-\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 32]>() as
                        libc::c_ulong);
        }
        key &=
            !(35184372088832u64 | 70368744177664u64 | 140737488355328u64 |
                  281474976710656u64);
        i = 0i32 as u_int;
        while (i as libc::c_ulong) <
                  (::std::mem::size_of::<[unnamed_35; 108]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<unnamed_35>()
                                                       as libc::c_ulong) {
            if key == key_string_table[i as usize].key { break ; }
            i = i.wrapping_add(1)
        }
        if i as libc::c_ulong !=
               (::std::mem::size_of::<[unnamed_35; 108]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<unnamed_35>()
                                                    as libc::c_ulong) {
            strlcat(out.as_mut_ptr(), key_string_table[i as usize].string,
                    ::std::mem::size_of::<[libc::c_char; 32]>() as
                        libc::c_ulong);
            return out.as_mut_ptr()
        } else {
            if key > 127i32 as libc::c_ulonglong && key < 268435456u64 {
                if utf8_split(key as wchar_t, &mut ud as *mut utf8_data) as
                       libc::c_uint ==
                       UTF8_DONE as libc::c_int as libc::c_uint {
                    off = strlen(out.as_mut_ptr());
                    memcpy(out.as_mut_ptr().offset(off as isize) as
                               *mut libc::c_void,
                           ud.data.as_mut_ptr() as *const libc::c_void,
                           ud.size as libc::c_ulong);
                    out[off.wrapping_add(ud.size as libc::c_ulong) as usize] =
                        0 as libc::c_char;
                    return out.as_mut_ptr()
                }
            }
            if key == 127i32 as libc::c_ulonglong ||
                   key > 255i32 as libc::c_ulonglong {
                snprintf(out.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 32]>() as
                             libc::c_ulong,
                         b"Invalid#%llx\x00" as *const u8 as
                             *const libc::c_char, key);
                return out.as_mut_ptr()
            } else {
                if key <= 32i32 as libc::c_ulonglong {
                    if key == 0i32 as libc::c_ulonglong ||
                           key > 26i32 as libc::c_ulonglong {
                        xsnprintf(tmp.as_mut_ptr(),
                                  ::std::mem::size_of::<[libc::c_char; 8]>()
                                      as libc::c_ulong,
                                  b"C-%c\x00" as *const u8 as
                                      *const libc::c_char,
                                  (64i32 as
                                       libc::c_ulonglong).wrapping_add(key) as
                                      libc::c_int);
                    } else {
                        xsnprintf(tmp.as_mut_ptr(),
                                  ::std::mem::size_of::<[libc::c_char; 8]>()
                                      as libc::c_ulong,
                                  b"C-%c\x00" as *const u8 as
                                      *const libc::c_char,
                                  (96i32 as
                                       libc::c_ulonglong).wrapping_add(key) as
                                      libc::c_int);
                    }
                } else if key >= 32i32 as libc::c_ulonglong &&
                              key <= 126i32 as libc::c_ulonglong {
                    tmp[0usize] = key as libc::c_char;
                    tmp[1usize] = 0 as libc::c_char
                } else if key >= 128i32 as libc::c_ulonglong {
                    xsnprintf(tmp.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 8]>() as
                                  libc::c_ulong,
                              b"\\%llo\x00" as *const u8 as
                                  *const libc::c_char, key);
                }
                strlcat(out.as_mut_ptr(), tmp.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 32]>() as
                            libc::c_ulong);
                return out.as_mut_ptr()
            }
        }
    };
}

