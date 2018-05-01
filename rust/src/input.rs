extern crate libc;
extern "C" {
    pub type screen_titles;
    pub type tmuxproc;
    pub type event_base;
    pub type evbuffer;
    pub type options;
    pub type tmuxpeer;
    pub type environ;
    pub type format_tree;
    pub type args_entry;
    pub type screen_write_collect_item;
    pub type tty_code;
    pub type screen_write_collect_line;
    pub type options_entry;
    pub type bufferevent_ops;
    pub type hooks;
    pub type format_job_tree;
    pub type _IO_FILE_plus;
    #[no_mangle]
    static in6addr_any: in6_addr;
    #[no_mangle]
    static in6addr_loopback: in6_addr;
    #[no_mangle]
    static _sys_siglist: [*const libc::c_char; 65];
    #[no_mangle]
    static sys_siglist: [*const libc::c_char; 65];
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
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    static _ns_flagdata: [_ns_flagdata; 0];
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn bsearch(__key: *const libc::c_void, __base: *const libc::c_void,
               __nmemb: size_t, __size: size_t, __compar: __compar_fn_t)
     -> *mut libc::c_void;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char)
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
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
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
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t)
     -> *mut libc::c_uchar;
    #[no_mangle]
    fn bufferevent_write(bufev: *mut bufferevent, data: *const libc::c_void,
                         size: size_t) -> libc::c_int;
    #[no_mangle]
    fn strtonum(_: *const libc::c_char, _: libc::c_longlong,
                _: libc::c_longlong, _: *mut *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    fn b64_pton(_: *const libc::c_char, _: *mut u_char, _: size_t)
     -> libc::c_int;
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
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xvasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char,
                  _: *mut __va_list_tag) -> libc::c_int;
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
    fn paste_add(_: *mut libc::c_char, _: size_t) -> ();
    #[no_mangle]
    fn notify_input(_: *mut window_pane, _: *mut evbuffer) -> ();
    #[no_mangle]
    fn notify_pane(_: *const libc::c_char, _: *mut window_pane) -> ();
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    fn options_set_number(_: *mut options, _: *const libc::c_char,
                          _: libc::c_longlong) -> *mut options_entry;
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
    fn alerts_queue(_: *mut window, _: libc::c_int) -> ();
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    fn server_status_window(_: *mut window) -> ();
    #[no_mangle]
    fn screen_write_collect_add(_: *mut screen_write_ctx, _: *const grid_cell)
     -> ();
    #[no_mangle]
    fn utf8_copy(_: *mut utf8_data, _: *const utf8_data) -> ();
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn utf8_append(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn utf8_open(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn utf8_set(_: *mut utf8_data, _: u_char) -> ();
    #[no_mangle]
    fn screen_write_carriagereturn(_: *mut screen_write_ctx) -> ();
    #[no_mangle]
    fn screen_write_linefeed(_: *mut screen_write_ctx, _: libc::c_int,
                             _: u_int) -> ();
    #[no_mangle]
    fn screen_write_backspace(_: *mut screen_write_ctx) -> ();
    #[no_mangle]
    fn screen_write_alignmenttest(_: *mut screen_write_ctx) -> ();
    #[no_mangle]
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn screen_write_mode_clear(_: *mut screen_write_ctx, _: libc::c_int)
     -> ();
    #[no_mangle]
    fn screen_write_mode_set(_: *mut screen_write_ctx, _: libc::c_int) -> ();
    #[no_mangle]
    fn screen_write_reverseindex(_: *mut screen_write_ctx, _: u_int) -> ();
    #[no_mangle]
    fn screen_write_reset(_: *mut screen_write_ctx) -> ();
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    fn window_pane_reset_palette(_: *mut window_pane) -> ();
    #[no_mangle]
    fn window_set_name(_: *mut window, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn utf8_isvalid(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn screen_set_title(_: *mut screen, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn screen_set_cursor_colour(_: *mut screen, _: *const libc::c_char) -> ();
    #[no_mangle]
    fn window_pane_unset_palette(_: *mut window_pane, _: u_int) -> ();
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx) -> ();
    #[no_mangle]
    fn screen_write_setselection(_: *mut screen_write_ctx, _: *mut u_char,
                                 _: u_int) -> ();
    #[no_mangle]
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut window_pane,
                          _: *mut screen) -> ();
    #[no_mangle]
    fn colour_join_rgb(_: u_char, _: u_char, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn window_pane_set_palette(_: *mut window_pane, _: u_int, _: libc::c_int)
     -> ();
    #[no_mangle]
    fn screen_set_cursor_style(_: *mut screen, _: u_int) -> ();
    #[no_mangle]
    fn screen_write_scrollup(_: *mut screen_write_ctx, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn window_pane_alternate_on(_: *mut window_pane, _: *mut grid_cell,
                                _: libc::c_int) -> ();
    #[no_mangle]
    fn screen_write_clearscreen(_: *mut screen_write_ctx, _: u_int) -> ();
    #[no_mangle]
    fn window_pane_alternate_off(_: *mut window_pane, _: *mut grid_cell,
                                 _: libc::c_int) -> ();
    #[no_mangle]
    fn screen_write_insertline(_: *mut screen_write_ctx, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn screen_write_insertcharacter(_: *mut screen_write_ctx, _: u_int,
                                    _: u_int) -> ();
    #[no_mangle]
    fn screen_write_clearline(_: *mut screen_write_ctx, _: u_int) -> ();
    #[no_mangle]
    fn screen_write_clearstartofline(_: *mut screen_write_ctx, _: u_int)
     -> ();
    #[no_mangle]
    fn screen_write_clearendofline(_: *mut screen_write_ctx, _: u_int) -> ();
    #[no_mangle]
    fn screen_write_clearhistory(_: *mut screen_write_ctx) -> ();
    #[no_mangle]
    fn screen_write_clearstartofscreen(_: *mut screen_write_ctx, _: u_int)
     -> ();
    #[no_mangle]
    fn screen_write_clearendofscreen(_: *mut screen_write_ctx, _: u_int)
     -> ();
    #[no_mangle]
    fn __builtin_va_end(_: *mut __va_list_tag) -> ();
    #[no_mangle]
    fn __builtin_va_start(_: *mut __va_list_tag, ...) -> ();
    #[no_mangle]
    fn screen_write_deleteline(_: *mut screen_write_ctx, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn screen_write_scrollregion(_: *mut screen_write_ctx, _: u_int, _: u_int)
     -> ();
    #[no_mangle]
    fn screen_write_deletecharacter(_: *mut screen_write_ctx, _: u_int,
                                    _: u_int) -> ();
    #[no_mangle]
    fn screen_write_clearcharacter(_: *mut screen_write_ctx, _: u_int,
                                   _: u_int) -> ();
    #[no_mangle]
    fn screen_write_cursorup(_: *mut screen_write_ctx, _: u_int) -> ();
    #[no_mangle]
    fn screen_write_cursordown(_: *mut screen_write_ctx, _: u_int) -> ();
    #[no_mangle]
    fn screen_pop_title(_: *mut screen) -> ();
    #[no_mangle]
    fn screen_push_title(_: *mut screen) -> ();
    #[no_mangle]
    fn screen_write_cursorright(_: *mut screen_write_ctx, _: u_int) -> ();
    #[no_mangle]
    fn screen_write_cursorleft(_: *mut screen_write_ctx, _: u_int) -> ();
    #[no_mangle]
    fn screen_write_rawstring(_: *mut screen_write_ctx, _: *mut u_char,
                              _: u_int) -> ();
    #[no_mangle]
    fn screen_write_collect_end(_: *mut screen_write_ctx) -> ();
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn window_update_activity(_: *mut window) -> ();
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
    fn input_reply(ictx: *mut input_ctx, fmt: *const libc::c_char, ...) -> ();
}
pub type pid_t = __pid_t;
pub type unnamed = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_1 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_2 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_23,
    pub entry: unnamed_20,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const PROMPT_ENTRY: unnamed_37 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_3 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub const INPUT_CSI_TBC: input_csi_type = 30;
pub const JOB_CLOSED: unnamed = 2;
pub const INPUT_CSI_SM_PRIVATE: input_csi_type = 28;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type uint32_t = __uint32_t;
pub const INPUT_ESC_NEL: input_esc_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_6 {
    ev_io: unnamed_11,
    ev_signal: unnamed_39,
}
pub const INPUT_CSI_ED: input_csi_type = 16;
pub type __uint8_t = libc::c_uchar;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
pub const INPUT_ESC_DECSC: input_esc_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct input_state {
    pub name: *const libc::c_char,
    pub enter: Option<unsafe extern "C" fn(_: *mut input_ctx) -> ()>,
    pub exit: Option<unsafe extern "C" fn(_: *mut input_ctx) -> ()>,
    pub transitions: *const input_transition,
}
pub type input_esc_type = libc::c_uint;
pub type __u_short = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub const INPUT_CSI_REP: input_csi_type = 22;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
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
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const INPUT_CSI_DECSTBM: input_csi_type = 12;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub type cc_t = libc::c_uchar;
pub const INPUT_ESC_RI: input_esc_type = 8;
pub const TTY_VT102: unnamed_22 = 2;
pub const JOB_DEAD: unnamed = 1;
pub const INPUT_CSI_RM_PRIVATE: input_csi_type = 24;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _ns_flagdata {
    pub mask: libc::c_int,
    pub shift: libc::c_int,
}
pub const INPUT_CSI_ECH: input_csi_type = 15;
pub const UTF8_MORE: utf8_state = 0;
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
pub const TTY_VT420: unnamed_22 = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type u_char = __u_char;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_10,
}
pub const INPUT_CSI_WINOPS: input_csi_type = 32;
pub type __uint16_t = libc::c_ushort;
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed_36,
}
pub const UTF8_ERROR: utf8_state = 2;
pub const INPUT_CSI_RCP: input_csi_type = 21;
pub const INPUT_CSI_DL: input_csi_type = 13;
pub type va_list = __builtin_va_list;
pub const INPUT_CSI_CUP: input_csi_type = 6;
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
pub union unnamed_10 {
    offset: u_int,
    data: unnamed_7,
}
pub const CMDQ_COMMAND: cmdq_type = 0;
pub const INPUT_CSI_CPL: input_csi_type = 2;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub const JOB_RUNNING: unnamed = 0;
pub const INPUT_ESC_SCSG1_OFF: input_esc_type = 12;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_11 {
    pub ev_io_next: unnamed_19,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_12 {
    num: libc::c_int,
    str_0: *mut libc::c_char,
}
pub const INPUT_CSI_SGR: input_csi_type = 26;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub const INPUT_ESC_IND: input_esc_type = 6;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const TTY_VT101: unnamed_22 = 1;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
pub const INPUT_CSI_DA: input_csi_type = 8;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub type input_csi_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
pub type unnamed_15 = libc::c_uint;
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
    pub gentry: unnamed_31,
    pub entry: unnamed_18,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
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
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: unnamed_25,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: unnamed_6,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
pub const TTY_UNKNOWN: unnamed_22 = 6;
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
    pub entry: unnamed_16,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const INPUT_ESC_DECKPNM: input_esc_type = 2;
pub type options_table_type = libc::c_uint;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const INPUT_STRING: unnamed_15 = 2;
pub type ssize_t = __ssize_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_groups {
    pub rbh_root: *mut session_group,
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
    pub entry: unnamed_8,
    pub wentry: unnamed_30,
    pub sentry: unnamed_32,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_20 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
pub type __ssize_t = libc::c_long;
pub const INPUT_CSI_DCH: input_csi_type = 10;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
pub const INPUT_CSI_EL: input_csi_type = 17;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_40,
}
pub const INPUT_ESC_SCSG0_ON: input_esc_type = 11;
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
pub type u_short = __u_short;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_21 {
    evcb_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: libc::c_short,
                                               _: *mut libc::c_void) -> ()>,
    evcb_selfcb: Option<unsafe extern "C" fn(_: *mut event_callback,
                                             _: *mut libc::c_void) -> ()>,
    evcb_evfinalize: Option<unsafe extern "C" fn(_: *mut event,
                                                 _: *mut libc::c_void) -> ()>,
    evcb_cbfinalize: Option<unsafe extern "C" fn(_: *mut event_callback,
                                                 _: *mut libc::c_void) -> ()>,
}
pub type unnamed_22 = libc::c_uint;
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
pub const INPUT_CSI_SCP: input_csi_type = 25;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
pub const INPUT_CSI_RM: input_csi_type = 23;
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
    pub entry: unnamed_35,
}
pub type __uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub type utf8_state = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_25 {
    ev_next_with_common_timeout: unnamed_29,
    min_heap_idx: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_callback {
    pub evcb_active_next: unnamed_34,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: unnamed_21,
    pub evcb_arg: *mut libc::c_void,
}
pub type _IO_lock_t = ();
pub const INPUT_CSI_ICH: input_csi_type = 19;
pub type __u_int = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_1,
}
pub type layout_type = libc::c_uint;
pub type cmd_retval = libc::c_int;
pub const INPUT_ESC_RIS: input_esc_type = 9;
pub const INPUT_CSI_CBT: input_csi_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
pub const INPUT_ESC_ST: input_esc_type = 14;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_27,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type __pid_t = libc::c_int;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct input_cell {
    pub cell: grid_cell,
    pub set: libc::c_int,
    pub g0set: libc::c_int,
    pub g1set: libc::c_int,
}
pub const TTY_VT100: unnamed_22 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct input_table_entry {
    pub ch: libc::c_int,
    pub interm: *const libc::c_char,
    pub type_0: libc::c_int,
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
pub const INPUT_CSI_SM: input_csi_type = 27;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_17,
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
    pub alerts_entry: unnamed_0,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: unnamed_4,
    pub entry: unnamed_33,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed,
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
    pub entry: unnamed_28,
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const INPUT_ESC_SCSG1_ON: input_esc_type = 13;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_26 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
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
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub type __off_t = libc::c_long;
pub type cmdq_type = libc::c_uint;
pub const INPUT_MISSING: unnamed_15 = 0;
pub type __off64_t = libc::c_long;
pub const INPUT_CSI_IL: input_csi_type = 20;
pub const LINE_SEL_LEFT_RIGHT: unnamed_27 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type unnamed_27 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_28 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub type __time_t = libc::c_long;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub const INPUT_CSI_DECSCUSR: input_csi_type = 11;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: unnamed_2,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item)
                         -> cmd_retval>,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub const LINE_SEL_NONE: unnamed_27 = 0;
pub type __u_char = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_5,
}
pub const INPUT_CSI_CUD: input_csi_type = 4;
pub type uint8_t = __uint8_t;
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
    pub message_log: unnamed_26,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: unnamed_37,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: unnamed_13,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct input_transition {
    pub first: libc::c_int,
    pub last: libc::c_int,
    pub handler: Option<unsafe extern "C" fn(_: *mut input_ctx)
                            -> libc::c_int>,
    pub state: *const input_state,
}
pub const INPUT_CSI_HPA: input_csi_type = 18;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_31 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
pub const CMDQ_CALLBACK: cmdq_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_32 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
pub const LINE_SEL_RIGHT_LEFT: unnamed_27 = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
pub type __suseconds_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_34 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct input_ctx {
    pub wp: *mut window_pane,
    pub ctx: screen_write_ctx,
    pub cell: input_cell,
    pub old_cell: input_cell,
    pub old_cx: u_int,
    pub old_cy: u_int,
    pub interm_buf: [u_char; 4],
    pub interm_len: size_t,
    pub param_buf: [u_char; 64],
    pub param_len: size_t,
    pub input_buf: *mut u_char,
    pub input_len: size_t,
    pub input_space: size_t,
    pub param_list: [input_param; 24],
    pub param_list_len: u_int,
    pub utf8data: utf8_data,
    pub utf8started: libc::c_int,
    pub ch: libc::c_int,
    pub last: libc::c_int,
    pub flags: libc::c_int,
    pub state: *const input_state,
    pub timer: event,
    pub since_ground: *mut evbuffer,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub const INPUT_CSI_CUB: input_csi_type = 3;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const INPUT_CSI_DA_TWO: input_csi_type = 9;
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
pub const INPUT_CSI_DSR: input_csi_type = 14;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const INPUT_ESC_SCSG0_OFF: input_esc_type = 10;
pub type unnamed_37 = libc::c_uint;
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
    pub tree_entry: unnamed_14,
}
pub const INPUT_CSI_VPA: input_csi_type = 31;
pub const INPUT_NUMBER: unnamed_15 = 1;
pub const INPUT_ESC_DECRC: input_esc_type = 3;
pub const INPUT_CSI_CUF: input_csi_type = 5;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
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
pub type u_int = __u_int;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub const INPUT_CSI_CNL: input_csi_type = 1;
pub type cmd_find_type = libc::c_uint;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
pub type tcflag_t = libc::c_uint;
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
    pub term_type: unnamed_22,
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
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_38,
}
pub const UTF8_DONE: utf8_state = 1;
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
pub const TTY_VT220: unnamed_22 = 3;
pub const INPUT_ESC_HTS: input_esc_type = 5;
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
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub const TTY_VT320: unnamed_22 = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
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
pub struct unnamed_39 {
    pub ev_signal_next: unnamed_3,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
pub const PROMPT_COMMAND: unnamed_37 = 1;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub type uint16_t = __uint16_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
pub const INPUT_ESC_DECALN: input_esc_type = 0;
pub type time_t = __time_t;
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
pub const INPUT_CSI_SU: input_csi_type = 29;
pub const INPUT_CSI_CUU: input_csi_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: unnamed_9,
}
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct input_param {
    pub type_0: unnamed_15,
    pub unnamed: unnamed_12,
}
pub const INPUT_ESC_DECKPAM: input_esc_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_40 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
    return __x;
}
unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
    return __x;
}
unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
    return __x;
}
#[no_mangle]
pub unsafe extern "C" fn input_init(mut wp: *mut window_pane) -> () {
    let mut ictx: *mut input_ctx = 0 as *mut input_ctx;
    (*wp).ictx =
        xcalloc(1i32 as size_t,
                ::std::mem::size_of::<input_ctx>() as libc::c_ulong) as
            *mut input_ctx;
    ictx = (*wp).ictx;
    (*ictx).input_space = 32i32 as size_t;
    (*ictx).input_buf = xmalloc(32i32 as size_t) as *mut u_char;
    (*ictx).since_ground = evbuffer_new();
    event_set(&mut (*ictx).timer as *mut event, 1i32.wrapping_neg(),
              0i32 as libc::c_short, Some(input_timer_callback),
              ictx as *mut libc::c_void);
    input_reset(wp, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn input_reset(mut wp: *mut window_pane,
                                     mut clear: libc::c_int) -> () {
    let mut ictx: *mut input_ctx = (*wp).ictx;
    input_reset_cell(ictx);
    if 0 != clear {
        if (*wp).mode == 0 as *mut libc::c_void as *const window_mode {
            screen_write_start(&mut (*ictx).ctx as *mut screen_write_ctx, wp,
                               &mut (*wp).base as *mut screen);
        } else {
            screen_write_start(&mut (*ictx).ctx as *mut screen_write_ctx,
                               0 as *mut window_pane,
                               &mut (*wp).base as *mut screen);
        }
        screen_write_reset(&mut (*ictx).ctx as *mut screen_write_ctx);
        screen_write_stop(&mut (*ictx).ctx as *mut screen_write_ctx);
    }
    input_clear(ictx);
    (*ictx).last = 1i32.wrapping_neg();
    (*ictx).state = &input_state_ground as *const input_state;
    (*ictx).flags = 0i32;
}
pub static mut input_state_ground: input_state =
    unsafe {
        input_state{name: b"ground\x00" as *const u8 as *const libc::c_char,
                    enter: Some(input_ground),
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_ground_table: [input_transition; 10] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 126i32,
                          handler: Some(input_print),
                          state: 0 as *const input_state,},
         input_transition{first: 127i32,
                          last: 127i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 128i32,
                          last: 255i32,
                          handler: Some(input_top_bit_set),
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
unsafe extern "C" fn input_top_bit_set(mut ictx: *mut input_ctx)
 -> libc::c_int {
    let mut ud: *mut utf8_data = &mut (*ictx).utf8data as *mut utf8_data;
    (*ictx).last = 1i32.wrapping_neg();
    if 0 == (*ictx).utf8started {
        if utf8_open(ud, (*ictx).ch as u_char) as libc::c_uint !=
               UTF8_MORE as libc::c_int as libc::c_uint {
            return 0i32
        } else { (*ictx).utf8started = 1i32; return 0i32 }
    } else {
        match utf8_append(ud, (*ictx).ch as u_char) as libc::c_uint {
            0 => { return 0i32 }
            2 => { (*ictx).utf8started = 0i32; return 0i32 }
            1 | _ => {
                (*ictx).utf8started = 0i32;
                log_debug(b"%s %hhu \'%*s\' (width %hhu)\x00" as *const u8 as
                              *const libc::c_char,
                          (*::std::mem::transmute::<&[u8; 18],
                                                    &[libc::c_char; 18]>(b"input_top_bit_set\x00")).as_ptr(),
                          (*ud).size as libc::c_int,
                          (*ud).size as libc::c_int, (*ud).data.as_mut_ptr(),
                          (*ud).width as libc::c_int);
                utf8_copy(&mut (*ictx).cell.cell.data as *mut utf8_data, ud);
                screen_write_collect_add(&mut (*ictx).ctx as
                                             *mut screen_write_ctx,
                                         &mut (*ictx).cell.cell as
                                             *mut grid_cell);
                return 0i32
            }
        }
    };
}
unsafe extern "C" fn input_print(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut set: libc::c_int = 0;
    (*ictx).utf8started = 0i32;
    set =
        if (*ictx).cell.set == 0i32 {
            (*ictx).cell.g0set
        } else { (*ictx).cell.g1set };
    if set == 1i32 {
        (*ictx).cell.cell.attr =
            ((*ictx).cell.cell.attr as libc::c_int | 128i32) as u_short
    } else {
        (*ictx).cell.cell.attr =
            ((*ictx).cell.cell.attr as libc::c_int & !128i32) as u_short
    }
    utf8_set(&mut (*ictx).cell.cell.data as *mut utf8_data,
             (*ictx).ch as u_char);
    screen_write_collect_add(&mut (*ictx).ctx as *mut screen_write_ctx,
                             &mut (*ictx).cell.cell as *mut grid_cell);
    (*ictx).last = (*ictx).ch;
    (*ictx).cell.cell.attr =
        ((*ictx).cell.cell.attr as libc::c_int & !128i32) as u_short;
    return 0i32;
}
unsafe extern "C" fn input_c0_dispatch(mut ictx: *mut input_ctx)
 -> libc::c_int {
    let mut sctx: *mut screen_write_ctx =
        &mut (*ictx).ctx as *mut screen_write_ctx;
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut s: *mut screen = (*sctx).s;
    (*ictx).utf8started = 0i32;
    log_debug(b"%s: \'%c\'\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"input_c0_dispatch\x00")).as_ptr(),
              (*ictx).ch);
    match (*ictx).ch {
        0 => { }
        7 => { alerts_queue((*wp).window, 1i32); }
        8 => { screen_write_backspace(sctx); }
        9 => {
            if !((*s).cx >=
                     (*(*s).grid).sx.wrapping_sub(1i32 as libc::c_uint)) {
                loop  {
                    (*s).cx = (*s).cx.wrapping_add(1);
                    if 0 !=
                           *(*s).tabs.offset(((*s).cx >> 3i32) as isize) as
                               libc::c_int &
                               1i32 << ((*s).cx & 7i32 as libc::c_uint) {
                        break ;
                    }
                    if !((*s).cx <
                             (*(*s).grid).sx.wrapping_sub(1i32 as
                                                              libc::c_uint)) {
                        break ;
                    }
                }
            }
        }
        10 | 11 | 12 => {
            screen_write_linefeed(sctx, 0i32, (*ictx).cell.cell.bg as u_int);
        }
        13 => { screen_write_carriagereturn(sctx); }
        14 => { (*ictx).cell.set = 1i32 }
        15 => { (*ictx).cell.set = 0i32 }
        _ => {
            log_debug(b"%s: unknown \'%c\'\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"input_c0_dispatch\x00")).as_ptr(),
                      (*ictx).ch);
        }
    }
    (*ictx).last = 1i32.wrapping_neg();
    return 0i32;
}
pub static mut input_state_esc_enter: input_state =
    unsafe {
        input_state{name:
                        b"esc_enter\x00" as *const u8 as *const libc::c_char,
                    enter: Some(input_clear),
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_esc_enter_table: [input_transition; 23] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 47i32,
                          handler: Some(input_intermediate),
                          state:
                              &input_state_esc_intermediate as
                                  *const input_state,},
         input_transition{first: 48i32,
                          last: 79i32,
                          handler: Some(input_esc_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 80i32,
                          last: 80i32,
                          handler: None,
                          state:
                              &input_state_dcs_enter as *const input_state,},
         input_transition{first: 81i32,
                          last: 87i32,
                          handler: Some(input_esc_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 88i32,
                          last: 88i32,
                          handler: None,
                          state:
                              &input_state_consume_st as *const input_state,},
         input_transition{first: 89i32,
                          last: 89i32,
                          handler: Some(input_esc_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 90i32,
                          last: 90i32,
                          handler: Some(input_esc_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 91i32,
                          last: 91i32,
                          handler: None,
                          state:
                              &input_state_csi_enter as *const input_state,},
         input_transition{first: 92i32,
                          last: 92i32,
                          handler: Some(input_esc_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 93i32,
                          last: 93i32,
                          handler: None,
                          state:
                              &input_state_osc_string as *const input_state,},
         input_transition{first: 94i32,
                          last: 94i32,
                          handler: None,
                          state:
                              &input_state_consume_st as *const input_state,},
         input_transition{first: 95i32,
                          last: 95i32,
                          handler: None,
                          state:
                              &input_state_apc_string as *const input_state,},
         input_transition{first: 96i32,
                          last: 106i32,
                          handler: Some(input_esc_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 107i32,
                          last: 107i32,
                          handler: None,
                          state:
                              &input_state_rename_string as
                                  *const input_state,},
         input_transition{first: 108i32,
                          last: 126i32,
                          handler: Some(input_esc_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 127i32,
                          last: 255i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
unsafe extern "C" fn input_esc_dispatch(mut ictx: *mut input_ctx)
 -> libc::c_int {
    let mut sctx: *mut screen_write_ctx =
        &mut (*ictx).ctx as *mut screen_write_ctx;
    let mut s: *mut screen = (*sctx).s;
    let mut entry: *mut input_table_entry = 0 as *mut input_table_entry;
    if 0 != (*ictx).flags & 1i32 {
        return 0i32
    } else {
        log_debug(b"%s: \'%c\', %s\x00" as *const u8 as *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"input_esc_dispatch\x00")).as_ptr(),
                  (*ictx).ch, (*ictx).interm_buf.as_mut_ptr());
        entry =
            bsearch(ictx as *const libc::c_void,
                    input_esc_table.as_ptr() as *const libc::c_void,
                    (::std::mem::size_of::<[input_table_entry; 15]>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<input_table_entry>()
                                                         as libc::c_ulong),
                    ::std::mem::size_of::<input_table_entry>() as
                        libc::c_ulong, Some(input_table_compare)) as
                *mut input_table_entry;
        if entry == 0 as *mut libc::c_void as *mut input_table_entry {
            log_debug(b"%s: unknown \'%c\'\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"input_esc_dispatch\x00")).as_ptr(),
                      (*ictx).ch);
            return 0i32
        } else {
            match (*entry).type_0 {
                9 => {
                    window_pane_reset_palette((*ictx).wp);
                    input_reset_cell(ictx);
                    screen_write_reset(sctx);
                }
                6 => {
                    screen_write_linefeed(sctx, 0i32,
                                          (*ictx).cell.cell.bg as u_int);
                }
                7 => {
                    screen_write_carriagereturn(sctx);
                    screen_write_linefeed(sctx, 0i32,
                                          (*ictx).cell.cell.bg as u_int);
                }
                5 => {
                    if (*s).cx < (*(*s).grid).sx {
                        let ref mut fresh0 =
                            *(*s).tabs.offset(((*s).cx >> 3i32) as isize);
                        *fresh0 =
                            (*fresh0 as libc::c_int |
                                 1i32 << ((*s).cx & 7i32 as libc::c_uint)) as
                                bitstr_t
                    }
                }
                8 => {
                    screen_write_reverseindex(sctx,
                                              (*ictx).cell.cell.bg as u_int);
                }
                1 => { screen_write_mode_set(sctx, 8i32); }
                2 => { screen_write_mode_clear(sctx, 8i32); }
                4 => {
                    memcpy(&mut (*ictx).old_cell as *mut input_cell as
                               *mut libc::c_void,
                           &mut (*ictx).cell as *mut input_cell as
                               *const libc::c_void,
                           ::std::mem::size_of::<input_cell>() as
                               libc::c_ulong);
                    (*ictx).old_cx = (*s).cx;
                    (*ictx).old_cy = (*s).cy
                }
                3 => {
                    memcpy(&mut (*ictx).cell as *mut input_cell as
                               *mut libc::c_void,
                           &mut (*ictx).old_cell as *mut input_cell as
                               *const libc::c_void,
                           ::std::mem::size_of::<input_cell>() as
                               libc::c_ulong);
                    screen_write_cursormove(sctx, (*ictx).old_cx,
                                            (*ictx).old_cy);
                }
                0 => { screen_write_alignmenttest(sctx); }
                11 => { (*ictx).cell.g0set = 1i32 }
                10 => { (*ictx).cell.g0set = 0i32 }
                13 => { (*ictx).cell.g1set = 1i32 }
                12 => { (*ictx).cell.g1set = 0i32 }
                14 | _ => { }
            }
            (*ictx).last = 1i32.wrapping_neg();
            return 0i32
        }
    };
}
unsafe extern "C" fn input_reset_cell(mut ictx: *mut input_ctx) -> () {
    memcpy(&mut (*ictx).cell.cell as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    (*ictx).cell.set = 0i32;
    (*ictx).cell.g1set = 0i32;
    (*ictx).cell.g0set = (*ictx).cell.g1set;
    memcpy(&mut (*ictx).old_cell as *mut input_cell as *mut libc::c_void,
           &mut (*ictx).cell as *mut input_cell as *const libc::c_void,
           ::std::mem::size_of::<input_cell>() as libc::c_ulong);
    (*ictx).old_cx = 0i32 as u_int;
    (*ictx).old_cy = 0i32 as u_int;
}
unsafe extern "C" fn input_table_compare(mut key: *const libc::c_void,
                                         mut value: *const libc::c_void)
 -> libc::c_int {
    let mut ictx: *const input_ctx = key as *const input_ctx;
    let mut entry: *const input_table_entry =
        value as *const input_table_entry;
    if (*ictx).ch != (*entry).ch {
        return (*ictx).ch - (*entry).ch
    } else {
        return strcmp((*ictx).interm_buf.as_ptr() as *const libc::c_char,
                      (*entry).interm)
    };
}
static mut input_esc_table: [input_table_entry; 15] =
    unsafe {
        [input_table_entry{ch: 48,
                           interm:
                               b"(\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_SCSG0_ON as libc::c_int,},
         input_table_entry{ch: 48,
                           interm:
                               b")\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_SCSG1_ON as libc::c_int,},
         input_table_entry{ch: 55,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_DECSC as libc::c_int,},
         input_table_entry{ch: 56,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_DECRC as libc::c_int,},
         input_table_entry{ch: 56,
                           interm:
                               b"#\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_DECALN as libc::c_int,},
         input_table_entry{ch: 61,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_DECKPAM as libc::c_int,},
         input_table_entry{ch: 62,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_DECKPNM as libc::c_int,},
         input_table_entry{ch: 66,
                           interm:
                               b"(\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_SCSG0_OFF as libc::c_int,},
         input_table_entry{ch: 66,
                           interm:
                               b")\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_SCSG1_OFF as libc::c_int,},
         input_table_entry{ch: 68,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_IND as libc::c_int,},
         input_table_entry{ch: 69,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_NEL as libc::c_int,},
         input_table_entry{ch: 72,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_HTS as libc::c_int,},
         input_table_entry{ch: 77,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_RI as libc::c_int,},
         input_table_entry{ch: 92,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_ST as libc::c_int,},
         input_table_entry{ch: 99,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_ESC_RIS as libc::c_int,}]
    };
pub static mut input_state_rename_string: input_state =
    unsafe {
        input_state{name:
                        b"rename_string\x00" as *const u8 as
                            *const libc::c_char,
                    enter: Some(input_enter_rename),
                    exit: Some(input_exit_rename),
                    transitions: 0 as *const _,}
    };
pub static mut input_state_rename_string_table: [input_transition; 8] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 255i32,
                          handler: Some(input_input),
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
unsafe extern "C" fn input_input(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut available: size_t = 0;
    available = (*ictx).input_space;
    loop  {
        if (*ictx).input_len.wrapping_add(1i32 as libc::c_ulong) >= available
           {
            available =
                (available as
                     libc::c_ulong).wrapping_mul(2i32 as libc::c_ulong) as
                    size_t as size_t;
            if available > 1048576i32 as libc::c_ulong {
                (*ictx).flags |= 1i32;
                return 0i32
            } else {
                (*ictx).input_buf =
                    xrealloc((*ictx).input_buf as *mut libc::c_void,
                             available) as *mut u_char;
                (*ictx).input_space = available
            }
        } else {
            let fresh1 = (*ictx).input_len;
            (*ictx).input_len = (*ictx).input_len.wrapping_add(1);
            *(*ictx).input_buf.offset(fresh1 as isize) = (*ictx).ch as u_char;
            *(*ictx).input_buf.offset((*ictx).input_len as isize) =
                0 as u_char;
            return 0i32
        }
    };
}
unsafe extern "C" fn input_exit_rename(mut ictx: *mut input_ctx) -> () {
    if 0 != (*ictx).flags & 1i32 {
        return
    } else if 0 ==
                  options_get_number((*(*(*ictx).wp).window).options,
                                     b"allow-rename\x00" as *const u8 as
                                         *const libc::c_char) {
        return
    } else {
        log_debug(b"%s: \"%s\"\x00" as *const u8 as *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"input_exit_rename\x00")).as_ptr(),
                  (*ictx).input_buf);
        if 0 == utf8_isvalid((*ictx).input_buf as *const libc::c_char) {
            return
        } else {
            window_set_name((*(*ictx).wp).window,
                            (*ictx).input_buf as *const libc::c_char);
            options_set_number((*(*(*ictx).wp).window).options,
                               b"automatic-rename\x00" as *const u8 as
                                   *const libc::c_char,
                               0i32 as libc::c_longlong);
            server_status_window((*(*ictx).wp).window);
            return;
        }
    };
}
unsafe extern "C" fn input_enter_rename(mut ictx: *mut input_ctx) -> () {
    log_debug(b"%s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"input_enter_rename\x00")).as_ptr());
    input_clear(ictx);
    input_start_timer(ictx);
    (*ictx).last = 1i32.wrapping_neg();
}
unsafe extern "C" fn input_start_timer(mut ictx: *mut input_ctx) -> () {
    let mut tv: timeval =
        timeval{tv_sec: 0, tv_usec: 100000i32 as __suseconds_t,};
    event_del(&mut (*ictx).timer as *mut event);
    event_add(&mut (*ictx).timer as *mut event, &mut tv as *mut timeval);
}
unsafe extern "C" fn input_clear(mut ictx: *mut input_ctx) -> () {
    event_del(&mut (*ictx).timer as *mut event);
    *(*ictx).interm_buf.as_mut_ptr() = 0 as u_char;
    (*ictx).interm_len = 0i32 as size_t;
    *(*ictx).param_buf.as_mut_ptr() = 0 as u_char;
    (*ictx).param_len = 0i32 as size_t;
    *(*ictx).input_buf = 0 as u_char;
    (*ictx).input_len = 0i32 as size_t;
    (*ictx).flags &= !1i32;
}
pub static mut input_state_apc_string: input_state =
    unsafe {
        input_state{name:
                        b"apc_string\x00" as *const u8 as *const libc::c_char,
                    enter: Some(input_enter_apc),
                    exit: Some(input_exit_apc),
                    transitions: 0 as *const _,}
    };
pub static mut input_state_apc_string_table: [input_transition; 8] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 255i32,
                          handler: Some(input_input),
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
unsafe extern "C" fn input_exit_apc(mut ictx: *mut input_ctx) -> () {
    if 0 != (*ictx).flags & 1i32 {
        return
    } else {
        log_debug(b"%s: \"%s\"\x00" as *const u8 as *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"input_exit_apc\x00")).as_ptr(),
                  (*ictx).input_buf);
        if 0 == utf8_isvalid((*ictx).input_buf as *const libc::c_char) {
            return
        } else {
            screen_set_title((*ictx).ctx.s,
                             (*ictx).input_buf as *const libc::c_char);
            server_status_window((*(*ictx).wp).window);
            return;
        }
    };
}
unsafe extern "C" fn input_enter_apc(mut ictx: *mut input_ctx) -> () {
    log_debug(b"%s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"input_enter_apc\x00")).as_ptr());
    input_clear(ictx);
    input_start_timer(ictx);
    (*ictx).last = 1i32.wrapping_neg();
}
pub static mut input_state_consume_st: input_state =
    unsafe {
        input_state{name:
                        b"consume_st\x00" as *const u8 as *const libc::c_char,
                    enter: Some(input_enter_rename),
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_consume_st_table: [input_transition; 8] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 255i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
pub static mut input_state_osc_string: input_state =
    unsafe {
        input_state{name:
                        b"osc_string\x00" as *const u8 as *const libc::c_char,
                    enter: Some(input_enter_osc),
                    exit: Some(input_exit_osc),
                    transitions: 0 as *const _,}
    };
pub static mut input_state_osc_string_table: [input_transition; 10] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 6i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 7i32,
                          last: 7i32,
                          handler: None,
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 8i32,
                          last: 23i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 255i32,
                          handler: Some(input_input),
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
unsafe extern "C" fn input_exit_osc(mut ictx: *mut input_ctx) -> () {
    let mut p: *mut u_char = (*ictx).input_buf;
    let mut option: u_int = 0;
    if 0 != (*ictx).flags & 1i32 {
        return
    } else if (*ictx).input_len < 1i32 as libc::c_ulong ||
                  (*p as libc::c_int) < 48 || *p as libc::c_int > 57 {
        return
    } else {
        log_debug(b"%s: \"%s\"\x00" as *const u8 as *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"input_exit_osc\x00")).as_ptr(),
                  p);
        option = 0i32 as u_int;
        while *p as libc::c_int >= 48 && *p as libc::c_int <= 57 {
            let fresh2 = p;
            p = p.offset(1);
            option =
                option.wrapping_mul(10i32 as
                                        libc::c_uint).wrapping_add(*fresh2 as
                                                                       libc::c_uint).wrapping_sub(48
                                                                                                      as
                                                                                                      libc::c_uint)
        }
        if *p as libc::c_int == 59 { p = p.offset(1isize) }
        match option {
            0 | 2 => {
                if 0 != utf8_isvalid(p as *const libc::c_char) {
                    screen_set_title((*ictx).ctx.s, p as *const libc::c_char);
                    server_status_window((*(*ictx).wp).window);
                }
            }
            4 => { input_osc_4((*ictx).wp, p as *const libc::c_char); }
            10 => { input_osc_10((*ictx).wp, p as *const libc::c_char); }
            11 => { input_osc_11((*ictx).wp, p as *const libc::c_char); }
            12 => {
                if 0 != utf8_isvalid(p as *const libc::c_char) &&
                       *p as libc::c_int != 63 {
                    screen_set_cursor_colour((*ictx).ctx.s,
                                             p as *const libc::c_char);
                }
            }
            52 => { input_osc_52((*ictx).wp, p as *const libc::c_char); }
            104 => { input_osc_104((*ictx).wp, p as *const libc::c_char); }
            112 => {
                if *p as libc::c_int == 0 {
                    screen_set_cursor_colour((*ictx).ctx.s,
                                             b"\x00" as *const u8 as
                                                 *const libc::c_char);
                }
            }
            _ => {
                log_debug(b"%s: unknown \'%u\'\x00" as *const u8 as
                              *const libc::c_char,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"input_exit_osc\x00")).as_ptr(),
                          option);
            }
        }
        return;
    };
}
unsafe extern "C" fn input_osc_104(mut wp: *mut window_pane,
                                   mut p: *const libc::c_char) -> () {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: libc::c_long = 0;
    if *p as libc::c_int == 0 {
        window_pane_reset_palette(wp);
        return
    } else {
        s = xstrdup(p);
        copy = s;
        loop  {
            if *s as libc::c_int != 0 {
                idx = strtol(s, &mut s as *mut *mut libc::c_char, 10i32);
                if *s as libc::c_int != 0 && *s as libc::c_int != 59 {
                    break ;
                }
                if idx < 0i32 as libc::c_long || idx >= 256i32 as libc::c_long
                   {
                    break ;
                }
                window_pane_unset_palette(wp, idx as u_int);
                if !(*s as libc::c_int == 59) { continue ; }
                s = s.offset(1isize)
            } else { free(copy as *mut libc::c_void); return }
        }
        log_debug(b"bad OSC 104: %s\x00" as *const u8 as *const libc::c_char,
                  p);
        free(copy as *mut libc::c_void);
        return;
    };
}
unsafe extern "C" fn input_osc_52(mut wp: *mut window_pane,
                                  mut p: *const libc::c_char) -> () {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut out: *mut u_char = 0 as *mut u_char;
    let mut outlen: libc::c_int = 0;
    let mut state: libc::c_int = 0;
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
    state =
        options_get_number(global_options,
                           b"set-clipboard\x00" as *const u8 as
                               *const libc::c_char) as libc::c_int;
    if state != 2i32 {
        return
    } else {
        end = strchr(p, 59);
        if end == 0 as *mut libc::c_void as *mut libc::c_char {
            return
        } else {
            end = end.offset(1isize);
            if *end as libc::c_int == 0 {
                return
            } else {
                len =
                    strlen(end).wrapping_div(4i32 as
                                                 libc::c_ulong).wrapping_mul(3i32
                                                                                 as
                                                                                 libc::c_ulong);
                if len == 0i32 as libc::c_ulong {
                    return
                } else {
                    out = xmalloc(len) as *mut u_char;
                    outlen = b64_pton(end, out, len);
                    if outlen == 1i32.wrapping_neg() {
                        free(out as *mut libc::c_void);
                        return
                    } else {
                        screen_write_start(&mut ctx as *mut screen_write_ctx,
                                           wp, 0 as *mut screen);
                        screen_write_setselection(&mut ctx as
                                                      *mut screen_write_ctx,
                                                  out, outlen as u_int);
                        screen_write_stop(&mut ctx as *mut screen_write_ctx);
                        notify_pane(b"pane-set-clipboard\x00" as *const u8 as
                                        *const libc::c_char, wp);
                        paste_add(out as *mut libc::c_char, outlen as size_t);
                        return;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn input_osc_11(mut wp: *mut window_pane,
                                  mut p: *const libc::c_char) -> () {
    let mut r: u_int = 0;
    let mut g: u_int = 0;
    let mut b: u_int = 0;
    if sscanf(p, b"rgb:%2x/%2x/%2x\x00" as *const u8 as *const libc::c_char,
              &mut r as *mut u_int, &mut g as *mut u_int,
              &mut b as *mut u_int) != 3i32 {
        log_debug(b"bad OSC 11: %s\x00" as *const u8 as *const libc::c_char,
                  p);
        return;
    } else {
        (*wp).colgc.bg =
            colour_join_rgb(r as u_char, g as u_char, b as u_char);
        (*wp).flags |= 1i32;
        return
    };
}
unsafe extern "C" fn input_osc_10(mut wp: *mut window_pane,
                                  mut p: *const libc::c_char) -> () {
    let mut r: u_int = 0;
    let mut g: u_int = 0;
    let mut b: u_int = 0;
    if sscanf(p, b"rgb:%2x/%2x/%2x\x00" as *const u8 as *const libc::c_char,
              &mut r as *mut u_int, &mut g as *mut u_int,
              &mut b as *mut u_int) != 3i32 {
        log_debug(b"bad OSC 10: %s\x00" as *const u8 as *const libc::c_char,
                  p);
        return;
    } else {
        (*wp).colgc.fg =
            colour_join_rgb(r as u_char, g as u_char, b as u_char);
        (*wp).flags |= 1i32;
        return
    };
}
unsafe extern "C" fn input_osc_4(mut wp: *mut window_pane,
                                 mut p: *const libc::c_char) -> () {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: libc::c_long = 0;
    let mut r: u_int = 0;
    let mut g: u_int = 0;
    let mut b: u_int = 0;
    s = xstrdup(p);
    copy = s;
    loop  {
        if s != 0 as *mut libc::c_void as *mut libc::c_char &&
               *s as libc::c_int != 0 {
            idx = strtol(s, &mut next as *mut *mut libc::c_char, 10i32);
            let fresh3 = next;
            next = next.offset(1);
            if *fresh3 as libc::c_int != 59 { break ; }
            if idx < 0i32 as libc::c_long || idx >= 256i32 as libc::c_long {
                break ;
            }
            s =
                strsep(&mut next as *mut *mut libc::c_char,
                       b";\x00" as *const u8 as *const libc::c_char);
            if sscanf(s,
                      b"rgb:%2x/%2x/%2x\x00" as *const u8 as
                          *const libc::c_char, &mut r as *mut u_int,
                      &mut g as *mut u_int, &mut b as *mut u_int) != 3i32 {
                s = next
            } else {
                window_pane_set_palette(wp, idx as u_int,
                                        colour_join_rgb(r as u_char,
                                                        g as u_char,
                                                        b as u_char));
                s = next
            }
        } else { free(copy as *mut libc::c_void); return }
    }
    log_debug(b"bad OSC 4: %s\x00" as *const u8 as *const libc::c_char, p);
    free(copy as *mut libc::c_void);
}
unsafe extern "C" fn input_enter_osc(mut ictx: *mut input_ctx) -> () {
    log_debug(b"%s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"input_enter_osc\x00")).as_ptr());
    input_clear(ictx);
    input_start_timer(ictx);
    (*ictx).last = 1i32.wrapping_neg();
}
pub static mut input_state_csi_enter: input_state =
    unsafe {
        input_state{name:
                        b"csi_enter\x00" as *const u8 as *const libc::c_char,
                    enter: Some(input_clear),
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_csi_enter_table: [input_transition; 14] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 47i32,
                          handler: Some(input_intermediate),
                          state:
                              &input_state_csi_intermediate as
                                  *const input_state,},
         input_transition{first: 48i32,
                          last: 57i32,
                          handler: Some(input_parameter),
                          state:
                              &input_state_csi_parameter as
                                  *const input_state,},
         input_transition{first: 58i32,
                          last: 58i32,
                          handler: Some(input_parameter),
                          state:
                              &input_state_csi_parameter as
                                  *const input_state,},
         input_transition{first: 59i32,
                          last: 59i32,
                          handler: Some(input_parameter),
                          state:
                              &input_state_csi_parameter as
                                  *const input_state,},
         input_transition{first: 60i32,
                          last: 63i32,
                          handler: Some(input_intermediate),
                          state:
                              &input_state_csi_parameter as
                                  *const input_state,},
         input_transition{first: 64i32,
                          last: 126i32,
                          handler: Some(input_csi_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 127i32,
                          last: 255i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
unsafe extern "C" fn input_csi_dispatch(mut ictx: *mut input_ctx)
 -> libc::c_int {
    let mut current_block: u64;
    let mut sctx: *mut screen_write_ctx =
        &mut (*ictx).ctx as *mut screen_write_ctx;
    let mut s: *mut screen = (*sctx).s;
    let mut entry: *mut input_table_entry = 0 as *mut input_table_entry;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut cx: u_int = 0;
    let mut bg: u_int = (*ictx).cell.cell.bg as u_int;
    if 0 != (*ictx).flags & 1i32 {
        return 0i32
    } else {
        log_debug(b"%s: \'%c\' \"%s\" \"%s\"\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                  (*ictx).ch, (*ictx).interm_buf.as_mut_ptr(),
                  (*ictx).param_buf.as_mut_ptr());
        if input_split(ictx) != 0i32 {
            return 0i32
        } else {
            entry =
                bsearch(ictx as *const libc::c_void,
                        input_csi_table.as_ptr() as *const libc::c_void,
                        (::std::mem::size_of::<[input_table_entry; 34]>() as
                             libc::c_ulong).wrapping_div(::std::mem::size_of::<input_table_entry>()
                                                             as
                                                             libc::c_ulong),
                        ::std::mem::size_of::<input_table_entry>() as
                            libc::c_ulong, Some(input_table_compare)) as
                    *mut input_table_entry;
            if entry == 0 as *mut libc::c_void as *mut input_table_entry {
                log_debug(b"%s: unknown \'%c\'\x00" as *const u8 as
                              *const libc::c_char,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                          (*ictx).ch);
                return 0i32
            } else {
                match (*entry).type_0 {
                    0 => {
                        cx = (*s).cx;
                        if cx >
                               (*(*s).grid).sx.wrapping_sub(1i32 as
                                                                libc::c_uint)
                           {
                            cx =
                                (*(*s).grid).sx.wrapping_sub(1i32 as
                                                                 libc::c_uint)
                        }
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if !(n == 1i32.wrapping_neg()) {
                            while cx > 0i32 as libc::c_uint &&
                                      {
                                          let fresh4 = n;
                                          n = n - 1;
                                          fresh4 > 0i32
                                      } {
                                loop  {
                                    cx = cx.wrapping_sub(1);
                                    if !(cx > 0i32 as libc::c_uint &&
                                             0 ==
                                                 *(*s).tabs.offset((cx >>
                                                                        3i32)
                                                                       as
                                                                       isize)
                                                     as libc::c_int &
                                                     1i32 <<
                                                         (cx &
                                                              7i32 as
                                                                  libc::c_uint))
                                       {
                                        break ;
                                    }
                                }
                            }
                            (*s).cx = cx
                        }
                    }
                    3 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_cursorleft(sctx, n as u_int);
                        }
                    }
                    4 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_cursordown(sctx, n as u_int);
                        }
                    }
                    5 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_cursorright(sctx, n as u_int);
                        }
                    }
                    6 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        m = input_get(ictx, 1i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() &&
                               m != 1i32.wrapping_neg() {
                            screen_write_cursormove(sctx, (m - 1i32) as u_int,
                                                    (n - 1i32) as u_int);
                        }
                    }
                    32 => { input_csi_dispatch_winops(ictx); }
                    7 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_cursorup(sctx, n as u_int);
                        }
                    }
                    1 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_carriagereturn(sctx);
                            screen_write_cursordown(sctx, n as u_int);
                        }
                    }
                    2 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_carriagereturn(sctx);
                            screen_write_cursorup(sctx, n as u_int);
                        }
                    }
                    8 => {
                        match input_get(ictx, 0i32 as u_int, 0i32, 0i32) {
                            -1 => { }
                            0 => {
                                current_block = 8529571547960882576;
                                match current_block {
                                    13078237768377568798 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    _ => {
                                        input_reply(ictx,
                                                    b"\x1b[?1;2c\x00" as
                                                        *const u8 as
                                                        *const libc::c_char);
                                    }
                                }
                            }
                            _ => {
                                current_block = 13078237768377568798;
                                match current_block {
                                    13078237768377568798 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    _ => {
                                        input_reply(ictx,
                                                    b"\x1b[?1;2c\x00" as
                                                        *const u8 as
                                                        *const libc::c_char);
                                    }
                                }
                            }
                        }
                    }
                    9 => {
                        match input_get(ictx, 0i32 as u_int, 0i32, 0i32) {
                            -1 => { }
                            0 => {
                                current_block = 3510948148027343312;
                                match current_block {
                                    385272268267597458 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    _ => {
                                        input_reply(ictx,
                                                    b"\x1b[>84;0;0c\x00" as
                                                        *const u8 as
                                                        *const libc::c_char);
                                    }
                                }
                            }
                            _ => {
                                current_block = 385272268267597458;
                                match current_block {
                                    385272268267597458 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    _ => {
                                        input_reply(ictx,
                                                    b"\x1b[>84;0;0c\x00" as
                                                        *const u8 as
                                                        *const libc::c_char);
                                    }
                                }
                            }
                        }
                    }
                    15 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_clearcharacter(sctx, n as u_int, bg);
                        }
                    }
                    10 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_deletecharacter(sctx, n as u_int,
                                                         bg);
                        }
                    }
                    12 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        m =
                            input_get(ictx, 1i32 as u_int, 1i32,
                                      (*(*s).grid).sy as libc::c_int);
                        if n != 1i32.wrapping_neg() &&
                               m != 1i32.wrapping_neg() {
                            screen_write_scrollregion(sctx,
                                                      (n - 1i32) as u_int,
                                                      (m - 1i32) as u_int);
                        }
                    }
                    13 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_deleteline(sctx, n as u_int, bg);
                        }
                    }
                    14 => {
                        match input_get(ictx, 0i32 as u_int, 0i32, 0i32) {
                            -1 => { }
                            5 => {
                                current_block = 10413195123155574014;
                                match current_block {
                                    5745853274343298612 => {
                                        input_reply(ictx,
                                                    b"\x1b[%u;%uR\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    (*s).cy.wrapping_add(1i32
                                                                             as
                                                                             libc::c_uint),
                                                    (*s).cx.wrapping_add(1i32
                                                                             as
                                                                             libc::c_uint));
                                    }
                                    3832674074691517558 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    _ => {
                                        input_reply(ictx,
                                                    b"\x1b[0n\x00" as
                                                        *const u8 as
                                                        *const libc::c_char);
                                    }
                                }
                            }
                            6 => {
                                current_block = 5745853274343298612;
                                match current_block {
                                    5745853274343298612 => {
                                        input_reply(ictx,
                                                    b"\x1b[%u;%uR\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    (*s).cy.wrapping_add(1i32
                                                                             as
                                                                             libc::c_uint),
                                                    (*s).cx.wrapping_add(1i32
                                                                             as
                                                                             libc::c_uint));
                                    }
                                    3832674074691517558 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    _ => {
                                        input_reply(ictx,
                                                    b"\x1b[0n\x00" as
                                                        *const u8 as
                                                        *const libc::c_char);
                                    }
                                }
                            }
                            _ => {
                                current_block = 3832674074691517558;
                                match current_block {
                                    5745853274343298612 => {
                                        input_reply(ictx,
                                                    b"\x1b[%u;%uR\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    (*s).cy.wrapping_add(1i32
                                                                             as
                                                                             libc::c_uint),
                                                    (*s).cx.wrapping_add(1i32
                                                                             as
                                                                             libc::c_uint));
                                    }
                                    3832674074691517558 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    _ => {
                                        input_reply(ictx,
                                                    b"\x1b[0n\x00" as
                                                        *const u8 as
                                                        *const libc::c_char);
                                    }
                                }
                            }
                        }
                    }
                    16 => {
                        match input_get(ictx, 0i32 as u_int, 0i32, 0i32) {
                            -1 => { }
                            0 => {
                                current_block = 10008258937388880952;
                                match current_block {
                                    14231706479693062455 => {
                                        if input_get(ictx, 1i32 as u_int,
                                                     0i32, 0i32) == 0i32 {
                                            screen_write_clearhistory(sctx);
                                        }
                                    }
                                    9122380077187141842 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    6678751553009222334 => {
                                        screen_write_clearscreen(sctx, bg);
                                    }
                                    10008258937388880952 => {
                                        screen_write_clearendofscreen(sctx,
                                                                      bg);
                                    }
                                    _ => {
                                        screen_write_clearstartofscreen(sctx,
                                                                        bg);
                                    }
                                }
                            }
                            1 => {
                                current_block = 13816553114893123275;
                                match current_block {
                                    14231706479693062455 => {
                                        if input_get(ictx, 1i32 as u_int,
                                                     0i32, 0i32) == 0i32 {
                                            screen_write_clearhistory(sctx);
                                        }
                                    }
                                    9122380077187141842 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    6678751553009222334 => {
                                        screen_write_clearscreen(sctx, bg);
                                    }
                                    10008258937388880952 => {
                                        screen_write_clearendofscreen(sctx,
                                                                      bg);
                                    }
                                    _ => {
                                        screen_write_clearstartofscreen(sctx,
                                                                        bg);
                                    }
                                }
                            }
                            2 => {
                                current_block = 6678751553009222334;
                                match current_block {
                                    14231706479693062455 => {
                                        if input_get(ictx, 1i32 as u_int,
                                                     0i32, 0i32) == 0i32 {
                                            screen_write_clearhistory(sctx);
                                        }
                                    }
                                    9122380077187141842 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    6678751553009222334 => {
                                        screen_write_clearscreen(sctx, bg);
                                    }
                                    10008258937388880952 => {
                                        screen_write_clearendofscreen(sctx,
                                                                      bg);
                                    }
                                    _ => {
                                        screen_write_clearstartofscreen(sctx,
                                                                        bg);
                                    }
                                }
                            }
                            3 => {
                                current_block = 14231706479693062455;
                                match current_block {
                                    14231706479693062455 => {
                                        if input_get(ictx, 1i32 as u_int,
                                                     0i32, 0i32) == 0i32 {
                                            screen_write_clearhistory(sctx);
                                        }
                                    }
                                    9122380077187141842 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    6678751553009222334 => {
                                        screen_write_clearscreen(sctx, bg);
                                    }
                                    10008258937388880952 => {
                                        screen_write_clearendofscreen(sctx,
                                                                      bg);
                                    }
                                    _ => {
                                        screen_write_clearstartofscreen(sctx,
                                                                        bg);
                                    }
                                }
                            }
                            _ => {
                                current_block = 9122380077187141842;
                                match current_block {
                                    14231706479693062455 => {
                                        if input_get(ictx, 1i32 as u_int,
                                                     0i32, 0i32) == 0i32 {
                                            screen_write_clearhistory(sctx);
                                        }
                                    }
                                    9122380077187141842 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    6678751553009222334 => {
                                        screen_write_clearscreen(sctx, bg);
                                    }
                                    10008258937388880952 => {
                                        screen_write_clearendofscreen(sctx,
                                                                      bg);
                                    }
                                    _ => {
                                        screen_write_clearstartofscreen(sctx,
                                                                        bg);
                                    }
                                }
                            }
                        }
                    }
                    17 => {
                        match input_get(ictx, 0i32 as u_int, 0i32, 0i32) {
                            -1 => { }
                            0 => {
                                current_block = 10716407457819781312;
                                match current_block {
                                    14783241762875736529 => {
                                        screen_write_clearstartofline(sctx,
                                                                      bg);
                                    }
                                    10716407457819781312 => {
                                        screen_write_clearendofline(sctx, bg);
                                    }
                                    15425117094043254452 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    _ => { screen_write_clearline(sctx, bg); }
                                }
                            }
                            1 => {
                                current_block = 14783241762875736529;
                                match current_block {
                                    14783241762875736529 => {
                                        screen_write_clearstartofline(sctx,
                                                                      bg);
                                    }
                                    10716407457819781312 => {
                                        screen_write_clearendofline(sctx, bg);
                                    }
                                    15425117094043254452 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    _ => { screen_write_clearline(sctx, bg); }
                                }
                            }
                            2 => {
                                current_block = 7538114737374167612;
                                match current_block {
                                    14783241762875736529 => {
                                        screen_write_clearstartofline(sctx,
                                                                      bg);
                                    }
                                    10716407457819781312 => {
                                        screen_write_clearendofline(sctx, bg);
                                    }
                                    15425117094043254452 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    _ => { screen_write_clearline(sctx, bg); }
                                }
                            }
                            _ => {
                                current_block = 15425117094043254452;
                                match current_block {
                                    14783241762875736529 => {
                                        screen_write_clearstartofline(sctx,
                                                                      bg);
                                    }
                                    10716407457819781312 => {
                                        screen_write_clearendofline(sctx, bg);
                                    }
                                    15425117094043254452 => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                    _ => { screen_write_clearline(sctx, bg); }
                                }
                            }
                        }
                    }
                    18 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_cursormove(sctx, (n - 1i32) as u_int,
                                                    (*s).cy);
                        }
                    }
                    19 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_insertcharacter(sctx, n as u_int,
                                                         bg);
                        }
                    }
                    20 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_insertline(sctx, n as u_int, bg);
                        }
                    }
                    22 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if !(n == 1i32.wrapping_neg()) {
                            if !((*ictx).last == 1i32.wrapping_neg()) {
                                (*ictx).ch = (*ictx).last;
                                i = 0i32;
                                while i < n { input_print(ictx); i += 1 }
                            }
                        }
                    }
                    21 => {
                        memcpy(&mut (*ictx).cell as *mut input_cell as
                                   *mut libc::c_void,
                               &mut (*ictx).old_cell as *mut input_cell as
                                   *const libc::c_void,
                               ::std::mem::size_of::<input_cell>() as
                                   libc::c_ulong);
                        screen_write_cursormove(sctx, (*ictx).old_cx,
                                                (*ictx).old_cy);
                    }
                    23 => { input_csi_dispatch_rm(ictx); }
                    24 => { input_csi_dispatch_rm_private(ictx); }
                    25 => {
                        memcpy(&mut (*ictx).old_cell as *mut input_cell as
                                   *mut libc::c_void,
                               &mut (*ictx).cell as *mut input_cell as
                                   *const libc::c_void,
                               ::std::mem::size_of::<input_cell>() as
                                   libc::c_ulong);
                        (*ictx).old_cx = (*s).cx;
                        (*ictx).old_cy = (*s).cy
                    }
                    26 => { input_csi_dispatch_sgr(ictx); }
                    27 => { input_csi_dispatch_sm(ictx); }
                    28 => { input_csi_dispatch_sm_private(ictx); }
                    29 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_scrollup(sctx, n as u_int, bg);
                        }
                    }
                    30 => {
                        match input_get(ictx, 0i32 as u_int, 0i32, 0i32) {
                            -1 => { }
                            0 => {
                                current_block = 9427735788290307261;
                                match current_block {
                                    9427735788290307261 => {
                                        if (*s).cx < (*(*s).grid).sx {
                                            let ref mut fresh5 =
                                                *(*s).tabs.offset(((*s).cx >>
                                                                       3i32)
                                                                      as
                                                                      isize);
                                            *fresh5 =
                                                (*fresh5 as libc::c_int &
                                                     !(1i32 <<
                                                           ((*s).cx &
                                                                7i32 as
                                                                    libc::c_uint)))
                                                    as bitstr_t
                                        }
                                    }
                                    1924505913685386279 => {
                                        loop  {
                                            let mut _name: *mut bitstr_t =
                                                (*s).tabs;
                                            let mut _start: libc::c_int =
                                                0i32;
                                            let mut _stop: libc::c_int =
                                                (*(*s).grid).sx.wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_uint)
                                                    as libc::c_int;
                                            while _start <= _stop {
                                                let ref mut fresh6 =
                                                    *_name.offset((_start >>
                                                                       3i32)
                                                                      as
                                                                      isize);
                                                *fresh6 =
                                                    (*fresh6 as libc::c_int &
                                                         !(1i32 <<
                                                               (_start &
                                                                    7i32))) as
                                                        bitstr_t;
                                                _start += 1
                                            }
                                            if !(0 != 0i32) { break ; }
                                        }
                                    }
                                    _ => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                }
                            }
                            3 => {
                                current_block = 1924505913685386279;
                                match current_block {
                                    9427735788290307261 => {
                                        if (*s).cx < (*(*s).grid).sx {
                                            let ref mut fresh5 =
                                                *(*s).tabs.offset(((*s).cx >>
                                                                       3i32)
                                                                      as
                                                                      isize);
                                            *fresh5 =
                                                (*fresh5 as libc::c_int &
                                                     !(1i32 <<
                                                           ((*s).cx &
                                                                7i32 as
                                                                    libc::c_uint)))
                                                    as bitstr_t
                                        }
                                    }
                                    1924505913685386279 => {
                                        loop  {
                                            let mut _name: *mut bitstr_t =
                                                (*s).tabs;
                                            let mut _start: libc::c_int =
                                                0i32;
                                            let mut _stop: libc::c_int =
                                                (*(*s).grid).sx.wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_uint)
                                                    as libc::c_int;
                                            while _start <= _stop {
                                                let ref mut fresh6 =
                                                    *_name.offset((_start >>
                                                                       3i32)
                                                                      as
                                                                      isize);
                                                *fresh6 =
                                                    (*fresh6 as libc::c_int &
                                                         !(1i32 <<
                                                               (_start &
                                                                    7i32))) as
                                                        bitstr_t;
                                                _start += 1
                                            }
                                            if !(0 != 0i32) { break ; }
                                        }
                                    }
                                    _ => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                }
                            }
                            _ => {
                                current_block = 3375814408018429252;
                                match current_block {
                                    9427735788290307261 => {
                                        if (*s).cx < (*(*s).grid).sx {
                                            let ref mut fresh5 =
                                                *(*s).tabs.offset(((*s).cx >>
                                                                       3i32)
                                                                      as
                                                                      isize);
                                            *fresh5 =
                                                (*fresh5 as libc::c_int &
                                                     !(1i32 <<
                                                           ((*s).cx &
                                                                7i32 as
                                                                    libc::c_uint)))
                                                    as bitstr_t
                                        }
                                    }
                                    1924505913685386279 => {
                                        loop  {
                                            let mut _name: *mut bitstr_t =
                                                (*s).tabs;
                                            let mut _start: libc::c_int =
                                                0i32;
                                            let mut _stop: libc::c_int =
                                                (*(*s).grid).sx.wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_uint)
                                                    as libc::c_int;
                                            while _start <= _stop {
                                                let ref mut fresh6 =
                                                    *_name.offset((_start >>
                                                                       3i32)
                                                                      as
                                                                      isize);
                                                *fresh6 =
                                                    (*fresh6 as libc::c_int &
                                                         !(1i32 <<
                                                               (_start &
                                                                    7i32))) as
                                                        bitstr_t;
                                                _start += 1
                                            }
                                            if !(0 != 0i32) { break ; }
                                        }
                                    }
                                    _ => {
                                        log_debug(b"%s: unknown \'%c\'\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  (*::std::mem::transmute::<&[u8; 19],
                                                                            &[libc::c_char; 19]>(b"input_csi_dispatch\x00")).as_ptr(),
                                                  (*ictx).ch);
                                    }
                                }
                            }
                        }
                    }
                    31 => {
                        n = input_get(ictx, 0i32 as u_int, 1i32, 1i32);
                        if n != 1i32.wrapping_neg() {
                            screen_write_cursormove(sctx, (*s).cx,
                                                    (n - 1i32) as u_int);
                        }
                    }
                    11 => {
                        n = input_get(ictx, 0i32 as u_int, 0i32, 0i32);
                        if n != 1i32.wrapping_neg() {
                            screen_set_cursor_style(s, n as u_int);
                        }
                    }
                    _ => { }
                }
                (*ictx).last = 1i32.wrapping_neg();
                return 0i32
            }
        }
    };
}
unsafe extern "C" fn input_get(mut ictx: *mut input_ctx, mut validx: u_int,
                               mut minval: libc::c_int,
                               mut defval: libc::c_int) -> libc::c_int {
    let mut ip: *mut input_param = 0 as *mut input_param;
    let mut retval: libc::c_int = 0;
    if validx >= (*ictx).param_list_len {
        return defval
    } else {
        ip = &mut (*ictx).param_list[validx as usize] as *mut input_param;
        if (*ip).type_0 as libc::c_uint ==
               INPUT_MISSING as libc::c_int as libc::c_uint {
            return defval
        } else if (*ip).type_0 as libc::c_uint ==
                      INPUT_STRING as libc::c_int as libc::c_uint {
            return 1i32.wrapping_neg()
        } else {
            retval = (*ip).unnamed.num;
            if retval < minval { return minval } else { return retval }
        }
    };
}
unsafe extern "C" fn input_csi_dispatch_sm_private(mut ictx: *mut input_ctx)
 -> () {
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut i: u_int = 0;
    i = 0i32 as u_int;
    while i < (*ictx).param_list_len {
        match input_get(ictx, i, 0i32, 1i32.wrapping_neg()) {
            -1 => { }
            1 => {
                screen_write_mode_set(&mut (*ictx).ctx as
                                          *mut screen_write_ctx, 4i32);
            }
            3 => {
                screen_write_cursormove(&mut (*ictx).ctx as
                                            *mut screen_write_ctx,
                                        0i32 as u_int, 0i32 as u_int);
                screen_write_clearscreen(&mut (*ictx).ctx as
                                             *mut screen_write_ctx,
                                         (*ictx).cell.cell.bg as u_int);
            }
            7 => {
                screen_write_mode_set(&mut (*ictx).ctx as
                                          *mut screen_write_ctx, 16i32);
            }
            12 => {
                screen_write_mode_set(&mut (*ictx).ctx as
                                          *mut screen_write_ctx, 128i32);
            }
            25 => {
                screen_write_mode_set(&mut (*ictx).ctx as
                                          *mut screen_write_ctx, 1i32);
            }
            1000 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx,
                                        32i32 | 64i32 | 4096i32);
                screen_write_mode_set(&mut (*ictx).ctx as
                                          *mut screen_write_ctx, 32i32);
            }
            1002 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx,
                                        32i32 | 64i32 | 4096i32);
                screen_write_mode_set(&mut (*ictx).ctx as
                                          *mut screen_write_ctx, 64i32);
            }
            1003 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx,
                                        32i32 | 64i32 | 4096i32);
                screen_write_mode_set(&mut (*ictx).ctx as
                                          *mut screen_write_ctx, 4096i32);
            }
            1004 => {
                if !(0 != (*(*ictx).ctx.s).mode & 2048i32) {
                    screen_write_mode_set(&mut (*ictx).ctx as
                                              *mut screen_write_ctx, 2048i32);
                    (*wp).flags |= 32i32
                }
            }
            1005 => {
                screen_write_mode_set(&mut (*ictx).ctx as
                                          *mut screen_write_ctx, 256i32);
            }
            1006 => {
                screen_write_mode_set(&mut (*ictx).ctx as
                                          *mut screen_write_ctx, 512i32);
            }
            47 | 1047 => {
                window_pane_alternate_on(wp,
                                         &mut (*ictx).cell.cell as
                                             *mut grid_cell, 0i32);
            }
            1049 => {
                window_pane_alternate_on(wp,
                                         &mut (*ictx).cell.cell as
                                             *mut grid_cell, 1i32);
            }
            2004 => {
                screen_write_mode_set(&mut (*ictx).ctx as
                                          *mut screen_write_ctx, 1024i32);
            }
            _ => {
                log_debug(b"%s: unknown \'%c\'\x00" as *const u8 as
                              *const libc::c_char,
                          (*::std::mem::transmute::<&[u8; 30],
                                                    &[libc::c_char; 30]>(b"input_csi_dispatch_sm_private\x00")).as_ptr(),
                          (*ictx).ch);
            }
        }
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn input_csi_dispatch_sm(mut ictx: *mut input_ctx) -> () {
    let mut i: u_int = 0;
    i = 0i32 as u_int;
    while i < (*ictx).param_list_len {
        match input_get(ictx, i, 0i32, 1i32.wrapping_neg()) {
            -1 => { }
            4 => {
                screen_write_mode_set(&mut (*ictx).ctx as
                                          *mut screen_write_ctx, 2i32);
            }
            34 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx, 128i32);
            }
            _ => {
                log_debug(b"%s: unknown \'%c\'\x00" as *const u8 as
                              *const libc::c_char,
                          (*::std::mem::transmute::<&[u8; 22],
                                                    &[libc::c_char; 22]>(b"input_csi_dispatch_sm\x00")).as_ptr(),
                          (*ictx).ch);
            }
        }
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn input_csi_dispatch_sgr(mut ictx: *mut input_ctx) -> () {
    let mut current_block: u64;
    let mut gc: *mut grid_cell = &mut (*ictx).cell.cell as *mut grid_cell;
    let mut i: u_int = 0;
    let mut n: libc::c_int = 0;
    if (*ictx).param_list_len == 0i32 as libc::c_uint {
        memcpy(gc as *mut libc::c_void,
               &grid_default_cell as *const grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        return
    } else {
        i = 0i32 as u_int;
        while i < (*ictx).param_list_len {
            if (*ictx).param_list[i as usize].type_0 as libc::c_uint ==
                   INPUT_STRING as libc::c_int as libc::c_uint {
                input_csi_dispatch_sgr_colon(ictx, i);
            } else {
                n = input_get(ictx, i, 0i32, 0i32);
                if !(n == 1i32.wrapping_neg()) {
                    if n == 38i32 || n == 48i32 {
                        i = i.wrapping_add(1);
                        match input_get(ictx, i, 0i32, 1i32.wrapping_neg()) {
                            2 => {
                                current_block = 1487533989625441528;
                                match current_block {
                                    12144037074258575129 => {
                                        input_csi_dispatch_sgr_256(ictx, n,
                                                                   &mut i as
                                                                       *mut u_int);
                                    }
                                    _ => {
                                        input_csi_dispatch_sgr_rgb(ictx, n,
                                                                   &mut i as
                                                                       *mut u_int);
                                    }
                                }
                            }
                            5 => {
                                current_block = 12144037074258575129;
                                match current_block {
                                    12144037074258575129 => {
                                        input_csi_dispatch_sgr_256(ictx, n,
                                                                   &mut i as
                                                                       *mut u_int);
                                    }
                                    _ => {
                                        input_csi_dispatch_sgr_rgb(ictx, n,
                                                                   &mut i as
                                                                       *mut u_int);
                                    }
                                }
                            }
                            _ => { }
                        }
                    } else {
                        match n {
                            0 => {
                                current_block = 14648179422885671807;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            1 => {
                                current_block = 3624320934322378446;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            2 => {
                                current_block = 8843785884804001563;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            3 => {
                                current_block = 14262552831985286233;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            4 => {
                                current_block = 16073863222772817704;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            5 => {
                                current_block = 2020259329779843065;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            7 => {
                                current_block = 1348679741070379233;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            8 => {
                                current_block = 16374999659469652627;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            9 => {
                                current_block = 17764639755544694524;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            22 => {
                                current_block = 485759478260003476;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            23 => {
                                current_block = 11718679158976737670;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            24 => {
                                current_block = 6629347938574934625;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            25 => {
                                current_block = 9932722462317761039;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            27 => {
                                current_block = 3866597541427977022;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            28 => {
                                current_block = 16695834369042141235;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            29 => {
                                current_block = 8608417946254211545;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 => {
                                current_block = 4429232765501753682;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            39 => {
                                current_block = 7679224362344455344;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 => {
                                current_block = 11742859648667696368;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            49 => {
                                current_block = 3733621521558651155;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 => {
                                current_block = 7896587573582807180;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 => {
                                current_block = 3639221797090915414;
                                match current_block {
                                    3624320934322378446 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 1i32)
                                                as u_short
                                    }
                                    485759478260003476 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !(1i32 | 2i32)) as u_short
                                    }
                                    16695834369042141235 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !32i32) as u_short
                                    }
                                    3639221797090915414 => {
                                        (*gc).bg = n - 10i32
                                    }
                                    11742859648667696368 => {
                                        (*gc).bg = n - 40i32
                                    }
                                    4429232765501753682 => {
                                        (*gc).fg = n - 30i32
                                    }
                                    14648179422885671807 => {
                                        memcpy(gc as *mut libc::c_void,
                                               &grid_default_cell as
                                                   *const grid_cell as
                                                   *const libc::c_void,
                                               ::std::mem::size_of::<grid_cell>()
                                                   as libc::c_ulong);
                                    }
                                    3866597541427977022 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !16i32) as u_short
                                    }
                                    16073863222772817704 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 4i32)
                                                as u_short
                                    }
                                    9932722462317761039 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !8i32) as u_short
                                    }
                                    14262552831985286233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 64i32) as u_short
                                    }
                                    7896587573582807180 => { (*gc).fg = n }
                                    1348679741070379233 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 16i32) as u_short
                                    }
                                    8608417946254211545 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !256i32) as u_short
                                    }
                                    7679224362344455344 => { (*gc).fg = 8i32 }
                                    8843785884804001563 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 2i32)
                                                as u_short
                                    }
                                    2020259329779843065 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int | 8i32)
                                                as u_short
                                    }
                                    16374999659469652627 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 32i32) as u_short
                                    }
                                    17764639755544694524 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int |
                                                 256i32) as u_short
                                    }
                                    6629347938574934625 => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !4i32) as u_short
                                    }
                                    3733621521558651155 => { (*gc).bg = 8i32 }
                                    _ => {
                                        (*gc).attr =
                                            ((*gc).attr as libc::c_int &
                                                 !64i32) as u_short
                                    }
                                }
                            }
                            _ => { }
                        }
                    }
                }
            }
            i = i.wrapping_add(1)
        }
        return;
    };
}
unsafe extern "C" fn input_csi_dispatch_sgr_256(mut ictx: *mut input_ctx,
                                                mut fgbg: libc::c_int,
                                                mut i: *mut u_int) -> () {
    let mut c: libc::c_int = 0;
    c =
        input_get(ictx, (*i).wrapping_add(1i32 as libc::c_uint), 0i32,
                  1i32.wrapping_neg());
    if 0 != input_csi_dispatch_sgr_256_do(ictx, fgbg, c) {
        *i = (*i).wrapping_add(1)
    };
}
unsafe extern "C" fn input_csi_dispatch_sgr_256_do(mut ictx: *mut input_ctx,
                                                   mut fgbg: libc::c_int,
                                                   mut c: libc::c_int)
 -> libc::c_int {
    let mut gc: *mut grid_cell = &mut (*ictx).cell.cell as *mut grid_cell;
    if c == 1i32.wrapping_neg() || c > 255i32 {
        if fgbg == 38i32 {
            (*gc).fg = 8i32
        } else if fgbg == 48i32 { (*gc).bg = 8i32 }
    } else if fgbg == 38i32 {
        (*gc).fg = c | 16777216i32
    } else if fgbg == 48i32 { (*gc).bg = c | 16777216i32 }
    return 1i32;
}
unsafe extern "C" fn input_csi_dispatch_sgr_rgb(mut ictx: *mut input_ctx,
                                                mut fgbg: libc::c_int,
                                                mut i: *mut u_int) -> () {
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    r =
        input_get(ictx, (*i).wrapping_add(1i32 as libc::c_uint), 0i32,
                  1i32.wrapping_neg());
    g =
        input_get(ictx, (*i).wrapping_add(2i32 as libc::c_uint), 0i32,
                  1i32.wrapping_neg());
    b =
        input_get(ictx, (*i).wrapping_add(3i32 as libc::c_uint), 0i32,
                  1i32.wrapping_neg());
    if 0 != input_csi_dispatch_sgr_rgb_do(ictx, fgbg, r, g, b) {
        *i =
            (*i as libc::c_uint).wrapping_add(3i32 as libc::c_uint) as u_int
                as u_int
    };
}
unsafe extern "C" fn input_csi_dispatch_sgr_rgb_do(mut ictx: *mut input_ctx,
                                                   mut fgbg: libc::c_int,
                                                   mut r: libc::c_int,
                                                   mut g: libc::c_int,
                                                   mut b: libc::c_int)
 -> libc::c_int {
    let mut gc: *mut grid_cell = &mut (*ictx).cell.cell as *mut grid_cell;
    if r == 1i32.wrapping_neg() || r > 255i32 {
        return 0i32
    } else if g == 1i32.wrapping_neg() || g > 255i32 {
        return 0i32
    } else if b == 1i32.wrapping_neg() || b > 255i32 {
        return 0i32
    } else {
        if fgbg == 38i32 {
            (*gc).fg = colour_join_rgb(r as u_char, g as u_char, b as u_char)
        } else if fgbg == 48i32 {
            (*gc).bg = colour_join_rgb(r as u_char, g as u_char, b as u_char)
        }
        return 1i32
    };
}
unsafe extern "C" fn input_csi_dispatch_sgr_colon(mut ictx: *mut input_ctx,
                                                  mut i: u_int) -> () {
    let mut current_block: u64;
    let mut s: *mut libc::c_char =
        (*ictx).param_list[i as usize].unnamed.str_0;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: [libc::c_int; 8] = [0; 8];
    let mut n: u_int = 0;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    n = 0i32 as u_int;
    while (n as libc::c_ulong) <
              (::std::mem::size_of::<[libc::c_int; 8]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_int>()
                                                   as libc::c_ulong) {
        p[n as usize] = 1i32.wrapping_neg();
        n = n.wrapping_add(1)
    }
    n = 0i32 as u_int;
    copy = xstrdup(s);
    ptr = copy;
    loop  {
        out =
            strsep(&mut ptr as *mut *mut libc::c_char,
                   b":\x00" as *const u8 as *const libc::c_char);
        if out != 0 as *mut libc::c_void as *mut libc::c_char {
            if *out as libc::c_int != 0 {
                let fresh7 = n;
                n = n.wrapping_add(1);
                p[fresh7 as usize] =
                    strtonum(out, 0i32 as libc::c_longlong,
                             2147483647i32 as libc::c_longlong,
                             &mut errstr as *mut *const libc::c_char) as
                        libc::c_int;
                if errstr != 0 as *mut libc::c_void as *const libc::c_char ||
                       n as libc::c_ulong ==
                           (::std::mem::size_of::<[libc::c_int; 8]>() as
                                libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_int>()
                                                                as
                                                                libc::c_ulong)
                   {
                    free(copy as *mut libc::c_void);
                    return
                }
            }
            log_debug(b"%s: %u = %d\x00" as *const u8 as *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"input_csi_dispatch_sgr_colon\x00")).as_ptr(),
                      n.wrapping_sub(1i32 as libc::c_uint),
                      p[n.wrapping_sub(1i32 as libc::c_uint) as usize]);
        } else {
            free(copy as *mut libc::c_void);
            if n == 0i32 as libc::c_uint ||
                   p[0usize] != 38i32 && p[0usize] != 48i32 {
                current_block = 10879442775620481940;
                break ;
            } else { current_block = 4906268039856690917; break ; }
        }
    }
    match current_block {
        10879442775620481940 => { return }
        _ => {
            if p[1usize] == 1i32.wrapping_neg() {
                i = 2i32 as u_int
            } else { i = 1i32 as u_int }
            match p[i as usize] {
                2 => {
                    if !(n < i.wrapping_add(4i32 as libc::c_uint)) {
                        input_csi_dispatch_sgr_rgb_do(ictx, p[0usize],
                                                      p[i.wrapping_add(1i32 as
                                                                           libc::c_uint)
                                                            as usize],
                                                      p[i.wrapping_add(2i32 as
                                                                           libc::c_uint)
                                                            as usize],
                                                      p[i.wrapping_add(3i32 as
                                                                           libc::c_uint)
                                                            as usize]);
                    }
                }
                5 => {
                    if !(n < i.wrapping_add(2i32 as libc::c_uint)) {
                        input_csi_dispatch_sgr_256_do(ictx, p[0usize],
                                                      p[i.wrapping_add(1i32 as
                                                                           libc::c_uint)
                                                            as usize]);
                    }
                }
                _ => { }
            }
            return;
        }
    };
}
unsafe extern "C" fn input_csi_dispatch_rm_private(mut ictx: *mut input_ctx)
 -> () {
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut i: u_int = 0;
    i = 0i32 as u_int;
    while i < (*ictx).param_list_len {
        match input_get(ictx, i, 0i32, 1i32.wrapping_neg()) {
            -1 => { }
            1 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx, 4i32);
            }
            3 => {
                screen_write_cursormove(&mut (*ictx).ctx as
                                            *mut screen_write_ctx,
                                        0i32 as u_int, 0i32 as u_int);
                screen_write_clearscreen(&mut (*ictx).ctx as
                                             *mut screen_write_ctx,
                                         (*ictx).cell.cell.bg as u_int);
            }
            7 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx, 16i32);
            }
            12 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx, 128i32);
            }
            25 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx, 1i32);
            }
            1000 | 1001 | 1002 | 1003 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx,
                                        32i32 | 64i32 | 4096i32);
            }
            1004 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx, 2048i32);
            }
            1005 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx, 256i32);
            }
            1006 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx, 512i32);
            }
            47 | 1047 => {
                window_pane_alternate_off(wp,
                                          &mut (*ictx).cell.cell as
                                              *mut grid_cell, 0i32);
            }
            1049 => {
                window_pane_alternate_off(wp,
                                          &mut (*ictx).cell.cell as
                                              *mut grid_cell, 1i32);
            }
            2004 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx, 1024i32);
            }
            _ => {
                log_debug(b"%s: unknown \'%c\'\x00" as *const u8 as
                              *const libc::c_char,
                          (*::std::mem::transmute::<&[u8; 30],
                                                    &[libc::c_char; 30]>(b"input_csi_dispatch_rm_private\x00")).as_ptr(),
                          (*ictx).ch);
            }
        }
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn input_csi_dispatch_rm(mut ictx: *mut input_ctx) -> () {
    let mut i: u_int = 0;
    i = 0i32 as u_int;
    while i < (*ictx).param_list_len {
        match input_get(ictx, i, 0i32, 1i32.wrapping_neg()) {
            -1 => { }
            4 => {
                screen_write_mode_clear(&mut (*ictx).ctx as
                                            *mut screen_write_ctx, 2i32);
            }
            34 => {
                screen_write_mode_set(&mut (*ictx).ctx as
                                          *mut screen_write_ctx, 128i32);
            }
            _ => {
                log_debug(b"%s: unknown \'%c\'\x00" as *const u8 as
                              *const libc::c_char,
                          (*::std::mem::transmute::<&[u8; 22],
                                                    &[libc::c_char; 22]>(b"input_csi_dispatch_rm\x00")).as_ptr(),
                          (*ictx).ch);
            }
        }
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn input_csi_dispatch_winops(mut ictx: *mut input_ctx)
 -> () {
    let mut current_block: u64;
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    m = 0i32;
    loop  {
        n = input_get(ictx, m as u_int, 0i32, 1i32.wrapping_neg());
        if n != 1i32.wrapping_neg() {
            match n {
                1 | 2 | 5 | 6 | 7 | 11 | 13 | 14 | 19 | 20 | 21 | 24 => {
                    current_block = 16559507199688588974;
                }
                3 | 4 | 8 => {
                    m += 1;
                    if input_get(ictx, m as u_int, 0i32, 1i32.wrapping_neg())
                           == 1i32.wrapping_neg() {
                        return
                    } else { current_block = 5662140698440578959; }
                }
                9 | 10 => { current_block = 5662140698440578959; }
                22 => {
                    m += 1;
                    match input_get(ictx, m as u_int, 0i32,
                                    1i32.wrapping_neg()) {
                        -1 => {
                            current_block = 2659015382916962265;
                            match current_block {
                                8870762555684084047 => {
                                    screen_push_title((*ictx).ctx.s);
                                }
                                _ => { return }
                            }
                            current_block = 16559507199688588974;
                        }
                        0 | 2 => {
                            current_block = 8870762555684084047;
                            match current_block {
                                8870762555684084047 => {
                                    screen_push_title((*ictx).ctx.s);
                                }
                                _ => { return }
                            }
                            current_block = 16559507199688588974;
                        }
                        _ => { current_block = 16559507199688588974; }
                    }
                }
                23 => {
                    m += 1;
                    match input_get(ictx, m as u_int, 0i32,
                                    1i32.wrapping_neg()) {
                        -1 => {
                            current_block = 12572443718578411601;
                            match current_block {
                                12572443718578411601 => { return }
                                _ => {
                                    screen_pop_title((*ictx).ctx.s);
                                    server_status_window((*(*ictx).wp).window);
                                }
                            }
                            current_block = 16559507199688588974;
                        }
                        0 | 2 => {
                            current_block = 5123873023739312379;
                            match current_block {
                                12572443718578411601 => { return }
                                _ => {
                                    screen_pop_title((*ictx).ctx.s);
                                    server_status_window((*(*ictx).wp).window);
                                }
                            }
                            current_block = 16559507199688588974;
                        }
                        _ => { current_block = 16559507199688588974; }
                    }
                }
                18 => {
                    input_reply(ictx,
                                b"\x1b[8;%u;%ut\x00" as *const u8 as
                                    *const libc::c_char, (*wp).sy, (*wp).sx);
                    current_block = 16559507199688588974;
                }
                _ => {
                    log_debug(b"%s: unknown \'%c\'\x00" as *const u8 as
                                  *const libc::c_char,
                              (*::std::mem::transmute::<&[u8; 26],
                                                        &[libc::c_char; 26]>(b"input_csi_dispatch_winops\x00")).as_ptr(),
                              (*ictx).ch);
                    current_block = 16559507199688588974;
                }
            }
            match current_block {
                5662140698440578959 => {
                    m += 1;
                    if input_get(ictx, m as u_int, 0i32, 1i32.wrapping_neg())
                           == 1i32.wrapping_neg() {
                        return
                    }
                }
                _ => { }
            }
            m += 1
        } else { return; }
    };
}
static mut input_csi_table: [input_table_entry; 34] =
    unsafe {
        [input_table_entry{ch: 64,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_ICH as libc::c_int,},
         input_table_entry{ch: 65,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_CUU as libc::c_int,},
         input_table_entry{ch: 66,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_CUD as libc::c_int,},
         input_table_entry{ch: 67,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_CUF as libc::c_int,},
         input_table_entry{ch: 68,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_CUB as libc::c_int,},
         input_table_entry{ch: 69,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_CNL as libc::c_int,},
         input_table_entry{ch: 70,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_CPL as libc::c_int,},
         input_table_entry{ch: 71,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_HPA as libc::c_int,},
         input_table_entry{ch: 72,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_CUP as libc::c_int,},
         input_table_entry{ch: 74,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_ED as libc::c_int,},
         input_table_entry{ch: 75,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_EL as libc::c_int,},
         input_table_entry{ch: 76,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_IL as libc::c_int,},
         input_table_entry{ch: 77,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_DL as libc::c_int,},
         input_table_entry{ch: 80,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_DCH as libc::c_int,},
         input_table_entry{ch: 83,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_SU as libc::c_int,},
         input_table_entry{ch: 88,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_ECH as libc::c_int,},
         input_table_entry{ch: 90,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_CBT as libc::c_int,},
         input_table_entry{ch: 98,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_REP as libc::c_int,},
         input_table_entry{ch: 99,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_DA as libc::c_int,},
         input_table_entry{ch: 99,
                           interm:
                               b">\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_DA_TWO as libc::c_int,},
         input_table_entry{ch: 100,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_VPA as libc::c_int,},
         input_table_entry{ch: 102,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_CUP as libc::c_int,},
         input_table_entry{ch: 103,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_TBC as libc::c_int,},
         input_table_entry{ch: 104,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_SM as libc::c_int,},
         input_table_entry{ch: 104,
                           interm:
                               b"?\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_SM_PRIVATE as libc::c_int,},
         input_table_entry{ch: 108,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_RM as libc::c_int,},
         input_table_entry{ch: 108,
                           interm:
                               b"?\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_RM_PRIVATE as libc::c_int,},
         input_table_entry{ch: 109,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_SGR as libc::c_int,},
         input_table_entry{ch: 110,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_DSR as libc::c_int,},
         input_table_entry{ch: 113,
                           interm:
                               b" \x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_DECSCUSR as libc::c_int,},
         input_table_entry{ch: 114,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_DECSTBM as libc::c_int,},
         input_table_entry{ch: 115,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_SCP as libc::c_int,},
         input_table_entry{ch: 116,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_WINOPS as libc::c_int,},
         input_table_entry{ch: 117,
                           interm:
                               b"\x00" as *const u8 as *const libc::c_char,
                           type_0: INPUT_CSI_RCP as libc::c_int,}]
    };
unsafe extern "C" fn input_split(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip: *mut input_param = 0 as *mut input_param;
    let mut i: u_int = 0;
    i = 0i32 as u_int;
    while i < (*ictx).param_list_len {
        if (*ictx).param_list[i as usize].type_0 as libc::c_uint ==
               INPUT_STRING as libc::c_int as libc::c_uint {
            free((*ictx).param_list[i as usize].unnamed.str_0 as
                     *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    }
    (*ictx).param_list_len = 0i32 as u_int;
    if (*ictx).param_len == 0i32 as libc::c_ulong {
        return 0i32
    } else {
        ip = &mut (*ictx).param_list[0usize] as *mut input_param;
        ptr = (*ictx).param_buf.as_mut_ptr() as *mut libc::c_char;
        loop  {
            out =
                strsep(&mut ptr as *mut *mut libc::c_char,
                       b";\x00" as *const u8 as *const libc::c_char);
            if out != 0 as *mut libc::c_void as *mut libc::c_char {
                if *out as libc::c_int == 0 {
                    (*ip).type_0 = INPUT_MISSING
                } else if strchr(out, 58) !=
                              0 as *mut libc::c_void as *mut libc::c_char {
                    (*ip).type_0 = INPUT_STRING;
                    (*ip).unnamed.str_0 = xstrdup(out)
                } else {
                    (*ip).type_0 = INPUT_NUMBER;
                    (*ip).unnamed.num =
                        strtonum(out, 0i32 as libc::c_longlong,
                                 2147483647i32 as libc::c_longlong,
                                 &mut errstr as *mut *const libc::c_char) as
                            libc::c_int;
                    if errstr != 0 as *mut libc::c_void as *const libc::c_char
                       {
                        return 1i32.wrapping_neg()
                    }
                }
                (*ictx).param_list_len =
                    (*ictx).param_list_len.wrapping_add(1);
                ip =
                    &mut (*ictx).param_list[(*ictx).param_list_len as usize]
                        as *mut input_param;
                if !((*ictx).param_list_len as libc::c_ulong ==
                         (::std::mem::size_of::<[input_param; 24]>() as
                              libc::c_ulong).wrapping_div(::std::mem::size_of::<input_param>()
                                                              as
                                                              libc::c_ulong))
                   {
                    continue ;
                }
                return 1i32.wrapping_neg()
            } else { i = 0i32 as u_int; break ; }
        }
        while i < (*ictx).param_list_len {
            ip = &mut (*ictx).param_list[i as usize] as *mut input_param;
            if (*ip).type_0 as libc::c_uint ==
                   INPUT_MISSING as libc::c_int as libc::c_uint {
                log_debug(b"parameter %u: missing\x00" as *const u8 as
                              *const libc::c_char, i);
            } else if (*ip).type_0 as libc::c_uint ==
                          INPUT_STRING as libc::c_int as libc::c_uint {
                log_debug(b"parameter %u: string %s\x00" as *const u8 as
                              *const libc::c_char, i, (*ip).unnamed.str_0);
            } else if (*ip).type_0 as libc::c_uint ==
                          INPUT_NUMBER as libc::c_int as libc::c_uint {
                log_debug(b"parameter %u: number %d\x00" as *const u8 as
                              *const libc::c_char, i, (*ip).unnamed.num);
            }
            i = i.wrapping_add(1)
        }
        return 0i32
    };
}
pub static mut input_state_csi_parameter: input_state =
    unsafe {
        input_state{name:
                        b"csi_parameter\x00" as *const u8 as
                            *const libc::c_char,
                    enter: None,
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_csi_parameter_table: [input_transition; 14] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 47i32,
                          handler: Some(input_intermediate),
                          state:
                              &input_state_csi_intermediate as
                                  *const input_state,},
         input_transition{first: 48i32,
                          last: 57i32,
                          handler: Some(input_parameter),
                          state: 0 as *const input_state,},
         input_transition{first: 58i32,
                          last: 58i32,
                          handler: Some(input_parameter),
                          state: 0 as *const input_state,},
         input_transition{first: 59i32,
                          last: 59i32,
                          handler: Some(input_parameter),
                          state: 0 as *const input_state,},
         input_transition{first: 60i32,
                          last: 63i32,
                          handler: None,
                          state:
                              &input_state_csi_ignore as *const input_state,},
         input_transition{first: 64i32,
                          last: 126i32,
                          handler: Some(input_csi_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 127i32,
                          last: 255i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
pub static mut input_state_csi_ignore: input_state =
    unsafe {
        input_state{name:
                        b"csi_ignore\x00" as *const u8 as *const libc::c_char,
                    enter: None,
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_csi_ignore_table: [input_transition; 10] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 63i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 64i32,
                          last: 126i32,
                          handler: None,
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 127i32,
                          last: 255i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
unsafe extern "C" fn input_parameter(mut ictx: *mut input_ctx)
 -> libc::c_int {
    if (*ictx).param_len ==
           (::std::mem::size_of::<[u_char; 64]>() as
                libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) {
        (*ictx).flags |= 1i32
    } else {
        let fresh8 = (*ictx).param_len;
        (*ictx).param_len = (*ictx).param_len.wrapping_add(1);
        (*ictx).param_buf[fresh8 as usize] = (*ictx).ch as u_char;
        (*ictx).param_buf[(*ictx).param_len as usize] = 0 as u_char
    }
    return 0i32;
}
pub static mut input_state_csi_intermediate: input_state =
    unsafe {
        input_state{name:
                        b"csi_intermediate\x00" as *const u8 as
                            *const libc::c_char,
                    enter: None,
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_csi_intermediate_table: [input_transition; 11] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 47i32,
                          handler: Some(input_intermediate),
                          state: 0 as *const input_state,},
         input_transition{first: 48i32,
                          last: 63i32,
                          handler: None,
                          state:
                              &input_state_csi_ignore as *const input_state,},
         input_transition{first: 64i32,
                          last: 126i32,
                          handler: Some(input_csi_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 127i32,
                          last: 255i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
unsafe extern "C" fn input_intermediate(mut ictx: *mut input_ctx)
 -> libc::c_int {
    if (*ictx).interm_len ==
           (::std::mem::size_of::<[u_char; 4]>() as
                libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) {
        (*ictx).flags |= 1i32
    } else {
        let fresh9 = (*ictx).interm_len;
        (*ictx).interm_len = (*ictx).interm_len.wrapping_add(1);
        (*ictx).interm_buf[fresh9 as usize] = (*ictx).ch as u_char;
        (*ictx).interm_buf[(*ictx).interm_len as usize] = 0 as u_char
    }
    return 0i32;
}
pub static mut input_state_dcs_enter: input_state =
    unsafe {
        input_state{name:
                        b"dcs_enter\x00" as *const u8 as *const libc::c_char,
                    enter: Some(input_enter_dcs),
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_dcs_enter_table: [input_transition; 14] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 47i32,
                          handler: Some(input_intermediate),
                          state:
                              &input_state_dcs_intermediate as
                                  *const input_state,},
         input_transition{first: 48i32,
                          last: 57i32,
                          handler: Some(input_parameter),
                          state:
                              &input_state_dcs_parameter as
                                  *const input_state,},
         input_transition{first: 58i32,
                          last: 58i32,
                          handler: None,
                          state:
                              &input_state_dcs_ignore as *const input_state,},
         input_transition{first: 59i32,
                          last: 59i32,
                          handler: Some(input_parameter),
                          state:
                              &input_state_dcs_parameter as
                                  *const input_state,},
         input_transition{first: 60i32,
                          last: 63i32,
                          handler: Some(input_intermediate),
                          state:
                              &input_state_dcs_parameter as
                                  *const input_state,},
         input_transition{first: 64i32,
                          last: 126i32,
                          handler: Some(input_input),
                          state:
                              &input_state_dcs_handler as
                                  *const input_state,},
         input_transition{first: 127i32,
                          last: 255i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
pub static mut input_state_dcs_handler: input_state =
    unsafe {
        input_state{name:
                        b"dcs_handler\x00" as *const u8 as
                            *const libc::c_char,
                    enter: None,
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_dcs_handler_table: [input_transition; 4] =
    unsafe {
        [input_transition{first: 0i32,
                          last: 26i32,
                          handler: Some(input_input),
                          state: 0 as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_dcs_escape as *const input_state,},
         input_transition{first: 28i32,
                          last: 255i32,
                          handler: Some(input_input),
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
pub static mut input_state_dcs_escape: input_state =
    unsafe {
        input_state{name:
                        b"dcs_escape\x00" as *const u8 as *const libc::c_char,
                    enter: None,
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_dcs_escape_table: [input_transition; 4] =
    unsafe {
        [input_transition{first: 0i32,
                          last: 91i32,
                          handler: Some(input_input),
                          state:
                              &input_state_dcs_handler as
                                  *const input_state,},
         input_transition{first: 92i32,
                          last: 92i32,
                          handler: Some(input_dcs_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 93i32,
                          last: 255i32,
                          handler: Some(input_input),
                          state:
                              &input_state_dcs_handler as
                                  *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
unsafe extern "C" fn input_dcs_dispatch(mut ictx: *mut input_ctx)
 -> libc::c_int {
    let prefix: [libc::c_char; 6] =
        *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"tmux;\x00");
    let prefix_len: u_int =
        (::std::mem::size_of::<[libc::c_char; 6]>() as
             libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) as u_int;
    if 0 != (*ictx).flags & 1i32 {
        return 0i32
    } else {
        log_debug(b"%s: \"%s\"\x00" as *const u8 as *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"input_dcs_dispatch\x00")).as_ptr(),
                  (*ictx).input_buf);
        if (*ictx).input_len >= prefix_len as libc::c_ulong &&
               strncmp((*ictx).input_buf as *const libc::c_char,
                       prefix.as_ptr(), prefix_len as libc::c_ulong) == 0i32 {
            screen_write_rawstring(&mut (*ictx).ctx as *mut screen_write_ctx,
                                   (*ictx).input_buf.offset(prefix_len as
                                                                isize),
                                   (*ictx).input_len.wrapping_sub(prefix_len
                                                                      as
                                                                      libc::c_ulong)
                                       as u_int);
        }
        return 0i32
    };
}
pub static mut input_state_dcs_parameter: input_state =
    unsafe {
        input_state{name:
                        b"dcs_parameter\x00" as *const u8 as
                            *const libc::c_char,
                    enter: None,
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_dcs_parameter_table: [input_transition; 14] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 47i32,
                          handler: Some(input_intermediate),
                          state:
                              &input_state_dcs_intermediate as
                                  *const input_state,},
         input_transition{first: 48i32,
                          last: 57i32,
                          handler: Some(input_parameter),
                          state: 0 as *const input_state,},
         input_transition{first: 58i32,
                          last: 58i32,
                          handler: None,
                          state:
                              &input_state_dcs_ignore as *const input_state,},
         input_transition{first: 59i32,
                          last: 59i32,
                          handler: Some(input_parameter),
                          state: 0 as *const input_state,},
         input_transition{first: 60i32,
                          last: 63i32,
                          handler: None,
                          state:
                              &input_state_dcs_ignore as *const input_state,},
         input_transition{first: 64i32,
                          last: 126i32,
                          handler: Some(input_input),
                          state:
                              &input_state_dcs_handler as
                                  *const input_state,},
         input_transition{first: 127i32,
                          last: 255i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
pub static mut input_state_dcs_ignore: input_state =
    unsafe {
        input_state{name:
                        b"dcs_ignore\x00" as *const u8 as *const libc::c_char,
                    enter: None,
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_dcs_ignore_table: [input_transition; 8] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 255i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
pub static mut input_state_dcs_intermediate: input_state =
    unsafe {
        input_state{name:
                        b"dcs_intermediate\x00" as *const u8 as
                            *const libc::c_char,
                    enter: None,
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_dcs_intermediate_table: [input_transition; 11] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 47i32,
                          handler: Some(input_intermediate),
                          state: 0 as *const input_state,},
         input_transition{first: 48i32,
                          last: 63i32,
                          handler: None,
                          state:
                              &input_state_dcs_ignore as *const input_state,},
         input_transition{first: 64i32,
                          last: 126i32,
                          handler: Some(input_input),
                          state:
                              &input_state_dcs_handler as
                                  *const input_state,},
         input_transition{first: 127i32,
                          last: 255i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
unsafe extern "C" fn input_enter_dcs(mut ictx: *mut input_ctx) -> () {
    log_debug(b"%s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"input_enter_dcs\x00")).as_ptr());
    input_clear(ictx);
    input_start_timer(ictx);
    (*ictx).last = 1i32.wrapping_neg();
}
pub static mut input_state_esc_intermediate: input_state =
    unsafe {
        input_state{name:
                        b"esc_intermediate\x00" as *const u8 as
                            *const libc::c_char,
                    enter: None,
                    exit: None,
                    transitions: 0 as *const _,}
    };
pub static mut input_state_esc_intermediate_table: [input_transition; 10] =
    unsafe {
        [input_transition{first: 24i32,
                          last: 24i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 26i32,
                          last: 26i32,
                          handler: Some(input_c0_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 27i32,
                          last: 27i32,
                          handler: None,
                          state:
                              &input_state_esc_enter as *const input_state,},
         input_transition{first: 0i32,
                          last: 23i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 25i32,
                          last: 25i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 28i32,
                          last: 31i32,
                          handler: Some(input_c0_dispatch),
                          state: 0 as *const input_state,},
         input_transition{first: 32i32,
                          last: 47i32,
                          handler: Some(input_intermediate),
                          state: 0 as *const input_state,},
         input_transition{first: 48i32,
                          last: 126i32,
                          handler: Some(input_esc_dispatch),
                          state: &input_state_ground as *const input_state,},
         input_transition{first: 127i32,
                          last: 255i32,
                          handler: None,
                          state: 0 as *const input_state,},
         input_transition{first: -1i32,
                          last: -1i32,
                          handler: None,
                          state: 0 as *const input_state,}]
    };
unsafe extern "C" fn input_ground(mut ictx: *mut input_ctx) -> () {
    event_del(&mut (*ictx).timer as *mut event);
    evbuffer_drain((*ictx).since_ground,
                   evbuffer_get_length((*ictx).since_ground));
    if (*ictx).input_space > 32i32 as libc::c_ulong {
        (*ictx).input_space = 32i32 as size_t;
        (*ictx).input_buf =
            xrealloc((*ictx).input_buf as *mut libc::c_void, 32i32 as size_t)
                as *mut u_char
    };
}
unsafe extern "C" fn input_timer_callback(mut fd: libc::c_int,
                                          mut events: libc::c_short,
                                          mut arg: *mut libc::c_void) -> () {
    let mut ictx: *mut input_ctx = arg as *mut input_ctx;
    let mut wp: *mut window_pane = (*ictx).wp;
    log_debug(b"%s: %%%u %s expired\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"input_timer_callback\x00")).as_ptr(),
              (*wp).id, (*(*ictx).state).name);
    input_reset(wp, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn input_free(mut wp: *mut window_pane) -> () {
    let mut ictx: *mut input_ctx = (*wp).ictx;
    let mut i: u_int = 0;
    i = 0i32 as u_int;
    while i < (*ictx).param_list_len {
        if (*ictx).param_list[i as usize].type_0 as libc::c_uint ==
               INPUT_STRING as libc::c_int as libc::c_uint {
            free((*ictx).param_list[i as usize].unnamed.str_0 as
                     *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    }
    event_del(&mut (*ictx).timer as *mut event);
    free((*ictx).input_buf as *mut libc::c_void);
    evbuffer_free((*ictx).since_ground);
    free(ictx as *mut libc::c_void);
    (*wp).ictx = 0 as *mut input_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn input_pending(mut wp: *mut window_pane)
 -> *mut evbuffer {
    return (*(*wp).ictx).since_ground;
}
#[no_mangle]
pub unsafe extern "C" fn input_parse(mut wp: *mut window_pane) -> () {
    let mut ictx: *mut input_ctx = (*wp).ictx;
    let mut itr: *const input_transition = 0 as *const input_transition;
    let mut evb: *mut evbuffer = (*(*wp).event).input;
    let mut buf: *mut u_char = 0 as *mut u_char;
    let mut len: size_t = 0;
    let mut off: size_t = 0;
    if evbuffer_get_length(evb) == 0i32 as libc::c_ulong {
        return
    } else {
        window_update_activity((*wp).window);
        (*wp).flags |= 128i32;
        if (*wp).mode == 0 as *mut libc::c_void as *const window_mode {
            screen_write_start(&mut (*ictx).ctx as *mut screen_write_ctx, wp,
                               &mut (*wp).base as *mut screen);
        } else {
            screen_write_start(&mut (*ictx).ctx as *mut screen_write_ctx,
                               0 as *mut window_pane,
                               &mut (*wp).base as *mut screen);
        }
        (*ictx).wp = wp;
        buf = evbuffer_pullup(evb, 1i32.wrapping_neg() as ssize_t);
        len = evbuffer_get_length(evb);
        off = 0i32 as size_t;
        notify_input(wp, evb);
        log_debug(b"%s: %%%u %s, %zu bytes: %.*s\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"input_parse\x00")).as_ptr(),
                  (*wp).id, (*(*ictx).state).name, len, len as libc::c_int,
                  buf);
        loop  {
            if off < len {
                let fresh10 = off;
                off = off.wrapping_add(1);
                (*ictx).ch = *buf.offset(fresh10 as isize) as libc::c_int;
                itr = (*(*ictx).state).transitions;
                while (*itr).first != 1i32.wrapping_neg() &&
                          (*itr).last != 1i32.wrapping_neg() {
                    if (*ictx).ch >= (*itr).first && (*ictx).ch <= (*itr).last
                       {
                        break ;
                    }
                    itr = itr.offset(1isize)
                }
                if (*itr).first == 1i32.wrapping_neg() ||
                       (*itr).last == 1i32.wrapping_neg() {
                    fatalx(b"no transition from state\x00" as *const u8 as
                               *const libc::c_char);
                } else {
                    if (*itr).handler != Some(input_print) {
                        screen_write_collect_end(&mut (*ictx).ctx as
                                                     *mut screen_write_ctx);
                    }
                    if (*itr).handler !=
                           ::std::mem::transmute::<*mut libc::c_void,
                                                   Option<unsafe extern "C" fn(_:
                                                                                   *mut input_ctx)
                                                              ->
                                                                  libc::c_int>>(0
                                                                                    as
                                                                                    *mut libc::c_void)
                           &&
                           (*itr).handler.expect("non-null function pointer")(ictx)
                               != 0i32 {
                        continue ;
                    }
                    if (*itr).state !=
                           0 as *mut libc::c_void as *const input_state {
                        input_set_state(wp, itr);
                    }
                    if !((*ictx).state !=
                             &input_state_ground as *const input_state) {
                        continue ;
                    }
                    evbuffer_add((*ictx).since_ground,
                                 &mut (*ictx).ch as *mut libc::c_int as
                                     *const libc::c_void, 1i32 as size_t);
                }
            } else {
                screen_write_stop(&mut (*ictx).ctx as *mut screen_write_ctx);
                evbuffer_drain(evb, len);
                return;
            }
        }
    };
}
unsafe extern "C" fn input_set_state(mut wp: *mut window_pane,
                                     mut itr: *const input_transition) -> () {
    let mut ictx: *mut input_ctx = (*wp).ictx;
    if (*(*ictx).state).exit !=
           ::std::mem::transmute::<*mut libc::c_void,
                                   Option<unsafe extern "C" fn(_:
                                                                   *mut input_ctx)
                                              -> ()>>(0 as *mut libc::c_void)
       {
        (*(*ictx).state).exit.expect("non-null function pointer")(ictx);
    }
    (*ictx).state = (*itr).state;
    if (*(*ictx).state).enter !=
           ::std::mem::transmute::<*mut libc::c_void,
                                   Option<unsafe extern "C" fn(_:
                                                                   *mut input_ctx)
                                              -> ()>>(0 as *mut libc::c_void)
       {
        (*(*ictx).state).enter.expect("non-null function pointer")(ictx);
    };
}
