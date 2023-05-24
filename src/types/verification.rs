use json_api_client::types::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    /// When the individual first starts the verification, the verification is initiated
    Initiated,
    /// When the individual submits their information, the server verifies their information
    Submitted,
    /// Once the server has verified the individual's information, the verification passes
    Passed,
    /// If the server fails to verify the individual's information, the verification fails
    Failed,
    /// Only for phone - The individual has verified that they have the physical device by entering a confirmation code. They have not submitted their information yet
    Confirmed,
    /// Not applicable for current verification
    NotApplicable,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum IdClass {
    #[serde(rename = "cct")]
    CitizenCertificate,
    #[serde(rename = "cid")]
    ConsularId,
    #[serde(rename = "dl")]
    DriverLicense,
    #[serde(rename = "hic")]
    HealthcareInsuranceCard,
    #[serde(rename = "id")]
    NationalId,
    #[serde(rename = "ipp")]
    InternalPassport,
    #[serde(rename = "ltpass")]
    LongTermPassCard,
    #[serde(rename = "mid")]
    MilitaryId,
    #[serde(rename = "myn")]
    MyNumberCard,
    #[serde(rename = "nbi")]
    NBICard,
    #[serde(rename = "nric")]
    NRIC,
    #[serde(rename = "ofw")]
    OFWId,
    #[serde(rename = "pan")]
    PANCard,
    #[serde(rename = "pid")]
    PostalId,
    #[serde(rename = "pp")]
    Passport,
    #[serde(rename = "ppc")]
    PassportCard,
    #[serde(rename = "pr")]
    PermanentResidentCard,
    #[serde(rename = "rp")]
    ResidencyPermit,
    #[serde(rename = "sss")]
    SocialSecurityId,
    #[serde(rename = "umid")]
    UMID,
    #[serde(rename = "vid")]
    VoterId,
    #[serde(rename = "visa")]
    Visa,
    #[serde(rename = "wp")]
    WorkPermit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Check {
    name: String,
    status: VerificationStatus,
    reasons: Vec<String>,
    // metadata - object
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    status: VerificationStatus,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: DateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub submitted_at: Option<DateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub completed_at: Option<DateTime>,
    pub country_code: Option<CountryCode>,
    pub entity_confidence_score: Option<u8>,
    pub entity_confidence_reasons: Vec<String>,
    pub left_photo_url: Option<String>,
    pub center_photo_url: Option<String>,
    pub right_photo_url: Option<String>,
    pub front_photo_url: Option<String>,
    pub back_photo_url: Option<String>,
    // pub photo_urls: Vec<???>
    pub document_similarity_score: Option<u8>,
    pub selfie_similarity_score_left: Option<u8>,
    pub selfie_similarity_score_right: Option<u8>,
    pub selfie_photo_url: Option<String>,
    pub id_class: Option<IdClass>,
    pub capture_method: String, // enum? e.g. "upload", "video"
    #[serde(flatten)]
    pub name: Option<Name>,
    pub birthdate: Option<Date>,
    #[serde(flatten)]
    pub address: Option<Address>,
    pub issuing_authority: Option<String>,
    pub issue_date: Option<Date>,
    pub expiration_date: Option<Date>,
    pub height: Option<String>,
    pub endorsements: Option<Vec<String>>,
    pub restrictions: Option<Vec<String>>,
    pub vehicle_class: Option<String>,
    pub driver_license_number: Option<String>,
    pub checks: Vec<Check>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {
    pub inquiry: Relation,
}

pub type Verification = EntityWithRelations<Attributes, Relationships>;
