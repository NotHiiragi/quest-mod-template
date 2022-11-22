#![feature(backtrace)]

// Including/importing stuff

#[no_mangle]
pub extern "C" fn setup(){
    //idk know, I am just looking at Ferns rust mods. but I saw quest_hook::setup!(); so ig if you use qh you add that here ¯\_(ツ)_/¯
}

#[no_mangle]
pub extern "C" fn load() {
    info!("Installing hooks!");
    //Your hooks here!
    info!("Installed hooks!");
}