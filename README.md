# try_swap_workspace

A small program to provide shared workspaces between all monitors for Hyprland (like XMonad or QTile). 

# Installation

### Manual Installation
```
git clone https://github.com/ZaheenJ/try_swap_workspace
cd try_swap_workspace
cargo build --release
sudo mv target/release/try_swap_workspace /usr/local/bin/
```
# Usage

Syntax: `try_swap_workspace <workspace>`

So in your hyprland.conf, instead of:
```
# Switch workspaces with mainMod + [0-9]
bind = $mainMod, 1, workspace, 1
bind = $mainMod, 2, workspace, 2
bind = $mainMod, 3, workspace, 3
bind = $mainMod, 4, workspace, 4
bind = $mainMod, 5, workspace, 5
bind = $mainMod, 6, workspace, 6
bind = $mainMod, 7, workspace, 7
bind = $mainMod, 8, workspace, 8
bind = $mainMod, 9, workspace, 9
bind = $mainMod, 0, workspace, 10
```
do:
```
# Switch workspaces with main_mod + [0-9]
bind = $main_mod, 1, exec, try_swap_workspace 1
bind = $main_mod, 2, exec, try_swap_workspace 2
bind = $main_mod, 3, exec, try_swap_workspace 3
bind = $main_mod, 4, exec, try_swap_workspace 4
bind = $main_mod, 5, exec, try_swap_workspace 5
bind = $main_mod, 6, exec, try_swap_workspace 6
bind = $main_mod, 7, exec, try_swap_workspace 7
bind = $main_mod, 8, exec, try_swap_workspace 8
bind = $main_mod, 9, exec, try_swap_workspace 9
bind = $main_mod, 0, exec, try_swap_workspace 10
```
# But [try_swap_workspace](https://github.com/hyprwm/contrib/blob/main/try_swap_workspace/try_swap_workspace) already exists

And it works great! However I felt like I could notice a bit of lag especially when switching workspaces fast, so I rewrote it in rust 🤷‍♂️.

### Hyperfine Benchmark

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `try_swap_workspace 1 && try_swap_workspace 7` | 162.6 ± 56.7 | 5.2 | 319.8 | 1.00 |
| `try_swap_workspace_bash 1 && try_swap_workspace_bash 7` | 436.8 ± 47.0 | 356.2 | 486.1 | 2.69 ± 0.98 |
