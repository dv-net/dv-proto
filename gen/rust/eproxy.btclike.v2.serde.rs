// @generated
impl serde::Serialize for MedianFeePerByteBlockRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.number != 0 {
            len += 1;
        }
        if self.blockchain != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.btclike.v2.MedianFeePerByteBlockRequest", len)?;
        if self.number != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("number", ToString::to_string(&self.number).as_str())?;
        }
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MedianFeePerByteBlockRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "number",
            "blockchain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Number,
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
                            "number" => Ok(GeneratedField::Number),
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
            type Value = MedianFeePerByteBlockRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.btclike.v2.MedianFeePerByteBlockRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MedianFeePerByteBlockRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut number__ = None;
                let mut blockchain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Number => {
                            if number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v2::Blockchain>()? as i32);
                        }
                    }
                }
                Ok(MedianFeePerByteBlockRequest {
                    number: number__.unwrap_or_default(),
                    blockchain: blockchain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.btclike.v2.MedianFeePerByteBlockRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MedianFeePerByteBlockResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.btclike.v2.MedianFeePerByteBlockResponse", len)?;
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MedianFeePerByteBlockResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "amount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
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
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MedianFeePerByteBlockResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.btclike.v2.MedianFeePerByteBlockResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MedianFeePerByteBlockResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MedianFeePerByteBlockResponse {
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.btclike.v2.MedianFeePerByteBlockResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovedBlockInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.created_at.is_some() {
            len += 1;
        }
        if self.block_number != 0 {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        if !self.transactions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.btclike.v2.RemovedBlockInfo", len)?;
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if self.block_number != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("blockNumber", ToString::to_string(&self.block_number).as_str())?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if !self.transactions.is_empty() {
            struct_ser.serialize_field("transactions", &self.transactions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovedBlockInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "created_at",
            "createdAt",
            "block_number",
            "blockNumber",
            "hash",
            "transactions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreatedAt,
            BlockNumber,
            Hash,
            Transactions,
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
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "blockNumber" | "block_number" => Ok(GeneratedField::BlockNumber),
                            "hash" => Ok(GeneratedField::Hash),
                            "transactions" => Ok(GeneratedField::Transactions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemovedBlockInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.btclike.v2.RemovedBlockInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovedBlockInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut created_at__ = None;
                let mut block_number__ = None;
                let mut hash__ = None;
                let mut transactions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::BlockNumber => {
                            if block_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockNumber"));
                            }
                            block_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Transactions => {
                            if transactions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactions"));
                            }
                            transactions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemovedBlockInfo {
                    created_at: created_at__,
                    block_number: block_number__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                    transactions: transactions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.btclike.v2.RemovedBlockInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovedBlockListRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.time_since_incident.is_some() {
            len += 1;
        }
        if self.blockchain != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.btclike.v2.RemovedBlockListRequest", len)?;
        if let Some(v) = self.time_since_incident.as_ref() {
            struct_ser.serialize_field("timeSinceIncident", v)?;
        }
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovedBlockListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "time_since_incident",
            "timeSinceIncident",
            "blockchain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TimeSinceIncident,
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
                            "timeSinceIncident" | "time_since_incident" => Ok(GeneratedField::TimeSinceIncident),
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
            type Value = RemovedBlockListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.btclike.v2.RemovedBlockListRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovedBlockListRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut time_since_incident__ = None;
                let mut blockchain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TimeSinceIncident => {
                            if time_since_incident__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeSinceIncident"));
                            }
                            time_since_incident__ = map_.next_value()?;
                        }
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v2::Blockchain>()? as i32);
                        }
                    }
                }
                Ok(RemovedBlockListRequest {
                    time_since_incident: time_since_incident__,
                    blockchain: blockchain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.btclike.v2.RemovedBlockListRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovedBlockListResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.blocks.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.btclike.v2.RemovedBlockListResponse", len)?;
        if !self.blocks.is_empty() {
            struct_ser.serialize_field("blocks", &self.blocks)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovedBlockListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "blocks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Blocks,
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
                            "blocks" => Ok(GeneratedField::Blocks),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemovedBlockListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.btclike.v2.RemovedBlockListResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovedBlockListResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blocks__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Blocks => {
                            if blocks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocks"));
                            }
                            blocks__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemovedBlockListResponse {
                    blocks: blocks__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.btclike.v2.RemovedBlockListResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovedBlockRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.number != 0 {
            len += 1;
        }
        if self.blockchain != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.btclike.v2.RemovedBlockRequest", len)?;
        if self.number != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("number", ToString::to_string(&self.number).as_str())?;
        }
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovedBlockRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "number",
            "blockchain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Number,
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
                            "number" => Ok(GeneratedField::Number),
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
            type Value = RemovedBlockRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.btclike.v2.RemovedBlockRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovedBlockRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut number__ = None;
                let mut blockchain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Number => {
                            if number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v2::Blockchain>()? as i32);
                        }
                    }
                }
                Ok(RemovedBlockRequest {
                    number: number__.unwrap_or_default(),
                    blockchain: blockchain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.btclike.v2.RemovedBlockRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovedBlockResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.removed_block_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.btclike.v2.RemovedBlockResponse", len)?;
        if let Some(v) = self.removed_block_info.as_ref() {
            struct_ser.serialize_field("removedBlockInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovedBlockResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "removed_block_info",
            "removedBlockInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemovedBlockInfo,
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
                            "removedBlockInfo" | "removed_block_info" => Ok(GeneratedField::RemovedBlockInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemovedBlockResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.btclike.v2.RemovedBlockResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovedBlockResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut removed_block_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemovedBlockInfo => {
                            if removed_block_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removedBlockInfo"));
                            }
                            removed_block_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RemovedBlockResponse {
                    removed_block_info: removed_block_info__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.btclike.v2.RemovedBlockResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UtxoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.blockchain != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.btclike.v2.UTXORequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UtxoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "blockchain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
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
            type Value = UtxoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.btclike.v2.UTXORequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UtxoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut blockchain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v2::Blockchain>()? as i32);
                        }
                    }
                }
                Ok(UtxoRequest {
                    address: address__.unwrap_or_default(),
                    blockchain: blockchain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.btclike.v2.UTXORequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UtxoResponse {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.btclike.v2.UTXOResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UtxoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UtxoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.btclike.v2.UTXOResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UtxoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UtxoResponse {
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.btclike.v2.UTXOResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for utxo_response::Item {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tx_hash.is_empty() {
            len += 1;
        }
        if self.sequence != 0 {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.pk_script.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.btclike.v2.UTXOResponse.Item", len)?;
        if !self.tx_hash.is_empty() {
            struct_ser.serialize_field("txHash", &self.tx_hash)?;
        }
        if self.sequence != 0 {
            struct_ser.serialize_field("sequence", &self.sequence)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.pk_script.is_empty() {
            struct_ser.serialize_field("pkScript", &self.pk_script)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for utxo_response::Item {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tx_hash",
            "txHash",
            "sequence",
            "amount",
            "pk_script",
            "pkScript",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TxHash,
            Sequence,
            Amount,
            PkScript,
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
                            "txHash" | "tx_hash" => Ok(GeneratedField::TxHash),
                            "sequence" => Ok(GeneratedField::Sequence),
                            "amount" => Ok(GeneratedField::Amount),
                            "pkScript" | "pk_script" => Ok(GeneratedField::PkScript),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = utxo_response::Item;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.btclike.v2.UTXOResponse.Item")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<utxo_response::Item, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tx_hash__ = None;
                let mut sequence__ = None;
                let mut amount__ = None;
                let mut pk_script__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TxHash => {
                            if tx_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txHash"));
                            }
                            tx_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PkScript => {
                            if pk_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pkScript"));
                            }
                            pk_script__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(utxo_response::Item {
                    tx_hash: tx_hash__.unwrap_or_default(),
                    sequence: sequence__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    pk_script: pk_script__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.btclike.v2.UTXOResponse.Item", FIELDS, GeneratedVisitor)
    }
}
