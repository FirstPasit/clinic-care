use yew::prelude::*;
use web_sys::HtmlInputElement;
use chrono::{Utc, Datelike};
use uuid::Uuid;
use crate::models::Patient;
use crate::store::Store;
use crate::components::{ToastContext, ToastAction, ToastType};

// Helper to filter non-digits
fn digits_only(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_digit()).collect()
}

// Helper to filter digits with max length
fn digits_max(s: &str, max: usize) -> String {
    digits_only(s).chars().take(max).collect()
}

#[function_component(Register)]
pub fn register() -> Html {
    let navigator = yew_router::prelude::use_navigator().unwrap();
    let toast = use_context::<ToastContext>();
    
    // Form state
    let hn = use_state(|| String::new()); // No longer default, manual entry
    let citizen_id = use_state(|| String::new());
    let title = use_state(|| "นาย".to_string());
    let first_name = use_state(|| String::new());
    let last_name = use_state(|| String::new());
    let birth_date = use_state(|| String::new());
    let age = use_state(|| String::new()); // อายุ
    let blood_group = use_state(|| "ไม่ทราบ".to_string());
    let underlying_disease = use_state(|| String::new());
    let drug_allergy = use_state(|| String::new());
    let phone = use_state(|| String::new());
    let address = use_state(|| String::new());
    
    // Validation states
    let form_valid = !(*hn).is_empty() && !(*first_name).is_empty() && !(*last_name).is_empty();

    let onsubmit = {
        let hn = hn.clone();
        let citizen_id = citizen_id.clone();
        let title = title.clone();
        let first_name = first_name.clone();
        let last_name = last_name.clone();
        let birth_date = birth_date.clone();
        let age = age.clone();
        let blood_group = blood_group.clone();
        let underlying_disease = underlying_disease.clone();
        let drug_allergy = drug_allergy.clone();
        let phone = phone.clone();
        let address = address.clone();
        let navigator = navigator.clone();
        let toast = toast.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            // Check if HN already exists (simple check)
            let existing_patients = Store::get_patients();
            if existing_patients.iter().any(|p| p.hn == *hn) {
                 if let Some(ref t) = toast {
                    t.dispatch(ToastAction::Add(
                        "❌ เลข HN นี้มีในระบบแล้ว".to_string(),
                        ToastType::Error
                    ));
                }
                return;
            }
            
            let bd = if (*birth_date).is_empty() {
                None
            } else {
                chrono::NaiveDate::parse_from_str(&birth_date, "%Y-%m-%d").ok()
            };
            
            let new_patient = Patient {
                id: Uuid::new_v4().to_string(),
                hn: (*hn).clone(),
                citizen_id: (*citizen_id).clone(),
                title: (*title).clone(),
                first_name: (*first_name).clone(),
                last_name: (*last_name).clone(),
                birth_date: bd,
                age: (*age).parse().ok(),
                blood_group: (*blood_group).clone(),
                underlying_disease: (*underlying_disease).clone(),
                drug_allergy: (*drug_allergy).clone(),
                phone: (*phone).clone(),
                address: (*address).clone(),
                created_at: Utc::now(),
            };

            Store::save_patient(new_patient);
            
            if let Some(ref t) = toast {
                t.dispatch(ToastAction::Add(
                    "✅ บันทึกข้อมูลผู้ป่วยเรียบร้อยแล้ว!".to_string(),
                    ToastType::Success
                ));
            }
            
            navigator.push(&crate::Route::Search);
        })
    };

    html! {
        <>
            <div class="page-header">
                <h1 class="page-title">{ "➕ ลงทะเบียนผู้ป่วยใหม่" }</h1>
                <p class="page-subtitle">{ "กรอกข้อมูลผู้ป่วยด้านล่าง" }</p>
            </div>
            
            <div class="card">
                <form onsubmit={onsubmit}>
                    <div class="grid grid-cols-2 gap-4">
                        // HN
                        <div class="form-group">
                            <label class="form-label">{ "เลข HN * (กรอกตามบัตร)" }</label>
                            <input type="text" value={(*hn).clone()} required=true
                                placeholder="เช่น 66001"
                                style="font-weight: bold; font-family: monospace;" 
                                oninput={
                                    let hn = hn.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        hn.set(input.value());
                                    })
                                } />
                        </div>
                        
                        // Title
                        <div class="form-group">
                            <label class="form-label">{ "คำนำหน้า" }</label>
                            <select onchange={
                                let title = title.clone();
                                Callback::from(move |e: Event| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    title.set(input.value());
                                })
                            }>
                                <option value="นาย" selected={*title == "นาย"}>{ "นาย" }</option>
                                <option value="นาง" selected={*title == "นาง"}>{ "นาง" }</option>
                                <option value="นางสาว" selected={*title == "นางสาว"}>{ "นางสาว" }</option>
                                <option value="เด็กชาย" selected={*title == "เด็กชาย"}>{ "เด็กชาย" }</option>
                                <option value="เด็กหญิง" selected={*title == "เด็กหญิง"}>{ "เด็กหญิง" }</option>
                            </select>
                        </div>
                        
                        // First Name
                        <div class="form-group">
                            <label class="form-label">{ "ชื่อ *" }</label>
                            <input type="text" required=true value={(*first_name).clone()}
                                placeholder="กรอกชื่อ"
                                oninput={
                                    let first_name = first_name.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        first_name.set(input.value());
                                    })
                                } />
                        </div>
                        
                        // Last Name
                        <div class="form-group">
                            <label class="form-label">{ "นามสกุล *" }</label>
                            <input type="text" required=true value={(*last_name).clone()}
                                placeholder="กรอกนามสกุล"
                                oninput={
                                    let last_name = last_name.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        last_name.set(input.value());
                                    })
                                } />
                        </div>

                        // Citizen ID (Optional)
                        <div class="form-group">
                            <label class="form-label">{ "เลขบัตรประชาชน (ไม่บังคับ)" }</label>
                            <input type="text" 
                                maxlength="13"
                                value={(*citizen_id).clone()}
                                placeholder="กรอกเลข 13 หลัก (ถ้ามี)"
                                oninput={
                                    let citizen_id = citizen_id.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        // Only allow digits, max 13
                                        let filtered = digits_max(&input.value(), 13);
                                        citizen_id.set(filtered.clone());
                                        input.set_value(&filtered);
                                    })
                                } />
                        </div>
                        
                        // Birth Date (Optional) - Auto-calculate age
                        <div class="form-group">
                            <label class="form-label">{ "วันเกิด (ไม่บังคับ)" }</label>
                            <input type="date" value={(*birth_date).clone()}
                                oninput={{
                                    let birth_date = birth_date.clone();
                                    let age = age.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        let bd_str = input.value();
                                        birth_date.set(bd_str.clone());
                                        
                                        // Auto-calculate age from birth date
                                        if let Ok(bd) = chrono::NaiveDate::parse_from_str(&bd_str, "%Y-%m-%d") {
                                            let today = chrono::Local::now().date_naive();
                                            let mut calculated_age = today.year() - bd.year();
                                            // Adjust if birthday hasn't occurred this year
                                            if today.month() < bd.month() || (today.month() == bd.month() && today.day() < bd.day()) {
                                                calculated_age -= 1;
                                            }
                                            if calculated_age >= 0 {
                                                age.set(calculated_age.to_string());
                                            }
                                        }
                                    })
                                }} />
                            <small style="color: #666;">{ "กรอกวันเกิดจะคำนวณอายุให้อัตโนมัติ" }</small>
                        </div>
                        
                        // Age (Optional)
                        <div class="form-group">
                            <label class="form-label">{ "อายุ (ปี)" }
                                { if !(*birth_date).is_empty() {
                                    html! { <span class="badge badge-success" style="margin-left: 0.5rem;">{ "คำนวณจากวันเกิด" }</span> }
                                } else { html! {} }}
                            </label>
                            <input type="number" min="0" max="150" value={(*age).clone()}
                                placeholder="เช่น 35"
                                oninput={
                                    let age = age.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        age.set(input.value());
                                    })
                                } />
                        </div>
                        
                        // Blood Group
                        <div class="form-group">
                            <label class="form-label">{ "กรุ๊ปเลือด" }</label>
                            <select onchange={
                                let blood_group = blood_group.clone();
                                Callback::from(move |e: Event| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    blood_group.set(input.value());
                                })
                            }>
                                <option value="ไม่ทราบ">{ "ไม่ทราบ" }</option>
                                <option value="A">{ "A" }</option>
                                <option value="B">{ "B" }</option>
                                <option value="AB">{ "AB" }</option>
                                <option value="O">{ "O" }</option>
                            </select>
                        </div>
                        
                        // Phone (Optional)
                        <div class="form-group">
                            <label class="form-label">{ "เบอร์โทรศัพท์" }</label>
                            <input type="tel" 
                                maxlength="10"
                                value={(*phone).clone()}
                                placeholder="08X-XXX-XXXX"
                                oninput={
                                    let phone = phone.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        let filtered = digits_max(&input.value(), 10);
                                        phone.set(filtered.clone());
                                        input.set_value(&filtered);
                                    })
                                } />
                        </div>
                        
                        // Drug Allergy
                        <div class="form-group">
                            <label class="form-label">
                                { "แพ้ยา" }
                                <span class="badge badge-error" style="margin-left: 5px;">{ "สำคัญ" }</span>
                            </label>
                            <input type="text" value={(*drug_allergy).clone()}
                                placeholder="ระบุชื่อยาที่แพ้ หรือ 'ไม่มี'"
                                oninput={
                                    let drug_allergy = drug_allergy.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        drug_allergy.set(input.value());
                                    })
                                } />
                        </div>

                        // Underlying Disease (NEW)
                        <div class="form-group">
                            <label class="form-label">{ "โรคประจำตัว" }</label>
                            <input type="text" value={(*underlying_disease).clone()}
                                placeholder="เช่น เบาหวาน, ความดัน (ถ้ามี)"
                                oninput={
                                    let underlying_disease = underlying_disease.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        underlying_disease.set(input.value());
                                    })
                                } />
                        </div>
                        
                        // Address
                        <div class="form-group" style="grid-column: 1 / -1;">
                            <label class="form-label">{ "ที่อยู่" }</label>
                            <textarea value={(*address).clone()}
                                placeholder="กรอกที่อยู่ (ถ้ามี)"
                                oninput={
                                    let address = address.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        address.set(input.value());
                                    })
                                } />
                        </div>
                    </div>
                    
                    <div class="flex justify-between items-center mt-6">
                        <div>
                            { if !form_valid {
                                html! { <p class="text-warning">{ "⚠️ กรุณากรอกข้อมูลที่มี * ให้ครบ" }</p> }
                            } else { 
                                html! { <p class="text-success">{ "✅ พร้อมบันทึก" }</p> }
                            }}
                        </div>
                        <button type="submit" class="btn btn-primary btn-lg" disabled={!form_valid}>
                            { "💾 บันทึกข้อมูล" }
                        </button>
                    </div>
                </form>
            </div>
        </>
    }
}
