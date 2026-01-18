use std::ffi::OsStr;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Stdio;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub struct HiddenCommand {
    inner: tokio::process::Command,
}

impl HiddenCommand {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        #[cfg_attr(not(windows), allow(unused_mut))]
        let mut cmd = tokio::process::Command::new(program);
        #[cfg(windows)]
        cmd.creation_flags(CREATE_NO_WINDOW);

        Self { inner: cmd }
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.inner.args(args);
        self
    }

    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.inner.current_dir(dir);
        self
    }

    pub fn stdout_null(&mut self) -> &mut Self {
        self.inner.stdout(Stdio::null());
        self
    }

    pub fn stderr_null(&mut self) -> &mut Self {
        self.inner.stderr(Stdio::null());
        self
    }

    pub fn stdin_null(&mut self) -> &mut Self {
        self.inner.stdin(Stdio::null());
        self
    }

    pub fn silence_all(&mut self) -> &mut Self {
        self.stdout_null().stderr_null().stdin_null()
    }

    pub fn spawn(&mut self) -> std::io::Result<tokio::process::Child> {
        self.inner.spawn()
    }
}

pub struct HiddenCommandSync {
    inner: std::process::Command,
}

impl HiddenCommandSync {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        #[cfg_attr(not(windows), allow(unused_mut))]
        let mut cmd = std::process::Command::new(program);
        #[cfg(windows)]
        cmd.creation_flags(CREATE_NO_WINDOW);

        Self { inner: cmd }
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.inner.args(args);
        self
    }

    pub fn output(&mut self) -> std::io::Result<std::process::Output> {
        self.inner.output()
    }
}
