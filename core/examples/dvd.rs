use arcdps_imgui::{
    self,
    sys::{igSetAllocatorFunctions, igSetCurrentContext},
    Context, Image, Slider, Ui, Window,
};
use nexus_rs::raw_structs::{
    AddonAPI, AddonDefinition, AddonVersion, EAddonFlags, ELogLevel, ERenderType, NexusLinkData,
    Texture,
};
use once_cell::sync::Lazy;
use rand::Rng;
use std::{
    arch::x86_64::_SIDD_POSITIVE_POLARITY,
    ffi::{c_char, c_void},
    mem::MaybeUninit,
    ptr,
};

static mut API: MaybeUninit<&'static AddonAPI> = MaybeUninit::uninit();
static mut CTX: MaybeUninit<Context> = MaybeUninit::uninit();
static mut UI: MaybeUninit<Ui> = MaybeUninit::uninit();
static mut NEXUS_DATA: Option<&'static NexusLinkData> = None;
static mut DVD_ICON: Option<&'static Texture> = None;

unsafe extern "C" fn load(a_api: *mut AddonAPI) {
    let api = &*a_api;
    API.write(&api);

    igSetCurrentContext(api.imgui_context);
    igSetAllocatorFunctions(
        Some(api.imgui_malloc),
        Some(api.imgui_free),
        ptr::null::<c_void>() as *mut _,
    );

    CTX.write(Context::current());
    UI.write(Ui::from_ctx(CTX.assume_init_ref()));
    let data = (api.get_resource)(b"DL_NEXUS_LINK\0" as *const _ as _) as *const NexusLinkData;
    if data.is_null() {
        (api.log)(
            ELogLevel::CRITICAL,
            b"Could not find DL_NEXUS_LINK data.\0".as_ptr() as _,
        );
    } else {
        NEXUS_DATA = Some(&*data);
    }
    // static DVD_ICON_DATA: &'static [u8] = include_bytes!("dvd.png");
    let p: *const i8 = (api.get_addon_directory)(b"/dvd.png\0" as *const _ as _);

    unsafe extern "C" fn texture_callback(_: *const i8, text: *mut Texture) {
        DVD_ICON = Some(&*text);
    }
    (api.load_texture_from_file)(b"ICON_DVD\0" as *const _ as _, p, texture_callback);
    // Add an options window and a regular render callback
    (api.register_render)(ERenderType::Render, render);
    (api.register_render)(ERenderType::OptionsRender, render_options);

    (api.log)(ELogLevel::INFO, b"DVD addon was loaded.\0".as_ptr() as _);
}
unsafe extern "C" fn unload() {
    (API.assume_init().unregister_render)(render);
    (API.assume_init().unregister_render)(render_options);
}
static mut SPEED_VAL: i32 = 2;
static mut SPEED: [i32; 2] = [1, 1];

pub unsafe extern "C" fn render_options() {
    let ui = UI.assume_init_ref();

    ui.separator();
    Slider::new("DVD Speed", 1i32, 50).build(&ui, &mut SPEED_VAL);
    //ui.input_int("DVD Speed", &mut SPEED_VAL).build();
}
pub unsafe extern "C" fn render() {
    let ui = UI.assume_init_ref();
    if DVD_ICON.is_none() {
        return;
    }
    if NEXUS_DATA.is_none() {
        return;
    }
    static mut X: Lazy<i32> = Lazy::new(|| unsafe {
        rand::thread_rng().gen_range(0..(NEXUS_DATA.unwrap().width - DVD_ICON.unwrap().width)) as _
    });
    static mut Y: Lazy<i32> = Lazy::new(|| unsafe {
        rand::thread_rng().gen_range(0..(NEXUS_DATA.unwrap().height - DVD_ICON.unwrap().height))
            as _
    });
    static mut TINT_COLOR: [f32; 4] = [0., 0., 0., 1.];

    unsafe fn randomize_color() {
        let mut rng = rand::thread_rng();
        TINT_COLOR[0] = rng.gen_range(0. ..=1.);
        TINT_COLOR[1] = rng.gen_range(0. ..=1.);
        TINT_COLOR[2] = rng.gen_range(0. ..=1.);
    }
    if NEXUS_DATA.unwrap().is_gameplay {
        return;
    }
    if let Some(w) = Window::new("DVD")
        .no_decoration()
        .draw_background(false)
        .position([*X as _, *Y as _], arcdps_imgui::Condition::Always)
        .begin(ui)
    {
        Image::new(
            (DVD_ICON.unwrap().resource).into(),
            [DVD_ICON.unwrap().width as _, DVD_ICON.unwrap().height as _],
        )
        .tint_col(TINT_COLOR)
        .build(ui);
        w.end();
    }
    if *X <= 0 || *X >= NEXUS_DATA.unwrap().width as i32 - DVD_ICON.unwrap().width as i32 {
        SPEED[0] = -SPEED[0];
        randomize_color();
    }
    if *Y <= 0 || *Y >= NEXUS_DATA.unwrap().height as i32 - DVD_ICON.unwrap().height as i32 {
        SPEED[1] = -SPEED[1];
        randomize_color();
    }
    *X += SPEED_VAL * SPEED[0];
    *Y += SPEED_VAL * SPEED[1];
}

#[no_mangle]
pub extern "C" fn GetAddonDef() -> *mut AddonDefinition {
    static AD: AddonDefinition = AddonDefinition {
        signature: -69420,
        apiversion: nexus_rs::raw_structs::NEXUS_API_VERSION,
        name: b"DVD\0".as_ptr() as *const c_char,
        version: AddonVersion {
            major: 0,
            minor: 3,
            build: 0,
            revision: 0,
        },
        author: b"belst\0".as_ptr() as *const c_char,
        description: b"Bouncy\0".as_ptr() as *const c_char,
        load,
        unload: Some(unload),
        flags: EAddonFlags::None,
        provider: nexus_rs::raw_structs::EUpdateProvider::None,
        update_link: None,
    };

    &AD as *const _ as _
}
