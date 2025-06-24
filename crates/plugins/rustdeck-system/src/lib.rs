use rustdeck_common::{Plugin, actions, decl_action, decl_plugin, decl_variable, variables};
use system_shutdown::{reboot, shutdown};

struct PluginState {}

const fn init() -> PluginState {
    PluginState {}
}

const fn update(_: &PluginState) {}

fn run_action(_: &mut PluginState, id: &str) {
    match id {
        "shutdown" => {
            _ = shutdown();
        }
        "reboot" => {
            _ = reboot();
        }
        _ => {}
    }
}

fn get_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |t| t.as_secs())
}

fn get_variable(_: &PluginState, id: &str) -> String {
    match id {
        "time_hours" => ((get_time() / 3600) % 24).to_string(),
        "time_minutes" => ((get_time() / 60) % 60).to_string(),
        "time" => {
            let time = get_time();
            let minutes = (time / 60) % 60;
            let hours = (time / 3600) % 24;
            format!("{hours}:{minutes}")
        }
        _ => String::new(),
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn build() -> *const Plugin {
    decl_plugin! {
        id: "rustdeck_system",
        name: "RustDeck System",
        desc: "System management plugin",
        variables: variables!(
            decl_variable! {
                id: "time",
                desc: "System time (hh:mm)",
                vtype: "string"
            },
            decl_variable! {
                id: "time_hours",
                desc: "System time (hh)",
                vtype: "string"
            },
            decl_variable! {
                id: "time_minutes",
                desc: "System time (mm)",
                vtype: "string"
            },
        ),
        actions: actions!(
            decl_action! {
                id: "shutdown",
                name: "Shutdown",
                desc: "Shutdown the system"
            },
            decl_action! {
                id: "reboot",
                name: "Reboot",
                desc: "Reboot the system"
            },
        ),

        fn_init: init,
        fn_update: update,
        fn_get_variable: get_variable,
        fn_run_action: run_action,
    }
}
