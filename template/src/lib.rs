use quest_hook::hook;
use quest_hook::libil2cpp::{Il2CppObject, Il2CppString};
use tracing::debug;

#[no_mangle]
pub extern "C" fn setup() {
    quest_hook::setup("#{ID}");
}

#[no_mangle]
pub extern "C" fn load() {
    debug!("Installing Hooks!");
    // Install your hook here!
    debug!("Installed All Hooks!")
}