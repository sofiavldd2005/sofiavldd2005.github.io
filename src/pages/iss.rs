use gloo_timers::future::sleep;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;
use std::time::Duration;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct IssData {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub velocity: f64,
}

#[component]
pub fn Iss() -> impl IntoView {
    // We use a signal to hold our live data so the UI updates automatically
    let (data, set_data) = signal(IssData::default());
    let (loading, set_loading) = signal(true);

    // This Effect runs exactly once when the page loads
    Effect::new(move |_| {
        // We spawn a local async task that loops endlessly
        spawn_local(async move {
            loop {
                // Fetch the live data from the API
                if let Ok(res) =
                    reqwest::get("https://api.wheretheiss.at/v1/satellites/25544").await
                {
                    if let Ok(new_data) = res.json::<IssData>().await {
                        set_data.set(new_data);
                        set_loading.set(false);
                    }
                }
                // Wait 3 seconds before fetching again
                sleep(Duration::from_secs(3)).await;
            }
        });
    });

    view! {
        <style>
            "
            body {
                background: linear-gradient(
                    rgba(30, 30, 46, 0.85),
                    rgba(203, 166, 247, 0.15)
                ), url('/assets/iss.webp') no-repeat center center fixed !important;
                -webkit-background-size: cover !important;
                background-size: cover !important;
            }

            .iss-container {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                min-height: 70vh;
                padding: 2rem;
            }

            .telemetry-screen {
                background-color: rgba(17, 17, 27, 0.55); 
                border: 2px solid var(--yellow);
                border-radius: 12px;
                padding: 2.5rem;
                width: 100%;
                max-width: 500px;
                box-shadow: 0 0 20px rgba(233, 142, 29, 0.2); 
            }

            .telemetry-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                border-bottom: 1px dashed var(--yellow);
                padding-bottom: 1rem;
                margin-bottom: 1.5rem;
            }

            .telemetry-title {
                color: var(--yellow);
                margin: 0;
                font-size: 1.2rem;
                font-weight: bold;
                text-transform: uppercase;
                letter-spacing: 2px;
            }

            .live-indicator {
                color: var(--red);
                font-weight: bold;
                display: flex;
                align-items: center;
                gap: 0.5rem;
            }

            .blink {
                width: 10px;
                height: 10px;
                background-color: var(--red);
                border-radius: 50%;
                animation: blinker 1s linear infinite;
            }

            @keyframes blinker {
                50% { opacity: 0; }
            }

            .data-row {
                display: flex;
                justify-content: space-between;
                color: var(--text);
                margin-bottom: 1rem;
                font-size: 1.1rem;
            }

            .data-label {
                color: var(--mauve);
                font-weight: bold;
            }

            .loading-text {
                color: var(--yellow);
                text-align: center;
                margin-top: 2rem;
                font-style: italic;
            }
            .photo-credit {
                position: fixed;
                bottom: 1rem;
                right: 1rem;
                background-color: rgba(30, 30, 46, 0.65);
                backdrop-filter: blur(8px);
                -webkit-backdrop-filter: blur(8px);
                border: 1px solid var(--surface-border, rgba(255,255,255,0.1));
                border-radius: 8px;
                padding: 0.5rem 0.8rem;
                font-size: 0.8rem;
                color: var(--text);
                z-index: 50;
                pointer-events: none; /* Ensures you can still click things underneath it */
                opacity: 0.7;
                transition: opacity 0.3s ease;
            }
 
            "
        </style>

        <div class="iss-container">
            <div class="telemetry-screen">
                <div class="telemetry-header">
                    <h2 class="telemetry-title">"ISS Telemetry"</h2>
                    <div class="live-indicator">
                        <div class="blink"></div> "LIVE"
                    </div>
                </div>

                <Show
                    when=move || !loading.get()
                    fallback=|| view! { <div class="loading-text">"Establishing connection with satellite..."</div> }
                >
                    <div class="data-row">
                        <span class="data-label">"LATITUDE:"</span>
                        // Format to 4 decimal places
                        <span>{move || format!("{:.4}°", data.get().latitude)}</span>
                    </div>
                    <div class="data-row">
                        <span class="data-label">"LONGITUDE:"</span>
                        <span>{move || format!("{:.4}°", data.get().longitude)}</span>
                    </div>
                    <div class="data-row">
                        <span class="data-label">"ALTITUDE:"</span>
                        <span>{move || format!("{:.2} km", data.get().altitude)}</span>
                    </div>
                    <div class="data-row">
                        <span class="data-label">"VELOCITY:"</span>
                        <span>{move || format!("{:.2} km/h", data.get().velocity)}</span>
                    </div>
                </Show>
            </div>
        </div>
         <div class="photo-credit">
                "ISS as of 2018 — Photographed by NASA"

            </div>
    }
}
