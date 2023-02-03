use std::{fs::File, path::Path};

use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, displaydoc::Display)]
pub enum TemplateError {
    /// Creating new file error: {0}
    CreateFile(std::io::Error),
    /// Writing template error: {0}
    WriteTemplate(serde_json::Error),
}

pub trait Template: Serialize {
    fn placeholder() -> Self;

    fn write_template<P, F>(p: P, f: F) -> Result<(), TemplateError>
    where
        P: AsRef<Path>,
        F: FnOnce(&mut File, &Self) -> Result<(), serde_json::Error>,
        Self: Sized,
    {
        let placeholder = Self::placeholder();

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(p)
            .map_err(TemplateError::CreateFile)?;

        f(&mut file, &placeholder).map_err(TemplateError::WriteTemplate)?;

        Ok(())
    }

    fn write_template_as_json<P>(p: P) -> Result<(), TemplateError>
    where
        Self: Sized,
        P: AsRef<Path>,
    {
        Self::write_template(p, |file, placeholder| {
            serde_json::to_writer_pretty(file, placeholder)
        })
    }
}
