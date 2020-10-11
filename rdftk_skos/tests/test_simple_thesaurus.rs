use rdftk_core::{DataType, Literal};
use rdftk_graph::Graph;
use rdftk_names::{dc, xsd};
use rdftk_skos::markdown::{write_concept_tree_markdown, write_markdown};
use rdftk_skos::model::{
    to_rdf_graph, Collection, Concept, LiteralProperty, Named, Propertied, Scheme,
};
use std::io::Cursor;

fn make_unesco_computer() -> Scheme {
    // Taken from http://vocabularies.unesco.org/browser/rest/v1/thesaurus/data?uri=http%3A%2F%2Fvocabularies.unesco.org%2Fthesaurus%2Fconcept2258&format=text/turtle
    let mut scheme: Scheme =
        Scheme::new("http://vocabularies.unesco.org/thesaurus".parse().unwrap());
    scheme.add_property(LiteralProperty::preferred_label_with(
        "UNESCO Thesaurus",
        "en",
    ));
    scheme.add_property(LiteralProperty::preferred_label_with(
        "Thésaurus de l'UNESCO",
        "fr",
    ));
    scheme.add_property(LiteralProperty::preferred_label_with(
        "Тезаурус ЮНЕСКО",
        "ru",
    ));
    scheme.add_property(LiteralProperty::preferred_label_with(
        "Tesauro de la UNESCO",
        "es",
    ));
    scheme.add_property(LiteralProperty::description("The UNESCO thesaurus."));

    let mut computers: Concept = Concept::new(
        "http://vocabularies.unesco.org/thesaurus/concept534"
            .parse()
            .unwrap(),
    );
    computers.add_property(LiteralProperty::preferred_label_with("حواسيب", "ar"));
    computers.add_property(LiteralProperty::preferred_label_with("Computers", "en"));
    computers.add_property(LiteralProperty::preferred_label_with("Ordinateur", "fr"));
    computers.add_property(LiteralProperty::preferred_label_with("Компьютеры", "ru"));

    let mut analog_computers = computers.narrower(
        "http://vocabularies.unesco.org/thesaurus/concept2258"
            .parse()
            .unwrap(),
    );
    analog_computers.add_property(LiteralProperty::preferred_label_with(
        "Calculateur analogique",
        "fr",
    ));
    analog_computers.add_property(LiteralProperty::preferred_label_with(
        "Аналоговые компьютеры",
        "ru",
    ));
    analog_computers.add_property(LiteralProperty::preferred_label_with(
        "Analog Computers",
        "en",
    ));
    analog_computers.add_property(LiteralProperty::preferred_label_with(
        "Ordenador analógico",
        "es",
    ));
    analog_computers.add_property(LiteralProperty::preferred_label_with("حواسب تناظرية", "ar"));
    analog_computers.add_property(LiteralProperty::hidden_label_with(
        "Ordenador analogico",
        "es",
    ));
    analog_computers.add_property(LiteralProperty::new(
        dc::terms::modified(),
        Literal::with_type(
            "2019-12-15T14:00:02Z",
            DataType::Other(Box::from(xsd::date_time())),
        ),
    ));

    let mut collection = Collection::new(
        "http://vocabularies.unesco.org/thesaurus/mt5.45"
            .parse()
            .unwrap(),
    );
    collection.add_property(LiteralProperty::preferred_label_with(
        "تكنولوجيا المعلومات (الأجهزة)",
        "ar",
    ));
    collection.add_property(LiteralProperty::preferred_label_with(
        "Информационная технология (технические средства)",
        "ru",
    ));
    collection.add_property(LiteralProperty::preferred_label_with(
        "Technologie de l'information (équipements)",
        "fr",
    ));
    collection.add_property(LiteralProperty::preferred_label_with(
        "Information technology (hardware)",
        "en",
    ));
    collection.add_property(LiteralProperty::preferred_label_with(
        "Tecnología de la información (equipos)",
        "es",
    ));
    collection.add_member(analog_computers.uri().clone());

    let mut domain_collection = Collection::new(
        "http://vocabularies.unesco.org/thesaurus/domain5"
            .parse()
            .unwrap(),
    );
    domain_collection.add_property(LiteralProperty::preferred_label_with(
        "معلومات واتصالات",
        "ar",
    ));
    domain_collection.add_property(LiteralProperty::preferred_label_with(
        "Информация и коммуникация",
        "ru",
    ));
    domain_collection.add_property(LiteralProperty::preferred_label_with(
        "Information et communication",
        "fr",
    ));
    domain_collection.add_property(LiteralProperty::preferred_label_with(
        "Information and communication",
        "en",
    ));
    domain_collection.add_property(LiteralProperty::preferred_label_with(
        "Información y comunicación",
        "es",
    ));
    domain_collection.add_member(collection.uri().clone());

    scheme.add_top_concept(computers);
    scheme.add_concept(analog_computers);
    scheme.add_collection(collection);
    scheme.add_collection(domain_collection);

    scheme
}

#[test]
fn test_simple_thesaurus_to_rdf() {
    let scheme = make_unesco_computer();

    let statements = to_rdf_graph(&scheme, None).statements();

    assert_eq!(statements.len(), 41);
}

const MARKDOWN: &str = include_str!("simple_thesaurus.md");

#[test]
fn test_simple_thesaurus_to_markdown() {
    let scheme = make_unesco_computer();

    let mut buffer = Cursor::new(Vec::new());
    let result = write_markdown(&mut buffer, &scheme, "en", None);

    assert!(result.is_ok());

    assert!(String::from_utf8(buffer.into_inner())
        .unwrap()
        .starts_with(MARKDOWN));
}

const MARKDOWN_TREE: &str = include_str!("simple_thesaurus_tree.md");

#[test]
fn test_simple_thesaurus_to_markdown_tree() {
    let scheme = make_unesco_computer();

    let mut buffer = Cursor::new(Vec::new());
    let result = write_concept_tree_markdown(&mut buffer, &scheme, "en");

    assert!(result.is_ok());

    assert_eq!(
        String::from_utf8(buffer.into_inner()).unwrap(),
        MARKDOWN_TREE
    );
}
