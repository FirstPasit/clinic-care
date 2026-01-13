use yew::prelude::*;
use crate::models::{Patient, TreatmentRecord, PrescriptionItem, InjectionItem};
use crate::store::Store;
use crate::components::{ToastContext, ToastAction, ToastType};
use web_sys::HtmlInputElement;
use chrono::Utc;
use uuid::Uuid;
use yew_router::prelude::*;
use crate::Route;

const SERVICE_FEE: f64 = 50.0; // ค่าบริการทางการพยาบาล fix

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

// Helper function to parse amount like "10 เม็ด" -> 10
fn parse_quantity(amount_str: &str) -> f64 {
    // Try to extract number from string like "10 เม็ด", "5", "20 ซอง"
    let digits: String = amount_str.chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    digits.parse::<f64>().unwrap_or(0.0)
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
    let manual_price_override = use_state(|| false); // Flag for manual override
    let is_drug_only_purchase = use_state(|| false); // ซื้อยาอย่างเดียว ไม่คิดค่าบริการ
    
    // Dynamic lists
    let prescriptions = use_state(|| Vec::<PrescriptionItem>::new());
    let _injections = use_state(|| Vec::<InjectionItem>::new());

    // Calculate total price automatically
    let calculated_drug_cost = {
        let drug_list = drug_list.clone();
        let prescriptions = (*prescriptions).clone();
        
        prescriptions.iter().map(|rx| {
            // Find drug in inventory by name
            let drug = drug_list.iter().find(|d| d.name == rx.name);
            let unit_price = drug.map(|d| d.sell_price).unwrap_or(0.0);
            let quantity = parse_quantity(&rx.amount);
            unit_price * quantity
        }).sum::<f64>()
    };
    
    // ถ้าซื้อยาอย่างเดียว ไม่คิดค่าบริการ 50 บาท
    let service_fee_to_add = if *is_drug_only_purchase { 0.0 } else { SERVICE_FEE };
    let calculated_total = service_fee_to_add + calculated_drug_cost;
    
    // Use calculated price unless manually overridden
    let final_price = use_state(|| 0.0_f64);
    
    // Update final price when prescriptions change (unless manual override)
    {
        let final_price = final_price.clone();
        let manual_override = *manual_price_override;
        let is_drug_only = *is_drug_only_purchase;
        let calc_total = calculated_total;
        
        use_effect_with(
            (prescriptions.clone(), manual_override, is_drug_only),
            move |(_, override_flag, _)| {
                if !*override_flag {
                    final_price.set(calc_total);
                }
                || ()
            }
        );
    }

    // Handlers
    let add_drug = {
        let prescriptions = prescriptions.clone();
        let manual_price_override = manual_price_override.clone();
        Callback::from(move |_: MouseEvent| {
            let mut current = (*prescriptions).clone();
            current.push(PrescriptionItem::default());
            prescriptions.set(current);
            manual_price_override.set(false); // Reset override when adding drugs
        })
    };

    let remove_drug = {
        let prescriptions = prescriptions.clone();
        let manual_price_override = manual_price_override.clone();
        Callback::from(move |idx: usize| {
            let mut current = (*prescriptions).clone();
            if idx < current.len() {
                current.remove(idx);
                prescriptions.set(current);
                manual_price_override.set(false); // Reset override
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
        let final_price = final_price.clone();
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
                price: *final_price,
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
                        <div>
                            <h3 class="card-title">{ "💊 รายการยา" }</h3>
                            // Drug-only purchase toggle
                            <div style="display: flex; align-items: center; gap: 0.5rem; margin-top: 0.5rem;">
                                <input type="checkbox" id="drug-only-purchase" 
                                    checked={*is_drug_only_purchase}
                                    onchange={{
                                        let is_drug_only_purchase = is_drug_only_purchase.clone();
                                        let manual_price_override = manual_price_override.clone();
                                        Callback::from(move |e: Event| {
                                            let checked = e.target_unchecked_into::<HtmlInputElement>().checked();
                                            is_drug_only_purchase.set(checked);
                                            manual_price_override.set(false); // Reset manual override
                                        })
                                    }} />
                                <label for="drug-only-purchase" style={if *is_drug_only_purchase { 
                                    "font-weight: bold; color: #059669; background: #dcfce7; padding: 0.25rem 0.5rem; border-radius: 4px;" 
                                } else { 
                                    "color: #666;" 
                                }}>
                                    { "💰 ซื้อยาอย่างเดียว (ไม่คิดค่าบริการ 50 บาท)" }
                                </label>
                            </div>
                        </div>
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
                                    let prescriptions_for_update2 = prescriptions.clone();
                                    let manual_override_clone = manual_price_override.clone();
                                    let remove = remove_drug.clone();
                                    let drug_list_clone = drug_list.clone();
                                    let drug_list_clone2 = drug_list.clone();
                                    
                                    // Get current drug price for display
                                    let current_drug = drug_list_clone2.iter().find(|d| d.name == item.name);
                                    let unit_price = current_drug.map(|d| d.sell_price).unwrap_or(0.0);
                                    let stock_available = current_drug.map(|d| d.stock).unwrap_or(0);
                                    let quantity = parse_quantity(&item.amount);
                                    let quantity_u32 = quantity as u32;
                                    let is_over_stock = current_drug.is_some() && quantity_u32 > stock_available;
                                    let line_total = unit_price * quantity;
                                    
                                    html! {
                                        <div class="card" style="background: var(--color-bg); margin-bottom: 1rem;">
                                            // Drug header with price info
                                            <div class="flex justify-between items-center mb-4">
                                                <div>
                                                    <h4 style="margin: 0;">{ format!("ยาตัวที่ {}", i + 1) }</h4>
                                                    { if unit_price > 0.0 {
                                                        html! {
                                                            <span style="font-size: 0.85rem; color: #059669;">
                                                                { format!("💰 ราคา {} บาท/{}", unit_price, current_drug.map(|d| d.unit.as_str()).unwrap_or("หน่วย")) }
                                                            </span>
                                                        }
                                                    } else { html! {} }}
                                                </div>
                                                <div class="flex items-center gap-2">
                                                    { if line_total > 0.0 {
                                                        html! {
                                                            <span style="background: #dcfce7; color: #166534; padding: 0.25rem 0.75rem; border-radius: 4px; font-weight: bold;">
                                                                { format!("= {} บาท", line_total) }
                                                            </span>
                                                        }
                                                    } else { html! {} }}
                                                    <button type="button" onclick={move |_| remove.emit(i)} class="btn btn-danger btn-sm">
                                                        { "🗑️ ลบ" }
                                                    </button>
                                                </div>
                                            </div>
                                            
                                            // Drug name with datalist
                                            <div class="form-group">
                                                <label class="form-label">{ "ชื่อยา" }</label>
                                                <input type="text" list={format!("drugs-{}", i)} value={item.name.clone()} 
                                                    placeholder="พิมพ์ชื่อยา หรือเลือกจากรายการ"
                                                    onchange={{
                                                        let prescriptions = prescriptions_for_update.clone();
                                                        let manual_override = manual_override_clone.clone();
                                                        let drug_list_for_autofill = drug_list_clone.clone();
                                                        move |e: Event| {
                                                            let drug_name = e.target_unchecked_into::<HtmlInputElement>().value();
                                                            let mut current = (*prescriptions).clone();
                                                            if let Some(rx) = current.get_mut(i) {
                                                                rx.name = drug_name.clone();
                                                                // Auto-fill usage and warning from drug inventory
                                                                if let Some(drug) = drug_list_for_autofill.iter().find(|d| d.name == drug_name) {
                                                                    // Only auto-fill if fields are empty
                                                                    if rx.usage.is_empty() && !drug.default_usage.is_empty() {
                                                                        rx.usage = drug.default_usage.clone();
                                                                    }
                                                                    if rx.warning.is_empty() && !drug.warning.is_empty() {
                                                                        rx.warning = drug.warning.clone();
                                                                    }
                                                                }
                                                            }
                                                            prescriptions.set(current);
                                                            manual_override.set(false);
                                                        }
                                                    }}
                                                    oninput={{
                                                        let prescriptions = prescriptions_for_update.clone();
                                                        let manual_override = manual_override_clone.clone();
                                                        let drug_list_for_autofill = drug_list_clone.clone();
                                                        move |e: InputEvent| {
                                                            let drug_name = e.target_unchecked_into::<HtmlInputElement>().value();
                                                            let mut current = (*prescriptions).clone();
                                                            if let Some(rx) = current.get_mut(i) {
                                                                rx.name = drug_name.clone();
                                                                // Auto-fill usage and warning when exact match found
                                                                if let Some(drug) = drug_list_for_autofill.iter().find(|d| d.name == drug_name) {
                                                                    if rx.usage.is_empty() && !drug.default_usage.is_empty() {
                                                                        rx.usage = drug.default_usage.clone();
                                                                    }
                                                                    if rx.warning.is_empty() && !drug.warning.is_empty() {
                                                                        rx.warning = drug.warning.clone();
                                                                    }
                                                                }
                                                            }
                                                            prescriptions.set(current);
                                                            manual_override.set(false);
                                                        }
                                                    }} />
                                                <datalist id={format!("drugs-{}", i)}>
                                                    { for drug_list_clone.iter().map(|d| {
                                                        html! { <option value={d.name.clone()} /> }
                                                    })}
                                                </datalist>
                                                // Show hint if drug found in inventory
                                                { if let Some(drug) = current_drug {
                                                    html! {
                                                        <div style="margin-top: 0.5rem; font-size: 0.85rem; color: #666;">
                                                            { if !drug.default_usage.is_empty() {
                                                                html! { <span style="margin-right: 1rem;">{ format!("💊 คำแนะนำเริ่มต้น: {}", drug.default_usage) }</span> }
                                                            } else { html! {} }}
                                                            { if !drug.warning.is_empty() {
                                                                html! { <span style="color: #dc2626;">{ format!("⚠️ คำเตือน: {}", drug.warning) }</span> }
                                                            } else { html! {} }}
                                                        </div>
                                                    }
                                                } else { html! {} }}
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
                                                    <label class="form-label">
                                                        { "จำนวนทั้งหมด (ใส่ตัวเลขก่อน)" }
                                                        { if current_drug.is_some() {
                                                            html! { 
                                                                <span style={if is_over_stock { "color: #dc2626; margin-left: 0.5rem;" } else { "color: #059669; margin-left: 0.5rem;" }}>
                                                                    { format!("(คงเหลือ {} {})", stock_available, current_drug.map(|d| d.unit.as_str()).unwrap_or("หน่วย")) }
                                                                </span>
                                                            }
                                                        } else { html! {} }}
                                                    </label>
                                                    <input type="text" value={item.amount.clone()} placeholder="เช่น 10 เม็ด, 1 ขวด"
                                                        style={if is_over_stock { "border-color: #dc2626; background: #fef2f2;" } else { "" }}
                                                        oninput={{
                                                            let prescriptions = prescriptions_for_update2.clone();
                                                            let manual_override = manual_override_clone.clone();
                                                            move |e: InputEvent| {
                                                                let mut current = (*prescriptions).clone();
                                                                if let Some(rx) = current.get_mut(i) {
                                                                    rx.amount = e.target_unchecked_into::<HtmlInputElement>().value();
                                                                }
                                                                prescriptions.set(current);
                                                                manual_override.set(false); // Recalculate
                                                            }
                                                        }} />
                                                    { if is_over_stock {
                                                        html! {
                                                            <div style="color: #dc2626; font-size: 0.85rem; margin-top: 0.25rem;">
                                                                { format!("⚠️ สั่งเกินสต็อก! (คงเหลือ {} ชิ้น)", stock_available) }
                                                            </div>
                                                        }
                                                    } else { html! {} }}
                                                </div>
                                            </div>
                                            
                                            // Usage for sticker (NEW)
                                            <div class="form-group">
                                                <label class="form-label">{ "💊 คำแนะนำยา (สำหรับสติกเกอร์)" }</label>
                                                <input type="text" value={item.usage.clone()} 
                                                    placeholder="เช่น รับประทานหลังอาหาร, ทาบริเวณที่เจ็บ"
                                                    style={if !item.usage.is_empty() { "border-color: #059669; background: #f0fdf4;" } else { "" }}
                                                    oninput={{
                                                        let prescriptions = prescriptions_for_update.clone();
                                                        move |e: InputEvent| {
                                                            let mut current = (*prescriptions).clone();
                                                            if let Some(rx) = current.get_mut(i) {
                                                                rx.usage = e.target_unchecked_into::<HtmlInputElement>().value();
                                                            }
                                                            prescriptions.set(current);
                                                        }
                                                    }} />
                                                { if !item.usage.is_empty() {
                                                    html! { <small style="color: #059669;">{ "✅ จะแสดงในสติกเกอร์" }</small> }
                                                } else {
                                                    html! { <small style="color: #666;">{ "ถ้าไม่กรอก จะไม่แสดงในสติกเกอร์" }</small> }
                                                }}
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
                
                // Price Calculation Summary Card
                <div class="card mb-6" style="border: 2px solid #059669; background: linear-gradient(135deg, #ecfdf5 0%, #d1fae5 100%);">
                    <div class="card-header">
                        <h3 class="card-title" style="color: #065f46;">{ "💰 สรุปค่าใช้จ่าย (คำนวณอัตโนมัติ)" }</h3>
                    </div>
                    
                    <div style="padding: 1rem;">
                        // Price Breakdown
                        <table style="width: 100%; margin-bottom: 1rem;">
                            <tbody>
                                <tr style={if *is_drug_only_purchase { 
                                    "border-bottom: 1px solid #a7f3d0; opacity: 0.5; text-decoration: line-through;" 
                                } else { 
                                    "border-bottom: 1px solid #a7f3d0;" 
                                }}>
                                    <td style="padding: 0.5rem 0;">
                                        { "ค่าบริการทางการพยาบาล" }
                                        { if *is_drug_only_purchase {
                                            html! { <span style="color: #dc2626; margin-left: 0.5rem;">{ "(ไม่คิด - ซื้อยาอย่างเดียว)" }</span> }
                                        } else { html! {} }}
                                    </td>
                                    <td style="padding: 0.5rem 0; text-align: right; font-weight: 500;">
                                        { format!("{:.2} บาท", service_fee_to_add) }
                                    </td>
                                </tr>
                                <tr style="border-bottom: 1px solid #a7f3d0;">
                                    <td style="padding: 0.5rem 0;">
                                        { format!("ค่ายาและเวชภัณฑ์ ({} รายการ)", prescriptions.len()) }
                                    </td>
                                    <td style="padding: 0.5rem 0; text-align: right; font-weight: 500;">
                                        { format!("{:.2} บาท", calculated_drug_cost) }
                                    </td>
                                </tr>
                                <tr style="background: #059669; color: white;">
                                    <td style="padding: 0.75rem; font-size: 1.25rem; font-weight: bold;">
                                        { "รวมทั้งสิ้น" }
                                    </td>
                                    <td style="padding: 0.75rem; text-align: right; font-size: 1.5rem; font-weight: bold;">
                                        { format!("{:.2} บาท", calculated_total) }
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                        
                        // Manual override option
                        <div style="background: #fef3c7; border: 1px solid #fbbf24; border-radius: 4px; padding: 1rem; margin-top: 1rem;">
                            <div class="flex items-center gap-2 mb-2">
                                <input type="checkbox" id="manual-override" 
                                    checked={*manual_price_override}
                                    onchange={{
                                        let manual_price_override = manual_price_override.clone();
                                        let final_price = final_price.clone();
                                        let calc_total = calculated_total;
                                        Callback::from(move |e: Event| {
                                            let checked = e.target_unchecked_into::<HtmlInputElement>().checked();
                                            manual_price_override.set(checked);
                                            if !checked {
                                                final_price.set(calc_total);
                                            }
                                        })
                                    }} />
                                <label for="manual-override" style="font-weight: 500; color: #92400e;">
                                    { "✏️ ปรับราคาเอง (กรณีพิเศษ เช่น ลดราคา, เหมาจ่าย)" }
                                </label>
                            </div>
                            
                            { if *manual_price_override {
                                html! {
                                    <div class="form-group" style="margin-top: 0.5rem;">
                                        <input type="number" min="0" step="1" 
                                            value={format!("{:.0}", *final_price)}
                                            style="font-size: 1.5rem; font-weight: 700; text-align: center; border: 2px solid #f59e0b;"
                                            oninput={{
                                                let final_price = final_price.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let val: f64 = e.target_unchecked_into::<HtmlInputElement>()
                                                        .value()
                                                        .parse()
                                                        .unwrap_or(0.0);
                                                    final_price.set(val);
                                                })
                                            }} />
                                    </div>
                                }
                            } else {
                                html! {}
                            }}
                        </div>
                    </div>
                </div>
                
                // Notes
                <div class="card mb-6">
                    <div class="form-group">
                        <label class="form-label">{ "📝 หมายเหตุแพทย์ " } <span class="form-label-optional">{ "(ถ้ามี)" }</span></label>
                        <textarea value={(*doctor_note).clone()} placeholder="หมายเหตุเพิ่มเติม..."
                            oninput={let n = doctor_note.clone(); Callback::from(move |e: InputEvent| n.set(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                    </div>
                </div>
                
                // Action Buttons - BIG
                <div class="flex justify-between items-center gap-4">
                    <Link<Route> to={Route::Search} classes="btn btn-ghost btn-lg">
                        { "← ยกเลิก" }
                    </Link<Route>>
                    <button type="submit" class="btn btn-success btn-lg" style="flex: 1; max-width: 400px;">
                        { format!("💾 บันทึกการรักษา ({:.0} บาท)", *final_price) }
                    </button>
                </div>
            </form>
        </>
    }
}
