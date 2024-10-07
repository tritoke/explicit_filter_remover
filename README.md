# explicit_filter_remover

Fix for ~/explicit_filter.restore being created by spotify

## Building:

Just `cargo build --release` is all you need

## Running:

I installed this via symlink to ~/.local/bin/explicit_filter_remover and then ran via this service snippet:

Create with `systemctl edit --force --user --full explicit-filter-remover.service`:

```systemd
[Unit]
Description=Remove ~/explicit_filter.restore

[Service]
Restart=always
ExecStart=/bin/sh -c '${HOME}/.local/bin/explicit_filter_remover'
```

Then you can start it with `systemctl enable --now explicit-filter-remover.service`.
