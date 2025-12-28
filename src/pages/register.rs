use yew::prelude::*;
use web_sys::HtmlInputElement;
use chrono::Utc;
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
    let hn = use_state(|| Store::next_hn());
    let citizen_id = use_state(|| String::new());
    let title = use_state(|| "นาย".to_string());
    let first_name = use_state(|| String::new());
    let last_name = use_state(|| String::new());
    let birth_date = use_state(|| String::new());
    let blood_group = use_state(|| "A".to_string());
    let drug_allergy = use_state(|| String::new());
    let phone = use_state(|| String::new());
    let address = use_state(|| String::new());
    
    // Validation states
    let citizen_id_valid = (*citizen_id).len() == 13;
    let phone_valid = (*phone).len() == 10;
    let form_valid = citizen_id_valid && !(*first_name).is_empty() && !(*last_name).is_empty();

    let onsubmit = {
        let hn = hn.clone();
        let citizen_id = citizen_id.clone();
        let title = title.clone();
        let first_name = first_name.clone();
        let last_name = last_name.clone();
        let birth_date = birth_date.clone();
        let blood_group = blood_group.clone();
        let drug_allergy = drug_allergy.clone();
        let phone = phone.clone();
        let address = address.clone();
        let navigator = navigator.clone();
        let toast = toast.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            // Validate citizen ID
            if (*citizen_id).len() != 13 {
                if let Some(ref t) = toast {
                    t.dispatch(ToastAction::Add(
                        "❌ กรุณากรอกเลขบัตรประชาชน 13 หลัก".to_string(),
                        ToastType::Error
                    ));
                }
                return;
            }
            
            let bd = chrono::NaiveDate::parse_from_str(&birth_date, "%Y-%m-%d").ok();
            
            let new_patient = Patient {
                id: Uuid::new_v4().to_string(),
                hn: (*hn).clone(),
                citizen_id: (*citizen_id).clone(),
                title: (*title).clone(),
                first_name: (*first_name).clone(),
                last_name: (*last_name).clone(),
                birth_date: bd,
                blood_group: (*blood_group).clone(),
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
                        <div class="form-group" style="grid-column: 1 / -1;">
                            <label class="form-label">{ "เลข HN (สร้างอัตโนมัติ)" }</label>
                            <input type="text" value={(*hn).clone()} readonly=true 
                                style="background: var(--color-bg); font-weight: bold; font-family: monospace;" />
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
                                <option value="นาย">{ "นาย" }</option>
                                <option value="นาง">{ "นาง" }</option>
                                <option value="นางสาว">{ "นางสาว" }</option>
                                <option value="เด็กชาย">{ "เด็กชาย" }</option>
                                <option value="เด็กหญิง">{ "เด็กหญิง" }</option>
                            </select>
                        </div>
                        
                        // Citizen ID - EXACTLY 13 DIGITS
                        <div class="form-group">
                            <label class="form-label">
                                { "เลขบัตรประชาชน " }
                                <span class={if citizen_id_valid { "badge badge-success" } else { "badge badge-warning" }}>
                                    { format!("{}/13 หลัก", (*citizen_id).len()) }
                                </span>
                            </label>
                            <input 
                                type="text" 
                                inputmode="numeric"
                                pattern="[0-9]*"
                                maxlength="13"
                                value={(*citizen_id).clone()}
                                placeholder="กรอกเลข 13 หลัก"
                                class={if !citizen_id_valid && !(*citizen_id).is_empty() { "input-error" } else { "" }}
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
                            { if !citizen_id_valid && !(*citizen_id).is_empty() {
                                html! { <span class="form-error">{ "⚠️ ต้องครบ 13 หลัก" }</span> }
                            } else { html! {} }}
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
                        
                        // Birth Date
                        <div class="form-group">
                            <label class="form-label">{ "วันเกิด" }</label>
                            <input type="date" value={(*birth_date).clone()}
                                oninput={
                                    let birth_date = birth_date.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        birth_date.set(input.value());
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
                                <option value="A">{ "A" }</option>
                                <option value="B">{ "B" }</option>
                                <option value="AB">{ "AB" }</option>
                                <option value="O">{ "O" }</option>
                            </select>
                        </div>
                        
                        // Phone - EXACTLY 10 DIGITS
                        <div class="form-group">
                            <label class="form-label">
                                { "เบอร์โทรศัพท์ " }
                                { if !(*phone).is_empty() {
                                    html! {
                                        <span class={if phone_valid { "badge badge-success" } else { "badge badge-warning" }}>
                                            { format!("{}/10 หลัก", (*phone).len()) }
                                        </span>
                                    }
                                } else { html! {} }}
                            </label>
                            <input 
                                type="tel" 
                                inputmode="numeric"
                                pattern="[0-9]*"
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
                                { "แพ้ยา " }
                                <span class="badge badge-error">{ "สำคัญ" }</span>
                            </label>
                            <input type="text" value={(*drug_allergy).clone()}
                                placeholder="เช่น Penicillin, Aspirin หรือ ไม่มี"
                                oninput={
                                    let drug_allergy = drug_allergy.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        drug_allergy.set(input.value());
                                    })
                                } />
                        </div>
                        
                        // Address
                        <div class="form-group" style="grid-column: 1 / -1;">
                            <label class="form-label">{ "ที่อยู่" }</label>
                            <textarea value={(*address).clone()}
                                placeholder="กรอกที่อยู่"
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
