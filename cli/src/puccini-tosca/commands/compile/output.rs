use super::{super::root::*, command::*, debug::*};

use {
    anstream::{eprint, println, stdout},
    compris::{annotate::*, depict::*, normal::*, ser::*},
    depiction::*,
    floria::*,
    problemo::{common::*, *},
    puccini_tosca::grammar::*,
    std::collections::*,
};

impl Compile {
    pub fn depict_problems(
        csar_problems: Problems,
        tosca_problems: Problems,
        floria_problems: Problems,
        print_first: &mut bool,
        output_floria: &mut bool,
        root: &Root,
    ) -> bool {
        let no_problems = csar_problems.is_empty() && tosca_problems.is_empty() && floria_problems.is_empty();

        if let Err(csar_problems) = csar_problems.check() {
            *output_floria = false;

            if Self::print_next(print_first, root) {
                csar_problems.annotated_depiction().with_heading("CSAR Errors").eprint_default_depiction();
            }
        }

        if let Err(tosca_problems) = tosca_problems.check() {
            *output_floria = false;

            if Self::print_next(print_first, root) {
                tosca_problems.annotated_depiction().with_heading("TOSCA Errors").eprint_default_depiction();
            }
        }

        #[cfg(feature = "plugins")]
        if let Err(floria_problems) = floria_problems.check() {
            *output_floria = false;

            if Self::print_next(print_first, root) {
                eprint!("{}", DEFAULT_THEME.heading("Floria Errors"));
                floria_problems.eprint_default_depiction();
            }
        }

        !no_problems
    }

    pub fn depict_debug(&self, catalog: &Catalog, print_first: &mut bool, output_floria: &mut bool, root: &Root) {
        match self.debug {
            Some(Debug::Namespaces) => {
                *output_floria = false;

                if Self::print_next(print_first, root) {
                    catalog.namespaces_depiction(true).print_default_depiction();
                }
            }

            Some(Debug::Parsed | Debug::Completed) => {
                *output_floria = false;

                if Self::print_next(print_first, root) {
                    catalog.entities_depiction(true).print_default_depiction();
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
        store: StoreT,
        print_first: &mut bool,
        output_floria: &mut bool,
        root: &Root,
    ) -> Result<(), Problem>
    where
        StoreT: Clone + Store,
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
                        floria_service_template.as_depict(&store).print_default_depiction();
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
        store: StoreT,
        print_first: &mut bool,
        output_floria: &mut bool,
        root: &Root,
    ) -> Result<(), Problem>
    where
        StoreT: Clone + Store,
    {
        if *output_floria
            && !matches!(self.debug, Some(Debug::Compiled))
            && let Some(floria_instance) = floria_instance
        {
            *output_floria = false;

            let format = self.compris_format()?;

            if !self.outputs.is_empty() {
                let mut outputs = BTreeMap::<Expression, Expression>::default();
                for name in &self.outputs {
                    match floria_instance.instance.properties.get(format!("output:{}", name).as_str()) {
                        Some(output) => {
                            outputs.insert(
                                name.clone().into(),
                                output.value.clone().unwrap_or_else(|| {
                                    // We can't serialize Undefined, so we'll use Null
                                    if format.is_some() { Expression::Null } else { Expression::Undefined }
                                }),
                            );
                        }

                        None => return Err(ExitError::failure_message(format!("undefined output: {}", name))),
                    }
                }

                let outputs = Expression::Map(outputs);
                match format {
                    Some(format) => self.output_compris(outputs, format, root)?,

                    None => {
                        if Self::print_next(print_first, root) {
                            DEFAULT_THEME.write_heading(&mut stdout(), "Service Outputs")?;
                            println!();
                            outputs.print_default_depiction();
                        }
                    }
                }
            } else {
                match format {
                    Some(format) => self.output_compris(floria_instance.into_expression(true, store)?, format, root)?,

                    None => {
                        if Self::print_next(print_first, root) {
                            floria_instance.as_depict(&store).print_default_depiction();
                        }
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

    fn output_compris(&self, expression: Expression, format: compris::Format, root: &Root) -> Result<(), Problem> {
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

    fn compris_format(&self) -> Result<Option<compris::Format>, Problem> {
        match &self.output_format {
            Some(output_format) => Ok(output_format.to_compris()),

            None => match self.output_file.as_ref().and_then(|output_file| compris::Format::from_path(output_file)) {
                Some(format) => Ok(Some(format)),

                None => {
                    if self.output_file.is_some() {
                        Err(ExitError::failure_message(
                            "cannot determine output format; specify it explicitly with --format/-f",
                        ))
                    } else {
                        Ok(None)
                    }
                }
            },
        }
    }
}
