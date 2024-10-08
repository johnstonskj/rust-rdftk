use rdftk_core::model::data_set::DataSet;

#[test]
fn test_create_data_set() {
    let data_set = DataSet::default();

    assert!(data_set.is_empty());
    assert_eq!(data_set.len(), 0);
}
