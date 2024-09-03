// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! impl_resource {
    ($actual:ident) => {
        impl $crate::Resource for $actual {
            fn uri(&self) -> &::rdftk_iri::IriRef {
                &self.uri
            }
        }
    };
}

macro_rules! impl_labeled {
    ($actual:ident) => {
        impl_resource!($actual);

        impl $crate::Labeled for $actual {
            fn add_label_property(&mut self, property: $crate::LabelProperty) {
                self.label_properties.push(property);
            }

            fn remove_label_property(&mut self, property: &$crate::LabelProperty) {
                self.label_properties.retain(|p| p != property)
            }

            fn label_properties(&self) -> Vec<&$crate::LabelProperty> {
                self.label_properties.iter().collect()
            }
        }
    };
}

macro_rules! impl_individual {
    ($actual:ident) => {
        impl_labeled!($actual);

        impl $crate::Individual for $actual {
            fn add_instance_of(&mut self, parent: ::rdftk_iri::IriRef) {
                self.instance_of.push(parent)
            }

            fn remove_instance_of(&mut self, parent: &::rdftk_iri::IriRef) {
                self.instance_of.retain(|p| p != parent)
            }

            fn instance_of(&self) -> Vec<&::rdftk_iri::IriRef> {
                self.instance_of.iter().collect()
            }
        }
    };
}

macro_rules! impl_subclassed {
    ($actual:ident) => {
        impl_individual!($actual);

        impl $crate::Subclassed for $actual {
            fn add_parent(&mut self, parent: ::rdftk_iri::IriRef) {
                self.parents.push(parent)
            }

            fn remove_parent(&mut self, parent: &::rdftk_iri::IriRef) {
                self.parents.retain(|p| p != parent)
            }

            fn parents(&self) -> Vec<&::rdftk_iri::IriRef> {
                self.parents.iter().collect()
            }
        }
    };
}
