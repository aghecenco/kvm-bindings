// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::fmt;

use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use super::bindings::*;

impl Serialize for kvm_coalesced_mmio_zone__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_coalesced_mmio_zone__bindgen_ty_1", 1)?;
        s.serialize_field("pad", unsafe { &self.pad })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_coalesced_mmio_zone__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_coalesced_mmio_zone__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_coalesced_mmio_zone__bindgen_ty_1_visitor {
            type Value = kvm_coalesced_mmio_zone__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_coalesced_mmio_zone__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let pad: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(kvm_coalesced_mmio_zone__bindgen_ty_1 { pad })
            }
        }

        const FIELDS: &'static [&'static str] = &["pad"];
        deserializer.deserialize_struct(
            "kvm_coalesced_mmio_zone__bindgen_ty_1",
            FIELDS,
            kvm_coalesced_mmio_zone__bindgen_ty_1_visitor,
        )
    }
}

impl Serialize for kvm_coalesced_mmio__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("kvm_coalesced_mmio__bindgen_ty_1", 1)?;
        s.serialize_field("pad", unsafe { &self.pad })?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for kvm_coalesced_mmio__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct kvm_coalesced_mmio__bindgen_ty_1_visitor;

        impl<'de> Visitor<'de> for kvm_coalesced_mmio__bindgen_ty_1_visitor {
            type Value = kvm_coalesced_mmio__bindgen_ty_1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("union kvm_coalesced_mmio__bindgen_ty_1")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let pad: u32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(kvm_coalesced_mmio__bindgen_ty_1 { pad })
            }
        }

        const FIELDS: &'static [&'static str] = &["pad"];
        deserializer.deserialize_struct(
            "kvm_coalesced_mmio__bindgen_ty_1",
            FIELDS,
            kvm_coalesced_mmio__bindgen_ty_1_visitor,
        )
    }
}
