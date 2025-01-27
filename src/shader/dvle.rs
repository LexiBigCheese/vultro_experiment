use bytemuck::{Pod, Zeroable};

use super::{Error::UnexpectedEof as EOF, GshMode, Kind};

pub struct DVLE {
    ///If this is set, this is a Geometry Shader
    pub(crate) geom: Option<DVLEGeom>,
    pub(crate) symbol_to_uniform: std::collections::HashMap<String, UniformEntry>,
}

#[derive(Clone, Copy)]
pub(crate) struct DVLEGeom {
    pub(crate) mode: GshMode,
    pub(crate) fixed_vertex_start: u8,
    pub(crate) variable_vertex_num: u8,
    pub(crate) fixed_vertex_num: u8,
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct ConstEntryRaw {
    kind: u16,
    id: u16,
    data: [u32; 4],
}

#[derive(Clone, Copy)]
struct ConstEntry {
    //TODO
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct OutEntryRaw {
    kind: u16,
    reg_id: u16,
    ///Make sure to `& 0xFF` because this is actually a byte!
    out_mask: u32,
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct UniformEntryRaw {
    symbol_offset: u32,
    start_reg: u16,
    end_reg: u16,
}

#[derive(Clone, Copy)]
pub struct UniformEntry {
    start_reg: u16,
    end_reg: u16,
}

impl DVLE {
    pub fn parse_dvle(data: &[u32]) -> Result<DVLE, super::Error> {
        let gd = |offset: usize| data.get(offset).map(|x| *x).ok_or(EOF);
        let sac = |offset: usize| data.split_at_checked(offset).ok_or(EOF);
        let kind = if ((gd(1)? >> 16) & 0xFF) == 1 {
            super::Kind::Geometry
        } else {
            super::Kind::Vertex
        };
        let merge_outmaps = ((gd(1)? >> 24) & 1) == 1;
        let main_offset = gd(2)?;
        let main_end_offset = gd(3)?;
        let geom = if let Kind::Geometry = kind {
            let gsh = gd(5)?;
            Some(DVLEGeom {
                mode: super::GshMode::try_from(gsh & 0xFF)?,
                fixed_vertex_start: ((gsh >> 8) & 0xFF) as u8,
                variable_vertex_num: ((gsh >> 16) & 0xFF) as u8,
                fixed_vertex_num: ((gsh >> 24) & 0xFF) as u8,
            })
        } else {
            None
        };
        let const_table_start_in_bytes = gd(6)? as usize;
        let const_table_start = const_table_start_in_bytes / 4;
        let const_table_size = gd(7)? as usize;
        let const_table: &[ConstEntryRaw] = bytemuck::try_cast_slice(sac(const_table_start)?.1)?
            .split_at_checked(const_table_size)
            .ok_or(EOF)?
            .0;
        let out_table_start_in_bytes = gd(10)? as usize;
        let out_table_start = out_table_start_in_bytes / 4;
        let out_table_size = gd(11)? as usize;
        let out_table: &[OutEntryRaw] = bytemuck::try_cast_slice(sac(out_table_start)?.1)?
            .split_at_checked(out_table_size)
            .ok_or(EOF)?
            .0;

        let uniform_table_start_in_bytes = gd(12)? as usize;
        let uniform_table_start = uniform_table_start_in_bytes / 4;
        let uniform_table_size = gd(13)? as usize;
        let uniform_table: &[UniformEntryRaw] =
            bytemuck::try_cast_slice(sac(uniform_table_start)?.1)?
                .split_at_checked(uniform_table_size)
                .ok_or(EOF)?
                .0;
        //Now we build a PROPER symbol table by SAFELY CHECKING the symbol table that we have.
        let symbol_table_start_in_bytes = gd(14)? as usize;
        let symbol_table_start = symbol_table_start_in_bytes / 4;
        let symbol_table_raw: &[u8] = bytemuck::try_cast_slice(sac(symbol_table_start)?.1)?;
        let mut symbol_to_uniform: std::collections::HashMap<String, UniformEntry> =
            std::collections::HashMap::new();
        for u in uniform_table {
            let sym = symbol_table_raw
                .split_at_checked(u.symbol_offset as usize)
                .ok_or(EOF)?
                .1;
            let sym = sym.split_once(|x| *x == 0).ok_or(EOF)?.0;
            let sym = std::str::from_utf8(sym)?;
            symbol_to_uniform.insert(sym.to_string(), UniformEntry {
                start_reg: u.start_reg,
                end_reg: u.end_reg,
            });
        }
        //Now we construct the OutMap
        //[0] is total, [1] is GPUREG_SH_OUTMAP_TOTAL | CONSECUTIVE_WRITES | extra_params(7)
        let mut outmap = [0x1F1F1F1Fu32; 10];
        let mut outmap_total = 0u32;
        outmap[1] = ctru_sys::GPUREG_SH_OUTMAP_TOTAL
            | crate::gpucmd::CONSECUTIVE_WRITING
            | crate::gpucmd::extra_params(7);
        let mut outmap_mask = 0u32;
        let mut outmap_mode = 0u32;
        let mut outmap_clock = 0u32;
        for entry in out_table {
            let OutEntryRaw {
                kind,
                reg_id,
                out_mask,
            } = entry;
            let reg_id = *reg_id as u32;
            let out = &mut outmap[(reg_id as usize) + 2];
            if outmap_mask & (1 << reg_id) == 0 {
                outmap_mask |= 1 << reg_id;
                outmap_total += 1;
            }
            let mut sem = 0x1F;
            let mut num = 0;
            use ctru_sys::*;
            match *kind as u8 {
                // case RESULT_POSITION:   sem = 0x00; num = 4;                                                     break;
                RESULT_POSITION => {
                    sem = 0x00;
                    num = 4;
                }
                // case RESULT_NORMALQUAT: sem = 0x04; num = 4; dvle->outmapClock |= BIT(24);                       break;
                RESULT_NORMALQUAT => {
                    sem = 0x04;
                    num = 4;
                    outmap_clock |= 1 << 24;
                }
                // case RESULT_COLOR:      sem = 0x08; num = 4; dvle->outmapClock |= BIT(1);                        break;
                RESULT_COLOR => {
                    sem = 0x08;
                    num = 4;
                    outmap_clock |= 1 << 1;
                }
                // case RESULT_TEXCOORD0:  sem = 0x0C; num = 2; dvle->outmapClock |= BIT(8);  dvle->outmapMode = 1; break;
                RESULT_TEXCOORD0 => {
                    sem = 0x0C;
                    num = 2;
                    outmap_clock |= 1 << 8;
                    outmap_mode = 1
                }
                // case RESULT_TEXCOORD0W: sem = 0x10; num = 1; dvle->outmapClock |= BIT(16); dvle->outmapMode = 1; break;
                RESULT_TEXCOORD0W => {
                    sem = 0x10;
                    num = 1;
                    outmap_clock |= 1 << 16;
                    outmap_mode = 1
                }
                // case RESULT_TEXCOORD1:  sem = 0x0E; num = 2; dvle->outmapClock |= BIT(9);  dvle->outmapMode = 1; break;
                RESULT_TEXCOORD1 => {
                    sem = 0x0E;
                    num = 2;
                    outmap_clock |= 1 << 9;
                    outmap_mode = 1
                }
                // case RESULT_TEXCOORD2:  sem = 0x16; num = 2; dvle->outmapClock |= BIT(10); dvle->outmapMode = 1; break;
                RESULT_TEXCOORD2 => {
                    sem = 0x16;
                    num = 2;
                    outmap_clock |= 1 << 10;
                    outmap_mode = 1
                }
                // case RESULT_VIEW:       sem = 0x12; num = 3; dvle->outmapClock |= BIT(24);                       break;
                RESULT_VIEW => {
                    sem = 0x12;
                    num = 3;
                    outmap_clock |= 1 << 24;
                }
                // default: continue;
                _ => (),
            }
            let mut j = 0;
            let mut k = 0;
            while j < 4 && k < num {
                if (outmap_mask & (1 << j)) != 0 {
                    *out &= (0xFF << (j * 8)) ^ 0xFFFFFFFF;
                    *out |= sem << (j * 8);

                    k += 1;
                    if *kind == (RESULT_POSITION as u16) && k == 3 {
                        outmap_clock |= 1;
                    }
                }
                j += 1;
            }
        }
        outmap[0] = outmap_total;
        todo!()
    }
}
