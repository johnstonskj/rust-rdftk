//!
//! This module provides the `Vocabulary` type and a set of constant `Vocabulary` values for
//! common RDF vocabularies and OWL ontologies.
//!

use crate::{iri::Iri, pname::Namespace};
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Vocabulary
// ------------------------------------------------------------------------------------------------

///
/// A Vocabulary is a light-weight way to describe a namespace, it consists of the most
/// common prefix, the namespace IRI, and an optional description string.
///
/// These values are stored in *raw* form as character slices, the methods
/// [`Vocabulary::prefix_as_namespace`] and [`Vocabulary::iri_as_iri`] will convert directly
/// to appropriate types for use in the [`IriPrefixMap`](../map/struct.IriPrefixMap.html) structure.
///
/// ## Example
///
/// ```rust
/// use rdftk_iri::{IriPrefixMap, vocab::Vocabulary};
///
/// pub const MY_NS: Vocabulary = Vocabulary::new("ex", "https://example.org/ns#")
///     .with_description("An example vocabulary.");
///
/// let map = IriPrefixMap::default().with_vocabulary(&MY_NS);
/// assert_eq!(map.len(), 1);
/// ```
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Vocabulary {
    prefix: &'static str,
    iri: &'static str,
    description: Option<&'static str>,
}

// ------------------------------------------------------------------------------------------------
// Public Constant Vocabularies
// ------------------------------------------------------------------------------------------------

///
/// The cal/ical vocabulary for calendar events.
///
/// ## Details
///
/// * **prefix** -- "cal"
/// * **IRI** -- `<http://www.w3.org/2002/12/cal/ical#>`
///
pub const VOCABULARY_CAL: Vocabulary =
    Vocabulary::new("cal", "http://www.w3.org/2002/12/cal/ical#")
        .with_description("The cal/ical vocabulary for calendar events.");

///
/// Wikipedia, as a database, in RDF.
///
/// ## Details
///
/// * **prefix** -- "dbpedia"
/// * **IRI** -- `<http://dbpedia.org/ontology/>`
///
pub const VOCABULARY_DBPEDIA: Vocabulary =
    Vocabulary::new("dbpedia", "http://dbpedia.org/ontology/")
        .with_description("Wikipedia, as a database, in RDF.");

///
/// Dublin Core Metadata Initiative's legacy "Elements" namespace.
///
/// ## Details
///
/// * **prefix** -- "dc"
/// * **IRI** -- `<http://purl.org/dc/elements/1.1/>`
/// * **See Also** -- [https://www.dublincore.org/specifications/dublin-core/dcmi-terms/](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/)
///
pub const VOCABULARY_DC_ELEMENTS: Vocabulary =
    Vocabulary::new("dc", "http://purl.org/dc/elements/1.1/")
        .with_description("Dublin Core Metadata Initiative's legacy 'Elements' namespace.");

///
/// Dublin Core Metadata Initiative's "Terms" namespace.
////
/// ## Details
///
/// * **prefix** -- "dcterms"
/// * **IRI** -- `<http://purl.org/dc/terms/>`
/// * **See Also** -- [https://www.dublincore.org/specifications/dublin-core/dcmi-terms/](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/)
//
pub const VOCABULARY_DC_TERMS: Vocabulary = Vocabulary::new("dcterms", "http://purl.org/dc/terms/")
    .with_description("Dublin Core Metadata Initiative's 'Terms' namespace.");

///
/// Description of a Project (DOAP) is an RDF Schema and XML vocabulary designed to describe
/// software projects, particularly free and open-source software.
///
/// ## Details
///
/// * **prefix** -- "doap"
/// * **IRI** -- `<http://usefulinc.com/ns/doap#>`
/// * **See Also** -- [Wikipedia](https://en.wikipedia.org/wiki/DOAP)
///
pub const VOCABULARY_DOAP: Vocabulary = Vocabulary::new("doap", "http://usefulinc.com/ns/doap#")
    .with_description("Description of a Project (DOAP) is an RDF Schema and XML vocabulary designed to describe software projects, particularly free and open-source software. See [Wikipedia](https://en.wikipedia.org/wiki/DOAP).");

///
/// FOAF (an acronym of friend of a friend) is a machine-readable ontology describing persons,
/// their activities and their relations to other people and objects.
///
/// ## Details
///
/// * **prefix** -- "foaf"
/// * **IRI** -- `<http://xmlns.com/foaf/0.1/>`
/// * **See Also** -- [Wikipedia](https://en.wikipedia.org/wiki/FOAF)
///
pub const VOCABULARY_FOAF: Vocabulary = Vocabulary::new("foaf", "http://xmlns.com/foaf/0.1/")
    .with_description("FOAF (an acronym of friend of a friend) is a machine-readable ontology describing persons, their activities and their relations to other people and objects. See [Wikipedia](https://en.wikipedia.org/wiki/FOAF).");

///
/// The GeoNames geographical database covers all countries and contains over eleven million placenames that
/// are available for download free of charge.
///
/// ## Details
///
/// * **prefix** -- "geonames"
/// * **IRI** -- `<http://www.geonames.org/ontology#>`
/// * **See Also** -- [geonames.org](http://sws.geonames.org/)
///
pub const VOCABULARY_GEO_NAMES: Vocabulary =
    Vocabulary::new("geonames", "http://www.geonames.org/ontology#")
        .with_description("See <http://sws.geonames.org/>");

///
/// A Mapping between the ISO standard thesaurus model and SKOS.
///
/// ## Details
///
/// * **prefix** -- "isothes"
/// * **IRI** -- `<http://purl.org/iso25964/skos-thes#>`
/// * **See Also** -- [Mapping ISO 25964 data model and the SKOS schema](https://www.dublincore.org/specifications/skos-thes/ns/)
///
pub const VOCABULARY_ISO_SKOS: Vocabulary =
    Vocabulary::new("isothes", "http://purl.org/iso25964/skos-thes#");

///
/// This standard supports representing and querying geospatial data on the Semantic Web. GeoSPARQL
/// defines a vocabulary for representing geospatial data in RDF, and it defines an extension to the
/// SPARQL query language for processing geospatial data.
///
/// ## Details
///
/// * **prefix** -- "opengis"
/// * **IRI** -- `<http://www.opengis.net/ont/geosparql#>`
/// * **See Also** -- [GeoSPARQL – A Geographic Query Language for RDF Data](https://www.ogc.org/standards/geosparql/)
///
pub const VOCABULARY_OPEN_GIS: Vocabulary =
    Vocabulary::new("opengis", "http://www.opengis.net/ont/geosparql#")
        .with_description("See <https://www.ogc.org/standards/geosparql/>");

///
/// This is  a core ontology for organizational structures, aimed at supporting linked data publishing of
/// organizational information across a number of domains. It is designed to allow domain-specific extensions
/// to add classification of organizations and roles, as well as extensions to support neighbouring information
/// such as organizational activities.
///
/// ## Details
///
/// * **prefix** -- "org"
/// * **IRI** -- `<http://www.w3.org/ns/org#>`
/// * **See Also** -- [The Organization Ontology](https://www.w3.org/TR/vocab-org/) and
///   [Registered Organization Vocabulary](https://www.w3.org/TR/vocab-regorg/).
///
pub const VOCABULARY_ORG: Vocabulary = Vocabulary::new("org", "http://www.w3.org/ns/org#")
    .with_description(
        "See <https://www.w3.org/TR/vocab-org/> and <https://www.w3.org/TR/vocab-regorg/>",
    );

///
/// The OWL 2 Web Ontology Language, informally OWL 2, is an ontology language for the Semantic Web with
/// formally defined meaning. OWL 2 ontologies provide classes, properties, individuals, and data values
/// and are stored as Semantic Web documents.
///
/// ## Details
///
/// * **prefix** -- "owl"
/// * **IRI** -- `<http://www.w3.org/2002/07/owl#>`
/// * **See Also** -- [OWL 2 Web Ontology Language Document Overview (Second Edition)](https://www.w3.org/TR/owl2-overview/)
///
pub const VOCABULARY_OWL: Vocabulary = Vocabulary::new("owl", "http://www.w3.org/2002/07/owl#");

///
/// The core RDF language components.
///
/// ## Details
///
/// * **prefix** -- "rdf"
/// * **IRI** -- `<http://www.w3.org/1999/02/22-rdf-syntax-ns#>`
/// * **See Also** -- [w3.org](https://www.w3.org/TR/rdf11-concepts/)
///
pub const VOCABULARY_RDF: Vocabulary =
    Vocabulary::new("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#")
    .with_description("The Resource Description Framework (RDF) is a framework for representing information in the Web. See [w3.org](https://www.w3.org/TR/rdf11-concepts/)");

///
/// RDF Schema provides a data-modelling vocabulary for RDF data; it is an extension
/// of the basic RDF vocabulary.
///
/// ## Details
///
/// * **prefix** -- "rdfs"
/// * **IRI** -- `<http://www.w3.org/2000/01/rdf-schema#">`
/// * **See Also** -- [w3.org](https://www.w3.org/TR/rdf11-schema/)
///
pub const VOCABULARY_RDF_SCHEMA: Vocabulary =
    Vocabulary::new("rdfs", "http://www.w3.org/2000/01/rdf-schema#")
        .with_description("RDF Schema provides a data-modelling vocabulary for RDF data; it is an extension of the basic RDF vocabulary. See [w3.org](https://www.w3.org/TR/rdf11-schema/).");

///
/// Semantically-Interlinked Online Communities (SIOC) is an ontology for
/// describing the information in online communities (forums, blogs, wikis, …).
///
/// ## Details
///
/// * **prefix** -- "sioc"
/// * **IRI** -- `<http://rdfs.org/sioc/ns#>`
/// * **See Also** -- [SIOC on the Semantic Web Wiki](https://www.w3.org/wiki/SIOC)
///
pub const VOCABULARY_SIOC: Vocabulary = Vocabulary::new("sioc", "http://rdfs.org/sioc/ns#")
    .with_description("See <https://www.w3.org/wiki/SIOC>");

///
/// The Simple Knowledge Organization System (SKOS) is a common data model for sharing and
/// linking knowledge organization systems via the Web.
///
/// ## Details
///
/// * **prefix** -- ""
/// * **IRI** -- `<>`
/// * **See Also** -- []()
///
pub const VOCABULARY_SKOS: Vocabulary =
    Vocabulary::new("skos", "http://www.w3.org/2004/02/skos/core#")
        .with_description(
            "The Simple Knowledge Organization System (SKOS) is a common data model for sharing and linking knowledge organization systems via the Web."
        );

///
/// SKOS-XL defines an extension for the Simple Knowledge Organization System, providing additional support
/// for describing and linking lexical entities.
///
/// ## Details
///
/// * **prefix** -- "skosxl"
/// * **IRI** -- `<http://www.w3.org/2008/05/skos-xl#>`
/// * **See Also** -- [SKOS Simple Knowledge Organization System eXtension for Labels (SKOS-XL) Namespace
///   Document - HTML Variant](https://www.w3.org/TR/skos-reference/skos-xl.html)
///
pub const VOCABULARY_SKOS_XL: Vocabulary =
    Vocabulary::new("skosxl", "http://www.w3.org/2008/05/skos-xl#");

///
/// This is a vocabulary for annotating vocabularies with examples and usage notes.
///
/// ## Details
///
/// * **prefix** -- "vann"
/// * **IRI** -- `<http://purl.org/vocab/vann/>`
///
pub const VOCABULARY_VANN: Vocabulary = Vocabulary::new("vann", "http://purl.org/vocab/vann/")
    .with_description(
        "This is a vocabulary for annotating vocabularies with examples and usage notes.",
    );

///
/// The Vocabulary of Interlinked Datasets (VoID) is concerned with metadata about RDF
/// datasets. See [w3.org](https://www.w3.org/TR/void/).
///
/// ## Details
///
/// * **prefix** -- "void"
/// * **IRI** -- `<http://rdfs.org/ns/void#>`
/// * **See Also** -- [w3.org](https://www.w3.org/TR/void/)
///
pub const VOCABULARY_VOID: Vocabulary = Vocabulary::new("void", "http://rdfs.org/ns/void#")
    .with_description(
        "The Vocabulary of Interlinked Datasets (VoID) is concerned with metadata about RDF datasets. See [w3.org](https://www.w3.org/TR/void/)."
    );

///
/// Store OpenPGP web-of-trust signatures in RDF documents.
///
/// ## Details
///
/// * **prefix** -- "wot"
/// * **IRI** -- `<http://xmlns.com/wot/0.1/>`
///
pub const VOCABULARY_WOT: Vocabulary = Vocabulary::new("wot", "http://xmlns.com/wot/0.1/")
    .with_description("Store OpenPGP web-of-trust signatures in RDF documents.");

///
/// Standard vocabulary for XML, used mainly for the `xml:base` and `xml:lang` attributes.
///
/// ## Details
///
/// * **prefix** -- "xml"
/// * **IRI** -- `<http://www.w3.org/XML/1998/namespace>`
/// * **See Also** -- [Extensible Markup Language (XML) 1.0 (Fifth Edition)](https://www.w3.org/TR/xml/),
///   [Namespaces in XML 1.0 (Third Edition)](https://www.w3.org/TR/xml-names/), and
///   [XML Base (Second Edition)](https://www.w3.org/TR/xmlbase/).
///
pub const VOCABULARY_XML: Vocabulary =
    Vocabulary::new("xml", "http://www.w3.org/XML/1998/namespace#");

///
/// XML Schema's datatypes, used extensively throughout RDF for typed literals.
///
/// ## Details
///
/// * **prefix** -- "xsd"
/// * **IRI** -- `<http://www.w3.org/2001/XMLSchema#>`
/// * **See Also** -- [W3C XML Schema Definition Language (XSD) 1.1 Part 2: Datatypes](https://www.w3.org/TR/xmlschema11-2/)
///
pub const VOCABULARY_XML_SCHEMA: Vocabulary =
    Vocabulary::new("xsd", "http://www.w3.org/2001/XMLSchema#")
        .with_description("Standard vocabulary for XML Schema, used mainly for datatypes.");

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Vocabulary
// ------------------------------------------------------------------------------------------------

impl Vocabulary {
    ///
    /// Construct a new Vocabulary instance with `prefix` and `iri`.
    ///
    /// Note that this constructor is **`const`** so that it can be used to create constant values
    /// such as [`VOCABULARY_OWL`] in this library.
    ///
    pub const fn new(prefix: &'static str, iri: &'static str) -> Self {
        Self {
            prefix,
            iri,
            description: None,
        }
    }

    ///
    /// Add a description to a vocabulary instance.
    ///
    /// Note that this constructor is **`const`** so that it can be used to create constant values
    /// such as [`VOCABULARY_OWL`] in this library.
    ///
    pub const fn with_description(mut self, description: &'static str) -> Self {
        self.description = Some(description);
        self
    }

    ///
    /// Return the *raw* string form of the vocabulary's prefix.
    ///
    pub const fn prefix(&self) -> &'static str {
        self.prefix
    }

    ///
    /// Return the vocabulary's prefix parsed into a [`Namespace`].
    ///
    /// ## Panics
    ///
    /// This method panics if the `prefix` string is not a valid value for `Name`.
    ///
    pub fn prefix_as_namespace(&self) -> Namespace {
        Namespace::from_str(&format!("{}:", self.prefix))
            .expect("provided string is not a valid Name for prefix")
    }

    ///
    /// Return the *raw* string form of the vocabulary's iri.
    ///
    pub const fn iri(&self) -> &'static str {
        self.iri
    }

    ///
    /// Return the vocabulary's iri parsed into a [`Iri`].
    ///
    /// ## Panics
    ///
    /// This method panics if the `iri` string is not a valid value for `Iri`.
    ///

    pub fn iri_as_iri(&self) -> Iri {
        Iri::from_str(self.iri).expect("provided string is not a valid Iri")
    }

    ///
    /// Return the vocabulary's optional description string.
    ///
    pub fn description(&self) -> Option<&&'static str> {
        self.description.as_ref()
    }
}
