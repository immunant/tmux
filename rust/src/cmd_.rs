extern crate libc;
extern "C" {
    pub type input_ctx;
    pub type args_entry;
    pub type evbuffer;
    pub type environ;
    pub type options;
    pub type format_tree;
    pub type options_entry;
    pub type format_job_tree;
    pub type screen_titles;
    pub type hooks;
    pub type event_base;
    pub type tty_code;
    pub type _IO_FILE_plus;
    pub type tmuxpeer;
    pub type bufferevent_ops;
    pub type tmuxproc;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
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
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> libc::c_ulong;
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
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, ...)
     -> libc::c_int;
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
    static mut tty_terms: tty_terms;
    #[no_mangle]
    fn args_parse(_: *const libc::c_char, _: libc::c_int,
                  _: *mut *mut libc::c_char) -> *mut args;
    #[no_mangle]
    fn args_free(_: *mut args) -> ();
    #[no_mangle]
    fn args_print(_: *mut args) -> *mut libc::c_char;
    #[no_mangle]
    static cmd_wait_for_entry: cmd_entry;
    #[no_mangle]
    static cmd_unlink_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_unbind_key_entry: cmd_entry;
    #[no_mangle]
    static cmd_switch_client_entry: cmd_entry;
    #[no_mangle]
    static cmd_swap_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_swap_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_suspend_client_entry: cmd_entry;
    #[no_mangle]
    static cmd_start_server_entry: cmd_entry;
    #[no_mangle]
    static cmd_split_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_source_file_entry: cmd_entry;
    #[no_mangle]
    static cmd_show_window_options_entry: cmd_entry;
    #[no_mangle]
    static cmd_show_options_entry: cmd_entry;
    #[no_mangle]
    static cmd_show_messages_entry: cmd_entry;
    #[no_mangle]
    static cmd_show_hooks_entry: cmd_entry;
    #[no_mangle]
    static cmd_show_environment_entry: cmd_entry;
    #[no_mangle]
    static cmd_show_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_set_window_option_entry: cmd_entry;
    #[no_mangle]
    static cmd_set_option_entry: cmd_entry;
    #[no_mangle]
    static cmd_set_hook_entry: cmd_entry;
    #[no_mangle]
    static cmd_set_environment_entry: cmd_entry;
    #[no_mangle]
    static cmd_set_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_send_prefix_entry: cmd_entry;
    #[no_mangle]
    static cmd_send_keys_entry: cmd_entry;
    #[no_mangle]
    static cmd_select_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_select_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_select_layout_entry: cmd_entry;
    #[no_mangle]
    static cmd_save_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_run_shell_entry: cmd_entry;
    #[no_mangle]
    static cmd_rotate_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_respawn_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_respawn_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_resize_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_rename_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_rename_session_entry: cmd_entry;
    #[no_mangle]
    static cmd_refresh_client_entry: cmd_entry;
    #[no_mangle]
    static cmd_previous_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_previous_layout_entry: cmd_entry;
    #[no_mangle]
    static cmd_pipe_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_paste_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_next_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_next_layout_entry: cmd_entry;
    #[no_mangle]
    static cmd_new_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_new_session_entry: cmd_entry;
    #[no_mangle]
    static cmd_move_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_move_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_lock_session_entry: cmd_entry;
    #[no_mangle]
    static cmd_lock_server_entry: cmd_entry;
    #[no_mangle]
    static cmd_lock_client_entry: cmd_entry;
    #[no_mangle]
    static cmd_load_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_windows_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_sessions_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_panes_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_keys_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_commands_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_clients_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_buffers_entry: cmd_entry;
    #[no_mangle]
    static cmd_link_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_last_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_last_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_kill_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_kill_session_entry: cmd_entry;
    #[no_mangle]
    static cmd_kill_server_entry: cmd_entry;
    #[no_mangle]
    static cmd_kill_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_join_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_if_shell_entry: cmd_entry;
    #[no_mangle]
    static cmd_has_session_entry: cmd_entry;
    #[no_mangle]
    static cmd_find_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_display_panes_entry: cmd_entry;
    #[no_mangle]
    static cmd_display_message_entry: cmd_entry;
    #[no_mangle]
    static cmd_detach_client_entry: cmd_entry;
    #[no_mangle]
    static cmd_delete_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_copy_mode_entry: cmd_entry;
    #[no_mangle]
    static cmd_confirm_before_entry: cmd_entry;
    #[no_mangle]
    static cmd_command_prompt_entry: cmd_entry;
    #[no_mangle]
    static cmd_clock_mode_entry: cmd_entry;
    #[no_mangle]
    static cmd_clear_history_entry: cmd_entry;
    #[no_mangle]
    static cmd_choose_tree_entry: cmd_entry;
    #[no_mangle]
    static cmd_choose_client_entry: cmd_entry;
    #[no_mangle]
    static cmd_choose_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_capture_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_break_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_bind_key_entry: cmd_entry;
    #[no_mangle]
    static cmd_attach_session_entry: cmd_entry;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, ...) -> ();
    #[no_mangle]
    fn cmd_string_split(_: *const libc::c_char, _: *mut libc::c_int,
                        _: *mut *mut *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    fn winlink_find_by_window(_: *mut winlinks, _: *mut window)
     -> *mut winlink;
    #[no_mangle]
    fn window_find_by_id(_: u_int) -> *mut window;
    #[no_mangle]
    fn session_find_by_id(_: u_int) -> *mut session;
    #[no_mangle]
    fn window_has_pane(_: *mut window, _: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn window_pane_find_by_id(_: u_int) -> *mut window_pane;
    #[no_mangle]
    static mut key_tables: key_tables;
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
    static cmd_down_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_up_pane_entry: cmd_entry;
}
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub type _IO_lock_t = ();
pub type __suseconds_t = libc::c_long;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub type size_t = libc::c_ulong;
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
    pub winlinks: unnamed_13,
    pub entry: unnamed_26,
}
pub type pid_t = __pid_t;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_0 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
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
    pub entry: unnamed_31,
}
pub type u_char = __u_char;
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
pub struct unnamed_1 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub type cc_t = libc::c_uchar;
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
pub struct unnamed_2 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
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
    pub entry: unnamed_37,
    pub tree_entry: unnamed_14,
}
pub const CMD_FIND_SESSION: cmd_find_type = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const PROMPT_COMMAND: unnamed_28 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
pub type __off_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
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
    pub entry: unnamed_6,
    pub wentry: unnamed_18,
    pub sentry: unnamed_19,
}
pub const JOB_RUNNING: unnamed_15 = 0;
pub type unnamed_3 = libc::c_uint;
pub type time_t = __time_t;
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_4 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_5 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_6 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type u_short = __u_short;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
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
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct job {
    pub state: unnamed_15,
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
    pub entry: unnamed_1,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_7 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_8 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_9 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
pub type options_table_type = libc::c_uint;
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
pub type __u_char = libc::c_uchar;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub type cmdq_type = libc::c_uint;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub type prompt_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const TTY_VT101: unnamed_32 = 1;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_10 {
    pub ev_io_next: unnamed_29,
    pub ev_timeout: timeval,
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
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: unnamed_24,
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
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_11 {
    __u6_addr8: [uint8_t; 16],
    __u6_addr16: [uint16_t; 8],
    __u6_addr32: [uint32_t; 4],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_12 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type job_free_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type prompt_input_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_13 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type uint8_t = libc::c_uchar;
pub type layout_type = libc::c_uint;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub type key_code = libc::c_ulonglong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct in6_addr {
    pub __in6_u: unnamed_11,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct winlinks {
    pub rbh_root: *mut winlink,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_14 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
pub type cmd_retval = libc::c_int;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub type unnamed_15 = libc::c_uint;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
pub const JOB_DEAD: unnamed_15 = 1;
pub const TTY_VT100: unnamed_32 = 0;
pub const LINE_SEL_RIGHT_LEFT: unnamed_3 = 2;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_16 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub type __u_short = libc::c_ushort;
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void)
               -> cmd_retval>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_17 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_18 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
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
    pub message_log: unnamed_7,
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
    pub entry: unnamed_9,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct windows {
    pub rbh_root: *mut window,
}
pub type u_int = __u_int;
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
    pub term_type: unnamed_32,
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
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type __u_int = libc::c_uint;
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
pub type options_table_scope = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_19 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub unnamed: unnamed_20,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_20 {
    offset: u_int,
    data: unnamed_8,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_21 {
    pub ev_signal_next: unnamed_36,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct event {
    pub ev_active_next: unnamed_16,
    pub ev_next: unnamed_12,
    pub ev_timeout_pos: unnamed_27,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub _ev: unnamed_34,
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
pub const TTY_VT220: unnamed_32 = 3;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: unnamed_23,
}
pub const JOB_CLOSED: unnamed_15 = 2;
pub type bitstr_t = libc::c_uchar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
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
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub const TTY_UNKNOWN: unnamed_32 = 6;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: unnamed_4,
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
pub type __time_t = libc::c_long;
pub type speed_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_22 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_23 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_24 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
pub const PROMPT_ENTRY: unnamed_28 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: unnamed_3,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const TTY_VT102: unnamed_32 = 2;
pub const TTY_VT320: unnamed_32 = 4;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: unnamed_2,
    pub entry: unnamed_35,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: unnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_25 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
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
pub const LINE_SEL_NONE: unnamed_3 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_27 {
    ev_next_with_common_timeout: unnamed_0,
    min_heap_idx: libc::c_int,
}
pub type unnamed_28 = libc::c_uint;
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
    pub entry: unnamed_30,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_29 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
pub type __pid_t = libc::c_int;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const TTY_VT420: unnamed_32 = 5;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub type cmd_find_type = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_30 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
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
pub struct unnamed_31 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type unnamed_32 = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: unnamed_25,
}
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_33 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_34 {
    ev_io: unnamed_10,
    ev_signal: unnamed_21,
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
pub const LINE_SEL_LEFT_RIGHT: unnamed_3 = 1;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_35 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
pub type uint16_t = libc::c_ushort;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_36 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_37 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct unnamed_38 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
pub type tcflag_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[no_mangle]
pub unsafe extern "C" fn cmd_pack_argv(mut argc: libc::c_int,
                                       mut argv: *mut *mut libc::c_char,
                                       mut buf: *mut libc::c_char,
                                       mut len: size_t) -> libc::c_int {
    let mut arglen: size_t = 0;
    let mut i: libc::c_int = 0;
    if argc == 0i32 {
        return 0i32
    } else {
        *buf = 0 as libc::c_char;
        i = 0i32;
        loop  {
            if i < argc {
                if strlcpy(buf, *argv.offset(i as isize), len) >= len {
                    return 1i32.wrapping_neg()
                } else {
                    arglen =
                        strlen(*argv.offset(i as
                                                isize)).wrapping_add(1i32 as
                                                                         libc::c_ulong);
                    buf = buf.offset(arglen as isize);
                    len =
                        (len as libc::c_ulong).wrapping_sub(arglen) as size_t
                            as size_t;
                    i += 1
                }
            } else { return 0i32 }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_unpack_argv(mut buf: *mut libc::c_char,
                                         mut len: size_t,
                                         mut argc: libc::c_int,
                                         mut argv:
                                             *mut *mut *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut arglen: size_t = 0;
    if argc == 0i32 {
        return 0i32
    } else {
        *argv =
            xcalloc(argc as size_t,
                    ::std::mem::size_of::<*mut libc::c_char>() as
                        libc::c_ulong) as *mut *mut libc::c_char;
        *buf.offset(len.wrapping_sub(1i32 as libc::c_ulong) as isize) =
            0 as libc::c_char;
        i = 0i32;
        loop  {
            if i < argc {
                if len == 0i32 as libc::c_ulong {
                    cmd_free_argv(argc, *argv);
                    return 1i32.wrapping_neg()
                } else {
                    arglen = strlen(buf).wrapping_add(1i32 as libc::c_ulong);
                    let ref mut fresh0 = *(*argv).offset(i as isize);
                    *fresh0 = xstrdup(buf);
                    buf = buf.offset(arglen as isize);
                    len =
                        (len as libc::c_ulong).wrapping_sub(arglen) as size_t
                            as size_t;
                    i += 1
                }
            } else { return 0i32 }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_free_argv(mut argc: libc::c_int,
                                       mut argv: *mut *mut libc::c_char)
 -> () {
    let mut i: libc::c_int = 0;
    if argc == 0i32 {
        return
    } else {
        i = 0i32;
        while i < argc {
            free(*argv.offset(i as isize) as *mut libc::c_void);
            i += 1
        }
        free(argv as *mut libc::c_void);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_copy_argv(mut argc: libc::c_int,
                                       mut argv: *mut *mut libc::c_char)
 -> *mut *mut libc::c_char {
    let mut new_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if argc == 0i32 {
        return 0 as *mut *mut libc::c_char
    } else {
        new_argv =
            xcalloc((argc + 1i32) as size_t,
                    ::std::mem::size_of::<*mut libc::c_char>() as
                        libc::c_ulong) as *mut *mut libc::c_char;
        i = 0i32;
        while i < argc {
            if *argv.offset(i as isize) !=
                   0 as *mut libc::c_void as *mut libc::c_char {
                let ref mut fresh1 = *new_argv.offset(i as isize);
                *fresh1 = xstrdup(*argv.offset(i as isize))
            }
            i += 1
        }
        return new_argv
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_stringify_argv(mut argc: libc::c_int,
                                            mut argv: *mut *mut libc::c_char)
 -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut len: size_t = 0;
    if argc == 0i32 {
        return xstrdup(b"\x00" as *const u8 as *const libc::c_char)
    } else {
        len = 0i32 as size_t;
        buf = 0 as *mut libc::c_char;
        i = 0i32;
        while i < argc {
            len =
                (len as
                     libc::c_ulong).wrapping_add(strlen(*argv.offset(i as
                                                                         isize)).wrapping_add(1i32
                                                                                                  as
                                                                                                  libc::c_ulong))
                    as size_t as size_t;
            buf =
                xrealloc(buf as *mut libc::c_void, len) as *mut libc::c_char;
            if i == 0i32 {
                *buf = 0 as libc::c_char
            } else {
                strlcat(buf, b" \x00" as *const u8 as *const libc::c_char,
                        len);
            }
            strlcat(buf, *argv.offset(i as isize), len);
            i += 1
        }
        return buf
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_parse(mut argc: libc::c_int,
                                   mut argv: *mut *mut libc::c_char,
                                   mut file: *const libc::c_char,
                                   mut line: u_int,
                                   mut cause: *mut *mut libc::c_char)
 -> *mut cmd {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut entryp: *mut *const cmd_entry = 0 as *mut *const cmd_entry;
    let mut entry: *const cmd_entry = 0 as *const cmd_entry;
    let mut cmd: *mut cmd = 0 as *mut cmd;
    let mut args: *mut args = 0 as *mut args;
    let mut s: [libc::c_char; 8192] = [0; 8192];
    let mut ambiguous: libc::c_int = 0;
    let mut allocated: libc::c_int = 0i32;
    *cause = 0 as *mut libc::c_char;
    if argc == 0i32 {
        xasprintf(cause,
                  b"no command\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut cmd
    } else {
        name = *argv.offset(0isize);
        loop  {
            ambiguous = 0i32;
            entry = 0 as *const cmd_entry;
            entryp = cmd_table.as_mut_ptr();
            while *entryp != 0 as *mut libc::c_void as *const cmd_entry {
                if (**entryp).alias !=
                       0 as *mut libc::c_void as *const libc::c_char &&
                       strcmp((**entryp).alias, *argv.offset(0isize)) == 0i32
                   {
                    ambiguous = 0i32;
                    entry = *entryp;
                    break ;
                } else {
                    if !(strncmp((**entryp).name, *argv.offset(0isize),
                                 strlen(*argv.offset(0isize))) != 0i32) {
                        if entry != 0 as *mut libc::c_void as *const cmd_entry
                           {
                            ambiguous = 1i32
                        }
                        entry = *entryp;
                        if strcmp((*entry).name, *argv.offset(0isize)) == 0i32
                           {
                            break ;
                        }
                    }
                    entryp = entryp.offset(1isize)
                }
            }
            if !((0 != ambiguous ||
                      entry == 0 as *mut libc::c_void as *const cmd_entry) &&
                     server_proc != 0 as *mut libc::c_void as *mut tmuxproc &&
                     0 == allocated &&
                     cmd_try_alias(&mut argc as *mut libc::c_int,
                                   &mut argv as *mut *mut *mut libc::c_char)
                         == 0i32) {
                break ;
            }
            allocated = 1i32
        }
        if 0 != ambiguous {
            *s.as_mut_ptr() = 0 as libc::c_char;
            entryp = cmd_table.as_mut_ptr();
            while *entryp != 0 as *mut libc::c_void as *const cmd_entry {
                if !(strncmp((**entryp).name, *argv.offset(0isize),
                             strlen(*argv.offset(0isize))) != 0i32) {
                    if strlcat(s.as_mut_ptr(), (**entryp).name,
                               ::std::mem::size_of::<[libc::c_char; 8192]>()
                                   as libc::c_ulong) >=
                           ::std::mem::size_of::<[libc::c_char; 8192]>() as
                               libc::c_ulong {
                        break ;
                    }
                    if strlcat(s.as_mut_ptr(),
                               b", \x00" as *const u8 as *const libc::c_char,
                               ::std::mem::size_of::<[libc::c_char; 8192]>()
                                   as libc::c_ulong) >=
                           ::std::mem::size_of::<[libc::c_char; 8192]>() as
                               libc::c_ulong {
                        break ;
                    }
                }
                entryp = entryp.offset(1isize)
            }
            s[strlen(s.as_mut_ptr()).wrapping_sub(2i32 as libc::c_ulong) as
                  usize] = 0 as libc::c_char;
            xasprintf(cause,
                      b"ambiguous command: %s, could be: %s\x00" as *const u8
                          as *const libc::c_char, name, s.as_mut_ptr());
            return 0 as *mut cmd
        } else if entry == 0 as *mut libc::c_void as *const cmd_entry {
            xasprintf(cause,
                      b"unknown command: %s\x00" as *const u8 as
                          *const libc::c_char, name);
            return 0 as *mut cmd
        } else {
            args = args_parse((*entry).args.template, argc, argv);
            if !(args == 0 as *mut libc::c_void as *mut args) {
                if !((*entry).args.lower != 1i32.wrapping_neg() &&
                         (*args).argc < (*entry).args.lower) {
                    if !((*entry).args.upper != 1i32.wrapping_neg() &&
                             (*args).argc > (*entry).args.upper) {
                        cmd =
                            xcalloc(1i32 as size_t,
                                    ::std::mem::size_of::<cmd>() as
                                        libc::c_ulong) as *mut cmd;
                        (*cmd).entry = entry;
                        (*cmd).args = args;
                        if file !=
                               0 as *mut libc::c_void as *const libc::c_char {
                            (*cmd).file = xstrdup(file)
                        }
                        (*cmd).line = line;
                        if 0 != allocated { cmd_free_argv(argc, argv); }
                        return cmd
                    }
                }
            }
            if args != 0 as *mut libc::c_void as *mut args {
                args_free(args);
            }
            xasprintf(cause,
                      b"usage: %s %s\x00" as *const u8 as *const libc::c_char,
                      (*entry).name, (*entry).usage);
            return 0 as *mut cmd
        }
    };
}
#[no_mangle]
pub static mut cmd_table: [*const cmd_entry; 84] =
    unsafe {
        [&cmd_attach_session_entry as *const cmd_entry,
         &cmd_bind_key_entry as *const cmd_entry,
         &cmd_break_pane_entry as *const cmd_entry,
         &cmd_capture_pane_entry as *const cmd_entry,
         &cmd_choose_buffer_entry as *const cmd_entry,
         &cmd_choose_client_entry as *const cmd_entry,
         &cmd_choose_tree_entry as *const cmd_entry,
         &cmd_clear_history_entry as *const cmd_entry,
         &cmd_clock_mode_entry as *const cmd_entry,
         &cmd_command_prompt_entry as *const cmd_entry,
         &cmd_confirm_before_entry as *const cmd_entry,
         &cmd_copy_mode_entry as *const cmd_entry,
         &cmd_delete_buffer_entry as *const cmd_entry,
         &cmd_detach_client_entry as *const cmd_entry,
         &cmd_display_message_entry as *const cmd_entry,
         &cmd_display_panes_entry as *const cmd_entry,
         &cmd_find_window_entry as *const cmd_entry,
         &cmd_has_session_entry as *const cmd_entry,
         &cmd_if_shell_entry as *const cmd_entry,
         &cmd_join_pane_entry as *const cmd_entry,
         &cmd_kill_pane_entry as *const cmd_entry,
         &cmd_kill_server_entry as *const cmd_entry,
         &cmd_kill_session_entry as *const cmd_entry,
         &cmd_kill_window_entry as *const cmd_entry,
         &cmd_last_pane_entry as *const cmd_entry,
         &cmd_last_window_entry as *const cmd_entry,
         &cmd_link_window_entry as *const cmd_entry,
         &cmd_list_buffers_entry as *const cmd_entry,
         &cmd_list_clients_entry as *const cmd_entry,
         &cmd_list_commands_entry as *const cmd_entry,
         &cmd_list_keys_entry as *const cmd_entry,
         &cmd_list_panes_entry as *const cmd_entry,
         &cmd_list_sessions_entry as *const cmd_entry,
         &cmd_list_windows_entry as *const cmd_entry,
         &cmd_load_buffer_entry as *const cmd_entry,
         &cmd_lock_client_entry as *const cmd_entry,
         &cmd_lock_server_entry as *const cmd_entry,
         &cmd_lock_session_entry as *const cmd_entry,
         &cmd_move_pane_entry as *const cmd_entry,
         &cmd_move_window_entry as *const cmd_entry,
         &cmd_new_session_entry as *const cmd_entry,
         &cmd_new_window_entry as *const cmd_entry,
         &cmd_next_layout_entry as *const cmd_entry,
         &cmd_next_window_entry as *const cmd_entry,
         &cmd_paste_buffer_entry as *const cmd_entry,
         &cmd_pipe_pane_entry as *const cmd_entry,
         &cmd_previous_layout_entry as *const cmd_entry,
         &cmd_previous_window_entry as *const cmd_entry,
         &cmd_refresh_client_entry as *const cmd_entry,
         &cmd_rename_session_entry as *const cmd_entry,
         &cmd_rename_window_entry as *const cmd_entry,
         &cmd_resize_pane_entry as *const cmd_entry,
         &cmd_respawn_pane_entry as *const cmd_entry,
         &cmd_respawn_window_entry as *const cmd_entry,
         &cmd_rotate_window_entry as *const cmd_entry,
         &cmd_run_shell_entry as *const cmd_entry,
         &cmd_save_buffer_entry as *const cmd_entry,
         &cmd_select_layout_entry as *const cmd_entry,
         &cmd_select_pane_entry as *const cmd_entry,
         &cmd_select_window_entry as *const cmd_entry,
         &cmd_send_keys_entry as *const cmd_entry,
         &cmd_send_prefix_entry as *const cmd_entry,
         &cmd_set_buffer_entry as *const cmd_entry,
         &cmd_set_environment_entry as *const cmd_entry,
         &cmd_set_hook_entry as *const cmd_entry,
         &cmd_set_option_entry as *const cmd_entry,
         &cmd_set_window_option_entry as *const cmd_entry,
         &cmd_show_buffer_entry as *const cmd_entry,
         &cmd_show_environment_entry as *const cmd_entry,
         &cmd_show_hooks_entry as *const cmd_entry,
         &cmd_show_messages_entry as *const cmd_entry,
         &cmd_show_options_entry as *const cmd_entry,
         &cmd_show_window_options_entry as *const cmd_entry,
         &cmd_source_file_entry as *const cmd_entry,
         &cmd_split_window_entry as *const cmd_entry,
         &cmd_start_server_entry as *const cmd_entry,
         &cmd_suspend_client_entry as *const cmd_entry,
         &cmd_swap_pane_entry as *const cmd_entry,
         &cmd_swap_window_entry as *const cmd_entry,
         &cmd_switch_client_entry as *const cmd_entry,
         &cmd_unbind_key_entry as *const cmd_entry,
         &cmd_unlink_window_entry as *const cmd_entry,
         &cmd_wait_for_entry as *const cmd_entry, 0 as *const cmd_entry]
    };
unsafe extern "C" fn cmd_try_alias(mut argc: *mut libc::c_int,
                                   mut argv: *mut *mut *mut libc::c_char)
 -> libc::c_int {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut old_argc: libc::c_int = *argc;
    let mut new_argc: libc::c_int = 0;
    let mut old_argv: *mut *mut libc::c_char = *argv;
    let mut new_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut size: u_int = 0;
    let mut idx: u_int = 0;
    let mut i: libc::c_int = 0;
    let mut wanted: size_t = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    o =
        options_get_only(global_options,
                         b"command-alias\x00" as *const u8 as
                             *const libc::c_char);
    if o == 0 as *mut libc::c_void as *mut options_entry ||
           options_array_size(o, &mut size as *mut u_int) ==
               1i32.wrapping_neg() || size == 0i32 as libc::c_uint {
        return 1i32.wrapping_neg()
    } else {
        wanted = strlen(*old_argv.offset(0isize));
        idx = 0i32 as u_int;
        while idx < size {
            s = options_array_get(o, idx);
            if !(s == 0 as *mut libc::c_void as *const libc::c_char) {
                cp = strchr(s, 61);
                if !(cp == 0 as *mut libc::c_void as *const libc::c_char ||
                         s.offset_to(cp).expect("bad offset_to") as
                             libc::c_long as size_t != wanted) {
                    if strncmp(*old_argv.offset(0isize), s, wanted) == 0i32 {
                        break ;
                    }
                }
            }
            idx = idx.wrapping_add(1)
        }
        if idx == size {
            return 1i32.wrapping_neg()
        } else if cmd_string_split(cp.offset(1isize),
                                   &mut new_argc as *mut libc::c_int,
                                   &mut new_argv as
                                       *mut *mut *mut libc::c_char) != 0i32 {
            return 1i32.wrapping_neg()
        } else {
            *argc = new_argc + old_argc - 1i32;
            *argv =
                xcalloc((*argc + 1i32) as size_t,
                        ::std::mem::size_of::<*mut libc::c_char>() as
                            libc::c_ulong) as *mut *mut libc::c_char;
            i = 0i32;
            while i < new_argc {
                let ref mut fresh2 = *(*argv).offset(i as isize);
                *fresh2 = xstrdup(*new_argv.offset(i as isize));
                i += 1
            }
            i = 1i32;
            while i < old_argc {
                let ref mut fresh3 =
                    *(*argv).offset((new_argc + i - 1i32) as isize);
                *fresh3 = xstrdup(*old_argv.offset(i as isize));
                i += 1
            }
            log_debug(b"alias: %s=%s\x00" as *const u8 as *const libc::c_char,
                      *old_argv.offset(0isize), cp.offset(1isize));
            i = 0i32;
            while i < *argc {
                log_debug(b"alias: argv[%d] = %s\x00" as *const u8 as
                              *const libc::c_char, i,
                          *(*argv).offset(i as isize));
                i += 1
            }
            cmd_free_argv(new_argc, new_argv);
            return 0i32
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_print(mut cmd: *mut cmd) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = args_print((*cmd).args);
    if *s as libc::c_int != 0 {
        xasprintf(&mut out as *mut *mut libc::c_char,
                  b"%s %s\x00" as *const u8 as *const libc::c_char,
                  (*(*cmd).entry).name, s);
    } else { out = xstrdup((*(*cmd).entry).name) }
    free(s as *mut libc::c_void);
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_mouse_at(mut wp: *mut window_pane,
                                      mut m: *mut mouse_event,
                                      mut xp: *mut u_int, mut yp: *mut u_int,
                                      mut last: libc::c_int) -> libc::c_int {
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    if 0 != last { x = (*m).lx; y = (*m).ly } else { x = (*m).x; y = (*m).y }
    if (*m).statusat == 0i32 && y > 0i32 as libc::c_uint {
        y = y.wrapping_sub(1)
    } else if (*m).statusat > 0i32 && y >= (*m).statusat as u_int {
        y = ((*m).statusat - 1i32) as u_int
    }
    if x < (*wp).xoff || x >= (*wp).xoff.wrapping_add((*wp).sx) {
        return 1i32.wrapping_neg()
    } else if y < (*wp).yoff || y >= (*wp).yoff.wrapping_add((*wp).sy) {
        return 1i32.wrapping_neg()
    } else {
        if xp != 0 as *mut libc::c_void as *mut u_int {
            *xp = x.wrapping_sub((*wp).xoff)
        }
        if yp != 0 as *mut libc::c_void as *mut u_int {
            *yp = y.wrapping_sub((*wp).yoff)
        }
        return 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_mouse_window(mut m: *mut mouse_event,
                                          mut sp: *mut *mut session)
 -> *mut winlink {
    let mut s: *mut session = 0 as *mut session;
    let mut w: *mut window = 0 as *mut window;
    if 0 == (*m).valid || (*m).s == 1i32.wrapping_neg() ||
           (*m).w == 1i32.wrapping_neg() {
        return 0 as *mut winlink
    } else {
        s = session_find_by_id((*m).s as u_int);
        if s == 0 as *mut libc::c_void as *mut session {
            return 0 as *mut winlink
        } else {
            w = window_find_by_id((*m).w as u_int);
            if w == 0 as *mut libc::c_void as *mut window {
                return 0 as *mut winlink
            } else {
                if sp != 0 as *mut libc::c_void as *mut *mut session {
                    *sp = s
                }
                return winlink_find_by_window(&mut (*s).windows as
                                                  *mut winlinks, w)
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_mouse_pane(mut m: *mut mouse_event,
                                        mut sp: *mut *mut session,
                                        mut wlp: *mut *mut winlink)
 -> *mut window_pane {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    wl = cmd_mouse_window(m, sp);
    if wl == 0 as *mut libc::c_void as *mut winlink {
        return 0 as *mut window_pane
    } else {
        wp = window_pane_find_by_id((*m).wp as u_int);
        if wp == 0 as *mut libc::c_void as *mut window_pane {
            return 0 as *mut window_pane
        } else if 0 == window_has_pane((*wl).window, wp) {
            return 0 as *mut window_pane
        } else {
            if wlp != 0 as *mut libc::c_void as *mut *mut winlink {
                *wlp = wl
            }
            return wp
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_template_replace(mut template:
                                                  *const libc::c_char,
                                              mut s: *const libc::c_char,
                                              mut idx: libc::c_int)
 -> *mut libc::c_char {
    let mut current_block: u64;
    let mut ch: libc::c_char = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let quote: [libc::c_char; 4] =
        *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"\"\\$\x00");
    let mut replaced: libc::c_int = 0;
    let mut quoted: libc::c_int = 0;
    let mut len: size_t = 0;
    if strchr(template, 37) == 0 as *mut libc::c_void as *mut libc::c_char {
        return xstrdup(template)
    } else {
        buf = xmalloc(1i32 as size_t) as *mut libc::c_char;
        *buf = 0 as libc::c_char;
        len = 0i32 as size_t;
        replaced = 0i32;
        ptr = template;
        while *ptr as libc::c_int != 0 {
            let fresh4 = ptr;
            ptr = ptr.offset(1);
            ch = *fresh4;
            match ch as libc::c_int {
                37 => {
                    if (*ptr as libc::c_int) < 49 || *ptr as libc::c_int > 57
                           || *ptr as libc::c_int - 48 != idx {
                        if *ptr as libc::c_int != 37 || 0 != replaced {
                            current_block = 4644295000439058019;
                        } else {
                            replaced = 1i32;
                            current_block = 735147466149431745;
                        }
                    } else { current_block = 735147466149431745; }
                    match current_block {
                        4644295000439058019 => { }
                        _ => {
                            ptr = ptr.offset(1isize);
                            quoted =
                                (*ptr as libc::c_int == 37) as libc::c_int;
                            if 0 != quoted { ptr = ptr.offset(1isize) }
                            buf =
                                xrealloc(buf as *mut libc::c_void,
                                         len.wrapping_add(strlen(s).wrapping_mul(3i32
                                                                                     as
                                                                                     libc::c_ulong)).wrapping_add(1i32
                                                                                                                      as
                                                                                                                      libc::c_ulong))
                                    as *mut libc::c_char;
                            cp = s;
                            while *cp as libc::c_int != 0 {
                                if 0 != quoted &&
                                       strchr(quote.as_ptr(),
                                              *cp as libc::c_int) !=
                                           0 as *mut libc::c_void as
                                               *mut libc::c_char {
                                    let fresh5 = len;
                                    len = len.wrapping_add(1);
                                    *buf.offset(fresh5 as isize) =
                                        92 as libc::c_char
                                }
                                if 0 != quoted && *cp as libc::c_int == 59 {
                                    let fresh6 = len;
                                    len = len.wrapping_add(1);
                                    *buf.offset(fresh6 as isize) =
                                        92 as libc::c_char;
                                    let fresh7 = len;
                                    len = len.wrapping_add(1);
                                    *buf.offset(fresh7 as isize) =
                                        92 as libc::c_char
                                }
                                let fresh8 = len;
                                len = len.wrapping_add(1);
                                *buf.offset(fresh8 as isize) = *cp;
                                cp = cp.offset(1isize)
                            }
                            *buf.offset(len as isize) = 0 as libc::c_char;
                            continue ;
                        }
                    }
                }
                _ => { }
            }
            buf =
                xrealloc(buf as *mut libc::c_void,
                         len.wrapping_add(2i32 as libc::c_ulong)) as
                    *mut libc::c_char;
            let fresh9 = len;
            len = len.wrapping_add(1);
            *buf.offset(fresh9 as isize) = ch;
            *buf.offset(len as isize) = 0 as libc::c_char
        }
        return buf
    };
}

