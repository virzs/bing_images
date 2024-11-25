use leptos::{component, create_effect, create_signal, view, IntoView, SignalGet, SignalSet};
use leptos_tutorial::services::request::{get_images_list, Image};
use thaw::{BackTop, Layout, LayoutHeader};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[component]
pub fn home() -> impl IntoView {
    let (images, set_images) = create_signal::<Vec<Image>>(Vec::new());

    create_effect(move |_| {
        console::log_1(&JsValue::from_str("create_effect triggered"));
        wasm_bindgen_futures::spawn_local(async move {
            match get_images_list().await {
                Ok(data) => {
                    console::log_1(&JsValue::from_str(&format!("Fetched images: {:?}", data)));
                    set_images.set(data);
                    console::log_1(&JsValue::from_str(&format!(
                        "Images set: {:?}",
                        images.get()
                    )));
                }
                Err(err) => {
                    console::log_1(&JsValue::from_str(&format!(
                        "Error fetching images: {:?}",
                        err
                    )));
                    set_images.set(vec![]);
                }
            }
        });
        ()
    });

    view! {
        <div class="w-screen h-screen">
            <Layout class="w-full h-full" content_class="h-full">
                <LayoutHeader>"Header"</LayoutHeader>
                <Layout>
                    <div class="grid grid-cols-3 gap-4 max-w-5xl mx-auto">
                        {move || images.get().iter().map(|image| view! {
                            <div>
                                <h2>{&image.title}</h2>
                                <img class="w-full" src={&image.url} alt={&image.title} />
                                <a href={&image.copyrightlink}>{&image.copyright}</a>
                            </div>
                        }).collect::<Vec<_>>()}
                    </div>
                </Layout>
            </Layout>
            <BackTop />
        </div>
    }
}
