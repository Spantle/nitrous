# CPUs
- ARM7 is ARMv4T, ARM9 is ARMv5TE.
- The ARM9 used in the NDS does not use Jazelle (J).
- ARM9 is the main processor while ARM7 deals with "auxiliary tasks" such as sound processing, network communications, the touchscreen and microphone, and the RTC [Emulating the Nintendo DS: Part 1]
- ARM9 has access to the hardware divison and square root registers, while the ARM7 has access to the touchscreen and microphone [Emulating the Nintendo DS: Part 1]

# 2D GPU
- A scanline being drawn is the HDraw period (256 pixels)
- After each scanline there is a pause known as the HBlank period lasting for 99 pixels
- There are 192 scanlines (VDraw period)
- After VDraw there is a pause before the screen is redrawn lasting for 71 blank scanlines (VBlank period)