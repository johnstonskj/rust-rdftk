use pretty_assertions::assert_eq;
use rdftk_core::graph::PrefixMappings;
use rdftk_core::{DataType, Literal};
use rdftk_iri::{IRIRef, IRI};
use rdftk_names::{dc, owl, xsd};
use rdftk_skos::document::make_document_with_mappings;
use rdftk_skos::model::{
    standard_mappings, to_rdf_graph, Labeled, LiteralProperty, Propertied, Scheme,
};
use somedoc::write::markdown::MarkdownFlavor;
use somedoc::write::write_document_to_string;
use std::str::FromStr;

fn make_unesco_computer() -> Scheme {
    // Taken from http://vocabularies.unesco.org/browser/rest/v1/thesaurus/data?uri=http%3A%2F%2Fvocabularies.unesco.org%2Fthesaurus%2Fconcept2258&format=text/turtle
    let mut scheme: Scheme = Scheme::new(&IRIRef::from(
        IRI::from_str("http://vocabularies.unesco.org/thesaurus").unwrap(),
    ));
    scheme.define("The UNESCO thesaurus.", "en");
    scheme.add_preferred_label("UNESCO Thesaurus", "en");
    scheme.add_preferred_label("Thésaurus de l'UNESCO", "fr");
    scheme.add_preferred_label("Тезаурус ЮНЕСКО", "ru");
    scheme.add_preferred_label("Tesauro de la UNESCO", "es");

    let computers = scheme.new_top_concept(&IRIRef::from(
        IRI::from_str("http://vocabularies.unesco.org/thesaurus/concept534").unwrap(),
    ));
    let mut computers = computers.borrow_mut();
    computers.add_preferred_label("حواسيب", "ar");
    computers.add_preferred_label("Computers", "en");
    computers.add_preferred_label("Ordinateur", "fr");
    computers.add_preferred_label("Компьютеры", "ru");
    computers.add_external_relation(
        owl::equivalent_class().clone(),
        IRI::from_str("http://dbpedia.org/ontology/InformationAppliance")
            .unwrap()
            .into(),
    );

    let analog_computers = computers.sub_concept(&IRIRef::from(
        IRI::from_str("http://vocabularies.unesco.org/thesaurus/concept2258").unwrap(),
    ));

    let mut ref_analog_computers = analog_computers.borrow_mut();
    ref_analog_computers.add_preferred_label("Calculateur analogique", "fr");
    ref_analog_computers.add_preferred_label("Аналоговые компьютеры", "ru");
    ref_analog_computers.add_preferred_label("Analog Computers", "en");
    ref_analog_computers.add_preferred_label("Ordenador analógico", "es");
    ref_analog_computers.add_preferred_label("حواسب تناظرية", "ar");
    ref_analog_computers.add_hidden_label("Ordenador analogico", "es");
    ref_analog_computers.add_property(LiteralProperty::new(
        dc::terms::modified().clone(),
        Literal::with_type(
            "2019-12-15T14:00:02Z",
            DataType::Other(xsd::date_time().clone()),
        ),
    ));

    let domain_collection = scheme.new_top_collection(
        &IRIRef::from(IRI::from_str("http://vocabularies.unesco.org/thesaurus/domain5").unwrap()),
        false,
    );
    let mut domain_collection = domain_collection.borrow_mut();
    domain_collection.add_preferred_label("معلومات واتصالات", "ar");
    domain_collection.add_preferred_label("Информация и коммуникация", "ru");
    domain_collection.add_preferred_label("Information et communication", "fr");
    domain_collection.add_preferred_label("Information and communication", "en");
    domain_collection.add_preferred_label("Información y comunicación", "es");

    let mt_collection = domain_collection.sub_collection(
        &IRIRef::from(IRI::from_str("http://vocabularies.unesco.org/thesaurus/mt5.45").unwrap()),
        false,
    );
    let mut mt_collection = mt_collection.borrow_mut();
    mt_collection.add_preferred_label("تكنولوجيا المعلومات (الأجهزة)", "ar");
    mt_collection.add_preferred_label("Информационная технология (технические средства)", "ru");
    mt_collection.add_preferred_label("Technologie de l'information (équipements)", "fr");
    mt_collection.add_preferred_label("Information technology (hardware)", "en");
    mt_collection.add_preferred_label("Tecnología de la información (equipos)", "es");
    mt_collection.add_member_concept(analog_computers.clone());

    scheme
}

#[test]
fn test_simple_thesaurus() {
    let scheme = make_unesco_computer();

    println!("{:#?}", scheme);
}

#[test]
fn test_simple_thesaurus_to_rdf() {
    let scheme = make_unesco_computer();

    let graph = to_rdf_graph(&scheme, None);
    let graph = graph.borrow();
    assert_eq!(graph.len(), 43);

    for statement in graph.statements() {
        println!("{}", statement);
    }
}

const MARKDOWN: &str = include_str!("simple_thesaurus.md");

#[test]
fn test_simple_thesaurus_to_markdown() {
    let scheme = make_unesco_computer();

    let mappings = standard_mappings();
    {
        let mut mappings = mappings.borrow_mut();
        mappings.insert(
            "dbpedia",
            IRI::from_str("http://dbpedia.org/ontology/")
                .unwrap()
                .into(),
        );
        mappings.set_default_namespace(
            IRI::from_str("http://vocabularies.unesco.org/thesaurus/")
                .unwrap()
                .into(),
        );
    }

    let result = make_document_with_mappings(&scheme, "en", mappings);

    assert!(result.is_ok());
    let doc = result.unwrap();

    let md = write_document_to_string(&doc, MarkdownFlavor::XWiki.into()).unwrap();
    println!("{}", md);

    let expected = MARKDOWN.replace("\r\n", "\n");

    // This allows the use of pretty_assertions to produce a nice diff if the assert_eq fails.
    let md = &md[0..expected.len()];
    assert_eq!(md, expected);
}
