use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    /// the pattern to look for
    pub pattern: String,

    /// the path to the file to read
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
}

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
	for line in content.lines() {
		if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
		}
	}
}
