use agglayer_interop_types::{Digest, MerkleProof};

use super::Error;
use crate::v1;

impl TryFrom<v1::MerkleProof> for MerkleProof {
    type Error = Error;

    fn try_from(value: v1::MerkleProof) -> Result<Self, Self::Error> {
        if value.siblings.len() != 32 {
            return Err(Error::invalid_data(format!(
                "expected 32 elements for merkle proof, got {}",
                value.siblings.len()
            )));
        }
        let siblings: Vec<Digest> = value
            .siblings
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()
            .map_err(|e: Error| e.inside_field("siblings"))?;
        let siblings: [Digest; 32] = siblings.try_into().unwrap(); // Checked just two statements above
        Ok(MerkleProof::new(required_field!(value, root), siblings))
    }
}

impl From<MerkleProof> for v1::MerkleProof {
    #[inline]
    fn from(value: MerkleProof) -> Self {
        v1::MerkleProof {
            root: Some(value.root.into()),
            siblings: value.siblings().iter().copied().map(Into::into).collect(),
        }
    }
}
