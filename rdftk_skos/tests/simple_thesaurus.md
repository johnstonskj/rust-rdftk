# Scheme: UNESCO Thesaurus

*The UNESCO thesaurus.*

<http://vocabularies.unesco.org/thesaurus>

## Labels

> **skos:prefLabel**

| Label text | Language |
| ---- | ----  |
| UNESCO Thesaurus | **en** |
| Thésaurus de l'UNESCO | fr |
| Тезаурус ЮНЕСКО | ru |
| Tesauro de la UNESCO | es |

## Other Properties

| Predicate | Literal Form | Data Type | Language |
| ---- | ---- | ---- | ----  |
| skos:definition | The UNESCO thesaurus. |  | **en** |

Jump to: [Concepts Hierarchy](#concepts-hierarchy) | [Concepts](#concepts) | [Collections](#collections) | [Appendix - RDF](#appendix---rdf)

## Concepts Hierarchy

* **[Computers](#concept-computers)**
  * [Analog Computers](#concept-analog-computers)

## Concepts

### Concept: Analog Computers

<http://vocabularies.unesco.org/thesaurus/concept2258>

#### Labels

> **skos:prefLabel**

| Label text | Language |
| ---- | ----  |
| Calculateur analogique | fr |
| Аналоговые компьютеры | ru |
| Analog Computers | **en** |
| Ordenador analógico | es |
| حواسب تناظرية | ar |

> **skos:hiddenLabel**

| Label text | Language |
| ---- | ----  |
| Ordenador analogico | es |

#### Other Properties

| Predicate | Literal Form | Data Type | Language |
| ---- | ---- | ---- | ----  |
| dcterms:modified | 2019-12-15T14:00:02Z | xsd:dateTime |   |

#### Related Concepts

| Relationship | Concept IRI |
| ---- | ----  |
| skos:broader | [Computers](#concept-computers) |

#### In Collections

* [Information technology (hardware)](#collection-information-technology-hardware)

### Concept: Computers

<http://vocabularies.unesco.org/thesaurus/concept534>

#### Labels

> **skos:prefLabel**

| Label text | Language |
| ---- | ----  |
| حواسيب | ar |
| Computers | **en** |
| Ordinateur | fr |
| Компьютеры | ru |

#### Related Concepts

| Relationship | Concept IRI |
| ---- | ----  |
| skos:narrower | [Analog Computers](#concept-analog-computers) |
| owl:equivalentClass | [dbpedia:InformationAppliance](http://dbpedia.org/ontology/InformationAppliance) |

## Collections

### Collection: Information and communication

<http://vocabularies.unesco.org/thesaurus/domain5>

#### Labels

> **skos:prefLabel**

| Label text | Language |
| ---- | ----  |
| معلومات واتصالات | ar |
| Информация и коммуникация | ru |
| Information et communication | fr |
| Information and communication | **en** |
| Información y comunicación | es |

#### Members

* Collection [Information technology (hardware)](#collection-information-technology-hardware)

### Collection: Information technology (hardware)

<http://vocabularies.unesco.org/thesaurus/mt5.45>

#### Labels

> **skos:prefLabel**

| Label text | Language |
| ---- | ----  |
| تكنولوجيا المعلومات (الأجهزة) | ar |
| Информационная технология (технические средства) | ru |
| Technologie de l'information (équipements) | fr |
| Information technology (hardware) | **en** |
| Tecnología de la información (equipos) | es |

#### Members

* Concept [Analog Computers](#concept-analog-computers)

#### In Collections

* [Information and communication](#collection-information-and-communication)

## Appendix - RDF
