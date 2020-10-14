/*!
Create a rich document for the provided scheme.

Details TBD

# Example

TBD

*/

use crate::model::{
    standard_mappings, to_rdf_graph, Collection, Concept, Labeled, LiteralProperty, Named,
    ObjectProperty, Propertied, Scheme,
};
use crate::ns;
use rdftk_core::graph::PrefixMappings;
use rdftk_core::DataType;
use rdftk_io::turtle::TurtleWriter;
use rdftk_io::GraphWriter;
use rdftk_iri::IRIRef;
use rdftk_memgraph::Mappings;
use rdftk_names::xsd;
use std::io::{Result, Write};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

struct Context<'a> {
    mappings: Mappings,
    scheme: &'a Scheme,
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

    write_named_obj_header(w, scheme, "Scheme", 1, &context)?;

    if scheme.has_properties() {
        write_labeled_obj(w, scheme, 2, &context)?;
    }

    if scheme.has_concepts() {
        writeln!(w, "{}", header(2, "Concepts"))?;
        let mut sorted: Vec<&Concept> = scheme.concepts().collect();
        sorted.sort_by_key(|&a| sort_label(a, &context));
        for concept in &sorted {
            write_concept(w, concept, &context)?;
        }
        writeln!(w, "{}", header(2, "Concept Tree"))?;
        write_concept_tree(w, &sorted, &context)?;
    }

    if scheme.has_collections() {
        writeln!(w, "{}", header(2, "Collections"))?;
        let mut sorted: Vec<&Collection> = scheme.collections().collect();
        sorted.sort_by_key(|&a| sort_label(a, &context));
        for collection in sorted {
            write_collection(w, &collection, &context)?;
        }
    }

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
    let context = Context::new(scheme, language);
    let mut sorted: Vec<&Concept> = scheme.concepts().collect();
    sorted.sort_by_key(|&a| sort_label(a, &context));
    writeln!(w, "{}", header(2, "Concept Tree"))?;
    write_concept_tree(w, &sorted, &context)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> Context<'a> {
    fn new(scheme: &'a Scheme, language: &'a str) -> Self {
        Self {
            mappings: standard_mappings(),
            scheme,
            language,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn sort_label<'a>(thing: &(impl Named + Labeled), context: &Context<'a>) -> String {
    match thing.preferred_label(context.language) {
        None => thing.uri().to_string(),
        Some(s) => s,
    }
}

fn write_named_obj_header<'a>(
    w: &mut impl Write,
    obj: &(impl Named + Labeled),
    header_text: &str,
    depth: usize,
    context: &Context<'a>,
) -> Result<()> {
    if let Some(label) = obj.preferred_label(context.language) {
        writeln!(w, "{}: {}", header(depth, header_text), label)?;
    } else {
        writeln!(w, "{}", header(depth, header_text))?;
    }

    writeln!(w)?;
    writeln!(w, "[<{}>]({})", obj.uri(), obj.uri())?;
    writeln!(w)
}

fn write_labeled_obj<'a>(
    w: &mut impl Write,
    obj: &impl Labeled,
    header_depth: usize,
    context: &Context<'a>,
) -> Result<()> {
    writeln!(w, "{}", header(header_depth, "Labels"))?;
    write_labels(w, &mut obj.labels(), &context)?;

    let mut other_properties: Vec<&LiteralProperty> = obj
        .properties()
        .into_iter()
        .filter(|property| {
            property.predicate() != ns::pref_label()
                && property.predicate() != ns::alt_label()
                && property.predicate() != ns::hidden_label()
        })
        .collect();
    if !other_properties.is_empty() {
        writeln!(w, "{}", header(header_depth, "Other Properties"))?;
        write_other_properties(w, &mut other_properties, &context)?;
    }
    Ok(())
}

fn write_labels<'a>(
    w: &mut impl Write,
    labels: &mut Vec<&LiteralProperty>,
    context: &Context<'a>,
) -> Result<()> {
    labels.sort_by(|a, b| {
        b.predicate()
            .to_string()
            .partial_cmp(&a.predicate().to_string())
            .unwrap()
    });
    let mut current_kind = &Default::default();
    for label in labels {
        if label.predicate() != current_kind {
            current_kind = label.predicate();
            writeln!(w)?;
            writeln!(
                w,
                "> **{}**",
                match context.mappings.compress(label.predicate().clone()) {
                    None => label.predicate().to_string(),
                    Some(qname) => qname.to_string(),
                }
            )?;
            writeln!(w, ">")?;
            writeln!(w, "> | Label Text | Language |")?;
            writeln!(w, "> |------------|----------|")?;
        }
        writeln!(
            w,
            "> | {} | {} |",
            label.value().lexical_form(),
            match label.value().language() {
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

fn write_other_properties<'a>(
    w: &mut impl Write,
    properties: &mut Vec<&LiteralProperty>,
    context: &Context<'a>,
) -> Result<()> {
    properties.sort_by(|a, b| {
        a.predicate()
            .to_string()
            .partial_cmp(&b.predicate().to_string())
            .unwrap()
    });
    writeln!(w)?;
    writeln!(w, "> | Predicate | Literal Form | Data Type | Language |")?;
    writeln!(w, "> |-----------|--------------|-----------|----------|")?;
    for property in properties {
        writeln!(
            w,
            "> | {} | {} | {} | {} |",
            match context.mappings.compress(property.predicate().clone()) {
                None => property.predicate().to_string(),
                Some(qname) => qname.to_string(),
            },
            property.value().lexical_form(),
            match property.value().data_type() {
                None => String::new(),
                Some(dt) => match context.mappings.compress(data_type_uri(dt)) {
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
    writeln!(w)?;
    write_named_obj_header(
        w,
        concept,
        if concept.is_top_concept() {
            "Top Concept"
        } else {
            "Concept"
        },
        3,
        &context,
    )?;

    if concept.has_properties() {
        write_labeled_obj(w, concept, 4, &context)?;
    }

    if concept.has_relations() {
        write_relationships(w, concept.relations(), context)?;
    }

    write_collection_membership(w, concept.uri(), context)
}

fn write_concept_tree<'a>(
    w: &mut impl Write,
    concepts: &Vec<&Concept>,
    context: &Context<'a>,
) -> Result<()> {
    writeln!(w)?;
    write_concept_tree_inner(
        w,
        &concepts
            .iter()
            .filter(|concept| concept.is_top_concept())
            .cloned()
            .collect(),
        0,
        context,
    )?;
    writeln!(w)
}

fn write_concept_tree_inner<'a>(
    w: &mut impl Write,
    current_concepts: &Vec<&Concept>,
    current_depth: usize,
    context: &Context<'a>,
) -> Result<()> {
    for concept in current_concepts {
        writeln!(
            w,
            "{} {}",
            list_item(current_depth),
            match concept.preferred_label(context.language) {
                None => concept.uri().to_string(),
                Some(label) => label,
            }
        )?;
        let next_level: Vec<&Concept> = concept
            .relations()
            .filter(|r| r.predicate() == ns::narrower())
            .map(|r| context.scheme.concept(r.other()).unwrap())
            .collect();
        write_concept_tree_inner(w, &next_level, current_depth + 1, context)?;
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
    writeln!(w)?;
    write_named_obj_header(
        w,
        collection,
        if collection.is_ordered() {
            "Ordered Collection"
        } else {
            "Collection"
        },
        3,
        &context,
    )?;

    if collection.has_properties() {
        write_labeled_obj(w, collection, 4, &context)?;
    }

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
    let in_collections: Vec<&Collection> = context
        .scheme
        .collections()
        .filter(|collection| collection.members().any(|member| member == member_uri))
        .collect();

    if !in_collections.is_empty() {
        writeln!(w, "{}", header(4, "In Collections"))?;
        writeln!(w)?;
        for collection in in_collections {
            let pref_label = collection.preferred_label(context.language);
            if let Some(label) = pref_label {
                writeln!(w, "* [{}]({})", label, collection.uri())?;
            } else {
                writeln!(w, "* [{}]({})", collection.uri(), collection.uri())?;
            }
        }
        writeln!(w)?;
    }
    Ok(())
}

fn write_collection_members<'a>(
    w: &mut impl Write,
    members: impl Iterator<Item = &'a IRIRef>,
    context: &Context<'a>,
) -> Result<()> {
    writeln!(w, "{}", header(4, "Members"))?;
    writeln!(w)?;
    for member in members {
        writeln!(w, "* [{}]({})", uri_to_label(member, context), member)?;
    }
    writeln!(w)
}

fn write_relationships<'a>(
    w: &mut impl Write,
    relations: impl Iterator<Item = &'a ObjectProperty>,
    context: &Context<'a>,
) -> Result<()> {
    writeln!(w, "{}", header(4, "Related Concepts"))?;
    writeln!(w)?;
    writeln!(w, "> | Relationship | Concept IRI |")?;
    writeln!(w, "> |--------------|-------------|")?;
    for relation in relations {
        writeln!(
            w,
            "> | {} | [{}]({}) |",
            match context.mappings.compress(relation.predicate().clone()) {
                None => relation.predicate().to_string(),
                Some(qname) => qname.to_string(),
            },
            uri_to_label(relation.other(), context),
            relation.other()
        )?;
    }
    writeln!(w)?;
    Ok(())
}

fn uri_to_label<'a>(uri: &IRIRef, context: &Context<'a>) -> String {
    if let Some(concept) = context.scheme.concepts().find(|c| c.uri() == uri) {
        if let Some(label) = concept.preferred_label(context.language) {
            return label;
        }
    }

    if let Some(collection) = context.scheme.collections().find(|c| c.uri() == uri) {
        if let Some(label) = collection.preferred_label(context.language) {
            return label;
        }
    }

    uri.to_string()
}

#[inline]
fn header(depth: usize, text: &str) -> String {
    format!("{} {}", format!("{:#<1$}", "", depth), text)
}
