/// Output.
macro_rules! output {
    ( $self:ident, $cli:ident,$store:ident, $entity:ident $(,)? ) => {
        match $self.get_output_format() {
            Some(output_format) => {
                let expression = $entity.into_expression(true, &$store)?;
                let variant: ::compris::normal::Variant<::compris::annotate::WithoutAnnotations> = expression.into();

                let serializer = ::compris::ser::Serializer::new(output_format)
                    .with_pretty(!$self.output_plain)
                    .with_base64($self.output_base64);

                if let Some(output_path) = &$self.output_path {
                    let mut file = ::std::fs::File::create(output_path)?;
                    serializer.write(&variant, &mut file)?;
                } else if !$cli.quiet {
                    serializer.print(&variant).expect("print");
                }
            }

            None => {
                if !$cli.quiet {
                    $entity.to_depict(&$store).print_default_depiction();
                }
            }
        }
    };
}

pub(crate) use output;
