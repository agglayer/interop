// @generated
impl serde::Serialize for AggchainData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.AggchainData", len)?;
        if let Some(v) = self.data.as_ref() {
            match v {
                aggchain_data::Data::Signature(v) => {
                    struct_ser.serialize_field("signature", v)?;
                }
                aggchain_data::Data::Generic(v) => {
                    struct_ser.serialize_field("generic", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AggchainData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signature",
            "generic",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signature,
            Generic,
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
                            "signature" => Ok(GeneratedField::Signature),
                            "generic" => Ok(GeneratedField::Generic),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AggchainData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.AggchainData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AggchainData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signature => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(aggchain_data::Data::Signature)
;
                        }
                        GeneratedField::Generic => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("generic"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(aggchain_data::Data::Generic)
;
                        }
                    }
                }
                Ok(AggchainData {
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.AggchainData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AggchainProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.aggchain_params.is_some() {
            len += 1;
        }
        if self.signature.is_some() {
            len += 1;
        }
        if !self.context.is_empty() {
            len += 1;
        }
        if self.proof.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.AggchainProof", len)?;
        if let Some(v) = self.aggchain_params.as_ref() {
            struct_ser.serialize_field("aggchainParams", v)?;
        }
        if let Some(v) = self.signature.as_ref() {
            struct_ser.serialize_field("signature", v)?;
        }
        if !self.context.is_empty() {
            let v: std::collections::HashMap<_, _> = self.context.iter()
                .map(|(k, v)| (k, pbjson::private::base64::encode(v))).collect();
            struct_ser.serialize_field("context", &v)?;
        }
        if let Some(v) = self.proof.as_ref() {
            match v {
                aggchain_proof::Proof::Sp1Stark(v) => {
                    struct_ser.serialize_field("sp1Stark", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AggchainProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "aggchain_params",
            "aggchainParams",
            "signature",
            "context",
            "sp1_stark",
            "sp1Stark",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AggchainParams,
            Signature,
            Context,
            Sp1Stark,
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
                            "aggchainParams" | "aggchain_params" => Ok(GeneratedField::AggchainParams),
                            "signature" => Ok(GeneratedField::Signature),
                            "context" => Ok(GeneratedField::Context),
                            "sp1Stark" | "sp1_stark" => Ok(GeneratedField::Sp1Stark),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AggchainProof;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.AggchainProof")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AggchainProof, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut aggchain_params__ = None;
                let mut signature__ = None;
                let mut context__ = None;
                let mut proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AggchainParams => {
                            if aggchain_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggchainParams"));
                            }
                            aggchain_params__ = map_.next_value()?;
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = map_.next_value()?;
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                        GeneratedField::Sp1Stark => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sp1Stark"));
                            }
                            proof__ = map_.next_value::<::std::option::Option<_>>()?.map(aggchain_proof::Proof::Sp1Stark)
;
                        }
                    }
                }
                Ok(AggchainProof {
                    aggchain_params: aggchain_params__,
                    signature: signature__,
                    context: context__.unwrap_or_default(),
                    proof: proof__,
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.AggchainProof", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BridgeExit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.leaf_type != 0 {
            len += 1;
        }
        if self.token_info.is_some() {
            len += 1;
        }
        if self.dest_network != 0 {
            len += 1;
        }
        if self.dest_address.is_some() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.BridgeExit", len)?;
        if self.leaf_type != 0 {
            let v = LeafType::try_from(self.leaf_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.leaf_type)))?;
            struct_ser.serialize_field("leafType", &v)?;
        }
        if let Some(v) = self.token_info.as_ref() {
            struct_ser.serialize_field("tokenInfo", v)?;
        }
        if self.dest_network != 0 {
            struct_ser.serialize_field("destNetwork", &self.dest_network)?;
        }
        if let Some(v) = self.dest_address.as_ref() {
            struct_ser.serialize_field("destAddress", v)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BridgeExit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "leaf_type",
            "leafType",
            "token_info",
            "tokenInfo",
            "dest_network",
            "destNetwork",
            "dest_address",
            "destAddress",
            "amount",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LeafType,
            TokenInfo,
            DestNetwork,
            DestAddress,
            Amount,
            Metadata,
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
                            "leafType" | "leaf_type" => Ok(GeneratedField::LeafType),
                            "tokenInfo" | "token_info" => Ok(GeneratedField::TokenInfo),
                            "destNetwork" | "dest_network" => Ok(GeneratedField::DestNetwork),
                            "destAddress" | "dest_address" => Ok(GeneratedField::DestAddress),
                            "amount" => Ok(GeneratedField::Amount),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BridgeExit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.BridgeExit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BridgeExit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut leaf_type__ = None;
                let mut token_info__ = None;
                let mut dest_network__ = None;
                let mut dest_address__ = None;
                let mut amount__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LeafType => {
                            if leaf_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leafType"));
                            }
                            leaf_type__ = Some(map_.next_value::<LeafType>()? as i32);
                        }
                        GeneratedField::TokenInfo => {
                            if token_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenInfo"));
                            }
                            token_info__ = map_.next_value()?;
                        }
                        GeneratedField::DestNetwork => {
                            if dest_network__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destNetwork"));
                            }
                            dest_network__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DestAddress => {
                            if dest_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destAddress"));
                            }
                            dest_address__ = map_.next_value()?;
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BridgeExit {
                    leaf_type: leaf_type__.unwrap_or_default(),
                    token_info: token_info__,
                    dest_network: dest_network__.unwrap_or_default(),
                    dest_address: dest_address__,
                    amount: amount__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.BridgeExit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClaimFromMainnet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.proof_leaf_mer.is_some() {
            len += 1;
        }
        if self.proof_ger_l1root.is_some() {
            len += 1;
        }
        if self.l1_leaf.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.ClaimFromMainnet", len)?;
        if let Some(v) = self.proof_leaf_mer.as_ref() {
            struct_ser.serialize_field("proofLeafMer", v)?;
        }
        if let Some(v) = self.proof_ger_l1root.as_ref() {
            struct_ser.serialize_field("proofGerL1root", v)?;
        }
        if let Some(v) = self.l1_leaf.as_ref() {
            struct_ser.serialize_field("l1Leaf", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClaimFromMainnet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "proof_leaf_mer",
            "proofLeafMer",
            "proof_ger_l1root",
            "proofGerL1root",
            "l1_leaf",
            "l1Leaf",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProofLeafMer,
            ProofGerL1root,
            L1Leaf,
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
                            "proofLeafMer" | "proof_leaf_mer" => Ok(GeneratedField::ProofLeafMer),
                            "proofGerL1root" | "proof_ger_l1root" => Ok(GeneratedField::ProofGerL1root),
                            "l1Leaf" | "l1_leaf" => Ok(GeneratedField::L1Leaf),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClaimFromMainnet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.ClaimFromMainnet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClaimFromMainnet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut proof_leaf_mer__ = None;
                let mut proof_ger_l1root__ = None;
                let mut l1_leaf__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProofLeafMer => {
                            if proof_leaf_mer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proofLeafMer"));
                            }
                            proof_leaf_mer__ = map_.next_value()?;
                        }
                        GeneratedField::ProofGerL1root => {
                            if proof_ger_l1root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proofGerL1root"));
                            }
                            proof_ger_l1root__ = map_.next_value()?;
                        }
                        GeneratedField::L1Leaf => {
                            if l1_leaf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("l1Leaf"));
                            }
                            l1_leaf__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ClaimFromMainnet {
                    proof_leaf_mer: proof_leaf_mer__,
                    proof_ger_l1root: proof_ger_l1root__,
                    l1_leaf: l1_leaf__,
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.ClaimFromMainnet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClaimFromRollup {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.proof_leaf_ler.is_some() {
            len += 1;
        }
        if self.proof_ler_rer.is_some() {
            len += 1;
        }
        if self.proof_ger_l1root.is_some() {
            len += 1;
        }
        if self.l1_leaf.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.ClaimFromRollup", len)?;
        if let Some(v) = self.proof_leaf_ler.as_ref() {
            struct_ser.serialize_field("proofLeafLer", v)?;
        }
        if let Some(v) = self.proof_ler_rer.as_ref() {
            struct_ser.serialize_field("proofLerRer", v)?;
        }
        if let Some(v) = self.proof_ger_l1root.as_ref() {
            struct_ser.serialize_field("proofGerL1root", v)?;
        }
        if let Some(v) = self.l1_leaf.as_ref() {
            struct_ser.serialize_field("l1Leaf", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClaimFromRollup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "proof_leaf_ler",
            "proofLeafLer",
            "proof_ler_rer",
            "proofLerRer",
            "proof_ger_l1root",
            "proofGerL1root",
            "l1_leaf",
            "l1Leaf",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProofLeafLer,
            ProofLerRer,
            ProofGerL1root,
            L1Leaf,
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
                            "proofLeafLer" | "proof_leaf_ler" => Ok(GeneratedField::ProofLeafLer),
                            "proofLerRer" | "proof_ler_rer" => Ok(GeneratedField::ProofLerRer),
                            "proofGerL1root" | "proof_ger_l1root" => Ok(GeneratedField::ProofGerL1root),
                            "l1Leaf" | "l1_leaf" => Ok(GeneratedField::L1Leaf),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClaimFromRollup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.ClaimFromRollup")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClaimFromRollup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut proof_leaf_ler__ = None;
                let mut proof_ler_rer__ = None;
                let mut proof_ger_l1root__ = None;
                let mut l1_leaf__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProofLeafLer => {
                            if proof_leaf_ler__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proofLeafLer"));
                            }
                            proof_leaf_ler__ = map_.next_value()?;
                        }
                        GeneratedField::ProofLerRer => {
                            if proof_ler_rer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proofLerRer"));
                            }
                            proof_ler_rer__ = map_.next_value()?;
                        }
                        GeneratedField::ProofGerL1root => {
                            if proof_ger_l1root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proofGerL1root"));
                            }
                            proof_ger_l1root__ = map_.next_value()?;
                        }
                        GeneratedField::L1Leaf => {
                            if l1_leaf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("l1Leaf"));
                            }
                            l1_leaf__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ClaimFromRollup {
                    proof_leaf_ler: proof_leaf_ler__,
                    proof_ler_rer: proof_ler_rer__,
                    proof_ger_l1root: proof_ger_l1root__,
                    l1_leaf: l1_leaf__,
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.ClaimFromRollup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FixedBytes20 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.FixedBytes20", len)?;
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FixedBytes20 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = FixedBytes20;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.FixedBytes20")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FixedBytes20, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(FixedBytes20 {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.FixedBytes20", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FixedBytes32 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.FixedBytes32", len)?;
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FixedBytes32 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = FixedBytes32;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.FixedBytes32")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FixedBytes32, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(FixedBytes32 {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.FixedBytes32", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FixedBytes65 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.FixedBytes65", len)?;
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FixedBytes65 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = FixedBytes65;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.FixedBytes65")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FixedBytes65, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(FixedBytes65 {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.FixedBytes65", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportedBridgeExit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bridge_exit.is_some() {
            len += 1;
        }
        if self.global_index.is_some() {
            len += 1;
        }
        if self.claim.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.ImportedBridgeExit", len)?;
        if let Some(v) = self.bridge_exit.as_ref() {
            struct_ser.serialize_field("bridgeExit", v)?;
        }
        if let Some(v) = self.global_index.as_ref() {
            struct_ser.serialize_field("globalIndex", v)?;
        }
        if let Some(v) = self.claim.as_ref() {
            match v {
                imported_bridge_exit::Claim::Mainnet(v) => {
                    struct_ser.serialize_field("mainnet", v)?;
                }
                imported_bridge_exit::Claim::Rollup(v) => {
                    struct_ser.serialize_field("rollup", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportedBridgeExit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bridge_exit",
            "bridgeExit",
            "global_index",
            "globalIndex",
            "mainnet",
            "rollup",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BridgeExit,
            GlobalIndex,
            Mainnet,
            Rollup,
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
                            "bridgeExit" | "bridge_exit" => Ok(GeneratedField::BridgeExit),
                            "globalIndex" | "global_index" => Ok(GeneratedField::GlobalIndex),
                            "mainnet" => Ok(GeneratedField::Mainnet),
                            "rollup" => Ok(GeneratedField::Rollup),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportedBridgeExit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.ImportedBridgeExit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportedBridgeExit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bridge_exit__ = None;
                let mut global_index__ = None;
                let mut claim__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BridgeExit => {
                            if bridge_exit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bridgeExit"));
                            }
                            bridge_exit__ = map_.next_value()?;
                        }
                        GeneratedField::GlobalIndex => {
                            if global_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("globalIndex"));
                            }
                            global_index__ = map_.next_value()?;
                        }
                        GeneratedField::Mainnet => {
                            if claim__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mainnet"));
                            }
                            claim__ = map_.next_value::<::std::option::Option<_>>()?.map(imported_bridge_exit::Claim::Mainnet)
;
                        }
                        GeneratedField::Rollup => {
                            if claim__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rollup"));
                            }
                            claim__ = map_.next_value::<::std::option::Option<_>>()?.map(imported_bridge_exit::Claim::Rollup)
;
                        }
                    }
                }
                Ok(ImportedBridgeExit {
                    bridge_exit: bridge_exit__,
                    global_index: global_index__,
                    claim: claim__,
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.ImportedBridgeExit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for L1InfoTreeLeaf {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.global_exit_root.is_some() {
            len += 1;
        }
        if self.block_hash.is_some() {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.L1InfoTreeLeaf", len)?;
        if let Some(v) = self.global_exit_root.as_ref() {
            struct_ser.serialize_field("globalExitRoot", v)?;
        }
        if let Some(v) = self.block_hash.as_ref() {
            struct_ser.serialize_field("blockHash", v)?;
        }
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for L1InfoTreeLeaf {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "global_exit_root",
            "globalExitRoot",
            "block_hash",
            "blockHash",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GlobalExitRoot,
            BlockHash,
            Timestamp,
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
                            "globalExitRoot" | "global_exit_root" => Ok(GeneratedField::GlobalExitRoot),
                            "blockHash" | "block_hash" => Ok(GeneratedField::BlockHash),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = L1InfoTreeLeaf;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.L1InfoTreeLeaf")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<L1InfoTreeLeaf, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut global_exit_root__ = None;
                let mut block_hash__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GlobalExitRoot => {
                            if global_exit_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("globalExitRoot"));
                            }
                            global_exit_root__ = map_.next_value()?;
                        }
                        GeneratedField::BlockHash => {
                            if block_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHash"));
                            }
                            block_hash__ = map_.next_value()?;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(L1InfoTreeLeaf {
                    global_exit_root: global_exit_root__,
                    block_hash: block_hash__,
                    timestamp: timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.L1InfoTreeLeaf", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for L1InfoTreeLeafWithContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.l1_info_tree_index != 0 {
            len += 1;
        }
        if self.rer.is_some() {
            len += 1;
        }
        if self.mer.is_some() {
            len += 1;
        }
        if self.inner.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.L1InfoTreeLeafWithContext", len)?;
        if self.l1_info_tree_index != 0 {
            struct_ser.serialize_field("l1InfoTreeIndex", &self.l1_info_tree_index)?;
        }
        if let Some(v) = self.rer.as_ref() {
            struct_ser.serialize_field("rer", v)?;
        }
        if let Some(v) = self.mer.as_ref() {
            struct_ser.serialize_field("mer", v)?;
        }
        if let Some(v) = self.inner.as_ref() {
            struct_ser.serialize_field("inner", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for L1InfoTreeLeafWithContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "l1_info_tree_index",
            "l1InfoTreeIndex",
            "rer",
            "mer",
            "inner",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            L1InfoTreeIndex,
            Rer,
            Mer,
            Inner,
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
                            "l1InfoTreeIndex" | "l1_info_tree_index" => Ok(GeneratedField::L1InfoTreeIndex),
                            "rer" => Ok(GeneratedField::Rer),
                            "mer" => Ok(GeneratedField::Mer),
                            "inner" => Ok(GeneratedField::Inner),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = L1InfoTreeLeafWithContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.L1InfoTreeLeafWithContext")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<L1InfoTreeLeafWithContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut l1_info_tree_index__ = None;
                let mut rer__ = None;
                let mut mer__ = None;
                let mut inner__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::L1InfoTreeIndex => {
                            if l1_info_tree_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("l1InfoTreeIndex"));
                            }
                            l1_info_tree_index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Rer => {
                            if rer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rer"));
                            }
                            rer__ = map_.next_value()?;
                        }
                        GeneratedField::Mer => {
                            if mer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mer"));
                            }
                            mer__ = map_.next_value()?;
                        }
                        GeneratedField::Inner => {
                            if inner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inner"));
                            }
                            inner__ = map_.next_value()?;
                        }
                    }
                }
                Ok(L1InfoTreeLeafWithContext {
                    l1_info_tree_index: l1_info_tree_index__.unwrap_or_default(),
                    rer: rer__,
                    mer: mer__,
                    inner: inner__,
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.L1InfoTreeLeafWithContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LeafType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "LEAF_TYPE_UNSPECIFIED",
            Self::Transfer => "LEAF_TYPE_TRANSFER",
            Self::Message => "LEAF_TYPE_MESSAGE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for LeafType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LEAF_TYPE_UNSPECIFIED",
            "LEAF_TYPE_TRANSFER",
            "LEAF_TYPE_MESSAGE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LeafType;

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
                    "LEAF_TYPE_UNSPECIFIED" => Ok(LeafType::Unspecified),
                    "LEAF_TYPE_TRANSFER" => Ok(LeafType::Transfer),
                    "LEAF_TYPE_MESSAGE" => Ok(LeafType::Message),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MerkleProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.root.is_some() {
            len += 1;
        }
        if !self.siblings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.MerkleProof", len)?;
        if let Some(v) = self.root.as_ref() {
            struct_ser.serialize_field("root", v)?;
        }
        if !self.siblings.is_empty() {
            struct_ser.serialize_field("siblings", &self.siblings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "root",
            "siblings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Root,
            Siblings,
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
                            "root" => Ok(GeneratedField::Root),
                            "siblings" => Ok(GeneratedField::Siblings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MerkleProof;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.MerkleProof")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MerkleProof, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut root__ = None;
                let mut siblings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Root => {
                            if root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("root"));
                            }
                            root__ = map_.next_value()?;
                        }
                        GeneratedField::Siblings => {
                            if siblings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("siblings"));
                            }
                            siblings__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MerkleProof {
                    root: root__,
                    siblings: siblings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.MerkleProof", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Sp1StarkProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version.is_empty() {
            len += 1;
        }
        if !self.proof.is_empty() {
            len += 1;
        }
        if !self.vkey.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.SP1StarkProof", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.proof.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("proof", pbjson::private::base64::encode(&self.proof).as_str())?;
        }
        if !self.vkey.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("vkey", pbjson::private::base64::encode(&self.vkey).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Sp1StarkProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "proof",
            "vkey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Proof,
            Vkey,
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
                            "proof" => Ok(GeneratedField::Proof),
                            "vkey" => Ok(GeneratedField::Vkey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Sp1StarkProof;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.SP1StarkProof")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Sp1StarkProof, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut proof__ = None;
                let mut vkey__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Proof => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proof"));
                            }
                            proof__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Vkey => {
                            if vkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vkey"));
                            }
                            vkey__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Sp1StarkProof {
                    version: version__.unwrap_or_default(),
                    proof: proof__.unwrap_or_default(),
                    vkey: vkey__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.SP1StarkProof", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TokenInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.origin_network != 0 {
            len += 1;
        }
        if self.origin_token_address.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("agglayer.interop.types.v1.TokenInfo", len)?;
        if self.origin_network != 0 {
            struct_ser.serialize_field("originNetwork", &self.origin_network)?;
        }
        if let Some(v) = self.origin_token_address.as_ref() {
            struct_ser.serialize_field("originTokenAddress", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TokenInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "origin_network",
            "originNetwork",
            "origin_token_address",
            "originTokenAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OriginNetwork,
            OriginTokenAddress,
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
                            "originNetwork" | "origin_network" => Ok(GeneratedField::OriginNetwork),
                            "originTokenAddress" | "origin_token_address" => Ok(GeneratedField::OriginTokenAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TokenInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct agglayer.interop.types.v1.TokenInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TokenInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut origin_network__ = None;
                let mut origin_token_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OriginNetwork => {
                            if origin_network__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originNetwork"));
                            }
                            origin_network__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OriginTokenAddress => {
                            if origin_token_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originTokenAddress"));
                            }
                            origin_token_address__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TokenInfo {
                    origin_network: origin_network__.unwrap_or_default(),
                    origin_token_address: origin_token_address__,
                })
            }
        }
        deserializer.deserialize_struct("agglayer.interop.types.v1.TokenInfo", FIELDS, GeneratedVisitor)
    }
}
