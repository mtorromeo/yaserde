use yaserde::*;

#[derive(YaDeserialize, Debug, PartialEq)]
#[yaserde(
  rename = "base",
  namespace = "http://base/",
  namespace = "ns1: http://ns1/",
  namespace = "ns2: http://ns2/"
)]
pub struct Base {
  #[yaserde(rename = "data", prefix = "ns1")]
  pub data: Ns2DataItemType,
}

#[derive(YaDeserialize, Debug, PartialEq, Default)]
#[yaserde(prefix = "ns1", namespace = "ns1: http://ns1/")]
pub enum Ns2DataItemType {
  #[yaserde(rename = "dataItem", prefix = "ns2")]
  DataItem(String),
  #[default]
  #[allow(non_camel_case_types)]
  __undefined__,
}

#[test]
fn mixed_namespaces() {
  use yaserde::de::from_str;

  let _ = env_logger::builder().is_test(true).try_init();

  let reference = Base {
    data: Ns2DataItemType::DataItem("value1".to_string()),
  };

  let content = r#"
    <base
        xmlns="http://base/"
        xmlns:ns1="http://ns1/"
        xmlns:ns2="http://ns2/">
      <ns1:data>
        <ns2:dataItem>value1</ns2:dataItem>
      </ns1:data>
    </base>
  "#;

  let loaded: Base = from_str(content).unwrap();
  println!("{:?}", loaded);

  assert_eq!(loaded, reference);
}
