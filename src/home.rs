use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <style>
            "
            .about { border: 2px solid var(--mauve); }
            .about h2 { color: var(--mauve); margin: 0 0 1.5rem 0; text-align: center; font-size: 1.8rem; }
            .about-text { font-size: 1.15rem; line-height: 1.7; max-width: 750px; margin: 0 auto; text-align: center; }
            "
        </style>
        <section class="card about">
            <h2>"🌸 About Me:"</h2>
            <div class="about-text">
                <p>"Hi! I like to build backend systems, write memory-safe code for embedded devices, and develop flight software for rockets!"</p>
                <p>"Currently architecting hybrid rocket avionics and pushing for Rust adoption in aerospace environments. 🎀"</p>
            </div>
        </section>
    }
}
