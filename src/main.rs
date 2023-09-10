use hyprland::{
    data::*,
    dispatch,
    dispatch::{Dispatch, DispatchType},
    prelude::*,
};
use std::{env, process};

const HELP_STR: &str = "Usage: try_swap_workspace <workspace number>";

fn main() {
    // From hyprland-rs get_socket_path()
    match env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(_) => (),
        Err(env::VarError::NotPresent) => {
            eprintln!("Is Hyprland running?");
            process::exit(1);
        }
        Err(env::VarError::NotUnicode(_)) => panic!("wtf no unicode?"),
    };

    let args: Vec<String> = env::args().collect();
    let target_workspace = args.get(1).unwrap_or(&String::new()).parse::<i32>();
    if args.len() != 2 || target_workspace.is_err() {
        eprintln!("{}", HELP_STR);
        process::exit(1);
    }
    let target_ws = target_workspace.expect("Workspace id too large.");

    // Should never fail if Hyprland is running
    let mons = Monitors::get().expect("Failed to fetch monitors.").to_vec();
    let target_mon = mons
        .iter()
        .find(|m| m.focused)
        .expect("No active monitor?")
        .id;
    let target_ws_cur_active_mon = mons.iter().find(|m| m.active_workspace.id == target_ws);

    if target_ws_cur_active_mon.is_some() {
        dispatch!(
            SwapActiveWorkspaces,
            dispatch::MonitorIdentifier::Id(target_mon.try_into().unwrap()),
            dispatch::MonitorIdentifier::Id(
                target_ws_cur_active_mon.unwrap().id.try_into().unwrap()
            )
        )
        .expect("Hyprctl dispatch SwapActiveWorkspaces failed!");
    } else {
        dispatch!(
            MoveWorkspaceToMonitor,
            dispatch::WorkspaceIdentifier::Id(target_ws),
            dispatch::MonitorIdentifier::Id(target_mon.try_into().unwrap())
        )
        .expect("Hyprctl disptach MoveWorkspaceToMonitor failed!");
        dispatch!(
            Workspace,
            dispatch::WorkspaceIdentifierWithSpecial::Id(target_ws)
        )
        .expect("Hyprland dispatch Workspace failed!");
    }
}
