pub use iutxo::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod iutxo {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"inputId\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidSignature\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"UTXONotFound\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct IUTXO.Output[]\",\"name\":\"outputs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"id_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUTXOById\",\"outputs\":[{\"internalType\":\"struct IUTXO.UTXO\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isSpent\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"ids_\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUTXOByIds\",\"outputs\":[{\"internalType\":\"struct IUTXO.UTXO[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isSpent\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUTXOsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"offset_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"limit_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"listUTXOs\",\"outputs\":[{\"internalType\":\"struct IUTXO.UTXO[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isSpent\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"address_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"offset_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"limit_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"listUTXOsByAddress\",\"outputs\":[{\"internalType\":\"struct IUTXO.UTXO[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isSpent\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct IUTXO.Input[]\",\"name\":\"inputs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct IUTXO.Output[]\",\"name\":\"outputs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct IUTXO.Input\",\"name\":\"input_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"address\",\"name\":\"to_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IUTXO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IUTXO<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IUTXO<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IUTXO<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IUTXO<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IUTXO<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IUTXO)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUTXO<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IUTXO_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `deposit` (0x9af4f87f) function
        pub fn deposit(
            &self,
            token: ::ethers::core::types::Address,
            outputs: ::std::vec::Vec<Output>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 244, 248, 127], (token, outputs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUTXOById` (0x2a6d04fb) function
        pub fn get_utxo_by_id(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Utxo> {
            self.0
                .method_hash([42, 109, 4, 251], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUTXOByIds` (0x6fd6f15f) function
        pub fn get_utxo_by_ids(
            &self,
            ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Utxo>> {
            self.0
                .method_hash([111, 214, 241, 95], ids)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUTXOsLength` (0xad7c290f) function
        pub fn get_utx_os_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 124, 41, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `listUTXOs` (0x7b4dd81b) function
        pub fn list_utx_os(
            &self,
            offset: ::ethers::core::types::U256,
            limit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Utxo>> {
            self.0
                .method_hash([123, 77, 216, 27], (offset, limit))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `listUTXOsByAddress` (0x632bea43) function
        pub fn list_utx_os_by_address(
            &self,
            address: ::ethers::core::types::Address,
            offset: ::ethers::core::types::U256,
            limit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Utxo>> {
            self.0
                .method_hash([99, 43, 234, 67], (address, offset, limit))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xf183ab79) function
        pub fn transfer(
            &self,
            inputs: ::std::vec::Vec<Input>,
            outputs: ::std::vec::Vec<Output>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 131, 171, 121], (inputs, outputs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x492c3107) function
        pub fn withdraw(
            &self,
            input: Input,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 44, 49, 7], (input, to))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IUTXO<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidSignature` with signature `InvalidSignature(address,uint256)` and selector `0xc41f0f69`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidSignature", abi = "InvalidSignature(address,uint256)")]
    pub struct InvalidSignature {
        pub owner: ::ethers::core::types::Address,
        pub input_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `UTXONotFound` with signature `UTXONotFound()` and selector `0x3a3e45b0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UTXONotFound", abi = "UTXONotFound()")]
    pub struct UTXONotFound;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUTXOErrors {
        InvalidSignature(InvalidSignature),
        UTXONotFound(UTXONotFound),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IUTXOErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <InvalidSignature as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSignature(decoded));
            }
            if let Ok(decoded)
                = <UTXONotFound as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UTXONotFound(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IUTXOErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UTXONotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IUTXOErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UTXONotFound as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IUTXOErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::UTXONotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IUTXOErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidSignature> for IUTXOErrors {
        fn from(value: InvalidSignature) -> Self {
            Self::InvalidSignature(value)
        }
    }
    impl ::core::convert::From<UTXONotFound> for IUTXOErrors {
        fn from(value: UTXONotFound) -> Self {
            Self::UTXONotFound(value)
        }
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(address,(uint256,address)[])` and selector `0x9af4f87f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deposit", abi = "deposit(address,(uint256,address)[])")]
    pub struct DepositCall {
        pub token: ::ethers::core::types::Address,
        pub outputs: ::std::vec::Vec<Output>,
    }
    ///Container type for all input parameters for the `getUTXOById` function with signature `getUTXOById(uint256)` and selector `0x2a6d04fb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getUTXOById", abi = "getUTXOById(uint256)")]
    pub struct GetUTXOByIdCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getUTXOByIds` function with signature `getUTXOByIds(uint256[])` and selector `0x6fd6f15f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getUTXOByIds", abi = "getUTXOByIds(uint256[])")]
    pub struct GetUTXOByIdsCall {
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `getUTXOsLength` function with signature `getUTXOsLength()` and selector `0xad7c290f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getUTXOsLength", abi = "getUTXOsLength()")]
    pub struct GetUTXOsLengthCall;
    ///Container type for all input parameters for the `listUTXOs` function with signature `listUTXOs(uint256,uint256)` and selector `0x7b4dd81b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "listUTXOs", abi = "listUTXOs(uint256,uint256)")]
    pub struct ListUTXOsCall {
        pub offset: ::ethers::core::types::U256,
        pub limit: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `listUTXOsByAddress` function with signature `listUTXOsByAddress(address,uint256,uint256)` and selector `0x632bea43`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "listUTXOsByAddress",
        abi = "listUTXOsByAddress(address,uint256,uint256)"
    )]
    pub struct ListUTXOsByAddressCall {
        pub address: ::ethers::core::types::Address,
        pub offset: ::ethers::core::types::U256,
        pub limit: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transfer` function with signature `transfer((uint256,bytes)[],(uint256,address)[])` and selector `0xf183ab79`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "transfer",
        abi = "transfer((uint256,bytes)[],(uint256,address)[])"
    )]
    pub struct TransferCall {
        pub inputs: ::std::vec::Vec<Input>,
        pub outputs: ::std::vec::Vec<Output>,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw((uint256,bytes),address)` and selector `0x492c3107`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdraw", abi = "withdraw((uint256,bytes),address)")]
    pub struct WithdrawCall {
        pub input: Input,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUTXOCalls {
        Deposit(DepositCall),
        GetUTXOById(GetUTXOByIdCall),
        GetUTXOByIds(GetUTXOByIdsCall),
        GetUTXOsLength(GetUTXOsLengthCall),
        ListUTXOs(ListUTXOsCall),
        ListUTXOsByAddress(ListUTXOsByAddressCall),
        Transfer(TransferCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for IUTXOCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <GetUTXOByIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetUTXOById(decoded));
            }
            if let Ok(decoded)
                = <GetUTXOByIdsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetUTXOByIds(decoded));
            }
            if let Ok(decoded)
                = <GetUTXOsLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetUTXOsLength(decoded));
            }
            if let Ok(decoded)
                = <ListUTXOsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ListUTXOs(decoded));
            }
            if let Ok(decoded)
                = <ListUTXOsByAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ListUTXOsByAddress(decoded));
            }
            if let Ok(decoded)
                = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IUTXOCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetUTXOById(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUTXOByIds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUTXOsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ListUTXOs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ListUTXOsByAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IUTXOCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUTXOById(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUTXOByIds(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUTXOsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::ListUTXOs(element) => ::core::fmt::Display::fmt(element, f),
                Self::ListUTXOsByAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositCall> for IUTXOCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<GetUTXOByIdCall> for IUTXOCalls {
        fn from(value: GetUTXOByIdCall) -> Self {
            Self::GetUTXOById(value)
        }
    }
    impl ::core::convert::From<GetUTXOByIdsCall> for IUTXOCalls {
        fn from(value: GetUTXOByIdsCall) -> Self {
            Self::GetUTXOByIds(value)
        }
    }
    impl ::core::convert::From<GetUTXOsLengthCall> for IUTXOCalls {
        fn from(value: GetUTXOsLengthCall) -> Self {
            Self::GetUTXOsLength(value)
        }
    }
    impl ::core::convert::From<ListUTXOsCall> for IUTXOCalls {
        fn from(value: ListUTXOsCall) -> Self {
            Self::ListUTXOs(value)
        }
    }
    impl ::core::convert::From<ListUTXOsByAddressCall> for IUTXOCalls {
        fn from(value: ListUTXOsByAddressCall) -> Self {
            Self::ListUTXOsByAddress(value)
        }
    }
    impl ::core::convert::From<TransferCall> for IUTXOCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for IUTXOCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `getUTXOById` function with signature `getUTXOById(uint256)` and selector `0x2a6d04fb`
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
    pub struct GetUTXOByIdReturn(pub Utxo);
    ///Container type for all return fields from the `getUTXOByIds` function with signature `getUTXOByIds(uint256[])` and selector `0x6fd6f15f`
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
    pub struct GetUTXOByIdsReturn(pub ::std::vec::Vec<Utxo>);
    ///Container type for all return fields from the `getUTXOsLength` function with signature `getUTXOsLength()` and selector `0xad7c290f`
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
    pub struct GetUTXOsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `listUTXOs` function with signature `listUTXOs(uint256,uint256)` and selector `0x7b4dd81b`
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
    pub struct ListUTXOsReturn(pub ::std::vec::Vec<Utxo>);
    ///Container type for all return fields from the `listUTXOsByAddress` function with signature `listUTXOsByAddress(address,uint256,uint256)` and selector `0x632bea43`
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
    pub struct ListUTXOsByAddressReturn(pub ::std::vec::Vec<Utxo>);
}
