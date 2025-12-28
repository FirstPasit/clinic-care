use yew::prelude::*;
use yew_router::prelude::Link;
use crate::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="empty-state" style="min-height: 60vh; display: flex; flex-direction: column; justify-content: center;">
            <div class="empty-state-icon">{ "404" }</div>
            <h1 class="empty-state-title">{ "ไม่พบหน้าที่คุณต้องการ" }</h1>
            <p class="empty-state-text">{ "หน้านี้ไม่มีอยู่หรือถูกย้ายไปแล้ว" }</p>
            <Link<Route> to={Route::Home} classes="btn btn-primary">
                { "กลับหน้าหลัก" }
            </Link<Route>>
        </div>
    }
}
