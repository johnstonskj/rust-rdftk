/*!
Functions that create IRIs for the [RDF Schema](https://www.w3.org/TR/rdf-schema/) namespace.
*/

namespace! {
    "rdfs",
    "http://www.w3.org/2000/01/rdf-schema#",
    {
        // §2. Classes
        resource, "Resource",
        class, "Class",
        literal, "Literal",
        data_type, "Datatype",
        lang_string, "langString",
        html_literal, "HTML",
        xml_literal, "XMLLiteral",
        property, "Property",

        // §3. Properties
        range, "range",
        domain, "domain",
        subclass_of, "subClassOf",
        subproperty_of, "subPropertyOf",
        label, "label",
        comment, "comment",

        // §5.1 Container Classes and Properties
        container, "Container",
        container_membership_property, "ContainerMembershipProperty",
        member, "member",

        // ±5.4 Utility Properties
        see_also, "seeAlso",
        is_defined_by, "isDefinedBy",
        value, "value"
    }
}
