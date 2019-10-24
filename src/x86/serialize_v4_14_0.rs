// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::mem;
use std::ptr;

use libc;
use serde::de::{Deserialize, Deserializer, Error};
use serde::{Serialize, Serializer};
use serde_bytes::ByteBuf;

use super::bindings::*;

fn serialize_ffi<T>(something: &T) -> ByteBuf {
    let mut serialized_self: Vec<u8> = vec![0; mem::size_of::<T>()];
    unsafe {
        libc::memcpy(
            serialized_self.as_mut_ptr() as *mut libc::c_void,
            something as *const T as *const libc::c_void,
            mem::size_of::<T>(),
        );
    }
    ByteBuf::from(serialized_self)
}

fn deserialize_ffi<T>(serialized: ByteBuf) -> T {
    unsafe { ptr::read(serialized.into_vec().as_ptr() as *const T) }
}

impl<Storage, Align> Serialize for __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = serialize_ffi::<__BindgenBitfieldUnit<Storage, Align>>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de, Storage, Align> Deserialize<'de> for __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.is_empty() {
           Err(D::Error::custom("Empty buffer"))
        } else {
            Ok(deserialize_ffi::<__BindgenBitfieldUnit<Storage, Align>>(v))
        }
    }
}

impl<T> Serialize for __IncompleteArrayField<T> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        [0u8; 0].serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for __IncompleteArrayField<T> {
    fn deserialize<D>(_: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(__IncompleteArrayField::new())
    }
}

impl Serialize for __kernel_fd_set {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<__kernel_fd_set>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for __kernel_fd_set {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<__kernel_fd_set>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<__kernel_fd_set>(v))
        }
    }
}


impl Serialize for __kernel_fsid_t {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<__kernel_fsid_t>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for __kernel_fsid_t {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<__kernel_fsid_t>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<__kernel_fsid_t>(v))
        }
    }
}


impl Serialize for kvm_memory_alias {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_memory_alias>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_memory_alias {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_memory_alias>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_memory_alias>(v))
        }
    }
}


impl Serialize for kvm_pic_state {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_pic_state>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_pic_state {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_pic_state>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_pic_state>(v))
        }
    }
}


impl Serialize for kvm_ioapic_state {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_ioapic_state>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_ioapic_state {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_ioapic_state>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_ioapic_state>(v))
        }
    }
}


impl Serialize for kvm_ioapic_state__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_ioapic_state__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_ioapic_state__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_ioapic_state__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_ioapic_state__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_regs {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_regs>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_regs {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_regs>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_regs>(v))
        }
    }
}


impl Serialize for kvm_lapic_state {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_lapic_state>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_lapic_state {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_lapic_state>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_lapic_state>(v))
        }
    }
}


impl Serialize for kvm_segment {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_segment>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_segment {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_segment>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_segment>(v))
        }
    }
}


impl Serialize for kvm_dtable {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_dtable>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_dtable {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_dtable>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_dtable>(v))
        }
    }
}


impl Serialize for kvm_sregs {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_sregs>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_sregs {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_sregs>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_sregs>(v))
        }
    }
}


impl Serialize for kvm_fpu {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_fpu>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_fpu {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_fpu>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_fpu>(v))
        }
    }
}


impl Serialize for kvm_msr_entry {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_msr_entry>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_msr_entry {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_msr_entry>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_msr_entry>(v))
        }
    }
}


impl Serialize for kvm_msrs {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_msrs>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_msrs {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_msrs>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_msrs>(v))
        }
    }
}


impl Serialize for kvm_msr_list {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_msr_list>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_msr_list {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_msr_list>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_msr_list>(v))
        }
    }
}


impl Serialize for kvm_cpuid_entry {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_cpuid_entry>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_cpuid_entry {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_cpuid_entry>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_cpuid_entry>(v))
        }
    }
}


impl Serialize for kvm_cpuid {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_cpuid>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_cpuid {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_cpuid>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_cpuid>(v))
        }
    }
}


impl Serialize for kvm_cpuid_entry2 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_cpuid_entry2>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_cpuid_entry2 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_cpuid_entry2>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_cpuid_entry2>(v))
        }
    }
}


impl Serialize for kvm_cpuid2 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_cpuid2>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_cpuid2 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_cpuid2>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_cpuid2>(v))
        }
    }
}


impl Serialize for kvm_pit_channel_state {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_pit_channel_state>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_pit_channel_state {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_pit_channel_state>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_pit_channel_state>(v))
        }
    }
}


impl Serialize for kvm_debug_exit_arch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_debug_exit_arch>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_debug_exit_arch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_debug_exit_arch>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_debug_exit_arch>(v))
        }
    }
}


impl Serialize for kvm_guest_debug_arch {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_guest_debug_arch>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_guest_debug_arch {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_guest_debug_arch>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_guest_debug_arch>(v))
        }
    }
}


impl Serialize for kvm_pit_state {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_pit_state>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_pit_state {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_pit_state>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_pit_state>(v))
        }
    }
}


impl Serialize for kvm_pit_state2 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_pit_state2>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_pit_state2 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_pit_state2>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_pit_state2>(v))
        }
    }
}


impl Serialize for kvm_reinject_control {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_reinject_control>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_reinject_control {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_reinject_control>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_reinject_control>(v))
        }
    }
}


impl Serialize for kvm_vcpu_events {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_vcpu_events>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_vcpu_events {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_vcpu_events>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_vcpu_events>(v))
        }
    }
}


impl Serialize for kvm_vcpu_events__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_vcpu_events__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_vcpu_events__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_vcpu_events__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_vcpu_events__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_vcpu_events__bindgen_ty_2 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_vcpu_events__bindgen_ty_2>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_vcpu_events__bindgen_ty_2 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_vcpu_events__bindgen_ty_2>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_vcpu_events__bindgen_ty_2>(v))
        }
    }
}


impl Serialize for kvm_vcpu_events__bindgen_ty_3 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_vcpu_events__bindgen_ty_3>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_vcpu_events__bindgen_ty_3 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_vcpu_events__bindgen_ty_3>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_vcpu_events__bindgen_ty_3>(v))
        }
    }
}


impl Serialize for kvm_vcpu_events__bindgen_ty_4 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_vcpu_events__bindgen_ty_4>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_vcpu_events__bindgen_ty_4 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_vcpu_events__bindgen_ty_4>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_vcpu_events__bindgen_ty_4>(v))
        }
    }
}


impl Serialize for kvm_debugregs {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_debugregs>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_debugregs {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_debugregs>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_debugregs>(v))
        }
    }
}


impl Serialize for kvm_xsave {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_xsave>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_xsave {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_xsave>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_xsave>(v))
        }
    }
}


impl Serialize for kvm_xcr {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_xcr>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_xcr {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_xcr>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_xcr>(v))
        }
    }
}


impl Serialize for kvm_xcrs {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_xcrs>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_xcrs {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_xcrs>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_xcrs>(v))
        }
    }
}


impl Serialize for kvm_sync_regs {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_sync_regs>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_sync_regs {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_sync_regs>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_sync_regs>(v))
        }
    }
}


impl Serialize for kvm_user_trace_setup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_user_trace_setup>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_user_trace_setup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_user_trace_setup>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_user_trace_setup>(v))
        }
    }
}


impl Serialize for kvm_breakpoint {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_breakpoint>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_breakpoint {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_breakpoint>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_breakpoint>(v))
        }
    }
}


impl Serialize for kvm_debug_guest {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_debug_guest>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_debug_guest {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_debug_guest>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_debug_guest>(v))
        }
    }
}


impl Serialize for kvm_memory_region {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_memory_region>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_memory_region {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_memory_region>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_memory_region>(v))
        }
    }
}


impl Serialize for kvm_userspace_memory_region {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_userspace_memory_region>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_userspace_memory_region {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_userspace_memory_region>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_userspace_memory_region>(v))
        }
    }
}


impl Serialize for kvm_irq_level {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_irq_level>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irq_level {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_irq_level>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_irq_level>(v))
        }
    }
}


impl Serialize for kvm_irq_level__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_irq_level__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irq_level__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_irq_level__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_irq_level__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_irqchip {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_irqchip>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irqchip {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_irqchip>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_irqchip>(v))
        }
    }
}


impl Serialize for kvm_irqchip__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_irqchip__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irqchip__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_irqchip__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_irqchip__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_pit_config {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_pit_config>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_pit_config {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_pit_config>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_pit_config>(v))
        }
    }
}


impl Serialize for kvm_s390_skeys {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_skeys>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_skeys {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_skeys>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_skeys>(v))
        }
    }
}


impl Serialize for kvm_s390_cmma_log {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_cmma_log>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_cmma_log {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_cmma_log>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_cmma_log>(v))
        }
    }
}


impl Serialize for kvm_s390_cmma_log__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_cmma_log__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_cmma_log__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_cmma_log__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_cmma_log__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_hyperv_exit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_hyperv_exit>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_hyperv_exit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_hyperv_exit>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_hyperv_exit>(v))
        }
    }
}


impl Serialize for kvm_hyperv_exit__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_hyperv_exit__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_hyperv_exit__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_hyperv_exit__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_hyperv_exit__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2>(v))
        }
    }
}


impl Serialize for kvm_run {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_2 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_2>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_2 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_2>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_2>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_3 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_3>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_3 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_3>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_3>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_4 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_4>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_4 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_4>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_4>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_5 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_5>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_5 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_5>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_5>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_6 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_6>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_6 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_6>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_6>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_7 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_7>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_7 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_7>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_7>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_8 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_8>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_8 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_8>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_8>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_9 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_9>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_9 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_9>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_9>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_10 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_10>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_10 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_10>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_10>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_11 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_11>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_11 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_11>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_11>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_12 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_12>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_12 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_12>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_12>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_13 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_13>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_13 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_13>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_13>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_14 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_14>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_14 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_14>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_14>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_15 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_15>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_15 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_15>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_15>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_16 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_16>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_16 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_16>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_16>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_17 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_17>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_17 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_17>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_17>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_18 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_18>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_18 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_18>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_18>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_1__bindgen_ty_19 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_19>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1__bindgen_ty_19 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_19>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_1__bindgen_ty_19>(v))
        }
    }
}


impl Serialize for kvm_run__bindgen_ty_2 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_run__bindgen_ty_2>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_2 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_run__bindgen_ty_2>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_run__bindgen_ty_2>(v))
        }
    }
}


impl Serialize for kvm_coalesced_mmio_zone {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_coalesced_mmio_zone>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_coalesced_mmio_zone {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_coalesced_mmio_zone>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_coalesced_mmio_zone>(v))
        }
    }
}


impl Serialize for kvm_coalesced_mmio {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_coalesced_mmio>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_coalesced_mmio {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_coalesced_mmio>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_coalesced_mmio>(v))
        }
    }
}


impl Serialize for kvm_coalesced_mmio_ring {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_coalesced_mmio_ring>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_coalesced_mmio_ring {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_coalesced_mmio_ring>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_coalesced_mmio_ring>(v))
        }
    }
}


impl Serialize for kvm_translation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_translation>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_translation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_translation>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_translation>(v))
        }
    }
}


impl Serialize for kvm_s390_mem_op {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_mem_op>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_mem_op {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_mem_op>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_mem_op>(v))
        }
    }
}


impl Serialize for kvm_interrupt {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_interrupt>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_interrupt {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_interrupt>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_interrupt>(v))
        }
    }
}


impl Serialize for kvm_dirty_log {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_dirty_log>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_dirty_log {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_dirty_log>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_dirty_log>(v))
        }
    }
}


impl Serialize for kvm_dirty_log__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_dirty_log__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_dirty_log__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_dirty_log__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_dirty_log__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_signal_mask {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_signal_mask>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_signal_mask {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_signal_mask>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_signal_mask>(v))
        }
    }
}


impl Serialize for kvm_tpr_access_ctl {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_tpr_access_ctl>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_tpr_access_ctl {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_tpr_access_ctl>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_tpr_access_ctl>(v))
        }
    }
}


impl Serialize for kvm_vapic_addr {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_vapic_addr>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_vapic_addr {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_vapic_addr>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_vapic_addr>(v))
        }
    }
}


impl Serialize for kvm_mp_state {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_mp_state>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_mp_state {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_mp_state>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_mp_state>(v))
        }
    }
}


impl Serialize for kvm_s390_psw {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_psw>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_psw {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_psw>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_psw>(v))
        }
    }
}


impl Serialize for kvm_s390_interrupt {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_interrupt>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_interrupt {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_interrupt>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_interrupt>(v))
        }
    }
}


impl Serialize for kvm_s390_io_info {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_io_info>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_io_info {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_io_info>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_io_info>(v))
        }
    }
}


impl Serialize for kvm_s390_ext_info {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_ext_info>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_ext_info {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_ext_info>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_ext_info>(v))
        }
    }
}


impl Serialize for kvm_s390_pgm_info {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_pgm_info>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_pgm_info {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_pgm_info>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_pgm_info>(v))
        }
    }
}


impl Serialize for kvm_s390_prefix_info {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_prefix_info>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_prefix_info {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_prefix_info>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_prefix_info>(v))
        }
    }
}


impl Serialize for kvm_s390_extcall_info {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_extcall_info>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_extcall_info {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_extcall_info>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_extcall_info>(v))
        }
    }
}


impl Serialize for kvm_s390_emerg_info {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_emerg_info>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_emerg_info {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_emerg_info>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_emerg_info>(v))
        }
    }
}


impl Serialize for kvm_s390_stop_info {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_stop_info>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_stop_info {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_stop_info>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_stop_info>(v))
        }
    }
}


impl Serialize for kvm_s390_mchk_info {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_mchk_info>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_mchk_info {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_mchk_info>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_mchk_info>(v))
        }
    }
}


impl Serialize for kvm_s390_irq {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_irq>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_irq {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_irq>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_irq>(v))
        }
    }
}


impl Serialize for kvm_s390_irq__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_irq__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_irq__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_irq__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_irq__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_s390_irq_state {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_irq_state>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_irq_state {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_irq_state>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_irq_state>(v))
        }
    }
}


impl Serialize for kvm_guest_debug {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_guest_debug>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_guest_debug {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_guest_debug>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_guest_debug>(v))
        }
    }
}


impl Serialize for kvm_ioeventfd {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_ioeventfd>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_ioeventfd {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_ioeventfd>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_ioeventfd>(v))
        }
    }
}


impl Serialize for kvm_enable_cap {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_enable_cap>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_enable_cap {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_enable_cap>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_enable_cap>(v))
        }
    }
}


impl Serialize for kvm_ppc_pvinfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_ppc_pvinfo>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_ppc_pvinfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_ppc_pvinfo>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_ppc_pvinfo>(v))
        }
    }
}


impl Serialize for kvm_ppc_one_page_size {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_ppc_one_page_size>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_ppc_one_page_size {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_ppc_one_page_size>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_ppc_one_page_size>(v))
        }
    }
}


impl Serialize for kvm_ppc_one_seg_page_size {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_ppc_one_seg_page_size>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_ppc_one_seg_page_size {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_ppc_one_seg_page_size>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_ppc_one_seg_page_size>(v))
        }
    }
}


impl Serialize for kvm_ppc_smmu_info {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_ppc_smmu_info>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_ppc_smmu_info {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_ppc_smmu_info>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_ppc_smmu_info>(v))
        }
    }
}


impl Serialize for kvm_ppc_resize_hpt {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_ppc_resize_hpt>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_ppc_resize_hpt {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_ppc_resize_hpt>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_ppc_resize_hpt>(v))
        }
    }
}


impl Serialize for kvm_irq_routing_irqchip {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_irq_routing_irqchip>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irq_routing_irqchip {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_irq_routing_irqchip>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_irq_routing_irqchip>(v))
        }
    }
}


impl Serialize for kvm_irq_routing_msi {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_irq_routing_msi>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irq_routing_msi {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_irq_routing_msi>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_irq_routing_msi>(v))
        }
    }
}


impl Serialize for kvm_irq_routing_msi__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_irq_routing_msi__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irq_routing_msi__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_irq_routing_msi__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_irq_routing_msi__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_irq_routing_s390_adapter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_irq_routing_s390_adapter>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irq_routing_s390_adapter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_irq_routing_s390_adapter>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_irq_routing_s390_adapter>(v))
        }
    }
}


impl Serialize for kvm_irq_routing_hv_sint {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_irq_routing_hv_sint>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irq_routing_hv_sint {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_irq_routing_hv_sint>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_irq_routing_hv_sint>(v))
        }
    }
}


impl Serialize for kvm_irq_routing_entry {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_irq_routing_entry>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irq_routing_entry {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_irq_routing_entry>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_irq_routing_entry>(v))
        }
    }
}


impl Serialize for kvm_irq_routing_entry__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_irq_routing_entry__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irq_routing_entry__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_irq_routing_entry__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_irq_routing_entry__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_irq_routing {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_irq_routing>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irq_routing {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_irq_routing>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_irq_routing>(v))
        }
    }
}


impl Serialize for kvm_x86_mce {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_x86_mce>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_x86_mce {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_x86_mce>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_x86_mce>(v))
        }
    }
}


impl Serialize for kvm_xen_hvm_config {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_xen_hvm_config>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_xen_hvm_config {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_xen_hvm_config>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_xen_hvm_config>(v))
        }
    }
}


impl Serialize for kvm_irqfd {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_irqfd>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irqfd {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_irqfd>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_irqfd>(v))
        }
    }
}


impl Serialize for kvm_clock_data {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_clock_data>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_clock_data {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_clock_data>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_clock_data>(v))
        }
    }
}


impl Serialize for kvm_config_tlb {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_config_tlb>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_config_tlb {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_config_tlb>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_config_tlb>(v))
        }
    }
}


impl Serialize for kvm_dirty_tlb {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_dirty_tlb>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_dirty_tlb {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_dirty_tlb>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_dirty_tlb>(v))
        }
    }
}


impl Serialize for kvm_reg_list {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_reg_list>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_reg_list {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_reg_list>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_reg_list>(v))
        }
    }
}


impl Serialize for kvm_one_reg {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_one_reg>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_one_reg {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_one_reg>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_one_reg>(v))
        }
    }
}


impl Serialize for kvm_msi {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_msi>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_msi {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_msi>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_msi>(v))
        }
    }
}


impl Serialize for kvm_arm_device_addr {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_arm_device_addr>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_arm_device_addr {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_arm_device_addr>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_arm_device_addr>(v))
        }
    }
}


impl Serialize for kvm_create_device {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_create_device>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_create_device {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_create_device>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_create_device>(v))
        }
    }
}


impl Serialize for kvm_device_attr {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_device_attr>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_device_attr {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_device_attr>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_device_attr>(v))
        }
    }
}


impl Serialize for kvm_vfio_spapr_tce {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_vfio_spapr_tce>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_vfio_spapr_tce {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_vfio_spapr_tce>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_vfio_spapr_tce>(v))
        }
    }
}


impl Serialize for kvm_s390_ucas_mapping {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_s390_ucas_mapping>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_s390_ucas_mapping {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_s390_ucas_mapping>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_s390_ucas_mapping>(v))
        }
    }
}


impl Serialize for kvm_assigned_pci_dev {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_assigned_pci_dev>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_assigned_pci_dev {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_assigned_pci_dev>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_assigned_pci_dev>(v))
        }
    }
}


impl Serialize for kvm_assigned_pci_dev__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_assigned_pci_dev__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_assigned_pci_dev__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_assigned_pci_dev__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_assigned_pci_dev__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_assigned_irq {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_assigned_irq>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_assigned_irq {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_assigned_irq>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_assigned_irq>(v))
        }
    }
}


impl Serialize for kvm_assigned_irq__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_assigned_irq__bindgen_ty_1>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_assigned_irq__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_assigned_irq__bindgen_ty_1>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_assigned_irq__bindgen_ty_1>(v))
        }
    }
}


impl Serialize for kvm_assigned_msix_nr {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_assigned_msix_nr>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_assigned_msix_nr {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_assigned_msix_nr>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_assigned_msix_nr>(v))
        }
    }
}


impl Serialize for kvm_assigned_msix_entry {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<kvm_assigned_msix_entry>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_assigned_msix_entry {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        if v.len() < mem::size_of::<kvm_assigned_msix_entry>() {
            Err(D::Error::custom(format!("Incomplete buffer: size {}", v.len())))
        } else {
            Ok(deserialize_ffi::<kvm_assigned_msix_entry>(v))
        }
    }
}

