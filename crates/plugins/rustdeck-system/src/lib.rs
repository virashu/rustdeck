use rustdeck_common::{Plugin, actions, decl_action, decl_plugin};
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

fn get_variable(_: &PluginState, _: &str) -> String {
    String::new()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn build() -> *const Plugin {
    decl_plugin! {
        id: "rustdeck_system",
        name: "RustDeck System",
        desc: "System management plugin",
        variables: ::std::ptr::null(),
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
