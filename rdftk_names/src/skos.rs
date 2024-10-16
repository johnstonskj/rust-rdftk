/*!
Namespaces for SKOS, SKOS eXtension for Labels (SKOS-XL), and the ISO-25964 thesaurus vocabulary.
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

/// SKOS eXtension for Labels (SKOS-XL)
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

/// ISO-25964 thesaurus vocabulary
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
            super_ordinate, "superOrdinate",

            broader_generic, "broaderGeneric",
            broader_instantial, "broaderInstantial",
            broader_partitive, "broaderPartitive",
            narrower_generic, "narrowerGeneric",
            narrower_instantial, "narrowerInstantial",
            narrower_partitive, "narrowerPartitive"
        }
    }
}

/// See https://www.w3.org/2003/06/sw-vocab-status/note.html
pub mod term_status {
    namespace! {
        "status",
        "http://www.w3.org/2003/06/sw-vocab-status/ns#",
        {
            term_status, "term_status"
        }
    }

    ///
    /// This term is marked as old-fashioned; although used, it is not considered typical of current
    /// best practice and alternative expressions may be preferable.
    ///
    pub const STATUS_ARCHAIC: &str = "archaic";

    ///
    /// This term is marked as deprecated; although it may still be in use, it is not considered typical
    /// of current best practice and alternative expressions may be preferable.
    ///
    pub const STATUS_DEPRECATED: &str = "deprecated";

    ///
    /// The meaning, deployment practices, documentation (or important associated software/services)
    /// associated with this term are new and therefore liable to change arbitrarily at some point
    /// in the future. They may not, but stability is not guaranteed.
    ///
    pub const STATUS_PROPOSED: &str = "proposed";

    ///
    /// The term is relatively stable, and its documentation and meaning are not expected to change
    /// substantially.
    ///
    pub const STATUS_STABLE: &str = "stable";

    ///
    /// The meaning, deployment practices, documentation and general understanding of this term are
    /// approaching some stability, but changes are still possible due to implementation experience
    /// or other unanticipated factors.
    ///
    pub const STATUS_TESTING: &str = "testing";

    ///
    /// The meaning, deployment practices, documentation (or important associated software/services)
    /// associated with this term are liable to change arbitrarily at some point in the future. They
    /// may not, but stability is not guaranteed. Use with caution.
    ///
    pub const STATUS_UNSTABLE: &str = "unstable";
}
