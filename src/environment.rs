use crate::{
  error::{self, Result},
  stderr::Stderr,
};
use snafu::ResultExt;
use std::{env, ffi::OsString, path::PathBuf};
use structopt::StructOpt;

#[cfg(test)]
use tempfile::TempDir;

const DEFAULT_PORT: &str = if cfg!(test) { "0" } else { "8080" };

#[derive(StructOpt)]
pub(crate) struct Arguments {
  #[structopt(long, default_value = DEFAULT_PORT, help = "Port to listen on")]
  pub(crate) port: u16,
  #[structopt(long, default_value = "www", help = "Directory of files to serve")]
  pub(crate) directory: PathBuf,
}

pub(crate) struct Environment {
  pub(crate) arguments: Vec<OsString>,
  pub(crate) working_directory: PathBuf,
  pub(crate) stderr: Stderr,
  #[cfg(test)]
  _working_directory_tempdir: TempDir,
}

impl Environment {
  pub(crate) fn production() -> Result<Self> {
    Ok(Environment {
      arguments: env::args_os().into_iter().collect(),
      stderr: Stderr::production(),
      working_directory: env::current_dir().context(error::CurrentDir)?,
      #[cfg(test)]
      _working_directory_tempdir: TempDir::new().unwrap(),
    })
  }

  #[cfg(test)]
  pub(crate) fn test(arguments: &[&str]) -> Self {
    let tempdir = tempfile::Builder::new()
      .prefix("foo-test")
      .tempdir()
      .unwrap();

    Environment {
      arguments: std::iter::once("foo".into())
        .chain(arguments.into_iter().map(|item| item.into()))
        .collect(),
      stderr: Stderr::test(),
      working_directory: tempdir.path().to_owned(),
      _working_directory_tempdir: tempdir,
    }
  }

  pub(crate) fn arguments(&self) -> Result<Arguments> {
    Ok(Arguments::from_iter_safe(&self.arguments)?)
  }
}