///`Input(uint256,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Input {
    pub id: ::ethers::core::types::U256,
    pub signature: ::ethers::core::types::Bytes,
}
///`Output(uint256,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Output {
    pub amount: ::ethers::core::types::U256,
    pub owner: ::ethers::core::types::Address,
}
///`Utxo(uint256,address,uint256,address,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Utxo {
    pub id: ::ethers::core::types::U256,
    pub token: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
    pub owner: ::ethers::core::types::Address,
    pub is_spent: bool,
}
