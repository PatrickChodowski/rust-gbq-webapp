use yew::prelude::*;
use gcp_bigquery_client::error::BQError;

mod gbq;
mod utils;

use crate::gbq::{connect, query};
use crate::utils::Config;


#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}


#[tokio::main]
async fn main() -> Result<(), BQError> {
    let c: Config = Config::load();
    if let Ok(_client) = connect("./config/sa.json").await {


        println!("Client Connected!");
        if let Some(_res) = query(&_client, 
                                  &format!("SELECT * FROM `{}.{}.{}`", c.project_id, c.dataset_id, c.table_id),
                                  &c.project_id
                                ).await{
            println!("Query results: {:?}", _res);
        }
    }

    yew::Renderer::<App>::new().render();
    
    Ok(())

}
