// @generated
impl serde::Serialize for AdditionalInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tron_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.addresses.v1.AdditionalInfo", len)?;
        if let Some(v) = self.tron_info.as_ref() {
            struct_ser.serialize_field("tronInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AdditionalInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tron_info",
            "tronInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TronInfo,
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
                            "tronInfo" | "tron_info" => Ok(GeneratedField::TronInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AdditionalInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.addresses.v1.AdditionalInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AdditionalInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tron_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TronInfo => {
                            if tron_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tronInfo"));
                            }
                            tron_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AdditionalInfo {
                    tron_info: tron_info__,
                })
            }
        }
        deserializer.deserialize_struct("manager.addresses.v1.AdditionalInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Address {
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
        if self.status != 0 {
            len += 1;
        }
        if self.additional_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.addresses.v1.Address", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.status != 0 {
            let v = super::super::common::v1::AddressStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.additional_info.as_ref() {
            struct_ser.serialize_field("additionalInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Address {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "status",
            "additional_info",
            "additionalInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Status,
            AdditionalInfo,
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
                            "status" => Ok(GeneratedField::Status),
                            "additionalInfo" | "additional_info" => Ok(GeneratedField::AdditionalInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Address;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.addresses.v1.Address")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Address, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut status__ = None;
                let mut additional_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<super::super::common::v1::AddressStatus>()? as i32);
                        }
                        GeneratedField::AdditionalInfo => {
                            if additional_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalInfo"));
                            }
                            additional_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Address {
                    address: address__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    additional_info: additional_info__,
                })
            }
        }
        deserializer.deserialize_struct("manager.addresses.v1.Address", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAddressesListRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.include_additional_info {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.addresses.v1.GetAddressesListRequest", len)?;
        if self.include_additional_info {
            struct_ser.serialize_field("includeAdditionalInfo", &self.include_additional_info)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAddressesListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "include_additional_info",
            "includeAdditionalInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IncludeAdditionalInfo,
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
                            "includeAdditionalInfo" | "include_additional_info" => Ok(GeneratedField::IncludeAdditionalInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAddressesListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.addresses.v1.GetAddressesListRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAddressesListRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut include_additional_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IncludeAdditionalInfo => {
                            if include_additional_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeAdditionalInfo"));
                            }
                            include_additional_info__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetAddressesListRequest {
                    include_additional_info: include_additional_info__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("manager.addresses.v1.GetAddressesListRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAddressesListResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.addresses.v1.GetAddressesListResponse", len)?;
        if !self.addresses.is_empty() {
            struct_ser.serialize_field("addresses", &self.addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAddressesListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "addresses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Addresses,
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
                            "addresses" => Ok(GeneratedField::Addresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAddressesListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.addresses.v1.GetAddressesListResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAddressesListResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut addresses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Addresses => {
                            if addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addresses"));
                            }
                            addresses__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetAddressesListResponse {
                    addresses: addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("manager.addresses.v1.GetAddressesListResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TronInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.total_energy_limit.is_empty() {
            len += 1;
        }
        if !self.total_net_limit.is_empty() {
            len += 1;
        }
        if !self.max_available_energy.is_empty() {
            len += 1;
        }
        if !self.max_available_net.is_empty() {
            len += 1;
        }
        if !self.max_available_acquired_energy.is_empty() {
            len += 1;
        }
        if !self.max_available_acquired_net.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.addresses.v1.TronInfo", len)?;
        if !self.total_energy_limit.is_empty() {
            struct_ser.serialize_field("totalEnergyLimit", &self.total_energy_limit)?;
        }
        if !self.total_net_limit.is_empty() {
            struct_ser.serialize_field("totalNetLimit", &self.total_net_limit)?;
        }
        if !self.max_available_energy.is_empty() {
            struct_ser.serialize_field("maxAvailableEnergy", &self.max_available_energy)?;
        }
        if !self.max_available_net.is_empty() {
            struct_ser.serialize_field("maxAvailableNet", &self.max_available_net)?;
        }
        if !self.max_available_acquired_energy.is_empty() {
            struct_ser.serialize_field("maxAvailableAcquiredEnergy", &self.max_available_acquired_energy)?;
        }
        if !self.max_available_acquired_net.is_empty() {
            struct_ser.serialize_field("maxAvailableAcquiredNet", &self.max_available_acquired_net)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TronInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "total_energy_limit",
            "totalEnergyLimit",
            "total_net_limit",
            "totalNetLimit",
            "max_available_energy",
            "maxAvailableEnergy",
            "max_available_net",
            "maxAvailableNet",
            "max_available_acquired_energy",
            "maxAvailableAcquiredEnergy",
            "max_available_acquired_net",
            "maxAvailableAcquiredNet",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalEnergyLimit,
            TotalNetLimit,
            MaxAvailableEnergy,
            MaxAvailableNet,
            MaxAvailableAcquiredEnergy,
            MaxAvailableAcquiredNet,
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
                            "totalEnergyLimit" | "total_energy_limit" => Ok(GeneratedField::TotalEnergyLimit),
                            "totalNetLimit" | "total_net_limit" => Ok(GeneratedField::TotalNetLimit),
                            "maxAvailableEnergy" | "max_available_energy" => Ok(GeneratedField::MaxAvailableEnergy),
                            "maxAvailableNet" | "max_available_net" => Ok(GeneratedField::MaxAvailableNet),
                            "maxAvailableAcquiredEnergy" | "max_available_acquired_energy" => Ok(GeneratedField::MaxAvailableAcquiredEnergy),
                            "maxAvailableAcquiredNet" | "max_available_acquired_net" => Ok(GeneratedField::MaxAvailableAcquiredNet),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TronInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.addresses.v1.TronInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TronInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total_energy_limit__ = None;
                let mut total_net_limit__ = None;
                let mut max_available_energy__ = None;
                let mut max_available_net__ = None;
                let mut max_available_acquired_energy__ = None;
                let mut max_available_acquired_net__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TotalEnergyLimit => {
                            if total_energy_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalEnergyLimit"));
                            }
                            total_energy_limit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalNetLimit => {
                            if total_net_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalNetLimit"));
                            }
                            total_net_limit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxAvailableEnergy => {
                            if max_available_energy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAvailableEnergy"));
                            }
                            max_available_energy__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxAvailableNet => {
                            if max_available_net__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAvailableNet"));
                            }
                            max_available_net__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxAvailableAcquiredEnergy => {
                            if max_available_acquired_energy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAvailableAcquiredEnergy"));
                            }
                            max_available_acquired_energy__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxAvailableAcquiredNet => {
                            if max_available_acquired_net__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAvailableAcquiredNet"));
                            }
                            max_available_acquired_net__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TronInfo {
                    total_energy_limit: total_energy_limit__.unwrap_or_default(),
                    total_net_limit: total_net_limit__.unwrap_or_default(),
                    max_available_energy: max_available_energy__.unwrap_or_default(),
                    max_available_net: max_available_net__.unwrap_or_default(),
                    max_available_acquired_energy: max_available_acquired_energy__.unwrap_or_default(),
                    max_available_acquired_net: max_available_acquired_net__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("manager.addresses.v1.TronInfo", FIELDS, GeneratedVisitor)
    }
}
