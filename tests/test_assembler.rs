use chip8::{peripherals::cartridge::Cartridge, shell::{Args, AssembleCommand, SubCommands}};

use std::fs;

#[test]
fn test_assembler_cmd() {
    use chip8;
    use tempdir::TempDir;

    // Create temp dir
    let tmpdir = TempDir::new("test-assembly").expect("failed to create tmpdir");

    // Example code.
    let example = "JP 020E\nJP 010A";
    let tmppath = tmpdir.path().join("example.asm");
    let outpath = tmpdir.path().join("example.ch8");
    fs::write(&tmppath, example).expect("failed to write to file");

    // Start executable in assembly mode
    chip8::start_shell_using_args(Args {
        verbose: false,
        nested: SubCommands::Assemble(
            AssembleCommand {
                source: tmppath,
                output: outpath.clone()
            }
        )
    }).unwrap();

    // Read the output file
    let cartridge = Cartridge::load_from_path(&outpath).expect("failed to read cartridge");
    let dis = cartridge.disassemble();
    let code = dis.1.join("\n");

    assert_eq!(code, example);
}
