use super::super::{cli::*, errors::*};

use {
    anstream::println,
    compris::{annotate::*, normal::*, ser::*},
    floria::*,
    kutil::{cli::depict::*, std::error::*},
    puccini_tosca::grammar::*,
    std::{fmt, fs::*},
};

impl Compile {
    pub(crate) fn depict_errors<AnnotatedT>(
        tosca_errors: &Errors<ToscaError<AnnotatedT>>,
        floria_errors: &Errors<FloriaError>,
        print_first: &mut bool,
        output_floria: &mut bool,
        cli: &CLI,
    ) where
        AnnotatedT: 'static + Annotated + fmt::Debug,
    {
        if let Err(tosca_errors) = tosca_errors.check() {
            *output_floria = false;

            if Self::print_next(print_first, cli) {
                tosca_errors.annotated_depictions(Some("TOSCA Errors".into())).eprint_default_depiction();
            }
        }

        #[cfg(feature = "plugins")]
        if let Err(floria_errors) = floria_errors.check() {
            *output_floria = false;

            if Self::print_next(print_first, cli) {
                floria_errors.to_depict("Floria Errors").eprint_default_depiction();
            }
        }
    }

    pub(crate) fn depict_debug(&self, catalog: &Catalog, print_first: &mut bool, output_floria: &mut bool, cli: &CLI) {
        match self.debug {
            Some(Debug::Namespaces) => {
                *output_floria = false;

                if Self::print_next(print_first, cli) {
                    catalog.namespaces_depiction().print_default_depiction();
                }
            }

            Some(Debug::Parsed | Debug::Completed) => {
                *output_floria = false;

                if Self::print_next(print_first, cli) {
                    catalog.entities_depiction().print_default_depiction();
                }
            }

            Some(Debug::Compiled) => {
                // Force it to be true, even if there were compilation errors
                *output_floria = true;
            }

            _ => {}
        }
    }

    pub(crate) fn output_floria_template<StoreT>(
        &self,
        floria_service_template_id: Option<ID>,
        store: &StoreT,
        print_first: &mut bool,
        output_floria: &mut bool,
        cli: &CLI,
    ) -> Result<(), MainError>
    where
        StoreT: Store,
    {
        if *output_floria
            && let Some(floria_service_template_id) = floria_service_template_id
            && let Some(floria_service_template) = store.get_vertex_template(&floria_service_template_id)?
        {
            match self.compris_format() {
                Some(format) => {
                    self.output_compris(floria_service_template.into_expression(true, store)?, format, cli)?
                }

                None => {
                    if Self::print_next(print_first, cli) {
                        floria_service_template.to_depict(store).print_default_depiction();
                    }
                }
            }
        }

        Ok(())
    }

    #[cfg(feature = "plugins")]
    pub(crate) fn output_floria_instance<StoreT>(
        &self,
        floria_instance: Option<Vertex>,
        store: &StoreT,
        print_first: &mut bool,
        output_floria: &mut bool,
        cli: &CLI,
    ) -> Result<(), MainError>
    where
        StoreT: Store,
    {
        if *output_floria
            && !matches!(self.debug, Some(Debug::Compiled))
            && let Some(floria_instance) = floria_instance
        {
            *output_floria = false;

            match self.compris_format() {
                Some(format) => self.output_compris(floria_instance.into_expression(true, store)?, format, cli)?,

                None => {
                    if Self::print_next(print_first, cli) {
                        floria_instance.to_depict(store).print_default_depiction();
                    }
                }
            }
        }

        Ok(())
    }

    fn print_next(print_first: &mut bool, cli: &CLI) -> bool {
        if !cli.quiet {
            if !*print_first {
                println!();
            }
            *print_first = false;
            true
        } else {
            false
        }
    }

    fn output_compris(&self, expression: Expression, format: compris::Format, cli: &CLI) -> Result<(), MainError> {
        let variant: Variant<WithoutAnnotations> = expression.into();

        let serializer = Serializer::new(format).with_pretty(!self.output_plain).with_base64(self.output_base64);
        if let Some(output_file) = &self.output_file {
            let mut file = File::create(output_file)?;
            serializer.write(&variant, &mut file)?;
        } else if !cli.quiet {
            serializer.print(&variant).expect("print");
        }

        Ok(())
    }

    fn compris_format(&self) -> Option<compris::Format> {
        match &self.output_format {
            Some(output_format) => output_format.to_compris(),
            None => self.output_file.as_ref().and_then(|output_file| compris::Format::from_path(output_file)),
        }
    }
}
