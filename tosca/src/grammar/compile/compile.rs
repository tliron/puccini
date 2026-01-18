use super::{
    super::{catalog::*, source::*},
    context::*,
};

use {duplicate::*, floria::*, problemo::*, read_url::*};

/// Compile TOSCA service template source to Floria vertex template.
#[duplicate_item(
  compile_tosca_to_floria                       load_source                       compile_service_template;
  [compile_tosca_to_floria_with_annotations]    [load_source_with_annotations]    [compile_service_template_with_annotations];
  [compile_tosca_to_floria_without_annotations] [load_source_without_annotations] [compile_service_template_without_annotations];
)]
pub fn compile_tosca_to_floria<ProblemReceiverT>(
    source_id: &SourceID,
    url_context: &UrlContextRef,
    store: StoreRef,
    directory: &Directory,
    catalog: &mut Catalog,
    problems: &mut ProblemReceiverT,
) -> Result<Option<ID>, Problem>
where
    ProblemReceiverT: ProblemReceiver,
{
    catalog.load_source(&source_id, &url_context, problems)?;
    catalog.complete_entities(problems)?;

    let mut context = CompilationContext::new(&source_id, catalog, directory, store.clone(), problems.as_ref());
    catalog.compile_service_template(&mut context)

    // if let Some(service_template_id) = service_template_id {
    //     store.get_vertex_template(&service_template_id).give_unwrap_or_default(problems)
    // } else {
    //     Ok(None)
    // }
}
