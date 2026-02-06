use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::store::Store;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let route = use_route::<Route>();
    let today_appointments = Store::get_today_appointments();
    let appointment_count = today_appointments.len();
    let low_stock_count = Store::get_low_stock_drugs().len();
    
    // Helper to check if route is active
    let is_active = |target: &Route| -> bool {
        match &route {
            Some(r) => std::mem::discriminant(r) == std::mem::discriminant(target),
            None => false,
        }
    };
    
    let nav_class = |target: &Route| -> Classes {
        if is_active(target) {
            classes!("nav-link", "active")
        } else {
            classes!("nav-link")
        }
    };

    html! {
        <aside class="sidebar">
            <div class="sidebar-header">
                <div class="sidebar-logo">
                    <div class="sidebar-logo-icon">{ "🏥" }</div>
                    <span class="sidebar-logo-text">{ "คลินิก" }</span>
                </div>
            </div>
            
            <nav class="sidebar-nav">
                <div class="nav-section">
                    <div class="nav-section-title">{ "หน้าหลัก" }</div>
                    <Link<Route> to={Route::Home} classes={nav_class(&Route::Home)}>
                        <span class="nav-link-icon">{ "🏠" }</span>
                        { "แดชบอร์ด" }
                    </Link<Route>>
                </div>
                
                <div class="nav-section">
                    <div class="nav-section-title">{ "ผู้ป่วย" }</div>
                    <Link<Route> to={Route::Register} classes={nav_class(&Route::Register)}>
                        <span class="nav-link-icon">{ "➕" }</span>
                        { "ลงทะเบียนใหม่" }
                    </Link<Route>>
                    <Link<Route> to={Route::Search} classes={nav_class(&Route::Search)}>
                        <span class="nav-link-icon">{ "🔍" }</span>
                        { "ค้นหาผู้ป่วย" }
                    </Link<Route>>
                </div>
                
                <div class="nav-section">
                    <div class="nav-section-title">{ "คลังยา" }</div>
                    <Link<Route> to={Route::Drugs} classes={nav_class(&Route::Drugs)}>
                        <span class="nav-link-icon">{ "💊" }</span>
                        <span class="nav-link-content">
                            { "จัดการยา" }
                            { if low_stock_count > 0 {
                                html! { <span class="nav-badge warning">{ low_stock_count }</span> }
                            } else { html! {} }}
                        </span>
                    </Link<Route>>
                </div>
                
                <div class="nav-section">
                    <div class="nav-section-title">{ "การเงิน" }</div>
                    <Link<Route> to={Route::Report} classes={nav_class(&Route::Report)}>
                        <span class="nav-link-icon">{ "📊" }</span>
                        { "รายงานรายเดือน" }
                    </Link<Route>>
                    <Link<Route> to={Route::Expenses} classes={nav_class(&Route::Expenses)}>
                        <span class="nav-link-icon">{ "💰" }</span>
                        { "ค่าใช้จ่าย" }
                    </Link<Route>>
                </div>
                
                <div class="nav-section">
                    <div class="nav-section-title">{ "นัดหมาย" }</div>
                    <Link<Route> to={Route::Appointments} classes={nav_class(&Route::Appointments)}>
                        <span class="nav-link-icon">{ "🗓️" }</span>
                        <span class="nav-link-content">
                            { "นัดหมายผู้ป่วย" }
                            { if appointment_count > 0 {
                                html! { <span class="nav-badge">{ appointment_count }</span> }
                            } else { html! {} }}
                        </span>
                    </Link<Route>>
                </div>
                
                <div class="nav-section">
                    <div class="nav-section-title">{ "ระบบ" }</div>
                    <Link<Route> to={Route::Settings} classes={nav_class(&Route::Settings)}>
                        <span class="nav-link-icon">{ "⚙️" }</span>
                        { "ตั้งค่าคลินิก" }
                    </Link<Route>>
                </div>
            </nav>
            
            // Version footer
            <div class="sidebar-footer">
                <div class="version-info">{ "v1.4.1" }</div>
            </div>
        </aside>
    }
}
