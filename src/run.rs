pub fn run() -> Result<(), BlpError> {
    // Unified CLI parsing:
    // - Help/Version → print and return Ok(())
    // - Other errors → print and exit with code 2
    let Some(cli) = (match Cli::try_parse() {
        Ok(cli) => Some(cli),
        Err(e) => {
            match e.kind() {
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
                    let _ = e.print(); // graceful 0
                    None
                }
                _ => {
                    let _ = e.print();
                    std::process::exit(e.exit_code()); // usually 2
                }
            }
        }
    }) else {
        return Ok(());
    };

}
