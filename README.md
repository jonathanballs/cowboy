# Cowboy

Howdy! Cowboy is a gameboy emulator that I've been building in Rust. I'm doing
it mostly as an educational exercise to learn not just about Rust but about
emulation as well. It also comes with a pretty nifty decompiler and debugger
built in which is very useful while developing! It is certainly not complete
(or playable really) but it can load some games and even get to main menu. My
main goal at the moment is to focus on emulating Tetris as it's one of the
simplest games on the platform.

![Screenshot 2024-09-03 at 18 44
30](https://github.com/user-attachments/assets/79f27012-5c56-417e-9d11-ecea3c862667)

## Running

You will need `rust` and `cargo` installed. By default the emulator will run
the tetris rom from the `roms/` directory.

```
$ cargo run
```

## References

Creating this emulator was a very educational experience for me. I'd like to
highlight the following resources which were very helpful for me when building
this and might be helpful for you if you're thinking of building an emulator as
well.

- [gbdev.io Documentation](https://gbdev.io/pandocs/). This documentation is
  very comprehensive.
- [gbdev.io Opcode table](https://gbdev.io/gb-opcodes/optables/). Table of all
  opcode instructions and includes as JSON file which is useful for testing.
- [gbdev.io Opcode reference](https://gbdev.io/gb-opcodes/optables/).
  Comprehensive opcode documentation
- [TLMBoy: Exploring the Game Boy's
  Boot](https://www.chciken.com/tlmboy/2022/05/02/gameboy-boot.html#25-load-the-logo).
  The first point of call for building a gameboy emulator should be emulating the
  boot rom. This blog post is a very thorough disection of the gameboy boot ROM
  and was invaluable for getting the first version of Cowboy running.
- [Game Boy: Complete Technical
  Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf) Probably one of the
  highest quality technical references I have worked with. The level of precision
  and attention to detail is unparalleled especially when debugging edge cases for
  specific CPU instructions or timing details.
- [The Ultimate Game Boy Talk
  (33c3)](https://www.youtube.com/watch?v=HyzD8pNlpwI&t=2540s) An excellent talk
  from the chaos computer club conference. This is very useful when you are
  getting started and want to get a feel for the system.
- [Tetris disassembly](https://github.com/osnr/tetris). This tetris disassembly
  has been commented. It was useful when my emulator got stuck because I could
  quickly understand the intention of the code it was struggling with.
- [Super mario land
  disassembly](https://github.com/kaspermeerts/supermarioland/). Likewise this
  SML disassembly was useful as it was the second game I approached after tetris.
