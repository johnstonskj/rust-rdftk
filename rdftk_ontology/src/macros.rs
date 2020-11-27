// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! impl_resource {
    ($actual:ident) => {
        impl Resource for $actual {
            fn uri(&self) -> &IRIRef {
                &self.uri
            }
        }
    };
}

macro_rules! impl_labeled {
    ($actual:ident) => {
        impl_resource!($actual);

        impl Labeled for $actual {
            fn add_label_property(&mut self, property: LabelProperty) {
                self.label_properties.push(property);
            }

            fn remove_label_property(&mut self, property: &LabelProperty) {
                self.label_properties.retain(|p| p != property)
            }

            fn label_properties(&self) -> Vec<&LabelProperty> {
                self.label_properties.iter().collect()
            }
        }
    };
}

macro_rules! impl_individual {
    ($actual:ident) => {
        impl_labeled!($actual);

        impl Individual for $actual {
            fn add_instance_of(&mut self, parent: IRIRef) {
                self.instance_of.push(parent)
            }

            fn remove_instance_of(&mut self, parent: &IRIRef) {
                self.instance_of.retain(|p| p != parent)
            }

            fn instance_of(&self) -> Vec<&IRIRef> {
                self.instance_of.iter().collect()
            }
        }
    };
}

macro_rules! impl_subclassed {
    ($actual:ident) => {
        impl_individual!($actual);

        impl Subclassed for $actual {
            fn add_parent(&mut self, parent: IRIRef) {
                self.parents.push(parent)
            }

            fn remove_parent(&mut self, parent: &IRIRef) {
                self.parents.retain(|p| p != parent)
            }

            fn parents(&self) -> Vec<&IRIRef> {
                self.parents.iter().collect()
            }
        }
    };
}
