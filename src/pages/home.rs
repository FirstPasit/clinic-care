use yew::prelude::*;
use crate::store::Store;
use yew_router::prelude::Link;
use crate::Route;
use chrono::prelude::*;

fn format_thai_date() -> String {
    let now = Local::now();
    let thai_days = ["อาทิตย์", "จันทร์", "อังคาร", "พุธ", "พฤหัสบดี", "ศุกร์", "เสาร์"];
    let thai_months = ["", "มกราคม", "กุมภาพันธ์", "มีนาคม", "เมษายน", "พฤษภาคม", "มิถุนายน", 
                       "กรกฎาคม", "สิงหาคม", "กันยายน", "ตุลาคม", "พฤศจิกายน", "ธันวาคม"];
    
    let day_name = thai_days[now.weekday().num_days_from_sunday() as usize];
    let day = now.day();
    let month = thai_months[now.month() as usize];
    let year = now.year() + 543; // Buddhist Era
    
    format!("วัน{} ที่ {} {} พ.ศ. {}", day_name, day, month, year)
}

#[function_component(Home)]
pub fn home() -> Html {
    let patients = Store::get_patients();
    let records = Store::get_records();
    let low_stock_drugs = Store::get_low_stock_drugs();
    let expiring_drugs = Store::get_expiring_drugs();
    let today_appointments = Store::get_today_appointments();
    
    let total_patients = patients.len();
    let total_records = records.len();
    let total_revenue: f64 = records.iter().map(|r| r.price).sum();
    let today_revenue = Store::get_today_revenue();
    let today_revenue = if today_revenue == 0.0 { 0.0_f64 } else { today_revenue.max(0.0) };
    let total_revenue = if total_revenue == 0.0 { 0.0_f64 } else { total_revenue.max(0.0) };
    let today_patients_count = Store::get_today_patient_count();
    
    let current_date = format_thai_date();

    html! {
        <>
            // Header with greeting
            <div class="dashboard-header">
                <div class="dashboard-greeting">
                    <h1 class="page-title">{ "🏠 สวัสดี" }</h1>
                    <p class="page-subtitle">{ "ยินดีต้อนรับเข้าสู่ระบบคลินิก" }</p>
                </div>
                <div class="dashboard-date">
                    <div class="date-icon">{ "📅" }</div>
                    <div class="date-text">{ current_date }</div>
                </div>
            </div>
            
            // Alerts section - compact
            { if !low_stock_drugs.is_empty() || !expiring_drugs.is_empty() {
                html! {
                    <div class="alerts-row">
                        { if !low_stock_drugs.is_empty() {
                            html! {
                                <div class="alert-compact warning">
                                    <span class="alert-icon">{ "📦" }</span>
                                    <span>{ format!("ยาสต็อกต่ำ {} รายการ", low_stock_drugs.len()) }</span>
                                    <Link<Route> to={Route::Drugs} classes="alert-link">{ "ดู →" }</Link<Route>>
                                </div>
                            }
                        } else { html! {} }}
                        { if !expiring_drugs.is_empty() {
                            html! {
                                <div class="alert-compact error">
                                    <span class="alert-icon">{ "⏰" }</span>
                                    <span>{ format!("ยาใกล้หมดอายุ {} รายการ", expiring_drugs.len()) }</span>
                                    <Link<Route> to={Route::Drugs} classes="alert-link">{ "ดู →" }</Link<Route>>
                                </div>
                            }
                        } else { html! {} }}
                    </div>
                }
            } else { html! {} }}
            
            // Today's Stats - Clean 4-column grid
            <div class="stats-grid-4">
                <div class="stat-card-minimal">
                    <div class="stat-icon accent">{ "👤" }</div>
                    <div class="stat-info">
                        <div class="stat-value">{ today_patients_count }</div>
                        <div class="stat-label">{ "ผู้ป่วยวันนี้" }</div>
                    </div>
                </div>
                
                <div class="stat-card-minimal">
                    <div class="stat-icon success">{ "💵" }</div>
                    <div class="stat-info">
                        <div class="stat-value">{ format!("฿{:.0}", today_revenue) }</div>
                        <div class="stat-label">{ "รายได้วันนี้" }</div>
                    </div>
                </div>
                
                <div class="stat-card-minimal">
                    <div class="stat-icon">{ "👥" }</div>
                    <div class="stat-info">
                        <div class="stat-value">{ total_patients }</div>
                        <div class="stat-label">{ "ผู้ป่วยทั้งหมด" }</div>
                    </div>
                </div>
                
                <div class="stat-card-minimal">
                    <div class="stat-icon success">{ "📈" }</div>
                    <div class="stat-info">
                        <div class="stat-value">{ format!("฿{:.0}", total_revenue) }</div>
                        <div class="stat-label">{ "รายได้รวม" }</div>
                    </div>
                </div>
            </div>
            
            // Two column layout: Quick Actions + Today's Appointments
            <div class="dashboard-grid">
                // Quick Actions
                <div class="card">
                    <div class="card-header">
                        <h3 class="card-title">{ "⚡ ทางลัด" }</h3>
                    </div>
                    <div class="quick-actions-grid">
                        <Link<Route> to={Route::Search} classes="quick-btn">
                            <span class="quick-btn-icon">{ "🔍" }</span>
                            <span>{ "ค้นหาผู้ป่วย" }</span>
                        </Link<Route>>
                        
                        <Link<Route> to={Route::Register} classes="quick-btn primary">
                            <span class="quick-btn-icon">{ "➕" }</span>
                            <span>{ "ลงทะเบียนใหม่" }</span>
                        </Link<Route>>
                        
                        <Link<Route> to={Route::Drugs} classes="quick-btn">
                            <span class="quick-btn-icon">{ "💊" }</span>
                            <span>{ "คลังยา" }</span>
                        </Link<Route>>
                        
                        <Link<Route> to={Route::Report} classes="quick-btn">
                            <span class="quick-btn-icon">{ "📊" }</span>
                            <span>{ "รายงาน" }</span>
                        </Link<Route>>
                    </div>
                </div>
                
                // Today's Appointments
                <div class="card">
                    <div class="card-header">
                        <h3 class="card-title">{ "🗓️ นัดหมายวันนี้" }</h3>
                        <Link<Route> to={Route::Appointments} classes="btn btn-ghost btn-sm">
                            { "ดูทั้งหมด →" }
                        </Link<Route>>
                    </div>
                    { if today_appointments.is_empty() {
                        html! {
                            <div class="empty-state-minimal">
                                <span class="empty-icon">{ "✨" }</span>
                                <span>{ "ไม่มีนัดหมายวันนี้" }</span>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="appointments-list">
                                { for today_appointments.iter().take(5).map(|apt| {
                                    let patient = patients.iter().find(|p| p.id == apt.patient_id);
                                    let patient_name = patient.map(|p| format!("{} {}", p.first_name, p.last_name))
                                        .unwrap_or_else(|| "-".to_string());
                                    html! {
                                        <div class="appointment-item">
                                            <div class="appointment-time">
                                                <span class="time-icon">{ "🕐" }</span>
                                                { &apt.time }
                                            </div>
                                            <div class="appointment-info">
                                                <div class="appointment-name">{ patient_name }</div>
                                                <div class="appointment-reason">{ &apt.reason }</div>
                                            </div>
                                            <span class="badge badge-warning">{ "รอ" }</span>
                                        </div>
                                    }
                                })}
                                { if today_appointments.len() > 5 {
                                    html! {
                                        <div class="more-link">
                                            { format!("+ อีก {} นัด", today_appointments.len() - 5) }
                                        </div>
                                    }
                                } else { html! {} }}
                            </div>
                        }
                    }}
                </div>
            </div>
            
            // Statistics Card
            { if total_records > 0 {
                html! {
                    <div class="card mt-4">
                        <div class="card-header">
                            <h3 class="card-title">{ "📊 สถิติการรักษา" }</h3>
                        </div>
                        <div class="stats-row">
                            <div class="stat-box">
                                <div class="stat-box-value">{ total_records }</div>
                                <div class="stat-box-label">{ "การรักษาทั้งหมด" }</div>
                            </div>
                            <div class="stat-divider"></div>
                            <div class="stat-box">
                                <div class="stat-box-value text-success">
                                    { format!("฿{:.0}", if total_records > 0 { total_revenue / total_records as f64 } else { 0.0 }) }
                                </div>
                                <div class="stat-box-label">{ "เฉลี่ยต่อครั้ง" }</div>
                            </div>
                        </div>
                    </div>
                }
            } else { html! {} }}
        </>
    }
}

