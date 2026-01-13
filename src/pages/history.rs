use yew::prelude::*;
use crate::models::Patient;
use crate::store::Store;
use chrono::prelude::*;
use yew_router::prelude::{Link, use_navigator};
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(History)]
pub fn history(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    
    let patient = use_state(|| -> Option<Patient> {
        Store::get_patients().into_iter().find(|p| p.id == props.id)
    });
    
    let records = use_state(|| Store::get_records_by_patient(&props.id));

    if patient.is_none() {
        return html! {
            <div class="empty-state">
                <div class="empty-state-icon">{ "❓" }</div>
                <h3 class="empty-state-title">{ "ไม่พบข้อมูลผู้ป่วย" }</h3>
                <Link<Route> to={Route::Search} classes="btn btn-primary btn-lg">
                    { "← กลับไปค้นหา" }
                </Link<Route>>
            </div>
        };
    }
    let p = patient.as_ref().unwrap();

    // Sort records by date descending
    let sorted_records = {
        let mut r = (*records).clone();
        r.sort_by(|a, b| b.date.cmp(&a.date));
        r
    };

    html! {
        <>
            <div class="page-header flex justify-between items-center flex-wrap gap-4">
                <div>
                    <h1 class="page-title">{ "📋 ประวัติการรักษา" }</h1>
                    <p class="page-subtitle">{ format!("{}{} {} • HN: {}", p.title, p.first_name, p.last_name, p.hn) }</p>
                </div>
                <div class="flex gap-3">
                    <Link<Route> to={Route::EditPatient { id: props.id.clone() }} classes="btn btn-warning btn-lg">
                        { "✏️ แก้ไขข้อมูล" }
                    </Link<Route>>
                    <button class="btn btn-danger" onclick={
                        let id = props.id.clone();
                        let navigator = navigator.clone();
                        move |_| {
                            if web_sys::window().unwrap().confirm_with_message("⚠️ ยืนยันการลบข้อมูลผู้ป่วยและประวัติทั้งหมด? (ไม่สามารถกู้คืนได้)").unwrap() {
                                Store::delete_patient(&id);
                                navigator.push(&Route::Search);
                            }
                        }
                    }>
                        { "🗑️ ลบ" }
                    </button>
                    <Link<Route> to={Route::Search} classes="btn btn-secondary btn-lg">
                        { "← กลับ" }
                    </Link<Route>>
                    <Link<Route> to={Route::Treatment { id: props.id.clone() }} classes="btn btn-primary btn-lg">
                        { "➕ รักษาใหม่" }
                    </Link<Route>>
                </div>
            </div>
            
            { if sorted_records.is_empty() {
                html! {
                    <div class="card">
                        <div class="empty-state">
                            <div class="empty-state-icon">{ "📋" }</div>
                            <h3 class="empty-state-title">{ "ไม่มีประวัติการรักษา" }</h3>
                            <p class="empty-state-text">{ "ผู้ป่วยรายนี้ยังไม่เคยเข้ารับการรักษา" }</p>
                            <Link<Route> to={Route::Treatment { id: props.id.clone() }} classes="btn btn-primary btn-lg">
                                { "💉 เริ่มการรักษา" }
                            </Link<Route>>
                        </div>
                    </div>
                }
            } else {
                html! {
                    <div class="history-timeline">
                        { for sorted_records.iter().map(|r| {
                            let date_str = r.date.with_timezone(&Local).format("%d/%m/%Y เวลา %H:%M น.").to_string();
                            let id = r.id.clone();
                            let navigator = navigator.clone(); // Clone for this iteration
                            
                            html! {
                                <div class="history-item">
                                    <div class="history-item-header">
                                        <div class="history-item-date">{ "📅 " }{ date_str }</div>
                                        <div class="history-item-price">{ format!("฿{:.2}", r.price) }</div>
                                    </div>
                                    
                                    <div class="history-item-details">
                                        <div>
                                            <div class="history-item-label">{ "การวินิจฉัย" }</div>
                                            <div style="font-weight: 600;">{ &r.diagnosis }</div>
                                        </div>
                                        <div>
                                            <div class="history-item-label">{ "อาการ" }</div>
                                            <div>{ &r.symptoms }</div>
                                        </div>
                                    </div>
                                    
                                    { if !r.prescriptions.is_empty() {
                                        let navigator = navigator.clone(); // Clone for rx list
                                        html! {
                                            <div class="history-item-rx">
                                                <div class="history-item-label">{ "💊 รายการยา" }</div>
                                                <ul style="padding-left: 1.5rem; margin: 0.5rem 0 0;">
                                                    { for r.prescriptions.iter().enumerate().map(|(idx, rx)| {
                                                        let navigator = navigator.clone(); // Clone for this item
                                                        let record_id = id.clone();
                                                        html! {
                                                            <li style="margin-bottom: 0.75rem; display: flex; justify-content: space-between; align-items: center;">
                                                                <div>
                                                                    <strong>{ &rx.name }</strong>
                                                                    { format!(" - {} ({})", rx.amount, 
                                                                        if rx.timing.is_empty() { "หลังอาหาร" } else { &rx.timing }) }
                                                                    { if rx.morning > 0 || rx.noon > 0 || rx.evening > 0 || rx.before_bed > 0 {
                                                                        html! {
                                                                            <span style="color: var(--color-accent); margin-left: 0.5rem;">
                                                                                { format!("เช้า {} กลางวัน {} เย็น {} ก่อนนอน {}", 
                                                                                    rx.morning, rx.noon, rx.evening, rx.before_bed) }
                                                                            </span>
                                                                        }
                                                                    } else { html! {} }}
                                                                </div>
                                                                <button class="btn btn-secondary btn-sm" onclick={move |_| {
                                                                    navigator.push(&Route::Sticker { record_id: record_id.clone(), drug_index: idx });
                                                                }}>
                                                                    { "🏷️ พิมพ์สติกเกอร์" }
                                                                </button>
                                                            </li>
                                                        }
                                                    })}
                                                </ul>
                                            </div>
                                        }
                                    } else { html! {} }}
                                    
                                    // Action buttons
                                    <div class="history-item-actions">
                                        <button class="btn btn-secondary" onclick={let navigator = navigator.clone(); let id=id.clone(); move |_| {
                                            navigator.push(&Route::Document { doc_type: "receipt".to_string(), id: id.clone() });
                                        }}>
                                            { "🧾 ใบเสร็จ" }
                                        </button>
                                        <button class="btn btn-secondary" onclick={let navigator = navigator.clone(); let id=id.clone(); move |_| {
                                            navigator.push(&Route::Document { doc_type: "prescription".to_string(), id: id.clone() });
                                        }}>
                                            { "📋 ใบสั่งยา" }
                                        </button>
                                        <button class="btn btn-secondary" onclick={let navigator = navigator.clone(); let id=id.clone(); move |_| {
                                            navigator.push(&Route::Document { doc_type: "cert".to_string(), id: id.clone() });
                                        }}>
                                            { "📄 ใบรับรองแพทย์" }
                                        </button>
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                }
            }}
        </>
    }
}
