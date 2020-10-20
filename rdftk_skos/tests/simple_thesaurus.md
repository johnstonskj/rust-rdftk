# <a name="s__unesco_thesaurus">Scheme: UNESCO Thesaurus

[<http://vocabularies.unesco.org/thesaurus>](http://vocabularies.unesco.org/thesaurus)

## Labels

> **skos:prefLabel**
>
> | Label Text | Language |
> |------------|----------|
> | UNESCO Thesaurus | **en** |
> | Thésaurus de l'UNESCO | fr |
> | Тезаурус ЮНЕСКО | ru |
> | Tesauro de la UNESCO | es |

## Other Properties

> | Predicate | Literal Form | Data Type | Language |
> |-----------|--------------|-----------|----------|
> | dcterms:description | The UNESCO thesaurus. |  |  |

----------

## Concept Hierarchy

* **[Computers](#c__computers)**
  * [Analog Computers](#c__analog_computers)

----------

## Concepts

### <a name="c__analog_computers">Concept: Analog Computers

[<http://vocabularies.unesco.org/thesaurus/concept2258>](http://vocabularies.unesco.org/thesaurus/concept2258)

#### Labels

> **skos:prefLabel**
>
> | Label Text | Language |
> |------------|----------|
> | Calculateur analogique | fr |
> | Аналоговые компьютеры | ru |
> | Analog Computers | **en** |
> | Ordenador analógico | es |
> | حواسب تناظرية | ar |

> **skos:hiddenLabel**
>
> | Label Text | Language |
> |------------|----------|
> | Ordenador analogico | es |

#### Other Properties

> | Predicate | Literal Form | Data Type | Language |
> |-----------|--------------|-----------|----------|
> | dcterms:modified | 2019-12-15T14:00:02Z | xsd:dateTime |  |

#### Related Concepts

> | Relationship | Concept IRI |
> |--------------|-------------|
> | skos:broader | [Computers](#c__computers) |

#### In Collections

* [Information technology (hardware)](cc__information_technology_(hardware))

### <a name="c__computers">Concept: Computers

[<http://vocabularies.unesco.org/thesaurus/concept534>](http://vocabularies.unesco.org/thesaurus/concept534)

#### Labels

> **skos:prefLabel**
>
> | Label Text | Language |
> |------------|----------|
> | حواسيب | ar |
> | Computers | **en** |
> | Ordinateur | fr |
> | Компьютеры | ru |

#### Related Concepts

> | Relationship | Concept IRI |
> |--------------|-------------|
> | skos:narrower | [Analog Computers](#c__analog_computers) |

----------

## Collections

### <a name="cc__information_and_communication">Collection: Information and communication

[<http://vocabularies.unesco.org/thesaurus/domain5>](http://vocabularies.unesco.org/thesaurus/domain5)

#### Labels

> **skos:prefLabel**
>
> | Label Text | Language |
> |------------|----------|
> | معلومات واتصالات | ar |
> | Информация и коммуникация | ru |
> | Information et communication | fr |
> | Information and communication | **en** |
> | Información y comunicación | es |

#### Members

* Collection [Information technology (hardware)](cc__information_technology_(hardware))

### <a name="cc__information_technology_(hardware)">Collection: Information technology (hardware)

[<http://vocabularies.unesco.org/thesaurus/mt5.45>](http://vocabularies.unesco.org/thesaurus/mt5.45)

#### Labels

> **skos:prefLabel**
>
> | Label Text | Language |
> |------------|----------|
> | تكنولوجيا المعلومات (الأجهزة) | ar |
> | Информационная технология (технические средства) | ru |
> | Technologie de l'information (équipements) | fr |
> | Information technology (hardware) | **en** |
> | Tecnología de la información (equipos) | es |

#### Members

* Concept [Analog Computers](c__analog_computers)

#### In Collections

* [Information and communication](cc__information_and_communication)

----------

## Appendix - RDF

```turtle
