use rdftk_core::model::statement::StatementList;
use rdftk_core::simple::resource::Resource;
use rdftk_iri::IriRef;
use rdftk_iri::Iri;
use std::str::FromStr;

fn contact(name: &str) -> IriRef {
    Iri::from_str(&format!(
        "http://www.w3.org/2000/10/swap/pim/contact#{}",
        name
    ))
    .unwrap()
    .into()
}

#[test]
fn wikipedia_example_01() {
    let resource = Resource::named(
        Iri::from_str("http://www.w3.org/People/EM/contact#me")
            .unwrap()
            .into(),
    )
    .literal_str(contact("fullName"), "Eric Miller")
    .resource_named(
        contact("mailbox"),
        Iri::from_str("mailto:e.miller123(at)example")
            .unwrap()
            .into(),
    )
    .literal_str(contact("personalTitle"), "Dr.")
    .instance_of(contact("Person"))
    .to_owned();
    let sts: StatementList = resource.into();
    assert_eq!(sts.len(), 4);
    for st in sts {
        println!("{}", st);
    }
}
