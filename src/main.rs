use hyprland::data::*;
use hyprland::prelude::*;
use hyprland::dispatch::*;
use hyprland::Result;

struct MyMon {
    name: String,
    focused: bool,
}

fn main() -> Result<()> {


    let monitors = list_mons()?;

    for mon in monitors {
        if !mon.focused {
            Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Name(&mon.name)) )?;
        }
    }


    Ok(())
}

fn list_mons() -> Result<Vec<MyMon>> {
    let monitors = Monitors::get()?.to_vec();

    let mut mons_name = Vec::new();

    for monitor in monitors {
        mons_name.push(MyMon {
            name: monitor.name.clone(),
            focused: monitor.focused
        });
    }

    return Ok(mons_name);
}
