from glob import glob
import os


SER_RS_HEADER = """\
// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::mem;
use std::ptr;

use libc;
use serde::de::{Deserialize, Deserializer};
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
        Ok(deserialize_ffi::<__BindgenBitfieldUnit<Storage, Align>>(v))
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
"""

SER_DESER_TEMPLATE = """
impl Serialize for TYPENAME {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let bytes = serialize_ffi::<TYPENAME>(&self);
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TYPENAME {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
        Ok(deserialize_ffi::<TYPENAME>(v))
    }
}

"""

# For these, it's easier to implement manually.
BLACKLIST = [
    '__BindgenBitfieldUnit',
    '__IncompleteArrayField'
]


def is_relevant(line):
    for prefix in ['pub struct ', 'pub union ']:
        if line.startswith(prefix):
            for blacklisted in BLACKLIST:
                if blacklisted in line:
                    return False, -1
            return True, len(prefix)
    return False, -1


def main():
    srcdir = 'src/x86'
    path_prefix = os.path.join(srcdir, 'bindings_')
    for fname in glob('{}*.rs'.format(path_prefix)):
        suffix = fname[len(path_prefix):]
        fname_out = os.path.join(srcdir, 'serialize_{}'.format(suffix))

        with open(fname) as fin, open(fname_out, 'w') as fout:
            fout.write(SER_RS_HEADER)

            for line in fin:
                relevant, idx = is_relevant(line)
                if relevant:
                    type = line[idx:line.rfind(' ')]
                    fout.write(SER_DESER_TEMPLATE.replace('TYPENAME', type))


if __name__ == '__main__':
    main()
