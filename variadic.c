#include <ctype.h>
#include <errno.h>
#include <sys/types.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "tmux.h"
// #include "variadic.h"

// extern int xvasprintf(char **, const char *, va_list) __attribute__((__nonnull__ (2)));

extern char **cfg_causes;
extern u_int cfg_ncauses;
// format.c:
typedef void (*format_cb)(struct format_tree *, struct format_entry *);
extern struct format_entry {
	char			*key;
	char			*value;
	time_t			 t;
	format_cb		 cb;
	RB_ENTRY(format_entry)	 entry;
};
extern struct format_tree {
	struct window		*w;
	struct winlink		*wl;
	struct session		*s;
	struct window_pane	*wp;

	struct client		*client;
	u_int			 tag;
	int			 flags;

	RB_HEAD(format_entry_tree, format_entry) tree;
};
extern void status_message_callback(int, short, void *);
extern struct options_entry {
	struct options				 *owner;

	const char				 *name;
	const struct options_table_entry	 *tableentry;

	union {
		char				 *string;
		long long			  number;
		struct grid_cell		  style;
		struct {
			const char		**array;
			u_int			  arraysize;
		};
	};

	RB_ENTRY(options_entry)			  entry;
};
extern struct input_param {
	enum {
		INPUT_MISSING,
		INPUT_NUMBER,
		INPUT_STRING
	}                       type;
	union {
		int		num;
		char	       *str;
	};
};
extern struct input_cell {
	struct grid_cell	cell;
	int			set;
	int			g0set;	/* 1 if ACS */
	int			g1set;	/* 1 if ACS */
};
extern struct input_ctx {
	struct window_pane     *wp;
	struct screen_write_ctx ctx;

	struct input_cell	cell;

	struct input_cell	old_cell;
	u_int 			old_cx;
	u_int			old_cy;

	u_char			interm_buf[4];
	size_t			interm_len;

	u_char			param_buf[64];
	size_t			param_len;

#define INPUT_BUF_START 32
#define INPUT_BUF_LIMIT 1048576
	u_char		       *input_buf;
	size_t			input_len;
	size_t			input_space;

	struct input_param	param_list[24];
	u_int			param_list_len;

	struct utf8_data	utf8data;
	int			utf8started;

	int			ch;
	int			last;

	int			flags;
#define INPUT_DISCARD 0x1

	const struct input_state *state;

	struct event		timer;

	/*
	 * All input received since we were last in the ground state. Sent to
	 * control clients on connection.
	 */
	struct evbuffer	 	*since_ground;
};

/* Add to string. */
void printflike(3, 4)
args_print_add(char **buf, size_t *len, const char *fmt, ...)
{
	va_list  ap;
	char	*s;
	size_t	 slen;

	va_start(ap, fmt);
	slen = xvasprintf(&s, fmt, ap);
	va_end(ap);

	*len += slen;
	*buf = xrealloc(*buf, *len);

	strlcat(*buf, s, *len);
	free(s);
}

void
cfg_add_cause(const char *fmt, ...)
{
	va_list	 ap;
	char	*msg;

	va_start(ap, fmt);
	xvasprintf(&msg, fmt, ap);
	va_end(ap);

	cfg_ncauses++;
	cfg_causes = xreallocarray(cfg_causes, cfg_ncauses, sizeof *cfg_causes);
	cfg_causes[cfg_ncauses - 1] = msg;
}

/* Add a format to command queue. */
void
cmdq_format(struct cmdq_item *item, const char *key, const char *fmt, ...)
{
	struct cmdq_shared	*shared = item->shared;
	va_list			 ap;
	char			*value;

	va_start(ap, fmt);
	xvasprintf(&value, fmt, ap);
	va_end(ap);

	if (shared->formats == NULL)
		shared->formats = format_create(NULL, NULL, FORMAT_NONE, 0);
	format_add(shared->formats, key, "%s", value);

	free(value);
}

/* Show message from command. */
void
cmdq_print(struct cmdq_item *item, const char *fmt, ...)
{
	struct client	*c = item->client;
	struct window	*w;
	va_list		 ap;
	char		*tmp, *msg;

	va_start(ap, fmt);

	if (c == NULL)
		/* nothing */;
	else if (c->session == NULL || (c->flags & CLIENT_CONTROL)) {
		if (~c->flags & CLIENT_UTF8) {
			xvasprintf(&tmp, fmt, ap);
			msg = utf8_sanitize(tmp);
			free(tmp);
			evbuffer_add(c->stdout_data, msg, strlen(msg));
			free(msg);
		} else
			evbuffer_add_vprintf(c->stdout_data, fmt, ap);
		evbuffer_add(c->stdout_data, "\n", 1);
		server_client_push_stdout(c);
	} else {
		w = c->session->curw->window;
		if (w->active->mode != &window_copy_mode) {
			window_pane_reset_mode(w->active);
			window_pane_set_mode(w->active, &window_copy_mode, NULL,
			    NULL);
			window_copy_init_for_output(w->active);
		}
		window_copy_vadd(w->active, fmt, ap);
	}

	va_end(ap);
}

/* Write a line. */
void
control_write(struct client *c, const char *fmt, ...)
{
	va_list		 ap;

	va_start(ap, fmt);
	evbuffer_add_vprintf(c->stdout_data, fmt, ap);
	va_end(ap);

	evbuffer_add(c->stdout_data, "\n", 1);
	server_client_push_stdout(c);
}

/* Set an environment variable. */
void
environ_set(struct environ *env, const char *name, const char *fmt, ...)
{
	struct environ_entry	*envent;
	va_list			 ap;

	va_start(ap, fmt);
	if ((envent = environ_find(env, name)) != NULL) {
		free(envent->value);
		xvasprintf(&envent->value, fmt, ap);
	} else {
		envent = xmalloc(sizeof *envent);
		envent->name = xstrdup(name);
		xvasprintf(&envent->value, fmt, ap);
		RB_INSERT(environ, env, envent);
	}
	va_end(ap);
}

/* Log the environment. */
void
environ_log(struct environ *env, const char *fmt, ...)
{
	struct environ_entry	*envent;
	va_list			 ap;
	char			*prefix;

	va_start(ap, fmt);
	vasprintf(&prefix, fmt, ap);
	va_end(ap);

	RB_FOREACH(envent, environ, env) {
		if (envent->value != NULL && *envent->name != '\0') {
			log_debug("%s%s=%s", prefix, envent->name,
			    envent->value);
		}
	}

	free(prefix);
}

/* Add a key-value pair. */
void
format_add(struct format_tree *ft, const char *key, const char *fmt, ...)
{
	struct format_entry	*fe;
	struct format_entry	*fe_now;
	va_list			 ap;

	fe = xmalloc(sizeof *fe);
	fe->key = xstrdup(key);

	fe_now = RB_INSERT(format_entry_tree, &ft->tree, fe);
	if (fe_now != NULL) {
		free(fe->key);
		free(fe);
		free(fe_now->value);
		fe = fe_now;
	}

	fe->cb = NULL;
	fe->t = 0;

	va_start(ap, fmt);
	xvasprintf(&fe->value, fmt, ap);
	va_end(ap);
}

void
hooks_insert(struct hooks *hooks, struct cmdq_item *item,
    struct cmd_find_state *fs, const char *fmt, ...)
{
	struct hook		*hook;
	va_list			 ap;
	char			*name;
	struct cmdq_item	*new_item;

	if (item->flags & CMDQ_NOHOOKS)
		return;

	va_start(ap, fmt);
	xvasprintf(&name, fmt, ap);
	va_end(ap);

	hook = hooks_find(hooks, name);
	if (hook == NULL) {
		free(name);
		return;
	}
	log_debug("running hook %s (parent %p)", name, item);

	new_item = cmdq_get_command(hook->cmdlist, fs, NULL, CMDQ_NOHOOKS);
	cmdq_format(new_item, "hook", "%s", name);
	if (item != NULL)
		cmdq_insert_after(item, new_item);
	else
		cmdq_append(NULL, new_item);

	free(name);
}

/* Reply to terminal query. */
void
input_reply(struct input_ctx *ictx, const char *fmt, ...)
{
	va_list	ap;
	char   *reply;

	va_start(ap, fmt);
	xvasprintf(&reply, fmt, ap);
	va_end(ap);

	bufferevent_write(ictx->wp->event, reply, strlen(reply));
	free(reply);
}

/* Log a debug message. */
void
log_debug(const char *msg, ...)
{
	va_list	ap;

	va_start(ap, msg);
	log_vwrite(msg, ap);
	va_end(ap);
}

/* Log a critical error with error string and die. */
__dead void
fatal(const char *msg, ...)
{
	char	*fmt;
	va_list	 ap;

	va_start(ap, msg);
	if (asprintf(&fmt, "fatal: %s: %s", msg, strerror(errno)) == -1)
		exit(1);
	log_vwrite(fmt, ap);
	va_end(ap);
	exit(1);
}

/* Log a critical error and die. */
__dead void
fatalx(const char *msg, ...)
{
	char	*fmt;
	va_list	 ap;

	va_start(ap, msg);
	if (asprintf(&fmt, "fatal: %s", msg) == -1)
		exit(1);
	log_vwrite(fmt, ap);
	va_end(ap);
	exit(1);
}

struct options_entry *
options_set_string(struct options *oo, const char *name, int append,
    const char *fmt, ...)
{
	struct options_entry	*o;
	va_list			 ap;
	char			*s, *value;

	va_start(ap, fmt);
	xvasprintf(&s, fmt, ap);
	va_end(ap);

	o = options_get_only(oo, name);
	if (o != NULL && append && OPTIONS_IS_STRING(o)) {
		xasprintf(&value, "%s%s", o->string, s);
		free(s);
	} else
		value = s;
	if (o == NULL && *name == '@')
		o = options_add(oo, name);
	else if (o == NULL) {
		o = options_default(oo, options_parent_table_entry(oo, name));
		if (o == NULL)
			return (NULL);
	}

	if (!OPTIONS_IS_STRING(o))
		fatalx("option %s is not a string", name);
	free(o->string);
	o->string = value;
	return (o);
}

/* Calculate string length, with embedded formatting. */
size_t
screen_write_cstrlen(const char *fmt, ...)
{
	va_list	ap;
	char   *msg, *msg2, *ptr, *ptr2;
	size_t	size;

	va_start(ap, fmt);
	xvasprintf(&msg, fmt, ap);
	va_end(ap);
	msg2 = xmalloc(strlen(msg) + 1);

	ptr = msg;
	ptr2 = msg2;
	while (*ptr != '\0') {
		if (ptr[0] == '#' && ptr[1] == '[') {
			while (*ptr != ']' && *ptr != '\0')
				ptr++;
			if (*ptr == ']')
				ptr++;
			continue;
		}
		*ptr2++ = *ptr++;
	}
	*ptr2 = '\0';

	size = screen_write_strlen("%s", msg2);

	free(msg);
	free(msg2);

	return (size);
}

/* Calculate string length. */
size_t
screen_write_strlen(const char *fmt, ...)
{
	va_list			ap;
	char   	       	       *msg;
	struct utf8_data	ud;
	u_char 	      	       *ptr;
	size_t			left, size = 0;
	enum utf8_state		more;

	va_start(ap, fmt);
	xvasprintf(&msg, fmt, ap);
	va_end(ap);

	ptr = msg;
	while (*ptr != '\0') {
		if (*ptr > 0x7f && utf8_open(&ud, *ptr) == UTF8_MORE) {
			ptr++;

			left = strlen(ptr);
			if (left < (size_t)ud.size - 1)
				break;
			while ((more = utf8_append(&ud, *ptr)) == UTF8_MORE)
				ptr++;
			ptr++;

			if (more == UTF8_DONE)
				size += ud.width;
		} else {
			if (*ptr > 0x1f && *ptr < 0x7f)
				size++;
			ptr++;
		}
	}

	free(msg);
	return (size);
}

/* Write simple string (no UTF-8 or maximum length). */
void
screen_write_puts(struct screen_write_ctx *ctx, const struct grid_cell *gcp,
    const char *fmt, ...)
{
	va_list	ap;

	va_start(ap, fmt);
	screen_write_vnputs(ctx, -1, gcp, fmt, ap);
	va_end(ap);
}

/* Write string with length limit (-1 for unlimited). */
void
screen_write_nputs(struct screen_write_ctx *ctx, ssize_t maxlen,
    const struct grid_cell *gcp, const char *fmt, ...)
{
	va_list	ap;

	va_start(ap, fmt);
	screen_write_vnputs(ctx, maxlen, gcp, fmt, ap);
	va_end(ap);
}

/* Write string, similar to nputs, but with embedded formatting (#[]). */
void
screen_write_cnputs(struct screen_write_ctx *ctx, ssize_t maxlen,
    const struct grid_cell *gcp, const char *fmt, ...)
{
	struct grid_cell	 gc;
	struct utf8_data	*ud = &gc.data;
	va_list			 ap;
	char			*msg;
	u_char 			*ptr, *last;
	size_t			 left, size = 0;
	enum utf8_state		 more;

	memcpy(&gc, gcp, sizeof gc);

	va_start(ap, fmt);
	xvasprintf(&msg, fmt, ap);
	va_end(ap);

	ptr = msg;
	while (*ptr != '\0') {
		if (ptr[0] == '#' && ptr[1] == '[') {
			ptr += 2;
			last = ptr + strcspn(ptr, "]");
			if (*last == '\0') {
				/* No ]. Not much point in doing anything. */
				break;
			}
			*last = '\0';

			style_parse(gcp, &gc, ptr);
			ptr = last + 1;
			continue;
		}

		if (*ptr > 0x7f && utf8_open(ud, *ptr) == UTF8_MORE) {
			ptr++;

			left = strlen(ptr);
			if (left < (size_t)ud->size - 1)
				break;
			while ((more = utf8_append(ud, *ptr)) == UTF8_MORE)
				ptr++;
			ptr++;

			if (more != UTF8_DONE)
				continue;
			if (maxlen > 0 && size + ud->width > (size_t)maxlen) {
				while (size < (size_t)maxlen) {
					screen_write_putc(ctx, &gc, ' ');
					size++;
				}
				break;
			}
			size += ud->width;
			screen_write_cell(ctx, &gc);
		} else {
			if (maxlen > 0 && size + 1 > (size_t)maxlen)
				break;

			if (*ptr > 0x1f && *ptr < 0x7f) {
				size++;
				screen_write_putc(ctx, &gc, *ptr);
			}
			ptr++;
		}
	}

	free(msg);
}

/* Add to client message log. */
void
server_client_add_message(struct client *c, const char *fmt, ...)
{
	struct message_entry	*msg, *msg1;
	char			*s;
	va_list			 ap;
	u_int			 limit;

	va_start(ap, fmt);
	xvasprintf(&s, fmt, ap);
	va_end(ap);

	log_debug("message %s (client %p)", s, c);

	msg = xcalloc(1, sizeof *msg);
	msg->msg_time = time(NULL);
	msg->msg_num = c->message_next++;
	msg->msg = s;
	TAILQ_INSERT_TAIL(&c->message_log, msg, entry);

	limit = options_get_number(global_options, "message-limit");
	TAILQ_FOREACH_SAFE(msg, &c->message_log, entry, msg1) {
		if (msg->msg_num + limit >= c->message_next)
			break;
		free(msg->msg);
		TAILQ_REMOVE(&c->message_log, msg, entry);
		free(msg);
	}
}

/* Set a status line message. */
void
status_message_set(struct client *c, const char *fmt, ...)
{
	struct timeval	tv;
	va_list		ap;
	int		delay;

	status_message_clear(c);

	if (c->status.old_status == NULL) {
		c->status.old_status = xmalloc(sizeof *c->status.old_status);
		memcpy(c->status.old_status, &c->status.status,
		    sizeof *c->status.old_status);
		screen_init(&c->status.status, c->tty.sx, 1, 0);
	}

	va_start(ap, fmt);
	xvasprintf(&c->message_string, fmt, ap);
	va_end(ap);

	server_client_add_message(c, "%s", c->message_string);

	delay = options_get_number(c->session->options, "display-time");
	if (delay > 0) {
		tv.tv_sec = delay / 1000;
		tv.tv_usec = (delay % 1000) * 1000L;

		if (event_initialized(&c->message_timer))
			evtimer_del(&c->message_timer);
		evtimer_set(&c->message_timer, status_message_callback, c);
		evtimer_add(&c->message_timer, &tv);
	}

	c->tty.flags |= (TTY_NOCURSOR|TTY_FREEZE);
	c->flags |= CLIENT_STATUS;
}

/* Show error from command. */
void
cmdq_error(struct cmdq_item *item, const char *fmt, ...)
{
	struct client	*c = item->client;
	struct cmd	*cmd = item->cmd;
	va_list		 ap;
	char		*msg;
	size_t		 msglen;
	char		*tmp;

	va_start(ap, fmt);
	msglen = xvasprintf(&msg, fmt, ap);
	va_end(ap);

	log_debug("%s: %s", __func__, msg);

	if (c == NULL)
		cfg_add_cause("%s:%u: %s", cmd->file, cmd->line, msg);
	else if (c->session == NULL || (c->flags & CLIENT_CONTROL)) {
		if (~c->flags & CLIENT_UTF8) {
			tmp = msg;
			msg = utf8_sanitize(tmp);
			free(tmp);
			msglen = strlen(msg);
		}
		evbuffer_add(c->stderr_data, msg, msglen);
		evbuffer_add(c->stderr_data, "\n", 1);
		server_client_push_stderr(c);
		c->retval = 1;
	} else {
		*msg = toupper((u_char) *msg);
		status_message_set(c, "%s", msg);
	}

	free(msg);
}

void
window_copy_add(struct window_pane *wp, const char *fmt, ...)
{
	va_list	ap;

	va_start(ap, fmt);
	window_copy_vadd(wp, fmt, ap);
	va_end(ap);
}

int
xasprintf(char **ret, const char *fmt, ...)
{
	va_list ap;
	int i;

	va_start(ap, fmt);
	i = xvasprintf(ret, fmt, ap);
	va_end(ap);

	return i;
}

int
xsnprintf(char *str, size_t len, const char *fmt, ...)
{
	va_list ap;
	int i;

	va_start(ap, fmt);
	i = xvsnprintf(str, len, fmt, ap);
	va_end(ap);

	return i;
}
