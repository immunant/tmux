#ifndef __TMUX_VARIADIC_H
#define __TMUX_VARIADIC_H

struct cmdq_item;
struct client;
struct environ;
struct format_tree;
struct hooks;
struct cmd_find_state;
struct options;
struct screen_write_ctx;
struct grid_cell;
struct window_pane;

void printflike(3, 4) args_print_add(char **buf, size_t *len, const char *fmt, ...);
void cfg_add_cause(const char *fmt, ...);
void cmdq_format(struct cmdq_item *item, const char *key, const char *fmt, ...);
void cmdq_print(struct cmdq_item *item, const char *fmt, ...);
void cmdq_error(struct cmdq_item *item, const char *fmt, ...);
void control_write(struct client *c, const char *fmt, ...);
void environ_set(struct environ *env, const char *name, const char *fmt, ...);
void environ_log(struct environ *env, const char *fmt, ...);
void format_add(struct format_tree *ft, const char *key, const char *fmt, ...);
void hooks_insert(struct hooks *hooks, struct cmdq_item *item, struct cmd_find_state *fs, const char *fmt, ...);
void printflike(2, 3) input_reply(struct input_ctx *, const char *, ...);
void log_debug(const char *msg, ...);
__dead void fatal(const char *msg, ...);
__dead void fatalx(const char *msg, ...);
struct options_entry *options_set_string(struct options *oo, const char *name, int append, const char *fmt, ...);
size_t screen_write_cstrlen(const char *fmt, ...);
size_t screen_write_strlen(const char *fmt, ...);
void screen_write_puts(struct screen_write_ctx *ctx, const struct grid_cell *gcp, const char *fmt, ...);
void screen_write_nputs(struct screen_write_ctx *ctx, ssize_t maxlen, const struct grid_cell *gcp, const char *fmt, ...);
void screen_write_cnputs(struct screen_write_ctx *ctx, ssize_t maxlen, const struct grid_cell *gcp, const char *fmt, ...);
void server_client_add_message(struct client *c, const char *fmt, ...);
void status_message_set(struct client *c, const char *fmt, ...);
void window_copy_add(struct window_pane *wp, const char *fmt, ...);
int xasprintf(char **ret, const char *fmt, ...);
int xsnprintf(char *str, size_t len, const char *fmt, ...);
void setproctitle(__unused const char *fmt, ...);

#endif // __TMUX_VARIADIC_H
