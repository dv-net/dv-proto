// @generated
impl serde::Serialize for Block {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height != 0 {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if !self.prev_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.Block", len)?;
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if !self.prev_hash.is_empty() {
            struct_ser.serialize_field("prevHash", &self.prev_hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Block {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
            "hash",
            "created_at",
            "createdAt",
            "prev_hash",
            "prevHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
            Hash,
            CreatedAt,
            PrevHash,
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
                            "height" => Ok(GeneratedField::Height),
                            "hash" => Ok(GeneratedField::Hash),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "prevHash" | "prev_hash" => Ok(GeneratedField::PrevHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Block;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.Block")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Block, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                let mut hash__ = None;
                let mut created_at__ = None;
                let mut prev_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::PrevHash => {
                            if prev_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prevHash"));
                            }
                            prev_hash__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Block {
                    height: height__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                    created_at: created_at__,
                    prev_hash: prev_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.Block", FIELDS, GeneratedVisitor)
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
        if self.common.is_some() {
            len += 1;
        }
        if self.from_block_height.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.FindRequest", len)?;
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        if let Some(v) = self.common.as_ref() {
            struct_ser.serialize_field("common", v)?;
        }
        if let Some(v) = self.from_block_height.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("fromBlockHeight", ToString::to_string(&v).as_str())?;
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
            "common",
            "from_block_height",
            "fromBlockHeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Blockchain,
            Common,
            FromBlockHeight,
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
                            "common" => Ok(GeneratedField::Common),
                            "fromBlockHeight" | "from_block_height" => Ok(GeneratedField::FromBlockHeight),
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
                formatter.write_str("struct eproxy.blocks.v2.FindRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FindRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blockchain__ = None;
                let mut common__ = None;
                let mut from_block_height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v2::Blockchain>()? as i32);
                        }
                        GeneratedField::Common => {
                            if common__.is_some() {
                                return Err(serde::de::Error::duplicate_field("common"));
                            }
                            common__ = map_.next_value()?;
                        }
                        GeneratedField::FromBlockHeight => {
                            if from_block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fromBlockHeight"));
                            }
                            from_block_height__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(FindRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                    common: common__,
                    from_block_height: from_block_height__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.FindRequest", FIELDS, GeneratedVisitor)
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
        if self.count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.FindResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if self.count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
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
            "count",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
            Count,
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
                            "count" => Ok(GeneratedField::Count),
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
                formatter.write_str("struct eproxy.blocks.v2.FindResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FindResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                let mut count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(FindResponse {
                    items: items__.unwrap_or_default(),
                    count: count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.FindResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRequest {
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
        if self.height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.GetRequest", len)?;
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "blockchain",
            "height",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Blockchain,
            Height,
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
                            "height" => Ok(GeneratedField::Height),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.GetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blockchain__ = None;
                let mut height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v2::Blockchain>()? as i32);
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.GetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.item.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.GetResponse", len)?;
        if let Some(v) = self.item.as_ref() {
            struct_ser.serialize_field("item", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "item",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Item,
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
                            "item" => Ok(GeneratedField::Item),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.GetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut item__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Item => {
                            if item__.is_some() {
                                return Err(serde::de::Error::duplicate_field("item"));
                            }
                            item__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetResponse {
                    item: item__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.GetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LastBlockHeightRequest {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.LastBlockHeightRequest", len)?;
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LastBlockHeightRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "blockchain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = LastBlockHeightRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.LastBlockHeightRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LastBlockHeightRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blockchain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v2::Blockchain>()? as i32);
                        }
                    }
                }
                Ok(LastBlockHeightRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.LastBlockHeightRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LastBlockHeightResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.LastBlockHeightResponse", len)?;
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LastBlockHeightResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
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
                            "height" => Ok(GeneratedField::Height),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LastBlockHeightResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.LastBlockHeightResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LastBlockHeightResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(LastBlockHeightResponse {
                    height: height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.LastBlockHeightResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MinBlockHeightRequest {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.MinBlockHeightRequest", len)?;
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MinBlockHeightRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "blockchain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = MinBlockHeightRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.MinBlockHeightRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MinBlockHeightRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blockchain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v2::Blockchain>()? as i32);
                        }
                    }
                }
                Ok(MinBlockHeightRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.MinBlockHeightRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MinBlockHeightResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.MinBlockHeightResponse", len)?;
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MinBlockHeightResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
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
                            "height" => Ok(GeneratedField::Height),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MinBlockHeightResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.MinBlockHeightResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MinBlockHeightResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MinBlockHeightResponse {
                    height: height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.MinBlockHeightResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NodeLastBlockHeightRequest {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.NodeLastBlockHeightRequest", len)?;
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeLastBlockHeightRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "blockchain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = NodeLastBlockHeightRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.NodeLastBlockHeightRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NodeLastBlockHeightRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blockchain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v2::Blockchain>()? as i32);
                        }
                    }
                }
                Ok(NodeLastBlockHeightRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.NodeLastBlockHeightRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NodeLastBlockHeightResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.NodeLastBlockHeightResponse", len)?;
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeLastBlockHeightResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
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
                            "height" => Ok(GeneratedField::Height),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeLastBlockHeightResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.NodeLastBlockHeightResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NodeLastBlockHeightResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(NodeLastBlockHeightResponse {
                    height: height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.NodeLastBlockHeightResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubscribeRequest {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.SubscribeRequest", len)?;
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubscribeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "blockchain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = SubscribeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.SubscribeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubscribeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blockchain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v2::Blockchain>()? as i32);
                        }
                    }
                }
                Ok(SubscribeRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.SubscribeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubscribeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.item.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.SubscribeResponse", len)?;
        if let Some(v) = self.item.as_ref() {
            struct_ser.serialize_field("item", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubscribeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "item",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Item,
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
                            "item" => Ok(GeneratedField::Item),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubscribeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.SubscribeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubscribeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut item__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Item => {
                            if item__.is_some() {
                                return Err(serde::de::Error::duplicate_field("item"));
                            }
                            item__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SubscribeResponse {
                    item: item__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.SubscribeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TotalInfoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("eproxy.blocks.v2.TotalInfoRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TotalInfoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TotalInfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.TotalInfoRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TotalInfoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(TotalInfoRequest {
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.TotalInfoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TotalInfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.info.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.TotalInfoResponse", len)?;
        if !self.info.is_empty() {
            struct_ser.serialize_field("info", &self.info)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TotalInfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "info",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Info,
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
                            "info" => Ok(GeneratedField::Info),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TotalInfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.TotalInfoResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TotalInfoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(TotalInfoResponse {
                    info: info__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.TotalInfoResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for total_info_response::Item {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.db_min != 0 {
            len += 1;
        }
        if self.db_max != 0 {
            len += 1;
        }
        if self.db_diff != 0 {
            len += 1;
        }
        if self.node_max != 0 {
            len += 1;
        }
        if self.node_diff != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.blocks.v2.TotalInfoResponse.Item", len)?;
        if self.db_min != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("dbMin", ToString::to_string(&self.db_min).as_str())?;
        }
        if self.db_max != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("dbMax", ToString::to_string(&self.db_max).as_str())?;
        }
        if self.db_diff != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("dbDiff", ToString::to_string(&self.db_diff).as_str())?;
        }
        if self.node_max != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nodeMax", ToString::to_string(&self.node_max).as_str())?;
        }
        if self.node_diff != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nodeDiff", ToString::to_string(&self.node_diff).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for total_info_response::Item {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "db_min",
            "dbMin",
            "db_max",
            "dbMax",
            "db_diff",
            "dbDiff",
            "node_max",
            "nodeMax",
            "node_diff",
            "nodeDiff",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbMin,
            DbMax,
            DbDiff,
            NodeMax,
            NodeDiff,
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
                            "dbMin" | "db_min" => Ok(GeneratedField::DbMin),
                            "dbMax" | "db_max" => Ok(GeneratedField::DbMax),
                            "dbDiff" | "db_diff" => Ok(GeneratedField::DbDiff),
                            "nodeMax" | "node_max" => Ok(GeneratedField::NodeMax),
                            "nodeDiff" | "node_diff" => Ok(GeneratedField::NodeDiff),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = total_info_response::Item;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.blocks.v2.TotalInfoResponse.Item")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<total_info_response::Item, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_min__ = None;
                let mut db_max__ = None;
                let mut db_diff__ = None;
                let mut node_max__ = None;
                let mut node_diff__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DbMin => {
                            if db_min__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbMin"));
                            }
                            db_min__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DbMax => {
                            if db_max__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbMax"));
                            }
                            db_max__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DbDiff => {
                            if db_diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbDiff"));
                            }
                            db_diff__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NodeMax => {
                            if node_max__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeMax"));
                            }
                            node_max__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NodeDiff => {
                            if node_diff__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeDiff"));
                            }
                            node_diff__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(total_info_response::Item {
                    db_min: db_min__.unwrap_or_default(),
                    db_max: db_max__.unwrap_or_default(),
                    db_diff: db_diff__.unwrap_or_default(),
                    node_max: node_max__.unwrap_or_default(),
                    node_diff: node_diff__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.blocks.v2.TotalInfoResponse.Item", FIELDS, GeneratedVisitor)
    }
}
