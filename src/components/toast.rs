use yew::prelude::*;
use gloo::timers::callback::Timeout;

#[derive(Clone, PartialEq)]
pub enum ToastType {
    Success,
    Error,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToastProps {
    pub message: String,
    pub toast_type: ToastType,
    pub on_close: Callback<()>,
}

#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    let visible = use_state(|| true);
    
    // Auto close after 3 seconds
    {
        let visible = visible.clone();
        let on_close = props.on_close.clone();
        use_effect_with((), move |_| {
            let timeout = Timeout::new(3000, move || {
                visible.set(false);
                on_close.emit(());
            });
            timeout.forget();
            || ()
        });
    }
    
    let (icon, class) = match props.toast_type {
        ToastType::Success => ("✅", "toast-success"),
        ToastType::Error => ("❌", "toast-error"),
    };
    
    if !*visible {
        return html! {};
    }
    
    let on_close = props.on_close.clone();
    
    html! {
        <div class={classes!("toast", class)}>
            <span class="toast-icon">{ icon }</span>
            <span class="toast-message">{ &props.message }</span>
            <button class="toast-close" onclick={move |_| on_close.emit(())}>{ "✕" }</button>
        </div>
    }
}

// Toast Container for multiple toasts
#[derive(Clone, PartialEq)]
pub struct ToastMessage {
    pub id: u32,
    pub message: String,
    pub toast_type: ToastType,
}

// Global toast state using context
#[derive(Clone, PartialEq, Default)]
pub struct ToastState {
    pub toasts: Vec<ToastMessage>,
    pub next_id: u32,
}

pub enum ToastAction {
    Add(String, ToastType),
    Remove(u32),
}

impl Reducible for ToastState {
    type Action = ToastAction;
    
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            ToastAction::Add(message, toast_type) => {
                let mut toasts = self.toasts.clone();
                toasts.push(ToastMessage {
                    id: self.next_id,
                    message,
                    toast_type,
                });
                std::rc::Rc::new(ToastState {
                    toasts,
                    next_id: self.next_id + 1,
                })
            }
            ToastAction::Remove(id) => {
                let toasts = self.toasts.iter()
                    .filter(|t| t.id != id)
                    .cloned()
                    .collect();
                std::rc::Rc::new(ToastState {
                    toasts,
                    next_id: self.next_id,
                })
            }
        }
    }
}

pub type ToastContext = UseReducerHandle<ToastState>;

#[derive(Properties, PartialEq)]
pub struct ToastProviderProps {
    pub children: Children,
}

#[function_component(ToastProvider)]
pub fn toast_provider(props: &ToastProviderProps) -> Html {
    let state = use_reducer(ToastState::default);
    
    html! {
        <ContextProvider<ToastContext> context={state.clone()}>
            { props.children.clone() }
            <div class="toast-container">
                { for state.toasts.iter().map(|t| {
                    let id = t.id;
                    let state = state.clone();
                    html! {
                        <Toast 
                            message={t.message.clone()} 
                            toast_type={t.toast_type.clone()}
                            on_close={Callback::from(move |_| {
                                state.dispatch(ToastAction::Remove(id));
                            })}
                        />
                    }
                })}
            </div>
        </ContextProvider<ToastContext>>
    }
}

// Hook to use toast
#[hook]
pub fn use_toast() -> Callback<(String, ToastType)> {
    let ctx = use_context::<ToastContext>();
    
    Callback::from(move |(message, toast_type): (String, ToastType)| {
        if let Some(ref ctx) = ctx {
            ctx.dispatch(ToastAction::Add(message, toast_type));
        }
    })
}
