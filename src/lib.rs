mod table;
mod config;
use serde;


use config::*;
use crate::table::{StageTable, StageKind};
use once_cell::sync::Lazy;
use skyline::{hooks::InlineCtx, install_hooks};
use std::collections::HashMap;
use rand::{seq::SliceRandom, Rng};

#[repr(u32)]
enum RuleStageType {
    InOrder = 0,
    Random = 1,
    BattlefieldAndOmega = 2,
    BattlefieldOnly = 3,
    OmegaOnly = 4,
    ManuallyPicked = 0xfffffffd,
}

static mut STAGE_IS_RANDOM: [bool; 3] = [false, false, false];
pub static mut STAGE_TABLE: Lazy<StageTable> = Lazy::new(|| StageTable::new(0x45499C0, 0x16B));
pub static mut MAPPED_STAGE_NUM_TO_KIND_ID: Lazy<HashMap<u32, HashMap<StageKind, u32>>> = Lazy::new(|| HashMap::new());
pub static mut STAGE_PARAM_CONFIG_DATA: Lazy<DataConfig> = Lazy::new(|| DataConfig(HashMap::new()));

#[skyline::hook(offset = 0x184a698, inline)]
unsafe fn set_selected_stage_hash(ctx: &InlineCtx) {
    let stage_placement_id = *ctx.registers[23].w.as_ref();
    let ui_stage_hash = *ctx.registers[8].x.as_ref() & 0xFFFFFFFFFF;
    STAGE_IS_RANDOM[stage_placement_id as usize] = ui_stage_hash == 0x0f1f1e484c;
}

#[skyline::hook(offset = 0x17939ec, inline)]
unsafe fn check_selection_type(ctx: &InlineCtx) {
    let selection_type = *ctx.registers[21].w.as_ref();
    if selection_type == RuleStageType::Random as u32 {
        for x in 0..3 {
            STAGE_IS_RANDOM[x] = true;
        }
    } else if selection_type != RuleStageType::ManuallyPicked as u32 {
        for x in 0..3 {
            STAGE_IS_RANDOM[x] = false;
        }
    }
}

// Original plan, but HDR hooks the same exact location apparently so it conflicts with that
// #[skyline::hook(offset = 0x178ab60)]
// unsafe fn setup_stage(
//     stage_morph_id: u64,
//     mut stage_id: u32,
//     ui_bgm_id: u64,
//     mut hazards_on: bool,
//     mc_biomes_type: u32
// ){
//     if STAGE_IS_RANDOM[stage_morph_id as usize] {
//         let selected_stage = STAGE_TABLE.table[stage_id as usize];
//         let param_hash = selected_stage.param_name_hash.0;
        
//         match STAGE_PARAM_CONFIG_DATA.0.get(&param_hash) {
//             Some(v) => {
//                 // Set stage kind (normal, bf, fd)
//                 match v.valid_types.choose(&mut rand::thread_rng()) {
//                     Some(kind) => {
//                         match MAPPED_STAGE_NUM_TO_KIND_ID.get(&selected_stage.stage_num) {
//                             Some(kind_table) => {
//                                 match kind_table.get(kind) {
//                                     Some(kind_stage_id) => stage_id = *kind_stage_id,
//                                     None => {}
//                                 }
//                             },
//                             None => {}
//                         }
//                     },
//                     None => {}
//                 }

//                 match v.hazard {
//                     HazardState::Off => hazards_on = false,
//                     HazardState::On => hazards_on = true,
//                     HazardState::Random => hazards_on = rand::thread_rng().gen_bool(0.5),
//                     HazardState::Default => {},
//                 }
//             },
//             None => {}
//         }
//     }

//     call_original!(stage_morph_id, stage_id, ui_bgm_id, hazards_on, mc_biomes_type);
// }

// Inline context hook instead to avoid the place HDR hooks
#[skyline::hook(offset = 0x178ab60 + (4 * 5), inline)]
unsafe fn setup_stage_offseted(
    ctx: &mut InlineCtx
    // stage_morph_id: u64, -> $x0
    // mut stage_id: u32,   -> $w1
    // ui_bgm_id: u64,      -> $x2
    // mut hazards_on: bool,-> $w3
    // mc_biomes_type: u32  -> $w4
){
    let stage_morph_id = *ctx.registers[0].x.as_ref();
    let stage_id = *ctx.registers[1].w.as_ref();
    let hazards_on = *ctx.registers[3].w.as_ref();

    if STAGE_IS_RANDOM[stage_morph_id as usize] {
        let selected_stage = STAGE_TABLE.table[stage_id as usize];
        let param_hash = selected_stage.param_name_hash.0;
        
        match STAGE_PARAM_CONFIG_DATA.0.get(&param_hash) {
            Some(v) => {
                // Set stage kind (normal, bf, fd)
                match v.valid_types.choose(&mut rand::thread_rng()) {
                    Some(kind) => {
                        match MAPPED_STAGE_NUM_TO_KIND_ID.get(&selected_stage.stage_num) {
                            Some(kind_table) => {
                                println!("{:?}", kind_table);
                                match kind_table.get(kind) {
                                    Some(kind_stage_id) => *ctx.registers[1].w.as_mut() = *kind_stage_id,
                                    None => {}
                                }
                            },
                            None => {}
                        }
                    },
                    None => {}
                }

                match v.hazard {
                    HazardState::Off => *ctx.registers[3].w.as_mut() = false as u32,
                    HazardState::On => *ctx.registers[3].w.as_mut() = true as u32,
                    HazardState::Random => *ctx.registers[3].w.as_mut() = rand::thread_rng().gen_bool(0.5) as u32,
                    HazardState::Default => {},
                }
            },
            None => {}
        }
    }

    // call_original!(stage_morph_id, stage_id, ui_bgm_id, hazards_on, mc_biomes_type);
}

#[skyline::main(name = "stage-random-form-config")]
pub fn main() {
    
    match DataConfig::load() {
        Some(res) => {
            unsafe {
                *STAGE_PARAM_CONFIG_DATA = res;
                for x in 0..STAGE_TABLE.table.len() {
                    let stage_num = STAGE_TABLE.table[x].stage_num;
                    if !MAPPED_STAGE_NUM_TO_KIND_ID.contains_key(&stage_num){
                        MAPPED_STAGE_NUM_TO_KIND_ID.insert(stage_num.clone(), HashMap::new());
                    }
                    MAPPED_STAGE_NUM_TO_KIND_ID.get_mut(&stage_num).unwrap().insert(STAGE_TABLE.table[x].stage_kind, STAGE_TABLE.table[x].stage_id);
                }
                println!("{:?}", STAGE_PARAM_CONFIG_DATA);
            }
            // install hooks
            install_hooks!(setup_stage_offseted, check_selection_type, set_selected_stage_hash);
        },
        None => {
            println!("Failed getting config! Not doing anything else.");
        }
    }
}
