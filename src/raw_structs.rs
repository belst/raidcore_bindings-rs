#![allow(clippy::upper_case_acronyms)]

use std::os::raw::{c_char, c_int, c_short, c_uint, c_ushort, c_void};

pub type HMODULE = *mut c_void;
pub type LPVOID = *mut c_void;
pub type HWND = *mut c_void;
pub type UINT = u32;
pub type WPARAM = usize;
pub type LPARAM = isize;

// Your addon must use the same IMGUI Version 1.80
pub const IMGUI_VERSION_NUM: u32 = 18000;
pub const NEXUS_API_VERSION: u32 = 1;

#[repr(C)]
enum ERenderType {
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
enum EMHStatus {
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

pub type MINHOOK_CREATE =
    unsafe extern "C" fn(pTarget: LPVOID, pDetour: LPVOID, ppOriginal: *mut LPVOID) -> EMHStatus;
pub type MINHOOK_REMOVE = unsafe extern "C" fn(pTarget: LPVOID) -> EMHStatus;
pub type MINHOOK_ENABLE = unsafe extern "C" fn(pTarget: LPVOID) -> EMHStatus;
pub type MINHOOK_DISABLE = unsafe extern "C" fn(pTarget: LPVOID) -> EMHStatus;

#[repr(C)]
enum ELogLevel {
    ELogLevel_OFF = 0,
    ELogLevel_CRITICAL = 1,
    ELogLevel_WARNING = 2,
    ELogLevel_INFO = 3,
    ELogLevel_DEBUG = 4,
    ELogLevel_TRACE = 5,
    ELogLevel_ALL,
}

pub type LOGGER_LOGA = unsafe extern "C" fn(aLogLevel: ELogLevel, aStr: *const c_char);
pub type EVENT_CONSUME = unsafe extern "C" fn(aEventArgs: *mut c_void);
pub type EVENTS_RAISE = unsafe extern "C" fn(aIdentifier: *const c_char, aEventData: *mut c_void);
pub type EVENTS_SUBSCRIBE =
    unsafe extern "C" fn(aIdentifier: *const c_char, aConsumeEventCallback: EVENT_CONSUME);

pub type WNDPROC_CALLBACK =
    unsafe extern "C" fn(hWnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> UINT;
pub type WNDPROC_ADDREM = unsafe extern "C" fn(aWndProcCallback: WNDPROC_CALLBACK);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Keybind {
    pub Key: c_ushort,
    pub Alt: bool,
    pub Ctrl: bool,
    pub Shift: bool,
}

pub type KEYBINDS_PROCESS = unsafe extern "C" fn(aIdentifier: *const c_char);
pub type KEYBINDS_REGISTERWITHSTRING = unsafe extern "C" fn(
    aIdentifier: *const c_char,
    aKeybindHandler: KEYBINDS_PROCESS,
    aKeybind: *const c_char,
);
pub type KEYBINDS_REGISTERWITHSTRUCT = unsafe extern "C" fn(
    aIdentifier: *const c_char,
    aKeybindHandler: KEYBINDS_PROCESS,
    aKeybind: Keybind,
);
pub type KEYBINDS_UNREGISTER = unsafe extern "C" fn(aIdentifier: *const c_char);

pub type DATALINK_GETRESOURCE = unsafe extern "C" fn(aIdentifier: *const c_char) -> *mut c_void;
pub type DATALINK_SHARERESOURCE =
    unsafe extern "C" fn(aIdentifier: *const c_char, aResourceSize: usize) -> *mut c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Texture {
    pub Width: c_uint,
    pub Height: c_uint,
    // ID3D11ShaderResourceView* Resource
    pub Resource: *mut c_void,
}

pub type TEXTURES_RECEIVECALLBACK =
    unsafe extern "C" fn(aIdentifier: *const c_char, aTexture: *mut Texture);
pub type TEXTURES_GET = unsafe extern "C" fn(aIdentifier: *const c_char) -> *mut Texture;
pub type TEXTURES_LOADFROMFILE = unsafe extern "C" fn(
    aIdentifier: *const c_char,
    aFilename: *const c_char,
    aCallback: TEXTURES_RECEIVECALLBACK,
);
pub type TEXTURES_LOADFROMRESOURCE = unsafe extern "C" fn(
    aIdentifier: *const c_char,
    aResourceID: c_uint,
    aModule: HMODULE,
    aCallback: TEXTURES_RECEIVECALLBACK,
);
pub type TEXTURES_LOADFROMURL = ::std::option::Option<
    unsafe extern "C" fn(
        aIdentifier: *const c_char,
        aRemote: *const c_char,
        aEndpoint: *const c_char,
        aCallback: TEXTURES_RECEIVECALLBACK,
    ),
>;

pub type QUICKACCESS_ADDSHORTCUT = unsafe extern "C" fn(
    aIdentifier: *const c_char,
    aTextureIdentifier: *const c_char,
    aTextureHoverIdentifier: *const c_char,
    aKeybindIdentifier: *const c_char,
    aTooltipText: *const c_char,
);
pub type QUICKACCESS_ADDSIMPLE =
    unsafe extern "C" fn(aIdentifier: *const c_char, aShortcutRenderCallback: GUI_RENDER);
pub type QUICKACCESS_REMOVE = unsafe extern "C" fn(aIdentifier: *const c_char);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NexusLinkData {
    pub Width: c_uint,
    pub Height: c_uint,
    pub Scaling: f32,
    pub IsMoving: bool,
    pub IsCameraMoving: bool,
    pub IsGameplay: bool,
    // ImFont*
    pub Font: *mut c_void,
    pub FontBig: *mut c_void,
    pub FontUI: *mut c_void,
}

// Revision 1
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AddonAPI {
    /// Renderer
    // IDXGISwapChain*
    pub swap_chain: *mut c_void,
    // ImGuiContext*
    pub imgui_context: *mut c_void,
    pub imgui_malloc: *mut c_void,
    pub imgui_free: *mut c_void,
    pub register_render: GuiAddrender,
    pub unregister_render: GuiRemrender,

    /// Paths
    pub get_game_directory: PathsGetgamedir,
    pub get_addon_directory: PathsGetaddondir,
    pub get_common_directory: PathsGetcommondir,
    /// Minhook
    pub CreateHook: MINHOOK_CREATE,
    pub RemoveHook: MINHOOK_REMOVE,
    pub EnableHook: MINHOOK_ENABLE,
    pub DisableHook: MINHOOK_DISABLE,
    /// Logging
    pub Log: LOGGER_LOGA,
    /// Events
    pub RaiseEvent: EVENTS_RAISE,
    pub SubscribeEvent: EVENTS_SUBSCRIBE,
    pub UnsubscribeEvent: EVENTS_SUBSCRIBE,
    /// WndProc
    pub RegisterWndProc: WNDPROC_ADDREM,
    pub UnregisterWndProc: WNDPROC_ADDREM,
    /// Keybinds
    pub RegisterKeybindWithString: KEYBINDS_REGISTERWITHSTRING,
    pub RegisterKeybindWithStruct: KEYBINDS_REGISTERWITHSTRUCT,
    pub UnregisterKeybind: KEYBINDS_UNREGISTER,
    /// DataLink
    pub GetResource: DATALINK_GETRESOURCE,
    pub ShareResource: DATALINK_SHARERESOURCE,
    /// Textures
    pub GetTexture: TEXTURES_GET,
    pub LoadTextureFromFile: TEXTURES_LOADFROMFILE,
    pub LoadTextureFromResource: TEXTURES_LOADFROMRESOURCE,
    pub LoadTextureFromURL: TEXTURES_LOADFROMURL,
    /// Shortcuts
    pub AddShortcut: QUICKACCESS_ADDSHORTCUT,
    pub RemoveShortcut: QUICKACCESS_REMOVE,
    pub AddSimpleShortcut: QUICKACCESS_ADDSIMPLE,
    pub RemoveSimpleShortcut: QUICKACCESS_REMOVE,
}

pub type ADDON_LOAD = unsafe extern "C" fn(aAPI: *mut AddonAPI);
pub type ADDON_UNLOAD = unsafe extern "C" fn();

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AddonVersion {
    pub Major: c_short,
    pub Minor: c_short,
    pub Build: c_short,
    pub Revision: c_short,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EAddonFlags {
    None = 0,
    /// is hooking functions or doing anything else that's volatile and game build dependant
    IsVolatile = 1,
    /// prevents unloading at runtime, aka. will require a restart if updated, etc.
    DisableHotloading = 2,
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
    pub Signature: c_int,
    /// Determines which AddonAPI struct revision the Loader will pass, use the NEXUS_API_VERSION define from Nexus.h
    pub APIVersion: c_int,
    /// Name of the addon as shown in the library
    pub Name: *const c_char,
    pub Version: AddonVersion,
    /// Author of the addon
    pub Author: *const c_char,
    /// Short description
    pub Description: *const c_char,
    /// Pointer to Load Function of the addon
    pub Load: ADDON_LOAD,
    /// Pointer to Unload Function of the addon. Not required if EAddonFlags::DisableHotloading is set.
    pub Unload: ADDON_UNLOAD,
    /// Information about the addon
    pub Flags: EAddonFlags,
    /// What platform is the the addon hosted on
    pub Provider: EUpdateProvider,
    /// Link to the update resource
    pub UpdateLink: *const c_char,
}
