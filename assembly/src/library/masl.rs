use super::*;

pub struct LibraryEntry {
    pub name: LibraryPath,
    pub source_path: std::path::PathBuf,
}

pub struct WalkLibrary<'a> {
    namespace: LibraryNamespace,
    root: &'a std::path::Path,
    stack: alloc::collections::VecDeque<std::io::Result<std::fs::DirEntry>>,
}

impl<'a> WalkLibrary<'a> {
    pub fn new(namespace: LibraryNamespace, path: &'a std::path::Path) -> std::io::Result<Self> {
        use alloc::collections::VecDeque;

        let stack = VecDeque::from_iter(std::fs::read_dir(path)?);

        Ok(Self { namespace, root: path, stack })
    }

    fn next_entry(
        &mut self,
        entry: &std::fs::DirEntry,
        ty: &std::fs::FileType,
    ) -> Result<Option<LibraryEntry>, crate::diagnostics::Report> {
        use std::{ffi::OsStr, fs};

        use crate::{
            diagnostics::{IntoDiagnostic, Report},
            LibraryPath,
        };

        if ty.is_dir() {
            let dir = entry.path();
            self.stack.extend(fs::read_dir(dir).into_diagnostic()?);
            return Ok(None);
        }

        let mut file_path = entry.path();
        let is_module = file_path
            .extension()
            .map(|ext| ext == AsRef::<OsStr>::as_ref(CompiledLibrary::MODULE_EXTENSION))
            .unwrap_or(false);
        if !is_module {
            return Ok(None);
        }

        // Remove the file extension, and the root prefix, leaving us
        // with a namespace-relative path
        file_path.set_extension("");
        if file_path.is_dir() {
            return Err(Report::msg(format!(
                "file and directory with same name are not allowed: {}",
                file_path.display()
            )));
        }
        let relative_path = file_path
            .strip_prefix(self.root)
            .expect("expected path to be a child of the root directory");

        // Construct a [LibraryPath] from the path components, after validating them
        let mut libpath = LibraryPath::from(self.namespace.clone());
        for component in relative_path.iter() {
            let component = component.to_str().ok_or_else(|| {
                let p = entry.path();
                Report::msg(format!("{} is an invalid directory entry", p.display()))
            })?;
            libpath.push(component).into_diagnostic()?;
        }
        Ok(Some(LibraryEntry { name: libpath, source_path: entry.path() }))
    }
}

impl<'a> Iterator for WalkLibrary<'a> {
    type Item = Result<LibraryEntry, crate::diagnostics::Report>;
    fn next(&mut self) -> Option<Self::Item> {
        use crate::diagnostics::IntoDiagnostic;
        loop {
            let entry = self
                .stack
                .pop_front()?
                .and_then(|entry| entry.file_type().map(|ft| (entry, ft)))
                .into_diagnostic();

            match entry {
                Ok((ref entry, ref file_type)) => {
                    match self.next_entry(entry, file_type).transpose() {
                        None => continue,
                        result => break result,
                    }
                },
                Err(err) => break Some(Err(err)),
            }
        }
    }
}
