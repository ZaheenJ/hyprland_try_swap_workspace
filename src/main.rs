use std::{env, process};
use hyprland::{prelude::*, data::*, dispatch, dispatch::{Dispatch, DispatchType}};

const HELP_STR: &str = "Usage: try_swap_workspace <workspace number>";

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_workspace = args.get(1).unwrap_or(&String::new()).parse::<i32>();
    if args.len() != 2 || target_workspace.is_err() {
        println!("{}", HELP_STR);
        process::exit(1);
    }
    let target_ws = target_workspace.unwrap();

    let mons = Monitors::get().unwrap().to_vec();
    let target_mon = mons.iter().find(|m| m.focused).unwrap().id;
    let target_ws_cur_active_mon = mons.iter().find(|m| m.active_workspace.id == target_ws);

    if target_ws_cur_active_mon.is_some() {
        dispatch!(SwapActiveWorkspaces, dispatch::MonitorIdentifier::Id(target_mon.try_into().unwrap()), dispatch::MonitorIdentifier::Id(target_ws_cur_active_mon.unwrap().id.try_into().unwrap()));
    } else {
        dispatch!(MoveWorkspaceToMonitor, dispatch::WorkspaceIdentifier::Id(target_ws), dispatch::MonitorIdentifier::Id(target_mon.try_into().unwrap()));
        dispatch!(Workspace, dispatch::WorkspaceIdentifierWithSpecial::Id(target_ws));
    }
}
