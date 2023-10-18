# melonDS Blog Notes
Assorted notes that could be useful that I found by reading the entire [melonDS Blog](https://melonds.kuribo64.net/).

[Checking the GitHub issues could also be beneficial in the future too.](https://github.com/melonDS-emu/melonDS/issues)

## [So what's melonDS?](https://melonds.kuribo64.net/comments.php?id=9)
- ARMWrestler is an ARM9 instruction tester for NDS emulators.

## [Getting somewhere](https://melonds.kuribo64.net/comments.php?id=10)
- RTC needs to be implemented for firmware to boot.
- DS software communicates with the RTC by "bitbanging" a GPIO register. The emulator must put binary "back together" to get data you can work with.
- The BIOS does an initialization sequence for carts using 2 different encryption methods. Read the post for more information.
- Cart DMA (direct memory access) is also discussed. I don't understand it yet.

## [A lot closer to a finished product](https://melonds.kuribo64.net/comments.php?id=11)
- The LDR opcode has a peculiarity.
- Save memory is an EEPROM or Flash chip accessed over a dedicated SPI bus.
- Games use different memory types that apparently must be detected. More details in the post.
- The 3D GPU/Graphics Engine has a FIFO for sending commands to it, called the GX FIFO (could this be the "Command FIFO" Copetti talks about in his article?).
- The GX FIFO can be set to trigger an IRQ when it is emptied or less than half full.
- Some games wait for the IRQ before sending more commands, some others use the DMA to send their command lists automatically. 
- Mario 64 DS has some peculiarities regarding the above.

## [Deeper than it seems](https://melonds.kuribo64.net/comments.php?id=12)
- Colours are 15-bit, but the screens are 18-bit.
- The conversion process is well documented for the 3D GPU, but not for the 2D GPU (at least in 2017).
- The post details some experimenting and results.

## [On the way to 3D graphics](https://melonds.kuribo64.net/comments.php?id=13)
- More 3D experimenting.

## [melonDS 0.1: soon a thing!](https://melonds.kuribo64.net/comments.php?id=15)
- ARMWrestler requires VRAM display to function.
- NDS aging cart test exists.

## [The aging cart](https://melonds.kuribo64.net/comments.php?id=16)
- DMA stuff.
- Capture control test will fail unless you have pixel perfect 3D graphics due to how it tests.

## [Breaking the silence](https://melonds.kuribo64.net/comments.php?id=18)
- Sound hardware is 16 channels with data encoded in PCM8, PCM16, or IMA-ADPCM.
- Channels 8 to 13 support rectangular waves (PSG), and channels 14 and 15 can produce white noise.
- Sound apparently must be synchronous in order to be accurate, mainly because of sound capture.
- The DS has two sound capture units. One is to record audio output and apply effects (such as reverb).
- Channels 1 and 3 should be used to output altered capture data.
- Sound capture setups expect capture buffers to be filled at a fixed interval which can only be accurately achieved by having a synchronous sound engine.

## [Fixing Pokémon White](https://melonds.kuribo64.net/comments.php?id=20)
- Would not boot unless launched from the firmware.
- Occurred due to a bug in melonDS.
- Debugging idea: look at FIFO traffic.

## [More fun fixes](https://melonds.kuribo64.net/comments.php?id=21)
- Debugging idea: Dump and compare RAM.
- The cart interface can only be enabled for one CPU at a time, and thus DMA should only be checked for that CPU.

## [Slicing the melons!](https://melonds.kuribo64.net/comments.php?id=22)
- The 3D renderer's state cannot be modified while it is rendering. Because of this, the "timing" doesn't have to be precise and we can run the 3D renderer in a separate thread.
- Wi-Fi stuff.
- Writable VCount, a register which reflects the current scanline being drawn. Not used often.
- Framerate limiter is dynamic based on how long frames last.

## [Threads!](https://melonds.kuribo64.net/comments.php?id=23)
- In emulation, you can't just throw everything onto separate threads. It depends on how tightly synchronised different components are. [This article explains it more](https://arstechnica.com/gaming/2011/08/accuracy-takes-power-one-mans-3ghz-quest-to-build-a-perfect-snes-emulator/)
- The 3D renderer is not tightly synchronised with the rest of the system, so it can be run on a separate thread.
- Read the post for 3D and 2D shenanigans.

## [So there it is, melonDS 0.3](https://melonds.kuribo64.net/comments.php?id=24)
- Have an option for the RTC to NOT synchronise with the host system's clock. The RTC causes so much randomness in issues, so having them be easily reproduceable (by having one defined time) is useful.

## [Opening to the outer world](https://melonds.kuribo64.net/comments.php?id=25)
- Very detailed Wi-Fi stuff.
- Wi-Fi requires a stub in order for games to get past Wi-Fi checks.

## [Nightmare in viewport street](https://melonds.kuribo64.net/comments.php?id=27)
- 3D stuff.
- Viewports.
- A lot of it goes over my head right now.

## [melonDS 0.4 -- It's here, finally!](https://melonds.kuribo64.net/comments.php?id=28)
- Boxtest stuff.
- The niche Display FIFO is used by Splinter Cell.
