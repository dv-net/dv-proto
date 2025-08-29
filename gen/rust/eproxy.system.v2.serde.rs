// @generated
impl serde::Serialize for AppInfo {
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
        if !self.version.is_empty() {
            len += 1;
        }
        if !self.commit.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.system.v2.AppInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.commit.is_empty() {
            struct_ser.serialize_field("commit", &self.commit)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AppInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version",
            "commit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Version,
            Commit,
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
                            "version" => Ok(GeneratedField::Version),
                            "commit" => Ok(GeneratedField::Commit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AppInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.system.v2.AppInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AppInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version__ = None;
                let mut commit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Commit => {
                            if commit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commit"));
                            }
                            commit__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AppInfo {
                    name: name__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    commit: commit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.system.v2.AppInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlockchainInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.node_last_block_height != 0 {
            len += 1;
        }
        if self.db_last_block_height != 0 {
            len += 1;
        }
        if self.db_min_block_height != 0 {
            len += 1;
        }
        if self.node_diff != 0 {
            len += 1;
        }
        if self.total_parsed_blocks != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.system.v2.BlockchainInfo", len)?;
        if self.node_last_block_height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nodeLastBlockHeight", ToString::to_string(&self.node_last_block_height).as_str())?;
        }
        if self.db_last_block_height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("dbLastBlockHeight", ToString::to_string(&self.db_last_block_height).as_str())?;
        }
        if self.db_min_block_height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("dbMinBlockHeight", ToString::to_string(&self.db_min_block_height).as_str())?;
        }
        if self.node_diff != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nodeDiff", ToString::to_string(&self.node_diff).as_str())?;
        }
        if self.total_parsed_blocks != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("totalParsedBlocks", ToString::to_string(&self.total_parsed_blocks).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockchainInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node_last_block_height",
            "nodeLastBlockHeight",
            "db_last_block_height",
            "dbLastBlockHeight",
            "db_min_block_height",
            "dbMinBlockHeight",
            "node_diff",
            "nodeDiff",
            "total_parsed_blocks",
            "totalParsedBlocks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NodeLastBlockHeight,
            DbLastBlockHeight,
            DbMinBlockHeight,
            NodeDiff,
            TotalParsedBlocks,
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
                            "nodeLastBlockHeight" | "node_last_block_height" => Ok(GeneratedField::NodeLastBlockHeight),
                            "dbLastBlockHeight" | "db_last_block_height" => Ok(GeneratedField::DbLastBlockHeight),
                            "dbMinBlockHeight" | "db_min_block_height" => Ok(GeneratedField::DbMinBlockHeight),
                            "nodeDiff" | "node_diff" => Ok(GeneratedField::NodeDiff),
                            "totalParsedBlocks" | "total_parsed_blocks" => Ok(GeneratedField::TotalParsedBlocks),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockchainInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.system.v2.BlockchainInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlockchainInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node_last_block_height__ = None;
                let mut db_last_block_height__ = None;
                let mut db_min_block_height__ = None;
                let mut node_diff__ = None;
                let mut total_parsed_blocks__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NodeLastBlockHeight => {
                            if node_last_block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeLastBlockHeight"));
                            }
                            node_last_block_height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DbLastBlockHeight => {
                            if db_last_block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbLastBlockHeight"));
                            }
                            db_last_block_height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DbMinBlockHeight => {
                            if db_min_block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbMinBlockHeight"));
                            }
                            db_min_block_height__ = 
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
                        GeneratedField::TotalParsedBlocks => {
                            if total_parsed_blocks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalParsedBlocks"));
                            }
                            total_parsed_blocks__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BlockchainInfo {
                    node_last_block_height: node_last_block_height__.unwrap_or_default(),
                    db_last_block_height: db_last_block_height__.unwrap_or_default(),
                    db_min_block_height: db_min_block_height__.unwrap_or_default(),
                    node_diff: node_diff__.unwrap_or_default(),
                    total_parsed_blocks: total_parsed_blocks__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.system.v2.BlockchainInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DbInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.db_current_size != 0. {
            len += 1;
        }
        if !self.db_items_count.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.system.v2.DBInfo", len)?;
        if self.db_current_size != 0. {
            struct_ser.serialize_field("dbCurrentSize", &self.db_current_size)?;
        }
        if !self.db_items_count.is_empty() {
            let v: std::collections::HashMap<_, _> = self.db_items_count.iter()
                .map(|(k, v)| (k, v.to_string())).collect();
            struct_ser.serialize_field("dbItemsCount", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DbInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "db_current_size",
            "dbCurrentSize",
            "db_items_count",
            "dbItemsCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbCurrentSize,
            DbItemsCount,
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
                            "dbCurrentSize" | "db_current_size" => Ok(GeneratedField::DbCurrentSize),
                            "dbItemsCount" | "db_items_count" => Ok(GeneratedField::DbItemsCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DbInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.system.v2.DBInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DbInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_current_size__ = None;
                let mut db_items_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DbCurrentSize => {
                            if db_current_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbCurrentSize"));
                            }
                            db_current_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DbItemsCount => {
                            if db_items_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbItemsCount"));
                            }
                            db_items_count__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::NumberDeserialize<u64>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                    }
                }
                Ok(DbInfo {
                    db_current_size: db_current_size__.unwrap_or_default(),
                    db_items_count: db_items_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.system.v2.DBInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HostInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hostname.is_empty() {
            len += 1;
        }
        if !self.uptime.is_empty() {
            len += 1;
        }
        if !self.boot_time.is_empty() {
            len += 1;
        }
        if self.procs != 0 {
            len += 1;
        }
        if !self.os.is_empty() {
            len += 1;
        }
        if !self.platform.is_empty() {
            len += 1;
        }
        if !self.platform_family.is_empty() {
            len += 1;
        }
        if !self.platform_version.is_empty() {
            len += 1;
        }
        if !self.kernel_version.is_empty() {
            len += 1;
        }
        if !self.kernel_arch.is_empty() {
            len += 1;
        }
        if !self.virtualization_system.is_empty() {
            len += 1;
        }
        if !self.virtualization_role.is_empty() {
            len += 1;
        }
        if !self.host_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.system.v2.HostInfo", len)?;
        if !self.hostname.is_empty() {
            struct_ser.serialize_field("hostname", &self.hostname)?;
        }
        if !self.uptime.is_empty() {
            struct_ser.serialize_field("uptime", &self.uptime)?;
        }
        if !self.boot_time.is_empty() {
            struct_ser.serialize_field("bootTime", &self.boot_time)?;
        }
        if self.procs != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("procs", ToString::to_string(&self.procs).as_str())?;
        }
        if !self.os.is_empty() {
            struct_ser.serialize_field("os", &self.os)?;
        }
        if !self.platform.is_empty() {
            struct_ser.serialize_field("platform", &self.platform)?;
        }
        if !self.platform_family.is_empty() {
            struct_ser.serialize_field("platformFamily", &self.platform_family)?;
        }
        if !self.platform_version.is_empty() {
            struct_ser.serialize_field("platformVersion", &self.platform_version)?;
        }
        if !self.kernel_version.is_empty() {
            struct_ser.serialize_field("kernelVersion", &self.kernel_version)?;
        }
        if !self.kernel_arch.is_empty() {
            struct_ser.serialize_field("kernelArch", &self.kernel_arch)?;
        }
        if !self.virtualization_system.is_empty() {
            struct_ser.serialize_field("virtualizationSystem", &self.virtualization_system)?;
        }
        if !self.virtualization_role.is_empty() {
            struct_ser.serialize_field("virtualizationRole", &self.virtualization_role)?;
        }
        if !self.host_id.is_empty() {
            struct_ser.serialize_field("hostId", &self.host_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hostname",
            "uptime",
            "boot_time",
            "bootTime",
            "procs",
            "os",
            "platform",
            "platform_family",
            "platformFamily",
            "platform_version",
            "platformVersion",
            "kernel_version",
            "kernelVersion",
            "kernel_arch",
            "kernelArch",
            "virtualization_system",
            "virtualizationSystem",
            "virtualization_role",
            "virtualizationRole",
            "host_id",
            "hostId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hostname,
            Uptime,
            BootTime,
            Procs,
            Os,
            Platform,
            PlatformFamily,
            PlatformVersion,
            KernelVersion,
            KernelArch,
            VirtualizationSystem,
            VirtualizationRole,
            HostId,
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
                            "hostname" => Ok(GeneratedField::Hostname),
                            "uptime" => Ok(GeneratedField::Uptime),
                            "bootTime" | "boot_time" => Ok(GeneratedField::BootTime),
                            "procs" => Ok(GeneratedField::Procs),
                            "os" => Ok(GeneratedField::Os),
                            "platform" => Ok(GeneratedField::Platform),
                            "platformFamily" | "platform_family" => Ok(GeneratedField::PlatformFamily),
                            "platformVersion" | "platform_version" => Ok(GeneratedField::PlatformVersion),
                            "kernelVersion" | "kernel_version" => Ok(GeneratedField::KernelVersion),
                            "kernelArch" | "kernel_arch" => Ok(GeneratedField::KernelArch),
                            "virtualizationSystem" | "virtualization_system" => Ok(GeneratedField::VirtualizationSystem),
                            "virtualizationRole" | "virtualization_role" => Ok(GeneratedField::VirtualizationRole),
                            "hostId" | "host_id" => Ok(GeneratedField::HostId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.system.v2.HostInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hostname__ = None;
                let mut uptime__ = None;
                let mut boot_time__ = None;
                let mut procs__ = None;
                let mut os__ = None;
                let mut platform__ = None;
                let mut platform_family__ = None;
                let mut platform_version__ = None;
                let mut kernel_version__ = None;
                let mut kernel_arch__ = None;
                let mut virtualization_system__ = None;
                let mut virtualization_role__ = None;
                let mut host_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hostname => {
                            if hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            hostname__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Uptime => {
                            if uptime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uptime"));
                            }
                            uptime__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BootTime => {
                            if boot_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bootTime"));
                            }
                            boot_time__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Procs => {
                            if procs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("procs"));
                            }
                            procs__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Os => {
                            if os__.is_some() {
                                return Err(serde::de::Error::duplicate_field("os"));
                            }
                            os__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Platform => {
                            if platform__.is_some() {
                                return Err(serde::de::Error::duplicate_field("platform"));
                            }
                            platform__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PlatformFamily => {
                            if platform_family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("platformFamily"));
                            }
                            platform_family__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PlatformVersion => {
                            if platform_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("platformVersion"));
                            }
                            platform_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KernelVersion => {
                            if kernel_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kernelVersion"));
                            }
                            kernel_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KernelArch => {
                            if kernel_arch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kernelArch"));
                            }
                            kernel_arch__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VirtualizationSystem => {
                            if virtualization_system__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualizationSystem"));
                            }
                            virtualization_system__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VirtualizationRole => {
                            if virtualization_role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualizationRole"));
                            }
                            virtualization_role__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostId => {
                            if host_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostId"));
                            }
                            host_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HostInfo {
                    hostname: hostname__.unwrap_or_default(),
                    uptime: uptime__.unwrap_or_default(),
                    boot_time: boot_time__.unwrap_or_default(),
                    procs: procs__.unwrap_or_default(),
                    os: os__.unwrap_or_default(),
                    platform: platform__.unwrap_or_default(),
                    platform_family: platform_family__.unwrap_or_default(),
                    platform_version: platform_version__.unwrap_or_default(),
                    kernel_version: kernel_version__.unwrap_or_default(),
                    kernel_arch: kernel_arch__.unwrap_or_default(),
                    virtualization_system: virtualization_system__.unwrap_or_default(),
                    virtualization_role: virtualization_role__.unwrap_or_default(),
                    host_id: host_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.system.v2.HostInfo", FIELDS, GeneratedVisitor)
    }
}
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
        let mut struct_ser = serializer.serialize_struct("eproxy.system.v2.InfoRequest", len)?;
        if self.blockchain != 0 {
            let v = super::super::common::v2::Blockchain::try_from(self.blockchain)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.blockchain)))?;
            struct_ser.serialize_field("blockchain", &v)?;
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
            type Value = InfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.system.v2.InfoRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InfoRequest, V::Error>
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
                Ok(InfoRequest {
                    blockchain: blockchain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.system.v2.InfoRequest", FIELDS, GeneratedVisitor)
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
        if self.app_info.is_some() {
            len += 1;
        }
        if self.blockchain_info.is_some() {
            len += 1;
        }
        if self.db_info.is_some() {
            len += 1;
        }
        if self.system_info.is_some() {
            len += 1;
        }
        if self.host_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.system.v2.InfoResponse", len)?;
        if let Some(v) = self.app_info.as_ref() {
            struct_ser.serialize_field("appInfo", v)?;
        }
        if let Some(v) = self.blockchain_info.as_ref() {
            struct_ser.serialize_field("blockchainInfo", v)?;
        }
        if let Some(v) = self.db_info.as_ref() {
            struct_ser.serialize_field("dbInfo", v)?;
        }
        if let Some(v) = self.system_info.as_ref() {
            struct_ser.serialize_field("systemInfo", v)?;
        }
        if let Some(v) = self.host_info.as_ref() {
            struct_ser.serialize_field("hostInfo", v)?;
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
            "app_info",
            "appInfo",
            "blockchain_info",
            "blockchainInfo",
            "db_info",
            "dbInfo",
            "system_info",
            "systemInfo",
            "host_info",
            "hostInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AppInfo,
            BlockchainInfo,
            DbInfo,
            SystemInfo,
            HostInfo,
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
                            "appInfo" | "app_info" => Ok(GeneratedField::AppInfo),
                            "blockchainInfo" | "blockchain_info" => Ok(GeneratedField::BlockchainInfo),
                            "dbInfo" | "db_info" => Ok(GeneratedField::DbInfo),
                            "systemInfo" | "system_info" => Ok(GeneratedField::SystemInfo),
                            "hostInfo" | "host_info" => Ok(GeneratedField::HostInfo),
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
                formatter.write_str("struct eproxy.system.v2.InfoResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InfoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut app_info__ = None;
                let mut blockchain_info__ = None;
                let mut db_info__ = None;
                let mut system_info__ = None;
                let mut host_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AppInfo => {
                            if app_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appInfo"));
                            }
                            app_info__ = map_.next_value()?;
                        }
                        GeneratedField::BlockchainInfo => {
                            if blockchain_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockchainInfo"));
                            }
                            blockchain_info__ = map_.next_value()?;
                        }
                        GeneratedField::DbInfo => {
                            if db_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbInfo"));
                            }
                            db_info__ = map_.next_value()?;
                        }
                        GeneratedField::SystemInfo => {
                            if system_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemInfo"));
                            }
                            system_info__ = map_.next_value()?;
                        }
                        GeneratedField::HostInfo => {
                            if host_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostInfo"));
                            }
                            host_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(InfoResponse {
                    app_info: app_info__,
                    blockchain_info: blockchain_info__,
                    db_info: db_info__,
                    system_info: system_info__,
                    host_info: host_info__,
                })
            }
        }
        deserializer.deserialize_struct("eproxy.system.v2.InfoResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SystemInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total_disk_space != 0. {
            len += 1;
        }
        if self.free_disk_space != 0. {
            len += 1;
        }
        if self.total_memory != 0. {
            len += 1;
        }
        if self.free_memory != 0. {
            len += 1;
        }
        if self.service_memory_total_used != 0. {
            len += 1;
        }
        if self.service_memory_current != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("eproxy.system.v2.SystemInfo", len)?;
        if self.total_disk_space != 0. {
            struct_ser.serialize_field("totalDiskSpace", &self.total_disk_space)?;
        }
        if self.free_disk_space != 0. {
            struct_ser.serialize_field("freeDiskSpace", &self.free_disk_space)?;
        }
        if self.total_memory != 0. {
            struct_ser.serialize_field("totalMemory", &self.total_memory)?;
        }
        if self.free_memory != 0. {
            struct_ser.serialize_field("freeMemory", &self.free_memory)?;
        }
        if self.service_memory_total_used != 0. {
            struct_ser.serialize_field("serviceMemoryTotalUsed", &self.service_memory_total_used)?;
        }
        if self.service_memory_current != 0. {
            struct_ser.serialize_field("serviceMemoryCurrent", &self.service_memory_current)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SystemInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "total_disk_space",
            "totalDiskSpace",
            "free_disk_space",
            "freeDiskSpace",
            "total_memory",
            "totalMemory",
            "free_memory",
            "freeMemory",
            "service_memory_total_used",
            "serviceMemoryTotalUsed",
            "service_memory_current",
            "serviceMemoryCurrent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalDiskSpace,
            FreeDiskSpace,
            TotalMemory,
            FreeMemory,
            ServiceMemoryTotalUsed,
            ServiceMemoryCurrent,
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
                            "totalDiskSpace" | "total_disk_space" => Ok(GeneratedField::TotalDiskSpace),
                            "freeDiskSpace" | "free_disk_space" => Ok(GeneratedField::FreeDiskSpace),
                            "totalMemory" | "total_memory" => Ok(GeneratedField::TotalMemory),
                            "freeMemory" | "free_memory" => Ok(GeneratedField::FreeMemory),
                            "serviceMemoryTotalUsed" | "service_memory_total_used" => Ok(GeneratedField::ServiceMemoryTotalUsed),
                            "serviceMemoryCurrent" | "service_memory_current" => Ok(GeneratedField::ServiceMemoryCurrent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SystemInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct eproxy.system.v2.SystemInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SystemInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total_disk_space__ = None;
                let mut free_disk_space__ = None;
                let mut total_memory__ = None;
                let mut free_memory__ = None;
                let mut service_memory_total_used__ = None;
                let mut service_memory_current__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TotalDiskSpace => {
                            if total_disk_space__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalDiskSpace"));
                            }
                            total_disk_space__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FreeDiskSpace => {
                            if free_disk_space__.is_some() {
                                return Err(serde::de::Error::duplicate_field("freeDiskSpace"));
                            }
                            free_disk_space__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalMemory => {
                            if total_memory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalMemory"));
                            }
                            total_memory__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FreeMemory => {
                            if free_memory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("freeMemory"));
                            }
                            free_memory__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ServiceMemoryTotalUsed => {
                            if service_memory_total_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceMemoryTotalUsed"));
                            }
                            service_memory_total_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ServiceMemoryCurrent => {
                            if service_memory_current__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceMemoryCurrent"));
                            }
                            service_memory_current__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SystemInfo {
                    total_disk_space: total_disk_space__.unwrap_or_default(),
                    free_disk_space: free_disk_space__.unwrap_or_default(),
                    total_memory: total_memory__.unwrap_or_default(),
                    free_memory: free_memory__.unwrap_or_default(),
                    service_memory_total_used: service_memory_total_used__.unwrap_or_default(),
                    service_memory_current: service_memory_current__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("eproxy.system.v2.SystemInfo", FIELDS, GeneratedVisitor)
    }
}
