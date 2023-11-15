use arcdps_imgui::{
    self,
    sys::{igSetAllocatorFunctions, igSetCurrentContext},
    Context, Ui, Window,
};
use std::{
    ffi::{c_char, c_void},
    mem::MaybeUninit,
    ptr,
};

use nexus_rs::raw_structs::{
    AddonAPI, AddonDefinition, AddonVersion, EAddonFlags, ELogLevel, ERenderType,
};

static mut API: MaybeUninit<&'static AddonAPI> = MaybeUninit::uninit();
static mut CTX: MaybeUninit<Context> = MaybeUninit::uninit();
static mut UI: MaybeUninit<Ui> = MaybeUninit::uninit();

unsafe extern "C" fn load(a_api: *mut AddonAPI) {
    let api = &*a_api;
    API.write(&api);
    (api.log)(ELogLevel::ALL, b"YepgeLog\0".as_ptr() as *const c_char);

    igSetCurrentContext(api.imgui_context);
    igSetAllocatorFunctions(
        Some(api.imgui_malloc),
        Some(api.imgui_free),
        ptr::null::<c_void>() as *mut _,
    );

    CTX.write(Context::current());
    UI.write(Ui::from_ctx(CTX.assume_init_ref()));

    // Add an options window and a regular render callback
    (api.register_render)(ERenderType::Render, render);
    (api.register_render)(ERenderType::OptionsRender, render_options);

    (api.log)(
        ELogLevel::DEBUG,
        b"My first addon was loaded.\0".as_ptr() as _,
    );
}
unsafe extern "C" fn unload() {
    (API.assume_init().unregister_render)(render);
    (API.assume_init().unregister_render)(render_options);
}

pub unsafe extern "C" fn render() {
    let ui = UI.assume_init_ref();
    static mut OPENED: bool = true;

    if !OPENED {
        return;
    }
    if let Some(w) = Window::new("Test Window Yepge")
        .opened(&mut OPENED)
        .begin(ui)
    {
        ui.text("Tschüss");
        w.end();
    }
}
pub unsafe extern "C" fn render_options() {
    let ui = UI.assume_init_ref();
    static mut STATE: bool = false;
    ui.separator();
    ui.text_disabled("My first Nexus addon");
    ui.checkbox("Some setting", &mut STATE);
}

#[no_mangle]
pub extern "C" fn GetAddonDef() -> *mut AddonDefinition {
    let ad = AddonDefinition {
        signature: -42069,
        apiversion: nexus_rs::raw_structs::NEXUS_API_VERSION,
        name: b"MonkaTest\0".as_ptr() as *const c_char,
        version: AddonVersion {
            major: 0,
            minor: 1,
            build: 0,
            revision: 0,
        },
        author: b"belst\0".as_ptr() as *const c_char,
        description: b"whatever\0".as_ptr() as *const c_char,
        load,
        unload,
        flags: EAddonFlags::None,
        provider: nexus_rs::raw_structs::EUpdateProvider::None,
        update_link: None,
    };

    let ptr = Box::new(ad);
    Box::leak(ptr)
}
