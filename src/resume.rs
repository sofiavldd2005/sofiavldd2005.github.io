use leptos::prelude::*;

#[component]
pub fn Resume() -> impl IntoView {
    view! {
        <style>
            "
            .resume { border: 2px solid var(--green); }
            .resume h2 { color: var(--green); margin: 0 0 1.5rem 0; text-align: center; font-size: 1.8rem; }
            .resume-grid { display: grid; gap: 3rem; }
            @media (min-width: 768px) { .resume-grid { grid-template-columns: 1fr 1fr; } }
            .section-heading { border-bottom: 1px solid var(--surface-border); padding-bottom: 0.5rem; margin-bottom: 1.5rem; font-size: 1.2rem; margin-top: 0; }
            .col-left .section-heading { color: var(--yellow); }
            .col-right .section-heading { color: var(--red); }
            .experience-list { list-style-type: none; padding: 0; margin: 0; }
            .experience-list li { margin-bottom: 1.5rem; }
            .role-title { color: var(--text); font-weight: bold; display: block; margin-bottom: 0.3rem; font-size: 1.05rem; }
            .role-desc { color: var(--subtext); font-size: 0.95rem; line-height: 1.5; }
            .skills-container { display: flex; flex-wrap: wrap; gap: 0.6rem; }
            .skill-badge { background-color: var(--surface); color: #f5e0dc; padding: 0.4rem 0.8rem; border-radius: 8px; font-size: 0.9rem; border: 1px solid var(--surface-border); font-weight: bold; }
            
            .download-btn { background-color: var(--pink); color: var(--base); padding: 0.7rem 1.5rem; border-radius: 8px; text-decoration: none; font-weight: bold; display: inline-block; transition: opacity 0.2s; margin-bottom: 2rem; }
            .download-btn:hover { opacity: 0.8; }
            "
        </style>
        <section class="card resume">
            <h2>"Service Record & Tech Stack"</h2>

            <div style="text-align: center;">
                // Make sure your PDF is inside the 'public' folder of your project!
                <a href="/Sofia_Duarte_CV.pdf" download="Sofia_Duarte_CV.pdf" class="download-btn">
                    "Download Full CV"
                </a>
            </div>

            <div class="resume-grid">
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

                <div class="col-right">
                    <h3 class="section-heading">"💻 Hardware & Embedded"</h3>
                    <div class="skills-container">
                        <SkillBadge text="C/C++" /><SkillBadge text="Embedded Rust" /><SkillBadge text="RISC-V" />
                        <SkillBadge text="STM32 & ESP32" /><SkillBadge text="FreeRTOS" /><SkillBadge text="KiCad" />
                    </div>

                    <h3 class="section-heading" style="margin-top: 2.5rem;">"⚙️ Systems & Analysis"</h3>
                    <div class="skills-container">
                        <SkillBadge text="Async/Await (Embassy)" /><SkillBadge text="Python" /><SkillBadge text="MATLAB" />
                        <SkillBadge text="GitHub Actions CI/CD" /><SkillBadge text="Linux/Bash" />
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn SkillBadge(text: &'static str) -> impl IntoView {
    view! { <span class="skill-badge">{text}</span> }
}
