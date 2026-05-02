use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        // We inject CSS directly here to cleanly manage our Catppuccin theme and responsive layout!
        <style>
            "
            :root {
                --base: #1e1e2e;
                --mantle: #181825;
                --text: #cdd6f4;
                --subtext: #bac2de;
                --pink: #f5c2e7;
                --mauve: #cba6f7;
                --green: #a6e3a1;
                --yellow: #f9e2af;
                --red: #f38ba8;
                --surface: #313244;
                --surface-border: #45475a;
            }
            
            body {
                margin: 0;
                background-color: var(--base);
                color: var(--text);
                font-family: 'Nunito', system-ui, -apple-system, sans-serif;
            }

            .container {
                max-width: 950px;
                margin: 0 auto;
                padding: 3rem 1.5rem;
            }

            .header {
                border-bottom: 2px dashed var(--pink);
                padding-bottom: 2rem;
                margin-bottom: 3rem;
                text-align: center;
            }

            .header h1 {
                color: var(--pink);
                /* clamp() makes the font size scale smoothly based on screen width! */
                font-size: clamp(2rem, 5vw, 2.8rem); 
                margin: 0 0 0.5rem 0;
                line-height: 1.2;
            }

            .header p {
                color: var(--subtext);
                font-size: 1.1rem;
                font-style: italic;
                margin: 0;
            }

            .card {
                background-color: var(--mantle);
                border-radius: 16px;
                padding: 2.5rem;
                box-shadow: 0 4px 15px rgba(0,0,0,0.2);
                margin-bottom: 2.5rem;
            }

            .card.about { border: 2px solid var(--mauve); }
            .card.resume { border: 2px solid var(--green); }

            .card h2 {
                margin: 0 0 1.5rem 0;
                text-align: center;
                font-size: 1.8rem;
            }

            .card.about h2 { color: var(--mauve); }
            .card.resume h2 { color: var(--green); }
            
            .about-text {
                font-size: 1.15rem;
                line-height: 1.7;
                max-width: 750px;
                margin: 0 auto;
                text-align: center;
            }
            
            .resume-grid {
                display: grid;
                gap: 3rem;
            }
            
            /* Switches to two columns on larger screens */
            @media (min-width: 768px) {
                .resume-grid { grid-template-columns: 1fr 1fr; }
            }
            
            .section-heading {
                border-bottom: 1px solid var(--surface-border);
                padding-bottom: 0.5rem;
                margin-bottom: 1.5rem;
                font-size: 1.2rem;
                margin-top: 0;
            }
            
            .col-left .section-heading { color: var(--yellow); }
            .col-right .section-heading { color: var(--red); }
            
            .experience-list {
                list-style-type: none;
                padding: 0;
                margin: 0;
            }
            
            .experience-list li { margin-bottom: 1.5rem; }
            
            .role-title {
                color: var(--text);
                font-weight: bold;
                display: block;
                margin-bottom: 0.3rem;
                font-size: 1.05rem;
            }
            
            .role-desc {
                color: var(--subtext);
                font-size: 0.95rem;
                line-height: 1.5;
            }
            
            .skills-container {
                display: flex;
                flex-wrap: wrap;
                gap: 0.6rem;
            }
            
            .skill-badge {
                background-color: var(--surface);
                color: #f5e0dc;
                padding: 0.4rem 0.8rem;
                border-radius: 8px;
                font-size: 0.9rem;
                border: 1px solid var(--surface-border);
                font-weight: bold;
            }
            "
        </style>

        <main>
            <div class="container">
                <header class="header">
                    <h1>"✨ Sofia Duarte's Mission Control ✨"</h1>
                    <p>"Aerospace Engineering @ IST 🚀 | RED Rocketry | Embedded Rustacean 🦀"</p>
                </header>

                <section class="card about">
                    <h2>"🌸 About Me: "</h2>
                    <div class="about-text">
                        <p>"Hi! I build backend systems, write memory-safe code for embedded devices, and develop flight software for rockets :)"</p>
                        <p>"Currently architecting hybrid rocket avionics and pushing for Rust adoption in aerospace environments. 🎀"</p>
                    </div>
                </section>

                <CompactResume />
        <p> "The front end for this website was built using leptos Framework, built in rust for WebDev!"</p>
            </div>
        </main>
    }
}

#[component]
fn CompactResume() -> impl IntoView {
    view! {
        <section class="card resume">
            <h2>"Service Record & Tech Stack"</h2>

            <div class="resume-grid">

                // Left Column: Experience
                <div class="col-left">
                    <h3 class="section-heading">"🚀 Flight & Space Systems"</h3>
                    <ul class="experience-list">
                        <li>
                            <span class="role-title">"Co-Team Leader @ RED Electronics"</span>
                            <span class="role-desc">"Architecting hybrid rocket flight software on an RTOS. Managing distributed avionics, HIL testing, and driving the transition to embedded Rust."</span>
                        </li>
                        <li>
                            <span class="role-title">"Command & Data Handling @ LISAT"</span>
                            <span class="role-desc">"Developed cache optimization algorithms to streamline orbit data processing for satellite onboard computers."</span>
                        </li>
                    </ul>

                    <h3 class="section-heading" style="margin-top: 2rem;">"🦀 Open Source & Projects"</h3>
                    <ul class="experience-list">
                        <li>
                            <span class="role-title">"Rust HIL & Ground Station Framework"</span>
                            <span class="role-desc">"Modular, asynchronous simulation engine for avionics validation using Embassy and Rust."</span>
                        </li>
                    </ul>
                </div>

                // Right Column: Skills
                <div class="col-right">
                    <h3 class="section-heading">"💻 Hardware & Embedded"</h3>
                    <div class="skills-container">
                        <SkillBadge text="C/C++" />
                        <SkillBadge text="Embedded Rust" />
                        <SkillBadge text="RISC-V" />
                        <SkillBadge text="STM32 & ESP32" />
                        <SkillBadge text="FreeRTOS" />
                        <SkillBadge text="Async/Await (Embassy)" />
                        <SkillBadge text="KiCad" />
                    </div>

                    <h3 class="section-heading" style="margin-top: 2.5rem;">"⚙️ Systems & Analysis"</h3>
                    <div class="skills-container">
                        <SkillBadge text="Bash" />
                        <SkillBadge text="MATLAB" />
                        <SkillBadge text="GitHub Actions CI/CD" />
                        <SkillBadge text="Linux" />
                        <SkillBadge text= "Lua" />
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn SkillBadge(text: &'static str) -> impl IntoView {
    view! {
        <span class="skill-badge">
            {text}
        </span>
    }
}
