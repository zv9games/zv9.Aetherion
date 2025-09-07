#![doc = "Sidecar module for class [`Engine`][crate::classes::Engine].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Engine` enums](https://docs.godotengine.org/en/stable/classes/class_engine.html#enumerations).\n\n"]
use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, AsObjectArg, ClassName, CowArg, InParamTuple, ObjectArg, ObjectCow, OutParamTuple, ParamTuple, RefArg, Signature
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::classes::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Engine.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `Engine`](https://docs.godotengine.org/en/stable/classes/class_engine.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Engine::singleton()`][Engine::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Engine {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Engine {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"Engine");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn set_physics_ticks_per_second(&mut self, physics_ticks_per_second: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (physics_ticks_per_second,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2976usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_physics_ticks_per_second", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_ticks_per_second(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2977usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_physics_ticks_per_second", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_physics_steps_per_frame(&mut self, max_physics_steps: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_physics_steps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2978usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_max_physics_steps_per_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_physics_steps_per_frame(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2979usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_max_physics_steps_per_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_jitter_fix(&mut self, physics_jitter_fix: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (physics_jitter_fix,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2980usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_physics_jitter_fix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_jitter_fix(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2981usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_physics_jitter_fix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_interpolation_fraction(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2982usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_physics_interpolation_fraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_fps(&mut self, max_fps: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_fps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2983usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_max_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_fps(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_max_fps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_time_scale(&mut self, time_scale: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (time_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2985usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_time_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_time_scale(&mut self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2986usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_time_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frames_drawn(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_frames_drawn", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frames_per_second(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_frames_per_second", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_frames(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_physics_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_frames(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_process_frames", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_main_loop(&self,) -> Option < Gd < crate::classes::MainLoop > > {
            type CallRet = Option < Gd < crate::classes::MainLoop > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_main_loop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_version_info(&self,) -> Dictionary {
            type CallRet = Dictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_version_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_author_info(&self,) -> Dictionary {
            type CallRet = Dictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_author_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_copyright_info(&self,) -> Array < Dictionary > {
            type CallRet = Array < Dictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_copyright_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_donor_info(&self,) -> Dictionary {
            type CallRet = Dictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_donor_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_license_info(&self,) -> Dictionary {
            type CallRet = Dictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_license_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_license_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_license_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_architecture_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_architecture_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_in_physics_frame(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_in_physics_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_singleton(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "has_singleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_singleton(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Object > > {
            type CallRet = Option < Gd < crate::classes::Object > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_singleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_singleton(&mut self, name: impl AsArg < StringName >, instance: impl AsObjectArg < crate::classes::Object >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, ObjectArg < crate::classes::Object >,);
            let args = (name.into_arg(), instance.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "register_singleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unregister_singleton(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3003usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "unregister_singleton", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_singleton_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3004usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_singleton_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_script_language(&mut self, language: impl AsObjectArg < crate::classes::ScriptLanguage >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (ObjectArg < crate::classes::ScriptLanguage >,);
            let args = (language.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3005usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "register_script_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unregister_script_language(&mut self, language: impl AsObjectArg < crate::classes::ScriptLanguage >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (ObjectArg < crate::classes::ScriptLanguage >,);
            let args = (language.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3006usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "unregister_script_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_language_count(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3007usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_script_language_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_language(&self, index: i32,) -> Option < Gd < crate::classes::ScriptLanguage > > {
            type CallRet = Option < Gd < crate::classes::ScriptLanguage > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_script_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editor_hint(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3009usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_editor_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_embedded_in_editor(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3010usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_embedded_in_editor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_write_movie_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3011usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_write_movie_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_print_to_stdout(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3012usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_print_to_stdout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_printing_to_stdout(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3013usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_printing_to_stdout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_print_error_messages(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3014usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_print_error_messages", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_printing_error_messages(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3015usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_printing_error_messages", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        #[doc(hidden)]
        pub fn __object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
    }
    impl crate::obj::GodotClass for Engine {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Engine"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Engine {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Engine {
        
    }
    impl std::ops::Deref for Engine {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Engine {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Engine__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Engine` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Engine;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Engine {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}