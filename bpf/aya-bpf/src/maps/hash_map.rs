use core::{marker::PhantomData, mem};

use aya_bpf_cty::c_void;

use crate::{
    bindings::{bpf_map_def, bpf_map_type::BPF_MAP_TYPE_HASH},
    helpers::bpf_map_lookup_elem,
};

#[repr(transparent)]
pub struct HashMap<K, V> {
    def: bpf_map_def,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

impl<K, V> HashMap<K, V> {
    pub const fn with_max_entries(max_entries: u32, flags: u32) -> HashMap<K, V> {
        HashMap {
            def: bpf_map_def {
                type_: BPF_MAP_TYPE_HASH,
                key_size: mem::size_of::<K>() as u32,
                value_size: mem::size_of::<V>() as u32,
                max_entries,
                map_flags: flags,
            },
            _k: PhantomData,
            _v: PhantomData,
        }
    }

    pub unsafe fn get(&mut self, key: &K) -> Option<&V> {
        let value = bpf_map_lookup_elem(
            &mut self.def as *mut _ as *mut _,
            key as *const _ as *const c_void,
        );
        if value.is_null() {
            None
        } else {
            // FIXME: alignment
            Some(&*(value as *const V))
        }
    }
}