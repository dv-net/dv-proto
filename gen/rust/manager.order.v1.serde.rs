// @generated
impl serde::Serialize for BatchCreateOrderRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.orders.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.order.v1.BatchCreateOrderRequest", len)?;
        if !self.orders.is_empty() {
            struct_ser.serialize_field("orders", &self.orders)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreateOrderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "orders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Orders,
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
                            "orders" => Ok(GeneratedField::Orders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCreateOrderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.order.v1.BatchCreateOrderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreateOrderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut orders__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Orders => {
                            if orders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orders"));
                            }
                            orders__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchCreateOrderRequest {
                    orders: orders__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("manager.order.v1.BatchCreateOrderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCreateOrderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.orders.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.order.v1.BatchCreateOrderResponse", len)?;
        if !self.orders.is_empty() {
            struct_ser.serialize_field("orders", &self.orders)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreateOrderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "orders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Orders,
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
                            "orders" => Ok(GeneratedField::Orders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCreateOrderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.order.v1.BatchCreateOrderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreateOrderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut orders__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Orders => {
                            if orders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orders"));
                            }
                            orders__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchCreateOrderResponse {
                    orders: orders__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("manager.order.v1.BatchCreateOrderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelOrderRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.order.v1.CancelOrderRequest", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelOrderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelOrderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.order.v1.CancelOrderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelOrderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CancelOrderRequest {
                    order_id: order_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("manager.order.v1.CancelOrderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelOrderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.order.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.order.v1.CancelOrderResponse", len)?;
        if let Some(v) = self.order.as_ref() {
            struct_ser.serialize_field("order", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelOrderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Order,
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
                            "order" => Ok(GeneratedField::Order),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelOrderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.order.v1.CancelOrderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelOrderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Order => {
                            if order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CancelOrderResponse {
                    order: order__,
                })
            }
        }
        deserializer.deserialize_struct("manager.order.v1.CancelOrderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateOrderRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.order_type != 0 {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if self.duration.is_some() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.order.v1.CreateOrderRequest", len)?;
        if self.order_type != 0 {
            let v = super::super::common::v1::OrderType::try_from(self.order_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.order_type)))?;
            struct_ser.serialize_field("orderType", &v)?;
        }
        if let Some(v) = self.amount.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("amount", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.duration.as_ref() {
            struct_ser.serialize_field("duration", v)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateOrderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_type",
            "orderType",
            "amount",
            "duration",
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderType,
            Amount,
            Duration,
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
                            "orderType" | "order_type" => Ok(GeneratedField::OrderType),
                            "amount" => Ok(GeneratedField::Amount),
                            "duration" => Ok(GeneratedField::Duration),
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
            type Value = CreateOrderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.order.v1.CreateOrderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateOrderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_type__ = None;
                let mut amount__ = None;
                let mut duration__ = None;
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderType => {
                            if order_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderType"));
                            }
                            order_type__ = Some(map_.next_value::<super::super::common::v1::OrderType>()? as i32);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = map_.next_value()?;
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateOrderRequest {
                    order_type: order_type__.unwrap_or_default(),
                    amount: amount__,
                    duration: duration__,
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("manager.order.v1.CreateOrderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateOrderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.order.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.order.v1.CreateOrderResponse", len)?;
        if let Some(v) = self.order.as_ref() {
            struct_ser.serialize_field("order", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateOrderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Order,
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
                            "order" => Ok(GeneratedField::Order),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateOrderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.order.v1.CreateOrderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateOrderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Order => {
                            if order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateOrderResponse {
                    order: order__,
                })
            }
        }
        deserializer.deserialize_struct("manager.order.v1.CreateOrderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOrderRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.order.v1.GetOrderRequest", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOrderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOrderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.order.v1.GetOrderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOrderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetOrderRequest {
                    order_id: order_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("manager.order.v1.GetOrderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOrderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.order.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.order.v1.GetOrderResponse", len)?;
        if let Some(v) = self.order.as_ref() {
            struct_ser.serialize_field("order", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOrderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Order,
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
                            "order" => Ok(GeneratedField::Order),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOrderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.order.v1.GetOrderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOrderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Order => {
                            if order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetOrderResponse {
                    order: order__,
                })
            }
        }
        deserializer.deserialize_struct("manager.order.v1.GetOrderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOrdersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.order_type.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.executor_address.is_some() {
            len += 1;
        }
        if self.target_address.is_some() {
            len += 1;
        }
        if self.page.is_some() {
            len += 1;
        }
        if self.page_size.is_some() {
            len += 1;
        }
        if !self.order_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.order.v1.ListOrdersRequest", len)?;
        if let Some(v) = self.order_type.as_ref() {
            let v = super::super::common::v1::OrderType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("orderType", &v)?;
        }
        if let Some(v) = self.status.as_ref() {
            let v = super::super::common::v1::OrderStatus::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.executor_address.as_ref() {
            struct_ser.serialize_field("executorAddress", v)?;
        }
        if let Some(v) = self.target_address.as_ref() {
            struct_ser.serialize_field("targetAddress", v)?;
        }
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        if let Some(v) = self.page_size.as_ref() {
            struct_ser.serialize_field("pageSize", v)?;
        }
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOrdersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_type",
            "orderType",
            "status",
            "executor_address",
            "executorAddress",
            "target_address",
            "targetAddress",
            "page",
            "page_size",
            "pageSize",
            "order_id",
            "orderId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderType,
            Status,
            ExecutorAddress,
            TargetAddress,
            Page,
            PageSize,
            OrderId,
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
                            "orderType" | "order_type" => Ok(GeneratedField::OrderType),
                            "status" => Ok(GeneratedField::Status),
                            "executorAddress" | "executor_address" => Ok(GeneratedField::ExecutorAddress),
                            "targetAddress" | "target_address" => Ok(GeneratedField::TargetAddress),
                            "page" => Ok(GeneratedField::Page),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListOrdersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.order.v1.ListOrdersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOrdersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_type__ = None;
                let mut status__ = None;
                let mut executor_address__ = None;
                let mut target_address__ = None;
                let mut page__ = None;
                let mut page_size__ = None;
                let mut order_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderType => {
                            if order_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderType"));
                            }
                            order_type__ = map_.next_value::<::std::option::Option<super::super::common::v1::OrderType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value::<::std::option::Option<super::super::common::v1::OrderStatus>>()?.map(|x| x as i32);
                        }
                        GeneratedField::ExecutorAddress => {
                            if executor_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorAddress"));
                            }
                            executor_address__ = map_.next_value()?;
                        }
                        GeneratedField::TargetAddress => {
                            if target_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetAddress"));
                            }
                            target_address__ = map_.next_value()?;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListOrdersRequest {
                    order_type: order_type__,
                    status: status__,
                    executor_address: executor_address__,
                    target_address: target_address__,
                    page: page__,
                    page_size: page_size__,
                    order_id: order_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("manager.order.v1.ListOrdersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOrdersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.orders.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.order.v1.ListOrdersResponse", len)?;
        if !self.orders.is_empty() {
            struct_ser.serialize_field("orders", &self.orders)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOrdersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "orders",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Orders,
            Pagination,
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
                            "orders" => Ok(GeneratedField::Orders),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListOrdersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.order.v1.ListOrdersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListOrdersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut orders__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Orders => {
                            if orders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orders"));
                            }
                            orders__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListOrdersResponse {
                    orders: orders__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("manager.order.v1.ListOrdersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Order {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.order_id.is_empty() {
            len += 1;
        }
        if self.order_type != 0 {
            len += 1;
        }
        if self.order_status != 0 {
            len += 1;
        }
        if !self.executor_address.is_empty() {
            len += 1;
        }
        if !self.target_address.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if self.duration.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if !self.reservations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.order.v1.Order", len)?;
        if !self.order_id.is_empty() {
            struct_ser.serialize_field("orderId", &self.order_id)?;
        }
        if self.order_type != 0 {
            let v = super::super::common::v1::OrderType::try_from(self.order_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.order_type)))?;
            struct_ser.serialize_field("orderType", &v)?;
        }
        if self.order_status != 0 {
            let v = super::super::common::v1::OrderStatus::try_from(self.order_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.order_status)))?;
            struct_ser.serialize_field("orderStatus", &v)?;
        }
        if !self.executor_address.is_empty() {
            struct_ser.serialize_field("executorAddress", &self.executor_address)?;
        }
        if !self.target_address.is_empty() {
            struct_ser.serialize_field("targetAddress", &self.target_address)?;
        }
        if let Some(v) = self.amount.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("amount", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.duration.as_ref() {
            struct_ser.serialize_field("duration", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if !self.reservations.is_empty() {
            struct_ser.serialize_field("reservations", &self.reservations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Order {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "order_id",
            "orderId",
            "order_type",
            "orderType",
            "order_status",
            "orderStatus",
            "executor_address",
            "executorAddress",
            "target_address",
            "targetAddress",
            "amount",
            "duration",
            "created_at",
            "createdAt",
            "reservations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrderId,
            OrderType,
            OrderStatus,
            ExecutorAddress,
            TargetAddress,
            Amount,
            Duration,
            CreatedAt,
            Reservations,
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
                            "orderId" | "order_id" => Ok(GeneratedField::OrderId),
                            "orderType" | "order_type" => Ok(GeneratedField::OrderType),
                            "orderStatus" | "order_status" => Ok(GeneratedField::OrderStatus),
                            "executorAddress" | "executor_address" => Ok(GeneratedField::ExecutorAddress),
                            "targetAddress" | "target_address" => Ok(GeneratedField::TargetAddress),
                            "amount" => Ok(GeneratedField::Amount),
                            "duration" => Ok(GeneratedField::Duration),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "reservations" => Ok(GeneratedField::Reservations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Order;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.order.v1.Order")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Order, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut order_id__ = None;
                let mut order_type__ = None;
                let mut order_status__ = None;
                let mut executor_address__ = None;
                let mut target_address__ = None;
                let mut amount__ = None;
                let mut duration__ = None;
                let mut created_at__ = None;
                let mut reservations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrderId => {
                            if order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderId"));
                            }
                            order_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderType => {
                            if order_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderType"));
                            }
                            order_type__ = Some(map_.next_value::<super::super::common::v1::OrderType>()? as i32);
                        }
                        GeneratedField::OrderStatus => {
                            if order_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderStatus"));
                            }
                            order_status__ = Some(map_.next_value::<super::super::common::v1::OrderStatus>()? as i32);
                        }
                        GeneratedField::ExecutorAddress => {
                            if executor_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorAddress"));
                            }
                            executor_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetAddress => {
                            if target_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetAddress"));
                            }
                            target_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::Reservations => {
                            if reservations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservations"));
                            }
                            reservations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Order {
                    order_id: order_id__.unwrap_or_default(),
                    order_type: order_type__.unwrap_or_default(),
                    order_status: order_status__.unwrap_or_default(),
                    executor_address: executor_address__.unwrap_or_default(),
                    target_address: target_address__.unwrap_or_default(),
                    amount: amount__,
                    duration: duration__,
                    created_at: created_at__,
                    reservations: reservations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("manager.order.v1.Order", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrderReservation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reservation_amount != 0 {
            len += 1;
        }
        if self.resource_type != 0 {
            len += 1;
        }
        if self.resource_kind != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("manager.order.v1.OrderReservation", len)?;
        if self.reservation_amount != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("reservationAmount", ToString::to_string(&self.reservation_amount).as_str())?;
        }
        if self.resource_type != 0 {
            let v = super::super::common::v1::ResourceType::try_from(self.resource_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.resource_type)))?;
            struct_ser.serialize_field("resourceType", &v)?;
        }
        if self.resource_kind != 0 {
            let v = super::super::common::v1::ResourceKind::try_from(self.resource_kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.resource_kind)))?;
            struct_ser.serialize_field("resourceKind", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderReservation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reservation_amount",
            "reservationAmount",
            "resource_type",
            "resourceType",
            "resource_kind",
            "resourceKind",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReservationAmount,
            ResourceType,
            ResourceKind,
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
                            "reservationAmount" | "reservation_amount" => Ok(GeneratedField::ReservationAmount),
                            "resourceType" | "resource_type" => Ok(GeneratedField::ResourceType),
                            "resourceKind" | "resource_kind" => Ok(GeneratedField::ResourceKind),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrderReservation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct manager.order.v1.OrderReservation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrderReservation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reservation_amount__ = None;
                let mut resource_type__ = None;
                let mut resource_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReservationAmount => {
                            if reservation_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservationAmount"));
                            }
                            reservation_amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceType"));
                            }
                            resource_type__ = Some(map_.next_value::<super::super::common::v1::ResourceType>()? as i32);
                        }
                        GeneratedField::ResourceKind => {
                            if resource_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceKind"));
                            }
                            resource_kind__ = Some(map_.next_value::<super::super::common::v1::ResourceKind>()? as i32);
                        }
                    }
                }
                Ok(OrderReservation {
                    reservation_amount: reservation_amount__.unwrap_or_default(),
                    resource_type: resource_type__.unwrap_or_default(),
                    resource_kind: resource_kind__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("manager.order.v1.OrderReservation", FIELDS, GeneratedVisitor)
    }
}
