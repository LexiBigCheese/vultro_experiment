use super::{dvle::DVLE, dvlp::DVLP, Error::UnexpectedEof as EOF};

pub struct DVLB {
    pub dvlp: DVLP,
    pub dvles: Vec<DVLE>
}

impl DVLB {
    pub fn parse_file(file: &[u8]) -> Result<DVLB,super::Error> {
        let file32_entire = bytemuck::try_cast_slice::<u8,u32>(file)?;
        let num_dvle = (*file32_entire.get(1).ok_or(EOF)?) as usize;
        let file32 = &file32_entire[2..];
        let (dvles_data,dvlp_data) = file32.split_at_checked(num_dvle).ok_or(EOF)?;
        let dvlp = DVLP::parse_dvlp(dvlp_data)?;
        let mut dvles = Vec::with_capacity(num_dvle);
        for i in 0..num_dvle {
            let offset_from_file32_entire = *dvles_data.get(i).ok_or(EOF)? as usize / 4;
            let dvle_data = file32_entire.split_at_checked(offset_from_file32_entire).ok_or(EOF)?.1;
            let dvle = DVLE::parse_dvle(dvle_data)?;
            dvles.push(dvle);
        }
        Ok(DVLB {
            dvlp,
            dvles
        })
    }
}
