use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <style>
            "
            .projects-header {
                text-align: center;
                color: var(--red);
                margin: 2rem 0;
                font-size: 2.2rem;
            }

            .projects-grid {
                display: flex;
                flex-direction: column;
                gap: 1.5rem;
                margin: 0 auto 3rem auto; /* The 'auto' on left/right centers the column */
                max-width: 1000px; /* Keeps the cards from getting too wide on big screens */
            }

            .project-card {
                /* Make the anchor tag behave like a block container */
                display: block;
                text-decoration: none; 
                
                background-color: rgba(30, 30, 46, 0.55);
                backdrop-filter: blur(10px);
                -webkit-backdrop-filter: blur(5px);
                border: 1px solid var(--surface-border);
                border-radius: 16px;
                padding: 1.5rem;
                transition: transform 0.2s ease, border-color 0.2s ease, box-shadow 0.2s ease;
                text-align: left;
                cursor: pointer;
            }

            .project-card:hover {
                transform: translateY(-5px);
                border-color: var(--red);
                box-shadow: 0 8px 20px rgba(203, 166, 247, 0.15);
            }

            .project-title {
                color: var(--red);
                font-size: 1.4rem;
                margin-top: 0;
                margin-bottom: 0.5rem;
                text-align: center;
            }

            .project-tech {
                color: var(--green);
                font-size: 0.9rem;
                font-weight: bold;
                margin-bottom: 1rem;
                text-align: center;
            }

            .project-desc {
                color: var(--text);
                line-height: 1.6;
                font-size: 1rem;
                margin-bottom: 0;
            }
            "
        </style>

        <div>
            <h2 class="projects-header">"My Projects"</h2>
            <div class="projects-grid">


                <a href="https://github.com/sofiavldd2005/sofiavldd2005.github.io" target="_blank" rel="noopener noreferrer" class="project-card">
                    <h3 class="project-title">"WebAssembly Portfolio"</h3>
                    <div class="project-tech">"Rust • Leptos • WebAssembly • CSS"</div>
                    <p class="project-desc">"Designed and developed a fully responsive, client-side web application entirely in Rust. Features WASM compilation, client-side routing, and a custom glassmorphism UI."</p>
                </a>

                <a href="https://github.com/sofiavldd2005?tab=repositories" target="_blank" rel="noopener noreferrer" class="project-card">
                    <h3 class="project-title">"Rust HIL Framework"</h3>
                    <div class="project-tech">"Rust • Hardware-in-the-Loop"</div>
                    <p class="project-desc">"It is a WIP.  My goal is to  build Hardware-in-the-Loop testing framework to simulate flight and validate critical systems."</p>
                </a>
                 <a href="https://github.com/sofiavldd2005/Nvim_setup" target="_blank" rel="noopener noreferrer" class="project-card">
                    <h3 class="project-title">"My Neovim Setup"</h3>
                    <div class="project-tech">"Lua • Neovim • LazyVim  "</div>
                    <p class="project-desc">"In case you are wondering what is my setup to code, here's my neovim cofigs built on top of LazyVim."</p>
                </a>



                <a href="https://github.com/RED-PT" target="_blank" rel="noopener noreferrer" class="project-card">
                    <h3 class="project-title">"RED Flight Software"</h3>
                    <div class="project-tech">"C • FreeRTOS  • STM32 "</div>
                    <p class="project-desc">"Developed flight software architecture for RED's first Hybrid Rocket, ensuring telemetry and real-time execution for the avionics."</p>
                    <p class="project-desc">"Participated in the EuRoc 2025 Competition"</p>
                </a>

                <a href="https://github.com/RED-PT/R_D_Software_Inclita_25_26" target="_blank" rel="noopener noreferrer" class="project-card">
                    <h3 class="project-title">"Pilot Project in Embedded Rust"</h3>
                    <div class="project-tech">"Embedded • Embassy Framework • Rust"</div>
                    <p class="project-desc">"Programmed the flight software for the Inclita Geração Cansat launchs, in order to show Rust as an option for flight Software."</p>
                </a>
                <a href="https://github.com/sofiavldd2005/Learning-ROS2" target="_blank" rel="noopener noreferrer" class="project-card">
                    <h3 class="project-title">"Learning ROS2"</h3>
                    <div class="project-tech">"ROS2 • C++ • Python  • Robotics"</div>
                    <p class="project-desc">"Currently I am just playing with ROS2, in order to be able do venture into robotics in the future."</p>
                </a>


            </div>
        </div>
    }
}
