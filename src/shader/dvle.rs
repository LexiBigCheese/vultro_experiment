use bytemuck::{Pod, Zeroable};

use super::{Error::UnexpectedEof as EOF, GshMode, Kind};

pub struct DVLE {
    ///If this is set, this is a Geometry Shader
    pub(crate) geom: Option<DVLEGeom>,
    pub(crate) symbol_to_uniform: std::collections::HashMap<String, UniformEntry>
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
            symbol_to_uniform.insert(sym.to_string(),UniformEntry {
                start_reg: u.start_reg,
                end_reg: u.end_reg
            });
        }
        todo!()
    }
}
