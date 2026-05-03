use crate::pages::home::Home;
use crate::pages::projects::Projects;
use crate::pages::resume::Resume;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <style>
            "
            :root {
                --base: #1e1e2e; --mantle: #181825; --text: #cdd6f4; --subtext: #bac2de;
                --pink: #f5c2e7; --mauve: #cba6f7; --green: #a6e3a1; --yellow: #f9e2af;
                --red: #f38ba8; --surface: #313244; --surface-border: #45475a;
            }
            body {  
                margin: 0; 
                
                /* The Magic Overlay: Catppuccin Base at the top fading into a soft Mauve tint at the bottom */
                background: linear-gradient(
                    rgba(30, 30, 46, 0.85), /* Dark overlay to keep text readable */
                    rgba(203, 166, 247, 0.25) /* Cute mauve/pink tint */
                ), url('/assets/astronautkristina.jpg');
                
                background-size: cover;
                background-position: center;
                background-attachment: fixed; /* Keeps the background still while you scroll */
                
                color: var(--text); 
                font-family: 'Nunito', system-ui, sans-serif; 
             }
            .container { max-width: 950px; margin: 0 auto; padding: 3rem 1.5rem; }
            .header { border-bottom: 2px dashed var(--pink); padding-bottom: 2rem; margin-bottom: 3rem; text-align: center; }
            .header h1 { color: var(--mauve); font-size: clamp(2rem, 5vw, 2.8rem); margin: 0 0 0.5rem 0; line-height: 1.2; }
            .header p { color: var(--subtext); font-size: 1.1rem; font-style: italic; margin: 0; }
            
            /* Nav bar styling */
            .nav-bar { display: flex; justify-content: center; gap: 1rem; margin-top: 1.5rem; }
            .nav-link { text-decoration: none; font-weight: bold; padding: 0.5rem 1.2rem; border-radius: 8px; transition: all 0.2s; }
            .nav-link.home { color: var(--mauve); border: 1px solid var(--mauve); }
            .nav-link.home:hover { background-color: rgba(203, 166, 247, 0.1); }
            .nav-link.resume { color: var(--green); border: 1px solid var(--green); }
            .nav-link.resume:hover { background-color: rgba(166, 227, 161, 0.1); }
            .nav-link.projects { color: var(--red); border: 1px solid var(--red); }
            .nav-link.projects:hover { background-color: rgba(210, 15, 57, 0.1) }



            /* Card styling reused across pages */
            .card { background-color: var(--mantle); border-radius: 16px; padding: 2.5rem; box-shadow: 0 4px 15px rgba(0,0,0,0.2); }
            "
        </style>

        <Router>
            <main>
                <div class="container">
                    <header class="header">
                        <h1>"✨ My personal Webpage ✨"</h1>
                        <p>"Aerospace Engineering @ IST | RED Rocketry | Embedded Rustacean "</p>

                        <nav class="nav-bar">
                            <a href="/" class="nav-link home">"👾Home"</a>
                            <a href="/resume" class="nav-link resume">"Resume"</a>
                            <a href="/projects" class="nav-link projects"> "Projects" </a>
                        </nav>
                    </header>

                    // The Routes component swaps out the page below the header!
                    <Routes fallback=|| view! { <h2>"404 - Page not found in this orbit."</h2> }>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/resume") view=Resume />
                        <Route path=path!("/projects") view=Projects />
                    </Routes>
                </div>
            </main>
        </Router>
    }
}
