use welds::WeldsModel;

#[derive(Debug, WeldsModel, Clone, PartialEq, Eq)]
#[welds(schema = "welds", table = "binary_table")]
pub struct BinaryA {
    #[welds(primary_key)]
    pub id: uuid::Uuid,
    pub b1: Vec<u8>,
    pub b2: Vec<u8>,
    pub b3: Vec<u8>,
    pub ob1: Option<Vec<u8>>,
    pub ob2: Option<Vec<u8>>,
    pub ob3: Option<Vec<u8>>,
}

#[derive(Debug, WeldsModel, Clone, PartialEq, Eq)]
#[welds(schema = "welds", table = "binary_table")]
pub struct BinaryB {
    #[welds(primary_key)]
    pub id: Vec<u8>,
    pub b1: Vec<u8>,
    pub b2: Vec<u8>,
    pub b3: Vec<u8>,
}

#[derive(Debug, WeldsModel, Clone, PartialEq, Eq)]
#[welds(schema = "welds", table = "binary_key")]
pub struct BinaryKey {
    #[welds(primary_key)]
    pub id: Vec<u8>,
}
