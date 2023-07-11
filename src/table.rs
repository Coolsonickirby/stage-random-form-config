use skyline::hooks::{getRegionAddress, Region};

// Stage structs copied from jam's poc
// https://github.com/jam1garner/stage-table-refs/blob/master/src/main.rs#L231C1-L253C2
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, serde::Deserialize)]
pub enum StageKind {
    Normal,
    End,
    Battle,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Hash40(pub u64);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StageTableEntry {
    pub stage_id: u32,
    pub stage_num: u32,
    pub stage_kind: StageKind,
    pub param_name_hash: Hash40,
    pub stage_load_group_hash: Hash40,
    pub effect_load_group_hash: Hash40,
    pub nus3bank_path_hash: Hash40,
    pub sqb_path_hash: Hash40,
    pub nus3audio_path_hash: Hash40,
    pub tonelabel_path_hash: Hash40,
}


pub struct StageTable {
    pub offset: usize,                                       // Original table offset
    pub table: Vec<StageTableEntry>,                         // The new table
}

impl StageTable {
    pub fn new(offset: usize, size: usize) -> StageTable {
        // clone table with offset and size
        unsafe {
            let data = getRegionAddress(Region::Text) as usize;

            // Subtract start of data offset
            let mut current_offset = offset;

            let mut table: Vec<StageTableEntry> = vec![];

            for _ in 0..size {
                let stage_entry = *((data + current_offset) as *const StageTableEntry);
                table.push(stage_entry);
                current_offset += core::mem::size_of::<StageTableEntry>();
            }

            StageTable {
                offset: offset,
                table: table,
            }
        }
    }
}