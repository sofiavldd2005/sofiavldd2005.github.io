use leptos::prelude::*;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <style>
            "
            .lost-container {
                /* These rules ensure it covers the entire screen, hiding the global Earthrise background */
                position: fixed;
                top: 0;
                left: 0;
                width: 100vw;
                height: 100vh;
                z-index: 40;
                
                display: flex;
                align-items: center;
                justify-content: center;
                
                /* The astronaut background with a slight dark tint for text readability */
                background: linear-gradient(
                    rgba(30, 30, 46, 0.85),
                    rgba(203, 166, 247, 0.15)
                ), url('/assets/floatingastronaut.webp') no-repeat center center;
                background-size: cover;
            }

            .lost-card {
                background-color: rgba(30, 30, 46, 0.75);
                backdrop-filter: blur(12px);
                -webkit-backdrop-filter: blur(12px);
                border: 2px solid var(--mauve);
                border-radius: 16px;
                padding: 3rem 2rem;
                max-width: 450px;
                text-align: center;
                box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
            }

            .error-code {
                font-size: 6rem;
                color: var(--pink);
                margin: 0 0 0.5rem 0;
                font-weight: bold;
                text-shadow: 0 0 15px rgba(245, 194, 231, 0.4);
                line-height: 1;
            }

            .error-msg {
                color: var(--text);
                font-size: 1.2rem;
                line-height: 1.6;
                margin-bottom: 2.5rem;
            }

            .return-btn {
                display: inline-block;
                padding: 0.8rem 1.8rem;
                font-size: 1.1rem;
                font-weight: bold;
                color: var(--base);
                background-color: var(--mauve);
                border: none;
                border-radius: 8px;
                text-decoration: none;
                transition: all 0.2s ease;
            }

            .return-btn:hover {
                transform: translateY(-3px);
                box-shadow: 0 6px 15px rgba(203, 166, 247, 0.4);
                background-color: var(--pink);
            }
            .photo-credit {
                position: fixed;
                bottom: 1rem;
                right: 1rem;
                background-color: rgba(30, 30, 46, 0.65);
                backdrop-filter: blur(5px);
                -webkit-backdrop-filter: blur(5px);
                border: 1px solid var(--surface-border, rgba(255,255,255,0.1));
                border-radius: 8px;
                padding: 0.5rem 0.5rem;
                font-size: 0.8rem;
                color: var(--text);
                z-index: 50;
                pointer-events: none; /* Ensures you can still click things underneath it */
                opacity: 0.7;
                transition: opacity 0.3s ease;
            }
            "
        </style>

        <div class="lost-container">
            <div class="lost-card">
                <h1 class="error-code">"404"</h1>
                <p class="error-msg">"Signal Lost. The satellite you are looking for has drifted out of orbit."</p>
                <a href="/" class="return-btn">" Return to Base"</a>
            </div>
        </div>
        <div class="photo-credit">
                "Astronaut Bruce McCandless Floating Free - Photographed by NASA (2011)"
        </div>
    }
}
