// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::fmt;
use std::mem;
use std::os::raw::c_char;

use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use super::bindings::*;

impl Serialize for kvm_ioapic_state {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_ioapic_state", 6)?;
        s.serialize_field("base_address", &self.base_address)?;
        s.serialize_field("ioregsel", &self.ioregsel)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("irr", &self.irr)?;
        s.serialize_field("pad", &self.pad)?;
        s.serialize_field("redirtbl", &self.redirtbl.to_vec())?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_ioapic_state {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_ioapic_state_visitor;

        impl<'de> Visitor<'de> for kvm_ioapic_state_visitor {
            type Value = kvm_ioapic_state;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct kvm_ioapic_state")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let base_address: u64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let ioregsel: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let id: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let irr: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let pad: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let redirtbl_vec: Vec<kvm_ioapic_state__bindgen_ty_1> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(5, &self))?;
                let mut redirtbl: [kvm_ioapic_state__bindgen_ty_1; 24] =
                    unsafe { mem::uninitialized() };
                redirtbl.copy_from_slice(redirtbl_vec.as_slice());

                Ok(kvm_ioapic_state {
                    base_address,
                    ioregsel,
                    id,
                    irr,
                    pad,
                    redirtbl,
                })
            }
        }

        const FIELDS: &'static [&'static str] =
            &["base_address", "ioregsel", "id", "irr", "pad", "redirtbl"];
        deserializer.deserialize_struct("kvm_ioapic_state", FIELDS, kvm_ioapic_state_visitor)
    }
}

impl Serialize for kvm_ioapic_state__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_ioapic_state__bindgen_ty_1", 1)?;
        s.serialize_field("bits", unsafe { &self.bits })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_ioapic_state__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_ioapic_state__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_ioapic_state__bindgen_ty_1_visitor {
            type Value = kvm_ioapic_state__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_ioapic_state__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let bits: u64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(kvm_ioapic_state__bindgen_ty_1 { bits })
            }
        }

        const FIELDS: &'static [&'static str] = &["bits"];
        deserializer.deserialize_struct(
            "kvm_ioapic_state__bindgen_ty_1",
            FIELDS,
            kvm_ioapic_state__bindgen_ty_1_visitor,
        )
    }
}

impl Serialize for kvm_lapic_state {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_lapic_state", 1)?;
        s.serialize_field("regs", &self.regs.to_vec())?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_lapic_state {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_lapic_state_visitor;

        impl<'de> Visitor<'de> for kvm_lapic_state_visitor {
            type Value = kvm_lapic_state;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct kvm_lapic_state")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let regs_vec: Vec<c_char> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let mut regs: [c_char; 1024] = [0; 1024];
                regs.copy_from_slice(regs_vec.as_slice());

                Ok(kvm_lapic_state { regs })
            }
        }

        const FIELDS: &'static [&'static str] = &["regs"];
        deserializer.deserialize_struct("kvm_lapic_state", FIELDS, kvm_lapic_state_visitor)
    }
}

impl Serialize for kvm_xsave {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_xsave", 1)?;
        s.serialize_field("region", &self.region.to_vec())?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_xsave {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_xsave_visitor;

        impl<'de> Visitor<'de> for kvm_xsave_visitor {
            type Value = kvm_xsave;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct kvm_xsave")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let region_vec: Vec<u32> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let mut region: [u32; 1024] = [0; 1024];
                region.copy_from_slice(region_vec.as_slice());

                Ok(kvm_xsave { region })
            }
        }

        const FIELDS: &'static [&'static str] = &["region"];
        deserializer.deserialize_struct("kvm_xsave", FIELDS, kvm_xsave_visitor)
    }
}

impl Serialize for kvm_irq_level__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_irq_level__bindgen_ty_1", 1)?;
        s.serialize_field("irq", unsafe { &self.irq })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_irq_level__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_irq_level__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_irq_level__bindgen_ty_1_visitor {
            type Value = kvm_irq_level__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_irq_level__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let irq: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(kvm_irq_level__bindgen_ty_1 { irq })
            }
        }

        const FIELDS: &'static [&'static str] = &["irq"];
        deserializer.deserialize_struct(
            "kvm_irq_level__bindgen_ty_1",
            FIELDS,
            kvm_irq_level__bindgen_ty_1_visitor,
        )
    }
}

impl Serialize for kvm_irqchip__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_irqchip__bindgen_ty_1", 1)?;
        s.serialize_field("dummy", unsafe { &self.dummy.to_vec() })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_irqchip__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_irqchip__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_irqchip__bindgen_ty_1_visitor {
            type Value = kvm_irqchip__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_irqchip__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let dummy_vec: Vec<c_char> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let mut dummy: [c_char; 512] = [0; 512];
                dummy.copy_from_slice(dummy_vec.as_slice());
                Ok(kvm_irqchip__bindgen_ty_1 { dummy })
            }
        }

        const FIELDS: &'static [&'static str] = &["dummy"];
        deserializer.deserialize_struct(
            "kvm_irqchip__bindgen_ty_1",
            FIELDS,
            kvm_irqchip__bindgen_ty_1_visitor,
        )
    }
}

impl Serialize for kvm_s390_cmma_log__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_s390_cmma_log__bindgen_ty_1", 1)?;
        s.serialize_field("remaining", unsafe { &self.remaining })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_s390_cmma_log__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_s390_cmma_log__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_s390_cmma_log__bindgen_ty_1_visitor {
            type Value = kvm_s390_cmma_log__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_s390_cmma_log__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let remaining: u64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(kvm_s390_cmma_log__bindgen_ty_1 { remaining })
            }
        }

        const FIELDS: &'static [&'static str] = &["remaining"];
        deserializer.deserialize_struct(
            "kvm_s390_cmma_log__bindgen_ty_1",
            FIELDS,
            kvm_s390_cmma_log__bindgen_ty_1_visitor,
        )
    }
}

impl Serialize for kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s =
            serializer.serialize_struct("kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2", 1)?;
        s.serialize_field("input", &self.input)?;
        s.serialize_field("result", &self.result)?;
        s.serialize_field("params", &self.params.to_vec())?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2_visitor;

        impl<'de> Visitor<'de> for kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2_visitor {
            type Value = kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let input: u64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let result: u64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let params_vec: Vec<u64> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let mut params: [u64; 2] = [0; 2];
                params.copy_from_slice(params_vec.as_slice());

                Ok(kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2 {
                    input,
                    result,
                    params,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["imput", "result", "params"];
        deserializer.deserialize_struct(
            "kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2",
            FIELDS,
            kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2_visitor,
        )
    }
}

impl Serialize for kvm_hyperv_exit__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_hyperv_exit__bindgen_ty_1", 1)?;
        s.serialize_field("hcall", unsafe { &self.hcall })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_hyperv_exit__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_hyperv_exit__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_hyperv_exit__bindgen_ty_1_visitor {
            type Value = kvm_hyperv_exit__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_hyperv_exit__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let hcall: kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(kvm_hyperv_exit__bindgen_ty_1 { hcall })
            }
        }

        const FIELDS: &'static [&'static str] = &["hcall"];
        deserializer.deserialize_struct(
            "kvm_hyperv_exit__bindgen_ty_1",
            FIELDS,
            kvm_hyperv_exit__bindgen_ty_1_visitor,
        )
    }
}

impl Serialize for kvm_run__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_run__bindgen_ty_1", 1)?;
        s.serialize_field("padding", unsafe { &self.padding.to_vec() })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_run__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_run__bindgen_ty_1_visitor {
            type Value = kvm_run__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_run__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let padding_vec: Vec<c_char> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let mut padding: [c_char; 256] = [0; 256];
                padding.copy_from_slice(padding_vec.as_slice());

                Ok(kvm_run__bindgen_ty_1 { padding })
            }
        }

        const FIELDS: &'static [&'static str] = &["padding"];
        deserializer.deserialize_struct(
            "kvm_run__bindgen_ty_1",
            FIELDS,
            kvm_run__bindgen_ty_1_visitor,
        )
    }
}

impl Serialize for kvm_run__bindgen_ty_2 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_run__bindgen_ty_2", 1)?;
        s.serialize_field("regs", unsafe { &self.regs })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_run__bindgen_ty_2 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_run__bindgen_ty_2_visitor;

        impl<'de> Visitor<'de> for kvm_run__bindgen_ty_2_visitor {
            type Value = kvm_run__bindgen_ty_2;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_run__bindgen_ty_2")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let regs: kvm_sync_regs = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(kvm_run__bindgen_ty_2 { regs })
            }
        }

        const FIELDS: &'static [&'static str] = &["regs"];
        deserializer.deserialize_struct(
            "kvm_run__bindgen_ty_2",
            FIELDS,
            kvm_run__bindgen_ty_2_visitor,
        )
    }
}

impl Serialize for kvm_dirty_log__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_dirty_log__bindgen_ty_1", 1)?;
        s.serialize_field("padding2", unsafe { &self.padding2 })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_dirty_log__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_dirty_log__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_dirty_log__bindgen_ty_1_visitor {
            type Value = kvm_dirty_log__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_dirty_log__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let padding2: u64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(kvm_dirty_log__bindgen_ty_1 { padding2 })
            }
        }

        const FIELDS: &'static [&'static str] = &["padding2"];
        deserializer.deserialize_struct(
            "kvm_dirty_log__bindgen_ty_1",
            FIELDS,
            kvm_dirty_log__bindgen_ty_1_visitor,
        )
    }
}

impl Serialize for kvm_s390_irq__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_s390_irq__bindgen_ty_1", 1)?;
        s.serialize_field("reserved", unsafe { &self.reserved.to_vec() })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_s390_irq__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_s390_irq__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_s390_irq__bindgen_ty_1_visitor {
            type Value = kvm_s390_irq__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_s390_irq__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let reserved_vec: Vec<c_char> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let mut reserved: [c_char; 64] = [0; 64];
                reserved.copy_from_slice(reserved_vec.as_slice());

                Ok(kvm_s390_irq__bindgen_ty_1 { reserved })
            }
        }

        const FIELDS: &'static [&'static str] = &["reserved"];
        deserializer.deserialize_struct(
            "kvm_s390_irq__bindgen_ty_1",
            FIELDS,
            kvm_s390_irq__bindgen_ty_1_visitor,
        )
    }
}

impl Serialize for kvm_ioeventfd {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_ioeventfd", 5)?;
        s.serialize_field("datamatch", &self.datamatch)?;
        s.serialize_field("addr", &self.addr)?;
        s.serialize_field("len", &self.len)?;
        s.serialize_field("fd", &self.fd)?;
        s.serialize_field("flags", &self.flags)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_ioeventfd {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_ioeventfd_visitor;

        impl<'de> Visitor<'de> for kvm_ioeventfd_visitor {
            type Value = kvm_ioeventfd;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct kvm_ioeventfd")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let datamatch: u64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let addr: u64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let len: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let fd: i32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let flags: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let pad = [u8::from(0); 36];

                Ok(kvm_ioeventfd {
                    datamatch,
                    addr,
                    len,
                    fd,
                    flags,
                    pad,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["datamatch", "addr", "len", "fd", "flags", "pad"];
        deserializer.deserialize_struct("kvm_ioeventfd", FIELDS, kvm_ioeventfd_visitor)
    }
}

impl Serialize for kvm_ppc_pvinfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_ppc_pvinfo", 2)?;
        s.serialize_field("flags", &self.flags)?;
        s.serialize_field("hcall", &self.hcall.to_vec())?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_ppc_pvinfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_ppc_pvinfo_visitor;

        impl<'de> Visitor<'de> for kvm_ppc_pvinfo_visitor {
            type Value = kvm_ppc_pvinfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct kvm_ppc_pvinfo")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let flags: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let hcall_vec: Vec<u32> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let mut hcall: [u32; 4] = [0; 4];
                hcall.copy_from_slice(hcall_vec.as_slice());
                let pad: [u8; 108] = [0; 108];

                Ok(kvm_ppc_pvinfo { flags, hcall, pad })
            }
        }

        const FIELDS: &'static [&'static str] = &["flags", "hcall", "pad"];
        deserializer.deserialize_struct("kvm_ppc_pvinfo", FIELDS, kvm_ppc_pvinfo_visitor)
    }
}

impl Serialize for kvm_irq_routing_msi__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_irq_routing_msi__bindgen_ty_1", 1)?;
        s.serialize_field("devid", unsafe { &self.devid })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_irq_routing_msi__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_irq_routing_msi__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_irq_routing_msi__bindgen_ty_1_visitor {
            type Value = kvm_irq_routing_msi__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_irq_routing_msi__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let devid: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(kvm_irq_routing_msi__bindgen_ty_1 { devid })
            }
        }

        const FIELDS: &'static [&'static str] = &["devid"];
        deserializer.deserialize_struct(
            "kvm_irq_routing_msi__bindgen_ty_1",
            FIELDS,
            kvm_irq_routing_msi__bindgen_ty_1_visitor,
        )
    }
}

impl Serialize for kvm_irq_routing_entry__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_irq_routing_entry__bindgen_ty_1", 1)?;
        s.serialize_field("pad", unsafe { &self.pad.to_vec() })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_irq_routing_entry__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_irq_routing_entry__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_irq_routing_entry__bindgen_ty_1_visitor {
            type Value = kvm_irq_routing_entry__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_irq_routing_entry__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let pad_vec: Vec<u32> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let mut pad: [u32; 8] = [0; 8];
                pad.copy_from_slice(pad_vec.as_slice());

                Ok(kvm_irq_routing_entry__bindgen_ty_1 { pad })
            }
        }

        const FIELDS: &'static [&'static str] = &["pad"];
        deserializer.deserialize_struct(
            "kvm_irq_routing_entry__bindgen_ty_1",
            FIELDS,
            kvm_irq_routing_entry__bindgen_ty_1_visitor,
        )
    }
}

impl Serialize for kvm_assigned_pci_dev__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_assigned_pci_dev__bindgen_ty_1", 1)?;
        s.serialize_field("reserved", unsafe { &self.reserved.to_vec() })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_assigned_pci_dev__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_assigned_pci_dev__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_assigned_pci_dev__bindgen_ty_1_visitor {
            type Value = kvm_assigned_pci_dev__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_assigned_pci_dev__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let reserved_vec: Vec<u32> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let mut reserved: [u32; 11] = [0; 11];
                reserved.copy_from_slice(reserved_vec.as_slice());

                Ok(kvm_assigned_pci_dev__bindgen_ty_1 { reserved })
            }
        }

        const FIELDS: &'static [&'static str] = &["reserved"];
        deserializer.deserialize_struct(
            "kvm_assigned_pci_dev__bindgen_ty_1",
            FIELDS,
            kvm_assigned_pci_dev__bindgen_ty_1_visitor,
        )
    }
}

impl Serialize for kvm_assigned_irq__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_assigned_irq__bindgen_ty_1", 1)?;
        s.serialize_field("reserved", unsafe { &self.reserved.to_vec() })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_assigned_irq__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_assigned_irq__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_assigned_irq__bindgen_ty_1_visitor {
            type Value = kvm_assigned_irq__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_assigned_irq__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let reserved_vec: Vec<u32> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let mut reserved: [u32; 12] = [0; 12];
                reserved.copy_from_slice(reserved_vec.as_slice());

                Ok(kvm_assigned_irq__bindgen_ty_1 { reserved })
            }
        }

        const FIELDS: &'static [&'static str] = &["reserved"];
        deserializer.deserialize_struct(
            "kvm_assigned_irq__bindgen_ty_1",
            FIELDS,
            kvm_assigned_irq__bindgen_ty_1_visitor,
        )
    }
}

impl Serialize for kvm_enable_cap {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_enable_cap", 3)?;
        s.serialize_field("cap", &self.cap)?;
        s.serialize_field("flags", &self.flags)?;
        s.serialize_field("args", &self.args.to_vec())?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_enable_cap {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_enable_cap_visitor;

        impl<'de> Visitor<'de> for kvm_enable_cap_visitor {
            type Value = kvm_enable_cap;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct kvm_enable_cap")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let cap: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let flags: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let args_vec: Vec<u64> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let mut args: [u64; 4] = [0; 4];
                args.copy_from_slice(args_vec.as_slice());
                let pad: [u8; 64] = [0; 64];

                Ok(kvm_enable_cap {
                    cap,
                    flags,
                    args,
                    pad,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["cap", "flags", "args"];
        deserializer.deserialize_struct("kvm_enable_cap", FIELDS, kvm_enable_cap_visitor)
    }
}
