use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <style>
            "
            .about { border: 2px solid var(--mauve);
            text-align: center;
            background-color: rgba(30, 30, 46, 0.55); /* Catppuccin dark base at 65% opacity */
            backdrop-filter: blur(5px); /* Softly blurs the Earthrise image behind the card */
            -webkit-backdrop-filter: blur(10px); /* Safari support (when it decides to work!) */
            border-radius: 16px;
            padding: 2rem;
            
            }
            .about h2 { color: var(--mauve); margin: 0 0 1.5rem 0; text-align: center; font-size: 1.8rem; text-align: left }
            .about-text { font-size: 1.15rem; line-height: 1.7; max-width: 750px; margin: 0 auto; text-align: left; }
            /* Social Buttons Styling */
            .socials-container {
                display: flex;
                justify-content: center;
                gap: 1rem;
                margin-top: 2rem;
                flex-wrap: wrap;
            }
            /* New lead-in text styling */
            .reach-me {
                text-align: center;
                margin-top: 2.5rem;
                margin-bottom: 1rem;
                color: var(--subtext);
                font-size: 1.1rem;
                font-weight: bold;
            }
            /* Custom hover colors for each platform */
            .social-btn.email:hover { border-color: var(--red); color: var(--red); }
            .social-btn.github:hover { border-color: var(--mauve); color: var(--mauve); }
            .social-btn.linkedin:hover { border-color: var(--green); color: var(--green); }
            "
        </style>
        <section class="card about">
            <h2>"About Me:"</h2>
            <div class="about-text">
                <p>"Hi! I like to build backend systems, write memory-safe code for embedded devices, and develop flight software for rockets!"</p>
                <p>"Currently architecting hybrid rocket avionics and pushing for Rust adoption in aerospace environments."</p>
            </div>
            <div class="reach-me">
                    <h3>"Here is how you can reach me :) "</h3>
                </div>
            <div class="socials-container">

                    <a href="mailto:sofia.vazquez.duarte@tecnico.ulisboa.pt" class="social-btn email">
                        "Email"
                    </a>
                    <a href="https://github.com/sofiavldd2005" target="_blank" rel="noopener noreferrer" class="social-btn github">
                        "GitHub"
                    </a>
                    <a href="https://www.linkedin.com/in/sofia-duarte-2528682a1" target="_blank" rel="noopener noreferrer" class="social-btn linkedin">
                        "LinkedIn"
                    </a>
                </div>
        </section>
        <div class="built-with">
                "Built with 💖 and "
                <a href="https://leptos.dev/" target="_blank" rel="noopener noreferrer">"Leptos"</a>
            </div>
    }
}
