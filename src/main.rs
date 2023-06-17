use yew::prelude::*;
use gcp_bigquery_client::error::BQError;

mod gbq;
mod utils;

use crate::gbq::{connect, query};
use crate::utils::Config;

mod about;
mod counter;


#[function_component]
fn App() -> Html {
    html! {
        <>
            <about::AboutPage />
            <counter::Counter />
        </>
    }
}



fn main() {
    let c: Config = Config::load();
    // if let Ok(_client) = connect("./config/sa.json").await {


    //     println!("Client Connected!");
    //     if let Some(_res) = query(&_client, 
    //                               &format!("SELECT * FROM `{}.{}.{}`", c.project_id, c.dataset_id, c.table_id),
    //                               &c.project_id
    //                             ).await{
    //         println!("Query results: {:?}", _res);
    //     }
    // }

    yew::Renderer::<App>::new().render();
    
}
