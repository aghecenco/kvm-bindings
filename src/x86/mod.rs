// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "fam-wrappers")]
mod fam_wrappers;

#[cfg(feature = "kvm-v4_14_0")]
mod bindings_v4_14_0;
#[cfg(feature = "kvm-v4_20_0")]
mod bindings_v4_20_0;

#[cfg(feature = "with-serde")]
mod serializers;

// Major hack to have a default version in case no feature is specified:
// If no version is specified by using the features, just use the latest one
// which currently is 4.20.
#[cfg(all(not(feature = "kvm-v4_14_0"), not(feature = "kvm-v4_20_0")))]
mod bindings_v4_20_0;

pub mod bindings {
    #[cfg(feature = "kvm-v4_14_0")]
    pub use super::bindings_v4_14_0::*;

    #[cfg(feature = "kvm-v4_20_0")]
    pub use super::bindings_v4_20_0::*;

    #[cfg(all(not(feature = "kvm-v4_14_0"), not(feature = "kvm-v4_20_0")))]
    pub use super::bindings_v4_20_0::*;

    #[cfg(feature = "fam-wrappers")]
    pub use super::fam_wrappers::*;

    #[cfg(feature = "with-serde")]
    pub use super::serializers::*;
}

#[cfg(all(test, feature = "with-serde"))]
mod tests {
    extern crate serde_json;

    use super::bindings::*;

    #[test]
    fn test_ser_deser() {
        {
            // Test kvm_regs ser/deser.
            let val = kvm_regs::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_regs>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_segment ser/deser.
            let val = kvm_segment::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_segment>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_dtable ser/deser.
            let val = kvm_dtable::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_dtable>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_sregs ser/deser.
            let val = kvm_sregs::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_sregs>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_pit_state2 ser/deser.
            // Also covers kvm_pit_channel_state.
            let val = kvm_pit_state2::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_pit_state2>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_vcpu_events ser/deser.
            // Also covers:
            // * kvm_vcpu_events__bindgen_ty_1
            // * kvm_vcpu_events__bindgen_ty_2
            // * kvm_vcpu_events__bindgen_ty_3
            // * kvm_vcpu_events__bindgen_ty_4
            let val = kvm_vcpu_events::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_vcpu_events>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_debugregs ser/deser.
            let val = kvm_debugregs::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_debugregs>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_xcrs ser/deser.
            // Also covers kvm_xcr.
            let val = kvm_xcrs::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_xcrs>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_mp_state ser/deser.
            let val = kvm_mp_state::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_mp_state>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_clock_data ser/deser.
            let val = kvm_clock_data::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_clock_data>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_regs ser/deser.
            let val = kvm_regs::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_regs>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_lapic_state ser/deser.
            let val = kvm_lapic_state::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_lapic_state>(val_ser.as_str()).unwrap();
            assert_eq!(&val.regs[..], &val_deser.regs[..]);
        }

        {
            // Test kvm_xsave ser/deser.
            let val = kvm_xsave::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_xsave>(val_ser.as_str()).unwrap();
            assert_eq!(&val.region[..], &val_deser.region[..]);
        }

        {
            // Test kvm_msrs ser/deser.
            let val = kvm_msrs::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_msrs>(val_ser.as_str()).unwrap();
            // TODO encapsulate in FamStructWrappers and operate on those after
            // https://github.com/rust-vmm/vmm-sys-util/issues/65 is closed.
            // Until then, only partial comparison can be performed.
            assert_eq!(val.nmsrs, val_deser.nmsrs);
        }

        {
            // Test kvm_cpuid2 ser/deser.
            let val = kvm_cpuid2::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_cpuid2>(val_ser.as_str()).unwrap();
            // TODO encapsulate in FamStructWrappers and operate on those after
            // https://github.com/rust-vmm/vmm-sys-util/issues/65 is closed.
            // Until then, only partial comparison can be performed.
            assert_eq!(val.nent, val_deser.nent);
        }
    }
}
