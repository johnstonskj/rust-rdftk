use rdftk_core::{DataType, Literal};
use rdftk_iri::{IRIRef, IRI};
use rdftk_names::{dc, xsd};
use rdftk_skos::simple::{to_rdf_graph, Labeled, LiteralProperty, Propertied, Scheme};
use std::str::FromStr;

fn make_unesco_computer() -> Scheme {
    // Taken from http://vocabularies.unesco.org/browser/rest/v1/thesaurus/data?uri=http%3A%2F%2Fvocabularies.unesco.org%2Fthesaurus%2Fconcept2258&format=text/turtle
    let mut scheme: Scheme = Scheme::new(&IRIRef::from(
        IRI::from_str("http://vocabularies.unesco.org/thesaurus").unwrap(),
    ));
    scheme.add_preferred_label("UNESCO Thesaurus", "en");
    scheme.add_preferred_label("Thésaurus de l'UNESCO", "fr");
    scheme.add_preferred_label("Тезаурус ЮНЕСКО", "ru");
    scheme.add_preferred_label("Tesauro de la UNESCO", "es");
    scheme.add_property(LiteralProperty::description("The UNESCO thesaurus."));

    let computers = scheme.new_top_concept(&IRIRef::from(
        IRI::from_str("http://vocabularies.unesco.org/thesaurus/concept534").unwrap(),
    ));
    let mut computers = computers.borrow_mut();
    computers.add_preferred_label("حواسيب", "ar");
    computers.add_preferred_label("Computers", "en");
    computers.add_preferred_label("Ordinateur", "fr");
    computers.add_preferred_label("Компьютеры", "ru");

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
    use rdftk_core::graph::Graph;

    let scheme = make_unesco_computer();

    let statements = to_rdf_graph(&scheme, None).statements();

    for statement in &statements {
        println!("{}", statement);
    }
    assert_eq!(statements.len(), 42);
}

const MARKDOWN: &str = include_str!("simple_thesaurus.md");

#[test]
fn test_simple_thesaurus_to_markdown_all() {
    use rdftk_skos::markdown::write_markdown;
    use std::io::Cursor;

    let scheme = make_unesco_computer();

    let mut buffer = Cursor::new(Vec::new());
    let result = write_markdown(&mut buffer, &scheme, "en", None);

    assert!(result.is_ok());

    //assert_eq!(String::from_utf8(buffer.into_inner()).unwrap(), MARKDOWN);

    assert!(String::from_utf8(buffer.into_inner())
        .unwrap()
        .starts_with(MARKDOWN));
}

const MARKDOWN_TREE: &str = include_str!("simple_thesaurus_tree.md");

#[test]
fn test_simple_thesaurus_to_markdown_tree() {
    use rdftk_skos::markdown::write_concept_tree_markdown;
    use std::io::Cursor;

    let scheme = make_unesco_computer();

    let mut buffer = Cursor::new(Vec::new());
    let result = write_concept_tree_markdown(&mut buffer, &scheme, "en");

    assert!(result.is_ok());

    assert_eq!(
        String::from_utf8(buffer.into_inner()).unwrap(),
        MARKDOWN_TREE
    );
}
