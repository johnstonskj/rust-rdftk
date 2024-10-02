use rdftk_core::model::data_set::{DataSet, DataSetFactory};
use rdftk_core::simple::data_set::SimpleDataSetFactory;

#[test]
fn test_create_data_set() {
    let factory = SimpleDataSetFactory::default();
    let data_set = factory.data_set();

    assert_eq!(data_set.len(), 0);
}
