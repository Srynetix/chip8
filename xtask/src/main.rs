use argh::FromArgs;

/// Tasks.
#[derive(FromArgs)]
struct Args {
    #[argh(subcommand)]
    cmd: Command,
}

/// Tasks.
#[derive(FromArgs)]
#[argh(subcommand)]
enum Command {
    Format(FormatCommand),
    Lint(LintCommand),
    Test(TestCommand),
    Build(BuildCommand),
}

/// Format.
#[derive(FromArgs)]
#[argh(subcommand, name = "fmt")]
struct FormatCommand {
    /// check mode
    #[argh(switch)]
    check: bool,
}

/// Lint.
#[derive(FromArgs)]
#[argh(subcommand, name = "lint")]
struct LintCommand {
    /// error mode
    #[argh(switch)]
    error: bool,
}

/// Test.
#[derive(FromArgs)]
#[argh(subcommand, name = "test")]
struct TestCommand {}

/// Build.
#[derive(FromArgs)]
#[argh(subcommand, name = "build")]
struct BuildCommand {
    /// release mode
    #[argh(switch)]
    release: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = argh::from_env();

    match args.cmd {
        Command::Build(cmd) => {
            if cmd.release {
                duct::cmd!("cargo", "build", "--all", "--tests", "--release").run()?;
            } else {
                duct::cmd!("cargo", "build", "--all", "--tests").run()?;
            }
        }
        Command::Format(cmd) => {
            if cmd.check {
                duct::cmd!("cargo", "fmt", "--all", "--", "--check").run()?;
            } else {
                duct::cmd!("cargo", "fmt", "--all").run()?;
            }
        }
        Command::Lint(cmd) => {
            if cmd.error {
                duct::cmd!("cargo", "clippy", "--all", "--tests", "--", "-D", "warnings").run()?;
            } else {
                duct::cmd!("cargo", "clippy", "--all", "--tests").run()?;
            }
        }
        Command::Test(_) => {
            duct::cmd!("cargo", "test", "--all").run()?;
        }
    }

    Ok(())
}
