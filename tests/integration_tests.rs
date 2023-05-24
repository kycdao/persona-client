use persona_client::*;
use test_log::test;

const SANDBOX_TOKEN: &str = "REPLACE_ME";

fn get_client() -> Client {
    Client::new(SANDBOX_TOKEN).unwrap()
}

#[test(tokio::test)]
#[ignore]
async fn list_inquiries() {
    let resp = get_client().list_inquiries().await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn get_inquiry() {
    let resp = get_client().get_inquiry("inq_urLp56nbbaPTh4inBnrTqBFz").await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn create_empty_inquiry() {
    let resp = get_client().create_empty_inquiry("itmpl_TphbBze4KLtx69cP6urZMyda").await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn list_reports() {
    let resp = get_client().list_reports().await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn list_accounts() {
    let resp = get_client().list_accounts().await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn get_account() {
    let resp = get_client().get_account("act_rJ5kTVBg9yGJ9B5GWdSgeVFS").await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn list_cases() {
    let resp = get_client().list_cases().await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn get_document() {
    let resp = get_client().get_document("doc_6GCqB9GhoovEiHdX9hNdwzBw").await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn get_verification() {
    let verification_id = "ver_dTHDVnboZ9xG92aJZu8mr19B";
    let resp = get_client().get_verification(verification_id).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());

    let verification_id = "ver_ETnWwqsJSZxbqTtFbmT77TBm";
    let resp = get_client().get_verification(verification_id).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
async fn get_event() {
    let resp = get_client().get_event::<Inquiry>("evt_71fVVErvRpkrFd1SGZowX1zR").await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}
