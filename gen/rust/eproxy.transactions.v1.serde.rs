// @generated
impl serde::Serialize for AdditionalDataFull {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status.is_some() {
            len += 1;
        }
        if self.bitcoin_like_data.is_some() {
            len += 1;
        }
        if self.ethereum_data.is_some() {
            len += 1;
        }
        if self.tron_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.AdditionalDataFull", len)?;
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.bitcoin_like_data.as_ref() {
            struct_ser.serialize_field("bitcoinLikeData", v)?;
        }
        if let Some(v) = self.ethereum_data.as_ref() {
            struct_ser.serialize_field("ethereumData", v)?;
        }
        if let Some(v) = self.tron_data.as_ref() {
            struct_ser.serialize_field("tronData", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AdditionalDataFull {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "bitcoin_like_data",
            "bitcoinLikeData",
            "ethereum_data",
            "ethereumData",
            "tron_data",
            "tronData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            BitcoinLikeData,
            EthereumData,
            TronData,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "bitcoinLikeData" | "bitcoin_like_data" => Ok(GeneratedField::BitcoinLikeData),
                            "ethereumData" | "ethereum_data" => Ok(GeneratedField::EthereumData),
                            "tronData" | "tron_data" => Ok(GeneratedField::TronData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AdditionalDataFull;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.AdditionalDataFull")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AdditionalDataFull, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut bitcoin_like_data__ = None;
                let mut ethereum_data__ = None;
                let mut tron_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::BitcoinLikeData => {
                            if bitcoin_like_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitcoinLikeData"));
                            }
                            bitcoin_like_data__ = map_.next_value()?;
                        }
                        GeneratedField::EthereumData => {
                            if ethereum_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethereumData"));
                            }
                            ethereum_data__ = map_.next_value()?;
                        }
                        GeneratedField::TronData => {
                            if tron_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tronData"));
                            }
                            tron_data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AdditionalDataFull {
                    status: status__,
                    bitcoin_like_data: bitcoin_like_data__,
                    ethereum_data: ethereum_data__,
                    tron_data: tron_data__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.AdditionalDataFull", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for additional_data_full::BitcoinLikeData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hex.is_empty() {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.size != 0 {
            len += 1;
        }
        if self.v_size != 0 {
            len += 1;
        }
        if self.weight != 0 {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        if self.lock_time != 0 {
            len += 1;
        }
        if self.is_coinbase {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.AdditionalDataFull.BitcoinLikeData", len)?;
        if !self.hex.is_empty() {
            struct_ser.serialize_field("hex", &self.hex)?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if self.size != 0 {
            struct_ser.serialize_field("size", &self.size)?;
        }
        if self.v_size != 0 {
            struct_ser.serialize_field("vSize", &self.v_size)?;
        }
        if self.weight != 0 {
            struct_ser.serialize_field("weight", &self.weight)?;
        }
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if self.lock_time != 0 {
            struct_ser.serialize_field("lockTime", &self.lock_time)?;
        }
        if self.is_coinbase {
            struct_ser.serialize_field("isCoinbase", &self.is_coinbase)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for additional_data_full::BitcoinLikeData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hex",
            "hash",
            "size",
            "v_size",
            "vSize",
            "weight",
            "version",
            "lock_time",
            "lockTime",
            "is_coinbase",
            "isCoinbase",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hex,
            Hash,
            Size,
            VSize,
            Weight,
            Version,
            LockTime,
            IsCoinbase,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hex" => Ok(GeneratedField::Hex),
                            "hash" => Ok(GeneratedField::Hash),
                            "size" => Ok(GeneratedField::Size),
                            "vSize" | "v_size" => Ok(GeneratedField::VSize),
                            "weight" => Ok(GeneratedField::Weight),
                            "version" => Ok(GeneratedField::Version),
                            "lockTime" | "lock_time" => Ok(GeneratedField::LockTime),
                            "isCoinbase" | "is_coinbase" => Ok(GeneratedField::IsCoinbase),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = additional_data_full::BitcoinLikeData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.AdditionalDataFull.BitcoinLikeData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<additional_data_full::BitcoinLikeData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hex__ = None;
                let mut hash__ = None;
                let mut size__ = None;
                let mut v_size__ = None;
                let mut weight__ = None;
                let mut version__ = None;
                let mut lock_time__ = None;
                let mut is_coinbase__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hex => {
                            if hex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hex"));
                            }
                            hex__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VSize => {
                            if v_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vSize"));
                            }
                            v_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LockTime => {
                            if lock_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lockTime"));
                            }
                            lock_time__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IsCoinbase => {
                            if is_coinbase__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isCoinbase"));
                            }
                            is_coinbase__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(additional_data_full::BitcoinLikeData {
                    hex: hex__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                    size: size__.unwrap_or_default(),
                    v_size: v_size__.unwrap_or_default(),
                    weight: weight__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    lock_time: lock_time__.unwrap_or_default(),
                    is_coinbase: is_coinbase__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.AdditionalDataFull.BitcoinLikeData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for additional_data_full::EthereumData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chain_id != 0 {
            len += 1;
        }
        if self.txn_type != 0 {
            len += 1;
        }
        if self.gas != 0 {
            len += 1;
        }
        if self.gas_price != 0 {
            len += 1;
        }
        if self.gas_used != 0 {
            len += 1;
        }
        if self.max_fee_per_gas != 0 {
            len += 1;
        }
        if self.max_priority_fee_per_gas != 0 {
            len += 1;
        }
        if self.cumulative_gas_used != 0 {
            len += 1;
        }
        if self.effective_gas_price != 0 {
            len += 1;
        }
        if self.nonce != 0 {
            len += 1;
        }
        if !self.v.is_empty() {
            len += 1;
        }
        if !self.r.is_empty() {
            len += 1;
        }
        if !self.s.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.AdditionalDataFull.EthereumData", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if self.txn_type != 0 {
            struct_ser.serialize_field("txnType", &self.txn_type)?;
        }
        if self.gas != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gas", ToString::to_string(&self.gas).as_str())?;
        }
        if self.gas_price != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gasPrice", ToString::to_string(&self.gas_price).as_str())?;
        }
        if self.gas_used != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("gasUsed", ToString::to_string(&self.gas_used).as_str())?;
        }
        if self.max_fee_per_gas != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxFeePerGas", ToString::to_string(&self.max_fee_per_gas).as_str())?;
        }
        if self.max_priority_fee_per_gas != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxPriorityFeePerGas", ToString::to_string(&self.max_priority_fee_per_gas).as_str())?;
        }
        if self.cumulative_gas_used != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("cumulativeGasUsed", ToString::to_string(&self.cumulative_gas_used).as_str())?;
        }
        if self.effective_gas_price != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("effectiveGasPrice", ToString::to_string(&self.effective_gas_price).as_str())?;
        }
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        if !self.v.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("v", pbjson::private::base64::encode(&self.v).as_str())?;
        }
        if !self.r.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("r", pbjson::private::base64::encode(&self.r).as_str())?;
        }
        if !self.s.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("s", pbjson::private::base64::encode(&self.s).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for additional_data_full::EthereumData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "txn_type",
            "txnType",
            "gas",
            "gas_price",
            "gasPrice",
            "gas_used",
            "gasUsed",
            "max_fee_per_gas",
            "maxFeePerGas",
            "max_priority_fee_per_gas",
            "maxPriorityFeePerGas",
            "cumulative_gas_used",
            "cumulativeGasUsed",
            "effective_gas_price",
            "effectiveGasPrice",
            "nonce",
            "v",
            "r",
            "s",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            TxnType,
            Gas,
            GasPrice,
            GasUsed,
            MaxFeePerGas,
            MaxPriorityFeePerGas,
            CumulativeGasUsed,
            EffectiveGasPrice,
            Nonce,
            V,
            R,
            S,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "txnType" | "txn_type" => Ok(GeneratedField::TxnType),
                            "gas" => Ok(GeneratedField::Gas),
                            "gasPrice" | "gas_price" => Ok(GeneratedField::GasPrice),
                            "gasUsed" | "gas_used" => Ok(GeneratedField::GasUsed),
                            "maxFeePerGas" | "max_fee_per_gas" => Ok(GeneratedField::MaxFeePerGas),
                            "maxPriorityFeePerGas" | "max_priority_fee_per_gas" => Ok(GeneratedField::MaxPriorityFeePerGas),
                            "cumulativeGasUsed" | "cumulative_gas_used" => Ok(GeneratedField::CumulativeGasUsed),
                            "effectiveGasPrice" | "effective_gas_price" => Ok(GeneratedField::EffectiveGasPrice),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "v" => Ok(GeneratedField::V),
                            "r" => Ok(GeneratedField::R),
                            "s" => Ok(GeneratedField::S),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = additional_data_full::EthereumData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.AdditionalDataFull.EthereumData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<additional_data_full::EthereumData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut txn_type__ = None;
                let mut gas__ = None;
                let mut gas_price__ = None;
                let mut gas_used__ = None;
                let mut max_fee_per_gas__ = None;
                let mut max_priority_fee_per_gas__ = None;
                let mut cumulative_gas_used__ = None;
                let mut effective_gas_price__ = None;
                let mut nonce__ = None;
                let mut v__ = None;
                let mut r__ = None;
                let mut s__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TxnType => {
                            if txn_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txnType"));
                            }
                            txn_type__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Gas => {
                            if gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gas"));
                            }
                            gas__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GasPrice => {
                            if gas_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasPrice"));
                            }
                            gas_price__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GasUsed => {
                            if gas_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasUsed"));
                            }
                            gas_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxFeePerGas => {
                            if max_fee_per_gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxFeePerGas"));
                            }
                            max_fee_per_gas__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxPriorityFeePerGas => {
                            if max_priority_fee_per_gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPriorityFeePerGas"));
                            }
                            max_priority_fee_per_gas__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CumulativeGasUsed => {
                            if cumulative_gas_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cumulativeGasUsed"));
                            }
                            cumulative_gas_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EffectiveGasPrice => {
                            if effective_gas_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectiveGasPrice"));
                            }
                            effective_gas_price__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::V => {
                            if v__.is_some() {
                                return Err(serde::de::Error::duplicate_field("v"));
                            }
                            v__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::R => {
                            if r__.is_some() {
                                return Err(serde::de::Error::duplicate_field("r"));
                            }
                            r__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::S => {
                            if s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("s"));
                            }
                            s__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(additional_data_full::EthereumData {
                    chain_id: chain_id__.unwrap_or_default(),
                    txn_type: txn_type__.unwrap_or_default(),
                    gas: gas__.unwrap_or_default(),
                    gas_price: gas_price__.unwrap_or_default(),
                    gas_used: gas_used__.unwrap_or_default(),
                    max_fee_per_gas: max_fee_per_gas__.unwrap_or_default(),
                    max_priority_fee_per_gas: max_priority_fee_per_gas__.unwrap_or_default(),
                    cumulative_gas_used: cumulative_gas_used__.unwrap_or_default(),
                    effective_gas_price: effective_gas_price__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    v: v__.unwrap_or_default(),
                    r: r__.unwrap_or_default(),
                    s: s__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.AdditionalDataFull.EthereumData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for additional_data_full::TronData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.energy_usage.is_some() {
            len += 1;
        }
        if self.energy_fee.is_some() {
            len += 1;
        }
        if self.origin_energy_usage.is_some() {
            len += 1;
        }
        if self.energy_usage_total.is_some() {
            len += 1;
        }
        if self.net_usage.is_some() {
            len += 1;
        }
        if self.net_fee.is_some() {
            len += 1;
        }
        if self.energy_penalty_total.is_some() {
            len += 1;
        }
        if self.withdraw_amount.is_some() {
            len += 1;
        }
        if self.unfreeze_amount.is_some() {
            len += 1;
        }
        if self.withdraw_expire_amount.is_some() {
            len += 1;
        }
        if self.resource_contract_type.is_some() {
            len += 1;
        }
        if self.resource_type.is_some() {
            len += 1;
        }
        if self.resource_value.is_some() {
            len += 1;
        }
        if self.stacked_trx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.AdditionalDataFull.TronData", len)?;
        if let Some(v) = self.energy_usage.as_ref() {
            struct_ser.serialize_field("energyUsage", v)?;
        }
        if let Some(v) = self.energy_fee.as_ref() {
            struct_ser.serialize_field("energyFee", v)?;
        }
        if let Some(v) = self.origin_energy_usage.as_ref() {
            struct_ser.serialize_field("originEnergyUsage", v)?;
        }
        if let Some(v) = self.energy_usage_total.as_ref() {
            struct_ser.serialize_field("energyUsageTotal", v)?;
        }
        if let Some(v) = self.net_usage.as_ref() {
            struct_ser.serialize_field("netUsage", v)?;
        }
        if let Some(v) = self.net_fee.as_ref() {
            struct_ser.serialize_field("netFee", v)?;
        }
        if let Some(v) = self.energy_penalty_total.as_ref() {
            struct_ser.serialize_field("energyPenaltyTotal", v)?;
        }
        if let Some(v) = self.withdraw_amount.as_ref() {
            struct_ser.serialize_field("withdrawAmount", v)?;
        }
        if let Some(v) = self.unfreeze_amount.as_ref() {
            struct_ser.serialize_field("unfreezeAmount", v)?;
        }
        if let Some(v) = self.withdraw_expire_amount.as_ref() {
            struct_ser.serialize_field("withdrawExpireAmount", v)?;
        }
        if let Some(v) = self.resource_contract_type.as_ref() {
            struct_ser.serialize_field("resourceContractType", v)?;
        }
        if let Some(v) = self.resource_type.as_ref() {
            struct_ser.serialize_field("resourceType", v)?;
        }
        if let Some(v) = self.resource_value.as_ref() {
            struct_ser.serialize_field("resourceValue", v)?;
        }
        if let Some(v) = self.stacked_trx.as_ref() {
            struct_ser.serialize_field("stackedTrx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for additional_data_full::TronData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "energy_usage",
            "energyUsage",
            "energy_fee",
            "energyFee",
            "origin_energy_usage",
            "originEnergyUsage",
            "energy_usage_total",
            "energyUsageTotal",
            "net_usage",
            "netUsage",
            "net_fee",
            "netFee",
            "energy_penalty_total",
            "energyPenaltyTotal",
            "withdraw_amount",
            "withdrawAmount",
            "unfreeze_amount",
            "unfreezeAmount",
            "withdraw_expire_amount",
            "withdrawExpireAmount",
            "resource_contract_type",
            "resourceContractType",
            "resource_type",
            "resourceType",
            "resource_value",
            "resourceValue",
            "stacked_trx",
            "stackedTrx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EnergyUsage,
            EnergyFee,
            OriginEnergyUsage,
            EnergyUsageTotal,
            NetUsage,
            NetFee,
            EnergyPenaltyTotal,
            WithdrawAmount,
            UnfreezeAmount,
            WithdrawExpireAmount,
            ResourceContractType,
            ResourceType,
            ResourceValue,
            StackedTrx,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "energyUsage" | "energy_usage" => Ok(GeneratedField::EnergyUsage),
                            "energyFee" | "energy_fee" => Ok(GeneratedField::EnergyFee),
                            "originEnergyUsage" | "origin_energy_usage" => Ok(GeneratedField::OriginEnergyUsage),
                            "energyUsageTotal" | "energy_usage_total" => Ok(GeneratedField::EnergyUsageTotal),
                            "netUsage" | "net_usage" => Ok(GeneratedField::NetUsage),
                            "netFee" | "net_fee" => Ok(GeneratedField::NetFee),
                            "energyPenaltyTotal" | "energy_penalty_total" => Ok(GeneratedField::EnergyPenaltyTotal),
                            "withdrawAmount" | "withdraw_amount" => Ok(GeneratedField::WithdrawAmount),
                            "unfreezeAmount" | "unfreeze_amount" => Ok(GeneratedField::UnfreezeAmount),
                            "withdrawExpireAmount" | "withdraw_expire_amount" => Ok(GeneratedField::WithdrawExpireAmount),
                            "resourceContractType" | "resource_contract_type" => Ok(GeneratedField::ResourceContractType),
                            "resourceType" | "resource_type" => Ok(GeneratedField::ResourceType),
                            "resourceValue" | "resource_value" => Ok(GeneratedField::ResourceValue),
                            "stackedTrx" | "stacked_trx" => Ok(GeneratedField::StackedTrx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = additional_data_full::TronData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.AdditionalDataFull.TronData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<additional_data_full::TronData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut energy_usage__ = None;
                let mut energy_fee__ = None;
                let mut origin_energy_usage__ = None;
                let mut energy_usage_total__ = None;
                let mut net_usage__ = None;
                let mut net_fee__ = None;
                let mut energy_penalty_total__ = None;
                let mut withdraw_amount__ = None;
                let mut unfreeze_amount__ = None;
                let mut withdraw_expire_amount__ = None;
                let mut resource_contract_type__ = None;
                let mut resource_type__ = None;
                let mut resource_value__ = None;
                let mut stacked_trx__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EnergyUsage => {
                            if energy_usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("energyUsage"));
                            }
                            energy_usage__ = map_.next_value()?;
                        }
                        GeneratedField::EnergyFee => {
                            if energy_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("energyFee"));
                            }
                            energy_fee__ = map_.next_value()?;
                        }
                        GeneratedField::OriginEnergyUsage => {
                            if origin_energy_usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originEnergyUsage"));
                            }
                            origin_energy_usage__ = map_.next_value()?;
                        }
                        GeneratedField::EnergyUsageTotal => {
                            if energy_usage_total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("energyUsageTotal"));
                            }
                            energy_usage_total__ = map_.next_value()?;
                        }
                        GeneratedField::NetUsage => {
                            if net_usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("netUsage"));
                            }
                            net_usage__ = map_.next_value()?;
                        }
                        GeneratedField::NetFee => {
                            if net_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("netFee"));
                            }
                            net_fee__ = map_.next_value()?;
                        }
                        GeneratedField::EnergyPenaltyTotal => {
                            if energy_penalty_total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("energyPenaltyTotal"));
                            }
                            energy_penalty_total__ = map_.next_value()?;
                        }
                        GeneratedField::WithdrawAmount => {
                            if withdraw_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withdrawAmount"));
                            }
                            withdraw_amount__ = map_.next_value()?;
                        }
                        GeneratedField::UnfreezeAmount => {
                            if unfreeze_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unfreezeAmount"));
                            }
                            unfreeze_amount__ = map_.next_value()?;
                        }
                        GeneratedField::WithdrawExpireAmount => {
                            if withdraw_expire_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withdrawExpireAmount"));
                            }
                            withdraw_expire_amount__ = map_.next_value()?;
                        }
                        GeneratedField::ResourceContractType => {
                            if resource_contract_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceContractType"));
                            }
                            resource_contract_type__ = map_.next_value()?;
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceType"));
                            }
                            resource_type__ = map_.next_value()?;
                        }
                        GeneratedField::ResourceValue => {
                            if resource_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceValue"));
                            }
                            resource_value__ = map_.next_value()?;
                        }
                        GeneratedField::StackedTrx => {
                            if stacked_trx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stackedTrx"));
                            }
                            stacked_trx__ = map_.next_value()?;
                        }
                    }
                }
                Ok(additional_data_full::TronData {
                    energy_usage: energy_usage__,
                    energy_fee: energy_fee__,
                    origin_energy_usage: origin_energy_usage__,
                    energy_usage_total: energy_usage_total__,
                    net_usage: net_usage__,
                    net_fee: net_fee__,
                    energy_penalty_total: energy_penalty_total__,
                    withdraw_amount: withdraw_amount__,
                    unfreeze_amount: unfreeze_amount__,
                    withdraw_expire_amount: withdraw_expire_amount__,
                    resource_contract_type: resource_contract_type__,
                    resource_type: resource_type__,
                    resource_value: resource_value__,
                    stacked_trx: stacked_trx__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.AdditionalDataFull.TronData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddtitionalData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status.is_some() {
            len += 1;
        }
        if self.tron_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.AddtitionalData", len)?;
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.tron_data.as_ref() {
            struct_ser.serialize_field("tronData", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddtitionalData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "tron_data",
            "tronData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            TronData,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "tronData" | "tron_data" => Ok(GeneratedField::TronData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddtitionalData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.AddtitionalData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddtitionalData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut tron_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::TronData => {
                            if tron_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tronData"));
                            }
                            tron_data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddtitionalData {
                    status: status__,
                    tron_data: tron_data__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.AddtitionalData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for addtitional_data::TronData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.contract_type.is_some() {
            len += 1;
        }
        if self.asset_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.AddtitionalData.TronData", len)?;
        if let Some(v) = self.contract_type.as_ref() {
            struct_ser.serialize_field("contractType", v)?;
        }
        if let Some(v) = self.asset_type.as_ref() {
            struct_ser.serialize_field("assetType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for addtitional_data::TronData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_type",
            "contractType",
            "asset_type",
            "assetType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractType,
            AssetType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "contractType" | "contract_type" => Ok(GeneratedField::ContractType),
                            "assetType" | "asset_type" => Ok(GeneratedField::AssetType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = addtitional_data::TronData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.AddtitionalData.TronData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<addtitional_data::TronData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut contract_type__ = None;
                let mut asset_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContractType => {
                            if contract_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractType"));
                            }
                            contract_type__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AssetType => {
                            if asset_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetType"));
                            }
                            asset_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(addtitional_data::TronData {
                    contract_type: contract_type__,
                    asset_type: asset_type__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.AddtitionalData.TronData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Event {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        if self.original_data.is_some() {
            len += 1;
        }
        if self.encoded_data.is_some() {
            len += 1;
        }
        if self.blockchain_uniq_key.is_some() {
            len += 1;
        }
        if self.asset_identify.is_some() {
            len += 1;
        }
        if self.address_from.is_some() {
            len += 1;
        }
        if self.address_to.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.Event", len)?;
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        if let Some(v) = self.original_data.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("originalData", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.encoded_data.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("encodedData", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.blockchain_uniq_key.as_ref() {
            struct_ser.serialize_field("blockchainUniqKey", v)?;
        }
        if let Some(v) = self.asset_identify.as_ref() {
            struct_ser.serialize_field("assetIdentify", v)?;
        }
        if let Some(v) = self.address_from.as_ref() {
            struct_ser.serialize_field("addressFrom", v)?;
        }
        if let Some(v) = self.address_to.as_ref() {
            struct_ser.serialize_field("addressTo", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Event {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "original_data",
            "originalData",
            "encoded_data",
            "encodedData",
            "blockchain_uniq_key",
            "blockchainUniqKey",
            "asset_identify",
            "assetIdentify",
            "address_from",
            "addressFrom",
            "address_to",
            "addressTo",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            OriginalData,
            EncodedData,
            BlockchainUniqKey,
            AssetIdentify,
            AddressFrom,
            AddressTo,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "originalData" | "original_data" => Ok(GeneratedField::OriginalData),
                            "encodedData" | "encoded_data" => Ok(GeneratedField::EncodedData),
                            "blockchainUniqKey" | "blockchain_uniq_key" => Ok(GeneratedField::BlockchainUniqKey),
                            "assetIdentify" | "asset_identify" => Ok(GeneratedField::AssetIdentify),
                            "addressFrom" | "address_from" => Ok(GeneratedField::AddressFrom),
                            "addressTo" | "address_to" => Ok(GeneratedField::AddressTo),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Event;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.Event")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Event, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut original_data__ = None;
                let mut encoded_data__ = None;
                let mut blockchain_uniq_key__ = None;
                let mut asset_identify__ = None;
                let mut address_from__ = None;
                let mut address_to__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map_.next_value()?;
                        }
                        GeneratedField::OriginalData => {
                            if original_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalData"));
                            }
                            original_data__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::EncodedData => {
                            if encoded_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encodedData"));
                            }
                            encoded_data__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BlockchainUniqKey => {
                            if blockchain_uniq_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchainUniqKey"));
                            }
                            blockchain_uniq_key__ = map_.next_value()?;
                        }
                        GeneratedField::AssetIdentify => {
                            if asset_identify__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIdentify"));
                            }
                            asset_identify__ = map_.next_value()?;
                        }
                        GeneratedField::AddressFrom => {
                            if address_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressFrom"));
                            }
                            address_from__ = map_.next_value()?;
                        }
                        GeneratedField::AddressTo => {
                            if address_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressTo"));
                            }
                            address_to__ = map_.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Event {
                    r#type: r#type__,
                    original_data: original_data__,
                    encoded_data: encoded_data__,
                    blockchain_uniq_key: blockchain_uniq_key__,
                    asset_identify: asset_identify__,
                    address_from: address_from__,
                    address_to: address_to__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.Event", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.blockchain != 0 {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        if self.search.is_some() {
            len += 1;
        }
        if self.hash.is_some() {
            len += 1;
        }
        if self.block_height.is_some() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if self.asset_identify.is_some() {
            len += 1;
        }
        if self.bitcoin_params.is_some() {
            len += 1;
        }
        if self.tron_params.is_some() {
            len += 1;
        }
        if self.litecoin_params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.FindRequest", len)?;
        if self.blockchain != 0 {
            let v = super::super::common::v1::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if let Some(v) = self.search.as_ref() {
            struct_ser.serialize_field("search", v)?;
        }
        if let Some(v) = self.hash.as_ref() {
            struct_ser.serialize_field("hash", v)?;
        }
        if let Some(v) = self.block_height.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("blockHeight", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.asset_identify.as_ref() {
            struct_ser.serialize_field("assetIdentify", v)?;
        }
        if let Some(v) = self.bitcoin_params.as_ref() {
            struct_ser.serialize_field("bitcoinParams", v)?;
        }
        if let Some(v) = self.tron_params.as_ref() {
            struct_ser.serialize_field("tronParams", v)?;
        }
        if let Some(v) = self.litecoin_params.as_ref() {
            struct_ser.serialize_field("litecoinParams", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "blockchain",
            "pagination",
            "search",
            "hash",
            "block_height",
            "blockHeight",
            "address",
            "asset_identify",
            "assetIdentify",
            "bitcoin_params",
            "bitcoinParams",
            "tron_params",
            "tronParams",
            "litecoin_params",
            "litecoinParams",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Blockchain,
            Pagination,
            Search,
            Hash,
            BlockHeight,
            Address,
            AssetIdentify,
            BitcoinParams,
            TronParams,
            LitecoinParams,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "blockchain" => Ok(GeneratedField::Blockchain),
                            "pagination" => Ok(GeneratedField::Pagination),
                            "search" => Ok(GeneratedField::Search),
                            "hash" => Ok(GeneratedField::Hash),
                            "blockHeight" | "block_height" => Ok(GeneratedField::BlockHeight),
                            "address" => Ok(GeneratedField::Address),
                            "assetIdentify" | "asset_identify" => Ok(GeneratedField::AssetIdentify),
                            "bitcoinParams" | "bitcoin_params" => Ok(GeneratedField::BitcoinParams),
                            "tronParams" | "tron_params" => Ok(GeneratedField::TronParams),
                            "litecoinParams" | "litecoin_params" => Ok(GeneratedField::LitecoinParams),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FindRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.FindRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FindRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blockchain__ = None;
                let mut pagination__ = None;
                let mut search__ = None;
                let mut hash__ = None;
                let mut block_height__ = None;
                let mut address__ = None;
                let mut asset_identify__ = None;
                let mut bitcoin_params__ = None;
                let mut tron_params__ = None;
                let mut litecoin_params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v1::Blockchain>()? as i32);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::Search => {
                            if search__.is_some() {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            search__ = map_.next_value()?;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = map_.next_value()?;
                        }
                        GeneratedField::BlockHeight => {
                            if block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeight"));
                            }
                            block_height__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::AssetIdentify => {
                            if asset_identify__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIdentify"));
                            }
                            asset_identify__ = map_.next_value()?;
                        }
                        GeneratedField::BitcoinParams => {
                            if bitcoin_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitcoinParams"));
                            }
                            bitcoin_params__ = map_.next_value()?;
                        }
                        GeneratedField::TronParams => {
                            if tron_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tronParams"));
                            }
                            tron_params__ = map_.next_value()?;
                        }
                        GeneratedField::LitecoinParams => {
                            if litecoin_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("litecoinParams"));
                            }
                            litecoin_params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FindRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                    pagination: pagination__,
                    search: search__,
                    hash: hash__,
                    block_height: block_height__,
                    address: address__,
                    asset_identify: asset_identify__,
                    bitcoin_params: bitcoin_params__,
                    tron_params: tron_params__,
                    litecoin_params: litecoin_params__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.FindRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for find_request::BitcoinParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter_by_address.is_some() {
            len += 1;
        }
        if self.filter_by_mempool.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.FindRequest.BitcoinParams", len)?;
        if let Some(v) = self.filter_by_address.as_ref() {
            struct_ser.serialize_field("filterByAddress", v)?;
        }
        if let Some(v) = self.filter_by_mempool.as_ref() {
            struct_ser.serialize_field("filterByMempool", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for find_request::BitcoinParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter_by_address",
            "filterByAddress",
            "filter_by_mempool",
            "filterByMempool",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilterByAddress,
            FilterByMempool,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filterByAddress" | "filter_by_address" => Ok(GeneratedField::FilterByAddress),
                            "filterByMempool" | "filter_by_mempool" => Ok(GeneratedField::FilterByMempool),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = find_request::BitcoinParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.FindRequest.BitcoinParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<find_request::BitcoinParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter_by_address__ = None;
                let mut filter_by_mempool__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilterByAddress => {
                            if filter_by_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByAddress"));
                            }
                            filter_by_address__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByMempool => {
                            if filter_by_mempool__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByMempool"));
                            }
                            filter_by_mempool__ = map_.next_value()?;
                        }
                    }
                }
                Ok(find_request::BitcoinParams {
                    filter_by_address: filter_by_address__,
                    filter_by_mempool: filter_by_mempool__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.FindRequest.BitcoinParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for find_request::LitecoinParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter_by_address.is_some() {
            len += 1;
        }
        if self.filter_by_mempool.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.FindRequest.LitecoinParams", len)?;
        if let Some(v) = self.filter_by_address.as_ref() {
            struct_ser.serialize_field("filterByAddress", v)?;
        }
        if let Some(v) = self.filter_by_mempool.as_ref() {
            struct_ser.serialize_field("filterByMempool", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for find_request::LitecoinParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter_by_address",
            "filterByAddress",
            "filter_by_mempool",
            "filterByMempool",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilterByAddress,
            FilterByMempool,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filterByAddress" | "filter_by_address" => Ok(GeneratedField::FilterByAddress),
                            "filterByMempool" | "filter_by_mempool" => Ok(GeneratedField::FilterByMempool),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = find_request::LitecoinParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.FindRequest.LitecoinParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<find_request::LitecoinParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter_by_address__ = None;
                let mut filter_by_mempool__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilterByAddress => {
                            if filter_by_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByAddress"));
                            }
                            filter_by_address__ = map_.next_value()?;
                        }
                        GeneratedField::FilterByMempool => {
                            if filter_by_mempool__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterByMempool"));
                            }
                            filter_by_mempool__ = map_.next_value()?;
                        }
                    }
                }
                Ok(find_request::LitecoinParams {
                    filter_by_address: filter_by_address__,
                    filter_by_mempool: filter_by_mempool__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.FindRequest.LitecoinParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for find_request::TronParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.contract_type.is_some() {
            len += 1;
        }
        if self.is_internal.is_some() {
            len += 1;
        }
        if self.is_include_detail_info {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.FindRequest.TronParams", len)?;
        if let Some(v) = self.contract_type.as_ref() {
            struct_ser.serialize_field("contractType", v)?;
        }
        if let Some(v) = self.is_internal.as_ref() {
            struct_ser.serialize_field("isInternal", v)?;
        }
        if self.is_include_detail_info {
            struct_ser.serialize_field("isIncludeDetailInfo", &self.is_include_detail_info)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for find_request::TronParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_type",
            "contractType",
            "is_internal",
            "isInternal",
            "is_include_detail_info",
            "isIncludeDetailInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractType,
            IsInternal,
            IsIncludeDetailInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "contractType" | "contract_type" => Ok(GeneratedField::ContractType),
                            "isInternal" | "is_internal" => Ok(GeneratedField::IsInternal),
                            "isIncludeDetailInfo" | "is_include_detail_info" => Ok(GeneratedField::IsIncludeDetailInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = find_request::TronParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.FindRequest.TronParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<find_request::TronParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut contract_type__ = None;
                let mut is_internal__ = None;
                let mut is_include_detail_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContractType => {
                            if contract_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractType"));
                            }
                            contract_type__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::IsInternal => {
                            if is_internal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isInternal"));
                            }
                            is_internal__ = map_.next_value()?;
                        }
                        GeneratedField::IsIncludeDetailInfo => {
                            if is_include_detail_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isIncludeDetailInfo"));
                            }
                            is_include_detail_info__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(find_request::TronParams {
                    contract_type: contract_type__,
                    is_internal: is_internal__,
                    is_include_detail_info: is_include_detail_info__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.FindRequest.TronParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        if self.next_page_exists {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.FindResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if self.next_page_exists {
            struct_ser.serialize_field("nextPageExists", &self.next_page_exists)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
            "next_page_exists",
            "nextPageExists",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
            NextPageExists,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "items" => Ok(GeneratedField::Items),
                            "nextPageExists" | "next_page_exists" => Ok(GeneratedField::NextPageExists),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FindResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.FindResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FindResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                let mut next_page_exists__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageExists => {
                            if next_page_exists__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageExists"));
                            }
                            next_page_exists__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FindResponse {
                    items: items__.unwrap_or_default(),
                    next_page_exists: next_page_exists__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.FindResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetInfoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.blockchain != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.GetInfoRequest", len)?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if self.blockchain != 0 {
            let v = super::super::common::v1::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetInfoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "blockchain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            Blockchain,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            "blockchain" => Ok(GeneratedField::Blockchain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetInfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.GetInfoRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetInfoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut blockchain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v1::Blockchain>()? as i32);
                        }
                    }
                }
                Ok(GetInfoRequest {
                    hash: hash__.unwrap_or_default(),
                    blockchain: blockchain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.GetInfoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetInfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transaction_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.GetInfoResponse", len)?;
        if let Some(v) = self.transaction_info.as_ref() {
            struct_ser.serialize_field("transactionInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetInfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transaction_info",
            "transactionInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransactionInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "transactionInfo" | "transaction_info" => Ok(GeneratedField::TransactionInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetInfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.GetInfoResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetInfoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transaction_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransactionInfo => {
                            if transaction_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionInfo"));
                            }
                            transaction_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetInfoResponse {
                    transaction_info: transaction_info__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.GetInfoResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Transaction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.block_height != 0 {
            len += 1;
        }
        if self.confirmations != 0 {
            len += 1;
        }
        if self.address_from.is_some() {
            len += 1;
        }
        if self.address_to.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if self.fee.is_some() {
            len += 1;
        }
        if self.asset_identify.is_some() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if self.additional_data.is_some() {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.in_mempool {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.Transaction", len)?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if self.block_height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("blockHeight", ToString::to_string(&self.block_height).as_str())?;
        }
        if self.confirmations != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("confirmations", ToString::to_string(&self.confirmations).as_str())?;
        }
        if let Some(v) = self.address_from.as_ref() {
            struct_ser.serialize_field("addressFrom", v)?;
        }
        if let Some(v) = self.address_to.as_ref() {
            struct_ser.serialize_field("addressTo", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        if let Some(v) = self.asset_identify.as_ref() {
            struct_ser.serialize_field("assetIdentify", v)?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", &self.index)?;
        }
        if let Some(v) = self.additional_data.as_ref() {
            struct_ser.serialize_field("additionalData", v)?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if self.in_mempool {
            struct_ser.serialize_field("inMempool", &self.in_mempool)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Transaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "block_height",
            "blockHeight",
            "confirmations",
            "address_from",
            "addressFrom",
            "address_to",
            "addressTo",
            "value",
            "fee",
            "asset_identify",
            "assetIdentify",
            "index",
            "additional_data",
            "additionalData",
            "events",
            "created_at",
            "createdAt",
            "in_mempool",
            "inMempool",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            BlockHeight,
            Confirmations,
            AddressFrom,
            AddressTo,
            Value,
            Fee,
            AssetIdentify,
            Index,
            AdditionalData,
            Events,
            CreatedAt,
            InMempool,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            "blockHeight" | "block_height" => Ok(GeneratedField::BlockHeight),
                            "confirmations" => Ok(GeneratedField::Confirmations),
                            "addressFrom" | "address_from" => Ok(GeneratedField::AddressFrom),
                            "addressTo" | "address_to" => Ok(GeneratedField::AddressTo),
                            "value" => Ok(GeneratedField::Value),
                            "fee" => Ok(GeneratedField::Fee),
                            "assetIdentify" | "asset_identify" => Ok(GeneratedField::AssetIdentify),
                            "index" => Ok(GeneratedField::Index),
                            "additionalData" | "additional_data" => Ok(GeneratedField::AdditionalData),
                            "events" => Ok(GeneratedField::Events),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "inMempool" | "in_mempool" => Ok(GeneratedField::InMempool),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Transaction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.Transaction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Transaction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut block_height__ = None;
                let mut confirmations__ = None;
                let mut address_from__ = None;
                let mut address_to__ = None;
                let mut value__ = None;
                let mut fee__ = None;
                let mut asset_identify__ = None;
                let mut index__ = None;
                let mut additional_data__ = None;
                let mut events__ = None;
                let mut created_at__ = None;
                let mut in_mempool__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlockHeight => {
                            if block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeight"));
                            }
                            block_height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Confirmations => {
                            if confirmations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("confirmations"));
                            }
                            confirmations__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AddressFrom => {
                            if address_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressFrom"));
                            }
                            address_from__ = map_.next_value()?;
                        }
                        GeneratedField::AddressTo => {
                            if address_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressTo"));
                            }
                            address_to__ = map_.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map_.next_value()?;
                        }
                        GeneratedField::AssetIdentify => {
                            if asset_identify__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIdentify"));
                            }
                            asset_identify__ = map_.next_value()?;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AdditionalData => {
                            if additional_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalData"));
                            }
                            additional_data__ = map_.next_value()?;
                        }
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::InMempool => {
                            if in_mempool__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inMempool"));
                            }
                            in_mempool__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Transaction {
                    hash: hash__.unwrap_or_default(),
                    block_height: block_height__.unwrap_or_default(),
                    confirmations: confirmations__.unwrap_or_default(),
                    address_from: address_from__,
                    address_to: address_to__,
                    value: value__,
                    fee: fee__,
                    asset_identify: asset_identify__,
                    index: index__.unwrap_or_default(),
                    additional_data: additional_data__,
                    events: events__.unwrap_or_default(),
                    created_at: created_at__,
                    in_mempool: in_mempool__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.Transaction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransactionInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.block_height != 0 {
            len += 1;
        }
        if self.confirmations != 0 {
            len += 1;
        }
        if self.address_from.is_some() {
            len += 1;
        }
        if self.address_to.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if self.fee.is_some() {
            len += 1;
        }
        if self.asset_identify.is_some() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if self.additional_data_full.is_some() {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.in_mempool {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v1.TransactionInfo", len)?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if self.block_height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("blockHeight", ToString::to_string(&self.block_height).as_str())?;
        }
        if self.confirmations != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("confirmations", ToString::to_string(&self.confirmations).as_str())?;
        }
        if let Some(v) = self.address_from.as_ref() {
            struct_ser.serialize_field("addressFrom", v)?;
        }
        if let Some(v) = self.address_to.as_ref() {
            struct_ser.serialize_field("addressTo", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        if let Some(v) = self.asset_identify.as_ref() {
            struct_ser.serialize_field("assetIdentify", v)?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", &self.index)?;
        }
        if let Some(v) = self.additional_data_full.as_ref() {
            struct_ser.serialize_field("additionalDataFull", v)?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if self.in_mempool {
            struct_ser.serialize_field("inMempool", &self.in_mempool)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransactionInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "block_height",
            "blockHeight",
            "confirmations",
            "address_from",
            "addressFrom",
            "address_to",
            "addressTo",
            "value",
            "fee",
            "asset_identify",
            "assetIdentify",
            "index",
            "additional_data_full",
            "additionalDataFull",
            "events",
            "created_at",
            "createdAt",
            "in_mempool",
            "inMempool",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            BlockHeight,
            Confirmations,
            AddressFrom,
            AddressTo,
            Value,
            Fee,
            AssetIdentify,
            Index,
            AdditionalDataFull,
            Events,
            CreatedAt,
            InMempool,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            "blockHeight" | "block_height" => Ok(GeneratedField::BlockHeight),
                            "confirmations" => Ok(GeneratedField::Confirmations),
                            "addressFrom" | "address_from" => Ok(GeneratedField::AddressFrom),
                            "addressTo" | "address_to" => Ok(GeneratedField::AddressTo),
                            "value" => Ok(GeneratedField::Value),
                            "fee" => Ok(GeneratedField::Fee),
                            "assetIdentify" | "asset_identify" => Ok(GeneratedField::AssetIdentify),
                            "index" => Ok(GeneratedField::Index),
                            "additionalDataFull" | "additional_data_full" => Ok(GeneratedField::AdditionalDataFull),
                            "events" => Ok(GeneratedField::Events),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "inMempool" | "in_mempool" => Ok(GeneratedField::InMempool),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransactionInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v1.TransactionInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransactionInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut block_height__ = None;
                let mut confirmations__ = None;
                let mut address_from__ = None;
                let mut address_to__ = None;
                let mut value__ = None;
                let mut fee__ = None;
                let mut asset_identify__ = None;
                let mut index__ = None;
                let mut additional_data_full__ = None;
                let mut events__ = None;
                let mut created_at__ = None;
                let mut in_mempool__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlockHeight => {
                            if block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeight"));
                            }
                            block_height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Confirmations => {
                            if confirmations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("confirmations"));
                            }
                            confirmations__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AddressFrom => {
                            if address_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressFrom"));
                            }
                            address_from__ = map_.next_value()?;
                        }
                        GeneratedField::AddressTo => {
                            if address_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressTo"));
                            }
                            address_to__ = map_.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map_.next_value()?;
                        }
                        GeneratedField::AssetIdentify => {
                            if asset_identify__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIdentify"));
                            }
                            asset_identify__ = map_.next_value()?;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AdditionalDataFull => {
                            if additional_data_full__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalDataFull"));
                            }
                            additional_data_full__ = map_.next_value()?;
                        }
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::InMempool => {
                            if in_mempool__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inMempool"));
                            }
                            in_mempool__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TransactionInfo {
                    hash: hash__.unwrap_or_default(),
                    block_height: block_height__.unwrap_or_default(),
                    confirmations: confirmations__.unwrap_or_default(),
                    address_from: address_from__,
                    address_to: address_to__,
                    value: value__,
                    fee: fee__,
                    asset_identify: asset_identify__,
                    index: index__.unwrap_or_default(),
                    additional_data_full: additional_data_full__,
                    events: events__.unwrap_or_default(),
                    created_at: created_at__,
                    in_mempool: in_mempool__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v1.TransactionInfo", FIELDS, GeneratedVisitor)
    }
}
