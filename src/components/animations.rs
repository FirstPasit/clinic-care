use yew::prelude::*;
use gloo::timers::callback::Timeout;

#[derive(Properties, PartialEq, Clone)]
pub struct SuccessAnimationProps {
    pub message: String,
    #[prop_or_default]
    pub submessage: Option<String>,
    pub on_complete: Callback<()>,
    #[prop_or(2000)]
    pub duration_ms: u32,
}

#[function_component(SuccessAnimation)]
pub fn success_animation(props: &SuccessAnimationProps) -> Html {
    let on_complete = props.on_complete.clone();
    let duration = props.duration_ms;
    
    // Auto-close after duration
    use_effect_with((), move |_| {
        let timeout = Timeout::new(duration, move || {
            on_complete.emit(());
        });
        timeout.forget();
        || ()
    });
    
    html! {
        <div class="success-animation-overlay">
            <div class="success-checkmark"></div>
            <div class="success-message">{ &props.message }</div>
            { if let Some(sub) = &props.submessage {
                html! { <div class="success-submessage">{ sub }</div> }
            } else {
                html! {}
            }}
        </div>
    }
}

// Empty State component with floating illustration
#[derive(Properties, PartialEq, Clone)]
pub struct EmptyStateProps {
    pub emoji: String,
    pub title: String,
    pub description: String,
    #[prop_or_default]
    pub action_label: Option<String>,
    #[prop_or_default]
    pub on_action: Option<Callback<()>>,
}

#[function_component(EmptyState)]
pub fn empty_state(props: &EmptyStateProps) -> Html {
    let on_click = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| {
            if let Some(cb) = &on_action {
                cb.emit(());
            }
        })
    };
    
    html! {
        <div class="empty-state-interactive">
            <div class="empty-state-illustration">
                <span class="emoji">{ &props.emoji }</span>
            </div>
            <h3 class="empty-state-title">{ &props.title }</h3>
            <p class="empty-state-description">{ &props.description }</p>
            { if let Some(label) = &props.action_label {
                html! {
                    <div class="empty-state-action">
                        <button class="btn btn-primary btn-lg" onclick={on_click}>
                            { label }
                        </button>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
