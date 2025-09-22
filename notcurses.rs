use notcurses::*;

fn main() -> NotcursesResult<()> {
    let nc = Notcurses::new_cli()?;
    let mut cli = nc.cli_plane()?;
    cli.render()?;
    Ok(())
}
