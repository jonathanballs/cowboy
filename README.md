# Cowboy

Yeehaw!

This is a gameboy emulator that I'm writing while learning rust.

It targets gameboy colour and I'm trying to get it to run Pokemon Gold!

Status: very early...

![Screenshot 2024-08-28 at 15 52 16](https://github.com/user-attachments/assets/c74eb742-c370-4170-b3af-bb9a34692e9a)

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

These links were very helpful
