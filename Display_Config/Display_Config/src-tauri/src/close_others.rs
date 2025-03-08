use sysinfo::{Pid, System};

#[tauri::command]
pub fn close_others() -> Result<(), String> {
    let mut system = System::new_all();
    system.refresh_all();
    let pids = get_pids(&system);
    println!("PIDs: {:?}", pids);
    for (_, pid) in pids {
        kill_recursive(&system, pid);
    }
    Ok(())
}

fn get_pids(system: &System) -> Vec<(String, Pid)> {
    let current_pid = std::process::id();
    let parent_pid = system
        .process(sysinfo::Pid::from_u32(current_pid))
        .and_then(|p| p.parent())
        .map(|p| p.clone());

    let targets = ["Supreme.exe", "Supreme_v1.035.exe"];

    system
        .processes()
        .iter()
        .filter(|(pid, process)| {
            targets.contains(&process.name().to_string_lossy().as_ref())
                && Some(**pid) != parent_pid
        })
        .map(|(pid, process)| (process.name().to_string_lossy().to_string(), *pid))
        .collect()
}

fn kill_recursive(system: &System, target_pid: Pid) {
    let children: Vec<Pid> = system
        .processes()
        .iter()
        .filter(|(_, process)| {
            process
                .parent()
                .map_or(false, |parent| parent == target_pid)
        })
        .map(|(pid, _)| *pid)
        .collect();

    for child_pid in children {
        kill_recursive(system, child_pid);
    }

    if let Some(process) = system.process(target_pid) {
        process.kill();
    }
}
