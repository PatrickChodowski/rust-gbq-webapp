use gcp_bigquery_client::{*, error::BQError};
use gcp_bigquery_client::model::query_response::ResultSet;
use gcp_bigquery_client::model::query_request::QueryRequest;

pub async fn connect(sa: &str) -> Result<Client, BQError> {
    let client = Client::from_service_account_key_file(sa).await?;
    return Ok(client);
}


pub async fn query(client: &Client, query: &str, project_id: &str) -> Option<ResultSet> {
    if let Ok(_res) = client.job().query(project_id, QueryRequest::new(query)).await{
        return Some(_res);
    } else {
        return None;
    }
}


pub fn unpack_result(res: Option<ResultSet>) {

    // let v: 

    // if let Some(_res) = res {

    //     res.rows

    // }    

}