use darling_api as darling;

pub struct Npm;

pub static PACKAGE_MANAGER: Npm = Npm;

impl darling::PackageManager for Npm {
    fn name(&self) -> String {
        "npm".to_owned()
    }

    fn install(&self, _context: &darling::Context, package: &darling::InstallationEntry) -> anyhow::Result<()> {
        std::process::Command::new("npm")
            .arg("install")
            .arg("-g")
            .arg(&package.name)
            .spawn()?
            .wait()?;

        Ok(())
    }

    fn uninstall(&self, _context: &darling::Context, package: &darling::InstallationEntry) -> anyhow::Result<()> {
        std::process::Command::new("npm")
            .arg("uninstall")
            .arg("-g")
            .arg(&package.name)
            .spawn()?
            .wait()?;

        Ok(())
    }

    fn get_all_explicit(&self, _context: &darling::Context) -> anyhow::Result<Vec<(String, String)>> {
        let output = String::from_utf8(
            std::process::Command::new("npm")
                .arg("list")
                .arg("-g")
                .arg("--depth")
                .arg("0")
                .output()?
                .stdout,
        )?;
        Ok(output
            .lines()
            .filter_map(|line| {
                regex_macro::regex!(r"(\S+)@(\S+)")
                    .captures(line)
                    .map(|captures| (captures[1].to_owned(), captures[2].to_owned()))
            })
            .collect::<Vec<_>>())
    }
}
