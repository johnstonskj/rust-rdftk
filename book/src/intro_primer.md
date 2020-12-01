# RDF Primer

RDF is a data model for describing things in a manner that allows for open extension, easy merging of data, built on 
the same concepts as the web itself and which allows for the inference of facts from those asserted. This data model 
is abstract in the sense that it is described entirely based upon its semantics and not on its syntax. Multiple 
serialization forms, representations, are described for RDF data but none describe RDF itself. 

This brief primer introduces enough to get started on understanding the RDFtk set of crates and more details will be
introduced in crate-specific sections. There is a more complete [W3C RDF Primer](https://www.w3.org/TR/rdf-primer/) 
that has a lot more information than presented here.

## Resources, Statements, and Literals

The data model for RDF is surprisingly simple, the core component of which is the **Statement**. A statement asserts
a fact based on the linguistic form *subject* *predicate* *object* (Property is sometimes used instead of the more
formal predicate). Examples might be:

* *Simon* *authored* *this*
* <<https://github.com/johnstonskj/rust-rdftk.git>> *is-a* *repository*
* <<http://crates.io/crates/rdftk_core>> *description* "The core data model."

1. The **Subject** of a statement is a **Resource** which is either addressed with an [IRI](https://tools.ietf.org/html/rfc3987)
(Internationalized Resource Identifiers), or an internally generated identifier commonly known as a **Blank Node**.
1. The **Predicate** is some relationship between the subject and object and is always an IRI. 
1. The **Object** of a statement is either a resource or a **Literal**. In the case of a resource the same details apply as
for the subject. 
1. A **Literal** is a string value that may have an associated **Language** *or* **Data Type** but not both.
   1. The language will be expressed as an ISO language identifier, "en", "en_us", etc.
   1. The data type will be expressed as an IRI usually to one of the defined [XML Schema Data Types](https://www.w3.org/TR/xmlschema-2/)
      or a type expressed in XML Schema.

### Visually

It is common in RDF documents to see diagrams representing a statement such as that below. The predicate will be shown 
as a directed, and annotated, arrow from subject to object.

<a name="fig_1_2"></a>![An RDF Statement](img/primer-spo.png)
<div class="caption figure">1.2: An RDF Statement</div>

To visually distinguish between named resources, blank nodes, and literals, the following conventions are used.

1. Named resources will be shown as large ovals, their inner name is some short form of the resource's IRI.
1. Blank nodes will be shown as small ovals usually with no inner name (as they are effectively anonymous).
1. Literals will be shown as rectangles; their content may, or may not, be shown in double quotes.

<a name="fig_1_3"></a>![Statement Node Types](img/primer-nodes.png)
<div class="caption figure">1.3: Statement Node Types</div>

## An Annotated Example

As a simple, and easy to approach, example let us consider the `Cargo.toml` file for the core crate. This is a set of 
facts stated about a Cargo crate, and we will work up to an RDF representation of it.

```toml
[package]
name = "rdftk_core"
version = "0.1.10"
authors = ["Simon Johnston <johnstonskj@gmail.com>"]
edition = "2018"
description = "The core data model."
documentation = "https://docs.rs/rdftk_core/"
repository = "https://github.com/johnstonskj/rust-rdftk.git"
license = "MIT"
readme = "README.md"
publish = true

[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu"]

[dependencies]
error-chain = "0.12.2"
unique_id = "0.1.3"
rdftk_iri = { version = "0.1.2", path = "../rdftk_iri" }
rdftk_names = { version = "0.1.5", path = "../rdftk_names" }
```

First, let's start with a single statement, we will assert that a resource, identified as `rdftk:core`, is related to 
a resource, identified as `cargo:crate`, with the relationship `rdf:type`. The astute reader will notice that these
names appear to be namespaced, `type` in the namespace `rdf` for example, and this is true. Namespaces, or vocabularies, 
are described below. Note, the shading of the subject node in this case is simply to anchor the examples, it has no 
additional meaning.

<a name="fig_1_4"></a>![A Statement](img/primer-statement.png)
<div class="caption figure">1.4: A Statement</div>

Given this simple start we can begin adding other attributes from the *package* section of the file. These additional
properties are literal values, so they have been drawn as rectangles.

<a name="fig_1_5"></a>![Properties as Statements](img/primer-props.png)
<div class="caption figure">1.5: Properties as Statements</div>

When we get to the attribute `publish` however we want to express that this is a boolean value and not just a string. 
To do this we add a data type to the literal value, as shown below.

<a name="fig_1_6"></a>![Typing a Property](img/primer-datatype.png)
<div class="caption figure">1.6: Typing a Property</div>

This kind of type annotation can also be applied to a resource, if that resource has an `rdf:type` predicate, thus 
saving an arrow in the diagram. The different forms of annotation can be seen in the following figure. 

<a name="fig_1_7"></a>![Type Annotations](img/primer-annotations.png)
<div class="caption figure">1.7: Type Annotations</div>

Sometimes we want to have a resource with multiple predicates, but without the strong identity of an IRI. To do this 
we use blank nodes, nodes that have an internally generated, and therefore not externally addressable, identity. For
example, we could choose to break up the string value for the `authors` property into separate name and email, but don't 
want to give this representation an IRI.

<a name="fig_1_8"></a>![An Anonymous Subject](img/primer-bnode.png)
<div class="caption figure">1.8: An Anonymous Subject</div>

However, the actual attribute `authors` is a TOML list, and we have just added a single predicate relationship. In RDF
it is perfectly reasonable, and frankly quite common, to simply allow multiple predicate instances. 

<a name="fig_1_9"></a>![Multiple Predicate Values](img/primer-multiples.png)
<div class="caption figure">1.9: Multiple Predicate Values</div>

However, what is the semantic meaning of this? are these all authors, is there implied ordering? To this end RDF has
a number of collection structures, **List**, **Seq**uence, **Alt**ernatives, and **Bag** that model linked lists,
ordered lists, a list of alternatives, and an unordered collection. For a good description of RDF collections, see the 
blog post [Ordered Data in RDF](https://ontola.io/blog/ordered-data-in-rdf/).

For our example, we choose to model the `authors` attribute as an RDF sequence, the sequence itself is a blank node 
containing other blank nodes from the previous author example.

<a name="fig_1_10"></a>![Collections](img/primer-seq.png)
<div class="caption figure">1.10: Collections</div>

Finally, the real power of the RDF data model is that both subject and object can be resources, and in particular named
resources, and we can therefore link statements via these. For example; let us assume that the `crates.io` site also
describes users in RDF, now we can model our author predicate to *either* a blank node as shown above, or a named
user by using their IRI.

<a name="fig_1_11"></a>![Linking Statements](img/primer-linkage.png)
<div class="caption figure">1.11: Linking Statements</div>

## Graphs

As you can hopefully see, the RDF data model is inherently graph-structured.

## Vocabulary Descriptions

### Common Vocabularies

| Section | Attribute | Predicate | Restriction |
| ------- | ----- | -------- | -----------
| package | name  | `dcterms:identifier` | |
|         | version | `semver:version` | |
|         | authors | `dcterms:contributor` | multiples |
|         | edition | `cargo:edition` | |
|         | description | `dcterms:description` | |
|         | documentation | `crate:documentation` | |
|         | repository    | `crate:repository` | |
|         | license | `dcterms:license` | |
|         | readme  | `cargo:readme` | |
|         | publish | `cargo:publish` | |
| features | | `dcterms:has_part` | blank node |
|          | *name* | `dcterms:identifier` | |
|          | *parts* | `dcterms:requires` | `rdf:Bag` |
| dependencies | | `dcterms:requires` | blank node |
|              | *name* | `dcterms:identifier` | |
|              | version | `semver:matches` |
|              | path | `cargo:path` |

## Output Serialization

```turtle
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix dcterms: <http://purl.org/dc/terms/> .
@prefix vcard: <http://www.w3.org/2006/vcard/ns> .
@prefix crate: <http://crates.io/rdf/crates#> .
@prefix semver: <http://crates.io/rdf/semver#> .

<http://crates.io/crates/rdftk_core>
    dcterms:identifier  "rdftk_core" ;
    dcterms:version     "0.1.10" ;
    dcterms:contributor [
        vcard:fn        "Simon Johnston" ;
        vcard:hasEmail  "johnstonskj@gmail.com" 
    ] ;
    crate:edition       "2018"^^xsd:gYear ;
    dcterms:description "The core data model." ;
    crate:documentation <https://docs.rs/rdftk_core/> ;
    crate:repository    <https://github.com/johnstonskj/rust-rdftk.git> ;
    dcterms:license     "MIT" ;
    crate:readme        "README.md" ;
    crate:publish       "true"^^xsd:boolean ;
    dcterms:requires    [
        rdf:type        rdf:Bag ;
        rdf:_1          [
            dcterms:identifier  "error-chain" ;
            semver:matches      "0.12.2"
        ] ;
        rdf:_2          [
            dcterms:identifier  "unique_id" ;
            semver:matches      "0.1.3"
        ] ;
        rdf:_3          [
            dcterms:identifier  "rdftk_iri" ;
            semver:matches      "0.1.2" ;
            cargo:path          "../rdftk_iri"
        ] ;
        rdf:_4          [
            dcterms:identifier  "rdftk_names" ;
            semver:matches      "0.1.5"
            cargo:path          "../rdftk_names"
        ]
    ] 
.
```