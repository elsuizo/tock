//! Named constants for NVIC ids, and nvic functions useful for EDU-CIAA only

// name = value; //exception_number vector_offset flags
pub const DAC: u32 = 0; //16 0x40 -
pub const M0APP: u32 = 1; //17 0x44 Cortex-M0APP; Latched TXEV; for M4-M0APP communication
pub const DMA: u32 = 2; //18 0x48 -
pub const RESERVED_1: u32 = 3; //19 0x4C Reserved
pub const FLASHEEPROM: u32 = 4; //20 0x50 ORed flash bank A, flash bank B, EEPROM interrupts
pub const ETHERNET: u32 = 5; //21 0x54 Ethernet interrupt
pub const SDIO: u32 = 6; //22 0x58 SD/MMC interrupt
pub const LCD: u32 = 7; //23 0x5C -
pub const USB0: u32 = 8; //24 0x60 OTG interrupt
pub const USB1: u32 = 9; //25 0x64 -
pub const SCTIMER_PWM: u32 = 10; //26 0x68 SCTimer/PWM combined interrupt
pub const RITIMER: u32 = 11; //27 0x6C -
pub const TIMER0: u32 = 12; //28 0x70 -
pub const TIMER1: u32 = 13; //29 0x74 -
pub const TIMER2: u32 = 14; //30 0x78 -
pub const TIMER3: u32 = 15; //31 0x7C -
pub const MCPWM: u32 = 16; //32 0x80 Motor control PWM
pub const ADC0: u32 = 17; //33 0x84 -
pub const I2C0: u32 = 18; //34 0x88 -
pub const I2C1: u32 = 19; //35 0x8C -
pub const SPI: u32 = 20; //36 0x90 -
pub const ADC1: u32 = 21; //37 0x94 -
pub const SSP0: u32 = 22; //38 0x98 -
pub const SSP1: u32 = 23; //39 0x9C -
pub const USART0: u32 = 24; //40 0xA0 -
pub const UART1: u32 = 25; //41 0xA4 Combined UART interrupt with Modem interrupt
pub const USART2: u32 = 26; //42 0xA8 -
pub const USART3: u32 = 27; //43 0xAC Combined USART interrupt with IrDA interrupt
pub const I2S0: u32 = 28; //44 0xB0 -
pub const I2S1: u32 = 29; //45 0xB4 -
pub const SPIFI: u32 = 30; //46 0xB8 -
pub const SGPIO: u32 = 31; //47 0xBC -
pub const PIN_INT0: u32 = 32; //48 0xC0 GPIO pin interrupt 0
pub const PIN_INT1: u32 = 33; //49 0xC4 GPIO pin interrupt 1
pub const PIN_INT2: u32 = 34; //50 0xC8 GPIO pin interrupt 2
pub const PIN_INT3: u32 = 35; //51 0xCC GPIO pin interrupt 3
pub const PIN_INT4: u32 = 36; //52 0xD0 GPIO pin interrupt 4
pub const PIN_INT5: u32 = 37; //53 0xD4 GPIO pin interrupt 5
pub const PIN_INT6: u32 = 38; //54 0xD8 GPIO pin interrupt 6
pub const PIN_INT7: u32 = 39; //55 0xDC GPIO pin interrupt 7
pub const GINT0: u32 = 40; //56 0xE0 GPIO global interrupt 0
pub const GINT1: u32 = 41; //57 0xE4 GPIO global interrupt 1
pub const EVENTROUTER: u32 = 42; //58 0xE8 Event router interrupt
pub const C_CAN1: u32 = 43; //59 0xEC -
pub const RESERVED_2: u32 = 44; //60 0xF0 -
pub const ADCHS: u32 = 45; //61 0xF4 ADCHS combined interrupt
pub const ATIMER: u32 = 46; //62 0xF8 Alarm timer interrupt
pub const RTC: u32 = 47; //63 0xFC -
pub const RESERVED_3: u32 = 48; //64 0x100 -
pub const WWDT: u32 = 49; //65 0x104 -
pub const M0SUB: u32 = 50; //66 0x108 TXE instruction from the M0 subsystem core
pub const C_CAN0: u32 = 51; //67 0x10C -
pub const QEI: u32 = 52; //68 0x110 -