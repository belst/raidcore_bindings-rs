#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod raw_structs {
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
    pub const IMGUI_VERSION_NUM: u32 = 18000;
    pub const NEXUS_API_VERSION: i32 = 1;
    #[repr(C)]
    pub enum ERenderType {
        PreRender = 0,
        Render = 1,
        PostRender = 2,
        OptionsRender = 3,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ERenderType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ERenderType::PreRender => "PreRender",
                    ERenderType::Render => "Render",
                    ERenderType::PostRender => "PostRender",
                    ERenderType::OptionsRender => "OptionsRender",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ERenderType {
        #[inline]
        fn clone(&self) -> ERenderType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ERenderType {}
    pub type GuiRender = unsafe extern "C" fn();
    pub type GuiAddrender = unsafe extern "C" fn(
        aRenderType: ERenderType,
        aRenderCallback: GuiRender,
    );
    pub type GuiRemrender = unsafe extern "C" fn(aRenderCallback: GuiRender);
    pub type PathsGetgamedir = unsafe extern "C" fn() -> *const c_char;
    pub type PathsGetaddondir = unsafe extern "C" fn(
        aName: *const c_char,
    ) -> *const c_char;
    pub type PathsGetcommondir = unsafe extern "C" fn() -> *const c_char;
    #[repr(C)]
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
    #[automatically_derived]
    impl ::core::fmt::Debug for EMHStatus {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EMHStatus::MhUnknown => "MhUnknown",
                    EMHStatus::MhOk => "MhOk",
                    EMHStatus::MhErrorAlreadyInitialized => "MhErrorAlreadyInitialized",
                    EMHStatus::MhErrorNotInitialized => "MhErrorNotInitialized",
                    EMHStatus::MhErrorAlreadyCreated => "MhErrorAlreadyCreated",
                    EMHStatus::MhErrorNotCreated => "MhErrorNotCreated",
                    EMHStatus::MhErrorEnabled => "MhErrorEnabled",
                    EMHStatus::MhErrorDisabled => "MhErrorDisabled",
                    EMHStatus::MhErrorNotExecutable => "MhErrorNotExecutable",
                    EMHStatus::MhErrorUnsupportedFunction => "MhErrorUnsupportedFunction",
                    EMHStatus::MhErrorMemoryAlloc => "MhErrorMemoryAlloc",
                    EMHStatus::MhErrorMemoryProtect => "MhErrorMemoryProtect",
                    EMHStatus::MhErrorModuleNotFound => "MhErrorModuleNotFound",
                    EMHStatus::MhErrorFunctionNotFound => "MhErrorFunctionNotFound",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EMHStatus {
        #[inline]
        fn clone(&self) -> EMHStatus {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EMHStatus {}
    pub type MinhookCreate = unsafe extern "C" fn(
        pTarget: LPVOID,
        pDetour: LPVOID,
        ppOriginal: *mut LPVOID,
    ) -> EMHStatus;
    pub type MinhookRemove = unsafe extern "C" fn(pTarget: LPVOID) -> EMHStatus;
    pub type MinhookEnable = unsafe extern "C" fn(pTarget: LPVOID) -> EMHStatus;
    pub type MinhookDisable = unsafe extern "C" fn(pTarget: LPVOID) -> EMHStatus;
    #[repr(C)]
    pub enum ELogLevel {
        Off = 0,
        Critical = 1,
        Warning = 2,
        INFO = 3,
        DEBUG = 4,
        TRACE = 5,
        ALL,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ELogLevel {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ELogLevel::Off => "Off",
                    ELogLevel::Critical => "Critical",
                    ELogLevel::Warning => "Warning",
                    ELogLevel::INFO => "INFO",
                    ELogLevel::DEBUG => "DEBUG",
                    ELogLevel::TRACE => "TRACE",
                    ELogLevel::ALL => "ALL",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ELogLevel {
        #[inline]
        fn clone(&self) -> ELogLevel {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ELogLevel {}
    pub type LoggerLoga = unsafe extern "C" fn(
        aLogLevel: ELogLevel,
        aStr: *const c_char,
    );
    pub type EventConsume = unsafe extern "C" fn(aEventArgs: *mut c_void);
    pub type EventsRaise = unsafe extern "C" fn(
        aIdentifier: *const c_char,
        aEventData: *mut c_void,
    );
    pub type EventsSubscribe = unsafe extern "C" fn(
        aIdentifier: *const c_char,
        aConsumeEventCallback: EventConsume,
    );
    pub type WndprocCallback = unsafe extern "C" fn(
        hWnd: HWND,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> UINT;
    pub type WndprocAddrem = unsafe extern "C" fn(aWndProcCallback: WndprocCallback);
    #[repr(C)]
    pub struct Keybind {
        pub key: c_ushort,
        pub alt: bool,
        pub ctrl: bool,
        pub shift: bool,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Keybind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Keybind",
                "key",
                &self.key,
                "alt",
                &self.alt,
                "ctrl",
                &self.ctrl,
                "shift",
                &&self.shift,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Keybind {}
    #[automatically_derived]
    impl ::core::clone::Clone for Keybind {
        #[inline]
        fn clone(&self) -> Keybind {
            let _: ::core::clone::AssertParamIsClone<c_ushort>;
            let _: ::core::clone::AssertParamIsClone<bool>;
            *self
        }
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
    pub type DatalinkGetresource = unsafe extern "C" fn(
        aIdentifier: *const c_char,
    ) -> *mut c_void;
    pub type DatalinkShareresource = unsafe extern "C" fn(
        aIdentifier: *const c_char,
        aResourceSize: usize,
    ) -> *mut c_void;
    #[repr(C)]
    pub struct Texture {
        pub width: c_uint,
        pub height: c_uint,
        pub resource: *mut ID3D11ShaderResourceView,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Texture {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Texture",
                "width",
                &self.width,
                "height",
                &self.height,
                "resource",
                &&self.resource,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Texture {}
    #[automatically_derived]
    impl ::core::clone::Clone for Texture {
        #[inline]
        fn clone(&self) -> Texture {
            let _: ::core::clone::AssertParamIsClone<c_uint>;
            let _: ::core::clone::AssertParamIsClone<*mut ID3D11ShaderResourceView>;
            *self
        }
    }
    pub type TexturesReceivecallback = unsafe extern "C" fn(
        aIdentifier: *const c_char,
        aTexture: *mut Texture,
    );
    pub type TexturesGet = unsafe extern "C" fn(
        aIdentifier: *const c_char,
    ) -> *mut Texture;
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
    pub type QuickaccessAddsimple = unsafe extern "C" fn(
        aIdentifier: *const c_char,
        aShortcutRenderCallback: GuiRender,
    );
    pub type QuickaccessRemove = unsafe extern "C" fn(aIdentifier: *const c_char);
    #[repr(C)]
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
    #[automatically_derived]
    impl ::core::fmt::Debug for NexusLinkData {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "width",
                "height",
                "scaling",
                "is_moving",
                "is_camera_moving",
                "is_gameplay",
                "font",
                "font_big",
                "font_ui",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.width,
                &self.height,
                &self.scaling,
                &self.is_moving,
                &self.is_camera_moving,
                &self.is_gameplay,
                &self.font,
                &self.font_big,
                &&self.font_ui,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "NexusLinkData",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for NexusLinkData {}
    #[automatically_derived]
    impl ::core::clone::Clone for NexusLinkData {
        #[inline]
        fn clone(&self) -> NexusLinkData {
            let _: ::core::clone::AssertParamIsClone<c_uint>;
            let _: ::core::clone::AssertParamIsClone<f32>;
            let _: ::core::clone::AssertParamIsClone<bool>;
            let _: ::core::clone::AssertParamIsClone<*mut ImFont>;
            let _: ::core::clone::AssertParamIsClone<*mut ImFont>;
            let _: ::core::clone::AssertParamIsClone<*mut ImFont>;
            *self
        }
    }
    #[repr(C)]
    pub struct AddonAPI {
        /// Renderer
        pub swap_chain: *mut c_void,
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
    #[automatically_derived]
    impl ::core::fmt::Debug for AddonAPI {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "swap_chain",
                "imgui_context",
                "imgui_malloc",
                "imgui_free",
                "register_render",
                "unregister_render",
                "get_game_directory",
                "get_addon_directory",
                "get_common_directory",
                "create_hook",
                "remove_hook",
                "enable_hook",
                "disable_hook",
                "log",
                "raise_event",
                "subscribe_event",
                "unsubscribe_event",
                "register_wnd_proc",
                "unregister_wnd_proc",
                "register_keybind_with_string",
                "register_keybind_with_struct",
                "unregister_keybind",
                "get_resource",
                "share_resource",
                "get_texture",
                "load_texture_from_file",
                "load_texture_from_resource",
                "load_texture_from_url",
                "add_shortcut",
                "remove_shortcut",
                "add_simple_shortcut",
                "remove_simple_shortcut",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.swap_chain,
                &self.imgui_context,
                &self.imgui_malloc,
                &self.imgui_free,
                &self.register_render,
                &self.unregister_render,
                &self.get_game_directory,
                &self.get_addon_directory,
                &self.get_common_directory,
                &self.create_hook,
                &self.remove_hook,
                &self.enable_hook,
                &self.disable_hook,
                &self.log,
                &self.raise_event,
                &self.subscribe_event,
                &self.unsubscribe_event,
                &self.register_wnd_proc,
                &self.unregister_wnd_proc,
                &self.register_keybind_with_string,
                &self.register_keybind_with_struct,
                &self.unregister_keybind,
                &self.get_resource,
                &self.share_resource,
                &self.get_texture,
                &self.load_texture_from_file,
                &self.load_texture_from_resource,
                &self.load_texture_from_url,
                &self.add_shortcut,
                &self.remove_shortcut,
                &self.add_simple_shortcut,
                &&self.remove_simple_shortcut,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "AddonAPI",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AddonAPI {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddonAPI {
        #[inline]
        fn clone(&self) -> AddonAPI {
            let _: ::core::clone::AssertParamIsClone<*mut c_void>;
            let _: ::core::clone::AssertParamIsClone<*mut ImGuiContext>;
            let _: ::core::clone::AssertParamIsClone<
                unsafe extern "C" fn(usize, *mut c_void) -> *mut c_void,
            >;
            let _: ::core::clone::AssertParamIsClone<
                unsafe extern "C" fn(*mut c_void, *mut c_void),
            >;
            let _: ::core::clone::AssertParamIsClone<GuiAddrender>;
            let _: ::core::clone::AssertParamIsClone<GuiRemrender>;
            let _: ::core::clone::AssertParamIsClone<PathsGetgamedir>;
            let _: ::core::clone::AssertParamIsClone<PathsGetaddondir>;
            let _: ::core::clone::AssertParamIsClone<PathsGetcommondir>;
            let _: ::core::clone::AssertParamIsClone<MinhookCreate>;
            let _: ::core::clone::AssertParamIsClone<MinhookRemove>;
            let _: ::core::clone::AssertParamIsClone<MinhookEnable>;
            let _: ::core::clone::AssertParamIsClone<MinhookDisable>;
            let _: ::core::clone::AssertParamIsClone<LoggerLoga>;
            let _: ::core::clone::AssertParamIsClone<EventsRaise>;
            let _: ::core::clone::AssertParamIsClone<EventsSubscribe>;
            let _: ::core::clone::AssertParamIsClone<WndprocAddrem>;
            let _: ::core::clone::AssertParamIsClone<KeybindsRegisterwithstring>;
            let _: ::core::clone::AssertParamIsClone<KeybindsRegisterwithstruct>;
            let _: ::core::clone::AssertParamIsClone<KeybindsUnregister>;
            let _: ::core::clone::AssertParamIsClone<DatalinkGetresource>;
            let _: ::core::clone::AssertParamIsClone<DatalinkShareresource>;
            let _: ::core::clone::AssertParamIsClone<TexturesGet>;
            let _: ::core::clone::AssertParamIsClone<TexturesLoadfromfile>;
            let _: ::core::clone::AssertParamIsClone<TexturesLoadfromresource>;
            let _: ::core::clone::AssertParamIsClone<TexturesLoadfromurl>;
            let _: ::core::clone::AssertParamIsClone<QuickaccessAddshortcut>;
            let _: ::core::clone::AssertParamIsClone<QuickaccessRemove>;
            let _: ::core::clone::AssertParamIsClone<QuickaccessAddsimple>;
            *self
        }
    }
    pub type AddonLoad = unsafe extern "C" fn(aAPI: *mut AddonAPI);
    pub type AddonUnload = unsafe extern "C" fn();
    #[repr(C)]
    pub struct AddonVersion {
        pub major: c_short,
        pub minor: c_short,
        pub build: c_short,
        pub revision: c_short,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddonVersion {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "AddonVersion",
                "major",
                &self.major,
                "minor",
                &self.minor,
                "build",
                &self.build,
                "revision",
                &&self.revision,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AddonVersion {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddonVersion {
        #[inline]
        fn clone(&self) -> AddonVersion {
            let _: ::core::clone::AssertParamIsClone<c_short>;
            *self
        }
    }
    pub struct EAddonFlags(
        <EAddonFlags as ::bitflags::__private::PublicFlags>::Internal,
    );
    #[automatically_derived]
    impl ::core::fmt::Debug for EAddonFlags {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "EAddonFlags", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EAddonFlags {}
    #[automatically_derived]
    impl ::core::clone::Clone for EAddonFlags {
        #[inline]
        fn clone(&self) -> EAddonFlags {
            let _: ::core::clone::AssertParamIsClone<
                <EAddonFlags as ::bitflags::__private::PublicFlags>::Internal,
            >;
            *self
        }
    }
    impl EAddonFlags {
        #[allow(deprecated, non_upper_case_globals)]
        pub const None: Self = Self::from_bits_retain(0);
        /// is hooking functions or doing anything else that's volatile and game build dependant
        #[allow(deprecated, non_upper_case_globals)]
        pub const IsVolatile: Self = Self::from_bits_retain(1);
        /// prevents unloading at runtime, aka. will require a restart if updated, etc.
        #[allow(deprecated, non_upper_case_globals)]
        pub const DisableHotloading: Self = Self::from_bits_retain(2);
    }
    impl ::bitflags::Flags for EAddonFlags {
        const FLAGS: &'static [::bitflags::Flag<EAddonFlags>] = &[
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("None", EAddonFlags::None)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("IsVolatile", EAddonFlags::IsVolatile)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new(
                    "DisableHotloading",
                    EAddonFlags::DisableHotloading,
                )
            },
        ];
        type Bits = c_int;
        fn bits(&self) -> c_int {
            EAddonFlags::bits(self)
        }
        fn from_bits_retain(bits: c_int) -> EAddonFlags {
            EAddonFlags::from_bits_retain(bits)
        }
    }
    #[allow(
        dead_code,
        deprecated,
        unused_doc_comments,
        unused_attributes,
        unused_mut,
        unused_imports,
        non_upper_case_globals,
        clippy::assign_op_pattern,
        clippy::indexing_slicing,
        clippy::same_name_method,
        clippy::iter_without_into_iter,
    )]
    const _: () = {
        #[repr(transparent)]
        pub struct InternalBitFlags(c_int);
        #[automatically_derived]
        impl ::core::clone::Clone for InternalBitFlags {
            #[inline]
            fn clone(&self) -> InternalBitFlags {
                let _: ::core::clone::AssertParamIsClone<c_int>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for InternalBitFlags {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for InternalBitFlags {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for InternalBitFlags {
            #[inline]
            fn eq(&self, other: &InternalBitFlags) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for InternalBitFlags {}
        #[automatically_derived]
        impl ::core::cmp::Eq for InternalBitFlags {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<c_int>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for InternalBitFlags {
            #[inline]
            fn partial_cmp(
                &self,
                other: &InternalBitFlags,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for InternalBitFlags {
            #[inline]
            fn cmp(&self, other: &InternalBitFlags) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for InternalBitFlags {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        impl ::bitflags::__private::PublicFlags for EAddonFlags {
            type Primitive = c_int;
            type Internal = InternalBitFlags;
        }
        impl ::bitflags::__private::core::default::Default for InternalBitFlags {
            #[inline]
            fn default() -> Self {
                InternalBitFlags::empty()
            }
        }
        impl ::bitflags::__private::core::fmt::Debug for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
            ) -> ::bitflags::__private::core::fmt::Result {
                if self.is_empty() {
                    f.write_fmt(
                        format_args!("{0:#x}", <c_int as ::bitflags::Bits>::EMPTY),
                    )
                } else {
                    ::bitflags::__private::core::fmt::Display::fmt(self, f)
                }
            }
        }
        impl ::bitflags::__private::core::fmt::Display for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::parser::to_writer(&EAddonFlags(*self), f)
            }
        }
        impl ::bitflags::__private::core::str::FromStr for InternalBitFlags {
            type Err = ::bitflags::parser::ParseError;
            fn from_str(
                s: &str,
            ) -> ::bitflags::__private::core::result::Result<Self, Self::Err> {
                ::bitflags::parser::from_str::<EAddonFlags>(s).map(|flags| flags.0)
            }
        }
        impl ::bitflags::__private::core::convert::AsRef<c_int> for InternalBitFlags {
            fn as_ref(&self) -> &c_int {
                &self.0
            }
        }
        impl ::bitflags::__private::core::convert::From<c_int> for InternalBitFlags {
            fn from(bits: c_int) -> Self {
                Self::from_bits_retain(bits)
            }
        }
        #[allow(dead_code, deprecated, unused_attributes)]
        impl InternalBitFlags {
            /// Get a flags value with all bits unset.
            #[inline]
            pub const fn empty() -> Self {
                { Self(<c_int as ::bitflags::Bits>::EMPTY) }
            }
            /// Get a flags value with all known bits set.
            #[inline]
            pub const fn all() -> Self {
                {
                    let mut truncated = <c_int as ::bitflags::Bits>::EMPTY;
                    let mut i = 0;
                    {
                        {
                            let flag = <EAddonFlags as ::bitflags::Flags>::FLAGS[i]
                                .value()
                                .bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    {
                        {
                            let flag = <EAddonFlags as ::bitflags::Flags>::FLAGS[i]
                                .value()
                                .bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    {
                        {
                            let flag = <EAddonFlags as ::bitflags::Flags>::FLAGS[i]
                                .value()
                                .bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    let _ = i;
                    Self::from_bits_retain(truncated)
                }
            }
            /// Get the underlying bits value.
            ///
            /// The returned value is exactly the bits set in this flags value.
            #[inline]
            pub const fn bits(&self) -> c_int {
                let f = self;
                { f.0 }
            }
            /// Convert from a bits value.
            ///
            /// This method will return `None` if any unknown bits are set.
            #[inline]
            pub const fn from_bits(
                bits: c_int,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                let bits = bits;
                {
                    let truncated = Self::from_bits_truncate(bits).0;
                    if truncated == bits {
                        ::bitflags::__private::core::option::Option::Some(Self(bits))
                    } else {
                        ::bitflags::__private::core::option::Option::None
                    }
                }
            }
            /// Convert from a bits value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: c_int) -> Self {
                let bits = bits;
                { Self(bits & Self::all().bits()) }
            }
            /// Convert from a bits value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: c_int) -> Self {
                let bits = bits;
                { Self(bits) }
            }
            /// Get a flags value with the bits of a flag with the given name set.
            ///
            /// This method will return `None` if `name` is empty or doesn't
            /// correspond to any named flag.
            #[inline]
            pub fn from_name(
                name: &str,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                let name = name;
                {
                    {
                        if name == "None" {
                            return ::bitflags::__private::core::option::Option::Some(
                                Self(EAddonFlags::None.bits()),
                            );
                        }
                    };
                    {
                        if name == "IsVolatile" {
                            return ::bitflags::__private::core::option::Option::Some(
                                Self(EAddonFlags::IsVolatile.bits()),
                            );
                        }
                    };
                    {
                        if name == "DisableHotloading" {
                            return ::bitflags::__private::core::option::Option::Some(
                                Self(EAddonFlags::DisableHotloading.bits()),
                            );
                        }
                    };
                    let _ = name;
                    ::bitflags::__private::core::option::Option::None
                }
            }
            /// Whether all bits in this flags value are unset.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                let f = self;
                { f.bits() == <c_int as ::bitflags::Bits>::EMPTY }
            }
            /// Whether all known bits in this flags value are set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                let f = self;
                { Self::all().bits() | f.bits() == f.bits() }
            }
            /// Whether any set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                { f.bits() & other.bits() != <c_int as ::bitflags::Bits>::EMPTY }
            }
            /// Whether all set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                { f.bits() & other.bits() == other.bits() }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    *f = Self::from_bits_retain(f.bits()).union(other);
                }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `remove` won't truncate `other`, but the `!` operator will.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    *f = Self::from_bits_retain(f.bits()).difference(other);
                }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    *f = Self::from_bits_retain(f.bits()).symmetric_difference(other);
                }
            }
            /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                let f = self;
                let other = other;
                let value = value;
                {
                    if value {
                        f.insert(other);
                    } else {
                        f.remove(other);
                    }
                }
            }
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self::from_bits_retain(f.bits() & other.bits()) }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self::from_bits_retain(f.bits() | other.bits()) }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self::from_bits_retain(f.bits() & !other.bits()) }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self::from_bits_retain(f.bits() ^ other.bits()) }
            }
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                let f = self;
                { Self::from_bits_truncate(!f.bits()) }
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::__private::core::fmt::Binary::fmt(&self.0, f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::__private::core::fmt::Octal::fmt(&self.0, f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::__private::core::fmt::LowerHex::fmt(&self.0, f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::__private::core::fmt::UpperHex::fmt(&self.0, f)
            }
        }
        impl ::bitflags::__private::core::ops::BitOr for InternalBitFlags {
            type Output = Self;
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor(self, other: InternalBitFlags) -> Self {
                self.union(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitOrAssign for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.insert(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitXor for InternalBitFlags {
            type Output = Self;
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitXorAssign for InternalBitFlags {
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.toggle(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitAnd for InternalBitFlags {
            type Output = Self;
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitAndAssign for InternalBitFlags {
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = Self::from_bits_retain(self.bits()).intersection(other);
            }
        }
        impl ::bitflags::__private::core::ops::Sub for InternalBitFlags {
            type Output = Self;
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::SubAssign for InternalBitFlags {
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.remove(other);
            }
        }
        impl ::bitflags::__private::core::ops::Not for InternalBitFlags {
            type Output = Self;
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }
        impl ::bitflags::__private::core::iter::Extend<InternalBitFlags>
        for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::__private::core::iter::FromIterator<InternalBitFlags>
        for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn from_iter<
                T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
            >(iterator: T) -> Self {
                use ::bitflags::__private::core::iter::Extend;
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl InternalBitFlags {
            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> ::bitflags::iter::Iter<EAddonFlags> {
                ::bitflags::iter::Iter::__private_const_new(
                    <EAddonFlags as ::bitflags::Flags>::FLAGS,
                    EAddonFlags::from_bits_retain(self.bits()),
                    EAddonFlags::from_bits_retain(self.bits()),
                )
            }
            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<EAddonFlags> {
                ::bitflags::iter::IterNames::__private_const_new(
                    <EAddonFlags as ::bitflags::Flags>::FLAGS,
                    EAddonFlags::from_bits_retain(self.bits()),
                    EAddonFlags::from_bits_retain(self.bits()),
                )
            }
        }
        impl ::bitflags::__private::core::iter::IntoIterator for InternalBitFlags {
            type Item = EAddonFlags;
            type IntoIter = ::bitflags::iter::Iter<EAddonFlags>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
        impl InternalBitFlags {
            /// Returns a mutable reference to the raw value of the flags currently stored.
            #[inline]
            pub fn bits_mut(&mut self) -> &mut c_int {
                &mut self.0
            }
        }
        #[allow(dead_code, deprecated, unused_attributes)]
        impl EAddonFlags {
            /// Get a flags value with all bits unset.
            #[inline]
            pub const fn empty() -> Self {
                { Self(InternalBitFlags::empty()) }
            }
            /// Get a flags value with all known bits set.
            #[inline]
            pub const fn all() -> Self {
                { Self(InternalBitFlags::all()) }
            }
            /// Get the underlying bits value.
            ///
            /// The returned value is exactly the bits set in this flags value.
            #[inline]
            pub const fn bits(&self) -> c_int {
                let f = self;
                { f.0.bits() }
            }
            /// Convert from a bits value.
            ///
            /// This method will return `None` if any unknown bits are set.
            #[inline]
            pub const fn from_bits(
                bits: c_int,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                let bits = bits;
                {
                    match InternalBitFlags::from_bits(bits) {
                        ::bitflags::__private::core::option::Option::Some(bits) => {
                            ::bitflags::__private::core::option::Option::Some(Self(bits))
                        }
                        ::bitflags::__private::core::option::Option::None => {
                            ::bitflags::__private::core::option::Option::None
                        }
                    }
                }
            }
            /// Convert from a bits value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: c_int) -> Self {
                let bits = bits;
                { Self(InternalBitFlags::from_bits_truncate(bits)) }
            }
            /// Convert from a bits value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: c_int) -> Self {
                let bits = bits;
                { Self(InternalBitFlags::from_bits_retain(bits)) }
            }
            /// Get a flags value with the bits of a flag with the given name set.
            ///
            /// This method will return `None` if `name` is empty or doesn't
            /// correspond to any named flag.
            #[inline]
            pub fn from_name(
                name: &str,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                let name = name;
                {
                    match InternalBitFlags::from_name(name) {
                        ::bitflags::__private::core::option::Option::Some(bits) => {
                            ::bitflags::__private::core::option::Option::Some(Self(bits))
                        }
                        ::bitflags::__private::core::option::Option::None => {
                            ::bitflags::__private::core::option::Option::None
                        }
                    }
                }
            }
            /// Whether all bits in this flags value are unset.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                let f = self;
                { f.0.is_empty() }
            }
            /// Whether all known bits in this flags value are set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                let f = self;
                { f.0.is_all() }
            }
            /// Whether any set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                { f.0.intersects(other.0) }
            }
            /// Whether all set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                { f.0.contains(other.0) }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                let f = self;
                let other = other;
                { f.0.insert(other.0) }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `remove` won't truncate `other`, but the `!` operator will.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                let f = self;
                let other = other;
                { f.0.remove(other.0) }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                let f = self;
                let other = other;
                { f.0.toggle(other.0) }
            }
            /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                let f = self;
                let other = other;
                let value = value;
                { f.0.set(other.0, value) }
            }
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self(f.0.intersection(other.0)) }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self(f.0.union(other.0)) }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self(f.0.difference(other.0)) }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self(f.0.symmetric_difference(other.0)) }
            }
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                let f = self;
                { Self(f.0.complement()) }
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for EAddonFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::__private::core::fmt::Binary::fmt(&self.0, f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for EAddonFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::__private::core::fmt::Octal::fmt(&self.0, f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for EAddonFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::__private::core::fmt::LowerHex::fmt(&self.0, f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for EAddonFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::__private::core::fmt::UpperHex::fmt(&self.0, f)
            }
        }
        impl ::bitflags::__private::core::ops::BitOr for EAddonFlags {
            type Output = Self;
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor(self, other: EAddonFlags) -> Self {
                self.union(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitOrAssign for EAddonFlags {
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.insert(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitXor for EAddonFlags {
            type Output = Self;
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitXorAssign for EAddonFlags {
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.toggle(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitAnd for EAddonFlags {
            type Output = Self;
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitAndAssign for EAddonFlags {
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = Self::from_bits_retain(self.bits()).intersection(other);
            }
        }
        impl ::bitflags::__private::core::ops::Sub for EAddonFlags {
            type Output = Self;
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::SubAssign for EAddonFlags {
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.remove(other);
            }
        }
        impl ::bitflags::__private::core::ops::Not for EAddonFlags {
            type Output = Self;
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }
        impl ::bitflags::__private::core::iter::Extend<EAddonFlags> for EAddonFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::__private::core::iter::FromIterator<EAddonFlags>
        for EAddonFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn from_iter<
                T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
            >(iterator: T) -> Self {
                use ::bitflags::__private::core::iter::Extend;
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl EAddonFlags {
            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> ::bitflags::iter::Iter<EAddonFlags> {
                ::bitflags::iter::Iter::__private_const_new(
                    <EAddonFlags as ::bitflags::Flags>::FLAGS,
                    EAddonFlags::from_bits_retain(self.bits()),
                    EAddonFlags::from_bits_retain(self.bits()),
                )
            }
            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<EAddonFlags> {
                ::bitflags::iter::IterNames::__private_const_new(
                    <EAddonFlags as ::bitflags::Flags>::FLAGS,
                    EAddonFlags::from_bits_retain(self.bits()),
                    EAddonFlags::from_bits_retain(self.bits()),
                )
            }
        }
        impl ::bitflags::__private::core::iter::IntoIterator for EAddonFlags {
            type Item = EAddonFlags;
            type IntoIter = ::bitflags::iter::Iter<EAddonFlags>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
    };
    #[repr(C)]
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
    #[automatically_derived]
    impl ::core::fmt::Debug for EUpdateProvider {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EUpdateProvider::None => "None",
                    EUpdateProvider::Raidcore => "Raidcore",
                    EUpdateProvider::GitHub => "GitHub",
                    EUpdateProvider::Direct => "Direct",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EUpdateProvider {
        #[inline]
        fn clone(&self) -> EUpdateProvider {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EUpdateProvider {}
    #[repr(C)]
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
    #[automatically_derived]
    impl ::core::fmt::Debug for AddonDefinition {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "signature",
                "apiversion",
                "name",
                "version",
                "author",
                "description",
                "load",
                "unload",
                "flags",
                "provider",
                "update_link",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.signature,
                &self.apiversion,
                &self.name,
                &self.version,
                &self.author,
                &self.description,
                &self.load,
                &self.unload,
                &self.flags,
                &self.provider,
                &&self.update_link,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "AddonDefinition",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AddonDefinition {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddonDefinition {
        #[inline]
        fn clone(&self) -> AddonDefinition {
            let _: ::core::clone::AssertParamIsClone<c_int>;
            let _: ::core::clone::AssertParamIsClone<*const c_char>;
            let _: ::core::clone::AssertParamIsClone<AddonVersion>;
            let _: ::core::clone::AssertParamIsClone<*const c_char>;
            let _: ::core::clone::AssertParamIsClone<*const c_char>;
            let _: ::core::clone::AssertParamIsClone<AddonLoad>;
            let _: ::core::clone::AssertParamIsClone<Option<AddonUnload>>;
            let _: ::core::clone::AssertParamIsClone<EAddonFlags>;
            let _: ::core::clone::AssertParamIsClone<EUpdateProvider>;
            let _: ::core::clone::AssertParamIsClone<Option<*const c_char>>;
            *self
        }
    }
    unsafe impl Sync for AddonDefinition {}
}
use std::{iter::Map, mem::MaybeUninit};
use arcdps_imgui::Ui;
use raw_structs::EAddonFlags;
pub use raw_structs::ERenderType;
type RenderFn = dyn Fn(&Ui);
pub struct AddonApi<'a> {
    inner: raw_structs::AddonAPI,
    ui: &'a Ui<'a>,
    render_callbacks: Map<ERenderType, Vec<Box<RenderFn>>>,
}
