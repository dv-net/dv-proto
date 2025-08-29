// @generated
impl serde::Serialize for GetResourcesRequest {
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
        let mut struct_ser = serializer.serialize_struct("eproxy.tron.v1.GetResourcesRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourcesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResourcesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.tron.v1.GetResourcesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResourcesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetResourcesRequest {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.tron.v1.GetResourcesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourcesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.free_net_used != 0 {
            len += 1;
        }
        if self.free_net_limit != 0 {
            len += 1;
        }
        if self.net_used != 0 {
            len += 1;
        }
        if self.net_limit != 0 {
            len += 1;
        }
        if !self.asset_net_used.is_empty() {
            len += 1;
        }
        if !self.asset_net_limit.is_empty() {
            len += 1;
        }
        if self.total_net_limit != 0 {
            len += 1;
        }
        if self.total_net_weight != 0 {
            len += 1;
        }
        if self.total_tron_power_weight != 0 {
            len += 1;
        }
        if self.tron_power_used != 0 {
            len += 1;
        }
        if self.tron_power_limit != 0 {
            len += 1;
        }
        if self.energy_used != 0 {
            len += 1;
        }
        if self.energy_limit != 0 {
            len += 1;
        }
        if self.total_energy_limit != 0 {
            len += 1;
        }
        if self.total_energy_weight != 0 {
            len += 1;
        }
        if self.storage_used != 0 {
            len += 1;
        }
        if self.storage_limit != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.tron.v1.GetResourcesResponse", len)?;
        if self.free_net_used != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("freeNetUsed", ToString::to_string(&self.free_net_used).as_str())?;
        }
        if self.free_net_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("freeNetLimit", ToString::to_string(&self.free_net_limit).as_str())?;
        }
        if self.net_used != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("netUsed", ToString::to_string(&self.net_used).as_str())?;
        }
        if self.net_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("netLimit", ToString::to_string(&self.net_limit).as_str())?;
        }
        if !self.asset_net_used.is_empty() {
            let v: std::collections::HashMap<_, _> = self.asset_net_used.iter()
                .map(|(k, v)| (k, v.to_string())).collect();
            struct_ser.serialize_field("assetNetUsed", &v)?;
        }
        if !self.asset_net_limit.is_empty() {
            let v: std::collections::HashMap<_, _> = self.asset_net_limit.iter()
                .map(|(k, v)| (k, v.to_string())).collect();
            struct_ser.serialize_field("assetNetLimit", &v)?;
        }
        if self.total_net_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("totalNetLimit", ToString::to_string(&self.total_net_limit).as_str())?;
        }
        if self.total_net_weight != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("totalNetWeight", ToString::to_string(&self.total_net_weight).as_str())?;
        }
        if self.total_tron_power_weight != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("totalTronPowerWeight", ToString::to_string(&self.total_tron_power_weight).as_str())?;
        }
        if self.tron_power_used != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("tronPowerUsed", ToString::to_string(&self.tron_power_used).as_str())?;
        }
        if self.tron_power_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("tronPowerLimit", ToString::to_string(&self.tron_power_limit).as_str())?;
        }
        if self.energy_used != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("energyUsed", ToString::to_string(&self.energy_used).as_str())?;
        }
        if self.energy_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("energyLimit", ToString::to_string(&self.energy_limit).as_str())?;
        }
        if self.total_energy_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("totalEnergyLimit", ToString::to_string(&self.total_energy_limit).as_str())?;
        }
        if self.total_energy_weight != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("totalEnergyWeight", ToString::to_string(&self.total_energy_weight).as_str())?;
        }
        if self.storage_used != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("storageUsed", ToString::to_string(&self.storage_used).as_str())?;
        }
        if self.storage_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("storageLimit", ToString::to_string(&self.storage_limit).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourcesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "free_net_used",
            "freeNetUsed",
            "free_net_limit",
            "freeNetLimit",
            "net_used",
            "netUsed",
            "net_limit",
            "netLimit",
            "asset_net_used",
            "assetNetUsed",
            "asset_net_limit",
            "assetNetLimit",
            "total_net_limit",
            "totalNetLimit",
            "total_net_weight",
            "totalNetWeight",
            "total_tron_power_weight",
            "totalTronPowerWeight",
            "tron_power_used",
            "tronPowerUsed",
            "tron_power_limit",
            "tronPowerLimit",
            "energy_used",
            "energyUsed",
            "energy_limit",
            "energyLimit",
            "total_energy_limit",
            "totalEnergyLimit",
            "total_energy_weight",
            "totalEnergyWeight",
            "storage_used",
            "storageUsed",
            "storage_limit",
            "storageLimit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FreeNetUsed,
            FreeNetLimit,
            NetUsed,
            NetLimit,
            AssetNetUsed,
            AssetNetLimit,
            TotalNetLimit,
            TotalNetWeight,
            TotalTronPowerWeight,
            TronPowerUsed,
            TronPowerLimit,
            EnergyUsed,
            EnergyLimit,
            TotalEnergyLimit,
            TotalEnergyWeight,
            StorageUsed,
            StorageLimit,
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
                            "freeNetUsed" | "free_net_used" => Ok(GeneratedField::FreeNetUsed),
                            "freeNetLimit" | "free_net_limit" => Ok(GeneratedField::FreeNetLimit),
                            "netUsed" | "net_used" => Ok(GeneratedField::NetUsed),
                            "netLimit" | "net_limit" => Ok(GeneratedField::NetLimit),
                            "assetNetUsed" | "asset_net_used" => Ok(GeneratedField::AssetNetUsed),
                            "assetNetLimit" | "asset_net_limit" => Ok(GeneratedField::AssetNetLimit),
                            "totalNetLimit" | "total_net_limit" => Ok(GeneratedField::TotalNetLimit),
                            "totalNetWeight" | "total_net_weight" => Ok(GeneratedField::TotalNetWeight),
                            "totalTronPowerWeight" | "total_tron_power_weight" => Ok(GeneratedField::TotalTronPowerWeight),
                            "tronPowerUsed" | "tron_power_used" => Ok(GeneratedField::TronPowerUsed),
                            "tronPowerLimit" | "tron_power_limit" => Ok(GeneratedField::TronPowerLimit),
                            "energyUsed" | "energy_used" => Ok(GeneratedField::EnergyUsed),
                            "energyLimit" | "energy_limit" => Ok(GeneratedField::EnergyLimit),
                            "totalEnergyLimit" | "total_energy_limit" => Ok(GeneratedField::TotalEnergyLimit),
                            "totalEnergyWeight" | "total_energy_weight" => Ok(GeneratedField::TotalEnergyWeight),
                            "storageUsed" | "storage_used" => Ok(GeneratedField::StorageUsed),
                            "storageLimit" | "storage_limit" => Ok(GeneratedField::StorageLimit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResourcesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.tron.v1.GetResourcesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResourcesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut free_net_used__ = None;
                let mut free_net_limit__ = None;
                let mut net_used__ = None;
                let mut net_limit__ = None;
                let mut asset_net_used__ = None;
                let mut asset_net_limit__ = None;
                let mut total_net_limit__ = None;
                let mut total_net_weight__ = None;
                let mut total_tron_power_weight__ = None;
                let mut tron_power_used__ = None;
                let mut tron_power_limit__ = None;
                let mut energy_used__ = None;
                let mut energy_limit__ = None;
                let mut total_energy_limit__ = None;
                let mut total_energy_weight__ = None;
                let mut storage_used__ = None;
                let mut storage_limit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FreeNetUsed => {
                            if free_net_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("freeNetUsed"));
                            }
                            free_net_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FreeNetLimit => {
                            if free_net_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("freeNetLimit"));
                            }
                            free_net_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NetUsed => {
                            if net_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("netUsed"));
                            }
                            net_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NetLimit => {
                            if net_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("netLimit"));
                            }
                            net_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AssetNetUsed => {
                            if asset_net_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetNetUsed"));
                            }
                            asset_net_used__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::NumberDeserialize<i64>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                        GeneratedField::AssetNetLimit => {
                            if asset_net_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetNetLimit"));
                            }
                            asset_net_limit__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::NumberDeserialize<i64>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                        GeneratedField::TotalNetLimit => {
                            if total_net_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalNetLimit"));
                            }
                            total_net_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalNetWeight => {
                            if total_net_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalNetWeight"));
                            }
                            total_net_weight__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalTronPowerWeight => {
                            if total_tron_power_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalTronPowerWeight"));
                            }
                            total_tron_power_weight__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TronPowerUsed => {
                            if tron_power_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tronPowerUsed"));
                            }
                            tron_power_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TronPowerLimit => {
                            if tron_power_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tronPowerLimit"));
                            }
                            tron_power_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EnergyUsed => {
                            if energy_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("energyUsed"));
                            }
                            energy_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EnergyLimit => {
                            if energy_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("energyLimit"));
                            }
                            energy_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalEnergyLimit => {
                            if total_energy_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalEnergyLimit"));
                            }
                            total_energy_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalEnergyWeight => {
                            if total_energy_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalEnergyWeight"));
                            }
                            total_energy_weight__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StorageUsed => {
                            if storage_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageUsed"));
                            }
                            storage_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StorageLimit => {
                            if storage_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageLimit"));
                            }
                            storage_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetResourcesResponse {
                    free_net_used: free_net_used__.unwrap_or_default(),
                    free_net_limit: free_net_limit__.unwrap_or_default(),
                    net_used: net_used__.unwrap_or_default(),
                    net_limit: net_limit__.unwrap_or_default(),
                    asset_net_used: asset_net_used__.unwrap_or_default(),
                    asset_net_limit: asset_net_limit__.unwrap_or_default(),
                    total_net_limit: total_net_limit__.unwrap_or_default(),
                    total_net_weight: total_net_weight__.unwrap_or_default(),
                    total_tron_power_weight: total_tron_power_weight__.unwrap_or_default(),
                    tron_power_used: tron_power_used__.unwrap_or_default(),
                    tron_power_limit: tron_power_limit__.unwrap_or_default(),
                    energy_used: energy_used__.unwrap_or_default(),
                    energy_limit: energy_limit__.unwrap_or_default(),
                    total_energy_limit: total_energy_limit__.unwrap_or_default(),
                    total_energy_weight: total_energy_weight__.unwrap_or_default(),
                    storage_used: storage_used__.unwrap_or_default(),
                    storage_limit: storage_limit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.tron.v1.GetResourcesResponse", FIELDS, GeneratedVisitor)
    }
}
