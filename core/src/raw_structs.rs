use arcdps_imgui::sys::{ImFont, ImGuiContext};
use bitflags::bitflags;
use std::os::raw::{c_char, c_int, c_short, c_uint, c_ushort, c_void};
use winapi::um::d3d11::ID3D11ShaderResourceView;

pub type HMODULE = *mut c_void;
pub type LPVOID = *mut c_void;
pub type HWND = *mut c_void;
pub type UINT = u32;
pub type WPARAM = usize;
pub type LPARAM = isize;

// Your addon must use the same IMGUI Version 1.80
pub const IMGUI_VERSION_NUM: u32 = 18000;
pub const NEXUS_API_VERSION: i32 = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ERenderType {
    PreRender = 0,
    Render = 1,
    PostRender = 2,
    OptionsRender = 3,
}

pub type GuiRender = unsafe extern "C" fn();
pub type GuiAddrender = unsafe extern "C" fn(aRenderType: ERenderType, aRenderCallback: GuiRender);
pub type GuiRemrender = unsafe extern "C" fn(aRenderCallback: GuiRender);

pub type PathsGetgamedir = unsafe extern "C" fn() -> *const c_char;
pub type PathsGetaddondir = unsafe extern "C" fn(aName: *const c_char) -> *const c_char;
pub type PathsGetcommondir = unsafe extern "C" fn() -> *const c_char;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EMHStatus {
    MhUnknown = -1,
    MhOk = 0,
    MhErrorAlreadyInitialized,
    MhErrorNotInitialized,
    MhErrorAlreadyCreated,
    MhErrorNotCreated,
    MhErrorEnabled,
    MhErrorDisabled,
    MhErrorNotExecutable,
    MhErrorUnsupportedFunction,
    MhErrorMemoryAlloc,
    MhErrorMemoryProtect,
    MhErrorModuleNotFound,
    MhErrorFunctionNotFound,
}

pub type MinhookCreate =
    unsafe extern "C" fn(pTarget: LPVOID, pDetour: LPVOID, ppOriginal: *mut LPVOID) -> EMHStatus;
pub type MinhookRemove = unsafe extern "C" fn(pTarget: LPVOID) -> EMHStatus;
pub type MinhookEnable = unsafe extern "C" fn(pTarget: LPVOID) -> EMHStatus;
pub type MinhookDisable = unsafe extern "C" fn(pTarget: LPVOID) -> EMHStatus;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ELogLevel {
    Off = 0,
    Critical = 1,
    Warning = 2,
    INFO = 3,
    DEBUG = 4,
    TRACE = 5,
    ALL,
}

pub type LoggerLoga = unsafe extern "C" fn(aLogLevel: ELogLevel, aStr: *const c_char);
pub type EventConsume = unsafe extern "C" fn(aEventArgs: *mut c_void);
pub type EventsRaise = unsafe extern "C" fn(aIdentifier: *const c_char, aEventData: *mut c_void);
pub type EventsSubscribe =
    unsafe extern "C" fn(aIdentifier: *const c_char, aConsumeEventCallback: EventConsume);

pub type WndprocCallback =
    unsafe extern "C" fn(hWnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> UINT;
pub type WndprocAddrem = unsafe extern "C" fn(aWndProcCallback: WndprocCallback);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Keybind {
    pub key: c_ushort,
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
}

pub type KeybindsProcess = unsafe extern "C" fn(aIdentifier: *const c_char);
pub type KeybindsRegisterwithstring = unsafe extern "C" fn(
    aIdentifier: *const c_char,
    aKeybindHandler: KeybindsProcess,
    aKeybind: *const c_char,
);
pub type KeybindsRegisterwithstruct = unsafe extern "C" fn(
    aIdentifier: *const c_char,
    aKeybindHandler: KeybindsProcess,
    aKeybind: Keybind,
);
pub type KeybindsUnregister = unsafe extern "C" fn(aIdentifier: *const c_char);

pub type DatalinkGetresource = unsafe extern "C" fn(aIdentifier: *const c_char) -> *mut c_void;
pub type DatalinkShareresource =
    unsafe extern "C" fn(aIdentifier: *const c_char, aResourceSize: usize) -> *mut c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Texture {
    pub width: c_uint,
    pub height: c_uint,
    pub resource: *mut ID3D11ShaderResourceView,
}

pub type TexturesReceivecallback =
    unsafe extern "C" fn(aIdentifier: *const c_char, aTexture: *mut Texture);
pub type TexturesGet = unsafe extern "C" fn(aIdentifier: *const c_char) -> *mut Texture;
pub type TexturesLoadfromfile = unsafe extern "C" fn(
    aIdentifier: *const c_char,
    aFilename: *const c_char,
    aCallback: TexturesReceivecallback,
);
pub type TexturesLoadfromresource = unsafe extern "C" fn(
    aIdentifier: *const c_char,
    aResourceID: c_uint,
    aModule: HMODULE,
    aCallback: TexturesReceivecallback,
);
pub type TexturesLoadfromurl = unsafe extern "C" fn(
    aIdentifier: *const c_char,
    aRemote: *const c_char,
    aEndpoint: *const c_char,
    aCallback: TexturesReceivecallback,
);

pub type QuickaccessAddshortcut = unsafe extern "C" fn(
    aIdentifier: *const c_char,
    aTextureIdentifier: *const c_char,
    aTextureHoverIdentifier: *const c_char,
    aKeybindIdentifier: *const c_char,
    aTooltipText: *const c_char,
);
pub type QuickaccessAddsimple =
    unsafe extern "C" fn(aIdentifier: *const c_char, aShortcutRenderCallback: GuiRender);
pub type QuickaccessRemove = unsafe extern "C" fn(aIdentifier: *const c_char);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NexusLinkData {
    pub width: c_uint,
    pub height: c_uint,
    pub scaling: f32,
    pub is_moving: bool,
    pub is_camera_moving: bool,
    pub is_gameplay: bool,
    pub font: *mut ImFont,
    pub font_big: *mut ImFont,
    pub font_ui: *mut ImFont,
}

// Revision 1
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AddonAPI {
    /// Renderer
    // IDXGISwapChain*
    pub swap_chain: *mut c_void,
    // ImGuiContext*
    pub imgui_context: *mut ImGuiContext,
    pub imgui_malloc: unsafe extern "C" fn(usize, *mut c_void) -> *mut c_void,
    pub imgui_free: unsafe extern "C" fn(*mut c_void, *mut c_void),
    pub register_render: GuiAddrender,
    pub unregister_render: GuiRemrender,

    /// Paths
    pub get_game_directory: PathsGetgamedir,
    pub get_addon_directory: PathsGetaddondir,
    pub get_common_directory: PathsGetcommondir,
    /// Minhook
    pub create_hook: MinhookCreate,
    pub remove_hook: MinhookRemove,
    pub enable_hook: MinhookEnable,
    pub disable_hook: MinhookDisable,
    /// Logging
    pub log: LoggerLoga,
    /// Events
    pub raise_event: EventsRaise,
    pub subscribe_event: EventsSubscribe,
    pub unsubscribe_event: EventsSubscribe,
    /// WndProc
    pub register_wnd_proc: WndprocAddrem,
    pub unregister_wnd_proc: WndprocAddrem,
    /// Keybinds
    pub register_keybind_with_string: KeybindsRegisterwithstring,
    pub register_keybind_with_struct: KeybindsRegisterwithstruct,
    pub unregister_keybind: KeybindsUnregister,
    /// DataLink
    pub get_resource: DatalinkGetresource,
    pub share_resource: DatalinkShareresource,
    /// Textures
    pub get_texture: TexturesGet,
    pub load_texture_from_file: TexturesLoadfromfile,
    pub load_texture_from_resource: TexturesLoadfromresource,
    pub load_texture_from_url: TexturesLoadfromurl,
    /// Shortcuts
    pub add_shortcut: QuickaccessAddshortcut,
    pub remove_shortcut: QuickaccessRemove,
    pub add_simple_shortcut: QuickaccessAddsimple,
    pub remove_simple_shortcut: QuickaccessRemove,
}

pub type AddonLoad = unsafe extern "C" fn(aAPI: *mut AddonAPI);
pub type AddonUnload = unsafe extern "C" fn();

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AddonVersion {
    pub major: c_short,
    pub minor: c_short,
    pub build: c_short,
    pub revision: c_short,
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct EAddonFlags: c_int {
        const None = 0;
        /// is hooking functions or doing anything else that's volatile and game build dependant
        const IsVolatile = 1;
        /// prevents unloading at runtime, aka. will require a restart if updated, etc.
        const DisableHotloading = 2;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EUpdateProvider {
    /// Does not support auto updating
    None = 0,
    /// Provider is Raidcore (via API)
    Raidcore = 1,
    /// Provider is GitHub Releases
    GitHub = 2,
    /// Provider is direct file link
    Direct = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AddonDefinition {
    /// required
    /// Raidcore Addon ID, set to random unqiue negative integer if not on Raidcore
    pub signature: c_int,
    /// Determines which AddonAPI struct revision the Loader will pass, use the NEXUS_API_VERSION define from Nexus.h
    pub apiversion: c_int,
    /// Name of the addon as shown in the library
    pub name: *const c_char,
    pub version: AddonVersion,
    /// Author of the addon
    pub author: *const c_char,
    /// Short description
    pub description: *const c_char,
    /// Pointer to Load Function of the addon
    pub load: AddonLoad,
    /// Pointer to Unload Function of the addon. Not required if EAddonFlags::DisableHotloading is set.
    pub unload: Option<AddonUnload>,
    /// Information about the addon
    pub flags: EAddonFlags,
    /// What platform is the the addon hosted on
    pub provider: EUpdateProvider,
    /// Link to the update resource
    pub update_link: Option<*const c_char>,
}

unsafe impl Sync for AddonDefinition {}
