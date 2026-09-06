//! Manual shared-memory diagnostics. Never launches or restarts the game implicitly.
use tas_shared::{TasCommand, TasSharedMemoryClient, TAS_SHARED_VERSION};

fn parse(args: &[String]) -> Result<Option<TasCommand>, String> {
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        [] => Ok(None),
        ["--command", name] => match *name {
            "record" => Ok(Some(TasCommand::ArmRec)),
            "play" => Ok(Some(TasCommand::ArmPlay)),
            "stop" => Ok(Some(TasCommand::Stop)),
            "restart" => Ok(Some(TasCommand::Restart)),
            _ => Err("command must be record, play, stop or restart (not a raw number)".into()),
        },
        _ => Err("Usage: tas_test shm [--command record|play|stop|restart]".into()),
    }
}

fn validate(version: u32, idle: bool, writing: bool) -> Result<(), String> {
    if version != TAS_SHARED_VERSION {
        return Err(format!(
            "Version mismatch: expected {TAS_SHARED_VERSION}, got {version}"
        ));
    }
    if writing && !idle {
        return Err("A command is pending; refusing to overwrite it".into());
    }
    Ok(())
}

pub fn run(args: &[String]) -> Result<(), String> {
    let command = parse(args)?;
    // open() checks the DLL protocol version before exposing the typed layout.
    let mut client = TasSharedMemoryClient::open()?;
    validate(
        client.state().version,
        client.command_idle(),
        command.is_some(),
    )?;
    if let Some(command) = command {
        client.send_command(command);
        println!("Published {command:?}; this is not an acknowledgement of completion.");
    }
    println!(
        "version={} command_idle={} renderer_id={} fpu_cw={:#06x}",
        client.state().version,
        client.command_idle(),
        client.renderer_id(),
        client.fpu_control_word()
    );
    println!("rider={:?}", tas_shared::rider_pair(client.state()));
    crate::harness::print_status(&client);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_read_only_and_raw_commands_are_rejected() {
        assert!(parse(&[]).unwrap().is_none());
        for name in ["record", "play", "stop", "restart"] {
            assert!(parse(&["--command".into(), name.into()]).unwrap().is_some());
        }
        for args in [
            vec!["--command", "5"],
            vec!["stop"],
            vec!["--command"],
            vec!["--command", "continue"],
        ] {
            assert!(parse(&args.into_iter().map(String::from).collect::<Vec<_>>()).is_err());
        }
    }

    #[test]
    fn wrong_version_and_busy_writes_are_rejected() {
        assert!(validate(TAS_SHARED_VERSION - 1, true, false).is_err());
        assert!(validate(TAS_SHARED_VERSION - 1, true, true).is_err());
        assert!(validate(TAS_SHARED_VERSION, false, true).is_err());
        assert!(validate(TAS_SHARED_VERSION, false, false).is_ok());
        assert!(validate(TAS_SHARED_VERSION, true, true).is_ok());
    }
}
