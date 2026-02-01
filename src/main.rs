use yew::prelude::*;
use yew_router::prelude::*;

mod models;
mod store;
mod pages;
mod components;

use pages::{Home, Register, Search, Treatment, History, Document, NotFound, Drugs, Sticker, Report, Settings, EditPatient, Expenses, Appointments};
use components::{ToastProvider, Sidebar};
use store::Store;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/register")]
    Register,
    #[at("/search")]
    Search,
    #[at("/treatment/:id")]
    Treatment { id: String },
    #[at("/history/:id")]
    History { id: String },
    #[at("/edit-patient/:id")]
    EditPatient { id: String },
    #[at("/document/:doc_type/:id")]
    Document { doc_type: String, id: String },
    #[at("/drugs")]
    Drugs,
    #[at("/sticker/:record_id/:drug_index")]
    Sticker { record_id: String, drug_index: usize },
    #[at("/report")]
    Report,
    #[at("/expenses")]
    Expenses,
    #[at("/appointments")]
    Appointments,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Register => html! { <Register /> },
        Route::Search => html! { <Search /> },
        Route::Treatment { id } => html! { <Treatment id={id} /> },
        Route::History { id } => html! { <History id={id} /> },
        Route::EditPatient { id } => html! { <EditPatient patient_id={id} /> },
        Route::Document { doc_type, id } => html! { <Document doc_type={doc_type} id={id} /> },
        Route::Drugs => html! { <Drugs /> },
        Route::Sticker { record_id, drug_index } => html! { <Sticker record_id={record_id} drug_index={drug_index} /> },
        Route::Report => html! { <Report /> },
        Route::Expenses => html! { <Expenses /> },
        Route::Appointments => html! { <Appointments /> },
        Route::Settings => html! { <Settings /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    // Apply font size on mount
    use_effect_with((), |_| {
        let settings = Store::get_settings();
        apply_font_size(&settings.font_size);
        || ()
    });
    
    html! {
        <ToastProvider>
            <BrowserRouter>
                <div class="app-layout">
                    <Sidebar />
                    
                    <main class="main-content">
                        <div class="page-container">
                            <Switch<Route> render={switch} />
                        </div>
                    </main>
                </div>
            </BrowserRouter>
        </ToastProvider>
    }
}

fn apply_font_size(size: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(html) = document.document_element() {
                // Remove all font classes
                let _ = html.class_list().remove_3("font-medium", "font-large", "font-xlarge");
                
                // Add the correct one
                let class = match size {
                    "medium" => "font-medium",
                    "large" => "font-large",
                    "xlarge" => "font-xlarge",
                    _ => "font-large",
                };
                let _ = html.class_list().add_1(class);
            }
        }
    }
}

// Export for use in other modules
pub fn set_font_size(size: &str) {
    apply_font_size(size);
}

fn main() {
    yew::Renderer::<App>::new().render();
}
