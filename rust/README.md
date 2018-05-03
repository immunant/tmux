# tmux-rs

tmux-rs has a yet unidentified bug(s) in the server component(s). This means at the moment you cannot use tmux-rs alone.
However, because tmux-rs client is functional, you can detach from an existing tmux session and attach to it with tmux-rs like so:

```shell
$ tmux
$ tmux detach
$ ./tmux-rs attach
```

To build or execute you can simply use cargo: `cargo build` / `cargo run` or grab the executable from the `target/` directory.
