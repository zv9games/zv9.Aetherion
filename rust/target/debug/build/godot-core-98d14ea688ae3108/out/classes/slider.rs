#![doc = "Sidecar module for class [`Slider`][crate::classes::Slider].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Slider` enums](https://docs.godotengine.org/en/stable/classes/class_slider.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Slider.`\n\nInherits [`Range`][crate::classes::Range].\n\nRelated symbols:\n\n* [`slider`][crate::classes::slider]: sidecar module with related enum/flag types\n* [`SignalsOfSlider`][crate::classes::slider::SignalsOfSlider]: signal collection\n\n\nSee also [Godot docs for `Slider`](https://docs.godotengine.org/en/stable/classes/class_slider.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Slider>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Slider {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Slider {
        pub fn set_ticks(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "set_ticks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ticks(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "get_ticks", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ticks_on_borders(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "get_ticks_on_borders", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ticks_on_borders(&mut self, ticks_on_border: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (ticks_on_border,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "set_ticks_on_borders", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_editable(&mut self, editable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (editable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "set_editable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "is_editable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scrollable(&mut self, scrollable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (scrollable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "set_scrollable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_scrollable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "is_scrollable", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Slider {
        type Base = crate::classes::Range;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Slider"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Slider {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Range > for Slider {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for Slider {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Slider {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Slider {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Slider {
        
    }
    impl std::ops::Deref for Slider {
        type Target = crate::classes::Range;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Slider {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Slider__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Slider` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Slider;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Slider`][crate::classes::Slider] class."]
    pub struct SignalsOfSlider < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfSlider < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn drag_started(&mut self) -> SigDragStarted < 'c, C > {
            SigDragStarted {
                typed: TypedSignal::extract(&mut self.__internal_obj, "drag_started")
            }
        }
        #[doc = "Signature: `(value_changed: bool)`"]
        pub fn drag_ended(&mut self) -> SigDragEnded < 'c, C > {
            SigDragEnded {
                typed: TypedSignal::extract(&mut self.__internal_obj, "drag_ended")
            }
        }
    }
    type TypedSigDragStarted < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigDragStarted < 'c, C: WithSignals > {
        typed: TypedSigDragStarted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDragStarted < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDragStarted < 'c, C > {
        type Target = TypedSigDragStarted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDragStarted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigDragEnded < 'c, C > = TypedSignal < 'c, C, (bool,) >;
    pub struct SigDragEnded < 'c, C: WithSignals > {
        typed: TypedSigDragEnded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDragEnded < 'c, C > {
        pub fn emit(&mut self, value_changed: bool,) {
            self.typed.emit_tuple((value_changed,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDragEnded < 'c, C > {
        type Target = TypedSigDragEnded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDragEnded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Slider {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfSlider < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfSlider < 'c, C > {
        type Target = < < Slider as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Slider;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfSlider < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Slider;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}