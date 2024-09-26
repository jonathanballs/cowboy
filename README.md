# Cowboy

Howdy! Cowboy is a Game Boy emulator built in Rust. It was built as an
educational exercise to learn both Rust and about emulation however it is
pretty much feature complete and runs most games which target the original Game
Boy. In the future I may extend it to support Game Boy Color. Cowboy also comes
with a pretty nifty debug tooling for games running in the emulator. The
tooling was created primarily to support the development of Cowboy however it
is useful for understanding Game Boy games work in general since it includes a
full decompiler, breakpoints, as well as full memory, register and interrupt
inspection.

![Screenshot 2024-09-03 at 18 44
30](https://github.com/user-attachments/assets/79f27012-5c56-417e-9d11-ecea3c862667)

## Running

You will need `rust` and `cargo` installed. By default the emulator will run
the super mario land rom from the `roms/` directory.

```
$ cargo run
```

You can also specify a ROM by passing the path as an argument:

```
$ cargo run roms/tetris.gb
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
