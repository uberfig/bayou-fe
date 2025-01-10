use leptos::{
    component,
    prelude::*,
    view, IntoView,
};
use crate::masto_types::status::MediaAttachment;

#[component]
pub fn Attachments(attachments: Vec<MediaAttachment>) -> impl IntoView {
    view! {
        {generate_attachments(attachments)}
    }
}

pub fn generate_attachments(attachments: Vec<MediaAttachment>) -> AnyView {
    let mut attachments = attachments.into_iter()
            .map(|attachment| {
                match attachment.type_field {
                    crate::masto_types::status::MediaType::Unknown => view! {<a href={attachment.url.to_string()}>{attachment.url.to_string()}</a>}.into_any(),
                    crate::masto_types::status::MediaType::Image => view! {
                        <img class="attachment attachment-img" src={attachment.url.to_string()} alt={attachment.description}/>
                    }.into_any(),
                    crate::masto_types::status::MediaType::Gifv => view! {
                        <video class="attachment attachment-gif" autoplay muted loop aria-label={attachment.description}>
                            <source src={attachment.url.to_string()} type="video/mp4" />
                        </video>
                    }.into_any(),
                    crate::masto_types::status::MediaType::Video => //view! {<p class="attachment attachment-video" >{attachment.url.to_string()}</p>}.into_any(),
                    view! {
                        <video class="attachment attachment-video" controls aria-label={attachment.description}>
                            <source src={attachment.url.to_string()} 
                                // type="video/mp4" 
                            />
                        </video>
                    }.into_any(),
                    crate::masto_types::status::MediaType::Audio => view! {
                        <audio class="attachment attachment-audio" controls>
                            <source src={attachment.url.to_string()} type="audio" />
                            {"Your browser does not support the audio element."}
                        </audio>
                    }.into_any(),
                }
            })
            .collect::<Vec<_>>();

    match attachments.len() {
        0 => view! {}.into_any(),
        1 => view! {
            <div class="attachment-container">
                {attachments}
            </div>
        }
        .into_any(),
        2 => {
            let first = attachments.remove(0);
            let second = attachments.remove(0);
            view! {
                <div class="attachment-container">
                    <div class="multiple-attachment">
                        <div class="w50 h100">
                            {first}
                        </div>
                        <div class="w50 h100">
                            {second}
                        </div>
                    </div>
                </div>
            }
            .into_any()
        }
        3 => {
            let first = attachments.remove(0);
            view! {
                <div class="attachment-container">
                    <div class="multiple-attachment">
                        <div class="primary w75 h100">
                            {first}
                        </div>
                        <div class="secondary w25 h50">
                            {attachments}
                        </div>
                    </div>
                </div>
            }
            .into_any()
        }
        4 => {
            let first = attachments.remove(0);
            let second = attachments.remove(0);
            let third = attachments.remove(0);
            let fourth = attachments.remove(0);
            view! {
                <div class="attachment-container">
                    <div class="multiple-attachment">
                        <div class="secondary w50 h50">
                            {first}
                            {second}
                        </div>
                        <div class="secondary w50 h50">
                            {third}
                            {fourth}
                        </div>
                    </div>
                </div>
            }
            .into_any()
        }
        _ => {
            let first = attachments.remove(0);
            view! {
                <div class="attachment-container">
                    <div class="multiple-attachment">
                        <div class="primary w75">
                            {first}
                        </div>
                        <div class="secondary w25">
                            {attachments}
                        </div>
                    </div>
                </div>
            }
            .into_any()
        }
    }
}