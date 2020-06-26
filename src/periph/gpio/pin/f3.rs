//! General-purpose I/O port pins.
//! for STM32F3 Series of mixed-signal MCUs with DSP and FPU instructions.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic GPIO pin peripheral variant.
    pub trait GpioPinMap {
        /// GPIO port head peripheral variant.
        type GpioHeadMap: super::super::head::GpioHeadMap;
    }

    /// Generic GPIO pin peripheral.
    pub struct GpioPinPeriph;

    GPIO {
        BRR {
            0x20 WoReg Shared;
            BR { WoWoRegFieldBit }
        }
        BSRR {
            0x20 WoReg Shared;
            BR { WoWoRegFieldBit }
            BS { WoWoRegFieldBit }
        }
        IDR {
            0x20 RoReg Shared;
            IDR { RoRoRegFieldBit }
        }
        LCKR {
            0x20 RwReg Shared;
            LCK { RwRwRegFieldBit }
        }
        ODR {
            0x20 RwReg Shared;
            ODR { RwRwRegFieldBit }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_gpio_pin {
    (
        $port_ty:ident,
        $pin_macro_doc:expr,
        $pin_macro:ident,
        $pin_ty_doc:expr,
        $pin_ty:ident,
        $gpio:ident,
        $afr_ty:ident,
        $asc_ty:ident,
        $br_ty:ident,
        $bs_ty:ident,
        $cnf_ty:ident,
        $mode_ty:ident,
        $idr_ty:ident,
        $lck_ty:ident,
        $moder_ty:ident,
        $odr_ty:ident,
        $ospeedr_ty:ident,
        $ot_ty:ident,
        $pupdr_ty:ident,
        $afr_path:ident,
        $cr_path:ident,($($ascr_option:ident)*),
    ) => {
        periph::map! {
            #[doc = $pin_macro_doc]
            pub macro $pin_macro;

            #[doc = $pin_ty_doc]
            pub struct $pin_ty;

            impl GpioPinMap for $pin_ty {
                //type GpioHeadMap = super::head::$port_ty;
                type GpioHeadMap = super::super::head::$port_ty;
            }

            drone_stm32_map_pieces::reg;
            crate::pin;

            GPIO {
                $gpio;
                BRR {
                    BRR Shared;
                    BR { $br_ty }
                }
                BSRR {
                    BSRR Shared;
                    BR { $br_ty }
                    BS { $bs_ty }
                }
                IDR {
                    IDR Shared;
                    IDR { $idr_ty }
                }
                LCKR {
                    LCKR Shared;
                    LCK { $lck_ty }
                }
                ODR {
                    ODR Shared;
                    ODR { $odr_ty }
                }
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! map_gpio_pins {
    (
        $port_ty:ident,
        $pin0_macro_doc:expr,
        $pin0_macro:ident,
        $pin0_ty_doc:expr,
        $pin0_ty:ident,
        $pin1_macro_doc:expr,
        $pin1_macro:ident,
        $pin1_ty_doc:expr,
        $pin1_ty:ident,
        $pin2_macro_doc:expr,
        $pin2_macro:ident,
        $pin2_ty_doc:expr,
        $pin2_ty:ident,
        $pin3_macro_doc:expr,
        $pin3_macro:ident,
        $pin3_ty_doc:expr,
        $pin3_ty:ident,
        $pin4_macro_doc:expr,
        $pin4_macro:ident,
        $pin4_ty_doc:expr,
        $pin4_ty:ident,
        $pin5_macro_doc:expr,
        $pin5_macro:ident,
        $pin5_ty_doc:expr,
        $pin5_ty:ident,
        $pin6_macro_doc:expr,
        $pin6_macro:ident,
        $pin6_ty_doc:expr,
        $pin6_ty:ident,
        $pin7_macro_doc:expr,
        $pin7_macro:ident,
        $pin7_ty_doc:expr,
        $pin7_ty:ident,
        $pin8_macro_doc:expr,
        $pin8_macro:ident,
        $pin8_ty_doc:expr,
        $pin8_ty:ident,
        $pin9_macro_doc:expr,
        $pin9_macro:ident,
        $pin9_ty_doc:expr,
        $pin9_ty:ident,
        $pin10_macro_doc:expr,
        $pin10_macro:ident,
        $pin10_ty_doc:expr,
        $pin10_ty:ident,
        $pin11_macro_doc:expr,
        $pin11_macro:ident,
        $pin11_ty_doc:expr,
        $pin11_ty:ident,
        $pin12_macro_doc:expr,
        $pin12_macro:ident,
        $pin12_ty_doc:expr,
        $pin12_ty:ident,
        $pin13_macro_doc:expr,
        $pin13_macro:ident,
        $pin13_ty_doc:expr,
        $pin13_ty:ident,
        $pin14_macro_doc:expr,
        $pin14_macro:ident,
        $pin14_ty_doc:expr,
        $pin14_ty:ident,
        $pin15_macro_doc:expr,
        $pin15_macro:ident,
        $pin15_ty_doc:expr,
        $pin15_ty:ident,
        $gpio:ident,
        ($($ascr_option:ident)*),
    ) => {
        map_gpio_pin! {
            $port_ty,
            $pin0_macro_doc,
            $pin0_macro,
            $pin0_ty_doc,
            $pin0_ty,
            $gpio,
            AFRL0,
            ASC0,
            BR0,
            BS0,
            CNF0,
            MODE0,
            IDR0,
            LCK0,
            MODER0,
            ODR0,
            OSPEEDR0,
            OT0,
            PUPDR0,
            AFRL,
            CRL,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin1_macro_doc,
            $pin1_macro,
            $pin1_ty_doc,
            $pin1_ty,
            $gpio,
            AFRL1,
            ASC1,
            BR1,
            BS1,
            CNF1,
            MODE1,
            IDR1,
            LCK1,
            MODER1,
            ODR1,
            OSPEEDR1,
            OT1,
            PUPDR1,
            AFRL,
            CRL,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin2_macro_doc,
            $pin2_macro,
            $pin2_ty_doc,
            $pin2_ty,
            $gpio,
            AFRL2,
            ASC2,
            BR2,
            BS2,
            CNF2,
            MODE2,
            IDR2,
            LCK2,
            MODER2,
            ODR2,
            OSPEEDR2,
            OT2,
            PUPDR2,
            AFRL,
            CRL,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin3_macro_doc,
            $pin3_macro,
            $pin3_ty_doc,
            $pin3_ty,
            $gpio,
            AFRL3,
            ASC3,
            BR3,
            BS3,
            CNF3,
            MODE3,
            IDR3,
            LCK3,
            MODER3,
            ODR3,
            OSPEEDR3,
            OT3,
            PUPDR3,
            AFRL,
            CRL,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin4_macro_doc,
            $pin4_macro,
            $pin4_ty_doc,
            $pin4_ty,
            $gpio,
            AFRL4,
            ASC4,
            BR4,
            BS4,
            CNF4,
            MODE4,
            IDR4,
            LCK4,
            MODER4,
            ODR4,
            OSPEEDR4,
            OT4,
            PUPDR4,
            AFRL,
            CRL,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin5_macro_doc,
            $pin5_macro,
            $pin5_ty_doc,
            $pin5_ty,
            $gpio,
            AFRL5,
            ASC5,
            BR5,
            BS5,
            CNF5,
            MODE5,
            IDR5,
            LCK5,
            MODER5,
            ODR5,
            OSPEEDR5,
            OT5,
            PUPDR5,
            AFRL,
            CRL,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin6_macro_doc,
            $pin6_macro,
            $pin6_ty_doc,
            $pin6_ty,
            $gpio,
            AFRL6,
            ASC6,
            BR6,
            BS6,
            CNF6,
            MODE6,
            IDR6,
            LCK6,
            MODER6,
            ODR6,
            OSPEEDR6,
            OT6,
            PUPDR6,
            AFRL,
            CRL,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin7_macro_doc,
            $pin7_macro,
            $pin7_ty_doc,
            $pin7_ty,
            $gpio,
            AFRL7,
            ASC7,
            BR7,
            BS7,
            CNF7,
            MODE7,
            IDR7,
            LCK7,
            MODER7,
            ODR7,
            OSPEEDR7,
            OT7,
            PUPDR7,
            AFRL,
            CRL,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin8_macro_doc,
            $pin8_macro,
            $pin8_ty_doc,
            $pin8_ty,
            $gpio,
            AFRH8,
            ASC8,
            BR8,
            BS8,
            CNF8,
            MODE8,
            IDR8,
            LCK8,
            MODER8,
            ODR8,
            OSPEEDR8,
            OT8,
            PUPDR8,
            AFRH,
            CRH,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin9_macro_doc,
            $pin9_macro,
            $pin9_ty_doc,
            $pin9_ty,
            $gpio,
            AFRH9,
            ASC9,
            BR9,
            BS9,
            CNF9,
            MODE9,
            IDR9,
            LCK9,
            MODER9,
            ODR9,
            OSPEEDR9,
            OT9,
            PUPDR9,
            AFRH,
            CRH,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin10_macro_doc,
            $pin10_macro,
            $pin10_ty_doc,
            $pin10_ty,
            $gpio,
            AFRH10,
            ASC10,
            BR10,
            BS10,
            CNF10,
            MODE10,
            IDR10,
            LCK10,
            MODER10,
            ODR10,
            OSPEEDR10,
            OT10,
            PUPDR10,
            AFRH,
            CRH,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin11_macro_doc,
            $pin11_macro,
            $pin11_ty_doc,
            $pin11_ty,
            $gpio,
            AFRH11,
            ASC11,
            BR11,
            BS11,
            CNF11,
            MODE11,
            IDR11,
            LCK11,
            MODER11,
            ODR11,
            OSPEEDR11,
            OT11,
            PUPDR11,
            AFRH,
            CRH,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin12_macro_doc,
            $pin12_macro,
            $pin12_ty_doc,
            $pin12_ty,
            $gpio,
            AFRH12,
            ASC12,
            BR12,
            BS12,
            CNF12,
            MODE12,
            IDR12,
            LCK12,
            MODER12,
            ODR12,
            OSPEEDR12,
            OT12,
            PUPDR12,
            AFRH,
            CRH,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin13_macro_doc,
            $pin13_macro,
            $pin13_ty_doc,
            $pin13_ty,
            $gpio,
            AFRH13,
            ASC13,
            BR13,
            BS13,
            CNF13,
            MODE13,
            IDR13,
            LCK13,
            MODER13,
            ODR13,
            OSPEEDR13,
            OT13,
            PUPDR13,
            AFRH,
            CRH,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin14_macro_doc,
            $pin14_macro,
            $pin14_ty_doc,
            $pin14_ty,
            $gpio,
            AFRH14,
            ASC14,
            BR14,
            BS14,
            CNF14,
            MODE14,
            IDR14,
            LCK14,
            MODER14,
            ODR14,
            OSPEEDR14,
            OT14,
            PUPDR14,
            AFRH,
            CRH,
            ($($ascr_option)*),
        }
        map_gpio_pin! {
            $port_ty,
            $pin15_macro_doc,
            $pin15_macro,
            $pin15_ty_doc,
            $pin15_ty,
            $gpio,
            AFRH15,
            ASC15,
            BR15,
            BS15,
            CNF15,
            MODE15,
            IDR15,
            LCK15,
            MODER15,
            ODR15,
            OSPEEDR15,
            OT15,
            PUPDR15,
            AFRH,
            CRH,
            ($($ascr_option)*),
        }
    };
}

map_gpio_pins! {
    GpioAHead,
    "Extracts GPIO port A pin 0 register tokens.",
    periph_gpio_a0,
    "GPIO port A pin 0 peripheral variant.",
    GpioA0,
    "Extracts GPIO port A pin 1 register tokens.",
    periph_gpio_a1,
    "GPIO port A pin 1 peripheral variant.",
    GpioA1,
    "Extracts GPIO port A pin 2 register tokens.",
    periph_gpio_a2,
    "GPIO port A pin 2 peripheral variant.",
    GpioA2,
    "Extracts GPIO port A pin 3 register tokens.",
    periph_gpio_a3,
    "GPIO port A pin 3 peripheral variant.",
    GpioA3,
    "Extracts GPIO port A pin 4 register tokens.",
    periph_gpio_a4,
    "GPIO port A pin 4 peripheral variant.",
    GpioA4,
    "Extracts GPIO port A pin 5 register tokens.",
    periph_gpio_a5,
    "GPIO port A pin 5 peripheral variant.",
    GpioA5,
    "Extracts GPIO port A pin 6 register tokens.",
    periph_gpio_a6,
    "GPIO port A pin 6 peripheral variant.",
    GpioA6,
    "Extracts GPIO port A pin 7 register tokens.",
    periph_gpio_a7,
    "GPIO port A pin 7 peripheral variant.",
    GpioA7,
    "Extracts GPIO port A pin 8 register tokens.",
    periph_gpio_a8,
    "GPIO port A pin 8 peripheral variant.",
    GpioA8,
    "Extracts GPIO port A pin 9 register tokens.",
    periph_gpio_a9,
    "GPIO port A pin 9 peripheral variant.",
    GpioA9,
    "Extracts GPIO port A pin 10 register tokens.",
    periph_gpio_a10,
    "GPIO port A pin 10 peripheral variant.",
    GpioA10,
    "Extracts GPIO port A pin 11 register tokens.",
    periph_gpio_a11,
    "GPIO port A pin 11 peripheral variant.",
    GpioA11,
    "Extracts GPIO port A pin 12 register tokens.",
    periph_gpio_a12,
    "GPIO port A pin 12 peripheral variant.",
    GpioA12,
    "Extracts GPIO port A pin 13 register tokens.",
    periph_gpio_a13,
    "GPIO port A pin 13 peripheral variant.",
    GpioA13,
    "Extracts GPIO port A pin 14 register tokens.",
    periph_gpio_a14,
    "GPIO port A pin 14 peripheral variant.",
    GpioA14,
    "Extracts GPIO port A pin 15 register tokens.",
    periph_gpio_a15,
    "GPIO port A pin 15 peripheral variant.",
    GpioA15,
    GPIOA,
    (Option),
}

map_gpio_pins! {
    GpioBHead,
    "Extracts GPIO port B pin 0 register tokens.",
    periph_gpio_b0,
    "GPIO port B pin 0 peripheral variant.",
    GpioB0,
    "Extracts GPIO port B pin 1 register tokens.",
    periph_gpio_b1,
    "GPIO port B pin 1 peripheral variant.",
    GpioB1,
    "Extracts GPIO port B pin 2 register tokens.",
    periph_gpio_b2,
    "GPIO port B pin 2 peripheral variant.",
    GpioB2,
    "Extracts GPIO port B pin 3 register tokens.",
    periph_gpio_b3,
    "GPIO port B pin 3 peripheral variant.",
    GpioB3,
    "Extracts GPIO port B pin 4 register tokens.",
    periph_gpio_b4,
    "GPIO port B pin 4 peripheral variant.",
    GpioB4,
    "Extracts GPIO port B pin 5 register tokens.",
    periph_gpio_b5,
    "GPIO port B pin 5 peripheral variant.",
    GpioB5,
    "Extracts GPIO port B pin 6 register tokens.",
    periph_gpio_b6,
    "GPIO port B pin 6 peripheral variant.",
    GpioB6,
    "Extracts GPIO port B pin 7 register tokens.",
    periph_gpio_b7,
    "GPIO port B pin 7 peripheral variant.",
    GpioB7,
    "Extracts GPIO port B pin 8 register tokens.",
    periph_gpio_b8,
    "GPIO port B pin 8 peripheral variant.",
    GpioB8,
    "Extracts GPIO port B pin 9 register tokens.",
    periph_gpio_b9,
    "GPIO port B pin 9 peripheral variant.",
    GpioB9,
    "Extracts GPIO port B pin 10 register tokens.",
    periph_gpio_b10,
    "GPIO port B pin 10 peripheral variant.",
    GpioB10,
    "Extracts GPIO port B pin 11 register tokens.",
    periph_gpio_b11,
    "GPIO port B pin 11 peripheral variant.",
    GpioB11,
    "Extracts GPIO port B pin 12 register tokens.",
    periph_gpio_b12,
    "GPIO port B pin 12 peripheral variant.",
    GpioB12,
    "Extracts GPIO port B pin 13 register tokens.",
    periph_gpio_b13,
    "GPIO port B pin 13 peripheral variant.",
    GpioB13,
    "Extracts GPIO port B pin 14 register tokens.",
    periph_gpio_b14,
    "GPIO port B pin 14 peripheral variant.",
    GpioB14,
    "Extracts GPIO port B pin 15 register tokens.",
    periph_gpio_b15,
    "GPIO port B pin 15 peripheral variant.",
    GpioB15,
    GPIOB,
    (Option),
}

map_gpio_pins! {
    GpioCHead,
    "Extracts GPIO port C pin 0 register tokens.",
    periph_gpio_c0,
    "GPIO port C pin 0 peripheral variant.",
    GpioC0,
    "Extracts GPIO port C pin 1 register tokens.",
    periph_gpio_c1,
    "GPIO port C pin 1 peripheral variant.",
    GpioC1,
    "Extracts GPIO port C pin 2 register tokens.",
    periph_gpio_c2,
    "GPIO port C pin 2 peripheral variant.",
    GpioC2,
    "Extracts GPIO port C pin 3 register tokens.",
    periph_gpio_c3,
    "GPIO port C pin 3 peripheral variant.",
    GpioC3,
    "Extracts GPIO port C pin 4 register tokens.",
    periph_gpio_c4,
    "GPIO port C pin 4 peripheral variant.",
    GpioC4,
    "Extracts GPIO port C pin 5 register tokens.",
    periph_gpio_c5,
    "GPIO port C pin 5 peripheral variant.",
    GpioC5,
    "Extracts GPIO port C pin 6 register tokens.",
    periph_gpio_c6,
    "GPIO port C pin 6 peripheral variant.",
    GpioC6,
    "Extracts GPIO port C pin 7 register tokens.",
    periph_gpio_c7,
    "GPIO port C pin 7 peripheral variant.",
    GpioC7,
    "Extracts GPIO port C pin 8 register tokens.",
    periph_gpio_c8,
    "GPIO port C pin 8 peripheral variant.",
    GpioC8,
    "Extracts GPIO port C pin 9 register tokens.",
    periph_gpio_c9,
    "GPIO port C pin 9 peripheral variant.",
    GpioC9,
    "Extracts GPIO port C pin 10 register tokens.",
    periph_gpio_c10,
    "GPIO port C pin 10 peripheral variant.",
    GpioC10,
    "Extracts GPIO port C pin 11 register tokens.",
    periph_gpio_c11,
    "GPIO port C pin 11 peripheral variant.",
    GpioC11,
    "Extracts GPIO port C pin 12 register tokens.",
    periph_gpio_c12,
    "GPIO port C pin 12 peripheral variant.",
    GpioC12,
    "Extracts GPIO port C pin 13 register tokens.",
    periph_gpio_c13,
    "GPIO port C pin 13 peripheral variant.",
    GpioC13,
    "Extracts GPIO port C pin 14 register tokens.",
    periph_gpio_c14,
    "GPIO port C pin 14 peripheral variant.",
    GpioC14,
    "Extracts GPIO port C pin 15 register tokens.",
    periph_gpio_c15,
    "GPIO port C pin 15 peripheral variant.",
    GpioC15,
    GPIOC,
    (Option),
}

map_gpio_pins! {
    GpioDHead,
    "Extracts GPIO port D pin 0 register tokens.",
    periph_gpio_d0,
    "GPIO port D pin 0 peripheral variant.",
    GpioD0,
    "Extracts GPIO port D pin 1 register tokens.",
    periph_gpio_d1,
    "GPIO port D pin 1 peripheral variant.",
    GpioD1,
    "Extracts GPIO port D pin 2 register tokens.",
    periph_gpio_d2,
    "GPIO port D pin 2 peripheral variant.",
    GpioD2,
    "Extracts GPIO port D pin 3 register tokens.",
    periph_gpio_d3,
    "GPIO port D pin 3 peripheral variant.",
    GpioD3,
    "Extracts GPIO port D pin 4 register tokens.",
    periph_gpio_d4,
    "GPIO port D pin 4 peripheral variant.",
    GpioD4,
    "Extracts GPIO port D pin 5 register tokens.",
    periph_gpio_d5,
    "GPIO port D pin 5 peripheral variant.",
    GpioD5,
    "Extracts GPIO port D pin 6 register tokens.",
    periph_gpio_d6,
    "GPIO port D pin 6 peripheral variant.",
    GpioD6,
    "Extracts GPIO port D pin 7 register tokens.",
    periph_gpio_d7,
    "GPIO port D pin 7 peripheral variant.",
    GpioD7,
    "Extracts GPIO port D pin 8 register tokens.",
    periph_gpio_d8,
    "GPIO port D pin 8 peripheral variant.",
    GpioD8,
    "Extracts GPIO port D pin 9 register tokens.",
    periph_gpio_d9,
    "GPIO port D pin 9 peripheral variant.",
    GpioD9,
    "Extracts GPIO port D pin 10 register tokens.",
    periph_gpio_d10,
    "GPIO port D pin 10 peripheral variant.",
    GpioD10,
    "Extracts GPIO port D pin 11 register tokens.",
    periph_gpio_d11,
    "GPIO port D pin 11 peripheral variant.",
    GpioD11,
    "Extracts GPIO port D pin 12 register tokens.",
    periph_gpio_d12,
    "GPIO port D pin 12 peripheral variant.",
    GpioD12,
    "Extracts GPIO port D pin 13 register tokens.",
    periph_gpio_d13,
    "GPIO port D pin 13 peripheral variant.",
    GpioD13,
    "Extracts GPIO port D pin 14 register tokens.",
    periph_gpio_d14,
    "GPIO port D pin 14 peripheral variant.",
    GpioD14,
    "Extracts GPIO port D pin 15 register tokens.",
    periph_gpio_d15,
    "GPIO port D pin 15 peripheral variant.",
    GpioD15,
    GPIOD,
    (Option),
}

map_gpio_pins! {
    GpioEHead,
    "Extracts GPIO port E pin 0 register tokens.",
    periph_gpio_e0,
    "GPIO port E pin 0 peripheral variant.",
    GpioE0,
    "Extracts GPIO port E pin 1 register tokens.",
    periph_gpio_e1,
    "GPIO port E pin 1 peripheral variant.",
    GpioE1,
    "Extracts GPIO port E pin 2 register tokens.",
    periph_gpio_e2,
    "GPIO port E pin 2 peripheral variant.",
    GpioE2,
    "Extracts GPIO port E pin 3 register tokens.",
    periph_gpio_e3,
    "GPIO port E pin 3 peripheral variant.",
    GpioE3,
    "Extracts GPIO port E pin 4 register tokens.",
    periph_gpio_e4,
    "GPIO port E pin 4 peripheral variant.",
    GpioE4,
    "Extracts GPIO port E pin 5 register tokens.",
    periph_gpio_e5,
    "GPIO port E pin 5 peripheral variant.",
    GpioE5,
    "Extracts GPIO port E pin 6 register tokens.",
    periph_gpio_e6,
    "GPIO port E pin 6 peripheral variant.",
    GpioE6,
    "Extracts GPIO port E pin 7 register tokens.",
    periph_gpio_e7,
    "GPIO port E pin 7 peripheral variant.",
    GpioE7,
    "Extracts GPIO port E pin 8 register tokens.",
    periph_gpio_e8,
    "GPIO port E pin 8 peripheral variant.",
    GpioE8,
    "Extracts GPIO port E pin 9 register tokens.",
    periph_gpio_e9,
    "GPIO port E pin 9 peripheral variant.",
    GpioE9,
    "Extracts GPIO port E pin 10 register tokens.",
    periph_gpio_e10,
    "GPIO port E pin 10 peripheral variant.",
    GpioE10,
    "Extracts GPIO port E pin 11 register tokens.",
    periph_gpio_e11,
    "GPIO port E pin 11 peripheral variant.",
    GpioE11,
    "Extracts GPIO port E pin 12 register tokens.",
    periph_gpio_e12,
    "GPIO port E pin 12 peripheral variant.",
    GpioE12,
    "Extracts GPIO port E pin 13 register tokens.",
    periph_gpio_e13,
    "GPIO port E pin 13 peripheral variant.",
    GpioE13,
    "Extracts GPIO port E pin 14 register tokens.",
    periph_gpio_e14,
    "GPIO port E pin 14 peripheral variant.",
    GpioE14,
    "Extracts GPIO port E pin 15 register tokens.",
    periph_gpio_e15,
    "GPIO port E pin 15 peripheral variant.",
    GpioE15,
    GPIOE,
    (Option),
}

map_gpio_pins! {
    GpioFHead,
    "Extracts GPIO port F pin 0 register tokens.",
    periph_gpio_f0,
    "GPIO port F pin 0 peripheral variant.",
    GpioF0,
    "Extracts GPIO port F pin 1 register tokens.",
    periph_gpio_f1,
    "GPIO port F pin 1 peripheral variant.",
    GpioF1,
    "Extracts GPIO port F pin 2 register tokens.",
    periph_gpio_f2,
    "GPIO port F pin 2 peripheral variant.",
    GpioF2,
    "Extracts GPIO port F pin 3 register tokens.",
    periph_gpio_f3,
    "GPIO port F pin 3 peripheral variant.",
    GpioF3,
    "Extracts GPIO port F pin 4 register tokens.",
    periph_gpio_f4,
    "GPIO port F pin 4 peripheral variant.",
    GpioF4,
    "Extracts GPIO port F pin 5 register tokens.",
    periph_gpio_f5,
    "GPIO port F pin 5 peripheral variant.",
    GpioF5,
    "Extracts GPIO port F pin 6 register tokens.",
    periph_gpio_f6,
    "GPIO port F pin 6 peripheral variant.",
    GpioF6,
    "Extracts GPIO port F pin 7 register tokens.",
    periph_gpio_f7,
    "GPIO port F pin 7 peripheral variant.",
    GpioF7,
    "Extracts GPIO port F pin 8 register tokens.",
    periph_gpio_f8,
    "GPIO port F pin 8 peripheral variant.",
    GpioF8,
    "Extracts GPIO port F pin 9 register tokens.",
    periph_gpio_f9,
    "GPIO port F pin 9 peripheral variant.",
    GpioF9,
    "Extracts GPIO port F pin 10 register tokens.",
    periph_gpio_f10,
    "GPIO port F pin 10 peripheral variant.",
    GpioF10,
    "Extracts GPIO port F pin 11 register tokens.",
    periph_gpio_f11,
    "GPIO port F pin 11 peripheral variant.",
    GpioF11,
    "Extracts GPIO port F pin 12 register tokens.",
    periph_gpio_f12,
    "GPIO port F pin 12 peripheral variant.",
    GpioF12,
    "Extracts GPIO port F pin 13 register tokens.",
    periph_gpio_f13,
    "GPIO port F pin 13 peripheral variant.",
    GpioF13,
    "Extracts GPIO port F pin 14 register tokens.",
    periph_gpio_f14,
    "GPIO port F pin 14 peripheral variant.",
    GpioF14,
    "Extracts GPIO port F pin 15 register tokens.",
    periph_gpio_f15,
    "GPIO port F pin 15 peripheral variant.",
    GpioF15,
    GPIOF,
    (Option),
}
