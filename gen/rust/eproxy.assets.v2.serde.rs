// @generated
impl serde::Serialize for InfoRequest {
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
        if !self.identifier.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.assets.v2.InfoRequest", len)?;
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        if !self.identifier.is_empty() {
            struct_ser.serialize_field("identifier", &self.identifier)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InfoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "blockchain",
            "identifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Blockchain,
            Identifier,
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
                            "identifier" => Ok(GeneratedField::Identifier),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.assets.v2.InfoRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InfoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blockchain__ = None;
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v2::Blockchain>()? as i32);
                        }
                        GeneratedField::Identifier => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            identifier__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InfoRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                    identifier: identifier__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.assets.v2.InfoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.symbol.is_empty() {
            len += 1;
        }
        if self.decimals.is_some() {
            len += 1;
        }
        if self.kind.is_some() {
            len += 1;
        }
        if self.storage_layout.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.assets.v2.InfoResponse", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        if let Some(v) = self.decimals.as_ref() {
            struct_ser.serialize_field("decimals", v)?;
        }
        if let Some(v) = self.kind.as_ref() {
            struct_ser.serialize_field("kind", v)?;
        }
        if let Some(v) = self.storage_layout.as_ref() {
            struct_ser.serialize_field("storageLayout", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "symbol",
            "decimals",
            "kind",
            "storage_layout",
            "storageLayout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Symbol,
            Decimals,
            Kind,
            StorageLayout,
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
                            "name" => Ok(GeneratedField::Name),
                            "symbol" => Ok(GeneratedField::Symbol),
                            "decimals" => Ok(GeneratedField::Decimals),
                            "kind" => Ok(GeneratedField::Kind),
                            "storageLayout" | "storage_layout" => Ok(GeneratedField::StorageLayout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.assets.v2.InfoResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InfoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut symbol__ = None;
                let mut decimals__ = None;
                let mut kind__ = None;
                let mut storage_layout__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Decimals => {
                            if decimals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decimals"));
                            }
                            decimals__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = map_.next_value()?;
                        }
                        GeneratedField::StorageLayout => {
                            if storage_layout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageLayout"));
                            }
                            storage_layout__ = map_.next_value()?;
                        }
                    }
                }
                Ok(InfoResponse {
                    name: name__.unwrap_or_default(),
                    symbol: symbol__.unwrap_or_default(),
                    decimals: decimals__,
                    kind: kind__,
                    storage_layout: storage_layout__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.assets.v2.InfoResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StorageLayout {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.balance_slot.is_some() {
            len += 1;
        }
        if self.allowance_slot.is_some() {
            len += 1;
        }
        if self.proxy_implementation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.assets.v2.StorageLayout", len)?;
        if let Some(v) = self.balance_slot.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("balanceSlot", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.allowance_slot.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("allowanceSlot", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.proxy_implementation.as_ref() {
            struct_ser.serialize_field("proxyImplementation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StorageLayout {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "balance_slot",
            "balanceSlot",
            "allowance_slot",
            "allowanceSlot",
            "proxy_implementation",
            "proxyImplementation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BalanceSlot,
            AllowanceSlot,
            ProxyImplementation,
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
                            "balanceSlot" | "balance_slot" => Ok(GeneratedField::BalanceSlot),
                            "allowanceSlot" | "allowance_slot" => Ok(GeneratedField::AllowanceSlot),
                            "proxyImplementation" | "proxy_implementation" => Ok(GeneratedField::ProxyImplementation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageLayout;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.assets.v2.StorageLayout")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StorageLayout, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut balance_slot__ = None;
                let mut allowance_slot__ = None;
                let mut proxy_implementation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BalanceSlot => {
                            if balance_slot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balanceSlot"));
                            }
                            balance_slot__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AllowanceSlot => {
                            if allowance_slot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowanceSlot"));
                            }
                            allowance_slot__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ProxyImplementation => {
                            if proxy_implementation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proxyImplementation"));
                            }
                            proxy_implementation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StorageLayout {
                    balance_slot: balance_slot__,
                    allowance_slot: allowance_slot__,
                    proxy_implementation: proxy_implementation__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.assets.v2.StorageLayout", FIELDS, GeneratedVisitor)
    }
}
