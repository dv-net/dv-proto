// @generated
impl serde::Serialize for AdditionalData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tron.is_some() {
            len += 1;
        }
        if self.evm.is_some() {
            len += 1;
        }
        if self.btclike.is_some() {
            len += 1;
        }
        if self.monero.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.AdditionalData", len)?;
        if let Some(v) = self.tron.as_ref() {
            struct_ser.serialize_field("tron", v)?;
        }
        if let Some(v) = self.evm.as_ref() {
            struct_ser.serialize_field("evm", v)?;
        }
        if let Some(v) = self.btclike.as_ref() {
            struct_ser.serialize_field("btclike", v)?;
        }
        if let Some(v) = self.monero.as_ref() {
            struct_ser.serialize_field("monero", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AdditionalData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tron",
            "evm",
            "btclike",
            "monero",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tron,
            Evm,
            Btclike,
            Monero,
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
                            "tron" => Ok(GeneratedField::Tron),
                            "evm" => Ok(GeneratedField::Evm),
                            "btclike" => Ok(GeneratedField::Btclike),
                            "monero" => Ok(GeneratedField::Monero),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AdditionalData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.AdditionalData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AdditionalData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tron__ = None;
                let mut evm__ = None;
                let mut btclike__ = None;
                let mut monero__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tron => {
                            if tron__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tron"));
                            }
                            tron__ = map_.next_value()?;
                        }
                        GeneratedField::Evm => {
                            if evm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("evm"));
                            }
                            evm__ = map_.next_value()?;
                        }
                        GeneratedField::Btclike => {
                            if btclike__.is_some() {
                                return Err(serde::de::Error::duplicate_field("btclike"));
                            }
                            btclike__ = map_.next_value()?;
                        }
                        GeneratedField::Monero => {
                            if monero__.is_some() {
                                return Err(serde::de::Error::duplicate_field("monero"));
                            }
                            monero__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AdditionalData {
                    tron: tron__,
                    evm: evm__,
                    btclike: btclike__,
                    monero: monero__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.AdditionalData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BitcoinLikeAdditionalDataFull {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.BitcoinLikeAdditionalDataFull", len)?;
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
impl<'de> serde::Deserialize<'de> for BitcoinLikeAdditionalDataFull {
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
            type Value = BitcoinLikeAdditionalDataFull;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.BitcoinLikeAdditionalDataFull")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BitcoinLikeAdditionalDataFull, V::Error>
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
                Ok(BitcoinLikeAdditionalDataFull {
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
        deserializer.deserialize_struct("eproxy.transactions.v2.BitcoinLikeAdditionalDataFull", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Bpp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.a.is_empty() {
            len += 1;
        }
        if !self.a1.is_empty() {
            len += 1;
        }
        if !self.b.is_empty() {
            len += 1;
        }
        if !self.r1.is_empty() {
            len += 1;
        }
        if !self.s1.is_empty() {
            len += 1;
        }
        if !self.d1.is_empty() {
            len += 1;
        }
        if !self.l.is_empty() {
            len += 1;
        }
        if !self.r.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.Bpp", len)?;
        if !self.a.is_empty() {
            struct_ser.serialize_field("a", &self.a)?;
        }
        if !self.a1.is_empty() {
            struct_ser.serialize_field("a1", &self.a1)?;
        }
        if !self.b.is_empty() {
            struct_ser.serialize_field("b", &self.b)?;
        }
        if !self.r1.is_empty() {
            struct_ser.serialize_field("r1", &self.r1)?;
        }
        if !self.s1.is_empty() {
            struct_ser.serialize_field("s1", &self.s1)?;
        }
        if !self.d1.is_empty() {
            struct_ser.serialize_field("d1", &self.d1)?;
        }
        if !self.l.is_empty() {
            struct_ser.serialize_field("l", &self.l)?;
        }
        if !self.r.is_empty() {
            struct_ser.serialize_field("r", &self.r)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Bpp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "a",
            "a1",
            "b",
            "r1",
            "s1",
            "d1",
            "l",
            "r",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            A,
            A1,
            B,
            R1,
            S1,
            D1,
            L,
            R,
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
                            "a" => Ok(GeneratedField::A),
                            "a1" => Ok(GeneratedField::A1),
                            "b" => Ok(GeneratedField::B),
                            "r1" => Ok(GeneratedField::R1),
                            "s1" => Ok(GeneratedField::S1),
                            "d1" => Ok(GeneratedField::D1),
                            "l" => Ok(GeneratedField::L),
                            "r" => Ok(GeneratedField::R),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Bpp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.Bpp")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Bpp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut a__ = None;
                let mut a1__ = None;
                let mut b__ = None;
                let mut r1__ = None;
                let mut s1__ = None;
                let mut d1__ = None;
                let mut l__ = None;
                let mut r__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::A => {
                            if a__.is_some() {
                                return Err(serde::de::Error::duplicate_field("a"));
                            }
                            a__ = Some(map_.next_value()?);
                        }
                        GeneratedField::A1 => {
                            if a1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("a1"));
                            }
                            a1__ = Some(map_.next_value()?);
                        }
                        GeneratedField::B => {
                            if b__.is_some() {
                                return Err(serde::de::Error::duplicate_field("b"));
                            }
                            b__ = Some(map_.next_value()?);
                        }
                        GeneratedField::R1 => {
                            if r1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("r1"));
                            }
                            r1__ = Some(map_.next_value()?);
                        }
                        GeneratedField::S1 => {
                            if s1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("s1"));
                            }
                            s1__ = Some(map_.next_value()?);
                        }
                        GeneratedField::D1 => {
                            if d1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("d1"));
                            }
                            d1__ = Some(map_.next_value()?);
                        }
                        GeneratedField::L => {
                            if l__.is_some() {
                                return Err(serde::de::Error::duplicate_field("l"));
                            }
                            l__ = Some(map_.next_value()?);
                        }
                        GeneratedField::R => {
                            if r__.is_some() {
                                return Err(serde::de::Error::duplicate_field("r"));
                            }
                            r__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Bpp {
                    a: a__.unwrap_or_default(),
                    a1: a1__.unwrap_or_default(),
                    b: b__.unwrap_or_default(),
                    r1: r1__.unwrap_or_default(),
                    s1: s1__.unwrap_or_default(),
                    d1: d1__.unwrap_or_default(),
                    l: l__.unwrap_or_default(),
                    r: r__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.Bpp", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClsaGs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.s.is_empty() {
            len += 1;
        }
        if !self.c1.is_empty() {
            len += 1;
        }
        if !self.d.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.CLSAGs", len)?;
        if !self.s.is_empty() {
            struct_ser.serialize_field("s", &self.s)?;
        }
        if !self.c1.is_empty() {
            struct_ser.serialize_field("c1", &self.c1)?;
        }
        if !self.d.is_empty() {
            struct_ser.serialize_field("d", &self.d)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClsaGs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "s",
            "c1",
            "d",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            S,
            C1,
            D,
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
                            "s" => Ok(GeneratedField::S),
                            "c1" => Ok(GeneratedField::C1),
                            "d" => Ok(GeneratedField::D),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClsaGs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.CLSAGs")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClsaGs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut s__ = None;
                let mut c1__ = None;
                let mut d__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::S => {
                            if s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("s"));
                            }
                            s__ = Some(map_.next_value()?);
                        }
                        GeneratedField::C1 => {
                            if c1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("c1"));
                            }
                            c1__ = Some(map_.next_value()?);
                        }
                        GeneratedField::D => {
                            if d__.is_some() {
                                return Err(serde::de::Error::duplicate_field("d"));
                            }
                            d__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClsaGs {
                    s: s__.unwrap_or_default(),
                    c1: c1__.unwrap_or_default(),
                    d: d__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.CLSAGs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Coinbase {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.Coinbase", len)?;
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Coinbase {
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
            type Value = Coinbase;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.Coinbase")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Coinbase, V::Error>
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
                Ok(Coinbase {
                    height: height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.Coinbase", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvmAdditionalDataFull {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.EVMAdditionalDataFull", len)?;
        if self.chain_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
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
impl<'de> serde::Deserialize<'de> for EvmAdditionalDataFull {
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
            type Value = EvmAdditionalDataFull;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.EVMAdditionalDataFull")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvmAdditionalDataFull, V::Error>
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
                Ok(EvmAdditionalDataFull {
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
        deserializer.deserialize_struct("eproxy.transactions.v2.EVMAdditionalDataFull", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EcdhInfo {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.EcdhInfo", len)?;
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EcdhInfo {
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
            type Value = EcdhInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.EcdhInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EcdhInfo, V::Error>
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
                Ok(EcdhInfo {
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.EcdhInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Error {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        if self.code.is_some() {
            len += 1;
        }
        if self.detail.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.Error", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.detail.as_ref() {
            struct_ser.serialize_field("detail", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Error {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
            "code",
            "detail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
            Code,
            Detail,
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
                            "message" => Ok(GeneratedField::Message),
                            "code" => Ok(GeneratedField::Code),
                            "detail" => Ok(GeneratedField::Detail),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Error;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.Error")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Error, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                let mut code__ = None;
                let mut detail__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map_.next_value()?;
                        }
                        GeneratedField::Detail => {
                            if detail__.is_some() {
                                return Err(serde::de::Error::duplicate_field("detail"));
                            }
                            detail__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Error {
                    message: message__.unwrap_or_default(),
                    code: code__,
                    detail: detail__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.Error", FIELDS, GeneratedVisitor)
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
        if !self.topics.is_empty() {
            len += 1;
        }
        if self.encoded_data.is_some() {
            len += 1;
        }
        if self.blockchain_uniq_key.is_some() {
            len += 1;
        }
        if self.asset_identifier.is_some() {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.Event", len)?;
        if let Some(v) = self.r#type.as_ref() {
            let v = EventType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.original_data.as_ref() {
            struct_ser.serialize_field("originalData", v)?;
        }
        if !self.topics.is_empty() {
            struct_ser.serialize_field("topics", &self.topics)?;
        }
        if let Some(v) = self.encoded_data.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("encodedData", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.blockchain_uniq_key.as_ref() {
            struct_ser.serialize_field("blockchainUniqKey", v)?;
        }
        if let Some(v) = self.asset_identifier.as_ref() {
            struct_ser.serialize_field("assetIdentifier", v)?;
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
            "topics",
            "encoded_data",
            "encodedData",
            "blockchain_uniq_key",
            "blockchainUniqKey",
            "asset_identifier",
            "assetIdentifier",
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
            Topics,
            EncodedData,
            BlockchainUniqKey,
            AssetIdentifier,
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
                            "topics" => Ok(GeneratedField::Topics),
                            "encodedData" | "encoded_data" => Ok(GeneratedField::EncodedData),
                            "blockchainUniqKey" | "blockchain_uniq_key" => Ok(GeneratedField::BlockchainUniqKey),
                            "assetIdentifier" | "asset_identifier" => Ok(GeneratedField::AssetIdentifier),
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
                formatter.write_str("struct eproxy.transactions.v2.Event")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Event, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut original_data__ = None;
                let mut topics__ = None;
                let mut encoded_data__ = None;
                let mut blockchain_uniq_key__ = None;
                let mut asset_identifier__ = None;
                let mut address_from__ = None;
                let mut address_to__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<EventType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::OriginalData => {
                            if original_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalData"));
                            }
                            original_data__ = map_.next_value()?;
                        }
                        GeneratedField::Topics => {
                            if topics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("topics"));
                            }
                            topics__ = Some(map_.next_value()?);
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
                        GeneratedField::AssetIdentifier => {
                            if asset_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIdentifier"));
                            }
                            asset_identifier__ = map_.next_value()?;
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
                    topics: topics__.unwrap_or_default(),
                    encoded_data: encoded_data__,
                    blockchain_uniq_key: blockchain_uniq_key__,
                    asset_identifier: asset_identifier__,
                    address_from: address_from__,
                    address_to: address_to__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.Event", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "EVENT_TYPE_UNSPECIFIED",
            Self::Transfer => "EVENT_TYPE_TRANSFER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EventType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EVENT_TYPE_UNSPECIFIED",
            "EVENT_TYPE_TRANSFER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "EVENT_TYPE_UNSPECIFIED" => Ok(EventType::Unspecified),
                    "EVENT_TYPE_TRANSFER" => Ok(EventType::Transfer),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        if self.search.is_some() {
            len += 1;
        }
        if self.hash.is_some() {
            len += 1;
        }
        if self.block_height.is_some() {
            len += 1;
        }
        if self.block_height_offset.is_some() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if self.contract_address.is_some() {
            len += 1;
        }
        if self.contract_type.is_some() {
            len += 1;
        }
        if self.is_include_additional_data {
            len += 1;
        }
        if self.tron_params.is_some() {
            len += 1;
        }
        if self.evm_params.is_some() {
            len += 1;
        }
        if self.bitcoin_like_params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.FindRequest", len)?;
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        if let Some(v) = self.common.as_ref() {
            struct_ser.serialize_field("common", v)?;
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
        if let Some(v) = self.block_height_offset.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("blockHeightOffset", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.contract_address.as_ref() {
            struct_ser.serialize_field("contractAddress", v)?;
        }
        if let Some(v) = self.contract_type.as_ref() {
            struct_ser.serialize_field("contractType", v)?;
        }
        if self.is_include_additional_data {
            struct_ser.serialize_field("isIncludeAdditionalData", &self.is_include_additional_data)?;
        }
        if let Some(v) = self.tron_params.as_ref() {
            struct_ser.serialize_field("tronParams", v)?;
        }
        if let Some(v) = self.evm_params.as_ref() {
            struct_ser.serialize_field("evmParams", v)?;
        }
        if let Some(v) = self.bitcoin_like_params.as_ref() {
            struct_ser.serialize_field("bitcoinLikeParams", v)?;
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
            "search",
            "hash",
            "block_height",
            "blockHeight",
            "block_height_offset",
            "blockHeightOffset",
            "address",
            "contract_address",
            "contractAddress",
            "contract_type",
            "contractType",
            "is_include_additional_data",
            "isIncludeAdditionalData",
            "tron_params",
            "tronParams",
            "evm_params",
            "evmParams",
            "bitcoin_like_params",
            "bitcoinLikeParams",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Blockchain,
            Common,
            Search,
            Hash,
            BlockHeight,
            BlockHeightOffset,
            Address,
            ContractAddress,
            ContractType,
            IsIncludeAdditionalData,
            TronParams,
            EvmParams,
            BitcoinLikeParams,
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
                            "search" => Ok(GeneratedField::Search),
                            "hash" => Ok(GeneratedField::Hash),
                            "blockHeight" | "block_height" => Ok(GeneratedField::BlockHeight),
                            "blockHeightOffset" | "block_height_offset" => Ok(GeneratedField::BlockHeightOffset),
                            "address" => Ok(GeneratedField::Address),
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            "contractType" | "contract_type" => Ok(GeneratedField::ContractType),
                            "isIncludeAdditionalData" | "is_include_additional_data" => Ok(GeneratedField::IsIncludeAdditionalData),
                            "tronParams" | "tron_params" => Ok(GeneratedField::TronParams),
                            "evmParams" | "evm_params" => Ok(GeneratedField::EvmParams),
                            "bitcoinLikeParams" | "bitcoin_like_params" => Ok(GeneratedField::BitcoinLikeParams),
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
                formatter.write_str("struct eproxy.transactions.v2.FindRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FindRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blockchain__ = None;
                let mut common__ = None;
                let mut search__ = None;
                let mut hash__ = None;
                let mut block_height__ = None;
                let mut block_height_offset__ = None;
                let mut address__ = None;
                let mut contract_address__ = None;
                let mut contract_type__ = None;
                let mut is_include_additional_data__ = None;
                let mut tron_params__ = None;
                let mut evm_params__ = None;
                let mut bitcoin_like_params__ = None;
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
                        GeneratedField::BlockHeightOffset => {
                            if block_height_offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeightOffset"));
                            }
                            block_height_offset__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map_.next_value()?;
                        }
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = map_.next_value()?;
                        }
                        GeneratedField::ContractType => {
                            if contract_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractType"));
                            }
                            contract_type__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::IsIncludeAdditionalData => {
                            if is_include_additional_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isIncludeAdditionalData"));
                            }
                            is_include_additional_data__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TronParams => {
                            if tron_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tronParams"));
                            }
                            tron_params__ = map_.next_value()?;
                        }
                        GeneratedField::EvmParams => {
                            if evm_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("evmParams"));
                            }
                            evm_params__ = map_.next_value()?;
                        }
                        GeneratedField::BitcoinLikeParams => {
                            if bitcoin_like_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitcoinLikeParams"));
                            }
                            bitcoin_like_params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FindRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                    common: common__,
                    search: search__,
                    hash: hash__,
                    block_height: block_height__,
                    block_height_offset: block_height_offset__,
                    address: address__,
                    contract_address: contract_address__,
                    contract_type: contract_type__,
                    is_include_additional_data: is_include_additional_data__.unwrap_or_default(),
                    tron_params: tron_params__,
                    evm_params: evm_params__,
                    bitcoin_like_params: bitcoin_like_params__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.FindRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for find_request::BitcoinLikeParams {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.FindRequest.BitcoinLikeParams", len)?;
        if let Some(v) = self.filter_by_address.as_ref() {
            struct_ser.serialize_field("filterByAddress", v)?;
        }
        if let Some(v) = self.filter_by_mempool.as_ref() {
            struct_ser.serialize_field("filterByMempool", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for find_request::BitcoinLikeParams {
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
            type Value = find_request::BitcoinLikeParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.FindRequest.BitcoinLikeParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<find_request::BitcoinLikeParams, V::Error>
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
                Ok(find_request::BitcoinLikeParams {
                    filter_by_address: filter_by_address__,
                    filter_by_mempool: filter_by_mempool__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.FindRequest.BitcoinLikeParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for find_request::EvmParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("eproxy.transactions.v2.FindRequest.EVMParams", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for find_request::EvmParams {
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
            type Value = find_request::EvmParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.FindRequest.EVMParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<find_request::EvmParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(find_request::EvmParams {
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.FindRequest.EVMParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for find_request::TronParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("eproxy.transactions.v2.FindRequest.TronParams", len)?;
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
            type Value = find_request::TronParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.FindRequest.TronParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<find_request::TronParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(find_request::TronParams {
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.FindRequest.TronParams", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.FindResponse", len)?;
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
                formatter.write_str("struct eproxy.transactions.v2.FindResponse")
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
        deserializer.deserialize_struct("eproxy.transactions.v2.FindResponse", FIELDS, GeneratedVisitor)
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
        if self.blockchain != 0 {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.GetInfoRequest", len)?;
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
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
            "blockchain",
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Blockchain,
            Hash,
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
                            "hash" => Ok(GeneratedField::Hash),
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
                formatter.write_str("struct eproxy.transactions.v2.GetInfoRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetInfoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut blockchain__ = None;
                let mut hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Blockchain => {
                            if blockchain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchain"));
                            }
                            blockchain__ = Some(map_.next_value::<super::super::common::v2::Blockchain>()? as i32);
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetInfoRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.GetInfoRequest", FIELDS, GeneratedVisitor)
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
        if self.transaction.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.GetInfoResponse", len)?;
        if let Some(v) = self.transaction.as_ref() {
            struct_ser.serialize_field("transaction", v)?;
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
            "transaction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transaction,
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
                            "transaction" => Ok(GeneratedField::Transaction),
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
                formatter.write_str("struct eproxy.transactions.v2.GetInfoResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetInfoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transaction__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Transaction => {
                            if transaction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transaction"));
                            }
                            transaction__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetInfoResponse {
                    transaction: transaction__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.GetInfoResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Key {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.amount != 0 {
            len += 1;
        }
        if !self.key_offsets.is_empty() {
            len += 1;
        }
        if !self.k_image.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.Key", len)?;
        if self.amount != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("amount", ToString::to_string(&self.amount).as_str())?;
        }
        if !self.key_offsets.is_empty() {
            struct_ser.serialize_field("keyOffsets", &self.key_offsets.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.k_image.is_empty() {
            struct_ser.serialize_field("kImage", &self.k_image)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Key {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "amount",
            "key_offsets",
            "keyOffsets",
            "k_image",
            "kImage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
            KeyOffsets,
            KImage,
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
                            "keyOffsets" | "key_offsets" => Ok(GeneratedField::KeyOffsets),
                            "kImage" | "k_image" => Ok(GeneratedField::KImage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Key;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.Key")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Key, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
                let mut key_offsets__ = None;
                let mut k_image__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::KeyOffsets => {
                            if key_offsets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyOffsets"));
                            }
                            key_offsets__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::KImage => {
                            if k_image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kImage"));
                            }
                            k_image__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Key {
                    amount: amount__.unwrap_or_default(),
                    key_offsets: key_offsets__.unwrap_or_default(),
                    k_image: k_image__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.Key", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MoneroAdditionalDataFull {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.as_hex.is_empty() {
            len += 1;
        }
        if self.block_timestamp != 0 {
            len += 1;
        }
        if self.double_spend_seen {
            len += 1;
        }
        if !self.output_indices.is_empty() {
            len += 1;
        }
        if !self.prunable_as_hex.is_empty() {
            len += 1;
        }
        if !self.prunable_hash.is_empty() {
            len += 1;
        }
        if !self.pruned_as_hex.is_empty() {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.MoneroAdditionalDataFull", len)?;
        if !self.as_hex.is_empty() {
            struct_ser.serialize_field("asHex", &self.as_hex)?;
        }
        if self.block_timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("blockTimestamp", ToString::to_string(&self.block_timestamp).as_str())?;
        }
        if self.double_spend_seen {
            struct_ser.serialize_field("doubleSpendSeen", &self.double_spend_seen)?;
        }
        if !self.output_indices.is_empty() {
            struct_ser.serialize_field("outputIndices", &self.output_indices.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.prunable_as_hex.is_empty() {
            struct_ser.serialize_field("prunableAsHex", &self.prunable_as_hex)?;
        }
        if !self.prunable_hash.is_empty() {
            struct_ser.serialize_field("prunableHash", &self.prunable_hash)?;
        }
        if !self.pruned_as_hex.is_empty() {
            struct_ser.serialize_field("prunedAsHex", &self.pruned_as_hex)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MoneroAdditionalDataFull {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "as_hex",
            "asHex",
            "block_timestamp",
            "blockTimestamp",
            "double_spend_seen",
            "doubleSpendSeen",
            "output_indices",
            "outputIndices",
            "prunable_as_hex",
            "prunableAsHex",
            "prunable_hash",
            "prunableHash",
            "pruned_as_hex",
            "prunedAsHex",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AsHex,
            BlockTimestamp,
            DoubleSpendSeen,
            OutputIndices,
            PrunableAsHex,
            PrunableHash,
            PrunedAsHex,
            Data,
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
                            "asHex" | "as_hex" => Ok(GeneratedField::AsHex),
                            "blockTimestamp" | "block_timestamp" => Ok(GeneratedField::BlockTimestamp),
                            "doubleSpendSeen" | "double_spend_seen" => Ok(GeneratedField::DoubleSpendSeen),
                            "outputIndices" | "output_indices" => Ok(GeneratedField::OutputIndices),
                            "prunableAsHex" | "prunable_as_hex" => Ok(GeneratedField::PrunableAsHex),
                            "prunableHash" | "prunable_hash" => Ok(GeneratedField::PrunableHash),
                            "prunedAsHex" | "pruned_as_hex" => Ok(GeneratedField::PrunedAsHex),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MoneroAdditionalDataFull;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.MoneroAdditionalDataFull")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MoneroAdditionalDataFull, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut as_hex__ = None;
                let mut block_timestamp__ = None;
                let mut double_spend_seen__ = None;
                let mut output_indices__ = None;
                let mut prunable_as_hex__ = None;
                let mut prunable_hash__ = None;
                let mut pruned_as_hex__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AsHex => {
                            if as_hex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asHex"));
                            }
                            as_hex__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlockTimestamp => {
                            if block_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockTimestamp"));
                            }
                            block_timestamp__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DoubleSpendSeen => {
                            if double_spend_seen__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doubleSpendSeen"));
                            }
                            double_spend_seen__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutputIndices => {
                            if output_indices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputIndices"));
                            }
                            output_indices__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::PrunableAsHex => {
                            if prunable_as_hex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prunableAsHex"));
                            }
                            prunable_as_hex__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrunableHash => {
                            if prunable_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prunableHash"));
                            }
                            prunable_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrunedAsHex => {
                            if pruned_as_hex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prunedAsHex"));
                            }
                            pruned_as_hex__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MoneroAdditionalDataFull {
                    as_hex: as_hex__.unwrap_or_default(),
                    block_timestamp: block_timestamp__.unwrap_or_default(),
                    double_spend_seen: double_spend_seen__.unwrap_or_default(),
                    output_indices: output_indices__.unwrap_or_default(),
                    prunable_as_hex: prunable_as_hex__.unwrap_or_default(),
                    prunable_hash: prunable_hash__.unwrap_or_default(),
                    pruned_as_hex: pruned_as_hex__.unwrap_or_default(),
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.MoneroAdditionalDataFull", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Prunable {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nbp != 0 {
            len += 1;
        }
        if !self.bpp.is_empty() {
            len += 1;
        }
        if !self.clsags.is_empty() {
            len += 1;
        }
        if !self.pseudo_outs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.Prunable", len)?;
        if self.nbp != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nbp", ToString::to_string(&self.nbp).as_str())?;
        }
        if !self.bpp.is_empty() {
            struct_ser.serialize_field("bpp", &self.bpp)?;
        }
        if !self.clsags.is_empty() {
            struct_ser.serialize_field("clsags", &self.clsags)?;
        }
        if !self.pseudo_outs.is_empty() {
            struct_ser.serialize_field("pseudoOuts", &self.pseudo_outs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Prunable {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nbp",
            "bpp",
            "clsags",
            "pseudo_outs",
            "pseudoOuts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nbp,
            Bpp,
            Clsags,
            PseudoOuts,
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
                            "nbp" => Ok(GeneratedField::Nbp),
                            "bpp" => Ok(GeneratedField::Bpp),
                            "clsags" => Ok(GeneratedField::Clsags),
                            "pseudoOuts" | "pseudo_outs" => Ok(GeneratedField::PseudoOuts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Prunable;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.Prunable")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Prunable, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nbp__ = None;
                let mut bpp__ = None;
                let mut clsags__ = None;
                let mut pseudo_outs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nbp => {
                            if nbp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nbp"));
                            }
                            nbp__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Bpp => {
                            if bpp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bpp"));
                            }
                            bpp__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Clsags => {
                            if clsags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clsags"));
                            }
                            clsags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PseudoOuts => {
                            if pseudo_outs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pseudoOuts"));
                            }
                            pseudo_outs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Prunable {
                    nbp: nbp__.unwrap_or_default(),
                    bpp: bpp__.unwrap_or_default(),
                    clsags: clsags__.unwrap_or_default(),
                    pseudo_outs: pseudo_outs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.Prunable", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RctSignatures {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if self.txn_fee != 0 {
            len += 1;
        }
        if !self.ecdh_info.is_empty() {
            len += 1;
        }
        if !self.out_pk.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.RctSignatures", len)?;
        if self.r#type != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("type", ToString::to_string(&self.r#type).as_str())?;
        }
        if self.txn_fee != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("txnFee", ToString::to_string(&self.txn_fee).as_str())?;
        }
        if !self.ecdh_info.is_empty() {
            struct_ser.serialize_field("ecdhInfo", &self.ecdh_info)?;
        }
        if !self.out_pk.is_empty() {
            struct_ser.serialize_field("outPk", &self.out_pk)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RctSignatures {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "txn_fee",
            "txnFee",
            "ecdh_info",
            "ecdhInfo",
            "out_pk",
            "outPk",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            TxnFee,
            EcdhInfo,
            OutPk,
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
                            "txnFee" | "txn_fee" => Ok(GeneratedField::TxnFee),
                            "ecdhInfo" | "ecdh_info" => Ok(GeneratedField::EcdhInfo),
                            "outPk" | "out_pk" => Ok(GeneratedField::OutPk),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RctSignatures;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.RctSignatures")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RctSignatures, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut txn_fee__ = None;
                let mut ecdh_info__ = None;
                let mut out_pk__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TxnFee => {
                            if txn_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txnFee"));
                            }
                            txn_fee__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EcdhInfo => {
                            if ecdh_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ecdhInfo"));
                            }
                            ecdh_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutPk => {
                            if out_pk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outPk"));
                            }
                            out_pk__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RctSignatures {
                    r#type: r#type__.unwrap_or_default(),
                    txn_fee: txn_fee__.unwrap_or_default(),
                    ecdh_info: ecdh_info__.unwrap_or_default(),
                    out_pk: out_pk__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.RctSignatures", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SuggestTxFeeRequest {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.SuggestTxFeeRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for SuggestTxFeeRequest {
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
            type Value = SuggestTxFeeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.SuggestTxFeeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SuggestTxFeeRequest, V::Error>
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
                Ok(SuggestTxFeeRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                    identifier: identifier__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.SuggestTxFeeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SuggestTxFeeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fee.is_empty() {
            len += 1;
        }
        if self.additional_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.SuggestTxFeeResponse", len)?;
        if !self.fee.is_empty() {
            struct_ser.serialize_field("fee", &self.fee)?;
        }
        if let Some(v) = self.additional_data.as_ref() {
            match v {
                suggest_tx_fee_response::AdditionalData::TronData(v) => {
                    struct_ser.serialize_field("tronData", v)?;
                }
                suggest_tx_fee_response::AdditionalData::EvmData(v) => {
                    struct_ser.serialize_field("evmData", v)?;
                }
                suggest_tx_fee_response::AdditionalData::BitcoinLikeData(v) => {
                    struct_ser.serialize_field("bitcoinLikeData", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SuggestTxFeeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fee",
            "tron_data",
            "tronData",
            "evm_data",
            "evmData",
            "bitcoin_like_data",
            "bitcoinLikeData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fee,
            TronData,
            EvmData,
            BitcoinLikeData,
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
                            "fee" => Ok(GeneratedField::Fee),
                            "tronData" | "tron_data" => Ok(GeneratedField::TronData),
                            "evmData" | "evm_data" => Ok(GeneratedField::EvmData),
                            "bitcoinLikeData" | "bitcoin_like_data" => Ok(GeneratedField::BitcoinLikeData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SuggestTxFeeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.SuggestTxFeeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SuggestTxFeeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fee__ = None;
                let mut additional_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TronData => {
                            if additional_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tronData"));
                            }
                            additional_data__ = map_.next_value::<::std::option::Option<_>>()?.map(suggest_tx_fee_response::AdditionalData::TronData)
;
                        }
                        GeneratedField::EvmData => {
                            if additional_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("evmData"));
                            }
                            additional_data__ = map_.next_value::<::std::option::Option<_>>()?.map(suggest_tx_fee_response::AdditionalData::EvmData)
;
                        }
                        GeneratedField::BitcoinLikeData => {
                            if additional_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitcoinLikeData"));
                            }
                            additional_data__ = map_.next_value::<::std::option::Option<_>>()?.map(suggest_tx_fee_response::AdditionalData::BitcoinLikeData)
;
                        }
                    }
                }
                Ok(SuggestTxFeeResponse {
                    fee: fee__.unwrap_or_default(),
                    additional_data: additional_data__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.SuggestTxFeeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for suggest_tx_fee_response::BitcoinLikeFeeData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fee_rate != 0 {
            len += 1;
        }
        if self.estimated_size != 0 {
            len += 1;
        }
        if !self.priority.is_empty() {
            len += 1;
        }
        if self.utxo_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.SuggestTxFeeResponse.BitcoinLikeFeeData", len)?;
        if self.fee_rate != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("feeRate", ToString::to_string(&self.fee_rate).as_str())?;
        }
        if self.estimated_size != 0 {
            struct_ser.serialize_field("estimatedSize", &self.estimated_size)?;
        }
        if !self.priority.is_empty() {
            struct_ser.serialize_field("priority", &self.priority)?;
        }
        if self.utxo_count != 0 {
            struct_ser.serialize_field("utxoCount", &self.utxo_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for suggest_tx_fee_response::BitcoinLikeFeeData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fee_rate",
            "feeRate",
            "estimated_size",
            "estimatedSize",
            "priority",
            "utxo_count",
            "utxoCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeeRate,
            EstimatedSize,
            Priority,
            UtxoCount,
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
                            "feeRate" | "fee_rate" => Ok(GeneratedField::FeeRate),
                            "estimatedSize" | "estimated_size" => Ok(GeneratedField::EstimatedSize),
                            "priority" => Ok(GeneratedField::Priority),
                            "utxoCount" | "utxo_count" => Ok(GeneratedField::UtxoCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = suggest_tx_fee_response::BitcoinLikeFeeData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.SuggestTxFeeResponse.BitcoinLikeFeeData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<suggest_tx_fee_response::BitcoinLikeFeeData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fee_rate__ = None;
                let mut estimated_size__ = None;
                let mut priority__ = None;
                let mut utxo_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeeRate => {
                            if fee_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeRate"));
                            }
                            fee_rate__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EstimatedSize => {
                            if estimated_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("estimatedSize"));
                            }
                            estimated_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UtxoCount => {
                            if utxo_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("utxoCount"));
                            }
                            utxo_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(suggest_tx_fee_response::BitcoinLikeFeeData {
                    fee_rate: fee_rate__.unwrap_or_default(),
                    estimated_size: estimated_size__.unwrap_or_default(),
                    priority: priority__.unwrap_or_default(),
                    utxo_count: utxo_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.SuggestTxFeeResponse.BitcoinLikeFeeData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for suggest_tx_fee_response::EvmFeeData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("eproxy.transactions.v2.SuggestTxFeeResponse.EVMFeeData", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for suggest_tx_fee_response::EvmFeeData {
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
            type Value = suggest_tx_fee_response::EvmFeeData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.SuggestTxFeeResponse.EVMFeeData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<suggest_tx_fee_response::EvmFeeData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(suggest_tx_fee_response::EvmFeeData {
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.SuggestTxFeeResponse.EVMFeeData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for suggest_tx_fee_response::TronFeeData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.energy_usage != 0 {
            len += 1;
        }
        if self.bandwidth_usage != 0 {
            len += 1;
        }
        if !self.burn_trx_fee.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.SuggestTxFeeResponse.TronFeeData", len)?;
        if self.energy_usage != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("energyUsage", ToString::to_string(&self.energy_usage).as_str())?;
        }
        if self.bandwidth_usage != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("bandwidthUsage", ToString::to_string(&self.bandwidth_usage).as_str())?;
        }
        if !self.burn_trx_fee.is_empty() {
            struct_ser.serialize_field("burnTrxFee", &self.burn_trx_fee)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for suggest_tx_fee_response::TronFeeData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "energy_usage",
            "energyUsage",
            "bandwidth_usage",
            "bandwidthUsage",
            "burn_trx_fee",
            "burnTrxFee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EnergyUsage,
            BandwidthUsage,
            BurnTrxFee,
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
                            "bandwidthUsage" | "bandwidth_usage" => Ok(GeneratedField::BandwidthUsage),
                            "burnTrxFee" | "burn_trx_fee" => Ok(GeneratedField::BurnTrxFee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = suggest_tx_fee_response::TronFeeData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.SuggestTxFeeResponse.TronFeeData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<suggest_tx_fee_response::TronFeeData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut energy_usage__ = None;
                let mut bandwidth_usage__ = None;
                let mut burn_trx_fee__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EnergyUsage => {
                            if energy_usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("energyUsage"));
                            }
                            energy_usage__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BandwidthUsage => {
                            if bandwidth_usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bandwidthUsage"));
                            }
                            bandwidth_usage__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BurnTrxFee => {
                            if burn_trx_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burnTrxFee"));
                            }
                            burn_trx_fee__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(suggest_tx_fee_response::TronFeeData {
                    energy_usage: energy_usage__.unwrap_or_default(),
                    bandwidth_usage: bandwidth_usage__.unwrap_or_default(),
                    burn_trx_fee: burn_trx_fee__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.SuggestTxFeeResponse.TronFeeData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Target {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tagged_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.Target", len)?;
        if let Some(v) = self.tagged_key.as_ref() {
            struct_ser.serialize_field("taggedKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Target {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tagged_key",
            "taggedKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaggedKey,
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
                            "taggedKey" | "tagged_key" => Ok(GeneratedField::TaggedKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Target;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.Target")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Target, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tagged_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TaggedKey => {
                            if tagged_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taggedKey"));
                            }
                            tagged_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Target {
                    tagged_key: tagged_key__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.Target", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TargetKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.view_tag.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.TargetKey", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.view_tag.is_empty() {
            struct_ser.serialize_field("viewTag", &self.view_tag)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TargetKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "view_tag",
            "viewTag",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            ViewTag,
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
                            "key" => Ok(GeneratedField::Key),
                            "viewTag" | "view_tag" => Ok(GeneratedField::ViewTag),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TargetKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.TargetKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TargetKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut view_tag__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ViewTag => {
                            if view_tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("viewTag"));
                            }
                            view_tag__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TargetKey {
                    key: key__.unwrap_or_default(),
                    view_tag: view_tag__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.TargetKey", FIELDS, GeneratedVisitor)
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
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.fee.is_empty() {
            len += 1;
        }
        if self.asset_identifier.is_some() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if self.in_mempool {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        if self.additional_data.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.Transaction", len)?;
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
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.fee.is_empty() {
            struct_ser.serialize_field("fee", &self.fee)?;
        }
        if let Some(v) = self.asset_identifier.as_ref() {
            struct_ser.serialize_field("assetIdentifier", v)?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", &self.index)?;
        }
        if self.in_mempool {
            struct_ser.serialize_field("inMempool", &self.in_mempool)?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        if let Some(v) = self.additional_data.as_ref() {
            struct_ser.serialize_field("additionalData", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
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
            "amount",
            "fee",
            "asset_identifier",
            "assetIdentifier",
            "index",
            "in_mempool",
            "inMempool",
            "status",
            "error",
            "events",
            "additional_data",
            "additionalData",
            "created_at",
            "createdAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            BlockHeight,
            Confirmations,
            AddressFrom,
            AddressTo,
            Amount,
            Fee,
            AssetIdentifier,
            Index,
            InMempool,
            Status,
            Error,
            Events,
            AdditionalData,
            CreatedAt,
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
                            "amount" => Ok(GeneratedField::Amount),
                            "fee" => Ok(GeneratedField::Fee),
                            "assetIdentifier" | "asset_identifier" => Ok(GeneratedField::AssetIdentifier),
                            "index" => Ok(GeneratedField::Index),
                            "inMempool" | "in_mempool" => Ok(GeneratedField::InMempool),
                            "status" => Ok(GeneratedField::Status),
                            "error" => Ok(GeneratedField::Error),
                            "events" => Ok(GeneratedField::Events),
                            "additionalData" | "additional_data" => Ok(GeneratedField::AdditionalData),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
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
                formatter.write_str("struct eproxy.transactions.v2.Transaction")
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
                let mut amount__ = None;
                let mut fee__ = None;
                let mut asset_identifier__ = None;
                let mut index__ = None;
                let mut in_mempool__ = None;
                let mut status__ = None;
                let mut error__ = None;
                let mut events__ = None;
                let mut additional_data__ = None;
                let mut created_at__ = None;
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
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetIdentifier => {
                            if asset_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIdentifier"));
                            }
                            asset_identifier__ = map_.next_value()?;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::InMempool => {
                            if in_mempool__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inMempool"));
                            }
                            in_mempool__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AdditionalData => {
                            if additional_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalData"));
                            }
                            additional_data__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Transaction {
                    hash: hash__.unwrap_or_default(),
                    block_height: block_height__.unwrap_or_default(),
                    confirmations: confirmations__.unwrap_or_default(),
                    address_from: address_from__,
                    address_to: address_to__,
                    amount: amount__.unwrap_or_default(),
                    fee: fee__.unwrap_or_default(),
                    asset_identifier: asset_identifier__,
                    index: index__.unwrap_or_default(),
                    in_mempool: in_mempool__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    error: error__,
                    events: events__.unwrap_or_default(),
                    additional_data: additional_data__,
                    created_at: created_at__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.Transaction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TronAdditionalData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.contract_type != 0 {
            len += 1;
        }
        if self.asset_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.TronAdditionalData", len)?;
        if self.contract_type != 0 {
            struct_ser.serialize_field("contractType", &self.contract_type)?;
        }
        if let Some(v) = self.asset_kind.as_ref() {
            struct_ser.serialize_field("assetKind", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TronAdditionalData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_type",
            "contractType",
            "asset_kind",
            "assetKind",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractType,
            AssetKind,
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
                            "assetKind" | "asset_kind" => Ok(GeneratedField::AssetKind),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TronAdditionalData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.TronAdditionalData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TronAdditionalData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut contract_type__ = None;
                let mut asset_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContractType => {
                            if contract_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractType"));
                            }
                            contract_type__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AssetKind => {
                            if asset_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetKind"));
                            }
                            asset_kind__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TronAdditionalData {
                    contract_type: contract_type__.unwrap_or_default(),
                    asset_kind: asset_kind__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.TronAdditionalData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TronAdditionalDataFull {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.contract_type != 0 {
            len += 1;
        }
        if self.asset_kind.is_some() {
            len += 1;
        }
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
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.TronAdditionalDataFull", len)?;
        if self.contract_type != 0 {
            struct_ser.serialize_field("contractType", &self.contract_type)?;
        }
        if let Some(v) = self.asset_kind.as_ref() {
            struct_ser.serialize_field("assetKind", v)?;
        }
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
impl<'de> serde::Deserialize<'de> for TronAdditionalDataFull {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "contract_type",
            "contractType",
            "asset_kind",
            "assetKind",
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
            ContractType,
            AssetKind,
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
                            "contractType" | "contract_type" => Ok(GeneratedField::ContractType),
                            "assetKind" | "asset_kind" => Ok(GeneratedField::AssetKind),
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
            type Value = TronAdditionalDataFull;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.TronAdditionalDataFull")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TronAdditionalDataFull, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut contract_type__ = None;
                let mut asset_kind__ = None;
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
                        GeneratedField::ContractType => {
                            if contract_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractType"));
                            }
                            contract_type__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AssetKind => {
                            if asset_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetKind"));
                            }
                            asset_kind__ = map_.next_value()?;
                        }
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
                Ok(TronAdditionalDataFull {
                    contract_type: contract_type__.unwrap_or_default(),
                    asset_kind: asset_kind__,
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
        deserializer.deserialize_struct("eproxy.transactions.v2.TronAdditionalDataFull", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Vin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.gen.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.Vin", len)?;
        if let Some(v) = self.gen.as_ref() {
            struct_ser.serialize_field("gen", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Vin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gen",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Gen,
            Key,
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
                            "gen" => Ok(GeneratedField::Gen),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Vin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.Vin")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Vin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut gen__ = None;
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Gen => {
                            if gen__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gen"));
                            }
                            gen__ = map_.next_value()?;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Vin {
                    gen: gen__,
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.Vin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Vout {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.amount != 0 {
            len += 1;
        }
        if self.target.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.Vout", len)?;
        if self.amount != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("amount", ToString::to_string(&self.amount).as_str())?;
        }
        if let Some(v) = self.target.as_ref() {
            struct_ser.serialize_field("target", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Vout {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "amount",
            "target",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
            Target,
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
                            "target" => Ok(GeneratedField::Target),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Vout;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.Vout")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Vout, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
                let mut target__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Vout {
                    amount: amount__.unwrap_or_default(),
                    target: target__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.Vout", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for XmrMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if self.unlock_time != 0 {
            len += 1;
        }
        if !self.vin.is_empty() {
            len += 1;
        }
        if !self.vout.is_empty() {
            len += 1;
        }
        if !self.extra.is_empty() {
            len += 1;
        }
        if self.rct_signatures.is_some() {
            len += 1;
        }
        if self.rctsig_prunable.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.transactions.v2.XmrMetadata", len)?;
        if self.version != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("version", ToString::to_string(&self.version).as_str())?;
        }
        if self.unlock_time != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("unlockTime", ToString::to_string(&self.unlock_time).as_str())?;
        }
        if !self.vin.is_empty() {
            struct_ser.serialize_field("vin", &self.vin)?;
        }
        if !self.vout.is_empty() {
            struct_ser.serialize_field("vout", &self.vout)?;
        }
        if !self.extra.is_empty() {
            struct_ser.serialize_field("extra", &self.extra.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.rct_signatures.as_ref() {
            struct_ser.serialize_field("rctSignatures", v)?;
        }
        if let Some(v) = self.rctsig_prunable.as_ref() {
            struct_ser.serialize_field("rctsigPrunable", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for XmrMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "unlock_time",
            "unlockTime",
            "vin",
            "vout",
            "extra",
            "rct_signatures",
            "rctSignatures",
            "rctsig_prunable",
            "rctsigPrunable",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            UnlockTime,
            Vin,
            Vout,
            Extra,
            RctSignatures,
            RctsigPrunable,
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
                            "version" => Ok(GeneratedField::Version),
                            "unlockTime" | "unlock_time" => Ok(GeneratedField::UnlockTime),
                            "vin" => Ok(GeneratedField::Vin),
                            "vout" => Ok(GeneratedField::Vout),
                            "extra" => Ok(GeneratedField::Extra),
                            "rctSignatures" | "rct_signatures" => Ok(GeneratedField::RctSignatures),
                            "rctsigPrunable" | "rctsig_prunable" => Ok(GeneratedField::RctsigPrunable),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = XmrMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.transactions.v2.XmrMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<XmrMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut unlock_time__ = None;
                let mut vin__ = None;
                let mut vout__ = None;
                let mut extra__ = None;
                let mut rct_signatures__ = None;
                let mut rctsig_prunable__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UnlockTime => {
                            if unlock_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unlockTime"));
                            }
                            unlock_time__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Vin => {
                            if vin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vin"));
                            }
                            vin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Vout => {
                            if vout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vout"));
                            }
                            vout__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Extra => {
                            if extra__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extra"));
                            }
                            extra__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::RctSignatures => {
                            if rct_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rctSignatures"));
                            }
                            rct_signatures__ = map_.next_value()?;
                        }
                        GeneratedField::RctsigPrunable => {
                            if rctsig_prunable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rctsigPrunable"));
                            }
                            rctsig_prunable__ = map_.next_value()?;
                        }
                    }
                }
                Ok(XmrMetadata {
                    version: version__.unwrap_or_default(),
                    unlock_time: unlock_time__.unwrap_or_default(),
                    vin: vin__.unwrap_or_default(),
                    vout: vout__.unwrap_or_default(),
                    extra: extra__.unwrap_or_default(),
                    rct_signatures: rct_signatures__,
                    rctsig_prunable: rctsig_prunable__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.transactions.v2.XmrMetadata", FIELDS, GeneratedVisitor)
    }
}
