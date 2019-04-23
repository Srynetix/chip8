use chip8::core::assembler::Assembler;
use chip8::peripherals::cartridge::Cartridge;

#[test]
fn test_assembler() {
    use tempdir::TempDir;

    // Example code.
    let example = "JP 020E\nJP 010A";
    let assembler = Assembler::from_string(example);
    let cartridge = assembler.assemble_cartridge().unwrap();

    // Save cartridge.
    let tmpdir = TempDir::new("test-assembly").unwrap();
    let tmppath = tmpdir.path().join("example.ch8");
    cartridge.save_to_path(&tmppath).unwrap();

    // Reload cartridge.
    let cartridge2 = Cartridge::load_from_path(&tmppath).unwrap();
    let dis = cartridge2.disassemble();
    let code = dis.1.join("\n");

    // Saved code should be equal to example code.
    assert_eq!(code, example);
}
