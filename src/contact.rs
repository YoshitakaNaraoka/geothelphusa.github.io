use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

const WEBHOOK_URL: &str = env!("WEBHOOK_URL");

#[function_component(Contact)]
pub fn contact() -> Html {
    let name = use_state(|| String::new());
    let message = use_state(|| String::new());

    let on_submit = {
        let name = name.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); // フォーム送信を防ぐ
            let name = name.clone();
            let message = message.clone();

            spawn_local(async move {
                let payload = serde_json::json!({
                    "content": format!("**お問い合わせ**\n**名前:** {}\n**メッセージ:** {}", *name, *message)
                });

                let response = Request::post(WEBHOOK_URL)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&payload).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.status() == 204 {
                            web_sys::console::log_1(&"送信成功".into());
                        } else {
                            web_sys::console::log_1(&format!("エラー: {}", resp.status()).into());
                        }
                    }
                    Err(err) => {
                        web_sys::console::log_1(&format!("リクエスト失敗: {:?}", err).into());
                    }
                }
            });
        })
    };

    html! {
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

            <label for="message">{ "メッセージ: " }</label>
            <textarea
                id="message"
                value={(*message).clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    let textarea: HtmlTextAreaElement = e.target_unchecked_into();
                    message.set(textarea.value());
                })}
            ></textarea>

            <button type="submit">{ "送信" }</button>
        </form>
    }
}
