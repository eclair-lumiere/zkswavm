#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LocationType {
    Heap = 0,
    Stack = 1,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccessType {
    Read = 1,
    Write = 2,
    Init = 3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VarType {
    U8 = 1,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
}

#[derive(Clone)]
pub struct MemoryTableEntry {
    pub eid: u64,
    pub emid: u64,
    pub mmid: u64,
    pub offset: u64,
    pub ltype: LocationType,
    pub atype: AccessType,
    pub vtype: VarType,
    pub value: u64,
}

impl MemoryTableEntry {
    pub fn is_same_location(&self, other: &MemoryTableEntry) -> bool {
        self.mmid == other.mmid && self.offset == other.offset && self.ltype == other.ltype
    }
}
