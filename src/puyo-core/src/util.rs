use std::arch::x86_64::*;
//use std::arch::arm::*;

pub fn pext16(input: u16, mask: u16) -> u16 {
    if cfg!(target_feature = "bmi2") {
        unsafe {
            return _pext_u32(input as u32, mask as u32) as u16;
        }
    } else {
        //return pext16_naive(input, mask);
        return pext15_emu(input, mask);
    }
}

//https://github.com/InstLatx64/InstLatX64_Demo/blob/master/PEXT_PDEP_Emu.cpp - edit for puyo (not all mask support for performance)
pub fn pext15_emu(v: u16, m: u16) -> u16 {
    unsafe {
        let v_u32: u32 = v as u32;
        let m_u32: u32 = m as u32;
        let pc = m_u32.count_ones();
        if pc == 16 {
            return v;
        }
        //shifting 2pop ~ 6pop case is slower than now
        if pc == 15 {
            let zero_location: u16 = !m;
            let zero_location_index = zero_location.trailing_zeros() + 1;
            let left_mask = (!0u16).wrapping_shr(zero_location_index).wrapping_shl(zero_location_index);
            let right_mask = (!left_mask).wrapping_shr(1);
            let shifted_left_value = (v & left_mask).wrapping_shr(1);
            return (v & right_mask) | shifted_left_value;
        } else {
            let mut mm = _mm_cvtsi32_si128(!m_u32 as i32);
            let mtwo = _mm_set1_epi64x((!0u64 - 1) as i64);
            let mut clmul = _mm_clmulepi64_si128(mm, mtwo, 0);
            let bit0 = _mm_cvtsi128_si32(clmul) as u32;
            let mut a = v_u32 & m_u32;
            a = (!bit0 & a) | ((bit0 & a).wrapping_shr(1));
            mm = _mm_and_si128(mm, clmul);
            clmul = _mm_clmulepi64_si128(mm, mtwo, 0);
            let bit1 = _mm_cvtsi128_si32(clmul) as u32;
            a = (!bit1 & a) | ((bit1 & a).wrapping_shr(2));
            mm = _mm_and_si128(mm, clmul);
            clmul = _mm_clmulepi64_si128(mm, mtwo, 0);
            let bit2 = _mm_cvtsi128_si32(clmul) as u32;
            a = (!bit2 & a) | ((bit2 & a).wrapping_shr(4));
            mm = _mm_and_si128(mm, clmul);
            clmul = _mm_clmulepi64_si128(mm, mtwo, 0);
            let bit3 = _mm_cvtsi128_si32(clmul) as u32;
            return ((!bit3 & a) | ((bit3 & a).wrapping_shr(8))) as u16;
        }
    }
}

pub fn pext16_naive(input: u16, mut mask: u16) -> u16 {
    let mut result: u16 = 0;
    let mut bb: u16 = 1;
    while mask != 0 {
        if input & mask & ((!mask).wrapping_add(1)) != 0 {
            result |= bb;
        }
        mask &= mask.wrapping_sub(1);
        bb = bb.wrapping_add(bb);
    }
    return result;
}
