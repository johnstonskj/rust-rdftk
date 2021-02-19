/*!
Create a rich document for the provided scheme.

Details TBD

# Example

TBD

*/

use crate::model::collection::Member;
use crate::model::concept::ConceptRelation;
use crate::model::properties::LabelKind;
use crate::model::{
    standard_mappings, to_rdf_graph_with_mappings, Collection, Concept, Label, Labeled,
    LiteralProperty, Resource, Scheme, ToURI,
};
use crate::ns;
use rdftk_core::graph::PrefixMappings;
use rdftk_core::DataType;
use rdftk_io::turtle::TurtleWriter;
use rdftk_io::write_graph_to_string;
use rdftk_iri::IRIRef;
use rdftk_memgraph::Mappings;
use rdftk_names::xsd;
use somedoc::error::Error;
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

struct Context<'a> {
    ns_mappings: Mappings,
    collections: Vec<Rc<RefCell<Collection>>>,
    language: &'a str,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn make_document(
    scheme: &Scheme,
    language: &str,
    default_namespace: Option<IRIRef>,
) -> Result<Document, Error> {
    let mut ns_mappings = standard_mappings();
    if let Some(default_namespace) = default_namespace {
        ns_mappings.insert_default(default_namespace);
    }

    make_document_with_mappings(scheme, language, ns_mappings)
}

pub fn make_document_with_mappings(
    scheme: &Scheme,
    language: &str,
    ns_mappings: Mappings,
) -> Result<Document, Error> {
    let context = Context::new(scheme, language, ns_mappings);

    let mut document: Document = Default::default();

    write_entity_header(&mut document, scheme, "Scheme", 1, &context)?;

    if scheme.has_top_concepts() {
        let mut links = Paragraph::default();
        links.add_text_str("Jump to: ");
        links.add_link(HyperLink::internal_with_caption_str(
            Anchor::from_str("ConceptsHierarchy").unwrap(),
            "Concepts Hierarchy",
        ));
        links.add_text_str(" | ");
        links.add_link(HyperLink::internal_with_caption_str(
            Anchor::from_str("Concepts").unwrap(),
            "Concepts",
        ));
        if scheme.has_top_collections() {
            links.add_text_str(" | ");
            links.add_link(HyperLink::internal_with_caption_str(
                Anchor::from_str("Collections").unwrap(),
                "Collections",
            ));
        }
        links.add_text_str(" | ");
        links.add_link(HyperLink::internal_with_caption_str(
            Anchor::from_str("Appendix-RDF").unwrap(),
            "Appendix - RDF",
        ));
        document.add_paragraph(links);

        write_concept_tree(&mut document, scheme, &context)?;

        document.add_heading(
            Heading::sub_section("Concepts")
                .set_label(Anchor::from_str("Concepts").unwrap())
                .clone(),
        );

        let mut concepts = scheme.concepts_flattened();
        concepts.sort_by_key(|concept| concept.borrow().preferred_label(language));
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
            write_concept(&mut document, &*concept.borrow(), &context)?;
        }
    }

    if scheme.has_top_collections() {
        document.add_heading(Heading::sub_section("Collections"));
        for collection in &context.collections {
            write_collection(&mut document, &collection.borrow(), &context)?;
        }
    }

    document.add_heading(
        Heading::sub_section("Appendix - RDF")
            .set_label(Anchor::from_str("Appendix-RDF").unwrap())
            .clone(),
    );

    let graph = to_rdf_graph_with_mappings(&scheme, context.ns_mappings);
    let writer = TurtleWriter::default();
    let code = write_graph_to_string(&writer, &graph)?;

    document.add_formatted(Formatted::from(code));

    Ok(document)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> Context<'a> {
    fn new(scheme: &'a Scheme, language: &'a str, ns_mappings: Mappings) -> Self {
        // make collection mappings!
        let mut collections = scheme.collections_flattened();
        collections.sort_by_key(|collection| collection.borrow().preferred_label(language));
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
fn write_entity_header<'a>(
    document: &mut Document,
    obj: &impl Resource,
    header_text: &str,
    depth: usize,
    context: &Context<'a>,
) -> Result<(), Error> {
    let heading = format!("{}: {}", header_text, obj.preferred_label(context.language));

    document.add_heading(
        Heading::new(&heading, level_to_heading(depth as u8))
            .set_label(Anchor::safe_from(
                &obj.preferred_label(context.language),
                Some(&header_text.to_lowercase()),
            ))
            .clone(),
    );

    let for_language = Some(context.language.to_string());
    if let Some(definition) = obj.properties().iter().find(|prop| {
        prop.predicate() == ns::definition() && prop.value().language() == &for_language
    }) {
        document.add_paragraph(Paragraph::italic_str(definition.value().lexical_form()));
    }

    document.add_paragraph(Paragraph::link(HyperLink::external(&obj.uri().to_string())));

    if obj.has_labels() {
        document.add_heading(Heading::new("Labels", level_to_heading((depth + 1) as u8)));

        write_labels(document, obj.labels().iter().collect(), &context)?;
    }

    if obj.has_properties() {
        document.add_heading(Heading::new(
            "Other Properties",
            level_to_heading((depth + 1) as u8),
        ));
        write_other_properties(document, obj.properties().iter().collect(), &context)?;
    }
    Ok(())
}

fn write_labels<'a>(
    document: &mut Document,
    labels: Vec<&Label>,
    context: &Context<'a>,
) -> Result<(), Error> {
    let mut labels = labels;
    labels.sort_by_key(|label| label.kind());
    let mut current_kind: Option<&LabelKind> = None;
    let mut table: Table = Default::default();
    for label in labels.iter() {
        if Some(label.kind()) != current_kind {
            if table.has_columns() {
                document.add_table(table);
            }
            current_kind = Some(label.kind());
            document.add_paragraph(Paragraph::bold_str(&match context
                .ns_mappings
                .compress(&label.kind().to_uri())
            {
                None => label.kind().to_uri().to_string(),
                Some(qname) => qname.to_string(),
            }));

            table = Table::new(&[Column::from("Label text"), Column::from("Language")]);
        }
        let lang = label.language();

        table.add_row(Row::new(&[
            Cell::text_str(label.text()),
            if lang == context.language {
                Cell::bold_str(&lang.to_string())
            } else {
                Cell::plain_str(&lang.to_string())
            },
        ]));
    }
    if table.has_columns() {
        document.add_table(table);
    }
    Ok(())
}

fn write_other_properties<'a>(
    document: &mut Document,
    properties: Vec<&LiteralProperty>,
    context: &Context<'a>,
) -> Result<(), Error> {
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
            Cell::text_str(&match context.ns_mappings.compress(&property.predicate()) {
                None => property.predicate().to_string(),
                Some(qname) => qname.to_string(),
            }),
            Cell::text_str(&property.value().lexical_form()),
            Cell::text_str(&match property.value().data_type() {
                None => String::new(),
                Some(dt) => match context.ns_mappings.compress(&data_type_uri(dt)) {
                    None => property.predicate().to_string(),
                    Some(qname) => qname.to_string(),
                },
            }),
            match property.value().language() {
                None => Cell::empty(),
                Some(lang) => {
                    if lang == context.language {
                        Cell::bold_str(&lang.to_string())
                    } else {
                        Cell::plain_str(&lang.to_string())
                    }
                }
            },
        ]));
    }
    document.add_table(table);
    Ok(())
}

fn data_type_uri(dt: &DataType) -> IRIRef {
    match dt {
        DataType::String => xsd::string(),
        DataType::QName => xsd::q_name(),
        DataType::IRI => xsd::any_uri(),
        DataType::Boolean => xsd::boolean(),
        DataType::Float => xsd::float(),
        DataType::Double => xsd::double(),
        DataType::Long => xsd::long(),
        DataType::Int => xsd::int(),
        DataType::Short => xsd::short(),
        DataType::Byte => xsd::byte(),
        DataType::UnsignedLong => xsd::unsigned_long(),
        DataType::UnsignedInt => xsd::unsigned_int(),
        DataType::UnsignedShort => xsd::unsigned_short(),
        DataType::UnsignedByte => xsd::unsigned_byte(),
        DataType::Duration => xsd::duration(),
        DataType::Other(iri) => iri,
    }
    .clone()
}

fn write_concept<'a>(
    document: &mut Document,
    concept: &Concept,
    context: &Context<'a>,
) -> Result<(), Error> {
    write_entity_header(document, concept, "Concept", 3, &context)?;

    if concept.has_concepts() {
        write_concept_relations(document, concept, context)?;
    }

    write_collection_membership(document, concept.uri(), context)?;

    Ok(())
}

fn write_concept_relations<'a>(
    document: &mut Document,
    concept: &Concept,
    context: &Context<'a>,
) -> Result<(), Error> {
    document.add_heading(Heading::new("Related Concepts", level_to_heading(4)));
    let mut table = Table::new(&[Column::from("Relationship"), Column::from("Concept IRI")]);
    for (relation, related) in concept.concepts() {
        let related = related.borrow();
        let label = related.preferred_label(context.language);
        table.add_row(Row::new(&[
            Cell::text_str(&match context.ns_mappings.compress(&relation.to_uri()) {
                None => relation.to_uri().to_string(),
                Some(qname) => qname.to_string(),
            }),
            Cell::link(HyperLink::internal_with_caption_str(
                Anchor::safe_from(&label, Some("concept")),
                &label,
            )),
        ]));
    }
    for (relation, related) in concept.external_relations() {
        let related_label = match context.ns_mappings.compress(&related.clone()) {
            None => related.to_string(),
            Some(qname) => qname.to_string(),
        };
        table.add_row(Row::new(&[
            Cell::text_str(&match context.ns_mappings.compress(&relation) {
                None => relation.to_string(),
                Some(qname) => qname.to_string(),
            }),
            Cell::link(HyperLink::external_with_caption_str(
                &&related.to_string(),
                &related_label,
            )),
        ]));
    }
    document.add_table(table);
    Ok(())
}

fn write_concept_tree<'a>(
    document: &mut Document,
    scheme: &Scheme,
    context: &Context<'a>,
) -> Result<(), Error> {
    document.add_heading(
        Heading::sub_section("Concepts Hierarchy")
            .set_label(Anchor::from_str("ConceptsHierarchy").unwrap())
            .clone(),
    );
    for concept in scheme.top_concepts().map(|concept| concept.borrow()) {
        let mut list = List::default();
        let label = concept.preferred_label(context.language);
        let link = HyperLink::internal_with_caption_str(
            Anchor::safe_from(&label, Some("concept")),
            &label,
        );
        list.add_item_from(Span::bold(link.into()).into());
        if concept.has_concepts() {
            write_concept_tree_inner(document, &mut list, concept.concepts().collect(), context)?;
        }
        document.add_list(list);
    }
    Ok(())
}

fn write_concept_tree_inner<'a>(
    document: &mut Document,
    list: &mut List,
    current_concepts: Vec<&(ConceptRelation, Rc<RefCell<Concept>>)>,
    context: &Context<'a>,
) -> Result<(), Error> {
    let mut current_concepts = current_concepts;
    current_concepts.sort_by_key(|(_, concept)| concept.borrow().preferred_label(context.language));
    let mut sub_list = List::default();
    for (relation, concept) in current_concepts.iter().filter(|(rel, _)| rel.is_narrower()) {
        let concept = concept.borrow();
        let pref_label = concept.preferred_label(context.language);
        let link = HyperLink::internal_with_caption_str(
            Anchor::safe_from(&pref_label, Some("concept")),
            &pref_label,
        );
        sub_list.add_item_from(
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
    list.add_sub_list(sub_list);
    Ok(())
}

fn write_collection<'a>(
    document: &mut Document,
    collection: &Collection,
    context: &Context<'a>,
) -> Result<(), Error> {
    write_entity_header(document, collection, "Collection", 3, &context)?;

    if collection.has_members() {
        write_collection_members(document, collection.members(), context)?;
    }

    write_collection_membership(document, collection.uri(), context)
}

fn write_collection_membership<'a>(
    document: &mut Document,
    member_uri: &IRIRef,
    context: &Context<'a>,
) -> Result<(), Error> {
    let in_collections: Vec<&Rc<RefCell<Collection>>> = context
        .collections
        .iter()
        .filter(|collection| collection.borrow().has_member(member_uri))
        .collect();

    if !in_collections.is_empty() {
        let mut list = List::default();
        document.add_heading(
            Heading::new("In Collections", level_to_heading(4))
                .set_label(Anchor::from_str("Collections").unwrap())
                .clone(),
        );
        for collection in in_collections {
            let collection = collection.borrow();
            let pref_label = collection.preferred_label(context.language);
            list.add_item_from(
                HyperLink::internal_with_caption_str(
                    Anchor::safe_from(&pref_label, Some("collection")),
                    &pref_label,
                )
                .into(),
            );
        }
        document.add_list(list);
    }
    Ok(())
}

fn write_collection_members<'a>(
    document: &mut Document,
    members: impl Iterator<Item = &'a Member>,
    context: &Context<'a>,
) -> Result<(), Error> {
    document.add_heading(Heading::new("Members", level_to_heading(4)));
    let mut list = List::default();
    for member in members {
        match member {
            Member::Collection(member) => {
                let member = member.borrow();
                let pref_label = member.preferred_label(context.language);
                let mut item_span = Span::default();
                item_span.add_text("Collection ".into());
                item_span.add_link(HyperLink::internal_with_caption_str(
                    Anchor::safe_from(&pref_label, Some("collection")),
                    &pref_label,
                ));
                list.add_item_from(item_span.into());
            }
            Member::Concept(member) => {
                let member = member.borrow();
                let pref_label = member.preferred_label(context.language);
                let mut item_span = Span::default();
                item_span.add_text("Concept ".into());
                item_span.add_link(HyperLink::internal_with_caption_str(
                    Anchor::safe_from(&pref_label, Some("concept")),
                    &pref_label,
                ));
                list.add_item_from(item_span.into());
            }
        }
    }
    if list.has_inner() {
        document.add_list(list);
    }
    Ok(())
}
