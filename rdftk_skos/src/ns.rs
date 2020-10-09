/*!
Namespaces for SKOS, SKOS eXtension for Labels (SKOS-XL), and the ISO-25964 thesaurus vocabulary.

Details TBD

# Example

TBD

*/

namespace! {
    "skos",
    "http://www.w3.org/2004/02/skos/core#",
    {
        collection, "Collection",
        concept, "Concept",
        concept_scheme, "ConceptScheme",
        ordered_collection, "OrderedCollection",

        alt_label, "altLabel",
        broad_match, "broadMatch",
        broader, "broader",
        broader_transitive, "broaderTransitive",
        change_note, "changeNote",
        close_match, "closeMatch",
        definition, "definition",
        editorial_note, "editorialNote",
        exact_match, "exactMatch",
        example, "example",
        has_top_concept, "hasTopConcept",
        hidden_label, "hiddenLabel",
        history_note, "historyNote",
        in_scheme, "inScheme",
        mapping_relation, "mappingRelation",
        member, "member",
        member_list, "memberList",
        narrow_match, "narrowMatch",
        narrower, "narrower",
        narrower_transitive, "narrowerTransitive",
        notation, "notation",
        note, "note",
        pref_label, "prefLabel",
        related, "related",
        related_match, "relatedMatch",
        scope_note, "scopeNote",
        semantic_relation, "semanticRelation",
        top_concept_of, "topConceptOf"
    }
}

pub mod xl {
    namespace! {
        "skosxl",
        "http://www.w3.org/2008/05/skos-xl#",
        {
            label, "Label",
            literal_form, "literalForm",
            pref_label, "prefLabel",
            alt_label, "altLabel",
            label_relation, "labelRelation"
        }
    }
}

pub mod iso {
    namespace! {
        "isothes",
        "http://purl.org/iso25964/skos-thes#",
        {
            compound_equivalence, "CompoundEquivalence",
            concept_group, "ConceptGroup",
            preferred_term, "PreferredTerm",
            thesaurus_array, "ThesaurusArray",

            micro_thesaurus_of, "microThesaurusOf",
            plus_uf_term, "plusUFTerm",
            plus_use_term, "plusUseTerm",
            sub_group, "subGroup",
            subordinate_array, "subordinateArray",
            super_group, "superGroup",
            super_ordinate, "superOrdinate"
        }
    }
}
