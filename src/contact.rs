use crate::webhook;

use log::debug;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;


#[function_component(Contact)]
pub fn contact() -> Html {
    let name = use_state(|| "".to_string());
    let address = use_state(|| "".to_string());
    let contents = use_state(|| "".to_string());

    let on_submit = {
        let name = name.clone();
        let address = address.clone();
        let contents = contents.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name.clone();
            let address = address.clone();
            let contents = contents.clone(); 


            spawn_local(async move {
                let webhook_url = webhook::WEBHOOK_URL;

                let payload = serde_json::json!({
                    "content": format!("**Contact**\n**Name:** {}\n**Message:** {}\n**Mail address:** {}", *name, *address, *contents,)
                });
                
                let response = Request::post(webhook_url)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&payload).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(resp) => debug!("Response: {:?}", resp),
                    Err(err) => log::error!("Failed to send webhook: {:?}", err),
                }

                
            });

            spawn_local(async move {
                let form_url = "";
                let data = format!(
                    "entry.12345678={}&entry.87654321={}&entry.13579246={}",
                    *name,
                    *address,
                    *contents
                );

                let form_submit_request = Request::post(form_url)
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(data);                
                let form_submit = match form_submit_request{
                    Ok(req) => req.send().await,
                    Err(err) => Err(err),
                };
                
                match form_submit {
                    Ok(response) => log::info!("送信成功！ありがとうございます。Response: {:?}", response),
                    Err(err) => log::error!("送信失敗しました。もう一度試してください。Error: {:?}", err),
                }
        
        });
    })
    };


    html! {
        <>
            <form onsubmit={on_submit}>
                <label for="name">{ "名前: " }</label>
                <input
                    id="name"
                    type="text"
                    value={(*name).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    name.set(input.value());
                })}
            />
            
            <label for="address">{ "mail address: " }</label>
            <input
                id="address"
                type="text"
                value={(*address).clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    address.set(input.value());
                })}
            />

            <label for="message">{ "メッセージ: " }</label>
            <textarea
                id="message"
                value={(*contents).clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    contents.set(input.value());
                })}
            ></textarea>
                <button type="submit">{ "送信" }</button>
            </form>        
            <>
                <div>
                    <h2>{"Collected data will be stored responsibly"}</h2>
                </div>
            </>
        </>
    }
}
