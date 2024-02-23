MEMORY {
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100

    /* Pick one of the two options for RAM layout     */

    /* OPTION A: Use all RAM banks as one big block   */
    /* Reasonable, unless you are doing something     */
    /* really particular with DMA or other concurrent */
    /* access that would benefit from striping        */
    RAM   : ORIGIN = 0x20000000, LENGTH = 264K

    /* OPTION B: Keep the unstriped sections separate */
    /* RAM: ORIGIN = 0x20000000, LENGTH = 256K        */
    /* SCRATCH_A: ORIGIN = 0x20040000, LENGTH = 4K    */
    /* SCRATCH_B: ORIGIN = 0x20041000, LENGTH = 4K    */
}

PROVIDE(TIMER_IRQ_0 = DefaultHandler);
PROVIDE(TIMER_IRQ_1 = DefaultHandler);
PROVIDE(TIMER_IRQ_2 = DefaultHandler);
PROVIDE(TIMER_IRQ_3 = DefaultHandler);
PROVIDE(PWM_IRQ_WRAP = DefaultHandler);
PROVIDE(USBCTRL_IRQ = DefaultHandler);
PROVIDE(XIP_IRQ = DefaultHandler);
PROVIDE(PIO0_IRQ_0 = DefaultHandler);
PROVIDE(PIO0_IRQ_1 = DefaultHandler);
PROVIDE(PIO1_IRQ_0 = DefaultHandler);
PROVIDE(PIO1_IRQ_1 = DefaultHandler);
PROVIDE(DMA_IRQ_0 = DefaultHandler);
PROVIDE(DMA_IRQ_1 = DefaultHandler);
PROVIDE(IO_IRQ_BANK0 = DefaultHandler);
PROVIDE(IO_IRQ_QSPI = DefaultHandler);
PROVIDE(SIO_IRQ_PROC0 = DefaultHandler);
PROVIDE(SIO_IRQ_PROC1 = DefaultHandler);
PROVIDE(CLOCKS_IRQ = DefaultHandler);
PROVIDE(SPI0_IRQ = DefaultHandler);
PROVIDE(SPI1_IRQ = DefaultHandler);
PROVIDE(UART0_IRQ = DefaultHandler);
PROVIDE(UART1_IRQ = DefaultHandler);
PROVIDE(ADC_IRQ_FIFO = DefaultHandler);
PROVIDE(I2C0_IRQ = DefaultHandler);
PROVIDE(I2C1_IRQ = DefaultHandler);
PROVIDE(RTC_IRQ = DefaultHandler);
PROVIDE(SW0_IRQ = DefaultHandler);
PROVIDE(SW1_IRQ = DefaultHandler);
PROVIDE(SW2_IRQ = DefaultHandler);
PROVIDE(SW3_IRQ = DefaultHandler);
PROVIDE(SW4_IRQ = DefaultHandler);
PROVIDE(SW5_IRQ = DefaultHandler);
