// @generated
impl serde::Serialize for GetHealthStatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.service_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.health.v1.GetHealthStatusRequest", len)?;
        if self.service_type != 0 {
            let v = ServiceType::try_from(self.service_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.service_type)))?;
            struct_ser.serialize_field("serviceType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetHealthStatusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_type",
            "serviceType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceType,
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
                            "serviceType" | "service_type" => Ok(GeneratedField::ServiceType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetHealthStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.health.v1.GetHealthStatusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetHealthStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ServiceType => {
                            if service_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceType"));
                            }
                            service_type__ = Some(map_.next_value::<ServiceType>()? as i32);
                        }
                    }
                }
                Ok(GetHealthStatusRequest {
                    service_type: service_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("manager.health.v1.GetHealthStatusRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetHealthStatusResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.service_type != 0 {
            len += 1;
        }
        if self.health_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.health.v1.GetHealthStatusResponse", len)?;
        if self.service_type != 0 {
            let v = ServiceType::try_from(self.service_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.service_type)))?;
            struct_ser.serialize_field("serviceType", &v)?;
        }
        if self.health_status != 0 {
            let v = HealthStatus::try_from(self.health_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.health_status)))?;
            struct_ser.serialize_field("healthStatus", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetHealthStatusResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_type",
            "serviceType",
            "health_status",
            "healthStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceType,
            HealthStatus,
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
                            "serviceType" | "service_type" => Ok(GeneratedField::ServiceType),
                            "healthStatus" | "health_status" => Ok(GeneratedField::HealthStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetHealthStatusResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.health.v1.GetHealthStatusResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetHealthStatusResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_type__ = None;
                let mut health_status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ServiceType => {
                            if service_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceType"));
                            }
                            service_type__ = Some(map_.next_value::<ServiceType>()? as i32);
                        }
                        GeneratedField::HealthStatus => {
                            if health_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthStatus"));
                            }
                            health_status__ = Some(map_.next_value::<HealthStatus>()? as i32);
                        }
                    }
                }
                Ok(GetHealthStatusResponse {
                    service_type: service_type__.unwrap_or_default(),
                    health_status: health_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("manager.health.v1.GetHealthStatusResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "HEALTH_STATUS_UNSPECIFIED",
            Self::Serving => "HEALTH_STATUS_SERVING",
            Self::NotServing => "HEALTH_STATUS_NOT_SERVING",
            Self::Disabled => "HEALTH_STATUS_DISABLED",
            Self::Unknown => "HEALTH_STATUS_UNKNOWN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for HealthStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HEALTH_STATUS_UNSPECIFIED",
            "HEALTH_STATUS_SERVING",
            "HEALTH_STATUS_NOT_SERVING",
            "HEALTH_STATUS_DISABLED",
            "HEALTH_STATUS_UNKNOWN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthStatus;

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
                    "HEALTH_STATUS_UNSPECIFIED" => Ok(HealthStatus::Unspecified),
                    "HEALTH_STATUS_SERVING" => Ok(HealthStatus::Serving),
                    "HEALTH_STATUS_NOT_SERVING" => Ok(HealthStatus::NotServing),
                    "HEALTH_STATUS_DISABLED" => Ok(HealthStatus::Disabled),
                    "HEALTH_STATUS_UNKNOWN" => Ok(HealthStatus::Unknown),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SERVICE_TYPE_UNSPECIFIED",
            Self::Delegator => "SERVICE_TYPE_DELEGATOR",
            Self::Activator => "SERVICE_TYPE_ACTIVATOR",
            Self::Tron => "SERVICE_TYPE_TRON",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ServiceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SERVICE_TYPE_UNSPECIFIED",
            "SERVICE_TYPE_DELEGATOR",
            "SERVICE_TYPE_ACTIVATOR",
            "SERVICE_TYPE_TRON",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceType;

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
                    "SERVICE_TYPE_UNSPECIFIED" => Ok(ServiceType::Unspecified),
                    "SERVICE_TYPE_DELEGATOR" => Ok(ServiceType::Delegator),
                    "SERVICE_TYPE_ACTIVATOR" => Ok(ServiceType::Activator),
                    "SERVICE_TYPE_TRON" => Ok(ServiceType::Tron),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
