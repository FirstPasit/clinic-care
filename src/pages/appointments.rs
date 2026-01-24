use yew::prelude::*;
use web_sys::HtmlInputElement;
use uuid::Uuid;
use crate::models::Appointment;
use crate::store::Store;
use crate::components::{ToastContext, ToastAction, ToastType};
use chrono::prelude::*;

#[function_component(Appointments)]
pub fn appointments() -> Html {
    let toast = use_context::<ToastContext>();
    let appointments = use_state(|| Store::get_appointments());
    let show_form = use_state(|| false);
    
    // Form state
    let patient_id = use_state(|| String::new());
    let patient_name = use_state(|| String::new());
    let date_str = use_state(|| (Local::now() + chrono::Duration::days(1)).format("%Y-%m-%d").to_string());
    let time = use_state(|| "09:00".to_string());
    let reason = use_state(|| String::new());
    let note = use_state(|| String::new());
    
    // View date filter
    let view_date = use_state(|| Local::now().date_naive());
    
    let patients = Store::get_patients();
    let today_appointments = Store::get_today_appointments();
    
    let filtered_appointments: Vec<Appointment> = {
        let date = *view_date;
        Store::get_appointments_by_date(date)
    };
    
    let on_save = {
        let patient_id = patient_id.clone();
        let patient_name = patient_name.clone();
        let date_str = date_str.clone();
        let time = time.clone();
        let reason = reason.clone();
        let note = note.clone();
        let appointments = appointments.clone();
        let show_form = show_form.clone();
        let toast = toast.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            if (*patient_name).is_empty() || (*reason).is_empty() {
                if let Some(ref t) = toast {
                    t.dispatch(ToastAction::Add("❌ กรุณากรอกข้อมูลให้ครบ".to_string(), ToastType::Error));
                }
                return;
            }
            
            let date = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .unwrap_or_else(|_| Local::now().date_naive());
            
            let appointment = Appointment {
                id: Uuid::new_v4().to_string(),
                patient_id: (*patient_id).clone(),
                patient_name: (*patient_name).clone(),
                date,
                time: (*time).clone(),
                reason: (*reason).clone(),
                status: "pending".to_string(),
                note: (*note).clone(),
            };
            
            Store::save_appointment(appointment);
            appointments.set(Store::get_appointments());
            
            // Reset form
            patient_id.set(String::new());
            patient_name.set(String::new());
            reason.set(String::new());
            note.set(String::new());
            show_form.set(false);
            
            if let Some(ref t) = toast {
                t.dispatch(ToastAction::Add("✅ บันทึกนัดหมายแล้ว".to_string(), ToastType::Success));
            }
        })
    };
    
    let on_complete = {
        let appointments = appointments.clone();
        let toast = toast.clone();
        
        Callback::from(move |id: String| {
            let all = Store::get_appointments();
            if let Some(mut apt) = all.into_iter().find(|a| a.id == id) {
                apt.status = "completed".to_string();
                Store::update_appointment(apt);
                appointments.set(Store::get_appointments());
                if let Some(ref t) = toast {
                    t.dispatch(ToastAction::Add("✅ เสร็จสิ้นนัดหมาย".to_string(), ToastType::Success));
                }
            }
        })
    };
    
    let on_cancel = {
        let appointments = appointments.clone();
        let toast = toast.clone();
        
        Callback::from(move |id: String| {
            if web_sys::window().unwrap().confirm_with_message("ยกเลิกนัดหมายนี้?").unwrap_or(false) {
                let all = Store::get_appointments();
                if let Some(mut apt) = all.into_iter().find(|a| a.id == id) {
                    apt.status = "cancelled".to_string();
                    Store::update_appointment(apt);
                    appointments.set(Store::get_appointments());
                    if let Some(ref t) = toast {
                        t.dispatch(ToastAction::Add("❌ ยกเลิกนัดหมายแล้ว".to_string(), ToastType::Error));
                    }
                }
            }
        })
    };
    
    let on_delete = {
        let appointments = appointments.clone();
        
        Callback::from(move |id: String| {
            if web_sys::window().unwrap().confirm_with_message("ลบนัดหมายนี้?").unwrap_or(false) {
                Store::delete_appointment(&id);
                appointments.set(Store::get_appointments());
            }
        })
    };

    html! {
        <>
            <div class="page-header flex justify-between items-center">
                <div>
                    <h1 class="page-title">{ "🗓️ นัดหมายผู้ป่วย" }</h1>
                    <p class="page-subtitle">{ "จัดการนัดหมายและติดตามผู้ป่วย" }</p>
                </div>
                <button class="btn btn-primary btn-lg" onclick={{
                    let show_form = show_form.clone();
                    move |_| show_form.set(!*show_form)
                }}>
                    { if *show_form { "❌ ยกเลิก" } else { "➕ เพิ่มนัดหมาย" } }
                </button>
            </div>
            
            // Today's appointments alert
            { if !today_appointments.is_empty() {
                html! {
                    <div class="alert alert-accent mb-4">
                        <span class="alert-icon">{ "📅" }</span>
                        <span>{ format!("วันนี้มีนัดหมาย {} ราย: ", today_appointments.len()) }
                            { today_appointments.iter().take(3).map(|a| {
                                format!("{} ({}น.)", a.patient_name, a.time)
                            }).collect::<Vec<_>>().join(", ") }
                        </span>
                    </div>
                }
            } else { html! {} }}
            
            // Add appointment form
            { if *show_form {
                html! {
                    <div class="card mb-4">
                        <h3 class="mb-4">{ "📝 เพิ่มนัดหมายใหม่" }</h3>
                        <form onsubmit={on_save}>
                            <div class="grid grid-cols-2 gap-4">
                                <div class="form-group">
                                    <label class="form-label">{ "เลือกผู้ป่วย (หรือพิมพ์ชื่อ)" }</label>
                                    <input type="text" list="patient-list" value={(*patient_name).clone()}
                                        placeholder="พิมพ์ชื่อผู้ป่วย"
                                        oninput={{
                                            let patient_name = patient_name.clone();
                                            let patient_id = patient_id.clone();
                                            let patients = patients.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                let name = input.value();
                                                patient_name.set(name.clone());
                                                // Find patient id if exact match
                                                if let Some(p) = patients.iter().find(|p| format!("{}{} {}", p.title, p.first_name, p.last_name) == name) {
                                                    patient_id.set(p.id.clone());
                                                } else {
                                                    patient_id.set(String::new());
                                                }
                                            })
                                        }} />
                                    <datalist id="patient-list">
                                        { for patients.iter().map(|p| {
                                            let full_name = format!("{}{} {}", p.title, p.first_name, p.last_name);
                                            html! { <option value={full_name} /> }
                                        })}
                                    </datalist>
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{ "วันที่นัด *" }</label>
                                    <input type="date" required=true value={(*date_str).clone()}
                                        oninput={{
                                            let date_str = date_str.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                date_str.set(input.value());
                                            })
                                        }} />
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{ "เวลา *" }</label>
                                    <select value={(*time).clone()} onchange={{
                                        let time = time.clone();
                                        Callback::from(move |e: Event| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            time.set(input.value());
                                        })
                                    }}>
                                        { for (8..=18).flat_map(|h| vec![format!("{:02}:00", h), format!("{:02}:30", h)]).map(|t| {
                                            html! { <option value={t.clone()} selected={*time == t}>{ &t }{ " น." }</option> }
                                        })}
                                    </select>
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{ "เหตุผลนัด *" }</label>
                                    <input type="text" required=true value={(*reason).clone()}
                                        placeholder="เช่น ติดตามอาการ, รับยา, ตรวจซ้ำ"
                                        oninput={{
                                            let reason = reason.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                reason.set(input.value());
                                            })
                                        }} />
                                </div>
                                
                                <div class="form-group" style="grid-column: 1 / -1;">
                                    <label class="form-label">{ "หมายเหตุ" }</label>
                                    <input type="text" value={(*note).clone()}
                                        placeholder="หมายเหตุเพิ่มเติม"
                                        oninput={{
                                            let note = note.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                note.set(input.value());
                                            })
                                        }} />
                                </div>
                            </div>
                            
                            <div class="flex justify-end mt-4">
                                <button type="submit" class="btn btn-primary btn-lg">{ "💾 บันทึก" }</button>
                            </div>
                        </form>
                    </div>
                }
            } else { html! {} }}
            
            // Date filter
            <div class="card mb-4">
                <div class="flex items-center gap-4">
                    <span class="font-bold">{ "📅 ดูวันที่:" }</span>
                    <input type="date" value={view_date.format("%Y-%m-%d").to_string()}
                        oninput={{
                            let view_date = view_date.clone();
                            Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                if let Ok(d) = chrono::NaiveDate::parse_from_str(&input.value(), "%Y-%m-%d") {
                                    view_date.set(d);
                                }
                            })
                        }} />
                    <button class="btn btn-secondary btn-sm" onclick={{
                        let view_date = view_date.clone();
                        move |_| view_date.set(Local::now().date_naive())
                    }}>{ "วันนี้" }</button>
                    <button class="btn btn-secondary btn-sm" onclick={{
                        let view_date = view_date.clone();
                        move |_| view_date.set(Local::now().date_naive() + chrono::Duration::days(1))
                    }}>{ "พรุ่งนี้" }</button>
                </div>
            </div>
            
            // Appointment list
            <div class="card">
                <h3 class="mb-4">{ format!("📋 นัดหมายวันที่ {}", view_date.format("%d/%m/%Y")) }</h3>
                { if filtered_appointments.is_empty() {
                    html! {
                        <div class="empty-state">
                            <div class="empty-state-icon">{ "📅" }</div>
                            <h3 class="empty-state-title">{ "ไม่มีนัดหมายในวันนี้" }</h3>
                        </div>
                    }
                } else {
                    html! {
                        <div class="grid gap-4">
                            { for filtered_appointments.iter().map(|apt| {
                                let id = apt.id.clone();
                                let on_complete = on_complete.clone();
                                let on_cancel = on_cancel.clone();
                                let on_delete = on_delete.clone();
                                
                                let status_badge = match apt.status.as_str() {
                                    "completed" => ("✅ เสร็จสิ้น", "badge-success"),
                                    "cancelled" => ("❌ ยกเลิก", "badge-error"),
                                    _ => ("⏳ รอดำเนินการ", "badge-warning"),
                                };
                                
                                html! {
                                    <div class="history-item" style={if apt.status == "cancelled" { "opacity: 0.5;" } else { "" }}>
                                        <div class="flex justify-between items-center">
                                            <div>
                                                <div class="history-item-date">{ format!("⏰ {} น.", apt.time) }</div>
                                                <div style="font-size: 1.3rem; font-weight: bold;">{ &apt.patient_name }</div>
                                                <div class="text-muted">{ format!("เหตุผล: {}", apt.reason) }</div>
                                                { if !apt.note.is_empty() {
                                                    html! { <div class="text-muted">{ format!("หมายเหตุ: {}", apt.note) }</div> }
                                                } else { html! {} }}
                                            </div>
                                            <div class="flex items-center gap-3">
                                                <span class={classes!("badge", status_badge.1)}>{ status_badge.0 }</span>
                                                { if apt.status == "pending" {
                                                    html! {
                                                        <>
                                                            <button class="btn btn-success btn-sm" onclick={let id = id.clone(); move |_| on_complete.emit(id.clone())}>
                                                                { "✅ เสร็จ" }
                                                            </button>
                                                            <button class="btn btn-warning btn-sm" onclick={let id = id.clone(); move |_| on_cancel.emit(id.clone())}>
                                                                { "❌ ยกเลิก" }
                                                            </button>
                                                        </>
                                                    }
                                                } else {
                                                    html! {
                                                        <button class="btn btn-error btn-sm" onclick={move |_| on_delete.emit(id.clone())}>
                                                            { "🗑️ ลบ" }
                                                        </button>
                                                    }
                                                }}
                                            </div>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    }
                }}
            </div>
        </>
    }
}
