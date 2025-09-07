#![doc = "Sidecar module for class [`ResourceUid`][crate::classes::ResourceUid].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ResourceUID` enums](https://docs.godotengine.org/en/stable/classes/class_resourceuid.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ResourceUID.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `ResourceUID`](https://docs.godotengine.org/en/stable/classes/class_resourceuid.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`ResourceUid::singleton()`][ResourceUid::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ResourceUid {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ResourceUid {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"ResourceUID");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn id_to_text(&self, id: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7554usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceUid", "id_to_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn text_to_id(&self, text_id: impl AsArg < GString >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text_id.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7555usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceUid", "text_to_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_id(&mut self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7556usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceUid", "create_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_id(&self, id: i64,) -> bool {
            type CallRet = bool;
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7557usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceUid", "has_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_id(&mut self, id: i64, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
            let args = (id, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7558usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceUid", "add_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_id(&mut self, id: i64, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
            let args = (id, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7559usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceUid", "set_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_id_path(&self, id: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7560usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceUid", "get_id_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_id(&mut self, id: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7561usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceUid", "remove_id", self.object_ptr, self.__checked_id(), args,)
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
        pub const INVALID_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for ResourceUid {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ResourceUID"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ResourceUid {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ResourceUid {
        
    }
    impl std::ops::Deref for ResourceUid {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ResourceUid {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ResourceUid__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ResourceUid` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ResourceUid;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ResourceUid {
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