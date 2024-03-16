use alloc::{
    borrow::Cow,
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};

use crate::{
    ast::{Module, ModuleKind},
    diagnostics::{
        Diagnostic, IntoDiagnostic, NamedSource, Report, SourceCode, SourceFile, WrapErr,
    },
    library::{LibraryNamespace, LibraryPath},
};

/// The set of options which can be used to control the behavior of the [Compile] trait.
#[derive(Debug, Clone)]
pub struct Options {
    /// The kind of [Module] to compile.
    ///
    /// The default kind is executable.
    pub kind: ModuleKind,
    /// The namespace to apply to the compiled [Module]
    ///
    /// If unset, the namespace will be derived from the [ModuleKind].
    pub namespace: Option<LibraryNamespace>,
    /// The name to give the compiled [Module]
    ///
    /// This option overrides `namespace`.
    ///
    /// If unset, and there is no name associated with the item being compiled (e.g. a file path)
    /// then the path will consist of just a namespace; using the value of `namespace` if provided,
    /// or deriving one from `kind`.
    pub path: Option<LibraryPath>,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            kind: ModuleKind::Executable,
            namespace: None,
            path: None,
        }
    }
}
impl Options {
    /// Configure a set of [Options] to compile a [Module] with the given `kind` and `path`.
    ///
    /// This is primarily useful when compiling a module from source code that has no meaningful
    /// [LibraryPath] associated with it, such as when compiling from a `str`. This will override
    /// the default name derived from the given [ModuleKind].
    pub fn new<P, E>(kind: ModuleKind, path: P) -> Result<Self, E>
    where
        P: TryInto<LibraryPath, Error = E>,
    {
        let path = path.try_into()?;
        Ok(Self {
            kind,
            namespace: None,
            path: Some(path),
        })
    }

    /// Get the default [Options] for compiling a library module
    pub fn for_library() -> Self {
        Self {
            kind: ModuleKind::Library,
            ..Default::default()
        }
    }

    /// Get the default [Options] for compiling a kernel module
    pub fn for_kernel() -> Self {
        Self {
            kind: ModuleKind::Kernel,
            ..Default::default()
        }
    }
}

/// This trait is meant to be implemented by any type that can be compiled to a [Module],
/// to allow methods which expect a [Module] to accept things like:
///
/// * A [Module] which was previously compiled or read from a [crate::Library]
/// * A string representing the source code of a [Module]
/// * A path to a file containing the source code of a [Module]
/// * A vector of [Form]s comprising the contents of a [Module]
pub trait Compile: Sized {
    /// Compile (or convert) `self` into an executable [Module].
    ///
    /// See [Compile::compile_with_opts] for more details.
    #[inline]
    fn compile(self) -> Result<Box<Module>, Report> {
        self.compile_with_opts(Options::default())
    }

    /// Compile (or convert) `self` into a [Module] using the provided `options`.
    ///
    /// Returns a [SyntaxError] if compilation fails due to a parsing or semantic analysis error,
    /// or if the module provided is of the wrong kind (e.g. we expected a library module but got
    /// an executable module)
    ///
    /// See the documentation for [Options] to see how compilation can be configured.
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report>;
}

/// This error occurs when attempting to use a [Module] in a context which is invalid for that module.
///
/// For example, using an executable module as a library.
#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("compilation failed: expected a {expected} module, but got a {actual} module")]
#[diagnostic()]
pub struct ModuleKindMismatchError {
    expected: ModuleKind,
    actual: ModuleKind,
}

impl Compile for Module {
    #[inline(always)]
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report> {
        Box::new(self).compile_with_opts(options)
    }
}

impl<'a> Compile for &'a Module {
    #[inline(always)]
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report> {
        Box::new(self.clone()).compile_with_opts(options)
    }
}

impl Compile for Box<Module> {
    fn compile_with_opts(mut self, options: Options) -> Result<Box<Module>, Report> {
        let actual = self.kind();
        if actual == options.kind {
            if let Some(path) = options.path {
                self.set_path(path);
            } else if let Some(ns) = options.namespace {
                if !self.is_in_namespace(&ns) {
                    self.set_namespace(ns);
                }
            }
            Ok(self)
        } else {
            Err(Report::msg(format!(
                "compilation failed: expected a {} module, but got a {actual} module",
                options.kind
            )))
        }
    }
}

impl Compile for Arc<Module> {
    #[inline(always)]
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report> {
        Box::new(Arc::unwrap_or_clone(self)).compile_with_opts(options)
    }
}

impl Compile for Arc<SourceFile> {
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report> {
        let path = match options.path {
            Some(path) => path,
            None => {
                let mut path = self
                    .name()
                    .parse::<LibraryPath>()
                    .into_diagnostic()
                    .wrap_err("cannot compile module as it has an invalid path/name")?;
                if let Some(ns) = options.namespace {
                    path.set_namespace(ns);
                }
                path
            }
        };
        Module::parse(path, options.kind, self)
    }
}

impl<'a> Compile for &'a str {
    #[inline(always)]
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report> {
        self.to_string().compile_with_opts(options)
    }
}

impl<'a> Compile for &'a String {
    #[inline(always)]
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report> {
        self.clone().compile_with_opts(options)
    }
}

impl Compile for String {
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report> {
        if let Some(path) = options.path {
            let source = Arc::new(SourceFile::new(path.path(), self));
            return Module::parse(path, options.kind, source);
        }
        if let Some(ns) = options.namespace {
            let path = LibraryPath::from(ns);
            let source = Arc::new(SourceFile::new(path.path(), self));
            return Module::parse(path, options.kind, source);
        }
        let path = LibraryPath::from(match options.kind {
            ModuleKind::Library => LibraryNamespace::Anon,
            ModuleKind::Executable => LibraryNamespace::Exec,
            ModuleKind::Kernel => LibraryNamespace::Kernel,
        });
        let source = Arc::new(SourceFile::new(path.path(), self));
        Module::parse(path, options.kind, source)
    }
}

impl<'a> Compile for Cow<'a, str> {
    #[inline(always)]
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report> {
        self.into_owned().compile_with_opts(options)
    }
}

impl<'a> Compile for &'a [u8] {
    #[inline(always)]
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report> {
        core::str::from_utf8(self)
            .map_err(|err| {
                Report::from(crate::parser::ParsingError::from(err)).with_source_code(self.to_vec())
            })
            .wrap_err("parsing failed: invalid source code")
            .and_then(|source| source.compile_with_opts(options))
    }
}

impl Compile for Vec<u8> {
    #[inline(always)]
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report> {
        String::from_utf8(self)
            .map_err(|err| {
                let error = crate::parser::ParsingError::from(err.utf8_error());
                Report::from(error).with_source_code(err.into_bytes())
            })
            .wrap_err("parsing failed: invalid source code")
            .and_then(|source| source.compile_with_opts(options))
    }
}

impl<T> Compile for NamedSource<T>
where
    T: SourceCode + AsRef<[u8]>,
{
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report> {
        let content = String::from_utf8(self.inner().as_ref().to_vec())
            .map_err(|err| {
                let error = crate::parser::ParsingError::from(err.utf8_error());
                Report::from(error).with_source_code(err.into_bytes())
            })
            .wrap_err("parsing failed: expected source code to be valid utf-8")?;
        Arc::new(SourceFile::new(self.name(), content)).compile_with_opts(options)
    }
}

#[cfg(feature = "std")]
impl<'a> Compile for &'a std::path::Path {
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report> {
        use crate::{ast::Ident, library::PathError};
        use std::path::Component;

        let path = match options.path {
            Some(path) => path,
            None => {
                let ns = options.namespace.unwrap_or_else(|| match options.kind {
                    ModuleKind::Library => LibraryNamespace::Anon,
                    ModuleKind::Executable => LibraryNamespace::Exec,
                    ModuleKind::Kernel => LibraryNamespace::Kernel,
                });
                let mut parts = Vec::default();
                self.components()
                    .skip_while(|component| {
                        matches!(
                            component,
                            Component::Prefix(_)
                                | Component::RootDir
                                | Component::ParentDir
                                | Component::CurDir
                        )
                    })
                    .try_for_each(|component| {
                        let part = component
                            .as_os_str()
                            .to_str()
                            .ok_or(PathError::InvalidUtf8)
                            .and_then(|s| Ident::new(s).map_err(PathError::InvalidComponent))
                            .into_diagnostic()
                            .wrap_err("invalid module path")?;
                        parts.push(part);

                        Ok::<(), Report>(())
                    })?;
                LibraryPath::new_from_components(ns, parts)
            }
        };
        Module::parse_file(path, options.kind, self)
    }
}

#[cfg(feature = "std")]
impl Compile for std::path::PathBuf {
    #[inline(always)]
    fn compile_with_opts(self, options: Options) -> Result<Box<Module>, Report> {
        self.as_path().compile_with_opts(options)
    }
}
