#![allow(unexpected_cfgs)]
mod protocol;
mod opts;
mod wkwebview;
mod cfnetwork;

// Folosim o funcție constructor nativă compatibilă cu Objective-C / iOS
#[link_section = "__DATA,__mod_init_func"]
#[used]
pub static INIT: extern "C" fn() = {
    extern "C" fn init() {
        unsafe {
            protocol::init_moonwave_url_protocol();
        }

        if opts::USE_PARTYHUB {
            unsafe {
                wkwebview::init_moonwave_webview_delegate();
                cfnetwork::init_moonwave_cfnetwork_hook();
            }
        }
    }
    init
};
