use std::{env, path::PathBuf};

pub struct Args {
    directory: Option<PathBuf>,
}

impl Args {
    pub fn from_env() -> Self {
        let mut args_builder = ArgsBuilder::new();
        let mut args = env::args();
        args.next().expect("args has a program name");

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--directory" => {
                    let directory = PathBuf::from(args.next().expect("Directory should be given"));

                    args_builder.with_directory(directory);
                }
                _ => {
                    panic!("Unknown argument");
                }
            }
        }

        args_builder.build()
    }

    pub fn take_directory(&mut self) -> Option<PathBuf> {
        self.directory.take()
    }

    fn new(directory: Option<PathBuf>) -> Self {
        Self { directory }
    }
}

struct ArgsBuilder {
    directory: Option<PathBuf>,
}

impl ArgsBuilder {
    fn new() -> Self {
        Self { directory: None }
    }

    fn with_directory(&mut self, directory: PathBuf) -> &mut Self {
        self.directory = Some(directory);

        self
    }

    fn build(self) -> Args {
        Args::new(self.directory)
    }
}
