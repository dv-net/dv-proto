// @generated
impl serde::Serialize for SuggestGasPriceRequest {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.evm.v2.SuggestGasPriceRequest", len)?;
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SuggestGasPriceRequest {
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
            type Value = SuggestGasPriceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.evm.v2.SuggestGasPriceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SuggestGasPriceRequest, V::Error>
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
                Ok(SuggestGasPriceRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.evm.v2.SuggestGasPriceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SuggestGasPriceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.gas_fee_wei.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.evm.v2.SuggestGasPriceResponse", len)?;
        if !self.gas_fee_wei.is_empty() {
            struct_ser.serialize_field("gasFeeWei", &self.gas_fee_wei)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SuggestGasPriceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gas_fee_wei",
            "gasFeeWei",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GasFeeWei,
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
                            "gasFeeWei" | "gas_fee_wei" => Ok(GeneratedField::GasFeeWei),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SuggestGasPriceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.evm.v2.SuggestGasPriceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SuggestGasPriceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut gas_fee_wei__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GasFeeWei => {
                            if gas_fee_wei__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasFeeWei"));
                            }
                            gas_fee_wei__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SuggestGasPriceResponse {
                    gas_fee_wei: gas_fee_wei__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.evm.v2.SuggestGasPriceResponse", FIELDS, GeneratedVisitor)
    }
}
