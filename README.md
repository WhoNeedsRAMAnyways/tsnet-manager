# tsnet

Quickly swap between Tailscale networks (tailnets). Wraps `tailscale switch`, which keeps a login profile per tailnet.

## Setup

```
sudo tailscale set --operator=$USER   # once, so switching doesn't need sudo
ln -s "$PWD/tsnet" ~/.local/bin/tsnet
```

Log in to each tailnet without running `tailscale logout` first, so each login is saved as a profile.

## Usage

```
tsnet                 show current tailnet and the list
tsnet ls              list saved tailnets
tsnet <name>          switch (matches ID, tailnet name, or account; substring ok)
tsnet next            cycle to the next tailnet (handy with two)
tsnet add             log in to an additional tailnet
tsnet alias <a> <n>   save alias <a> for tailnet <n> (e.g. tsnet alias work corp.ts.net)
```
