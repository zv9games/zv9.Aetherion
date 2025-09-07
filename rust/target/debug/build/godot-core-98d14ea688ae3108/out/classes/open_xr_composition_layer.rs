#![doc = "Sidecar module for class [`OpenXrCompositionLayer`][crate::classes::OpenXrCompositionLayer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRCompositionLayer` enums](https://docs.godotengine.org/en/stable/classes/class_openxrcompositionlayer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRCompositionLayer.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `OpenXRCompositionLayer`](https://docs.godotengine.org/en/stable/classes/class_openxrcompositionlayer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<OpenXrCompositionLayer>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrCompositionLayer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl OpenXrCompositionLayer {
        pub fn set_layer_viewport(&mut self, viewport: impl AsObjectArg < crate::classes::SubViewport >,) {
            type CallRet = ();
            type CallParams = (ObjectArg < crate::classes::SubViewport >,);
            let args = (viewport.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6062usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_layer_viewport", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layer_viewport(&self,) -> Option < Gd < crate::classes::SubViewport > > {
            type CallRet = Option < Gd < crate::classes::SubViewport > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6063usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_layer_viewport", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_android_surface(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6064usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_use_android_surface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_android_surface(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6065usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_use_android_surface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_android_surface_size(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6066usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_android_surface_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_android_surface_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6067usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_android_surface_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_hole_punch(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6068usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_enable_hole_punch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_enable_hole_punch(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_enable_hole_punch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sort_order(&mut self, order: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_sort_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sort_order(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_sort_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_blend(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_alpha_blend", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_blend(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_alpha_blend", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_android_surface(&mut self,) -> Option < Gd < crate::classes::JavaObject > > {
            type CallRet = Option < Gd < crate::classes::JavaObject > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_android_surface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_natively_supported(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "is_natively_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn intersects_ray(&self, origin: Vector3, direction: Vector3,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector3, Vector3,);
            let args = (origin, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6076usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "intersects_ray", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OpenXrCompositionLayer {
        type Base = crate::classes::Node3D;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"OpenXRCompositionLayer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrCompositionLayer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for OpenXrCompositionLayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for OpenXrCompositionLayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrCompositionLayer {
        
    }
    impl std::ops::Deref for OpenXrCompositionLayer {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrCompositionLayer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrCompositionLayer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `OpenXrCompositionLayer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrCompositionLayer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for OpenXrCompositionLayer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}