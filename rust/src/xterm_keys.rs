extern crate libc;
extern "C" {
    pub type options;
    pub type event_base;
    pub type format_tree;
    pub type format_job_tree;
    pub type tmuxpeer;
    pub type input_ctx;
    pub type evbuffer;
    pub type args_entry;
    pub type tty_code;
    pub type tmuxproc;
    pub type screen_titles;
    pub type environ;
    pub type _IO_FILE_plus;
    pub type hooks;
    pub type bufferevent_ops;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
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
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
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
pub const KEYC_MOUSEUP2_STATUS: unnamed_17 = 268435478;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    ev_next_with_common_timeout: unnamed_27,
    min_heap_idx: libc::c_int,
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
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type __off_t = libc::c_long;
pub const KEYC_KP_TWO: unnamed_17 = 268435560;
pub type __off64_t = libc::c_long;
pub const KEYC_KP_SEVEN: unnamed_17 = 268435552;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    ev_io: unnamed_16,
    ev_signal: unnamed_11,
}
pub const TTY_VT102: unnamed_36 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct xterm_keys_entry {
    pub key: key_code,
    pub template: *const libc::c_char,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const KEYC_IC: unnamed_17 = 268435538;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const KEYC_MOUSEDOWN2_PANE: unnamed_17 = 268435468;
pub const KEYC_DOUBLECLICK1_STATUS: unnamed_17 = 268435508;
pub const KEYC_F1: unnamed_17 = 268435526;
pub const KEYC_WHEELDOWN_STATUS: unnamed_17 = 268435505;
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
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_13,
}
pub const KEYC_MOUSEDRAG1_STATUS: unnamed_17 = 268435484;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type u_char = __u_char;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const KEYC_F8: unnamed_17 = 268435533;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const KEYC_TRIPLECLICK2_STATUS: unnamed_17 = 268435520;
pub type pid_t = __pid_t;
pub const TTY_UNKNOWN: unnamed_36 = 6;
pub const KEYC_KP_EIGHT: unnamed_17 = 268435553;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
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
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const KEYC_MOUSEDRAGEND2_STATUS: unnamed_17 = 268435496;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
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
pub const KEYC_DOUBLECLICK3_PANE: unnamed_17 = 268435513;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_10 {
    offset: u_int,
    data: unnamed_23,
}
pub const KEYC_MOUSEUP2_BORDER: unnamed_17 = 268435479;
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
    pub gentry: unnamed,
    pub entry: unnamed_18,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub ev_signal_next: unnamed_6,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
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
    pub message_log: unnamed_31,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_32,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_29,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub const KEYC_HOME: unnamed_17 = 268435540;
pub const KEYC_END: unnamed_17 = 268435541;
pub const KEYC_DOUBLECLICK3_BORDER: unnamed_17 = 268435515;
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
    pub winlinks: unnamed_7,
    pub entry: unnamed_39,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
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
pub const KEYC_MOUSEDRAGEND1_BORDER: unnamed_17 = 268435494;
pub type __u_short = libc::c_ushort;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const KEYC_MOUSEDRAGEND2_BORDER: unnamed_17 = 268435497;
pub const KEYC_KP_STAR: unnamed_17 = 268435550;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const KEYC_PASTE_START: unnamed_17 = 268435458;
pub const KEYC_MOUSE: unnamed_17 = 268435460;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const KEYC_WHEELUP_STATUS: unnamed_17 = 268435502;
pub const LINE_SEL_RIGHT_LEFT: unnamed_35 = 2;
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
pub const KEYC_TRIPLECLICK2_BORDER: unnamed_17 = 268435521;
pub const KEYC_UP: unnamed_17 = 268435545;
pub const KEYC_WHEELUP_BORDER: unnamed_17 = 268435503;
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
    pub entry: unnamed_24,
    pub tree_entry: unnamed_25,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const KEYC_MOUSEDRAG3_BORDER: unnamed_17 = 268435491;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_15 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const KEYC_MOUSEDRAGEND3_BORDER: unnamed_17 = 268435500;
pub const KEYC_MOUSEDRAG1_BORDER: unnamed_17 = 268435485;
pub const LINE_SEL_LEFT_RIGHT: unnamed_35 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_38,
}
pub const JOB_CLOSED: unnamed_34 = 2;
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
pub const KEYC_MOUSEDRAG3_PANE: unnamed_17 = 268435489;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const KEYC_TRIPLECLICK1_STATUS: unnamed_17 = 268435517;
pub const KEYC_KP_SLASH: unnamed_17 = 268435549;
pub const KEYC_BTAB: unnamed_17 = 268435544;
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
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_35,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub const KEYC_KP_FOUR: unnamed_17 = 268435556;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub ev_io_next: unnamed_1,
    pub ev_timeout: timeval,
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
pub type cmd_retval = libc::c_int;
pub const KEYC_MOUSEDRAGEND2_PANE: unnamed_17 = 268435495;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const JOB_RUNNING: unnamed_34 = 0;
pub const KEYC_DOUBLECLICK2_BORDER: unnamed_17 = 268435512;
pub const KEYC_MOUSEMOVE_BORDER: unnamed_17 = 268435464;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_15,
    pub entry: unnamed_5,
}
pub type unnamed_17 = libc::c_uint;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub type _IO_lock_t = ();
pub const KEYC_F6: unnamed_17 = 268435531;
pub const KEYC_KP_ZERO: unnamed_17 = 268435563;
pub const KEYC_MOUSEDOWN3_BORDER: unnamed_17 = 268435473;
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
pub type uint32_t = libc::c_uint;
pub const PROMPT_COMMAND: unnamed_32 = 1;
pub const TTY_VT100: unnamed_36 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_19 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
pub const KEYC_MOUSEDRAG1_PANE: unnamed_17 = 268435483;
pub const KEYC_MOUSEDOWN3_STATUS: unnamed_17 = 268435472;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const KEYC_MOUSEDRAGEND3_PANE: unnamed_17 = 268435498;
pub const KEYC_KP_THREE: unnamed_17 = 268435561;
pub const KEYC_MOUSEMOVE_STATUS: unnamed_17 = 268435463;
pub type __pid_t = libc::c_int;
pub const LINE_SEL_NONE: unnamed_35 = 0;
pub type bitstr_t = libc::c_uchar;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub const KEYC_MOUSEDRAG2_BORDER: unnamed_17 = 268435488;
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
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
pub const KEYC_FOCUS_OUT: unnamed_17 = 268435457;
pub const KEYC_KP_FIVE: unnamed_17 = 268435557;
pub type options_table_type = libc::c_uint;
pub const KEYC_KP_PLUS: unnamed_17 = 268435555;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const KEYC_F4: unnamed_17 = 268435529;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_9,
    pub ev_next: unnamed_21,
    pub ev_timeout_pos: unnamed_0,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_2,
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
pub type __u_int = libc::c_uint;
pub type key_code = libc::c_ulonglong;
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub const KEYC_FOCUS_IN: unnamed_17 = 268435456;
pub const KEYC_TRIPLECLICK3_STATUS: unnamed_17 = 268435523;
pub const KEYC_TRIPLECLICK1_PANE: unnamed_17 = 268435516;
pub const KEYC_DOUBLECLICK3_STATUS: unnamed_17 = 268435514;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_8,
}
pub const KEYC_F9: unnamed_17 = 268435534;
pub const TTY_VT101: unnamed_36 = 1;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const KEYC_TRIPLECLICK2_PANE: unnamed_17 = 268435519;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const KEYC_DOWN: unnamed_17 = 268435546;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub const KEYC_KP_NINE: unnamed_17 = 268435554;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const KEYC_F3: unnamed_17 = 268435528;
pub const KEYC_DOUBLECLICK1_PANE: unnamed_17 = 268435507;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const KEYC_PPAGE: unnamed_17 = 268435543;
pub type uint8_t = libc::c_uchar;
pub const KEYC_DRAGGING: unnamed_17 = 268435461;
pub const TTY_VT420: unnamed_36 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type options_table_scope = libc::c_uint;
pub const KEYC_MOUSEDRAG3_STATUS: unnamed_17 = 268435490;
pub const TTY_VT220: unnamed_36 = 3;
pub const KEYC_MOUSEDRAGEND3_STATUS: unnamed_17 = 268435499;
pub const KEYC_MOUSEDOWN2_STATUS: unnamed_17 = 268435469;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub type time_t = __time_t;
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
    pub term_type: unnamed_36,
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
pub type u_int = __u_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub const KEYC_TRIPLECLICK1_BORDER: unnamed_17 = 268435518;
pub const KEYC_F12: unnamed_17 = 268435537;
pub const KEYC_KP_MINUS: unnamed_17 = 268435551;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_27 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type cc_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_22,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const TTY_VT320: unnamed_36 = 4;
pub const KEYC_MOUSEDOWN2_BORDER: unnamed_17 = 268435470;
pub const KEYC_DC: unnamed_17 = 268435539;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_30,
}
pub const KEYC_LEFT: unnamed_17 = 268435547;
pub const KEYC_MOUSEUP3_STATUS: unnamed_17 = 268435481;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const KEYC_F2: unnamed_17 = 268435527;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub const KEYC_MOUSEUP2_PANE: unnamed_17 = 268435477;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_12,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub const KEYC_MOUSEUP1_STATUS: unnamed_17 = 268435475;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const KEYC_MOUSEDRAGEND1_PANE: unnamed_17 = 268435492;
pub const KEYC_MOUSEMOVE_PANE: unnamed_17 = 268435462;
pub const KEYC_KP_PERIOD: unnamed_17 = 268435564;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub type __u_char = libc::c_uchar;
pub const CMD_RETURN_STOP: cmd_retval = 2;
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
pub struct unnamed_30 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
pub type __time_t = libc::c_long;
pub const KEYC_KP_ONE: unnamed_17 = 268435559;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type uint16_t = libc::c_ushort;
pub const KEYC_F7: unnamed_17 = 268435532;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type unnamed_32 = libc::c_uint;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
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
    pub entry: unnamed_14,
}
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub type unnamed_34 = libc::c_uint;
pub const KEYC_TRIPLECLICK3_PANE: unnamed_17 = 268435522;
pub const KEYC_BSPACE: unnamed_17 = 268435525;
pub const KEYC_WHEELDOWN_BORDER: unnamed_17 = 268435506;
pub const KEYC_F10: unnamed_17 = 268435535;
pub const KEYC_MOUSEDRAGEND1_STATUS: unnamed_17 = 268435493;
pub type unnamed_35 = libc::c_uint;
pub const KEYC_MOUSEDRAG2_STATUS: unnamed_17 = 268435487;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_3,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
pub const KEYC_DOUBLECLICK2_STATUS: unnamed_17 = 268435511;
pub type layout_type = libc::c_uint;
pub const KEYC_F11: unnamed_17 = 268435536;
pub const KEYC_KP_SIX: unnamed_17 = 268435558;
pub const KEYC_MOUSEUP3_PANE: unnamed_17 = 268435480;
pub const KEYC_MOUSEUP1_BORDER: unnamed_17 = 268435476;
pub const KEYC_DOUBLECLICK2_PANE: unnamed_17 = 268435510;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const PROMPT_ENTRY: unnamed_32 = 0;
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
    pub wentry: unnamed_28,
    pub sentry: unnamed_26,
}
pub const KEYC_WHEELUP_PANE: unnamed_17 = 268435501;
pub const KEYC_DOUBLECLICK1_BORDER: unnamed_17 = 268435509;
pub const JOB_DEAD: unnamed_34 = 1;
pub type unnamed_36 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub const KEYC_WHEELDOWN_PANE: unnamed_17 = 268435504;
pub const KEYC_NPAGE: unnamed_17 = 268435542;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const KEYC_F5: unnamed_17 = 268435530;
pub const KEYC_KP_ENTER: unnamed_17 = 268435562;
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const KEYC_PASTE_END: unnamed_17 = 268435459;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_10,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_19,
}
pub type cmdq_type = libc::c_uint;
pub const KEYC_MOUSEUP1_PANE: unnamed_17 = 268435474;
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_39 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const KEYC_MOUSEDOWN1_STATUS: unnamed_17 = 268435466;
pub const KEYC_TRIPLECLICK3_BORDER: unnamed_17 = 268435524;
pub const KEYC_MOUSEDRAG2_PANE: unnamed_17 = 268435486;
pub const KEYC_MOUSEDOWN1_BORDER: unnamed_17 = 268435467;
pub const KEYC_MOUSEUP3_BORDER: unnamed_17 = 268435482;
pub const KEYC_MOUSEDOWN3_PANE: unnamed_17 = 268435471;
pub const KEYC_RIGHT: unnamed_17 = 268435548;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
pub type tcflag_t = libc::c_uint;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const KEYC_MOUSEDOWN1_PANE: unnamed_17 = 268435465;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[no_mangle]
pub unsafe extern "C" fn xterm_keys_lookup(mut key: key_code)
 -> *mut libc::c_char {
    let mut entry: *const xterm_keys_entry = 0 as *const xterm_keys_entry;
    let mut i: u_int = 0;
    let mut modifiers: key_code = 0;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    modifiers = 1i32 as key_code;
    if 0 != key & 140737488355328u64 {
        modifiers =
            (modifiers as
                 libc::c_ulonglong).wrapping_add(1i32 as libc::c_ulonglong) as
                key_code as key_code
    }
    if 0 != key & 35184372088832u64 {
        modifiers =
            (modifiers as
                 libc::c_ulonglong).wrapping_add(2i32 as libc::c_ulonglong) as
                key_code as key_code
    }
    if 0 != key & 70368744177664u64 {
        modifiers =
            (modifiers as
                 libc::c_ulonglong).wrapping_add(4i32 as libc::c_ulonglong) as
                key_code as key_code
    }
    if modifiers == 1i32 as libc::c_ulonglong {
        return 0 as *mut libc::c_char
    } else if key & (35184372088832u64 | 281474976710656u64) ==
                  35184372088832u64 {
        return 0 as *mut libc::c_char
    } else {
        key &=
            !(35184372088832u64 | 70368744177664u64 | 140737488355328u64 |
                  281474976710656u64);
        i = 0i32 as u_int;
        while (i as libc::c_ulong) <
                  (::std::mem::size_of::<[xterm_keys_entry; 57]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<xterm_keys_entry>()
                                                       as libc::c_ulong) {
            entry = &xterm_keys_table[i as usize] as *const xterm_keys_entry;
            if key == (*entry).key { break ; }
            i = i.wrapping_add(1)
        }
        if i as libc::c_ulong ==
               (::std::mem::size_of::<[xterm_keys_entry; 57]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<xterm_keys_entry>()
                                                    as libc::c_ulong) {
            return 0 as *mut libc::c_char
        } else {
            out = xstrdup((*entry).template);
            *out.offset(strcspn(out,
                                b"_\x00" as *const u8 as *const libc::c_char)
                            as isize) =
                (48 as libc::c_ulonglong).wrapping_add(modifiers) as
                    libc::c_char;
            return out
        }
    };
}
static mut xterm_keys_table: [xterm_keys_entry; 57] =
    unsafe {
        [xterm_keys_entry{key: KEYC_F1 as libc::c_int as key_code,
                          template:
                              b"\x1b[1;_P\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F1 as libc::c_int as key_code,
                          template:
                              b"\x1bO1;_P\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F1 as libc::c_int as key_code,
                          template:
                              b"\x1bO_P\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F2 as libc::c_int as key_code,
                          template:
                              b"\x1b[1;_Q\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F2 as libc::c_int as key_code,
                          template:
                              b"\x1bO1;_Q\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F2 as libc::c_int as key_code,
                          template:
                              b"\x1bO_Q\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F3 as libc::c_int as key_code,
                          template:
                              b"\x1b[1;_R\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F3 as libc::c_int as key_code,
                          template:
                              b"\x1bO1;_R\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F3 as libc::c_int as key_code,
                          template:
                              b"\x1bO_R\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F4 as libc::c_int as key_code,
                          template:
                              b"\x1b[1;_S\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F4 as libc::c_int as key_code,
                          template:
                              b"\x1bO1;_S\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F4 as libc::c_int as key_code,
                          template:
                              b"\x1bO_S\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F5 as libc::c_int as key_code,
                          template:
                              b"\x1b[15;_~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F6 as libc::c_int as key_code,
                          template:
                              b"\x1b[17;_~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F7 as libc::c_int as key_code,
                          template:
                              b"\x1b[18;_~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F8 as libc::c_int as key_code,
                          template:
                              b"\x1b[19;_~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F9 as libc::c_int as key_code,
                          template:
                              b"\x1b[20;_~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F10 as libc::c_int as key_code,
                          template:
                              b"\x1b[21;_~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F11 as libc::c_int as key_code,
                          template:
                              b"\x1b[23;_~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_F12 as libc::c_int as key_code,
                          template:
                              b"\x1b[24;_~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_UP as libc::c_int as key_code,
                          template:
                              b"\x1b[1;_A\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_DOWN as libc::c_int as key_code,
                          template:
                              b"\x1b[1;_B\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_RIGHT as libc::c_int as key_code,
                          template:
                              b"\x1b[1;_C\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_LEFT as libc::c_int as key_code,
                          template:
                              b"\x1b[1;_D\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_HOME as libc::c_int as key_code,
                          template:
                              b"\x1b[1;_H\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_END as libc::c_int as key_code,
                          template:
                              b"\x1b[1;_F\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_PPAGE as libc::c_int as key_code,
                          template:
                              b"\x1b[5;_~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_NPAGE as libc::c_int as key_code,
                          template:
                              b"\x1b[6;_~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_IC as libc::c_int as key_code,
                          template:
                              b"\x1b[2;_~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: KEYC_DC as libc::c_int as key_code,
                          template:
                              b"\x1b[3;_~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 33 as key_code,
                          template:
                              b"\x1b[27;_;33~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 35 as key_code,
                          template:
                              b"\x1b[27;_;35~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 40 as key_code,
                          template:
                              b"\x1b[27;_;40~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 41 as key_code,
                          template:
                              b"\x1b[27;_;41~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 43 as key_code,
                          template:
                              b"\x1b[27;_;43~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 44 as key_code,
                          template:
                              b"\x1b[27;_;44~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 45 as key_code,
                          template:
                              b"\x1b[27;_;45~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 46 as key_code,
                          template:
                              b"\x1b[27;_;46~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 48 as key_code,
                          template:
                              b"\x1b[27;_;48~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 49 as key_code,
                          template:
                              b"\x1b[27;_;49~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 50 as key_code,
                          template:
                              b"\x1b[27;_;50~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 51 as key_code,
                          template:
                              b"\x1b[27;_;51~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 52 as key_code,
                          template:
                              b"\x1b[27;_;52~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 53 as key_code,
                          template:
                              b"\x1b[27;_;53~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 54 as key_code,
                          template:
                              b"\x1b[27;_;54~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 55 as key_code,
                          template:
                              b"\x1b[27;_;55~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 56 as key_code,
                          template:
                              b"\x1b[27;_;56~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 57 as key_code,
                          template:
                              b"\x1b[27;_;57~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 58 as key_code,
                          template:
                              b"\x1b[27;_;58~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 59 as key_code,
                          template:
                              b"\x1b[27;_;59~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 60 as key_code,
                          template:
                              b"\x1b[27;_;60~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 61 as key_code,
                          template:
                              b"\x1b[27;_;61~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 62 as key_code,
                          template:
                              b"\x1b[27;_;62~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 63 as key_code,
                          template:
                              b"\x1b[27;_;63~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 39 as key_code,
                          template:
                              b"\x1b[27;_;39~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 13 as key_code,
                          template:
                              b"\x1b[27;_;13~\x00" as *const u8 as
                                  *const libc::c_char,},
         xterm_keys_entry{key: 9 as key_code,
                          template:
                              b"\x1b[27;_;9~\x00" as *const u8 as
                                  *const libc::c_char,}]
    };
#[no_mangle]
pub unsafe extern "C" fn xterm_keys_find(mut buf: *const libc::c_char,
                                         mut len: size_t,
                                         mut size: *mut size_t,
                                         mut key: *mut key_code)
 -> libc::c_int {
    let mut current_block: u64;
    let mut entry: *const xterm_keys_entry = 0 as *const xterm_keys_entry;
    let mut i: u_int = 0;
    let mut matched: libc::c_int = 0;
    let mut modifiers: key_code = 0;
    i = 0i32 as u_int;
    loop  {
        if (i as libc::c_ulong) <
               (::std::mem::size_of::<[xterm_keys_entry; 57]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<xterm_keys_entry>()
                                                    as libc::c_ulong) {
            entry = &xterm_keys_table[i as usize] as *const xterm_keys_entry;
            matched =
                xterm_keys_match((*entry).template, buf, len, size,
                                 &mut modifiers as *mut key_code);
            if matched == 1i32.wrapping_neg() {
                i = i.wrapping_add(1)
            } else if matched == 0i32 {
                current_block = 4644295000439058019;
                break ;
            } else { current_block = 10680521327981672866; break ; }
        } else { return 1i32.wrapping_neg() }
    }
    match current_block {
        4644295000439058019 => {
            *key = (*entry).key | modifiers | 281474976710656u64
        }
        _ => { }
    }
    return matched;
}
unsafe extern "C" fn xterm_keys_match(mut template: *const libc::c_char,
                                      mut buf: *const libc::c_char,
                                      mut len: size_t, mut size: *mut size_t,
                                      mut modifiers: *mut key_code)
 -> libc::c_int {
    let mut current_block: u64;
    let mut pos: size_t = 0;
    let mut retval: libc::c_int = 0;
    *modifiers = 0i32 as key_code;
    if len == 0i32 as libc::c_ulong {
        return 0i32
    } else {
        pos = 0i32 as size_t;
        loop  {
            if *template as libc::c_int == 95 {
                retval =
                    xterm_keys_modifiers(buf, len, &mut pos as *mut size_t,
                                         modifiers);
                if retval != 0i32 { return retval }
            } else if *buf.offset(pos as isize) as libc::c_int !=
                          *template as libc::c_int {
                return 1i32.wrapping_neg()
            } else { pos = pos.wrapping_add(1) }
            template = template.offset(1isize);
            if *template as libc::c_int != 0 && pos != len { continue ; }
            if *template as libc::c_int != 0 {
                current_block = 6873731126896040597;
                break ;
            } else { current_block = 15427931788582360902; break ; }
        }
        match current_block {
            6873731126896040597 => { return 1i32 }
            _ => { *size = pos; return 0i32 }
        }
    };
}
unsafe extern "C" fn xterm_keys_modifiers(mut buf: *const libc::c_char,
                                          mut len: size_t,
                                          mut pos: *mut size_t,
                                          mut modifiers: *mut key_code)
 -> libc::c_int {
    let mut flags: u_int = 0;
    if len.wrapping_sub(*pos) < 2i32 as libc::c_ulong {
        return 1i32
    } else if (*buf.offset(*pos as isize) as libc::c_int) < 48 ||
                  *buf.offset(*pos as isize) as libc::c_int > 57 {
        return 1i32.wrapping_neg()
    } else {
        let fresh0 = *pos;
        *pos = (*pos).wrapping_add(1);
        flags = (*buf.offset(fresh0 as isize) as libc::c_int - 48) as u_int;
        if *buf.offset(*pos as isize) as libc::c_int >= 48 &&
               *buf.offset(*pos as isize) as libc::c_int <= 57 {
            let fresh1 = *pos;
            *pos = (*pos).wrapping_add(1);
            flags =
                flags.wrapping_mul(10i32 as
                                       libc::c_uint).wrapping_add((*buf.offset(fresh1
                                                                                   as
                                                                                   isize)
                                                                       as
                                                                       libc::c_int
                                                                       - 48)
                                                                      as
                                                                      libc::c_uint)
        }
        flags =
            (flags as libc::c_uint).wrapping_sub(1i32 as libc::c_uint) as
                u_int as u_int;
        *modifiers = 0i32 as key_code;
        if 0 != flags & 1i32 as libc::c_uint {
            *modifiers |= 140737488355328u64
        }
        if 0 != flags & 2i32 as libc::c_uint {
            *modifiers |= 35184372088832u64
        }
        if 0 != flags & 4i32 as libc::c_uint {
            *modifiers |= 70368744177664u64
        }
        if 0 != flags & 8i32 as libc::c_uint {
            *modifiers |= 35184372088832u64
        }
        return 0i32
    };
}

