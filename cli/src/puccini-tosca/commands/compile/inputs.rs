use super::command::*;

use {
    compris::{annotate::*, normal::*, parse::*, *},
    problemo::*,
    puccini_tosca::grammar::*,
    read_url::*,
};

impl Compile {
    pub fn inputs<AnnotatedT>(&self, url_context: &UrlContextRef) -> Result<Option<Map<AnnotatedT>>, Problem>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut all_inputs = Map::default();

        let parser = Parser::new(Format::YAML);

        for inputs in &self.inputs {
            let inputs = parser.parse_string(&inputs)?;
            let Variant::Map(inputs) = inputs else {
                return Err(WrongTypeError::as_problem("inputs", inputs.type_name(), vec!["map".into()]));
            };
            all_inputs.inner.extend(inputs.inner);
        }

        for inputs_url in &self.inputs_from {
            let inputs_url = url_context.url_or_file_path(inputs_url)?;
            let mut inputs_url = inputs_url.open()?;
            let inputs = parser.parse_reader(&mut inputs_url)?;
            let Variant::Map(inputs) = inputs else {
                return Err(WrongTypeError::as_problem("inputs", inputs.type_name(), vec!["map".into()]));
            };
            all_inputs.inner.extend(inputs.inner);
        }

        // if let Some(all_inputs) = all_inputs {
        //     use depiction::*;
        //     all_inputs.eprint_default_depiction();
        //     return Err(ExitError::from("inputs provided").into());
        // }

        Ok(if all_inputs.inner.is_empty() { None } else { Some(all_inputs) })
    }
}
