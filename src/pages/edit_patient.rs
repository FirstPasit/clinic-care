use yew::prelude::*;
use web_sys::HtmlInputElement;
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

#[derive(Properties, PartialEq)]
pub struct Props {
    pub patient_id: String,
}

#[function_component(EditPatient)]
pub fn edit_patient(props: &Props) -> Html {
    let navigator = yew_router::prelude::use_navigator().unwrap();
    let toast = use_context::<ToastContext>();
    
    // Get existing patient
    let existing_patient = Store::get_patients()
        .into_iter()
        .find(|p| p.id == props.patient_id);
    
    if existing_patient.is_none() {
        return html! {
            <div class="card">
                <p>{ "❌ ไม่พบข้อมูลผู้ป่วย" }</p>
                <button class="btn btn-secondary" onclick={move |_| navigator.back()}>
                    { "← ย้อนกลับ" }
                </button>
            </div>
        };
    }
    
    let patient = existing_patient.unwrap();
    
    // Form state initialized from existing data
    let patient_id = use_state(|| patient.id.clone());
    let hn = use_state(|| patient.hn.clone());
    let citizen_id = use_state(|| patient.citizen_id.clone());
    let title = use_state(|| patient.title.clone());
    let first_name = use_state(|| patient.first_name.clone());
    let last_name = use_state(|| patient.last_name.clone());
    let birth_date = use_state(|| {
        patient.birth_date.map(|d| d.to_string()).unwrap_or_default()
    });
    let age = use_state(|| {
        patient.age.map(|a| a.to_string()).unwrap_or_default()
    });
    let blood_group = use_state(|| patient.blood_group.clone());
    let underlying_disease = use_state(|| patient.underlying_disease.clone());
    let drug_allergy = use_state(|| patient.drug_allergy.clone());
    let phone = use_state(|| patient.phone.clone());
    let address = use_state(|| patient.address.clone());
    let created_at = use_state(|| patient.created_at);
    
    // Validation
    let form_valid = !(*hn).is_empty() && !(*first_name).is_empty() && !(*last_name).is_empty();

    let onsubmit = {
        let patient_id = patient_id.clone();
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
        let created_at = created_at.clone();
        let navigator = navigator.clone();
        let toast = toast.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let bd = if (*birth_date).is_empty() {
                None
            } else {
                chrono::NaiveDate::parse_from_str(&birth_date, "%Y-%m-%d").ok()
            };
            
            let updated_patient = Patient {
                id: (*patient_id).clone(),
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
                created_at: *created_at,
            };

            Store::update_patient(updated_patient);
            
            if let Some(ref t) = toast {
                t.dispatch(ToastAction::Add(
                    "✅ บันทึกการแก้ไขเรียบร้อยแล้ว!".to_string(),
                    ToastType::Success
                ));
            }
            
            navigator.back();
        })
    };

    html! {
        <>
            <div class="page-header">
                <h1 class="page-title">{ "✏️ แก้ไขข้อมูลผู้ป่วย" }</h1>
                <p class="page-subtitle">{ format!("HN: {}", *hn) }</p>
            </div>
            
            <div class="card">
                <form onsubmit={onsubmit}>
                    <div class="grid grid-cols-2 gap-4">
                        // HN (Read-only)
                        <div class="form-group">
                            <label class="form-label">{ "เลข HN" }</label>
                            <input type="text" value={(*hn).clone()} disabled=true
                                style="font-weight: bold; font-family: monospace; background: #f0f0f0;" />
                            <small style="color: #666;">{ "* ไม่สามารถแก้ไขเลข HN ได้" }</small>
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

                        // Citizen ID
                        <div class="form-group">
                            <label class="form-label">{ "เลขบัตรประชาชน" }</label>
                            <input type="text" 
                                maxlength="13"
                                value={(*citizen_id).clone()}
                                placeholder="กรอกเลข 13 หลัก"
                                oninput={
                                    let citizen_id = citizen_id.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        let filtered = digits_max(&input.value(), 13);
                                        citizen_id.set(filtered.clone());
                                        input.set_value(&filtered);
                                    })
                                } />
                        </div>
                        
                        // Age
                        <div class="form-group">
                            <label class="form-label">{ "อายุ (ปี)" }</label>
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
                                <option value="ไม่ทราบ" selected={*blood_group == "ไม่ทราบ"}>{ "ไม่ทราบ" }</option>
                                <option value="A" selected={*blood_group == "A"}>{ "A" }</option>
                                <option value="B" selected={*blood_group == "B"}>{ "B" }</option>
                                <option value="AB" selected={*blood_group == "AB"}>{ "AB" }</option>
                                <option value="O" selected={*blood_group == "O"}>{ "O" }</option>
                            </select>
                        </div>
                        
                        // Phone
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

                        // Underlying Disease
                        <div class="form-group">
                            <label class="form-label">{ "โรคประจำตัว" }</label>
                            <input type="text" value={(*underlying_disease).clone()}
                                placeholder="เช่น เบาหวาน, ความดัน"
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
                        <button type="button" class="btn btn-secondary btn-lg" onclick={move |_| navigator.back()}>
                            { "← ยกเลิก" }
                        </button>
                        <div>
                            { if !form_valid {
                                html! { <span class="text-warning" style="margin-right: 1rem;">{ "⚠️ กรุณากรอกข้อมูลที่มี * ให้ครบ" }</span> }
                            } else { html! {} }}
                            <button type="submit" class="btn btn-primary btn-lg" disabled={!form_valid}>
                                { "💾 บันทึกการแก้ไข" }
                            </button>
                        </div>
                    </div>
                </form>
            </div>
        </>
    }
}
