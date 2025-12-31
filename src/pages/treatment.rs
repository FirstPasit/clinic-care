use yew::prelude::*;
use crate::models::{Patient, TreatmentRecord, PrescriptionItem, InjectionItem};
use crate::store::Store;
use crate::components::{ToastContext, ToastAction, ToastType};
use web_sys::HtmlInputElement;
use chrono::Utc;
use uuid::Uuid;
use yew_router::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(Treatment)]
pub fn treatment(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let toast = use_context::<ToastContext>();
    let patient = use_state(|| -> Option<Patient> {
        Store::get_patients().into_iter().find(|p| p.id == props.id)
    });
    
    // Drug list from inventory for autocomplete
    let drug_list = Store::get_drugs();

    if patient.is_none() {
        return html! {
            <div class="empty-state">
                <div class="empty-state-icon">{ "❓" }</div>
                <h3 class="empty-state-title">{ "ไม่พบข้อมูลผู้ป่วย" }</h3>
                <p class="empty-state-text">{ "ไม่พบผู้ป่วยที่คุณต้องการ" }</p>
                <Link<Route> to={Route::Search} classes="btn btn-primary btn-lg">
                    { "← กลับไปค้นหา" }
                </Link<Route>>
            </div>
        };
    }
    let patient_data = patient.as_ref().unwrap();

    // Form States
    let symptoms = use_state(|| String::new());
    let diagnosis = use_state(|| String::new());
    let weight = use_state(|| String::new());
    let pressure = use_state(|| String::new());
    let doctor_note = use_state(|| String::new());
    let price = use_state(|| String::new());
    
    // Dynamic lists
    let prescriptions = use_state(|| Vec::<PrescriptionItem>::new());
    let _injections = use_state(|| Vec::<InjectionItem>::new());

    // Handlers
    let add_drug = {
        let prescriptions = prescriptions.clone();
        Callback::from(move |_: MouseEvent| {
            let mut current = (*prescriptions).clone();
            current.push(PrescriptionItem::default());
            prescriptions.set(current);
        })
    };

    let remove_drug = {
        let prescriptions = prescriptions.clone();
        Callback::from(move |idx: usize| {
            let mut current = (*prescriptions).clone();
            if idx < current.len() {
                current.remove(idx);
                prescriptions.set(current);
            }
        })
    };

    let onsubmit = {
        let patient_id = props.id.clone();
        let symptoms = symptoms.clone();
        let diagnosis = diagnosis.clone();
        let weight = weight.clone();
        let pressure = pressure.clone();
        let doctor_note = doctor_note.clone();
        let price = price.clone();
        let prescriptions = prescriptions.clone();
        let navigator = navigator.clone();
        let toast = toast.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let record = TreatmentRecord {
                id: Uuid::new_v4().to_string(),
                patient_id: patient_id.clone(),
                date: Utc::now(),
                symptoms: (*symptoms).clone(),
                diagnosis: (*diagnosis).clone(),
                weight: weight.parse::<f32>().ok(),
                pressure: (*pressure).clone(),
                prescriptions: (*prescriptions).clone(),
                injections: vec![],
                doctor_note: (*doctor_note).clone(),
                price: price.parse::<f64>().unwrap_or(0.0),
            };
            
            Store::save_record(record);
            
            if let Some(ref t) = toast {
                t.dispatch(ToastAction::Add(
                    "✅ บันทึกการรักษาเรียบร้อยแล้ว!".to_string(),
                    ToastType::Success
                ));
            }
            
            navigator.push(&Route::History { id: patient_id.clone() });
        })
    };

    html! {
        <>
            <div class="page-header">
                <h1 class="page-title">{ "💉 บันทึกการรักษา" }</h1>
                <p class="page-subtitle">{ "บันทึกข้อมูลการรักษาผู้ป่วย" }</p>
            </div>
            
            // Patient Header with allergy warning
            <div class="patient-header">
                <div class="patient-header-info">
                    <h2>{ format!("{}{} {}", patient_data.title, patient_data.first_name, patient_data.last_name) }</h2>
                    <div class="patient-header-meta">
                        <span>{ format!("HN: {}", patient_data.hn) }</span>
                        <span>{ format!("กรุ๊ปเลือด: {}", patient_data.blood_group) }</span>
                    </div>
                </div>
                { if !patient_data.underlying_disease.is_empty() && patient_data.underlying_disease != "ไม่มี" {
                    html! {
                        <div style="color: #ea580c; background: #fff7ed; border: 1px solid #fdba74; padding: 0.5rem; border-radius: 4px; margin-right: 1rem;">
                            { "🤕 โรคประจำตัว: " }<strong>{ &patient_data.underlying_disease }</strong>
                        </div>
                    }
                } else { html! {} }}
                { if !patient_data.drug_allergy.is_empty() && patient_data.drug_allergy.to_lowercase() != "none" && patient_data.drug_allergy != "ไม่มี" {
                    html! {
                        <div class="patient-header-allergy">
                            { "⚠️ แพ้ยา: " }<strong>{ &patient_data.drug_allergy }</strong>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
            
            <form onsubmit={onsubmit}>
                // Clinical Data
                <div class="card mb-6">
                    <div class="card-header">
                        <h3 class="card-title">{ "📋 ข้อมูลการตรวจ" }</h3>
                    </div>
                    
                    <div class="grid grid-cols-2 gap-4">
                        <div class="form-group">
                            <label class="form-label">{ "น้ำหนัก (กก.)" }</label>
                            <input type="number" step="0.1" value={(*weight).clone()}
                                placeholder="0.0"
                                oninput={let w = weight.clone(); Callback::from(move |e: InputEvent| w.set(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                        </div>
                        
                        <div class="form-group">
                            <label class="form-label">{ "ความดันโลหิต" }</label>
                            <input type="text" placeholder="120/80" value={(*pressure).clone()}
                                oninput={let p = pressure.clone(); Callback::from(move |e: InputEvent| p.set(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                        </div>
                        
                        <div class="form-group" style="grid-column: 1 / -1;">
                            <label class="form-label">{ "อาการ / ข้อร้องเรียน" }</label>
                            <textarea value={(*symptoms).clone()} placeholder="อธิบายอาการของผู้ป่วย..."
                                oninput={let s = symptoms.clone(); Callback::from(move |e: InputEvent| s.set(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                        </div>
                        
                        <div class="form-group" style="grid-column: 1 / -1;">
                            <label class="form-label">{ "การวินิจฉัย" }</label>
                            <textarea value={(*diagnosis).clone()} placeholder="ระบุการวินิจฉัย..."
                                oninput={let d = diagnosis.clone(); Callback::from(move |e: InputEvent| d.set(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                        </div>
                    </div>
                </div>
                
                // Prescriptions with Enhanced UI
                <div class="card mb-6">
                    <div class="card-header">
                        <h3 class="card-title">{ "💊 รายการยา" }</h3>
                        <button type="button" onclick={add_drug} class="btn btn-primary">
                            { "➕ เพิ่มยา" }
                        </button>
                    </div>
                    
                    { if prescriptions.is_empty() {
                        html! {
                            <div class="empty-state" style="padding: 3rem;">
                                <div class="empty-state-icon">{ "💊" }</div>
                                <p class="empty-state-text">{ "ยังไม่มีรายการยา กดปุ่ม \"เพิ่มยา\" ด้านบน" }</p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="prescription-list">
                                { for prescriptions.iter().enumerate().map(|(i, item)| {
                                    let prescriptions_for_update = prescriptions.clone();
                                    let remove = remove_drug.clone();
                                    let drug_list_clone = drug_list.clone();
                                    
                                    html! {
                                        <div class="card" style="background: var(--color-bg); margin-bottom: 1rem;">
                                            // Drug header
                                            <div class="flex justify-between items-center mb-4">
                                                <h4 style="margin: 0;">{ format!("ยาตัวที่ {}", i + 1) }</h4>
                                                <button type="button" onclick={move |_| remove.emit(i)} class="btn btn-danger btn-sm">
                                                    { "🗑️ ลบ" }
                                                </button>
                                            </div>
                                            
                                            // Drug name with datalist
                                            <div class="form-group">
                                                <label class="form-label">{ "ชื่อยา" }</label>
                                                <input type="text" list={format!("drugs-{}", i)} value={item.name.clone()} 
                                                    placeholder="พิมพ์ชื่อยา หรือเลือกจากรายการ"
                                                    oninput={{
                                                        let prescriptions = prescriptions_for_update.clone();
                                                        move |e: InputEvent| {
                                                            let mut current = (*prescriptions).clone();
                                                            if let Some(rx) = current.get_mut(i) {
                                                                rx.name = e.target_unchecked_into::<HtmlInputElement>().value();
                                                            }
                                                            prescriptions.set(current);
                                                        }
                                                    }} />
                                                <datalist id={format!("drugs-{}", i)}>
                                                    { for drug_list_clone.iter().map(|d| {
                                                        html! { <option value={d.name.clone()} /> }
                                                    })}
                                                </datalist>
                                            </div>
                                            
                                            // Dosage boxes - BIG and EASY
                                            <div class="grid grid-cols-4 gap-4 mb-4">
                                                <div class="dose-input-group">
                                                    <label>{ "🌅 เช้า" }</label>
                                                    <input type="number" min="0" value={item.morning.to_string()}
                                                        oninput={{
                                                            let prescriptions = prescriptions_for_update.clone();
                                                            move |e: InputEvent| {
                                                                let mut current = (*prescriptions).clone();
                                                                if let Some(rx) = current.get_mut(i) {
                                                                    rx.morning = e.target_unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(0);
                                                                }
                                                                prescriptions.set(current);
                                                            }
                                                        }} />
                                                </div>
                                                <div class="dose-input-group">
                                                    <label>{ "☀️ กลางวัน" }</label>
                                                    <input type="number" min="0" value={item.noon.to_string()}
                                                        oninput={{
                                                            let prescriptions = prescriptions_for_update.clone();
                                                            move |e: InputEvent| {
                                                                let mut current = (*prescriptions).clone();
                                                                if let Some(rx) = current.get_mut(i) {
                                                                    rx.noon = e.target_unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(0);
                                                                }
                                                                prescriptions.set(current);
                                                            }
                                                        }} />
                                                </div>
                                                <div class="dose-input-group">
                                                    <label>{ "🌆 เย็น" }</label>
                                                    <input type="number" min="0" value={item.evening.to_string()}
                                                        oninput={{
                                                            let prescriptions = prescriptions_for_update.clone();
                                                            move |e: InputEvent| {
                                                                let mut current = (*prescriptions).clone();
                                                                if let Some(rx) = current.get_mut(i) {
                                                                    rx.evening = e.target_unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(0);
                                                                }
                                                                prescriptions.set(current);
                                                            }
                                                        }} />
                                                </div>
                                                <div class="dose-input-group">
                                                    <label>{ "🌙 ก่อนนอน" }</label>
                                                    <input type="number" min="0" value={item.before_bed.to_string()}
                                                        oninput={{
                                                            let prescriptions = prescriptions_for_update.clone();
                                                            move |e: InputEvent| {
                                                                let mut current = (*prescriptions).clone();
                                                                if let Some(rx) = current.get_mut(i) {
                                                                    rx.before_bed = e.target_unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(0);
                                                                }
                                                                prescriptions.set(current);
                                                            }
                                                        }} />
                                                </div>
                                            </div>
                                            
                                            // Timing and Amount
                                            <div class="grid grid-cols-2 gap-4">
                                                <div class="form-group">
                                                    <label class="form-label">{ "ช่วงเวลารับประทาน" }</label>
                                                    <select onchange={{
                                                        let prescriptions = prescriptions_for_update.clone();
                                                        move |e: Event| {
                                                            let mut current = (*prescriptions).clone();
                                                            if let Some(rx) = current.get_mut(i) {
                                                                rx.timing = e.target_unchecked_into::<HtmlInputElement>().value();
                                                            }
                                                            prescriptions.set(current);
                                                        }
                                                    }}>
                                                        <option value="ก่อนอาหาร" selected={item.timing == "ก่อนอาหาร"}>{ "ก่อนอาหาร" }</option>
                                                        <option value="หลังอาหาร" selected={item.timing == "หลังอาหาร" || item.timing.is_empty()}>{ "หลังอาหาร" }</option>
                                                        <option value="พร้อมอาหาร" selected={item.timing == "พร้อมอาหาร"}>{ "พร้อมอาหาร" }</option>
                                                        <option value="ก่อนอาหาร 30 นาที" selected={item.timing == "ก่อนอาหาร 30 นาที"}>{ "ก่อนอาหาร 30 นาที" }</option>
                                                        <option value="เมื่อมีอาการ" selected={item.timing == "เมื่อมีอาการ"}>{ "เมื่อมีอาการ" }</option>
                                                    </select>
                                                </div>
                                                <div class="form-group">
                                                    <label class="form-label">{ "จำนวนทั้งหมด" }</label>
                                                    <input type="text" value={item.amount.clone()} placeholder="เช่น 20 เม็ด"
                                                        oninput={{
                                                            let prescriptions = prescriptions_for_update.clone();
                                                            move |e: InputEvent| {
                                                                let mut current = (*prescriptions).clone();
                                                                if let Some(rx) = current.get_mut(i) {
                                                                    rx.amount = e.target_unchecked_into::<HtmlInputElement>().value();
                                                                }
                                                                prescriptions.set(current);
                                                            }
                                                        }} />
                                                </div>
                                            </div>
                                            
                                            // Warning
                                            <div class="form-group">
                                                <label class="form-label">{ "⚠️ คำเตือน (สำหรับสติกเกอร์)" }</label>
                                                <input type="text" value={item.warning.clone()} 
                                                    placeholder="เช่น ห้ามดื่มแอลกอฮอล์, ห้ามขับรถ, ทำให้ง่วง"
                                                    oninput={{
                                                        let prescriptions = prescriptions_for_update.clone();
                                                        move |e: InputEvent| {
                                                            let mut current = (*prescriptions).clone();
                                                            if let Some(rx) = current.get_mut(i) {
                                                                rx.warning = e.target_unchecked_into::<HtmlInputElement>().value();
                                                            }
                                                            prescriptions.set(current);
                                                        }
                                                    }} />
                                            </div>
                                        </div>
                                    }
                                })}
                            </div>
                        }
                    }}
                </div>
                
                // Notes & Price
                <div class="card mb-6">
                    <div class="grid grid-cols-2 gap-4">
                        <div class="form-group" style="grid-column: 1 / -1;">
                            <label class="form-label">{ "📝 หมายเหตุแพทย์ " } <span class="form-label-optional">{ "(ถ้ามี)" }</span></label>
                            <textarea value={(*doctor_note).clone()} placeholder="หมายเหตุเพิ่มเติม..."
                                oninput={let n = doctor_note.clone(); Callback::from(move |e: InputEvent| n.set(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                        </div>
                        
                        <div class="form-group">
                            <label class="form-label">{ "💰 ค่ารักษารวม (บาท) *" }</label>
                            <input type="number" required=true min="0" step="1" value={(*price).clone()} placeholder="0"
                                inputmode="numeric"
                                style="font-size: 2rem; font-weight: 700; text-align: center;"
                                oninput={let p = price.clone(); Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    // Only allow positive numbers
                                    let val = input.value();
                                    let filtered: String = val.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
                                    p.set(filtered);
                                })} />
                        </div>
                    </div>
                </div>
                
                // Action Buttons - BIG
                <div class="flex justify-between items-center gap-4">
                    <Link<Route> to={Route::Search} classes="btn btn-ghost btn-lg">
                        { "← ยกเลิก" }
                    </Link<Route>>
                    <button type="submit" class="btn btn-success btn-lg" style="flex: 1; max-width: 400px;">
                        { "💾 บันทึกการรักษา" }
                    </button>
                </div>
            </form>
        </>
    }
}
