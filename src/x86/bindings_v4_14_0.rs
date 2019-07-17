// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#[repr(C)]
#[derive(
    Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
#[derive(Default, Serialize, Deserialize)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
pub const __BITS_PER_LONG: u32 = 64;
pub const __FD_SETSIZE: u32 = 1024;
pub const _IOC_NRBITS: u32 = 8;
pub const _IOC_TYPEBITS: u32 = 8;
pub const _IOC_SIZEBITS: u32 = 14;
pub const _IOC_DIRBITS: u32 = 2;
pub const _IOC_NRMASK: u32 = 255;
pub const _IOC_TYPEMASK: u32 = 255;
pub const _IOC_SIZEMASK: u32 = 16383;
pub const _IOC_DIRMASK: u32 = 3;
pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = 8;
pub const _IOC_SIZESHIFT: u32 = 16;
pub const _IOC_DIRSHIFT: u32 = 30;
pub const _IOC_NONE: u32 = 0;
pub const _IOC_WRITE: u32 = 1;
pub const _IOC_READ: u32 = 2;
pub const IOC_IN: u32 = 1073741824;
pub const IOC_OUT: u32 = 2147483648;
pub const IOC_INOUT: u32 = 3221225472;
pub const IOCSIZE_MASK: u32 = 1073676288;
pub const IOCSIZE_SHIFT: u32 = 16;
pub const KVM_PIO_PAGE_OFFSET: u32 = 1;
pub const KVM_COALESCED_MMIO_PAGE_OFFSET: u32 = 2;
pub const DE_VECTOR: u32 = 0;
pub const DB_VECTOR: u32 = 1;
pub const BP_VECTOR: u32 = 3;
pub const OF_VECTOR: u32 = 4;
pub const BR_VECTOR: u32 = 5;
pub const UD_VECTOR: u32 = 6;
pub const NM_VECTOR: u32 = 7;
pub const DF_VECTOR: u32 = 8;
pub const TS_VECTOR: u32 = 10;
pub const NP_VECTOR: u32 = 11;
pub const SS_VECTOR: u32 = 12;
pub const GP_VECTOR: u32 = 13;
pub const PF_VECTOR: u32 = 14;
pub const MF_VECTOR: u32 = 16;
pub const AC_VECTOR: u32 = 17;
pub const MC_VECTOR: u32 = 18;
pub const XM_VECTOR: u32 = 19;
pub const VE_VECTOR: u32 = 20;
pub const KVM_NR_INTERRUPTS: u32 = 256;
pub const KVM_IOAPIC_NUM_PINS: u32 = 24;
pub const KVM_IRQCHIP_PIC_MASTER: u32 = 0;
pub const KVM_IRQCHIP_PIC_SLAVE: u32 = 1;
pub const KVM_IRQCHIP_IOAPIC: u32 = 2;
pub const KVM_NR_IRQCHIPS: u32 = 3;
pub const KVM_RUN_X86_SMM: u32 = 1;
pub const KVM_APIC_REG_SIZE: u32 = 1024;
pub const KVM_CPUID_FLAG_SIGNIFCANT_INDEX: u32 = 1;
pub const KVM_CPUID_FLAG_STATEFUL_FUNC: u32 = 2;
pub const KVM_CPUID_FLAG_STATE_READ_NEXT: u32 = 4;
pub const KVM_GUESTDBG_USE_SW_BP: u32 = 65536;
pub const KVM_GUESTDBG_USE_HW_BP: u32 = 131072;
pub const KVM_GUESTDBG_INJECT_DB: u32 = 262144;
pub const KVM_GUESTDBG_INJECT_BP: u32 = 524288;
pub const KVM_PIT_FLAGS_HPET_LEGACY: u32 = 1;
pub const KVM_VCPUEVENT_VALID_NMI_PENDING: u32 = 1;
pub const KVM_VCPUEVENT_VALID_SIPI_VECTOR: u32 = 2;
pub const KVM_VCPUEVENT_VALID_SHADOW: u32 = 4;
pub const KVM_VCPUEVENT_VALID_SMM: u32 = 8;
pub const KVM_X86_SHADOW_INT_MOV_SS: u32 = 1;
pub const KVM_X86_SHADOW_INT_STI: u32 = 2;
pub const KVM_MAX_XCRS: u32 = 16;
pub const KVM_X86_QUIRK_LINT0_REENABLED: u32 = 1;
pub const KVM_X86_QUIRK_CD_NW_CLEARED: u32 = 2;
pub const KVM_API_VERSION: u32 = 12;
pub const KVM_TRC_SHIFT: u32 = 16;
pub const KVM_TRC_ENTRYEXIT: u32 = 65536;
pub const KVM_TRC_HANDLER: u32 = 131072;
pub const KVM_TRC_VMENTRY: u32 = 65537;
pub const KVM_TRC_VMEXIT: u32 = 65538;
pub const KVM_TRC_PAGE_FAULT: u32 = 131073;
pub const KVM_TRC_HEAD_SIZE: u32 = 12;
pub const KVM_TRC_CYCLE_SIZE: u32 = 8;
pub const KVM_TRC_EXTRA_MAX: u32 = 7;
pub const KVM_TRC_INJ_VIRQ: u32 = 131074;
pub const KVM_TRC_REDELIVER_EVT: u32 = 131075;
pub const KVM_TRC_PEND_INTR: u32 = 131076;
pub const KVM_TRC_IO_READ: u32 = 131077;
pub const KVM_TRC_IO_WRITE: u32 = 131078;
pub const KVM_TRC_CR_READ: u32 = 131079;
pub const KVM_TRC_CR_WRITE: u32 = 131080;
pub const KVM_TRC_DR_READ: u32 = 131081;
pub const KVM_TRC_DR_WRITE: u32 = 131082;
pub const KVM_TRC_MSR_READ: u32 = 131083;
pub const KVM_TRC_MSR_WRITE: u32 = 131084;
pub const KVM_TRC_CPUID: u32 = 131085;
pub const KVM_TRC_INTR: u32 = 131086;
pub const KVM_TRC_NMI: u32 = 131087;
pub const KVM_TRC_VMMCALL: u32 = 131088;
pub const KVM_TRC_HLT: u32 = 131089;
pub const KVM_TRC_CLTS: u32 = 131090;
pub const KVM_TRC_LMSW: u32 = 131091;
pub const KVM_TRC_APIC_ACCESS: u32 = 131092;
pub const KVM_TRC_TDP_FAULT: u32 = 131093;
pub const KVM_TRC_GTLB_WRITE: u32 = 131094;
pub const KVM_TRC_STLB_WRITE: u32 = 131095;
pub const KVM_TRC_STLB_INVAL: u32 = 131096;
pub const KVM_TRC_PPC_INSTR: u32 = 131097;
pub const KVM_MEM_LOG_DIRTY_PAGES: u32 = 1;
pub const KVM_MEM_READONLY: u32 = 2;
pub const KVM_PIT_SPEAKER_DUMMY: u32 = 1;
pub const KVM_S390_CMMA_PEEK: u32 = 1;
pub const KVM_EXIT_HYPERV_SYNIC: u32 = 1;
pub const KVM_EXIT_HYPERV_HCALL: u32 = 2;
pub const KVM_S390_GET_SKEYS_NONE: u32 = 1;
pub const KVM_S390_SKEYS_MAX: u32 = 1048576;
pub const KVM_EXIT_UNKNOWN: u32 = 0;
pub const KVM_EXIT_EXCEPTION: u32 = 1;
pub const KVM_EXIT_IO: u32 = 2;
pub const KVM_EXIT_HYPERCALL: u32 = 3;
pub const KVM_EXIT_DEBUG: u32 = 4;
pub const KVM_EXIT_HLT: u32 = 5;
pub const KVM_EXIT_MMIO: u32 = 6;
pub const KVM_EXIT_IRQ_WINDOW_OPEN: u32 = 7;
pub const KVM_EXIT_SHUTDOWN: u32 = 8;
pub const KVM_EXIT_FAIL_ENTRY: u32 = 9;
pub const KVM_EXIT_INTR: u32 = 10;
pub const KVM_EXIT_SET_TPR: u32 = 11;
pub const KVM_EXIT_TPR_ACCESS: u32 = 12;
pub const KVM_EXIT_S390_SIEIC: u32 = 13;
pub const KVM_EXIT_S390_RESET: u32 = 14;
pub const KVM_EXIT_DCR: u32 = 15;
pub const KVM_EXIT_NMI: u32 = 16;
pub const KVM_EXIT_INTERNAL_ERROR: u32 = 17;
pub const KVM_EXIT_OSI: u32 = 18;
pub const KVM_EXIT_PAPR_HCALL: u32 = 19;
pub const KVM_EXIT_S390_UCONTROL: u32 = 20;
pub const KVM_EXIT_WATCHDOG: u32 = 21;
pub const KVM_EXIT_S390_TSCH: u32 = 22;
pub const KVM_EXIT_EPR: u32 = 23;
pub const KVM_EXIT_SYSTEM_EVENT: u32 = 24;
pub const KVM_EXIT_S390_STSI: u32 = 25;
pub const KVM_EXIT_IOAPIC_EOI: u32 = 26;
pub const KVM_EXIT_HYPERV: u32 = 27;
pub const KVM_INTERNAL_ERROR_EMULATION: u32 = 1;
pub const KVM_INTERNAL_ERROR_SIMUL_EX: u32 = 2;
pub const KVM_INTERNAL_ERROR_DELIVERY_EV: u32 = 3;
pub const KVM_EXIT_IO_IN: u32 = 0;
pub const KVM_EXIT_IO_OUT: u32 = 1;
pub const KVM_S390_RESET_POR: u32 = 1;
pub const KVM_S390_RESET_CLEAR: u32 = 2;
pub const KVM_S390_RESET_SUBSYSTEM: u32 = 4;
pub const KVM_S390_RESET_CPU_INIT: u32 = 8;
pub const KVM_S390_RESET_IPL: u32 = 16;
pub const KVM_SYSTEM_EVENT_SHUTDOWN: u32 = 1;
pub const KVM_SYSTEM_EVENT_RESET: u32 = 2;
pub const KVM_SYSTEM_EVENT_CRASH: u32 = 3;
pub const KVM_S390_MEMOP_LOGICAL_READ: u32 = 0;
pub const KVM_S390_MEMOP_LOGICAL_WRITE: u32 = 1;
pub const KVM_S390_MEMOP_F_CHECK_ONLY: u32 = 1;
pub const KVM_S390_MEMOP_F_INJECT_EXCEPTION: u32 = 2;
pub const KVM_MP_STATE_RUNNABLE: u32 = 0;
pub const KVM_MP_STATE_UNINITIALIZED: u32 = 1;
pub const KVM_MP_STATE_INIT_RECEIVED: u32 = 2;
pub const KVM_MP_STATE_HALTED: u32 = 3;
pub const KVM_MP_STATE_SIPI_RECEIVED: u32 = 4;
pub const KVM_MP_STATE_STOPPED: u32 = 5;
pub const KVM_MP_STATE_CHECK_STOP: u32 = 6;
pub const KVM_MP_STATE_OPERATING: u32 = 7;
pub const KVM_MP_STATE_LOAD: u32 = 8;
pub const KVM_S390_SIGP_STOP: u32 = 4294836224;
pub const KVM_S390_PROGRAM_INT: u32 = 4294836225;
pub const KVM_S390_SIGP_SET_PREFIX: u32 = 4294836226;
pub const KVM_S390_RESTART: u32 = 4294836227;
pub const KVM_S390_INT_PFAULT_INIT: u32 = 4294836228;
pub const KVM_S390_INT_PFAULT_DONE: u32 = 4294836229;
pub const KVM_S390_MCHK: u32 = 4294840320;
pub const KVM_S390_INT_CLOCK_COMP: u32 = 4294905860;
pub const KVM_S390_INT_CPU_TIMER: u32 = 4294905861;
pub const KVM_S390_INT_VIRTIO: u32 = 4294911491;
pub const KVM_S390_INT_SERVICE: u32 = 4294910977;
pub const KVM_S390_INT_EMERGENCY: u32 = 4294906369;
pub const KVM_S390_INT_EXTERNAL_CALL: u32 = 4294906370;
pub const KVM_S390_INT_IO_MIN: u32 = 0;
pub const KVM_S390_INT_IO_MAX: u32 = 4294836223;
pub const KVM_S390_INT_IO_AI_MASK: u32 = 67108864;
pub const KVM_S390_PGM_FLAGS_ILC_VALID: u32 = 1;
pub const KVM_S390_PGM_FLAGS_ILC_0: u32 = 2;
pub const KVM_S390_PGM_FLAGS_ILC_1: u32 = 4;
pub const KVM_S390_PGM_FLAGS_ILC_MASK: u32 = 6;
pub const KVM_S390_PGM_FLAGS_NO_REWIND: u32 = 8;
pub const KVM_S390_STOP_FLAG_STORE_STATUS: u32 = 1;
pub const KVM_GUESTDBG_ENABLE: u32 = 1;
pub const KVM_GUESTDBG_SINGLESTEP: u32 = 2;
pub const KVM_PPC_PVINFO_FLAGS_EV_IDLE: u32 = 1;
pub const KVM_PPC_PAGE_SIZES_MAX_SZ: u32 = 8;
pub const KVM_PPC_PAGE_SIZES_REAL: u32 = 1;
pub const KVM_PPC_1T_SEGMENTS: u32 = 2;
pub const KVMIO: u32 = 174;
pub const KVM_VM_S390_UCONTROL: u32 = 1;
pub const KVM_VM_PPC_HV: u32 = 1;
pub const KVM_VM_PPC_PR: u32 = 2;
pub const KVM_VM_MIPS_TE: u32 = 0;
pub const KVM_VM_MIPS_VZ: u32 = 1;
pub const KVM_S390_SIE_PAGE_OFFSET: u32 = 1;
pub const KVM_CAP_IRQCHIP: u32 = 0;
pub const KVM_CAP_HLT: u32 = 1;
pub const KVM_CAP_MMU_SHADOW_CACHE_CONTROL: u32 = 2;
pub const KVM_CAP_USER_MEMORY: u32 = 3;
pub const KVM_CAP_SET_TSS_ADDR: u32 = 4;
pub const KVM_CAP_VAPIC: u32 = 6;
pub const KVM_CAP_EXT_CPUID: u32 = 7;
pub const KVM_CAP_CLOCKSOURCE: u32 = 8;
pub const KVM_CAP_NR_VCPUS: u32 = 9;
pub const KVM_CAP_NR_MEMSLOTS: u32 = 10;
pub const KVM_CAP_PIT: u32 = 11;
pub const KVM_CAP_NOP_IO_DELAY: u32 = 12;
pub const KVM_CAP_PV_MMU: u32 = 13;
pub const KVM_CAP_MP_STATE: u32 = 14;
pub const KVM_CAP_COALESCED_MMIO: u32 = 15;
pub const KVM_CAP_SYNC_MMU: u32 = 16;
pub const KVM_CAP_IOMMU: u32 = 18;
pub const KVM_CAP_DESTROY_MEMORY_REGION_WORKS: u32 = 21;
pub const KVM_CAP_USER_NMI: u32 = 22;
pub const KVM_CAP_SET_GUEST_DEBUG: u32 = 23;
pub const KVM_CAP_REINJECT_CONTROL: u32 = 24;
pub const KVM_CAP_IRQ_ROUTING: u32 = 25;
pub const KVM_CAP_IRQ_INJECT_STATUS: u32 = 26;
pub const KVM_CAP_ASSIGN_DEV_IRQ: u32 = 29;
pub const KVM_CAP_JOIN_MEMORY_REGIONS_WORKS: u32 = 30;
pub const KVM_CAP_MCE: u32 = 31;
pub const KVM_CAP_IRQFD: u32 = 32;
pub const KVM_CAP_PIT2: u32 = 33;
pub const KVM_CAP_SET_BOOT_CPU_ID: u32 = 34;
pub const KVM_CAP_PIT_STATE2: u32 = 35;
pub const KVM_CAP_IOEVENTFD: u32 = 36;
pub const KVM_CAP_SET_IDENTITY_MAP_ADDR: u32 = 37;
pub const KVM_CAP_XEN_HVM: u32 = 38;
pub const KVM_CAP_ADJUST_CLOCK: u32 = 39;
pub const KVM_CAP_INTERNAL_ERROR_DATA: u32 = 40;
pub const KVM_CAP_VCPU_EVENTS: u32 = 41;
pub const KVM_CAP_S390_PSW: u32 = 42;
pub const KVM_CAP_PPC_SEGSTATE: u32 = 43;
pub const KVM_CAP_HYPERV: u32 = 44;
pub const KVM_CAP_HYPERV_VAPIC: u32 = 45;
pub const KVM_CAP_HYPERV_SPIN: u32 = 46;
pub const KVM_CAP_PCI_SEGMENT: u32 = 47;
pub const KVM_CAP_PPC_PAIRED_SINGLES: u32 = 48;
pub const KVM_CAP_INTR_SHADOW: u32 = 49;
pub const KVM_CAP_DEBUGREGS: u32 = 50;
pub const KVM_CAP_X86_ROBUST_SINGLESTEP: u32 = 51;
pub const KVM_CAP_PPC_OSI: u32 = 52;
pub const KVM_CAP_PPC_UNSET_IRQ: u32 = 53;
pub const KVM_CAP_ENABLE_CAP: u32 = 54;
pub const KVM_CAP_XSAVE: u32 = 55;
pub const KVM_CAP_XCRS: u32 = 56;
pub const KVM_CAP_PPC_GET_PVINFO: u32 = 57;
pub const KVM_CAP_PPC_IRQ_LEVEL: u32 = 58;
pub const KVM_CAP_ASYNC_PF: u32 = 59;
pub const KVM_CAP_TSC_CONTROL: u32 = 60;
pub const KVM_CAP_GET_TSC_KHZ: u32 = 61;
pub const KVM_CAP_PPC_BOOKE_SREGS: u32 = 62;
pub const KVM_CAP_SPAPR_TCE: u32 = 63;
pub const KVM_CAP_PPC_SMT: u32 = 64;
pub const KVM_CAP_PPC_RMA: u32 = 65;
pub const KVM_CAP_MAX_VCPUS: u32 = 66;
pub const KVM_CAP_PPC_HIOR: u32 = 67;
pub const KVM_CAP_PPC_PAPR: u32 = 68;
pub const KVM_CAP_SW_TLB: u32 = 69;
pub const KVM_CAP_ONE_REG: u32 = 70;
pub const KVM_CAP_S390_GMAP: u32 = 71;
pub const KVM_CAP_TSC_DEADLINE_TIMER: u32 = 72;
pub const KVM_CAP_S390_UCONTROL: u32 = 73;
pub const KVM_CAP_SYNC_REGS: u32 = 74;
pub const KVM_CAP_PCI_2_3: u32 = 75;
pub const KVM_CAP_KVMCLOCK_CTRL: u32 = 76;
pub const KVM_CAP_SIGNAL_MSI: u32 = 77;
pub const KVM_CAP_PPC_GET_SMMU_INFO: u32 = 78;
pub const KVM_CAP_S390_COW: u32 = 79;
pub const KVM_CAP_PPC_ALLOC_HTAB: u32 = 80;
pub const KVM_CAP_READONLY_MEM: u32 = 81;
pub const KVM_CAP_IRQFD_RESAMPLE: u32 = 82;
pub const KVM_CAP_PPC_BOOKE_WATCHDOG: u32 = 83;
pub const KVM_CAP_PPC_HTAB_FD: u32 = 84;
pub const KVM_CAP_S390_CSS_SUPPORT: u32 = 85;
pub const KVM_CAP_PPC_EPR: u32 = 86;
pub const KVM_CAP_ARM_PSCI: u32 = 87;
pub const KVM_CAP_ARM_SET_DEVICE_ADDR: u32 = 88;
pub const KVM_CAP_DEVICE_CTRL: u32 = 89;
pub const KVM_CAP_IRQ_MPIC: u32 = 90;
pub const KVM_CAP_PPC_RTAS: u32 = 91;
pub const KVM_CAP_IRQ_XICS: u32 = 92;
pub const KVM_CAP_ARM_EL1_32BIT: u32 = 93;
pub const KVM_CAP_SPAPR_MULTITCE: u32 = 94;
pub const KVM_CAP_EXT_EMUL_CPUID: u32 = 95;
pub const KVM_CAP_HYPERV_TIME: u32 = 96;
pub const KVM_CAP_IOAPIC_POLARITY_IGNORED: u32 = 97;
pub const KVM_CAP_ENABLE_CAP_VM: u32 = 98;
pub const KVM_CAP_S390_IRQCHIP: u32 = 99;
pub const KVM_CAP_IOEVENTFD_NO_LENGTH: u32 = 100;
pub const KVM_CAP_VM_ATTRIBUTES: u32 = 101;
pub const KVM_CAP_ARM_PSCI_0_2: u32 = 102;
pub const KVM_CAP_PPC_FIXUP_HCALL: u32 = 103;
pub const KVM_CAP_PPC_ENABLE_HCALL: u32 = 104;
pub const KVM_CAP_CHECK_EXTENSION_VM: u32 = 105;
pub const KVM_CAP_S390_USER_SIGP: u32 = 106;
pub const KVM_CAP_S390_VECTOR_REGISTERS: u32 = 107;
pub const KVM_CAP_S390_MEM_OP: u32 = 108;
pub const KVM_CAP_S390_USER_STSI: u32 = 109;
pub const KVM_CAP_S390_SKEYS: u32 = 110;
pub const KVM_CAP_MIPS_FPU: u32 = 111;
pub const KVM_CAP_MIPS_MSA: u32 = 112;
pub const KVM_CAP_S390_INJECT_IRQ: u32 = 113;
pub const KVM_CAP_S390_IRQ_STATE: u32 = 114;
pub const KVM_CAP_PPC_HWRNG: u32 = 115;
pub const KVM_CAP_DISABLE_QUIRKS: u32 = 116;
pub const KVM_CAP_X86_SMM: u32 = 117;
pub const KVM_CAP_MULTI_ADDRESS_SPACE: u32 = 118;
pub const KVM_CAP_GUEST_DEBUG_HW_BPS: u32 = 119;
pub const KVM_CAP_GUEST_DEBUG_HW_WPS: u32 = 120;
pub const KVM_CAP_SPLIT_IRQCHIP: u32 = 121;
pub const KVM_CAP_IOEVENTFD_ANY_LENGTH: u32 = 122;
pub const KVM_CAP_HYPERV_SYNIC: u32 = 123;
pub const KVM_CAP_S390_RI: u32 = 124;
pub const KVM_CAP_SPAPR_TCE_64: u32 = 125;
pub const KVM_CAP_ARM_PMU_V3: u32 = 126;
pub const KVM_CAP_VCPU_ATTRIBUTES: u32 = 127;
pub const KVM_CAP_MAX_VCPU_ID: u32 = 128;
pub const KVM_CAP_X2APIC_API: u32 = 129;
pub const KVM_CAP_S390_USER_INSTR0: u32 = 130;
pub const KVM_CAP_MSI_DEVID: u32 = 131;
pub const KVM_CAP_PPC_HTM: u32 = 132;
pub const KVM_CAP_SPAPR_RESIZE_HPT: u32 = 133;
pub const KVM_CAP_PPC_MMU_RADIX: u32 = 134;
pub const KVM_CAP_PPC_MMU_HASH_V3: u32 = 135;
pub const KVM_CAP_IMMEDIATE_EXIT: u32 = 136;
pub const KVM_CAP_MIPS_VZ: u32 = 137;
pub const KVM_CAP_MIPS_TE: u32 = 138;
pub const KVM_CAP_MIPS_64BIT: u32 = 139;
pub const KVM_CAP_S390_GS: u32 = 140;
pub const KVM_CAP_S390_AIS: u32 = 141;
pub const KVM_CAP_SPAPR_TCE_VFIO: u32 = 142;
pub const KVM_CAP_X86_GUEST_MWAIT: u32 = 143;
pub const KVM_CAP_ARM_USER_IRQ: u32 = 144;
pub const KVM_CAP_S390_CMMA_MIGRATION: u32 = 145;
pub const KVM_CAP_PPC_FWNMI: u32 = 146;
pub const KVM_CAP_PPC_SMT_POSSIBLE: u32 = 147;
pub const KVM_CAP_HYPERV_SYNIC2: u32 = 148;
pub const KVM_CAP_HYPERV_VP_INDEX: u32 = 149;
pub const KVM_IRQ_ROUTING_IRQCHIP: u32 = 1;
pub const KVM_IRQ_ROUTING_MSI: u32 = 2;
pub const KVM_IRQ_ROUTING_S390_ADAPTER: u32 = 3;
pub const KVM_IRQ_ROUTING_HV_SINT: u32 = 4;
pub const KVM_IRQFD_FLAG_DEASSIGN: u32 = 1;
pub const KVM_IRQFD_FLAG_RESAMPLE: u32 = 2;
pub const KVM_CLOCK_TSC_STABLE: u32 = 2;
pub const KVM_MMU_FSL_BOOKE_NOHV: u32 = 0;
pub const KVM_MMU_FSL_BOOKE_HV: u32 = 1;
pub const KVM_REG_ARCH_MASK: i64 = -72057594037927936;
pub const KVM_REG_GENERIC: u32 = 0;
pub const KVM_REG_PPC: u64 = 1152921504606846976;
pub const KVM_REG_X86: u64 = 2305843009213693952;
pub const KVM_REG_IA64: u64 = 3458764513820540928;
pub const KVM_REG_ARM: u64 = 4611686018427387904;
pub const KVM_REG_S390: u64 = 5764607523034234880;
pub const KVM_REG_ARM64: u64 = 6917529027641081856;
pub const KVM_REG_MIPS: u64 = 8070450532247928832;
pub const KVM_REG_SIZE_SHIFT: u32 = 52;
pub const KVM_REG_SIZE_MASK: u64 = 67553994410557440;
pub const KVM_REG_SIZE_U8: u32 = 0;
pub const KVM_REG_SIZE_U16: u64 = 4503599627370496;
pub const KVM_REG_SIZE_U32: u64 = 9007199254740992;
pub const KVM_REG_SIZE_U64: u64 = 13510798882111488;
pub const KVM_REG_SIZE_U128: u64 = 18014398509481984;
pub const KVM_REG_SIZE_U256: u64 = 22517998136852480;
pub const KVM_REG_SIZE_U512: u64 = 27021597764222976;
pub const KVM_REG_SIZE_U1024: u64 = 31525197391593472;
pub const KVM_MSI_VALID_DEVID: u32 = 1;
pub const KVM_CREATE_DEVICE_TEST: u32 = 1;
pub const KVM_DEV_VFIO_GROUP: u32 = 1;
pub const KVM_DEV_VFIO_GROUP_ADD: u32 = 1;
pub const KVM_DEV_VFIO_GROUP_DEL: u32 = 2;
pub const KVM_DEV_VFIO_GROUP_SET_SPAPR_TCE: u32 = 3;
pub const KVM_S390_STORE_STATUS_NOADDR: i32 = -1;
pub const KVM_S390_STORE_STATUS_PREFIXED: i32 = -2;
pub const KVM_DEV_ASSIGN_ENABLE_IOMMU: u32 = 1;
pub const KVM_DEV_ASSIGN_PCI_2_3: u32 = 2;
pub const KVM_DEV_ASSIGN_MASK_INTX: u32 = 4;
pub const KVM_DEV_IRQ_HOST_INTX: u32 = 1;
pub const KVM_DEV_IRQ_HOST_MSI: u32 = 2;
pub const KVM_DEV_IRQ_HOST_MSIX: u32 = 4;
pub const KVM_DEV_IRQ_GUEST_INTX: u32 = 256;
pub const KVM_DEV_IRQ_GUEST_MSI: u32 = 512;
pub const KVM_DEV_IRQ_GUEST_MSIX: u32 = 1024;
pub const KVM_DEV_IRQ_HOST_MASK: u32 = 255;
pub const KVM_DEV_IRQ_GUEST_MASK: u32 = 65280;
pub const KVM_MAX_MSIX_PER_DEV: u32 = 256;
pub const KVM_X2APIC_API_USE_32BIT_IDS: u32 = 1;
pub const KVM_X2APIC_API_DISABLE_BROADCAST_QUIRK: u32 = 2;
pub const KVM_ARM_DEV_EL1_VTIMER: u32 = 1;
pub const KVM_ARM_DEV_EL1_PTIMER: u32 = 2;
pub const KVM_ARM_DEV_PMU: u32 = 4;
pub type __s8 = ::std::os::raw::c_schar;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct __kernel_fd_set {
    pub fds_bits: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___kernel_fd_set() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_fd_set>())).fds_bits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fd_set),
            "::",
            stringify!(fds_bits)
        )
    );
}
pub type __kernel_sighandler_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type __kernel_key_t = ::std::os::raw::c_int;
pub type __kernel_mqd_t = ::std::os::raw::c_int;
pub type __kernel_old_uid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_gid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_dev_t = ::std::os::raw::c_ulong;
pub type __kernel_long_t = ::std::os::raw::c_long;
pub type __kernel_ulong_t = ::std::os::raw::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::std::os::raw::c_uint;
pub type __kernel_pid_t = ::std::os::raw::c_int;
pub type __kernel_ipc_pid_t = ::std::os::raw::c_int;
pub type __kernel_uid_t = ::std::os::raw::c_uint;
pub type __kernel_gid_t = ::std::os::raw::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::std::os::raw::c_int;
pub type __kernel_uid32_t = ::std::os::raw::c_uint;
pub type __kernel_gid32_t = ::std::os::raw::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct __kernel_fsid_t {
    pub val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___kernel_fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_fsid_t>())).val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fsid_t),
            "::",
            stringify!(val)
        )
    );
}
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::std::os::raw::c_longlong;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::std::os::raw::c_int;
pub type __kernel_clockid_t = ::std::os::raw::c_int;
pub type __kernel_caddr_t = *mut ::std::os::raw::c_char;
pub type __kernel_uid16_t = ::std::os::raw::c_ushort;
pub type __kernel_gid16_t = ::std::os::raw::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_memory_alias {
    pub slot: __u32,
    pub flags: __u32,
    pub guest_phys_addr: __u64,
    pub memory_size: __u64,
    pub target_phys_addr: __u64,
}
#[test]
fn bindgen_test_layout_kvm_memory_alias() {
    assert_eq!(
        ::std::mem::size_of::<kvm_memory_alias>(),
        32usize,
        concat!("Size of: ", stringify!(kvm_memory_alias))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_memory_alias>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_memory_alias))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_memory_alias>())).slot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_memory_alias),
            "::",
            stringify!(slot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_memory_alias>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_memory_alias),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_memory_alias>())).guest_phys_addr as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_memory_alias),
            "::",
            stringify!(guest_phys_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_memory_alias>())).memory_size as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_memory_alias),
            "::",
            stringify!(memory_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_memory_alias>())).target_phys_addr as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_memory_alias),
            "::",
            stringify!(target_phys_addr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_pic_state {
    pub last_irr: __u8,
    pub irr: __u8,
    pub imr: __u8,
    pub isr: __u8,
    pub priority_add: __u8,
    pub irq_base: __u8,
    pub read_reg_select: __u8,
    pub poll: __u8,
    pub special_mask: __u8,
    pub init_state: __u8,
    pub auto_eoi: __u8,
    pub rotate_on_auto_eoi: __u8,
    pub special_fully_nested_mode: __u8,
    pub init4: __u8,
    pub elcr: __u8,
    pub elcr_mask: __u8,
}
#[test]
fn bindgen_test_layout_kvm_pic_state() {
    assert_eq!(
        ::std::mem::size_of::<kvm_pic_state>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_pic_state))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_pic_state>(),
        1usize,
        concat!("Alignment of ", stringify!(kvm_pic_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).last_irr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(last_irr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).irr as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(irr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).imr as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(imr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).isr as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(isr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).priority_add as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(priority_add)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).irq_base as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(irq_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).read_reg_select as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(read_reg_select)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).poll as *const _ as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(poll)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).special_mask as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(special_mask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).init_state as *const _ as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(init_state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).auto_eoi as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(auto_eoi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pic_state>())).rotate_on_auto_eoi as *const _ as usize
        },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(rotate_on_auto_eoi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pic_state>())).special_fully_nested_mode as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(special_fully_nested_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).init4 as *const _ as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(init4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).elcr as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(elcr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pic_state>())).elcr_mask as *const _ as usize },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pic_state),
            "::",
            stringify!(elcr_mask)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ioapic_state {
    pub base_address: __u64,
    pub ioregsel: __u32,
    pub id: __u32,
    pub irr: __u32,
    pub pad: __u32,
    pub redirtbl: [kvm_ioapic_state__bindgen_ty_1; 24usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_ioapic_state__bindgen_ty_1 {
    pub bits: __u64,
    pub fields: kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1 {
    pub vector: __u8,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u8>,
    pub reserved: [__u8; 4usize],
    pub dest_id: __u8,
}
#[test]
fn bindgen_test_layout_kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1>())).vector
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(vector)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1>())).reserved
                as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(reserved)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1>())).dest_id
                as *const _ as usize
        },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(dest_id)
        )
    );
}
impl kvm_ioapic_state__bindgen_ty_1__bindgen_ty_1 {
    #[inline]
    pub fn delivery_mode(&self) -> __u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 3u8) as u8) }
    }
    #[inline]
    pub fn set_delivery_mode(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn dest_mode(&self) -> __u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_dest_mode(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn delivery_status(&self) -> __u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_delivery_status(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn polarity(&self) -> __u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_polarity(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn remote_irr(&self) -> __u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_remote_irr(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn trig_mode(&self) -> __u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_trig_mode(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn mask(&self) -> __u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_mask(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserve(&self) -> __u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 7u8) as u8) }
    }
    #[inline]
    pub fn set_reserve(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        delivery_mode: __u8,
        dest_mode: __u8,
        delivery_status: __u8,
        polarity: __u8,
        remote_irr: __u8,
        trig_mode: __u8,
        mask: __u8,
        reserve: __u8,
    ) -> __BindgenBitfieldUnit<[u8; 2usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 3u8, {
            let delivery_mode: u8 = unsafe { ::std::mem::transmute(delivery_mode) };
            delivery_mode as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let dest_mode: u8 = unsafe { ::std::mem::transmute(dest_mode) };
            dest_mode as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let delivery_status: u8 = unsafe { ::std::mem::transmute(delivery_status) };
            delivery_status as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let polarity: u8 = unsafe { ::std::mem::transmute(polarity) };
            polarity as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let remote_irr: u8 = unsafe { ::std::mem::transmute(remote_irr) };
            remote_irr as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let trig_mode: u8 = unsafe { ::std::mem::transmute(trig_mode) };
            trig_mode as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let mask: u8 = unsafe { ::std::mem::transmute(mask) };
            mask as u64
        });
        __bindgen_bitfield_unit.set(9usize, 7u8, {
            let reserve: u8 = unsafe { ::std::mem::transmute(reserve) };
            reserve as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout_kvm_ioapic_state__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_ioapic_state__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_ioapic_state__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_ioapic_state__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_ioapic_state__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_ioapic_state__bindgen_ty_1>())).bits as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state__bindgen_ty_1),
            "::",
            stringify!(bits)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_ioapic_state__bindgen_ty_1>())).fields as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state__bindgen_ty_1),
            "::",
            stringify!(fields)
        )
    );
}
impl Default for kvm_ioapic_state__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_kvm_ioapic_state() {
    assert_eq!(
        ::std::mem::size_of::<kvm_ioapic_state>(),
        216usize,
        concat!("Size of: ", stringify!(kvm_ioapic_state))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_ioapic_state>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_ioapic_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioapic_state>())).base_address as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state),
            "::",
            stringify!(base_address)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioapic_state>())).ioregsel as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state),
            "::",
            stringify!(ioregsel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioapic_state>())).id as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioapic_state>())).irr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state),
            "::",
            stringify!(irr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioapic_state>())).pad as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioapic_state>())).redirtbl as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioapic_state),
            "::",
            stringify!(redirtbl)
        )
    );
}
impl Default for kvm_ioapic_state {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_regs {
    pub rax: __u64,
    pub rbx: __u64,
    pub rcx: __u64,
    pub rdx: __u64,
    pub rsi: __u64,
    pub rdi: __u64,
    pub rsp: __u64,
    pub rbp: __u64,
    pub r8: __u64,
    pub r9: __u64,
    pub r10: __u64,
    pub r11: __u64,
    pub r12: __u64,
    pub r13: __u64,
    pub r14: __u64,
    pub r15: __u64,
    pub rip: __u64,
    pub rflags: __u64,
}
#[test]
fn bindgen_test_layout_kvm_regs() {
    assert_eq!(
        ::std::mem::size_of::<kvm_regs>(),
        144usize,
        concat!("Size of: ", stringify!(kvm_regs))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_regs>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_regs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rax as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rax)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rbx as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rbx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rcx as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rcx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rdx as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rdx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rsi as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rsi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rdi as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rdi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rsp as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rsp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rbp as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rbp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r8 as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r8)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r9 as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r9)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r10 as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r10)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r11 as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r11)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r12 as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r12)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r13 as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r13)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r14 as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r14)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r15 as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r15)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rip as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rflags as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rflags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_lapic_state {
    pub regs: [::std::os::raw::c_char; 1024usize],
}
#[test]
fn bindgen_test_layout_kvm_lapic_state() {
    assert_eq!(
        ::std::mem::size_of::<kvm_lapic_state>(),
        1024usize,
        concat!("Size of: ", stringify!(kvm_lapic_state))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_lapic_state>(),
        1usize,
        concat!("Alignment of ", stringify!(kvm_lapic_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_lapic_state>())).regs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_lapic_state),
            "::",
            stringify!(regs)
        )
    );
}
impl Default for kvm_lapic_state {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_segment {
    pub base: __u64,
    pub limit: __u32,
    pub selector: __u16,
    pub type_: __u8,
    pub present: __u8,
    pub dpl: __u8,
    pub db: __u8,
    pub s: __u8,
    pub l: __u8,
    pub g: __u8,
    pub avl: __u8,
    pub unusable: __u8,
    pub padding: __u8,
}
#[test]
fn bindgen_test_layout_kvm_segment() {
    assert_eq!(
        ::std::mem::size_of::<kvm_segment>(),
        24usize,
        concat!("Size of: ", stringify!(kvm_segment))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_segment>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_segment))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).base as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).limit as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(limit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).selector as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(selector)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).type_ as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).present as *const _ as usize },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(present)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).dpl as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(dpl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).db as *const _ as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(db)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).s as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(s)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).l as *const _ as usize },
        19usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(l)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).g as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(g)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).avl as *const _ as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(avl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).unusable as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(unusable)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).padding as *const _ as usize },
        23usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(padding)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_dtable {
    pub base: __u64,
    pub limit: __u16,
    pub padding: [__u16; 3usize],
}
#[test]
fn bindgen_test_layout_kvm_dtable() {
    assert_eq!(
        ::std::mem::size_of::<kvm_dtable>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_dtable))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_dtable>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_dtable))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_dtable>())).base as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dtable),
            "::",
            stringify!(base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_dtable>())).limit as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dtable),
            "::",
            stringify!(limit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_dtable>())).padding as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dtable),
            "::",
            stringify!(padding)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_sregs {
    pub cs: kvm_segment,
    pub ds: kvm_segment,
    pub es: kvm_segment,
    pub fs: kvm_segment,
    pub gs: kvm_segment,
    pub ss: kvm_segment,
    pub tr: kvm_segment,
    pub ldt: kvm_segment,
    pub gdt: kvm_dtable,
    pub idt: kvm_dtable,
    pub cr0: __u64,
    pub cr2: __u64,
    pub cr3: __u64,
    pub cr4: __u64,
    pub cr8: __u64,
    pub efer: __u64,
    pub apic_base: __u64,
    pub interrupt_bitmap: [__u64; 4usize],
}
#[test]
fn bindgen_test_layout_kvm_sregs() {
    assert_eq!(
        ::std::mem::size_of::<kvm_sregs>(),
        312usize,
        concat!("Size of: ", stringify!(kvm_sregs))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_sregs>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_sregs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).ds as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(ds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).es as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(es)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).fs as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(fs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).gs as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(gs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).ss as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(ss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).tr as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(tr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).ldt as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(ldt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).gdt as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(gdt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).idt as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(idt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr0 as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr2 as *const _ as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr3 as *const _ as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr4 as *const _ as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr8 as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr8)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).efer as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(efer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).apic_base as *const _ as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(apic_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).interrupt_bitmap as *const _ as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(interrupt_bitmap)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_fpu {
    pub fpr: [[__u8; 16usize]; 8usize],
    pub fcw: __u16,
    pub fsw: __u16,
    pub ftwx: __u8,
    pub pad1: __u8,
    pub last_opcode: __u16,
    pub last_ip: __u64,
    pub last_dp: __u64,
    pub xmm: [[__u8; 16usize]; 16usize],
    pub mxcsr: __u32,
    pub pad2: __u32,
}
#[test]
fn bindgen_test_layout_kvm_fpu() {
    assert_eq!(
        ::std::mem::size_of::<kvm_fpu>(),
        416usize,
        concat!("Size of: ", stringify!(kvm_fpu))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_fpu>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_fpu))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_fpu>())).fpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_fpu),
            "::",
            stringify!(fpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_fpu>())).fcw as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_fpu),
            "::",
            stringify!(fcw)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_fpu>())).fsw as *const _ as usize },
        130usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_fpu),
            "::",
            stringify!(fsw)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_fpu>())).ftwx as *const _ as usize },
        132usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_fpu),
            "::",
            stringify!(ftwx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_fpu>())).pad1 as *const _ as usize },
        133usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_fpu),
            "::",
            stringify!(pad1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_fpu>())).last_opcode as *const _ as usize },
        134usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_fpu),
            "::",
            stringify!(last_opcode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_fpu>())).last_ip as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_fpu),
            "::",
            stringify!(last_ip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_fpu>())).last_dp as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_fpu),
            "::",
            stringify!(last_dp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_fpu>())).xmm as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_fpu),
            "::",
            stringify!(xmm)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_fpu>())).mxcsr as *const _ as usize },
        408usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_fpu),
            "::",
            stringify!(mxcsr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_fpu>())).pad2 as *const _ as usize },
        412usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_fpu),
            "::",
            stringify!(pad2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_msr_entry {
    pub index: __u32,
    pub reserved: __u32,
    pub data: __u64,
}
#[test]
fn bindgen_test_layout_kvm_msr_entry() {
    assert_eq!(
        ::std::mem::size_of::<kvm_msr_entry>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_msr_entry))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_msr_entry>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_msr_entry))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msr_entry>())).index as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msr_entry),
            "::",
            stringify!(index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msr_entry>())).reserved as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msr_entry),
            "::",
            stringify!(reserved)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msr_entry>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msr_entry),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct kvm_msrs {
    pub nmsrs: __u32,
    pub pad: __u32,
    pub entries: __IncompleteArrayField<kvm_msr_entry>,
}
#[test]
fn bindgen_test_layout_kvm_msrs() {
    assert_eq!(
        ::std::mem::size_of::<kvm_msrs>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_msrs))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_msrs>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_msrs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msrs>())).nmsrs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msrs),
            "::",
            stringify!(nmsrs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msrs>())).pad as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msrs),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msrs>())).entries as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msrs),
            "::",
            stringify!(entries)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct kvm_msr_list {
    pub nmsrs: __u32,
    pub indices: __IncompleteArrayField<__u32>,
}
#[test]
fn bindgen_test_layout_kvm_msr_list() {
    assert_eq!(
        ::std::mem::size_of::<kvm_msr_list>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_msr_list))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_msr_list>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_msr_list))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msr_list>())).nmsrs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msr_list),
            "::",
            stringify!(nmsrs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msr_list>())).indices as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msr_list),
            "::",
            stringify!(indices)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_cpuid_entry {
    pub function: __u32,
    pub eax: __u32,
    pub ebx: __u32,
    pub ecx: __u32,
    pub edx: __u32,
    pub padding: __u32,
}
#[test]
fn bindgen_test_layout_kvm_cpuid_entry() {
    assert_eq!(
        ::std::mem::size_of::<kvm_cpuid_entry>(),
        24usize,
        concat!("Size of: ", stringify!(kvm_cpuid_entry))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_cpuid_entry>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_cpuid_entry))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry>())).function as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry),
            "::",
            stringify!(function)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry>())).eax as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry),
            "::",
            stringify!(eax)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry>())).ebx as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry),
            "::",
            stringify!(ebx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry>())).ecx as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry),
            "::",
            stringify!(ecx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry>())).edx as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry),
            "::",
            stringify!(edx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry>())).padding as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry),
            "::",
            stringify!(padding)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct kvm_cpuid {
    pub nent: __u32,
    pub padding: __u32,
    pub entries: __IncompleteArrayField<kvm_cpuid_entry>,
}
#[test]
fn bindgen_test_layout_kvm_cpuid() {
    assert_eq!(
        ::std::mem::size_of::<kvm_cpuid>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_cpuid))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_cpuid>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_cpuid))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid>())).nent as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid),
            "::",
            stringify!(nent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid>())).padding as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid),
            "::",
            stringify!(padding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid>())).entries as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid),
            "::",
            stringify!(entries)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_cpuid_entry2 {
    pub function: __u32,
    pub index: __u32,
    pub flags: __u32,
    pub eax: __u32,
    pub ebx: __u32,
    pub ecx: __u32,
    pub edx: __u32,
    pub padding: [__u32; 3usize],
}
#[test]
fn bindgen_test_layout_kvm_cpuid_entry2() {
    assert_eq!(
        ::std::mem::size_of::<kvm_cpuid_entry2>(),
        40usize,
        concat!("Size of: ", stringify!(kvm_cpuid_entry2))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_cpuid_entry2>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_cpuid_entry2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).function as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(function)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).index as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).flags as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).eax as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(eax)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).ebx as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(ebx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).ecx as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(ecx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).edx as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(edx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid_entry2>())).padding as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid_entry2),
            "::",
            stringify!(padding)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct kvm_cpuid2 {
    pub nent: __u32,
    pub padding: __u32,
    pub entries: __IncompleteArrayField<kvm_cpuid_entry2>,
}
#[test]
fn bindgen_test_layout_kvm_cpuid2() {
    assert_eq!(
        ::std::mem::size_of::<kvm_cpuid2>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_cpuid2))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_cpuid2>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_cpuid2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid2>())).nent as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid2),
            "::",
            stringify!(nent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid2>())).padding as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid2),
            "::",
            stringify!(padding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_cpuid2>())).entries as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_cpuid2),
            "::",
            stringify!(entries)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_pit_channel_state {
    pub count: __u32,
    pub latched_count: __u16,
    pub count_latched: __u8,
    pub status_latched: __u8,
    pub status: __u8,
    pub read_state: __u8,
    pub write_state: __u8,
    pub write_latch: __u8,
    pub rw_mode: __u8,
    pub mode: __u8,
    pub bcd: __u8,
    pub gate: __u8,
    pub count_load_time: __s64,
}
#[test]
fn bindgen_test_layout_kvm_pit_channel_state() {
    assert_eq!(
        ::std::mem::size_of::<kvm_pit_channel_state>(),
        24usize,
        concat!("Size of: ", stringify!(kvm_pit_channel_state))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_pit_channel_state>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_pit_channel_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_channel_state>())).count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).latched_count as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(latched_count)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).count_latched as *const _ as usize
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(count_latched)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).status_latched as *const _ as usize
        },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(status_latched)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_channel_state>())).status as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).read_state as *const _ as usize
        },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(read_state)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).write_state as *const _ as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(write_state)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).write_latch as *const _ as usize
        },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(write_latch)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_channel_state>())).rw_mode as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(rw_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_channel_state>())).mode as *const _ as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_channel_state>())).bcd as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(bcd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_channel_state>())).gate as *const _ as usize },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(gate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_pit_channel_state>())).count_load_time as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_channel_state),
            "::",
            stringify!(count_load_time)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_debug_exit_arch {
    pub exception: __u32,
    pub pad: __u32,
    pub pc: __u64,
    pub dr6: __u64,
    pub dr7: __u64,
}
#[test]
fn bindgen_test_layout_kvm_debug_exit_arch() {
    assert_eq!(
        ::std::mem::size_of::<kvm_debug_exit_arch>(),
        32usize,
        concat!("Size of: ", stringify!(kvm_debug_exit_arch))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_debug_exit_arch>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_debug_exit_arch))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debug_exit_arch>())).exception as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debug_exit_arch),
            "::",
            stringify!(exception)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debug_exit_arch>())).pad as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debug_exit_arch),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debug_exit_arch>())).pc as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debug_exit_arch),
            "::",
            stringify!(pc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debug_exit_arch>())).dr6 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debug_exit_arch),
            "::",
            stringify!(dr6)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debug_exit_arch>())).dr7 as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debug_exit_arch),
            "::",
            stringify!(dr7)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_guest_debug_arch {
    pub debugreg: [__u64; 8usize],
}
#[test]
fn bindgen_test_layout_kvm_guest_debug_arch() {
    assert_eq!(
        ::std::mem::size_of::<kvm_guest_debug_arch>(),
        64usize,
        concat!("Size of: ", stringify!(kvm_guest_debug_arch))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_guest_debug_arch>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_guest_debug_arch))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_guest_debug_arch>())).debugreg as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_guest_debug_arch),
            "::",
            stringify!(debugreg)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_pit_state {
    pub channels: [kvm_pit_channel_state; 3usize],
}
#[test]
fn bindgen_test_layout_kvm_pit_state() {
    assert_eq!(
        ::std::mem::size_of::<kvm_pit_state>(),
        72usize,
        concat!("Size of: ", stringify!(kvm_pit_state))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_pit_state>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_pit_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_state>())).channels as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_state),
            "::",
            stringify!(channels)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_pit_state2 {
    pub channels: [kvm_pit_channel_state; 3usize],
    pub flags: __u32,
    pub reserved: [__u32; 9usize],
}
#[test]
fn bindgen_test_layout_kvm_pit_state2() {
    assert_eq!(
        ::std::mem::size_of::<kvm_pit_state2>(),
        112usize,
        concat!("Size of: ", stringify!(kvm_pit_state2))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_pit_state2>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_pit_state2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_state2>())).channels as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_state2),
            "::",
            stringify!(channels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_state2>())).flags as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_state2),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_state2>())).reserved as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_state2),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_reinject_control {
    pub pit_reinject: __u8,
    pub reserved: [__u8; 31usize],
}
#[test]
fn bindgen_test_layout_kvm_reinject_control() {
    assert_eq!(
        ::std::mem::size_of::<kvm_reinject_control>(),
        32usize,
        concat!("Size of: ", stringify!(kvm_reinject_control))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_reinject_control>(),
        1usize,
        concat!("Alignment of ", stringify!(kvm_reinject_control))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_reinject_control>())).pit_reinject as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_reinject_control),
            "::",
            stringify!(pit_reinject)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_reinject_control>())).reserved as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_reinject_control),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_vcpu_events {
    pub exception: kvm_vcpu_events__bindgen_ty_1,
    pub interrupt: kvm_vcpu_events__bindgen_ty_2,
    pub nmi: kvm_vcpu_events__bindgen_ty_3,
    pub sipi_vector: __u32,
    pub flags: __u32,
    pub smi: kvm_vcpu_events__bindgen_ty_4,
    pub reserved: [__u32; 9usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_vcpu_events__bindgen_ty_1 {
    pub injected: __u8,
    pub nr: __u8,
    pub has_error_code: __u8,
    pub pad: __u8,
    pub error_code: __u32,
}
#[test]
fn bindgen_test_layout_kvm_vcpu_events__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_vcpu_events__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_vcpu_events__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_vcpu_events__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_vcpu_events__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_1>())).injected as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_1),
            "::",
            stringify!(injected)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_1>())).nr as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_1),
            "::",
            stringify!(nr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_1>())).has_error_code as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_1),
            "::",
            stringify!(has_error_code)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_1>())).pad as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_1),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_1>())).error_code as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_1),
            "::",
            stringify!(error_code)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_vcpu_events__bindgen_ty_2 {
    pub injected: __u8,
    pub nr: __u8,
    pub soft: __u8,
    pub shadow: __u8,
}
#[test]
fn bindgen_test_layout_kvm_vcpu_events__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<kvm_vcpu_events__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_vcpu_events__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_vcpu_events__bindgen_ty_2>(),
        1usize,
        concat!("Alignment of ", stringify!(kvm_vcpu_events__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_2>())).injected as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_2),
            "::",
            stringify!(injected)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_2>())).nr as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_2),
            "::",
            stringify!(nr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_2>())).soft as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_2),
            "::",
            stringify!(soft)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_2>())).shadow as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_2),
            "::",
            stringify!(shadow)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_vcpu_events__bindgen_ty_3 {
    pub injected: __u8,
    pub pending: __u8,
    pub masked: __u8,
    pub pad: __u8,
}
#[test]
fn bindgen_test_layout_kvm_vcpu_events__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<kvm_vcpu_events__bindgen_ty_3>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_vcpu_events__bindgen_ty_3))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_vcpu_events__bindgen_ty_3>(),
        1usize,
        concat!("Alignment of ", stringify!(kvm_vcpu_events__bindgen_ty_3))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_3>())).injected as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_3),
            "::",
            stringify!(injected)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_3>())).pending as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_3),
            "::",
            stringify!(pending)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_3>())).masked as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_3),
            "::",
            stringify!(masked)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_3>())).pad as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_3),
            "::",
            stringify!(pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_vcpu_events__bindgen_ty_4 {
    pub smm: __u8,
    pub pending: __u8,
    pub smm_inside_nmi: __u8,
    pub latched_init: __u8,
}
#[test]
fn bindgen_test_layout_kvm_vcpu_events__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<kvm_vcpu_events__bindgen_ty_4>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_vcpu_events__bindgen_ty_4))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_vcpu_events__bindgen_ty_4>(),
        1usize,
        concat!("Alignment of ", stringify!(kvm_vcpu_events__bindgen_ty_4))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_4>())).smm as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_4),
            "::",
            stringify!(smm)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_4>())).pending as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_4),
            "::",
            stringify!(pending)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_4>())).smm_inside_nmi as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_4),
            "::",
            stringify!(smm_inside_nmi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_vcpu_events__bindgen_ty_4>())).latched_init as *const _
                as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events__bindgen_ty_4),
            "::",
            stringify!(latched_init)
        )
    );
}
#[test]
fn bindgen_test_layout_kvm_vcpu_events() {
    assert_eq!(
        ::std::mem::size_of::<kvm_vcpu_events>(),
        64usize,
        concat!("Size of: ", stringify!(kvm_vcpu_events))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_vcpu_events>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_vcpu_events))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).exception as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(exception)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).interrupt as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(interrupt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).nmi as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(nmi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).sipi_vector as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(sipi_vector)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).flags as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).smi as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(smi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vcpu_events>())).reserved as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vcpu_events),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_debugregs {
    pub db: [__u64; 4usize],
    pub dr6: __u64,
    pub dr7: __u64,
    pub flags: __u64,
    pub reserved: [__u64; 9usize],
}
#[test]
fn bindgen_test_layout_kvm_debugregs() {
    assert_eq!(
        ::std::mem::size_of::<kvm_debugregs>(),
        128usize,
        concat!("Size of: ", stringify!(kvm_debugregs))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_debugregs>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_debugregs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debugregs>())).db as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debugregs),
            "::",
            stringify!(db)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debugregs>())).dr6 as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debugregs),
            "::",
            stringify!(dr6)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debugregs>())).dr7 as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debugregs),
            "::",
            stringify!(dr7)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debugregs>())).flags as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debugregs),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debugregs>())).reserved as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debugregs),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_xsave {
    pub region: [__u32; 1024usize],
}
#[test]
fn bindgen_test_layout_kvm_xsave() {
    assert_eq!(
        ::std::mem::size_of::<kvm_xsave>(),
        4096usize,
        concat!("Size of: ", stringify!(kvm_xsave))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_xsave>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_xsave))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xsave>())).region as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xsave),
            "::",
            stringify!(region)
        )
    );
}
impl Default for kvm_xsave {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_xcr {
    pub xcr: __u32,
    pub reserved: __u32,
    pub value: __u64,
}
#[test]
fn bindgen_test_layout_kvm_xcr() {
    assert_eq!(
        ::std::mem::size_of::<kvm_xcr>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_xcr))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_xcr>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_xcr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcr>())).xcr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcr),
            "::",
            stringify!(xcr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcr>())).reserved as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcr),
            "::",
            stringify!(reserved)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcr>())).value as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcr),
            "::",
            stringify!(value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_xcrs {
    pub nr_xcrs: __u32,
    pub flags: __u32,
    pub xcrs: [kvm_xcr; 16usize],
    pub padding: [__u64; 16usize],
}
#[test]
fn bindgen_test_layout_kvm_xcrs() {
    assert_eq!(
        ::std::mem::size_of::<kvm_xcrs>(),
        392usize,
        concat!("Size of: ", stringify!(kvm_xcrs))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_xcrs>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_xcrs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcrs>())).nr_xcrs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcrs),
            "::",
            stringify!(nr_xcrs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcrs>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcrs),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcrs>())).xcrs as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcrs),
            "::",
            stringify!(xcrs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xcrs>())).padding as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xcrs),
            "::",
            stringify!(padding)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_sync_regs {}
#[test]
fn bindgen_test_layout_kvm_sync_regs() {
    assert_eq!(
        ::std::mem::size_of::<kvm_sync_regs>(),
        0usize,
        concat!("Size of: ", stringify!(kvm_sync_regs))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_sync_regs>(),
        1usize,
        concat!("Alignment of ", stringify!(kvm_sync_regs))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_user_trace_setup {
    pub buf_size: __u32,
    pub buf_nr: __u32,
}
#[test]
fn bindgen_test_layout_kvm_user_trace_setup() {
    assert_eq!(
        ::std::mem::size_of::<kvm_user_trace_setup>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_user_trace_setup))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_user_trace_setup>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_user_trace_setup))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_user_trace_setup>())).buf_size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_user_trace_setup),
            "::",
            stringify!(buf_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_user_trace_setup>())).buf_nr as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_user_trace_setup),
            "::",
            stringify!(buf_nr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_breakpoint {
    pub enabled: __u32,
    pub padding: __u32,
    pub address: __u64,
}
#[test]
fn bindgen_test_layout_kvm_breakpoint() {
    assert_eq!(
        ::std::mem::size_of::<kvm_breakpoint>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_breakpoint))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_breakpoint>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_breakpoint))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_breakpoint>())).enabled as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_breakpoint),
            "::",
            stringify!(enabled)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_breakpoint>())).padding as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_breakpoint),
            "::",
            stringify!(padding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_breakpoint>())).address as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_breakpoint),
            "::",
            stringify!(address)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_debug_guest {
    pub enabled: __u32,
    pub pad: __u32,
    pub breakpoints: [kvm_breakpoint; 4usize],
    pub singlestep: __u32,
}
#[test]
fn bindgen_test_layout_kvm_debug_guest() {
    assert_eq!(
        ::std::mem::size_of::<kvm_debug_guest>(),
        80usize,
        concat!("Size of: ", stringify!(kvm_debug_guest))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_debug_guest>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_debug_guest))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debug_guest>())).enabled as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debug_guest),
            "::",
            stringify!(enabled)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debug_guest>())).pad as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debug_guest),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debug_guest>())).breakpoints as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debug_guest),
            "::",
            stringify!(breakpoints)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_debug_guest>())).singlestep as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_debug_guest),
            "::",
            stringify!(singlestep)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_memory_region {
    pub slot: __u32,
    pub flags: __u32,
    pub guest_phys_addr: __u64,
    pub memory_size: __u64,
}
#[test]
fn bindgen_test_layout_kvm_memory_region() {
    assert_eq!(
        ::std::mem::size_of::<kvm_memory_region>(),
        24usize,
        concat!("Size of: ", stringify!(kvm_memory_region))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_memory_region>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_memory_region))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_memory_region>())).slot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_memory_region),
            "::",
            stringify!(slot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_memory_region>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_memory_region),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_memory_region>())).guest_phys_addr as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_memory_region),
            "::",
            stringify!(guest_phys_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_memory_region>())).memory_size as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_memory_region),
            "::",
            stringify!(memory_size)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_userspace_memory_region {
    pub slot: __u32,
    pub flags: __u32,
    pub guest_phys_addr: __u64,
    pub memory_size: __u64,
    pub userspace_addr: __u64,
}
#[test]
fn bindgen_test_layout_kvm_userspace_memory_region() {
    assert_eq!(
        ::std::mem::size_of::<kvm_userspace_memory_region>(),
        32usize,
        concat!("Size of: ", stringify!(kvm_userspace_memory_region))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_userspace_memory_region>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_userspace_memory_region))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_userspace_memory_region>())).slot as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_userspace_memory_region),
            "::",
            stringify!(slot)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_userspace_memory_region>())).flags as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_userspace_memory_region),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_userspace_memory_region>())).guest_phys_addr as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_userspace_memory_region),
            "::",
            stringify!(guest_phys_addr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_userspace_memory_region>())).memory_size as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_userspace_memory_region),
            "::",
            stringify!(memory_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_userspace_memory_region>())).userspace_addr as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_userspace_memory_region),
            "::",
            stringify!(userspace_addr)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct kvm_irq_level {
    pub __bindgen_anon_1: kvm_irq_level__bindgen_ty_1,
    pub level: __u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_irq_level__bindgen_ty_1 {
    pub irq: __u32,
    pub status: __s32,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_kvm_irq_level__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irq_level__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_irq_level__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irq_level__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_irq_level__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_level__bindgen_ty_1>())).irq as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_level__bindgen_ty_1),
            "::",
            stringify!(irq)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irq_level__bindgen_ty_1>())).status as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_level__bindgen_ty_1),
            "::",
            stringify!(status)
        )
    );
}
impl Default for kvm_irq_level__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_kvm_irq_level() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irq_level>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_irq_level))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irq_level>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_irq_level))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_level>())).level as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_level),
            "::",
            stringify!(level)
        )
    );
}
impl Default for kvm_irq_level {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct kvm_irqchip {
    pub chip_id: __u32,
    pub pad: __u32,
    pub chip: kvm_irqchip__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_irqchip__bindgen_ty_1 {
    pub dummy: [::std::os::raw::c_char; 512usize],
    pub pic: kvm_pic_state,
    pub ioapic: kvm_ioapic_state,
    _bindgen_union_align: [u64; 64usize],
}
#[test]
fn bindgen_test_layout_kvm_irqchip__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irqchip__bindgen_ty_1>(),
        512usize,
        concat!("Size of: ", stringify!(kvm_irqchip__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irqchip__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_irqchip__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqchip__bindgen_ty_1>())).dummy as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqchip__bindgen_ty_1),
            "::",
            stringify!(dummy)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqchip__bindgen_ty_1>())).pic as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqchip__bindgen_ty_1),
            "::",
            stringify!(pic)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irqchip__bindgen_ty_1>())).ioapic as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqchip__bindgen_ty_1),
            "::",
            stringify!(ioapic)
        )
    );
}
impl Default for kvm_irqchip__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_kvm_irqchip() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irqchip>(),
        520usize,
        concat!("Size of: ", stringify!(kvm_irqchip))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irqchip>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_irqchip))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqchip>())).chip_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqchip),
            "::",
            stringify!(chip_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqchip>())).pad as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqchip),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqchip>())).chip as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqchip),
            "::",
            stringify!(chip)
        )
    );
}
impl Default for kvm_irqchip {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_pit_config {
    pub flags: __u32,
    pub pad: [__u32; 15usize],
}
#[test]
fn bindgen_test_layout_kvm_pit_config() {
    assert_eq!(
        ::std::mem::size_of::<kvm_pit_config>(),
        64usize,
        concat!("Size of: ", stringify!(kvm_pit_config))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_pit_config>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_pit_config))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_config>())).flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_config),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_pit_config>())).pad as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_pit_config),
            "::",
            stringify!(pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_skeys {
    pub start_gfn: __u64,
    pub count: __u64,
    pub skeydata_addr: __u64,
    pub flags: __u32,
    pub reserved: [__u32; 9usize],
}
#[test]
fn bindgen_test_layout_kvm_s390_skeys() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_skeys>(),
        64usize,
        concat!("Size of: ", stringify!(kvm_s390_skeys))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_skeys>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_s390_skeys))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_skeys>())).start_gfn as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_skeys),
            "::",
            stringify!(start_gfn)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_skeys>())).count as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_skeys),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_skeys>())).skeydata_addr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_skeys),
            "::",
            stringify!(skeydata_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_skeys>())).flags as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_skeys),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_skeys>())).reserved as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_skeys),
            "::",
            stringify!(reserved)
        )
    );
}
#[doc = " kvm_s390_cmma_log - Used for CMMA migration."]
#[doc = ""]
#[doc = " Used both for input and output."]
#[doc = ""]
#[doc = " @start_gfn: Guest page number to start from."]
#[doc = " @count: Size of the result buffer."]
#[doc = " @flags: Control operation mode via KVM_S390_CMMA_* flags"]
#[doc = " @remaining: Used with KVM_S390_GET_CMMA_BITS. Indicates how many dirty"]
#[doc = "             pages are still remaining."]
#[doc = " @mask: Used with KVM_S390_SET_CMMA_BITS. Bitmap of bits to actually set"]
#[doc = "        in the PGSTE."]
#[doc = " @values: Pointer to the values buffer."]
#[doc = ""]
#[doc = " Used in KVM_S390_{G,S}ET_CMMA_BITS ioctls."]
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct kvm_s390_cmma_log {
    pub start_gfn: __u64,
    pub count: __u32,
    pub flags: __u32,
    pub __bindgen_anon_1: kvm_s390_cmma_log__bindgen_ty_1,
    pub values: __u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_s390_cmma_log__bindgen_ty_1 {
    pub remaining: __u64,
    pub mask: __u64,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_kvm_s390_cmma_log__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_cmma_log__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_s390_cmma_log__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_cmma_log__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_s390_cmma_log__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_s390_cmma_log__bindgen_ty_1>())).remaining as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_cmma_log__bindgen_ty_1),
            "::",
            stringify!(remaining)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_s390_cmma_log__bindgen_ty_1>())).mask as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_cmma_log__bindgen_ty_1),
            "::",
            stringify!(mask)
        )
    );
}
impl Default for kvm_s390_cmma_log__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_kvm_s390_cmma_log() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_cmma_log>(),
        32usize,
        concat!("Size of: ", stringify!(kvm_s390_cmma_log))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_cmma_log>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_s390_cmma_log))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_cmma_log>())).start_gfn as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_cmma_log),
            "::",
            stringify!(start_gfn)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_cmma_log>())).count as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_cmma_log),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_cmma_log>())).flags as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_cmma_log),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_cmma_log>())).values as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_cmma_log),
            "::",
            stringify!(values)
        )
    );
}
impl Default for kvm_s390_cmma_log {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct kvm_hyperv_exit {
    pub type_: __u32,
    pub u: kvm_hyperv_exit__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_hyperv_exit__bindgen_ty_1 {
    pub synic: kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1,
    pub hcall: kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2,
    _bindgen_union_align: [u64; 4usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1 {
    pub msr: __u32,
    pub control: __u64,
    pub evt_page: __u64,
    pub msg_page: __u64,
}
#[test]
fn bindgen_test_layout_kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1>(),
        32usize,
        concat!(
            "Size of: ",
            stringify!(kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1>())).msr as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(msr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1>())).control
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(control)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1>())).evt_page
                as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(evt_page)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1>())).msg_page
                as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(msg_page)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2 {
    pub input: __u64,
    pub result: __u64,
    pub params: [__u64; 2usize],
}
#[test]
fn bindgen_test_layout_kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2>(),
        32usize,
        concat!(
            "Size of: ",
            stringify!(kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2>())).input
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(input)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2>())).result
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(result)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2>())).params
                as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(params)
        )
    );
}
#[test]
fn bindgen_test_layout_kvm_hyperv_exit__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_hyperv_exit__bindgen_ty_1>(),
        32usize,
        concat!("Size of: ", stringify!(kvm_hyperv_exit__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_hyperv_exit__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_hyperv_exit__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_hyperv_exit__bindgen_ty_1>())).synic as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_hyperv_exit__bindgen_ty_1),
            "::",
            stringify!(synic)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_hyperv_exit__bindgen_ty_1>())).hcall as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_hyperv_exit__bindgen_ty_1),
            "::",
            stringify!(hcall)
        )
    );
}
impl Default for kvm_hyperv_exit__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_kvm_hyperv_exit() {
    assert_eq!(
        ::std::mem::size_of::<kvm_hyperv_exit>(),
        40usize,
        concat!("Size of: ", stringify!(kvm_hyperv_exit))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_hyperv_exit>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_hyperv_exit))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_hyperv_exit>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_hyperv_exit),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_hyperv_exit>())).u as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_hyperv_exit),
            "::",
            stringify!(u)
        )
    );
}
impl Default for kvm_hyperv_exit {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct kvm_run {
    pub request_interrupt_window: __u8,
    pub immediate_exit: __u8,
    pub padding1: [__u8; 6usize],
    pub exit_reason: __u32,
    pub ready_for_interrupt_injection: __u8,
    pub if_flag: __u8,
    pub flags: __u16,
    pub cr8: __u64,
    pub apic_base: __u64,
    pub __bindgen_anon_1: kvm_run__bindgen_ty_1,
    pub kvm_valid_regs: __u64,
    pub kvm_dirty_regs: __u64,
    pub s: kvm_run__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_run__bindgen_ty_1 {
    pub hw: kvm_run__bindgen_ty_1__bindgen_ty_1,
    pub fail_entry: kvm_run__bindgen_ty_1__bindgen_ty_2,
    pub ex: kvm_run__bindgen_ty_1__bindgen_ty_3,
    pub io: kvm_run__bindgen_ty_1__bindgen_ty_4,
    pub debug: kvm_run__bindgen_ty_1__bindgen_ty_5,
    pub mmio: kvm_run__bindgen_ty_1__bindgen_ty_6,
    pub hypercall: kvm_run__bindgen_ty_1__bindgen_ty_7,
    pub tpr_access: kvm_run__bindgen_ty_1__bindgen_ty_8,
    pub s390_sieic: kvm_run__bindgen_ty_1__bindgen_ty_9,
    pub s390_reset_flags: __u64,
    pub s390_ucontrol: kvm_run__bindgen_ty_1__bindgen_ty_10,
    pub dcr: kvm_run__bindgen_ty_1__bindgen_ty_11,
    pub internal: kvm_run__bindgen_ty_1__bindgen_ty_12,
    pub osi: kvm_run__bindgen_ty_1__bindgen_ty_13,
    pub papr_hcall: kvm_run__bindgen_ty_1__bindgen_ty_14,
    pub s390_tsch: kvm_run__bindgen_ty_1__bindgen_ty_15,
    pub epr: kvm_run__bindgen_ty_1__bindgen_ty_16,
    pub system_event: kvm_run__bindgen_ty_1__bindgen_ty_17,
    pub s390_stsi: kvm_run__bindgen_ty_1__bindgen_ty_18,
    pub eoi: kvm_run__bindgen_ty_1__bindgen_ty_19,
    pub hyperv: kvm_hyperv_exit,
    pub padding: [::std::os::raw::c_char; 256usize],
    _bindgen_union_align: [u64; 32usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_1 {
    pub hardware_exit_reason: __u64,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_run__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_1>())).hardware_exit_reason
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(hardware_exit_reason)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_2 {
    pub hardware_entry_failure_reason: __u64,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_run__bindgen_ty_1__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_2>()))
                .hardware_entry_failure_reason as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(hardware_entry_failure_reason)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_3 {
    pub exception: __u32,
    pub error_code: __u32,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_3>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_run__bindgen_ty_1__bindgen_ty_3))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_3>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_3>())).exception as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(exception)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_3>())).error_code as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(error_code)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_4 {
    pub direction: __u8,
    pub size: __u8,
    pub port: __u16,
    pub count: __u32,
    pub data_offset: __u64,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_4>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_run__bindgen_ty_1__bindgen_ty_4))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_4>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_4>())).direction as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(direction)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_4>())).size as *const _
                as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_4>())).port as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(port)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_4>())).count as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_4>())).data_offset as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(data_offset)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_5 {
    pub arch: kvm_debug_exit_arch,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_5() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_5>(),
        32usize,
        concat!("Size of: ", stringify!(kvm_run__bindgen_ty_1__bindgen_ty_5))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_5>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_5>())).arch as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_5),
            "::",
            stringify!(arch)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_6 {
    pub phys_addr: __u64,
    pub data: [__u8; 8usize],
    pub len: __u32,
    pub is_write: __u8,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_6() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_6>(),
        24usize,
        concat!("Size of: ", stringify!(kvm_run__bindgen_ty_1__bindgen_ty_6))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_6>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_6>())).phys_addr as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(phys_addr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_6>())).data as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_6>())).len as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_6>())).is_write as *const _
                as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(is_write)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_7 {
    pub nr: __u64,
    pub args: [__u64; 6usize],
    pub ret: __u64,
    pub longmode: __u32,
    pub pad: __u32,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_7() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_7>(),
        72usize,
        concat!("Size of: ", stringify!(kvm_run__bindgen_ty_1__bindgen_ty_7))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_7>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_7>())).nr as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_7),
            "::",
            stringify!(nr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_7>())).args as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_7),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_7>())).ret as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_7),
            "::",
            stringify!(ret)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_7>())).longmode as *const _
                as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_7),
            "::",
            stringify!(longmode)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_7>())).pad as *const _ as usize
        },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_7),
            "::",
            stringify!(pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_8 {
    pub rip: __u64,
    pub is_write: __u32,
    pub pad: __u32,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_8() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_8>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_run__bindgen_ty_1__bindgen_ty_8))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_8>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_8>())).rip as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_8),
            "::",
            stringify!(rip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_8>())).is_write as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_8),
            "::",
            stringify!(is_write)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_8>())).pad as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_8),
            "::",
            stringify!(pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_9 {
    pub icptcode: __u8,
    pub ipa: __u16,
    pub ipb: __u32,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_9() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_9>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_run__bindgen_ty_1__bindgen_ty_9))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_9>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_9)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_9>())).icptcode as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_9),
            "::",
            stringify!(icptcode)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_9>())).ipa as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_9),
            "::",
            stringify!(ipa)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_9>())).ipb as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_9),
            "::",
            stringify!(ipb)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_10 {
    pub trans_exc_code: __u64,
    pub pgm_code: __u32,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_10() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_10>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_10)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_10>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_10)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_10>())).trans_exc_code
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_10),
            "::",
            stringify!(trans_exc_code)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_10>())).pgm_code as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_10),
            "::",
            stringify!(pgm_code)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_11 {
    pub dcrn: __u32,
    pub data: __u32,
    pub is_write: __u8,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_11() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_11>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_11)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_11>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_11)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_11>())).dcrn as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_11),
            "::",
            stringify!(dcrn)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_11>())).data as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_11),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_11>())).is_write as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_11),
            "::",
            stringify!(is_write)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_12 {
    pub suberror: __u32,
    pub ndata: __u32,
    pub data: [__u64; 16usize],
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_12() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_12>(),
        136usize,
        concat!(
            "Size of: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_12)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_12>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_12)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_12>())).suberror as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_12),
            "::",
            stringify!(suberror)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_12>())).ndata as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_12),
            "::",
            stringify!(ndata)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_12>())).data as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_12),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_13 {
    pub gprs: [__u64; 32usize],
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_13() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_13>(),
        256usize,
        concat!(
            "Size of: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_13)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_13>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_13)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_13>())).gprs as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_13),
            "::",
            stringify!(gprs)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_14 {
    pub nr: __u64,
    pub ret: __u64,
    pub args: [__u64; 9usize],
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_14() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_14>(),
        88usize,
        concat!(
            "Size of: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_14)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_14>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_14)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_14>())).nr as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_14),
            "::",
            stringify!(nr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_14>())).ret as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_14),
            "::",
            stringify!(ret)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_14>())).args as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_14),
            "::",
            stringify!(args)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_15 {
    pub subchannel_id: __u16,
    pub subchannel_nr: __u16,
    pub io_int_parm: __u32,
    pub io_int_word: __u32,
    pub ipb: __u32,
    pub dequeued: __u8,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_15() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_15>(),
        20usize,
        concat!(
            "Size of: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_15)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_15>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_15)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_15>())).subchannel_id
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_15),
            "::",
            stringify!(subchannel_id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_15>())).subchannel_nr
                as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_15),
            "::",
            stringify!(subchannel_nr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_15>())).io_int_parm as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_15),
            "::",
            stringify!(io_int_parm)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_15>())).io_int_word as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_15),
            "::",
            stringify!(io_int_word)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_15>())).ipb as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_15),
            "::",
            stringify!(ipb)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_15>())).dequeued as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_15),
            "::",
            stringify!(dequeued)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_16 {
    pub epr: __u32,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_16() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_16>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_16)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_16>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_16)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_16>())).epr as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_16),
            "::",
            stringify!(epr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_17 {
    pub type_: __u32,
    pub flags: __u64,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_17() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_17>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_17)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_17>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_17)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_17>())).type_ as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_17),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_17>())).flags as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_17),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_18 {
    pub addr: __u64,
    pub ar: __u8,
    pub reserved: __u8,
    pub fc: __u8,
    pub sel1: __u8,
    pub sel2: __u16,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_18() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_18>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_18)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_18>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_18)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_18>())).addr as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_18),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_18>())).ar as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_18),
            "::",
            stringify!(ar)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_18>())).reserved as *const _
                as usize
        },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_18),
            "::",
            stringify!(reserved)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_18>())).fc as *const _ as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_18),
            "::",
            stringify!(fc)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_18>())).sel1 as *const _
                as usize
        },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_18),
            "::",
            stringify!(sel1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_18>())).sel2 as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_18),
            "::",
            stringify!(sel2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_run__bindgen_ty_1__bindgen_ty_19 {
    pub vector: __u8,
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1__bindgen_ty_19() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1__bindgen_ty_19>(),
        1usize,
        concat!(
            "Size of: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_19)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1__bindgen_ty_19>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_19)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1__bindgen_ty_19>())).vector as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1__bindgen_ty_19),
            "::",
            stringify!(vector)
        )
    );
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_1>(),
        256usize,
        concat!("Size of: ", stringify!(kvm_run__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_run__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).hw as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(hw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).fail_entry as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(fail_entry)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).ex as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(ex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).io as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(io)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).debug as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(debug)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).mmio as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(mmio)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).hypercall as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(hypercall)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).tpr_access as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(tpr_access)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).s390_sieic as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(s390_sieic)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).s390_reset_flags as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(s390_reset_flags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).s390_ucontrol as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(s390_ucontrol)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).dcr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(dcr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).internal as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(internal)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).osi as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(osi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).papr_hcall as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(papr_hcall)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).s390_tsch as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(s390_tsch)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).epr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(epr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).system_event as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(system_event)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).s390_stsi as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(s390_stsi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).eoi as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(eoi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).hyperv as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(hyperv)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_1>())).padding as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_1),
            "::",
            stringify!(padding)
        )
    );
}
impl Default for kvm_run__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_run__bindgen_ty_2 {
    pub regs: kvm_sync_regs,
    pub padding: [::std::os::raw::c_char; 2048usize],
    _bindgen_union_align: [u8; 2048usize],
}
#[test]
fn bindgen_test_layout_kvm_run__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run__bindgen_ty_2>(),
        2048usize,
        concat!("Size of: ", stringify!(kvm_run__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run__bindgen_ty_2>(),
        1usize,
        concat!("Alignment of ", stringify!(kvm_run__bindgen_ty_2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_2>())).regs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_2),
            "::",
            stringify!(regs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run__bindgen_ty_2>())).padding as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run__bindgen_ty_2),
            "::",
            stringify!(padding)
        )
    );
}
impl Default for kvm_run__bindgen_ty_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_kvm_run() {
    assert_eq!(
        ::std::mem::size_of::<kvm_run>(),
        2352usize,
        concat!("Size of: ", stringify!(kvm_run))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_run>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_run))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run>())).request_interrupt_window as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run),
            "::",
            stringify!(request_interrupt_window)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run>())).immediate_exit as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run),
            "::",
            stringify!(immediate_exit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run>())).padding1 as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run),
            "::",
            stringify!(padding1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run>())).exit_reason as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run),
            "::",
            stringify!(exit_reason)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_run>())).ready_for_interrupt_injection as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run),
            "::",
            stringify!(ready_for_interrupt_injection)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run>())).if_flag as *const _ as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run),
            "::",
            stringify!(if_flag)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run>())).flags as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run>())).cr8 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run),
            "::",
            stringify!(cr8)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run>())).apic_base as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run),
            "::",
            stringify!(apic_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run>())).kvm_valid_regs as *const _ as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run),
            "::",
            stringify!(kvm_valid_regs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run>())).kvm_dirty_regs as *const _ as usize },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run),
            "::",
            stringify!(kvm_dirty_regs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_run>())).s as *const _ as usize },
        304usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_run),
            "::",
            stringify!(s)
        )
    );
}
impl Default for kvm_run {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_coalesced_mmio_zone {
    pub addr: __u64,
    pub size: __u32,
    pub pad: __u32,
}
#[test]
fn bindgen_test_layout_kvm_coalesced_mmio_zone() {
    assert_eq!(
        ::std::mem::size_of::<kvm_coalesced_mmio_zone>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_coalesced_mmio_zone))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_coalesced_mmio_zone>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_coalesced_mmio_zone))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_coalesced_mmio_zone>())).addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_coalesced_mmio_zone),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_coalesced_mmio_zone>())).size as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_coalesced_mmio_zone),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_coalesced_mmio_zone>())).pad as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_coalesced_mmio_zone),
            "::",
            stringify!(pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_coalesced_mmio {
    pub phys_addr: __u64,
    pub len: __u32,
    pub pad: __u32,
    pub data: [__u8; 8usize],
}
#[test]
fn bindgen_test_layout_kvm_coalesced_mmio() {
    assert_eq!(
        ::std::mem::size_of::<kvm_coalesced_mmio>(),
        24usize,
        concat!("Size of: ", stringify!(kvm_coalesced_mmio))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_coalesced_mmio>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_coalesced_mmio))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_coalesced_mmio>())).phys_addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_coalesced_mmio),
            "::",
            stringify!(phys_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_coalesced_mmio>())).len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_coalesced_mmio),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_coalesced_mmio>())).pad as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_coalesced_mmio),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_coalesced_mmio>())).data as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_coalesced_mmio),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct kvm_coalesced_mmio_ring {
    pub first: __u32,
    pub last: __u32,
    pub coalesced_mmio: __IncompleteArrayField<kvm_coalesced_mmio>,
}
#[test]
fn bindgen_test_layout_kvm_coalesced_mmio_ring() {
    assert_eq!(
        ::std::mem::size_of::<kvm_coalesced_mmio_ring>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_coalesced_mmio_ring))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_coalesced_mmio_ring>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_coalesced_mmio_ring))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_coalesced_mmio_ring>())).first as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_coalesced_mmio_ring),
            "::",
            stringify!(first)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_coalesced_mmio_ring>())).last as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_coalesced_mmio_ring),
            "::",
            stringify!(last)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_coalesced_mmio_ring>())).coalesced_mmio as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_coalesced_mmio_ring),
            "::",
            stringify!(coalesced_mmio)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_translation {
    pub linear_address: __u64,
    pub physical_address: __u64,
    pub valid: __u8,
    pub writeable: __u8,
    pub usermode: __u8,
    pub pad: [__u8; 5usize],
}
#[test]
fn bindgen_test_layout_kvm_translation() {
    assert_eq!(
        ::std::mem::size_of::<kvm_translation>(),
        24usize,
        concat!("Size of: ", stringify!(kvm_translation))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_translation>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_translation))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_translation>())).linear_address as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_translation),
            "::",
            stringify!(linear_address)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_translation>())).physical_address as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_translation),
            "::",
            stringify!(physical_address)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_translation>())).valid as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_translation),
            "::",
            stringify!(valid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_translation>())).writeable as *const _ as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_translation),
            "::",
            stringify!(writeable)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_translation>())).usermode as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_translation),
            "::",
            stringify!(usermode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_translation>())).pad as *const _ as usize },
        19usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_translation),
            "::",
            stringify!(pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_mem_op {
    pub gaddr: __u64,
    pub flags: __u64,
    pub size: __u32,
    pub op: __u32,
    pub buf: __u64,
    pub ar: __u8,
    pub reserved: [__u8; 31usize],
}
#[test]
fn bindgen_test_layout_kvm_s390_mem_op() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_mem_op>(),
        64usize,
        concat!("Size of: ", stringify!(kvm_s390_mem_op))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_mem_op>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_s390_mem_op))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_mem_op>())).gaddr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_mem_op),
            "::",
            stringify!(gaddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_mem_op>())).flags as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_mem_op),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_mem_op>())).size as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_mem_op),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_mem_op>())).op as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_mem_op),
            "::",
            stringify!(op)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_mem_op>())).buf as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_mem_op),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_mem_op>())).ar as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_mem_op),
            "::",
            stringify!(ar)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_mem_op>())).reserved as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_mem_op),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_interrupt {
    pub irq: __u32,
}
#[test]
fn bindgen_test_layout_kvm_interrupt() {
    assert_eq!(
        ::std::mem::size_of::<kvm_interrupt>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_interrupt))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_interrupt>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_interrupt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_interrupt>())).irq as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_interrupt),
            "::",
            stringify!(irq)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct kvm_dirty_log {
    pub slot: __u32,
    pub padding1: __u32,
    pub __bindgen_anon_1: kvm_dirty_log__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_dirty_log__bindgen_ty_1 {
    pub dirty_bitmap: *mut ::std::os::raw::c_void,
    pub padding2: __u64,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_kvm_dirty_log__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_dirty_log__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_dirty_log__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_dirty_log__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_dirty_log__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_dirty_log__bindgen_ty_1>())).dirty_bitmap as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dirty_log__bindgen_ty_1),
            "::",
            stringify!(dirty_bitmap)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_dirty_log__bindgen_ty_1>())).padding2 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dirty_log__bindgen_ty_1),
            "::",
            stringify!(padding2)
        )
    );
}
impl Default for kvm_dirty_log__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_kvm_dirty_log() {
    assert_eq!(
        ::std::mem::size_of::<kvm_dirty_log>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_dirty_log))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_dirty_log>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_dirty_log))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_dirty_log>())).slot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dirty_log),
            "::",
            stringify!(slot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_dirty_log>())).padding1 as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dirty_log),
            "::",
            stringify!(padding1)
        )
    );
}
impl Default for kvm_dirty_log {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct kvm_signal_mask {
    pub len: __u32,
    pub sigset: __IncompleteArrayField<__u8>,
}
#[test]
fn bindgen_test_layout_kvm_signal_mask() {
    assert_eq!(
        ::std::mem::size_of::<kvm_signal_mask>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_signal_mask))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_signal_mask>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_signal_mask))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_signal_mask>())).len as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_signal_mask),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_signal_mask>())).sigset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_signal_mask),
            "::",
            stringify!(sigset)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_tpr_access_ctl {
    pub enabled: __u32,
    pub flags: __u32,
    pub reserved: [__u32; 8usize],
}
#[test]
fn bindgen_test_layout_kvm_tpr_access_ctl() {
    assert_eq!(
        ::std::mem::size_of::<kvm_tpr_access_ctl>(),
        40usize,
        concat!("Size of: ", stringify!(kvm_tpr_access_ctl))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_tpr_access_ctl>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_tpr_access_ctl))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_tpr_access_ctl>())).enabled as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_tpr_access_ctl),
            "::",
            stringify!(enabled)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_tpr_access_ctl>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_tpr_access_ctl),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_tpr_access_ctl>())).reserved as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_tpr_access_ctl),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_vapic_addr {
    pub vapic_addr: __u64,
}
#[test]
fn bindgen_test_layout_kvm_vapic_addr() {
    assert_eq!(
        ::std::mem::size_of::<kvm_vapic_addr>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_vapic_addr))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_vapic_addr>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_vapic_addr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vapic_addr>())).vapic_addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vapic_addr),
            "::",
            stringify!(vapic_addr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_mp_state {
    pub mp_state: __u32,
}
#[test]
fn bindgen_test_layout_kvm_mp_state() {
    assert_eq!(
        ::std::mem::size_of::<kvm_mp_state>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_mp_state))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_mp_state>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_mp_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_mp_state>())).mp_state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_mp_state),
            "::",
            stringify!(mp_state)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_psw {
    pub mask: __u64,
    pub addr: __u64,
}
#[test]
fn bindgen_test_layout_kvm_s390_psw() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_psw>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_s390_psw))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_psw>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_s390_psw))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_psw>())).mask as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_psw),
            "::",
            stringify!(mask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_psw>())).addr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_psw),
            "::",
            stringify!(addr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_interrupt {
    pub type_: __u32,
    pub parm: __u32,
    pub parm64: __u64,
}
#[test]
fn bindgen_test_layout_kvm_s390_interrupt() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_interrupt>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_s390_interrupt))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_interrupt>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_s390_interrupt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_interrupt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_interrupt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_interrupt>())).parm as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_interrupt),
            "::",
            stringify!(parm)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_interrupt>())).parm64 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_interrupt),
            "::",
            stringify!(parm64)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_io_info {
    pub subchannel_id: __u16,
    pub subchannel_nr: __u16,
    pub io_int_parm: __u32,
    pub io_int_word: __u32,
}
#[test]
fn bindgen_test_layout_kvm_s390_io_info() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_io_info>(),
        12usize,
        concat!("Size of: ", stringify!(kvm_s390_io_info))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_io_info>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_s390_io_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_io_info>())).subchannel_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_io_info),
            "::",
            stringify!(subchannel_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_io_info>())).subchannel_nr as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_io_info),
            "::",
            stringify!(subchannel_nr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_io_info>())).io_int_parm as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_io_info),
            "::",
            stringify!(io_int_parm)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_io_info>())).io_int_word as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_io_info),
            "::",
            stringify!(io_int_word)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_ext_info {
    pub ext_params: __u32,
    pub pad: __u32,
    pub ext_params2: __u64,
}
#[test]
fn bindgen_test_layout_kvm_s390_ext_info() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_ext_info>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_s390_ext_info))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_ext_info>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_s390_ext_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_ext_info>())).ext_params as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_ext_info),
            "::",
            stringify!(ext_params)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_ext_info>())).pad as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_ext_info),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_ext_info>())).ext_params2 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_ext_info),
            "::",
            stringify!(ext_params2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_pgm_info {
    pub trans_exc_code: __u64,
    pub mon_code: __u64,
    pub per_address: __u64,
    pub data_exc_code: __u32,
    pub code: __u16,
    pub mon_class_nr: __u16,
    pub per_code: __u8,
    pub per_atmid: __u8,
    pub exc_access_id: __u8,
    pub per_access_id: __u8,
    pub op_access_id: __u8,
    pub flags: __u8,
    pub pad: [__u8; 2usize],
}
#[test]
fn bindgen_test_layout_kvm_s390_pgm_info() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_pgm_info>(),
        40usize,
        concat!("Size of: ", stringify!(kvm_s390_pgm_info))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_pgm_info>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_s390_pgm_info))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_s390_pgm_info>())).trans_exc_code as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_pgm_info),
            "::",
            stringify!(trans_exc_code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_pgm_info>())).mon_code as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_pgm_info),
            "::",
            stringify!(mon_code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_pgm_info>())).per_address as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_pgm_info),
            "::",
            stringify!(per_address)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_pgm_info>())).data_exc_code as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_pgm_info),
            "::",
            stringify!(data_exc_code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_pgm_info>())).code as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_pgm_info),
            "::",
            stringify!(code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_pgm_info>())).mon_class_nr as *const _ as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_pgm_info),
            "::",
            stringify!(mon_class_nr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_pgm_info>())).per_code as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_pgm_info),
            "::",
            stringify!(per_code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_pgm_info>())).per_atmid as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_pgm_info),
            "::",
            stringify!(per_atmid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_pgm_info>())).exc_access_id as *const _ as usize },
        34usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_pgm_info),
            "::",
            stringify!(exc_access_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_pgm_info>())).per_access_id as *const _ as usize },
        35usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_pgm_info),
            "::",
            stringify!(per_access_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_pgm_info>())).op_access_id as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_pgm_info),
            "::",
            stringify!(op_access_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_pgm_info>())).flags as *const _ as usize },
        37usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_pgm_info),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_pgm_info>())).pad as *const _ as usize },
        38usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_pgm_info),
            "::",
            stringify!(pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_prefix_info {
    pub address: __u32,
}
#[test]
fn bindgen_test_layout_kvm_s390_prefix_info() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_prefix_info>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_s390_prefix_info))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_prefix_info>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_s390_prefix_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_prefix_info>())).address as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_prefix_info),
            "::",
            stringify!(address)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_extcall_info {
    pub code: __u16,
}
#[test]
fn bindgen_test_layout_kvm_s390_extcall_info() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_extcall_info>(),
        2usize,
        concat!("Size of: ", stringify!(kvm_s390_extcall_info))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_extcall_info>(),
        2usize,
        concat!("Alignment of ", stringify!(kvm_s390_extcall_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_extcall_info>())).code as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_extcall_info),
            "::",
            stringify!(code)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_emerg_info {
    pub code: __u16,
}
#[test]
fn bindgen_test_layout_kvm_s390_emerg_info() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_emerg_info>(),
        2usize,
        concat!("Size of: ", stringify!(kvm_s390_emerg_info))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_emerg_info>(),
        2usize,
        concat!("Alignment of ", stringify!(kvm_s390_emerg_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_emerg_info>())).code as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_emerg_info),
            "::",
            stringify!(code)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_stop_info {
    pub flags: __u32,
}
#[test]
fn bindgen_test_layout_kvm_s390_stop_info() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_stop_info>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_s390_stop_info))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_stop_info>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_s390_stop_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_stop_info>())).flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_stop_info),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_mchk_info {
    pub cr14: __u64,
    pub mcic: __u64,
    pub failing_storage_address: __u64,
    pub ext_damage_code: __u32,
    pub pad: __u32,
    pub fixed_logout: [__u8; 16usize],
}
#[test]
fn bindgen_test_layout_kvm_s390_mchk_info() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_mchk_info>(),
        48usize,
        concat!("Size of: ", stringify!(kvm_s390_mchk_info))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_mchk_info>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_s390_mchk_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_mchk_info>())).cr14 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_mchk_info),
            "::",
            stringify!(cr14)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_mchk_info>())).mcic as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_mchk_info),
            "::",
            stringify!(mcic)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_s390_mchk_info>())).failing_storage_address as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_mchk_info),
            "::",
            stringify!(failing_storage_address)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_s390_mchk_info>())).ext_damage_code as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_mchk_info),
            "::",
            stringify!(ext_damage_code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_mchk_info>())).pad as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_mchk_info),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_mchk_info>())).fixed_logout as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_mchk_info),
            "::",
            stringify!(fixed_logout)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct kvm_s390_irq {
    pub type_: __u64,
    pub u: kvm_s390_irq__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_s390_irq__bindgen_ty_1 {
    pub io: kvm_s390_io_info,
    pub ext: kvm_s390_ext_info,
    pub pgm: kvm_s390_pgm_info,
    pub emerg: kvm_s390_emerg_info,
    pub extcall: kvm_s390_extcall_info,
    pub prefix: kvm_s390_prefix_info,
    pub stop: kvm_s390_stop_info,
    pub mchk: kvm_s390_mchk_info,
    pub reserved: [::std::os::raw::c_char; 64usize],
    _bindgen_union_align: [u64; 8usize],
}
#[test]
fn bindgen_test_layout_kvm_s390_irq__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_irq__bindgen_ty_1>(),
        64usize,
        concat!("Size of: ", stringify!(kvm_s390_irq__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_irq__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_s390_irq__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_irq__bindgen_ty_1>())).io as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq__bindgen_ty_1),
            "::",
            stringify!(io)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_irq__bindgen_ty_1>())).ext as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq__bindgen_ty_1),
            "::",
            stringify!(ext)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_irq__bindgen_ty_1>())).pgm as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq__bindgen_ty_1),
            "::",
            stringify!(pgm)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_s390_irq__bindgen_ty_1>())).emerg as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq__bindgen_ty_1),
            "::",
            stringify!(emerg)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_s390_irq__bindgen_ty_1>())).extcall as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq__bindgen_ty_1),
            "::",
            stringify!(extcall)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_s390_irq__bindgen_ty_1>())).prefix as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq__bindgen_ty_1),
            "::",
            stringify!(prefix)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_irq__bindgen_ty_1>())).stop as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq__bindgen_ty_1),
            "::",
            stringify!(stop)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_irq__bindgen_ty_1>())).mchk as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq__bindgen_ty_1),
            "::",
            stringify!(mchk)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_s390_irq__bindgen_ty_1>())).reserved as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq__bindgen_ty_1),
            "::",
            stringify!(reserved)
        )
    );
}
impl Default for kvm_s390_irq__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_kvm_s390_irq() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_irq>(),
        72usize,
        concat!("Size of: ", stringify!(kvm_s390_irq))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_irq>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_s390_irq))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_irq>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_irq>())).u as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq),
            "::",
            stringify!(u)
        )
    );
}
impl Default for kvm_s390_irq {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_irq_state {
    pub buf: __u64,
    pub flags: __u32,
    pub len: __u32,
    pub reserved: [__u32; 4usize],
}
#[test]
fn bindgen_test_layout_kvm_s390_irq_state() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_irq_state>(),
        32usize,
        concat!("Size of: ", stringify!(kvm_s390_irq_state))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_irq_state>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_s390_irq_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_irq_state>())).buf as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq_state),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_irq_state>())).flags as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq_state),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_irq_state>())).len as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq_state),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_irq_state>())).reserved as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_irq_state),
            "::",
            stringify!(reserved)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_guest_debug {
    pub control: __u32,
    pub pad: __u32,
    pub arch: kvm_guest_debug_arch,
}
#[test]
fn bindgen_test_layout_kvm_guest_debug() {
    assert_eq!(
        ::std::mem::size_of::<kvm_guest_debug>(),
        72usize,
        concat!("Size of: ", stringify!(kvm_guest_debug))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_guest_debug>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_guest_debug))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_guest_debug>())).control as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_guest_debug),
            "::",
            stringify!(control)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_guest_debug>())).pad as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_guest_debug),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_guest_debug>())).arch as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_guest_debug),
            "::",
            stringify!(arch)
        )
    );
}
pub const kvm_ioeventfd_flag_nr_datamatch: _bindgen_ty_1 = 0;
pub const kvm_ioeventfd_flag_nr_pio: _bindgen_ty_1 = 1;
pub const kvm_ioeventfd_flag_nr_deassign: _bindgen_ty_1 = 2;
pub const kvm_ioeventfd_flag_nr_virtio_ccw_notify: _bindgen_ty_1 = 3;
pub const kvm_ioeventfd_flag_nr_fast_mmio: _bindgen_ty_1 = 4;
pub const kvm_ioeventfd_flag_nr_max: _bindgen_ty_1 = 5;
pub type _bindgen_ty_1 = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ioeventfd {
    pub datamatch: __u64,
    pub addr: __u64,
    pub len: __u32,
    pub fd: __s32,
    pub flags: __u32,
    pub pad: [__u8; 36usize],
}
#[test]
fn bindgen_test_layout_kvm_ioeventfd() {
    assert_eq!(
        ::std::mem::size_of::<kvm_ioeventfd>(),
        64usize,
        concat!("Size of: ", stringify!(kvm_ioeventfd))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_ioeventfd>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_ioeventfd))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioeventfd>())).datamatch as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioeventfd),
            "::",
            stringify!(datamatch)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioeventfd>())).addr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioeventfd),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioeventfd>())).len as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioeventfd),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioeventfd>())).fd as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioeventfd),
            "::",
            stringify!(fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioeventfd>())).flags as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioeventfd),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ioeventfd>())).pad as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ioeventfd),
            "::",
            stringify!(pad)
        )
    );
}
impl Default for kvm_ioeventfd {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_enable_cap {
    pub cap: __u32,
    pub flags: __u32,
    pub args: [__u64; 4usize],
    pub pad: [__u8; 64usize],
}
#[test]
fn bindgen_test_layout_kvm_enable_cap() {
    assert_eq!(
        ::std::mem::size_of::<kvm_enable_cap>(),
        104usize,
        concat!("Size of: ", stringify!(kvm_enable_cap))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_enable_cap>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_enable_cap))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_enable_cap>())).cap as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_enable_cap),
            "::",
            stringify!(cap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_enable_cap>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_enable_cap),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_enable_cap>())).args as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_enable_cap),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_enable_cap>())).pad as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_enable_cap),
            "::",
            stringify!(pad)
        )
    );
}
impl Default for kvm_enable_cap {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ppc_pvinfo {
    pub flags: __u32,
    pub hcall: [__u32; 4usize],
    pub pad: [__u8; 108usize],
}
#[test]
fn bindgen_test_layout_kvm_ppc_pvinfo() {
    assert_eq!(
        ::std::mem::size_of::<kvm_ppc_pvinfo>(),
        128usize,
        concat!("Size of: ", stringify!(kvm_ppc_pvinfo))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_ppc_pvinfo>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_ppc_pvinfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ppc_pvinfo>())).flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_pvinfo),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ppc_pvinfo>())).hcall as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_pvinfo),
            "::",
            stringify!(hcall)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ppc_pvinfo>())).pad as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_pvinfo),
            "::",
            stringify!(pad)
        )
    );
}
impl Default for kvm_ppc_pvinfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_ppc_one_page_size {
    pub page_shift: __u32,
    pub pte_enc: __u32,
}
#[test]
fn bindgen_test_layout_kvm_ppc_one_page_size() {
    assert_eq!(
        ::std::mem::size_of::<kvm_ppc_one_page_size>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_ppc_one_page_size))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_ppc_one_page_size>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_ppc_one_page_size))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_ppc_one_page_size>())).page_shift as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_one_page_size),
            "::",
            stringify!(page_shift)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ppc_one_page_size>())).pte_enc as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_one_page_size),
            "::",
            stringify!(pte_enc)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_ppc_one_seg_page_size {
    pub page_shift: __u32,
    pub slb_enc: __u32,
    pub enc: [kvm_ppc_one_page_size; 8usize],
}
#[test]
fn bindgen_test_layout_kvm_ppc_one_seg_page_size() {
    assert_eq!(
        ::std::mem::size_of::<kvm_ppc_one_seg_page_size>(),
        72usize,
        concat!("Size of: ", stringify!(kvm_ppc_one_seg_page_size))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_ppc_one_seg_page_size>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_ppc_one_seg_page_size))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_ppc_one_seg_page_size>())).page_shift as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_one_seg_page_size),
            "::",
            stringify!(page_shift)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_ppc_one_seg_page_size>())).slb_enc as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_one_seg_page_size),
            "::",
            stringify!(slb_enc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ppc_one_seg_page_size>())).enc as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_one_seg_page_size),
            "::",
            stringify!(enc)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_ppc_smmu_info {
    pub flags: __u64,
    pub slb_size: __u32,
    pub data_keys: __u16,
    pub instr_keys: __u16,
    pub sps: [kvm_ppc_one_seg_page_size; 8usize],
}
#[test]
fn bindgen_test_layout_kvm_ppc_smmu_info() {
    assert_eq!(
        ::std::mem::size_of::<kvm_ppc_smmu_info>(),
        592usize,
        concat!("Size of: ", stringify!(kvm_ppc_smmu_info))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_ppc_smmu_info>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_ppc_smmu_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ppc_smmu_info>())).flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_smmu_info),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ppc_smmu_info>())).slb_size as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_smmu_info),
            "::",
            stringify!(slb_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ppc_smmu_info>())).data_keys as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_smmu_info),
            "::",
            stringify!(data_keys)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ppc_smmu_info>())).instr_keys as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_smmu_info),
            "::",
            stringify!(instr_keys)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ppc_smmu_info>())).sps as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_smmu_info),
            "::",
            stringify!(sps)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_ppc_resize_hpt {
    pub flags: __u64,
    pub shift: __u32,
    pub pad: __u32,
}
#[test]
fn bindgen_test_layout_kvm_ppc_resize_hpt() {
    assert_eq!(
        ::std::mem::size_of::<kvm_ppc_resize_hpt>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_ppc_resize_hpt))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_ppc_resize_hpt>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_ppc_resize_hpt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ppc_resize_hpt>())).flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_resize_hpt),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ppc_resize_hpt>())).shift as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_resize_hpt),
            "::",
            stringify!(shift)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_ppc_resize_hpt>())).pad as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_ppc_resize_hpt),
            "::",
            stringify!(pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_irq_routing_irqchip {
    pub irqchip: __u32,
    pub pin: __u32,
}
#[test]
fn bindgen_test_layout_kvm_irq_routing_irqchip() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irq_routing_irqchip>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_irq_routing_irqchip))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irq_routing_irqchip>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_irq_routing_irqchip))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing_irqchip>())).irqchip as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_irqchip),
            "::",
            stringify!(irqchip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing_irqchip>())).pin as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_irqchip),
            "::",
            stringify!(pin)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct kvm_irq_routing_msi {
    pub address_lo: __u32,
    pub address_hi: __u32,
    pub data: __u32,
    pub __bindgen_anon_1: kvm_irq_routing_msi__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_irq_routing_msi__bindgen_ty_1 {
    pub pad: __u32,
    pub devid: __u32,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_kvm_irq_routing_msi__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irq_routing_msi__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(kvm_irq_routing_msi__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irq_routing_msi__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_irq_routing_msi__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irq_routing_msi__bindgen_ty_1>())).pad as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_msi__bindgen_ty_1),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irq_routing_msi__bindgen_ty_1>())).devid as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_msi__bindgen_ty_1),
            "::",
            stringify!(devid)
        )
    );
}
impl Default for kvm_irq_routing_msi__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_kvm_irq_routing_msi() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irq_routing_msi>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_irq_routing_msi))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irq_routing_msi>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_irq_routing_msi))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing_msi>())).address_lo as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_msi),
            "::",
            stringify!(address_lo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing_msi>())).address_hi as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_msi),
            "::",
            stringify!(address_hi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing_msi>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_msi),
            "::",
            stringify!(data)
        )
    );
}
impl Default for kvm_irq_routing_msi {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_irq_routing_s390_adapter {
    pub ind_addr: __u64,
    pub summary_addr: __u64,
    pub ind_offset: __u64,
    pub summary_offset: __u32,
    pub adapter_id: __u32,
}
#[test]
fn bindgen_test_layout_kvm_irq_routing_s390_adapter() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irq_routing_s390_adapter>(),
        32usize,
        concat!("Size of: ", stringify!(kvm_irq_routing_s390_adapter))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irq_routing_s390_adapter>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_irq_routing_s390_adapter))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irq_routing_s390_adapter>())).ind_addr as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_s390_adapter),
            "::",
            stringify!(ind_addr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irq_routing_s390_adapter>())).summary_addr as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_s390_adapter),
            "::",
            stringify!(summary_addr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irq_routing_s390_adapter>())).ind_offset as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_s390_adapter),
            "::",
            stringify!(ind_offset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irq_routing_s390_adapter>())).summary_offset as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_s390_adapter),
            "::",
            stringify!(summary_offset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irq_routing_s390_adapter>())).adapter_id as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_s390_adapter),
            "::",
            stringify!(adapter_id)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_irq_routing_hv_sint {
    pub vcpu: __u32,
    pub sint: __u32,
}
#[test]
fn bindgen_test_layout_kvm_irq_routing_hv_sint() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irq_routing_hv_sint>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_irq_routing_hv_sint))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irq_routing_hv_sint>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_irq_routing_hv_sint))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing_hv_sint>())).vcpu as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_hv_sint),
            "::",
            stringify!(vcpu)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing_hv_sint>())).sint as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_hv_sint),
            "::",
            stringify!(sint)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct kvm_irq_routing_entry {
    pub gsi: __u32,
    pub type_: __u32,
    pub flags: __u32,
    pub pad: __u32,
    pub u: kvm_irq_routing_entry__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_irq_routing_entry__bindgen_ty_1 {
    pub irqchip: kvm_irq_routing_irqchip,
    pub msi: kvm_irq_routing_msi,
    pub adapter: kvm_irq_routing_s390_adapter,
    pub hv_sint: kvm_irq_routing_hv_sint,
    pub pad: [__u32; 8usize],
    _bindgen_union_align: [u64; 4usize],
}
#[test]
fn bindgen_test_layout_kvm_irq_routing_entry__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irq_routing_entry__bindgen_ty_1>(),
        32usize,
        concat!("Size of: ", stringify!(kvm_irq_routing_entry__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irq_routing_entry__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_irq_routing_entry__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irq_routing_entry__bindgen_ty_1>())).irqchip as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_entry__bindgen_ty_1),
            "::",
            stringify!(irqchip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irq_routing_entry__bindgen_ty_1>())).msi as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_entry__bindgen_ty_1),
            "::",
            stringify!(msi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irq_routing_entry__bindgen_ty_1>())).adapter as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_entry__bindgen_ty_1),
            "::",
            stringify!(adapter)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irq_routing_entry__bindgen_ty_1>())).hv_sint as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_entry__bindgen_ty_1),
            "::",
            stringify!(hv_sint)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_irq_routing_entry__bindgen_ty_1>())).pad as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_entry__bindgen_ty_1),
            "::",
            stringify!(pad)
        )
    );
}
impl Default for kvm_irq_routing_entry__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_kvm_irq_routing_entry() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irq_routing_entry>(),
        48usize,
        concat!("Size of: ", stringify!(kvm_irq_routing_entry))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irq_routing_entry>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_irq_routing_entry))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing_entry>())).gsi as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_entry),
            "::",
            stringify!(gsi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing_entry>())).type_ as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_entry),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing_entry>())).flags as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_entry),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing_entry>())).pad as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_entry),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing_entry>())).u as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing_entry),
            "::",
            stringify!(u)
        )
    );
}
impl Default for kvm_irq_routing_entry {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct kvm_irq_routing {
    pub nr: __u32,
    pub flags: __u32,
    pub entries: __IncompleteArrayField<kvm_irq_routing_entry>,
}
#[test]
fn bindgen_test_layout_kvm_irq_routing() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irq_routing>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_irq_routing))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irq_routing>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_irq_routing))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing>())).nr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing),
            "::",
            stringify!(nr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irq_routing>())).entries as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irq_routing),
            "::",
            stringify!(entries)
        )
    );
}
impl Default for kvm_irq_routing {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_x86_mce {
    pub status: __u64,
    pub addr: __u64,
    pub misc: __u64,
    pub mcg_status: __u64,
    pub bank: __u8,
    pub pad1: [__u8; 7usize],
    pub pad2: [__u64; 3usize],
}
#[test]
fn bindgen_test_layout_kvm_x86_mce() {
    assert_eq!(
        ::std::mem::size_of::<kvm_x86_mce>(),
        64usize,
        concat!("Size of: ", stringify!(kvm_x86_mce))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_x86_mce>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_x86_mce))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_x86_mce>())).status as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_x86_mce),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_x86_mce>())).addr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_x86_mce),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_x86_mce>())).misc as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_x86_mce),
            "::",
            stringify!(misc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_x86_mce>())).mcg_status as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_x86_mce),
            "::",
            stringify!(mcg_status)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_x86_mce>())).bank as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_x86_mce),
            "::",
            stringify!(bank)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_x86_mce>())).pad1 as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_x86_mce),
            "::",
            stringify!(pad1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_x86_mce>())).pad2 as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_x86_mce),
            "::",
            stringify!(pad2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_xen_hvm_config {
    pub flags: __u32,
    pub msr: __u32,
    pub blob_addr_32: __u64,
    pub blob_addr_64: __u64,
    pub blob_size_32: __u8,
    pub blob_size_64: __u8,
    pub pad2: [__u8; 30usize],
}
#[test]
fn bindgen_test_layout_kvm_xen_hvm_config() {
    assert_eq!(
        ::std::mem::size_of::<kvm_xen_hvm_config>(),
        56usize,
        concat!("Size of: ", stringify!(kvm_xen_hvm_config))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_xen_hvm_config>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_xen_hvm_config))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xen_hvm_config>())).flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xen_hvm_config),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xen_hvm_config>())).msr as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xen_hvm_config),
            "::",
            stringify!(msr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xen_hvm_config>())).blob_addr_32 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xen_hvm_config),
            "::",
            stringify!(blob_addr_32)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xen_hvm_config>())).blob_addr_64 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xen_hvm_config),
            "::",
            stringify!(blob_addr_64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xen_hvm_config>())).blob_size_32 as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xen_hvm_config),
            "::",
            stringify!(blob_size_32)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xen_hvm_config>())).blob_size_64 as *const _ as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xen_hvm_config),
            "::",
            stringify!(blob_size_64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_xen_hvm_config>())).pad2 as *const _ as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_xen_hvm_config),
            "::",
            stringify!(pad2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_irqfd {
    pub fd: __u32,
    pub gsi: __u32,
    pub flags: __u32,
    pub resamplefd: __u32,
    pub pad: [__u8; 16usize],
}
#[test]
fn bindgen_test_layout_kvm_irqfd() {
    assert_eq!(
        ::std::mem::size_of::<kvm_irqfd>(),
        32usize,
        concat!("Size of: ", stringify!(kvm_irqfd))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_irqfd>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_irqfd))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqfd>())).fd as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqfd),
            "::",
            stringify!(fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqfd>())).gsi as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqfd),
            "::",
            stringify!(gsi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqfd>())).flags as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqfd),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqfd>())).resamplefd as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqfd),
            "::",
            stringify!(resamplefd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_irqfd>())).pad as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_irqfd),
            "::",
            stringify!(pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_clock_data {
    pub clock: __u64,
    pub flags: __u32,
    pub pad: [__u32; 9usize],
}
#[test]
fn bindgen_test_layout_kvm_clock_data() {
    assert_eq!(
        ::std::mem::size_of::<kvm_clock_data>(),
        48usize,
        concat!("Size of: ", stringify!(kvm_clock_data))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_clock_data>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_clock_data))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_clock_data>())).clock as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_clock_data),
            "::",
            stringify!(clock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_clock_data>())).flags as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_clock_data),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_clock_data>())).pad as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_clock_data),
            "::",
            stringify!(pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_config_tlb {
    pub params: __u64,
    pub array: __u64,
    pub mmu_type: __u32,
    pub array_len: __u32,
}
#[test]
fn bindgen_test_layout_kvm_config_tlb() {
    assert_eq!(
        ::std::mem::size_of::<kvm_config_tlb>(),
        24usize,
        concat!("Size of: ", stringify!(kvm_config_tlb))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_config_tlb>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_config_tlb))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_config_tlb>())).params as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_config_tlb),
            "::",
            stringify!(params)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_config_tlb>())).array as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_config_tlb),
            "::",
            stringify!(array)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_config_tlb>())).mmu_type as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_config_tlb),
            "::",
            stringify!(mmu_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_config_tlb>())).array_len as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_config_tlb),
            "::",
            stringify!(array_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_dirty_tlb {
    pub bitmap: __u64,
    pub num_dirty: __u32,
}
#[test]
fn bindgen_test_layout_kvm_dirty_tlb() {
    assert_eq!(
        ::std::mem::size_of::<kvm_dirty_tlb>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_dirty_tlb))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_dirty_tlb>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_dirty_tlb))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_dirty_tlb>())).bitmap as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dirty_tlb),
            "::",
            stringify!(bitmap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_dirty_tlb>())).num_dirty as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dirty_tlb),
            "::",
            stringify!(num_dirty)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct kvm_reg_list {
    pub n: __u64,
    pub reg: __IncompleteArrayField<__u64>,
}
#[test]
fn bindgen_test_layout_kvm_reg_list() {
    assert_eq!(
        ::std::mem::size_of::<kvm_reg_list>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_reg_list))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_reg_list>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_reg_list))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_reg_list>())).n as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_reg_list),
            "::",
            stringify!(n)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_reg_list>())).reg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_reg_list),
            "::",
            stringify!(reg)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_one_reg {
    pub id: __u64,
    pub addr: __u64,
}
#[test]
fn bindgen_test_layout_kvm_one_reg() {
    assert_eq!(
        ::std::mem::size_of::<kvm_one_reg>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_one_reg))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_one_reg>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_one_reg))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_one_reg>())).id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_one_reg),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_one_reg>())).addr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_one_reg),
            "::",
            stringify!(addr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_msi {
    pub address_lo: __u32,
    pub address_hi: __u32,
    pub data: __u32,
    pub flags: __u32,
    pub devid: __u32,
    pub pad: [__u8; 12usize],
}
#[test]
fn bindgen_test_layout_kvm_msi() {
    assert_eq!(
        ::std::mem::size_of::<kvm_msi>(),
        32usize,
        concat!("Size of: ", stringify!(kvm_msi))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_msi>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_msi))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msi>())).address_lo as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msi),
            "::",
            stringify!(address_lo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msi>())).address_hi as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msi),
            "::",
            stringify!(address_hi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msi>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msi),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msi>())).flags as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msi),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msi>())).devid as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msi),
            "::",
            stringify!(devid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_msi>())).pad as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_msi),
            "::",
            stringify!(pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_arm_device_addr {
    pub id: __u64,
    pub addr: __u64,
}
#[test]
fn bindgen_test_layout_kvm_arm_device_addr() {
    assert_eq!(
        ::std::mem::size_of::<kvm_arm_device_addr>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_arm_device_addr))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_arm_device_addr>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_arm_device_addr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_arm_device_addr>())).id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_arm_device_addr),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_arm_device_addr>())).addr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_arm_device_addr),
            "::",
            stringify!(addr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_create_device {
    pub type_: __u32,
    pub fd: __u32,
    pub flags: __u32,
}
#[test]
fn bindgen_test_layout_kvm_create_device() {
    assert_eq!(
        ::std::mem::size_of::<kvm_create_device>(),
        12usize,
        concat!("Size of: ", stringify!(kvm_create_device))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_create_device>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_create_device))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_create_device>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_create_device),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_create_device>())).fd as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_create_device),
            "::",
            stringify!(fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_create_device>())).flags as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_create_device),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_device_attr {
    pub flags: __u32,
    pub group: __u32,
    pub attr: __u64,
    pub addr: __u64,
}
#[test]
fn bindgen_test_layout_kvm_device_attr() {
    assert_eq!(
        ::std::mem::size_of::<kvm_device_attr>(),
        24usize,
        concat!("Size of: ", stringify!(kvm_device_attr))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_device_attr>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_device_attr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_device_attr>())).flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_device_attr),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_device_attr>())).group as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_device_attr),
            "::",
            stringify!(group)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_device_attr>())).attr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_device_attr),
            "::",
            stringify!(attr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_device_attr>())).addr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_device_attr),
            "::",
            stringify!(addr)
        )
    );
}
pub const kvm_device_type_KVM_DEV_TYPE_FSL_MPIC_20: kvm_device_type = 1;
pub const kvm_device_type_KVM_DEV_TYPE_FSL_MPIC_42: kvm_device_type = 2;
pub const kvm_device_type_KVM_DEV_TYPE_XICS: kvm_device_type = 3;
pub const kvm_device_type_KVM_DEV_TYPE_VFIO: kvm_device_type = 4;
pub const kvm_device_type_KVM_DEV_TYPE_ARM_VGIC_V2: kvm_device_type = 5;
pub const kvm_device_type_KVM_DEV_TYPE_FLIC: kvm_device_type = 6;
pub const kvm_device_type_KVM_DEV_TYPE_ARM_VGIC_V3: kvm_device_type = 7;
pub const kvm_device_type_KVM_DEV_TYPE_ARM_VGIC_ITS: kvm_device_type = 8;
pub const kvm_device_type_KVM_DEV_TYPE_MAX: kvm_device_type = 9;
pub type kvm_device_type = u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_vfio_spapr_tce {
    pub groupfd: __s32,
    pub tablefd: __s32,
}
#[test]
fn bindgen_test_layout_kvm_vfio_spapr_tce() {
    assert_eq!(
        ::std::mem::size_of::<kvm_vfio_spapr_tce>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_vfio_spapr_tce))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_vfio_spapr_tce>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_vfio_spapr_tce))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vfio_spapr_tce>())).groupfd as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vfio_spapr_tce),
            "::",
            stringify!(groupfd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_vfio_spapr_tce>())).tablefd as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_vfio_spapr_tce),
            "::",
            stringify!(tablefd)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_s390_ucas_mapping {
    pub user_addr: __u64,
    pub vcpu_addr: __u64,
    pub length: __u64,
}
#[test]
fn bindgen_test_layout_kvm_s390_ucas_mapping() {
    assert_eq!(
        ::std::mem::size_of::<kvm_s390_ucas_mapping>(),
        24usize,
        concat!("Size of: ", stringify!(kvm_s390_ucas_mapping))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_s390_ucas_mapping>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_s390_ucas_mapping))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_ucas_mapping>())).user_addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_ucas_mapping),
            "::",
            stringify!(user_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_ucas_mapping>())).vcpu_addr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_ucas_mapping),
            "::",
            stringify!(vcpu_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_s390_ucas_mapping>())).length as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_s390_ucas_mapping),
            "::",
            stringify!(length)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct kvm_assigned_pci_dev {
    pub assigned_dev_id: __u32,
    pub busnr: __u32,
    pub devfn: __u32,
    pub flags: __u32,
    pub segnr: __u32,
    pub __bindgen_anon_1: kvm_assigned_pci_dev__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_assigned_pci_dev__bindgen_ty_1 {
    pub reserved: [__u32; 11usize],
    _bindgen_union_align: [u32; 11usize],
}
#[test]
fn bindgen_test_layout_kvm_assigned_pci_dev__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_assigned_pci_dev__bindgen_ty_1>(),
        44usize,
        concat!("Size of: ", stringify!(kvm_assigned_pci_dev__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_assigned_pci_dev__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(kvm_assigned_pci_dev__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_assigned_pci_dev__bindgen_ty_1>())).reserved as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_pci_dev__bindgen_ty_1),
            "::",
            stringify!(reserved)
        )
    );
}
impl Default for kvm_assigned_pci_dev__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_kvm_assigned_pci_dev() {
    assert_eq!(
        ::std::mem::size_of::<kvm_assigned_pci_dev>(),
        64usize,
        concat!("Size of: ", stringify!(kvm_assigned_pci_dev))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_assigned_pci_dev>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_assigned_pci_dev))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_assigned_pci_dev>())).assigned_dev_id as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_pci_dev),
            "::",
            stringify!(assigned_dev_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_assigned_pci_dev>())).busnr as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_pci_dev),
            "::",
            stringify!(busnr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_assigned_pci_dev>())).devfn as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_pci_dev),
            "::",
            stringify!(devfn)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_assigned_pci_dev>())).flags as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_pci_dev),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_assigned_pci_dev>())).segnr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_pci_dev),
            "::",
            stringify!(segnr)
        )
    );
}
impl Default for kvm_assigned_pci_dev {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct kvm_assigned_irq {
    pub assigned_dev_id: __u32,
    pub host_irq: __u32,
    pub guest_irq: __u32,
    pub flags: __u32,
    pub __bindgen_anon_1: kvm_assigned_irq__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_assigned_irq__bindgen_ty_1 {
    pub reserved: [__u32; 12usize],
    _bindgen_union_align: [u32; 12usize],
}
#[test]
fn bindgen_test_layout_kvm_assigned_irq__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<kvm_assigned_irq__bindgen_ty_1>(),
        48usize,
        concat!("Size of: ", stringify!(kvm_assigned_irq__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_assigned_irq__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_assigned_irq__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_assigned_irq__bindgen_ty_1>())).reserved as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_irq__bindgen_ty_1),
            "::",
            stringify!(reserved)
        )
    );
}
impl Default for kvm_assigned_irq__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_kvm_assigned_irq() {
    assert_eq!(
        ::std::mem::size_of::<kvm_assigned_irq>(),
        64usize,
        concat!("Size of: ", stringify!(kvm_assigned_irq))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_assigned_irq>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_assigned_irq))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_assigned_irq>())).assigned_dev_id as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_irq),
            "::",
            stringify!(assigned_dev_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_assigned_irq>())).host_irq as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_irq),
            "::",
            stringify!(host_irq)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_assigned_irq>())).guest_irq as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_irq),
            "::",
            stringify!(guest_irq)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_assigned_irq>())).flags as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_irq),
            "::",
            stringify!(flags)
        )
    );
}
impl Default for kvm_assigned_irq {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_assigned_msix_nr {
    pub assigned_dev_id: __u32,
    pub entry_nr: __u16,
    pub padding: __u16,
}
#[test]
fn bindgen_test_layout_kvm_assigned_msix_nr() {
    assert_eq!(
        ::std::mem::size_of::<kvm_assigned_msix_nr>(),
        8usize,
        concat!("Size of: ", stringify!(kvm_assigned_msix_nr))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_assigned_msix_nr>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_assigned_msix_nr))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_assigned_msix_nr>())).assigned_dev_id as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_msix_nr),
            "::",
            stringify!(assigned_dev_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_assigned_msix_nr>())).entry_nr as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_msix_nr),
            "::",
            stringify!(entry_nr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_assigned_msix_nr>())).padding as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_msix_nr),
            "::",
            stringify!(padding)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct kvm_assigned_msix_entry {
    pub assigned_dev_id: __u32,
    pub gsi: __u32,
    pub entry: __u16,
    pub padding: [__u16; 3usize],
}
#[test]
fn bindgen_test_layout_kvm_assigned_msix_entry() {
    assert_eq!(
        ::std::mem::size_of::<kvm_assigned_msix_entry>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_assigned_msix_entry))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_assigned_msix_entry>(),
        4usize,
        concat!("Alignment of ", stringify!(kvm_assigned_msix_entry))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_assigned_msix_entry>())).assigned_dev_id as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_msix_entry),
            "::",
            stringify!(assigned_dev_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_assigned_msix_entry>())).gsi as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_msix_entry),
            "::",
            stringify!(gsi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_assigned_msix_entry>())).entry as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_msix_entry),
            "::",
            stringify!(entry)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_assigned_msix_entry>())).padding as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_assigned_msix_entry),
            "::",
            stringify!(padding)
        )
    );
}
