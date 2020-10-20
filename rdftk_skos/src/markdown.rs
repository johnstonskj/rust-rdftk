/*!
Create a rich document for the provided scheme.

Details TBD

# Example

TBD

*/

use crate::simple::collection::Member;
use crate::simple::concept::ConceptRelation;
use crate::simple::properties::LabelKind;
use crate::simple::{
    standard_mappings, to_rdf_graph, Collection, Concept, Label, Labeled, LiteralProperty, Named,
    Propertied, Scheme, ToURI,
};
use rdftk_core::graph::PrefixMappings;
use rdftk_core::DataType;
use rdftk_io::turtle::TurtleWriter;
use rdftk_io::GraphWriter;
use rdftk_iri::IRIRef;
use rdftk_memgraph::Mappings;
use rdftk_names::xsd;
use std::cell::RefCell;
use std::io::{Result, Write};
use std::rc::Rc;

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

pub fn write_markdown(
    w: &mut impl Write,
    scheme: &Scheme,
    language: &str,
    default_namespace: Option<IRIRef>,
) -> Result<()> {
    let context = Context::new(scheme, language);

    write_entity_header(w, scheme, "s", "Scheme", 1, &context)?;

    if scheme.has_top_concepts() {
        write_line(w)?;

        write_concept_tree(w, scheme, &context)?;

        write_line(w)?;

        writeln!(w, "{}", header(2, "Concepts"))?;
        writeln!(w)?;

        let mut concepts = scheme.concepts_flattened();
        concepts.sort_by_key(|concept| concept.borrow().preferred_label(language));

        for concept in &concepts {
            for (relation, related) in concept.borrow().concepts() {
                if relation == &ConceptRelation::Narrower
                    || relation == &ConceptRelation::NarrowerPartitive
                    || relation == &ConceptRelation::NarrowerInstantial
                //                    || relation == &ConceptRelation::Related
                {
                    related
                        .borrow_mut()
                        .add_related_concept(relation.inverse(), concept.clone());
                }
            }
        }

        for concept in &concepts {
            write_concept(w, &*concept.borrow(), &context)?;
        }
    }

    write_line(w)?;

    if scheme.has_top_collections() {
        writeln!(w, "{}", header(2, "Collections"))?;
        writeln!(w)?;
        for collection in &context.collections {
            write_collection(w, &collection.borrow(), &context)?;
        }
    }

    write_line(w)?;

    writeln!(w, "{}", header(2, "Appendix - RDF"))?;
    writeln!(w)?;
    writeln!(w, "```turtle")?;
    let graph = to_rdf_graph(&scheme, default_namespace);
    let writer = TurtleWriter::default();
    writer.write(w, &graph)?;
    writeln!(w, "```")?;

    Ok(())
}

pub fn write_concept_tree_markdown<'a>(
    w: &mut impl Write,
    scheme: &Scheme,
    language: &str,
) -> Result<()> {
    write_concept_tree(w, scheme, &Context::new(scheme, language))
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> Context<'a> {
    fn new(scheme: &'a Scheme, language: &'a str) -> Self {
        // make collection mappings!
        let mut collections = scheme.collections_flattened();
        collections.sort_by_key(|collection| collection.borrow().preferred_label(language));
        Self {
            ns_mappings: standard_mappings(),
            collections,
            language,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn write_entity_header<'a>(
    w: &mut impl Write,
    obj: &(impl Named + Labeled + Propertied),
    anchor_prefix: &str,
    header_text: &str,
    depth: usize,
    context: &Context<'a>,
) -> Result<()> {
    let label = obj.preferred_label(context.language);
    writeln!(
        w,
        "{}: {}",
        anchored_header(
            depth,
            header_text,
            &label_to_fragment(&label, anchor_prefix)
        ),
        label
    )?;

    writeln!(w)?;
    writeln!(w, "[<{}>]({})", obj.uri(), obj.uri())?;
    writeln!(w)?;

    if obj.has_labels() {
        writeln!(w, "{}", header(depth + 1, "Labels"))?;
        write_labels(w, obj.labels().iter().collect(), &context)?;
    }

    if obj.has_properties() {
        writeln!(w, "{}", header(depth + 1, "Other Properties"))?;
        writeln!(w)?;
        write_other_properties(w, obj.properties().iter().collect(), &context)?;
    }
    Ok(())
}

fn write_labels<'a>(w: &mut impl Write, labels: Vec<&Label>, context: &Context<'a>) -> Result<()> {
    let mut labels = labels;
    labels.sort_by_key(|label| label.kind());
    let mut current_kind: Option<&LabelKind> = None;
    for label in labels.iter() {
        if Some(label.kind()) != current_kind {
            current_kind = Some(label.kind());
            writeln!(w)?;
            writeln!(
                w,
                "> **{}**",
                match context.ns_mappings.compress(label.kind().to_uri()) {
                    None => label.kind().to_uri().to_string(),
                    Some(qname) => qname.to_string(),
                }
            )?;
            writeln!(w, ">")?;
            writeln!(w, "> | Label Text | Language |")?;
            writeln!(w, "> |------------|----------|")?;
        }
        let lang = label.language();
        writeln!(
            w,
            "> | {} | {} |",
            label.text(),
            if lang == context.language {
                format!("**{}**", lang)
            } else {
                lang.to_string()
            }
        )?;
    }
    writeln!(w)?;
    Ok(())
}

fn write_other_properties<'a>(
    w: &mut impl Write,
    properties: Vec<&LiteralProperty>,
    context: &Context<'a>,
) -> Result<()> {
    let mut properties = properties;
    properties.sort_by_key(|property| property.predicate().to_string());
    writeln!(w, "> | Predicate | Literal Form | Data Type | Language |")?;
    writeln!(w, "> |-----------|--------------|-----------|----------|")?;
    for property in properties.iter() {
        writeln!(
            w,
            "> | {} | {} | {} | {} |",
            match context.ns_mappings.compress(property.predicate().clone()) {
                None => property.predicate().to_string(),
                Some(qname) => qname.to_string(),
            },
            property.value().lexical_form(),
            match property.value().data_type() {
                None => String::new(),
                Some(dt) => match context.ns_mappings.compress(data_type_uri(dt)) {
                    None => property.predicate().to_string(),
                    Some(qname) => qname.to_string(),
                },
            },
            match property.value().language() {
                None => String::new(),
                Some(lang) =>
                    if lang == context.language {
                        format!("**{}**", lang)
                    } else {
                        lang.to_string()
                    },
            }
        )?;
    }
    writeln!(w)?;
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

fn write_concept<'a>(w: &mut impl Write, concept: &Concept, context: &Context<'a>) -> Result<()> {
    write_entity_header(w, concept, "c", "Concept", 3, &context)?;

    if concept.has_concepts() {
        write_concept_relations(w, concept, context)?;
    }

    write_collection_membership(w, concept.uri(), context)?;

    Ok(())
}

fn write_concept_relations<'a>(
    w: &mut impl Write,
    concept: &Concept,
    context: &Context<'a>,
) -> Result<()> {
    writeln!(w, "{}", header(4, "Related Concepts"))?;
    writeln!(w)?;
    writeln!(w, "> | Relationship | Concept IRI |")?;
    writeln!(w, "> |--------------|-------------|")?;
    for (relation, related) in concept.concepts() {
        let related = related.borrow();
        let label = related.preferred_label(context.language);
        writeln!(
            w,
            "> | {} | [{}](#{}) |",
            match context.ns_mappings.compress(relation.to_uri()) {
                None => relation.to_uri().to_string(),
                Some(qname) => qname.to_string(),
            },
            label,
            &label_to_fragment(&label, "c")
        )?;
    }
    writeln!(w)?;
    Ok(())
}

fn write_concept_tree<'a>(
    w: &mut impl Write,
    scheme: &Scheme,
    context: &Context<'a>,
) -> Result<()> {
    writeln!(w, "{}", header(2, "Concept Hierarchy"))?;
    writeln!(w)?;
    for concept in scheme.top_concepts().map(|concept| concept.borrow()) {
        let label = concept.preferred_label(context.language);
        writeln!(
            w,
            "{} **[{}](#{})**",
            list_item(0),
            label,
            label_to_fragment(&label, "c")
        )?;
        if concept.has_concepts() {
            write_concept_tree_inner(w, concept.concepts().collect(), 1, context)?;
        }
    }

    writeln!(w)
}

fn write_concept_tree_inner<'a>(
    w: &mut impl Write,
    current_concepts: Vec<&(ConceptRelation, Rc<RefCell<Concept>>)>,
    current_depth: usize,
    context: &Context<'a>,
) -> Result<()> {
    let mut current_concepts = current_concepts;
    current_concepts.sort_by_key(|(_, concept)| concept.borrow().preferred_label(context.language));
    for (_, concept) in current_concepts {
        let concept = concept.borrow();
        let label = concept.preferred_label(context.language);
        writeln!(
            w,
            "{} [{}](#{})",
            list_item(current_depth),
            label,
            label_to_fragment(&label, "c")
        )?;
        if concept.has_concepts() {
            write_concept_tree_inner(w, concept.concepts().collect(), current_depth + 1, context)?;
        }
    }
    Ok(())
}

fn list_item(depth: usize) -> String {
    format!("{}*", format!("{: <1$}", "", depth * 2))
}

fn write_collection<'a>(
    w: &mut impl Write,
    collection: &Collection,
    context: &Context<'a>,
) -> Result<()> {
    write_entity_header(
        w,
        collection,
        "cc",
        if collection.is_ordered() {
            "Ordered Collection"
        } else {
            "Collection"
        },
        3,
        &context,
    )?;

    if collection.has_members() {
        write_collection_members(w, collection.members(), context)?;
    }

    write_collection_membership(w, collection.uri(), context)
}

fn write_collection_membership<'a>(
    w: &mut impl Write,
    member_uri: &IRIRef,
    context: &Context<'a>,
) -> Result<()> {
    let in_collections: Vec<&Rc<RefCell<Collection>>> = context
        .collections
        .iter()
        .filter(|collection| collection.borrow().has_member(member_uri))
        .collect();

    if !in_collections.is_empty() {
        writeln!(w, "{}", header(4, "In Collections"))?;
        writeln!(w)?;
        for collection in in_collections {
            let collection = collection.borrow();
            let pref_label = collection.preferred_label(context.language);
            writeln!(
                w,
                "* [{}]({})",
                pref_label,
                label_to_fragment(&pref_label, "cc")
            )?;
        }
        writeln!(w)?;
    }
    Ok(())
}

fn write_collection_members<'a>(
    w: &mut impl Write,
    members: impl Iterator<Item = &'a Member>,
    context: &Context<'a>,
) -> Result<()> {
    writeln!(w, "{}", header(4, "Members"))?;
    writeln!(w)?;
    for member in members {
        match member {
            Member::Collection(member) => {
                let member = member.borrow();
                let pref_label = member.preferred_label(context.language);
                writeln!(
                    w,
                    "* Collection [{}]({})",
                    pref_label,
                    label_to_fragment(&pref_label, "cc")
                )?;
            }
            Member::Concept(member) => {
                let member = member.borrow();
                let pref_label = member.preferred_label(context.language);
                writeln!(
                    w,
                    "* Concept [{}]({})",
                    pref_label,
                    label_to_fragment(&pref_label, "c")
                )?;
            }
        }
    }
    writeln!(w)
}

#[inline]
fn header(depth: usize, text: &str) -> String {
    format!("{} {}", format!("{:#<1$}", "", depth), text)
}

#[inline]
fn write_line(w: &mut impl Write) -> Result<()> {
    writeln!(w, "----------")?;
    writeln!(w)
}

#[inline]
fn anchored_header(depth: usize, text: &str, anchor: &str) -> String {
    format!(
        "{} <a name=\"{}\">{}",
        format!("{:#<1$}", "", depth),
        anchor,
        text
    )
}

#[inline]
fn label_to_fragment(label: &str, prefix: &str) -> String {
    format!("{}__{}", prefix, label.to_lowercase().replace(" ", "_"))
}
