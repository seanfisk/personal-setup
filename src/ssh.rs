use anyhow::Result;
use users::{os::unix::UserExt, User};

use std::io::Write;

use crate::user::UserExt as OtherUserExt;

pub(crate) fn configure(standard_user: &User) -> Result<()> {
    let bytes = include_bytes!("ssh-config");
    let ssh_dir = standard_user.home_dir().join(".ssh");
    let path = ssh_dir.join("config");

    standard_user.as_effective_user(|| {
        crate::fs::ensure_dir(&ssh_dir)?;
        let mut file = crate::fs::create_file(&path)?;
        file.write_all(bytes)?;
        Ok(())
    })
}
