mod error;
mod types;

use reqwest::header::*;
use serde_json::json;
use std::fmt::Debug;

pub use error::*;
pub use types::*;

use json_api_client::*;

const PERSONA_API_VERSION: &str = "2021-07-05";
const API_URL: &str = "https://withpersona.com/api/v1/";

pub struct Client {
    api: ApiClient,
}

impl Client {
    pub fn new(api_token: &str) -> Result<Client> {
        let mut headers = HeaderMap::new();

        headers.insert(HeaderName::from_static("key-inflection"), HeaderValue::from_static("snake"));
        headers.insert(HeaderName::from_static("persona-version"), HeaderValue::from_static(PERSONA_API_VERSION));

        let c = AuthorizationHeaderConfig {
            token: format!("Bearer {}", api_token),
        };

        let client = ApiClient::new(API_URL, AuthConfig::AuthorizationHeader(c), Some(headers))?;

        Ok(Client { api: client })
    }

    // TODO KYC-326 somehow parse Request-Id header for debugging reasons

    async fn get<T>(&self, path: &str) -> Result<Response<T>>
    where
        T: JsonResponse,
    {
        self.api.get(path, None, None).await.map_err(Error::from)
    }

    fn generate_idempotency_key() -> HeaderMap {
        // TODO KYC-327 generate random idempotency key
        #[allow(unused_mut)]
        let mut headers = HeaderMap::new();
        //let key = "RANDOM";
        //headers.insert(HeaderName::from_static("Idempotency-Key"), HeaderValue::from_str(key).expect("Generated idempotency key should be valid"));
        headers
    }

    async fn post<T>(&self, path: &str, data: &serde_json::Value) -> Result<Response<T>>
    where
        T: JsonResponse,
    {
        let headers = Client::generate_idempotency_key();
        self.api.post(path, Some(data), Some(headers)).await.map_err(Error::from)
    }

    #[allow(dead_code)]
    async fn patch<T>(&self, path: &str, data: &serde_json::Value) -> Result<Response<T>>
    where
        T: JsonResponse,
    {
        let headers = Client::generate_idempotency_key();
        self.api.patch(path, Some(data), Some(headers)).await.map_err(Error::from)
    }

    #[allow(dead_code)]
    async fn delete<T>(&self, path: &str) -> Result<Response<T>>
    where
        T: JsonResponse,
    {
        self.api.delete(path, None).await.map_err(Error::from)
    }

    // **********
    // Inquiries
    // **********

    pub async fn list_inquiries(&self) -> Result<Response<Vec<inquiry::Inquiry>>> {
        self.get("inquiries").await
    }

    pub async fn get_inquiry(&self, inquiry_id: &str) -> Result<Response<inquiry::Inquiry>> {
        let path = format!("inquiries/{}", inquiry_id);
        self.get(&path).await
    }

    pub async fn create_empty_inquiry(&self, inquiry_template_id: &str) -> Result<Response<inquiry::Inquiry>> {
        let data = json!({ "data": { "attributes": { "inquiry_template_id": inquiry_template_id }}});
        self.post("inquiries", &data).await
    }

    // Print Inquiry PDF
    // Update Inquiry
    // Approve Inquiry
    // Decline Inquiry
    // Redact Inquiry
    // Add / Remove / Set Inquiry tags

    // **************
    // Verifications
    // **************

    pub async fn get_verification(&self, verification_id: &str) -> Result<Response<verification::Verification>> {
        let path = format!("verifications/{}", verification_id);
        self.get(&path).await
    }

    // Print Verification PDF

    // **************
    // Reports
    // **************

    pub async fn list_reports(&self) -> Result<Response<Vec<report::ReportSummary>>> {
        self.get("reports").await
    }

    pub async fn get_report(&self, report_id: &str) -> Result<Response<report::ReportDetails>> {
        let path = format!("reports/{}", report_id);
        self.get(&path).await
    }

    // Create Report (requires Report Template)
    // Print Report PDF
    // Add / Remove / Set Report tags

    // *********
    // Accounts
    // *********

    pub async fn list_accounts(&self) -> Result<Response<Vec<account::Account>>> {
        self.get("accounts").await
    }

    pub async fn get_account(&self, account_id: &str) -> Result<Response<account::Account>> {
        let path = format!("accounts/{}", account_id);
        self.get(&path).await
    }

    // Create Account
    // Redact Account
    // Update Account
    // Add / Remove / Set Account tags
    // Consolidate Account

    // *********
    // Cases
    // *********

    pub async fn list_cases(&self) -> Result<Response<Vec<case::Case>>> {
        self.get("cases").await
    }

    pub async fn get_case(&self, case_id: &str) -> Result<Response<case::Case>> {
        let path = format!("cases/{}", case_id);
        self.get(&path).await
    }

    // Create Case
    // Add Object to Case
    // Assign Case

    // **********
    // Documents
    // **********

    pub async fn get_document(&self, document_id: &str) -> Result<Response<document::Document>> {
        let path = format!("documents/{}", document_id);
        self.get(&path).await
    }

    // Download a file "files/{file-id}/{file-name}"
    // TODO KYC-320 requires streaming

    // ******************
    // Events / Webhooks
    // ******************

    // TODO KYC-325 this would require a heterogeneous list!
    /*pub async fn list_events<T>(&self) -> Result<Response<Vec<event::Event<T>>>> {
        self.get("events").await
    }*/

    pub async fn get_event<T>(&self, event_id: &str) -> Result<Response<event::Event<T>>>
    where
        T: JsonResponse,
    {
        let path = format!("events/{}", event_id);
        self.get(&path).await
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    // put unittests here
}
