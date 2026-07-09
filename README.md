# tmux-info

Cross-platform tool that prints system information for use in tmux statuslines.

## Commands

| Command    | Description                        |
|------------|------------------------------------|
| `user`     | Print the current username         |
| `hostname` | Print the short hostname           |
| `ip`       | Print local non-loopback IPv4 addresses |

Output has no trailing newline, making it suitable for embedding in status bars.

## Usage

```sh
tmux-info user      # jdoe
tmux-info hostname  # myhost
tmux-info ip        # 192.168.1.5
```

In `~/.tmux.conf`:

```
set -g status-right "#[fg=cyan]$(tmux-info user)@$(tmux-info hostname) #[fg=green]$(tmux-info ip)"
```

## Install

```sh
cargo install --path .
```

Requires Rust. Pin the toolchain with `mise.toml` or use any recent stable Rust.
