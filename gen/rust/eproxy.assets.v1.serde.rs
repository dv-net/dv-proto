// @generated
impl serde::Serialize for AssetInfoRequest {
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
        if !self.asset_identifier.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.assets.v1.AssetInfoRequest", len)?;
        if self.blockchain != 0 {
            let v = super::super::common::v1::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        if !self.asset_identifier.is_empty() {
            struct_ser.serialize_field("assetIdentifier", &self.asset_identifier)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetInfoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "blockchain",
            "asset_identifier",
            "assetIdentifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Blockchain,
            AssetIdentifier,
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
                            "assetIdentifier" | "asset_identifier" => Ok(GeneratedField::AssetIdentifier),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetInfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.assets.v1.AssetInfoRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssetInfoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blockchain__ = None;
                let mut asset_identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v1::Blockchain>()? as i32);
                        }
                        GeneratedField::AssetIdentifier => {
                            if asset_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIdentifier"));
                            }
                            asset_identifier__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AssetInfoRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                    asset_identifier: asset_identifier__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.assets.v1.AssetInfoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssetInfoResponse {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.assets.v1.AssetInfoResponse", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        if let Some(v) = self.decimals.as_ref() {
            struct_ser.serialize_field("decimals", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetInfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "symbol",
            "decimals",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Symbol,
            Decimals,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetInfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.assets.v1.AssetInfoResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssetInfoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut symbol__ = None;
                let mut decimals__ = None;
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
                    }
                }
                Ok(AssetInfoResponse {
                    name: name__.unwrap_or_default(),
                    symbol: symbol__.unwrap_or_default(),
                    decimals: decimals__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.assets.v1.AssetInfoResponse", FIELDS, GeneratedVisitor)
    }
}
