use model::individual::{Individual, IndividualBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIndividualRequest {
    pub id: i32,
    pub name: String,
    pub document: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveIndividualRequest {
    pub name: String,
    pub document: String,
}

impl Into<Individual> for SaveIndividualRequest {
    fn into(self) -> Individual {
        IndividualBuilder::default()
            .name(self.name)
            .document(self.document)
            .build()
            .unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct SaveIndividualResponse {
    pub id: i32,
    pub name: String,
    pub document: String,
}

impl From<&Individual> for SaveIndividualResponse {
    fn from(value: &Individual) -> Self {
        SaveIndividualResponse {
            id: value.id,
            name: value.name.clone(),
            document: value.document.clone(),
        }
    }
}
