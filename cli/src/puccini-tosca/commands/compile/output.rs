use super::{
    super::{super::errors::*, root::*},
    command::*,
    debug::*,
};

use {
    anstream::println,
    compris::{annotate::*, normal::*, ser::*},
    depiction::*,
    floria::*,
    kutil::{cli::run::*, std::error::*},
    puccini_csar::*,
    puccini_tosca::grammar::*,
    std::fmt,
};

impl Compile {
    pub fn depict_errors<AnnotatedT>(
        csar_errors: &Errors<CsarError>,
        tosca_errors: &Errors<ToscaError<AnnotatedT>>,
        floria_errors: &Errors<FloriaError>,
        print_first: &mut bool,
        output_floria: &mut bool,
        root: &Root,
    ) where
        AnnotatedT: 'static + Annotated + fmt::Debug,
    {
        if let Err(csar_errors) = csar_errors.check() {
            *output_floria = false;

            if Self::print_next(print_first, root) {
                for error in csar_errors {
                    error.eprint_default_depiction();
                }
            }
        }

        if let Err(tosca_errors) = tosca_errors.check() {
            *output_floria = false;

            if Self::print_next(print_first, root) {
                tosca_errors.annotated_depictions(Some("TOSCA Errors".into())).eprint_default_depiction();
            }
        }

        #[cfg(feature = "plugins")]
        if let Err(floria_errors) = floria_errors.check() {
            *output_floria = false;

            if Self::print_next(print_first, root) {
                floria_errors.to_depict("Floria Errors").eprint_default_depiction();
            }
        }
    }

    pub fn depict_debug(&self, catalog: &Catalog, print_first: &mut bool, output_floria: &mut bool, root: &Root) {
        match self.debug {
            Some(Debug::Namespaces) => {
                *output_floria = false;

                if Self::print_next(print_first, root) {
                    catalog.namespaces_depiction().print_default_depiction();
                }
            }

            Some(Debug::Parsed | Debug::Completed) => {
                *output_floria = false;

                if Self::print_next(print_first, root) {
                    catalog.entities_depiction().print_default_depiction();
                }
            }

            Some(Debug::Compiled | Debug::Instance) => {
                // Force it to be true, even if there were compilation errors
                *output_floria = true;
            }

            _ => {}
        }
    }

    pub fn output_floria_template<StoreT>(
        &self,
        floria_service_template_id: Option<ID>,
        store: &StoreT,
        print_first: &mut bool,
        output_floria: &mut bool,
        root: &Root,
    ) -> Result<(), MainError>
    where
        StoreT: Store,
    {
        if *output_floria
            && let Some(floria_service_template_id) = floria_service_template_id
            && let Some(floria_service_template) = store.get_vertex_template(&floria_service_template_id)?
        {
            match self.compris_format()? {
                Some(format) => {
                    self.output_compris(floria_service_template.into_expression(true, store)?, format, root)?
                }

                None => {
                    if Self::print_next(print_first, root) {
                        floria_service_template.to_depict(store).print_default_depiction();
                    }
                }
            }
        }

        Ok(())
    }

    #[cfg(feature = "plugins")]
    pub fn output_floria_instance<StoreT>(
        &self,
        floria_instance: Option<Vertex>,
        store: &StoreT,
        print_first: &mut bool,
        output_floria: &mut bool,
        root: &Root,
    ) -> Result<(), MainError>
    where
        StoreT: Store,
    {
        if *output_floria
            && !matches!(self.debug, Some(Debug::Compiled))
            && let Some(floria_instance) = floria_instance
        {
            *output_floria = false;

            match self.compris_format()? {
                Some(format) => self.output_compris(floria_instance.into_expression(true, store)?, format, root)?,

                None => {
                    if Self::print_next(print_first, root) {
                        floria_instance.to_depict(store).print_default_depiction();
                    }
                }
            }
        }

        Ok(())
    }

    fn print_next(print_first: &mut bool, root: &Root) -> bool {
        if !root.quiet {
            if !*print_first {
                println!();
            }
            *print_first = false;
            true
        } else {
            false
        }
    }

    fn output_compris(&self, expression: Expression, format: compris::Format, root: &Root) -> Result<(), MainError> {
        let variant: Variant<WithoutAnnotations> = expression.into();

        RepresentationWriter::new(Some(format), !self.output_plain, self.output_base64).write_to_file_or_stdout(
            &variant,
            root.quiet,
            false,
            true,
            self.output_file.as_ref(),
        )?;

        Ok(())
    }

    fn compris_format(&self) -> Result<Option<compris::Format>, MainError> {
        match &self.output_format {
            Some(output_format) => Ok(output_format.to_compris()),

            None => match self.output_file.as_ref().and_then(|output_file| compris::Format::from_path(output_file)) {
                Some(format) => Ok(Some(format)),

                None => {
                    if self.output_file.is_some() {
                        Err(ExitError::from("cannot determine output format; specify it explicitly with --format/-f")
                            .into())
                    } else {
                        Ok(None)
                    }
                }
            },
        }
    }
}
