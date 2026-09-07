#![allow(unexpected_cfgs)]
mod protocol;
mod opts;
mod wkwebview;
mod cfnetwork;

// Punct de intrare standard exportat pe care loader-ul îl poate apela în siguranță
#[no_mangle]
pub extern "C" fn moonwave_entry() {
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
