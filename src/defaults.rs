use anyhow::{anyhow, Context, Result};
use std::ffi::CString;

mod sys {
    include!(concat!(env!("OUT_DIR"), "/defaults.rs"));
}

pub(crate) struct Application {
    c_id: CString,
}

impl Application {
    pub(crate) fn new<S: AsRef<str>>(id: S) -> Result<Application> {
        to_cstring(id.as_ref()).map(|c_id| Application { c_id: c_id })
    }

    pub(crate) fn bool(&self, key: &str, value: bool) -> Result<&Application> {
        // TODO logging
        let c_key = to_cstring(&key)?;
        unsafe { sys::defaults_set_bool(self.c_id.as_ptr(), c_key.as_ptr(), value) }
        Ok(self)
    }

    // TODO float, string, int

    // Would be nice to do this using Drop but it can fail and we want to propagate those failures
    pub(crate) fn sync(&self) -> Result<()> {
        // TODO logging
        let success = unsafe { sys::defaults_sync(self.c_id.as_ptr()) };
        if success {
            Ok(())
        } else {
            Err(anyhow!("Defaults synchronization failed"))
        }
    }
}

fn to_cstring(str: &str) -> Result<CString> {
    CString::new(str).with_context(|| format!("Converting to string {:?} to CString", str))
}