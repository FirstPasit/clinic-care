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
    
    let total_patients = patients.len();
    let total_records = records.len();
    let total_revenue: f64 = records.iter().map(|r| r.price).sum();
    let today_revenue = Store::get_today_revenue();
    let today_patients = Store::get_today_patient_count();
    
    let current_date = format_thai_date();

    html! {
        <>
            <div class="page-header">
                <div>
                    <h1 class="page-title">{ "🏠 แดชบอร์ด" }</h1>
                    <p class="page-subtitle">{ "ยินดีต้อนรับ สรุปข้อมูลคลินิก" }</p>
                </div>
                <div class="current-date" style="text-align: right; padding: 1rem; background: white; color: black; border: 1px solid #eee; border-radius: var(--radius-lg); font-size: 1.2rem; box-shadow: var(--shadow-sm);">
                    <div style="font-size: 0.9rem; color: #666;">{ "📅 วันนี้" }</div>
                    <div style="font-weight: bold; font-size: 1.3rem;">{ current_date }</div>
                </div>
            </div>
            
            // Low stock alert
            { if !low_stock_drugs.is_empty() {
                html! {
                    <div class="alert alert-warning">
                        <span class="alert-icon">{ "⚠️" }</span>
                        <span>{ format!("มียาใกล้หมด {} รายการ - กรุณาสั่งซื้อเพิ่ม", low_stock_drugs.len()) }</span>
                        <Link<Route> to={Route::Drugs} classes="btn btn-warning btn-sm">
                            { "ดูรายการ →" }
                        </Link<Route>>
                    </div>
                }
            } else { html! {} }}
            
            // Stats Grid
            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-card-icon accent">{ "📅" }</div>
                    <div class="stat-card-value">{ today_patients }</div>
                    <div class="stat-card-label">{ "ผู้ป่วยวันนี้" }</div>
                </div>
                
                <div class="stat-card">
                    <div class="stat-card-icon success">{ "💰" }</div>
                    <div class="stat-card-value">{ format!("฿{:.0}", today_revenue) }</div>
                    <div class="stat-card-label">{ "รายได้วันนี้" }</div>
                </div>
                
                <div class="stat-card">
                    <div class="stat-card-icon accent">{ "👥" }</div>
                    <div class="stat-card-value">{ total_patients }</div>
                    <div class="stat-card-label">{ "ผู้ป่วยทั้งหมด" }</div>
                </div>
                
                <div class="stat-card">
                    <div class="stat-card-icon success">{ "📊" }</div>
                    <div class="stat-card-value">{ format!("฿{:.0}", total_revenue) }</div>
                    <div class="stat-card-label">{ "รายได้รวม" }</div>
                </div>
            </div>
            
            // Quick Actions - BIG BUTTONS
            <div class="card">
                <div class="card-header">
                    <div>
                        <h3 class="card-title">{ "⚡ ทางลัด" }</h3>
                        <p class="card-subtitle">{ "กดปุ่มเพื่อเริ่มทำงานเลย" }</p>
                    </div>
                </div>
                
                <div class="quick-actions">
                    <Link<Route> to={Route::Search} classes="quick-action">
                        <div class="quick-action-icon">{ "🔍" }</div>
                        <span class="quick-action-label">{ "ค้นหาผู้ป่วย" }</span>
                    </Link<Route>>
                    
                    <Link<Route> to={Route::Register} classes="quick-action">
                        <div class="quick-action-icon">{ "➕" }</div>
                        <span class="quick-action-label">{ "ลงทะเบียนใหม่" }</span>
                    </Link<Route>>
                    
                    <Link<Route> to={Route::Drugs} classes="quick-action">
                        <div class="quick-action-icon">{ "💊" }</div>
                        <span class="quick-action-label">{ "คลังยา" }</span>
                    </Link<Route>>
                </div>
            </div>
            
            // Treatment Stats
            { if total_records > 0 {
                html! {
                    <div class="card mt-6">
                        <div class="card-header">
                            <h3 class="card-title">{ "📊 สถิติการรักษา" }</h3>
                        </div>
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <div class="history-item-label">{ "จำนวนการรักษาทั้งหมด" }</div>
                                <div style="font-size: 2rem; font-weight: 700;">{ total_records } { " ครั้ง" }</div>
                            </div>
                            <div>
                                <div class="history-item-label">{ "รายได้เฉลี่ยต่อครั้ง" }</div>
                                <div style="font-size: 2rem; font-weight: 700; color: var(--color-success);">
                                    { format!("฿{:.0}", if total_records > 0 { total_revenue / total_records as f64 } else { 0.0 }) }
                                </div>
                            </div>
                        </div>
                    </div>
                }
            } else { html! {} }}
        </>
    }
}
