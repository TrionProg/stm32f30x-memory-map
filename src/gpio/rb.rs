#[macro_export]
macro_rules! create_rb{
    ($address:expr) => (
        # [ doc = "General-purpose I/Os" ]
        # [ repr ( C ) ]
        pub struct GpioRB {
            # [ doc = "0x00 - GPIO port mode register" ]
            pub moder:RW32,
            # [ doc = "0x04 - GPIO port output type register" ]
            pub otyper:RW32,
            # [ doc = "0x08 - GPIO port output speed register" ]
            pub ospeedr:RW32,
            # [ doc = "0x0c - GPIO port pull-up/pull-down register" ]
            pub pupdr:RW32,
            # [ doc = "0x10 - GPIO port input data register" ]
            pub idr:RO32,
            # [ doc = "0x14 - GPIO port output data register" ]
            pub odr:RW32,
            # [ doc = "0x18 - GPIO port bit set/reset register" ]
            pub bsrr:WO32,
            # [ doc = "0x1c - GPIO port configuration lock register" ]
            pub lckr:RW32,
            # [ doc = "0x20 - GPIO alternate function low register" ]
            pub afrl:RW32,
            # [ doc = "0x24 - GPIO alternate function high register" ]
            pub afrh:RW32,
            # [ doc = "0x28 - Port bit reset register" ]
            pub brr:WO32,
        }

        impl GpioRBTrait for GpioRB{
            //Self
            fn get_gpio_rb() -> &'static Self{
                unsafe { &*($address as *const Self) }
            }

            fn get_gpio_rb_mut() -> &'static mut Self{
                unsafe{ &mut *($address as *mut Self) }
            }

            //Moder
            fn set_moder(index:u32,value:u32){
                Self::get_gpio_rb_mut().moder.set_bits(index*2,0b11,value);
            }

            fn get_moder(index:u32) -> u32{
                Self::get_gpio_rb().moder.get_bits(index*2,0b11)
            }

            //Otyper
            fn set_ot(index:u32,value:u32){
                Self::get_gpio_rb_mut().otyper.set_bits(index,0b1,value);
            }

            fn get_ot(index:u32) -> u32{
                Self::get_gpio_rb().otyper.get_bits(index,0b1)
            }

            //Ospeedr
            fn set_ospeedr(index:u32,value:u32){
                Self::get_gpio_rb_mut().ospeedr.set_bits(index*2,0b11,value);
            }

            fn get_ospeedr(index:u32) -> u32{
                Self::get_gpio_rb().ospeedr.get_bits(index*2,0b11)
            }

            //Pupdr
            fn set_pupdr(index:u32,value:u32){
                Self::get_gpio_rb_mut().pupdr.set_bits(index*2,0b11,value);
            }

            fn get_pupdr(index:u32) -> u32{
                Self::get_gpio_rb().pupdr.get_bits(index*2,0b11)
            }

            //Idr
            fn get_idr(index:u32) -> u32{
                Self::get_gpio_rb().idr.get_bits(index,0b1)
            }

            //Odr
            fn set_odr(index:u32,value:u32){
                Self::get_gpio_rb_mut().odr.set_bits(index,0b1,value);
            }

            fn get_odr(index:u32) -> u32{
                Self::get_gpio_rb().odr.get_bits(index,0b1)
            }

            //Bsrr
            fn set_bs(index:u32,value:u32){
                Self::get_gpio_rb_mut().bsrr.set_bits(index,0b1,value);
            }

            //Lckr
            fn set_lck(index:u32,value:u32){
                Self::get_gpio_rb_mut().lckr.set_bits(index,0b1,value);
            }

            fn get_lck(index:u32) -> u32{
                Self::get_gpio_rb().lckr.get_bits(index,0b1)
            }

            fn set_lckk(value:u32){
                Self::get_gpio_rb_mut().lckr.set_bits(16,0b1,value);
            }

            fn get_lckk() -> u32{
                Self::get_gpio_rb().lckr.get_bits(16,0b1)
            }

            //Afrl/Afrh
            fn set_afr(index:u32,value:u32){
                if index<8 {
                    Self::get_gpio_rb_mut().afrl.set_bits(index*4,0b1111,value);
                }else{
                    Self::get_gpio_rb_mut().afrh.set_bits((index-8)*4,0b1111,value);
                }
            }

            fn get_afr(index:u32) -> u32{
                if index<8 {
                    Self::get_gpio_rb().afrl.get_bits(index*4,0b1111)
                }else{
                    Self::get_gpio_rb().afrh.get_bits((index-8)*4,0b1111)
                }
            }

            //Brr
            fn set_br(index:u32,value:u32){
                Self::get_gpio_rb_mut().brr.set_bits(index,0b1,value);
            }
        }

    )
}
