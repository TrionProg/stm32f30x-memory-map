use ::stm32::register::{RW32,WO32};
use ::stm32::timer::{TimerRBTrait,BasicTimerRBTrait, UIFRemapRBTrait};

pub struct TimerRB{
    # [ doc = "0x00 - control register 1" ]
    pub cr1: RW32,
    # [ doc = "0x04 - control register 2" ]
    pub cr2: RW32,
    _reserved0: [u8; 4usize],
    # [ doc = "0x0c - DMA/Interrupt enable register" ]
    pub dier: RW32,
    # [ doc = "0x10 - status register" ]
    pub sr: RW32,
    # [ doc = "0x14 - event generation register" ]
    pub egr: WO32,
    _reserved1: [u8; 12usize],
    # [ doc = "0x24 - counter" ]
    pub cnt: RW32,
    # [ doc = "0x28 - prescaler" ]
    pub psc: RW32,
    # [ doc = "0x2c - auto-reload register" ]
    pub arr: RW32,
}

const ADDRESS: usize = 0x40001000;

impl TimerRBTrait for TimerRB{
    //Self
    fn get_rb() -> &'static Self{
        unsafe { &*(ADDRESS as *const Self) }
    }

    fn get_rb_mut() -> &'static mut Self{
        unsafe{ &mut *(ADDRESS as *mut Self) }
    }
}

impl BasicTimerRBTrait for TimerRB{
    //cr1
    fn set_cen(value:u32){
        Self::get_rb_mut().cr1.set_bits(0,0b1,value);
    }

    fn get_cen() -> u32{
        Self::get_rb().cr1.get_bits(0,0b1)
    }

    /// One-pulse mode
    fn set_opm(value:u32){
        Self::get_rb_mut().cr1.set_bits(3,0b1,value);
    }

    fn get_opm() -> u32{
        Self::get_rb().cr1.get_bits(3,0b1)
    }

    /// Auto-reload preload enable
    fn set_arpe(value:u32){
        Self::get_rb_mut().cr1.set_bits(7,0b1,value);
    }

    fn get_arpe() -> u32{
        Self::get_rb().cr1.get_bits(7,0b1)
    }

    //psc
    fn set_psc(value:u32){
        Self::get_rb_mut().psc.set_bits(0,0xFFFF,value);
    }

    fn get_psc() -> u32{
        Self::get_rb().psc.get_bits(0,0xFFFF)
    }

    //dier
    fn set_uie(value:u32){
        Self::get_rb_mut().dier.set_bits(0,0b1,value);
    }

    fn get_uie() -> u32{
        Self::get_rb().dier.get_bits(0,0b1)
    }

    fn set_ude(value:u32){
        Self::get_rb_mut().dier.set_bits(8,0b1,value);
    }

    fn get_ude() -> u32{
        Self::get_rb().dier.get_bits(8,0b1)
    }

    //sr
    fn reset_uif(){
        Self::get_rb_mut().sr.set_bits(0,0b1,0);
    }

    fn get_uif() -> u32{
        Self::get_rb().sr.get_bits(0,0b1)
    }

    //egr
    fn set_ug(value:u32){
        Self::get_rb_mut().egr.set_bits(0,0b1,value);
    }

    //arr
    fn set_arr(value:u32){
        Self::get_rb_mut().arr.set_bits(0,0xFFFF,value);
    }

    fn get_arr() -> u32{
        Self::get_rb().arr.get_bits(0,0xFFFF)
    }
}

impl UIFRemapRBTrait for TimerRB{
    fn set_uifremap(value:u32){
        Self::get_rb_mut().cr1.set_bits(11,0b1,value);
    }

    fn get_uifremap() -> u32{
        Self::get_rb().cr1.get_bits(11,0b1)
    }

    fn get_uif_copy() -> u32{
        Self::get_rb().cnt.get_bits(31,0b1)
    }
}


impl TimerRB{
    /// Update disable
    pub fn set_udis(value:u32){
        Self::get_rb_mut().cr1.set_bits(1,0b1,value);
    }

    pub fn get_udis() -> u32{
        Self::get_rb().cr1.get_bits(1,0b1)
    }

    /// Update request source
    pub fn set_urs(value:u32){
        Self::get_rb_mut().cr1.set_bits(2,0b1,value);
    }

    pub fn get_urs() -> u32{
        Self::get_rb().cr1.get_bits(2,0b1)
    }

    //cr2
    /// Master mode selection
    pub fn set_mms(value:u32){
        Self::get_rb_mut().cr2.set_bits(4,0b111,value);
    }

    pub fn get_mms() -> u32{
        Self::get_rb().cr2.get_bits(4,0b111)
    }

    //cnt
    /// Counter value
    pub fn set_cnt(value:u32){
        Self::get_rb_mut().cnt.set_bits(0,0xFFFF,value);
    }

    pub fn get_cnt() -> u32{
        Self::get_rb().cnt.get_bits(0,0xFFFF)
    }
}
