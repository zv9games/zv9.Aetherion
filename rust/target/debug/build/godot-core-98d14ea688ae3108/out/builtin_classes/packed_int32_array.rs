use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, AsObjectArg, ClassName, CowArg, InParamTuple, ObjectArg, ObjectCow, OutParamTuple, ParamTuple, RefArg, Signature
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
#[repr(transparent)]
pub struct InnerPackedInt32Array < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedInt32Array < 'a > {
    pub fn from_outer(outer: &PackedInt32Array) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn get(&self, index: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(765usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "get", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(766usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "set", self.sys_ptr, args)
        }
    }
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(767usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(768usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(769usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(770usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &PackedInt32Array,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, PackedInt32Array >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(771usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(772usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(773usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(774usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(775usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(776usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(777usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(778usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedInt32Array {
        type CallRet = PackedInt32Array;
        type CallParams = (i64, i64,);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(779usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "slice", self.sys_ptr, args)
        }
    }
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(780usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "to_byte_array", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(781usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: i64, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, bool,);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(782usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedInt32Array {
        type CallRet = PackedInt32Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(783usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: i64, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(784usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: i64, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(785usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(786usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt32Array", "count", self.sys_ptr, args)
        }
    }
}
impl PackedInt32Array {
    
}