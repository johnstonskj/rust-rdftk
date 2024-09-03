/*!
Create a rich document for the provided scheme.
*/

use crate::model::collection::Member;
use crate::model::concept::ConceptRelation;
use crate::model::properties::LabelKind;
use crate::model::{
    standard_mappings, to_rdf_graph_with_mappings, Collection, Concept, Label, Labeled,
    LiteralProperty, Resource, Scheme, ToUri,
};
use crate::ns;
use rdftk_core::model::graph::mapping::PrefixMappingRef;
use rdftk_core::model::literal::LanguageTag;
use rdftk_core::simple;
use rdftk_io::turtle::writer::TurtleWriter;
use rdftk_io::write_graph_to_string;
use rdftk_iri::IRIRef;
use somedoc::error::{Error, ErrorKind, ResultExt};
use somedoc::model::block::{
    Cell, Column, Formatted, HasBlockContent, HasLabel, Heading, HeadingLevel, Label as Anchor,
    List, Paragraph, Row, Table,
};
use somedoc::model::document::Document;
use somedoc::model::inline::{HasInlineContent, HyperLink, Span};
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

struct Context {
    ns_mappings: PrefixMappingRef,
    collections: Vec<Rc<RefCell<Collection>>>,
    language: Option<LanguageTag>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Construct a new `Document` instance which will describe the provided Scheme. If provided, the
/// specified `language` will be used to select preferred labels and descriptions. If provided, the
/// default namespace will be used when serializing the Turtle appendix.
///
pub fn make_document(
    scheme: &Scheme,
    language: Option<LanguageTag>,
    default_namespace: Option<IRIRef>,
) -> Result<Document, Error> {
    let ns_mappings = standard_mappings();
    if let Some(default_namespace) = default_namespace {
        let mut ns_mappings = ns_mappings.borrow_mut();
        let _ = ns_mappings.set_default_namespace(default_namespace);
    }

    make_document_with_mappings(scheme, language, ns_mappings)
}

///
/// Construct a new `Document` instance which will describe the provided Scheme. If provided, the
/// specified `language` will be used to select preferred labels and descriptions. If provided, the
///  set of mappings will be used as prefixes when serializing the Turtle appendix.
///
pub fn make_document_with_mappings(
    scheme: &Scheme,
    language: Option<LanguageTag>,
    ns_mappings: PrefixMappingRef,
) -> Result<Document, Error> {
    let context = Context::new(scheme, language, ns_mappings);

    let mut document: Document = Default::default();

    write_entity_header(&mut document, scheme, "Scheme", 1, &context);

    if scheme.has_top_concepts() {
        let mut links = Paragraph::default();
        let _ = links.add_text_str("Jump to: ");
        let _ = links.add_link(HyperLink::internal_with_caption_str(
            Anchor::from_str("ConceptsHierarchy").unwrap(),
            "Concepts Hierarchy",
        ));
        let _ = links.add_text_str(" | ");
        let _ = links.add_link(HyperLink::internal_with_caption_str(
            Anchor::from_str("Concepts").unwrap(),
            "Concepts",
        ));
        if scheme.has_top_collections() {
            let _ = links.add_text_str(" | ");
            let _ = links.add_link(HyperLink::internal_with_caption_str(
                Anchor::from_str("Collections").unwrap(),
                "Collections",
            ));
        }
        let _ = links.add_text_str(" | ");
        let _ = links.add_link(HyperLink::internal_with_caption_str(
            Anchor::from_str("Appendix-RDF").unwrap(),
            "Appendix - RDF",
        ));
        let _ = document.add_paragraph(links);

        write_concept_tree(&mut document, scheme, &context)?;

        let _ = document.add_heading(
            Heading::sub_section("Concepts")
                .set_label(Anchor::from_str("Concepts").unwrap())
                .clone(),
        );

        let mut concepts = scheme.concepts_flattened();
        concepts.sort_by_key(|concept| concept.borrow().get_preferred_label_for(&context.language));
        concepts.dedup();

        for concept in &concepts {
            for (relation, related) in concept.borrow().concepts() {
                if relation == &ConceptRelation::Narrower
                    || relation == &ConceptRelation::NarrowerPartitive
                    || relation == &ConceptRelation::NarrowerInstantial
                    || relation == &ConceptRelation::Related
                {
                    related
                        .borrow_mut()
                        .add_related_concept(relation.inverse(), concept.clone());
                }
            }
        }

        for concept in &concepts {
            write_concept(&mut document, &*concept.borrow(), &context);
        }
    }

    if scheme.has_top_collections() {
        let _ = document.add_heading(Heading::sub_section("Collections"));
        for collection in &context.collections {
            write_collection(&mut document, &collection.borrow(), &context);
        }
    }

    let _ = document.add_heading(
        Heading::sub_section("Appendix - RDF")
            .set_label(Anchor::from_str("Appendix-RDF").unwrap())
            .clone(),
    );

    let graph = to_rdf_graph_with_mappings(&scheme, context.ns_mappings, &simple::graph_factory());
    let writer = TurtleWriter::default();
    let code = write_graph_to_string(&writer, &graph)
        .chain_err(|| ErrorKind::Msg("Could not serialize graph".to_string()))?;

    let _ = document.add_formatted(Formatted::from(code));

    Ok(document)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Context {
    fn new(scheme: &Scheme, language: Option<LanguageTag>, ns_mappings: PrefixMappingRef) -> Self {
        // make collection mappings!
        let mut collections = scheme.collections_flattened();
        collections
            .sort_by_key(|collection| collection.borrow().get_preferred_label_for(&language));
        Self {
            ns_mappings,
            collections,
            language,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn level_to_heading(level: u8) -> HeadingLevel {
    match level {
        1 => HeadingLevel::Section,
        2 => HeadingLevel::SubSection,
        3 => HeadingLevel::SubSubSection,
        4 => HeadingLevel::SubSubSubSection,
        5 => HeadingLevel::SubSubSubSubSection,
        _ => HeadingLevel::SubSubSubSubSubSection,
    }
}
fn write_entity_header(
    document: &mut Document,
    obj: &impl Resource,
    header_text: &str,
    depth: usize,
    context: &Context,
) {
    let heading = format!(
        "{}: {}",
        header_text,
        obj.get_preferred_label_for(&context.language)
    );

    let _ = document.add_heading(
        Heading::new(&heading, level_to_heading(depth as u8))
            .set_label(Anchor::safe_from(
                &obj.get_preferred_label_for(&context.language),
                Some(&header_text.to_lowercase()),
            ))
            .clone(),
    );

    if let Some(definition) = obj.properties().iter().find(|prop| {
        prop.predicate() == ns::definition() && prop.language() == context.language.as_ref()
    }) {
        let _ = document.add_paragraph(Paragraph::italic_str(definition.lexical_form()));
    }

    let _ = document.add_paragraph(Paragraph::link(HyperLink::external(&obj.uri().to_string())));

    if obj.has_labels() {
        let _ = document.add_heading(Heading::new("Labels", level_to_heading((depth + 1) as u8)));

        write_labels(document, obj.labels().iter().collect(), &context);
    }

    if obj.has_properties() {
        let _ = document.add_heading(Heading::new(
            "Other Properties",
            level_to_heading((depth + 1) as u8),
        ));
        write_other_properties(document, obj.properties().iter().collect(), &context);
    }
}

fn write_labels(document: &mut Document, labels: Vec<&Label>, context: &Context) {
    let mut labels = labels;
    labels.sort_by_key(|label| label.kind());
    let mut current_kind: Option<&LabelKind> = None;
    let mut table: Table = Default::default();
    for label in labels.iter() {
        if Some(label.kind()) != current_kind {
            if table.has_columns() {
                let _ = document.add_table(table);
            }
            current_kind = Some(label.kind());
            let _ = document.add_paragraph(Paragraph::bold_str(&match context
                .ns_mappings
                .borrow()
                .compress(&label.kind().to_uri())
            {
                None => label.kind().to_uri().to_string(),
                Some(qname) => qname.to_string(),
            }));

            table = Table::new(&[Column::from("Label text"), Column::from("Language")]);
        }

        table.add_row(Row::new(&[
            Cell::text_str(label.text()),
            match label.language() {
                None => Cell::empty(),
                Some(lang) => {
                    if label.language() == context.language.as_ref() {
                        Cell::bold_str(&lang.to_string())
                    } else {
                        Cell::plain_str(&lang.to_string())
                    }
                }
            },
        ]));
    }
    if table.has_columns() {
        let _ = document.add_table(table);
    }
}

fn write_other_properties(
    document: &mut Document,
    properties: Vec<&LiteralProperty>,
    context: &Context,
) {
    let mut properties = properties;
    properties.sort_by_key(|property| property.predicate().to_string());
    let mut table = Table::new(&[
        Column::from("Predicate"),
        Column::from("Literal Form"),
        Column::from("Data Type"),
        Column::from("Language"),
    ]);
    for property in properties.iter() {
        table.add_row(Row::new(&[
            Cell::text_str(
                &match context.ns_mappings.borrow().compress(&property.predicate()) {
                    None => property.predicate().to_string(),
                    Some(qname) => qname.to_string(),
                },
            ),
            Cell::text_str(&property.lexical_form()),
            Cell::text_str(&match property.data_type() {
                None => String::new(),
                Some(dt) => match context.ns_mappings.borrow().compress(dt.as_iri()) {
                    None => property.predicate().to_string(),
                    Some(qname) => qname.to_string(),
                },
            }),
            match property.language() {
                None => Cell::empty(),
                Some(lang) => {
                    if property.language() == context.language.as_ref() {
                        Cell::bold_str(&lang.to_string())
                    } else {
                        Cell::plain_str(&lang.to_string())
                    }
                }
            },
        ]));
    }
    let _ = document.add_table(table);
}

fn write_concept(document: &mut Document, concept: &Concept, context: &Context) {
    write_entity_header(document, concept, "Concept", 3, &context);

    if concept.has_concepts() {
        write_concept_relations(document, concept, context);
    }

    write_collection_membership(document, concept.uri(), context);
}

fn write_concept_relations(document: &mut Document, concept: &Concept, context: &Context) {
    let _ = document.add_heading(Heading::new("Related Concepts", level_to_heading(4)));
    let mut table = Table::new(&[Column::from("Relationship"), Column::from("Concept IRI")]);
    for (relation, related) in concept.concepts() {
        let related = related.borrow();
        let label = related.get_preferred_label_for(&context.language);
        table.add_row(Row::new(&[
            Cell::text_str(
                &match context.ns_mappings.borrow().compress(&relation.to_uri()) {
                    None => relation.to_uri().to_string(),
                    Some(qname) => qname.to_string(),
                },
            ),
            Cell::link(HyperLink::internal_with_caption_str(
                Anchor::safe_from(&label, Some("concept")),
                &label,
            )),
        ]));
    }
    for (relation, related) in concept.external_relations() {
        let related_label = match context.ns_mappings.borrow().compress(&related.clone()) {
            None => related.to_string(),
            Some(qname) => qname.to_string(),
        };
        table.add_row(Row::new(&[
            Cell::text_str(&match context.ns_mappings.borrow().compress(&relation) {
                None => relation.to_string(),
                Some(qname) => qname.to_string(),
            }),
            Cell::link(HyperLink::external_with_caption_str(
                &&related.to_string(),
                &related_label,
            )),
        ]));
    }
    let _ = document.add_table(table);
}

fn write_concept_tree(
    document: &mut Document,
    scheme: &Scheme,
    context: &Context,
) -> Result<(), Error> {
    let _ = document.add_heading(
        Heading::sub_section("Concepts Hierarchy")
            .set_label(Anchor::from_str("ConceptsHierarchy").unwrap())
            .clone(),
    );
    for concept in scheme.top_concepts().map(|concept| concept.borrow()) {
        let mut list = List::default();
        let label = concept.get_preferred_label_for(&context.language);
        let link = HyperLink::internal_with_caption_str(
            Anchor::safe_from(&label, Some("concept")),
            &label,
        );
        let _ = list.add_item_from(Span::bold(link.into()).into());
        if concept.has_concepts() {
            write_concept_tree_inner(document, &mut list, concept.concepts().collect(), context)?;
        }
        let _ = document.add_list(list);
    }
    Ok(())
}

fn write_concept_tree_inner(
    document: &mut Document,
    list: &mut List,
    current_concepts: Vec<&(ConceptRelation, Rc<RefCell<Concept>>)>,
    context: &Context,
) -> Result<(), Error> {
    let mut current_concepts = current_concepts;
    current_concepts
        .sort_by_key(|(_, concept)| concept.borrow().get_preferred_label_for(&context.language));
    let mut sub_list = List::default();
    for (relation, concept) in current_concepts.iter().filter(|(rel, _)| rel.is_narrower()) {
        let concept = concept.borrow();
        let pref_label = concept.get_preferred_label_for(&context.language);
        let link = HyperLink::internal_with_caption_str(
            Anchor::safe_from(&pref_label, Some("concept")),
            &pref_label,
        );
        let _ = sub_list.add_item_from(
            match relation {
                ConceptRelation::NarrowerPartitive | ConceptRelation::NarrowerInstantial => {
                    Span::italic(link.into())
                }
                _ => Span::plain(link.into()),
            }
            .into(),
        );
        if concept.has_concepts() {
            write_concept_tree_inner(
                document,
                &mut sub_list,
                concept.concepts().collect(),
                context,
            )?;
        }
    }
    let _ = list.add_sub_list(sub_list);
    Ok(())
}

fn write_collection(document: &mut Document, collection: &Collection, context: &Context) {
    write_entity_header(document, collection, "Collection", 3, &context);

    if collection.has_members() {
        write_collection_members(document, collection.members(), context);
    }

    write_collection_membership(document, collection.uri(), context);
}

fn write_collection_membership(document: &mut Document, member_uri: &IRIRef, context: &Context) {
    let in_collections: Vec<&Rc<RefCell<Collection>>> = context
        .collections
        .iter()
        .filter(|collection| collection.borrow().has_member(member_uri))
        .collect();

    if !in_collections.is_empty() {
        let mut list = List::default();
        let _ = document.add_heading(
            Heading::new("In Collections", level_to_heading(4))
                .set_label(Anchor::from_str("Collections").unwrap())
                .clone(),
        );
        for collection in in_collections {
            let collection = collection.borrow();
            let pref_label = collection.get_preferred_label_for(&context.language);
            let _ = list.add_item_from(
                HyperLink::internal_with_caption_str(
                    Anchor::safe_from(&pref_label, Some("collection")),
                    &pref_label,
                )
                .into(),
            );
        }
        let _ = document.add_list(list);
    }
}

fn write_collection_members<'a>(
    document: &'a mut Document,
    members: impl Iterator<Item = &'a Member>,
    context: &Context,
) {
    let _ = document.add_heading(Heading::new("Members", level_to_heading(4)));
    let mut list = List::default();
    for member in members {
        match member {
            Member::Collection(member) => {
                let member = member.borrow();
                let pref_label = member.get_preferred_label_for(&context.language);
                let mut item_span = Span::default();
                let _ = item_span.add_text("Collection ".into());
                let _ = item_span.add_link(HyperLink::internal_with_caption_str(
                    Anchor::safe_from(&pref_label, Some("collection")),
                    &pref_label,
                ));
                let _ = list.add_item_from(item_span.into());
            }
            Member::Concept(member) => {
                let member = member.borrow();
                let pref_label = member.get_preferred_label_for(&context.language);
                let mut item_span = Span::default();
                let _ = item_span.add_text("Concept ".into());
                let _ = item_span.add_link(HyperLink::internal_with_caption_str(
                    Anchor::safe_from(&pref_label, Some("concept")),
                    &pref_label,
                ));
                let _ = list.add_item_from(item_span.into());
            }
        }
    }
    if list.has_inner() {
        let _ = document.add_list(list);
    }
}
