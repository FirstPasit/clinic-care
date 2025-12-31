use yew::prelude::*;
use crate::models::ClinicSettings;
use crate::store::Store;
use crate::components::{ToastContext, ToastAction, ToastType};
use web_sys::{HtmlInputElement, Blob, Url, HtmlAnchorElement};
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};

// Helper to filter non-digits
fn digits_only(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_digit()).collect()
}

// Helper to filter digits with max length
fn digits_max(s: &str, max: usize) -> String {
    digits_only(s).chars().take(max).collect()
}

fn apply_font_size(size: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(html) = document.document_element() {
                let _ = html.class_list().remove_3("font-medium", "font-large", "font-xlarge");
                let class = match size {
                    "medium" => "font-medium",
                    "large" => "font-large",
                    "xlarge" => "font-xlarge",
                    _ => "font-large",
                };
                let _ = html.class_list().add_1(class);
            }
        }
    }
}

// Backup data structure
#[derive(Serialize, Deserialize)]
struct BackupData {
    version: u32,
    backup_date: String,
    patients: Vec<crate::models::Patient>,
    records: Vec<crate::models::TreatmentRecord>,
    drugs: Vec<crate::models::DrugItem>,
    settings: ClinicSettings,
}

fn create_backup() -> String {
    let backup = BackupData {
        version: 1,
        backup_date: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        patients: Store::get_patients(),
        records: Store::get_records(),
        drugs: Store::get_drugs(),
        settings: Store::get_settings(),
    };
    serde_json::to_string_pretty(&backup).unwrap_or_default()
}

fn download_backup(data: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // Create blob
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&wasm_bindgen::JsValue::from_str(data));
    
    let blob_options = web_sys::BlobPropertyBag::new();
    blob_options.set_type("application/json");
    
    if let Ok(blob) = Blob::new_with_str_sequence_and_options(&blob_parts, &blob_options) {
        if let Ok(url) = Url::create_object_url_with_blob(&blob) {
            // Create download link
            let a: HtmlAnchorElement = document.create_element("a").unwrap().unchecked_into();
            let filename = format!("clinic_backup_{}.json", chrono::Local::now().format("%Y%m%d_%H%M%S"));
            a.set_href(&url);
            a.set_download(&filename);
            a.click();
            let _ = Url::revoke_object_url(&url);
        }
    }
}

fn restore_from_json(json: &str) -> Result<(usize, usize, usize), String> {
    let backup: BackupData = serde_json::from_str(json)
        .map_err(|e| format!("ไฟล์ไม่ถูกต้อง: {}", e))?;
    
    // Restore all data
    let patient_count = backup.patients.len();
    let record_count = backup.records.len();
    let drug_count = backup.drugs.len();
    
    for patient in backup.patients {
        Store::save_patient(patient);
    }
    
    for record in backup.records {
        Store::save_record(record);
    }
    
    for drug in backup.drugs {
        Store::save_drug(drug);
    }
    
    Store::save_settings(backup.settings);
    
    Ok((patient_count, record_count, drug_count))
}

async fn invoke_check_update() -> Result<String, String> {
    use wasm_bindgen::JsValue;
    use wasm_bindgen::JsCast;
    
    // ... code truncated for brevity, assume content is same as before but without duplicate Ok line ...
    // Wait, I can't express "same as before" here. I must provide the full content or range.
    // Let's fix the duplicate line first.
    let window = web_sys::window().unwrap();
    // ...
    // Actually the duplicate was at line 106-107, which is "Ok((patient_count, record_count, drug_count))"
    
    // And for ToastType::Info (line 301 and 308 in original file approx), change to Success or Error.
    // "กำลังตรวจสอบ..." -> Success (with message "Checking...") or just don't show toast?
    // Let's use Success but with different icon if possible? No, toast component hardcodes icon.
    // Just use Success for now.

    // I will redo the function invoke_check_update import fully to be safe.
    
    let window = web_sys::window().unwrap();
    let tauri = js_sys::Reflect::get(&window, &JsValue::from_str("__TAURI__"))
        .map_err(|_| "Tauri API not found".to_string())?;
        
    if tauri.is_undefined() {
        return Err("App is not running in Tauri context".to_string());
    }
    
    // Try to find invoke function (supports both v1 and v2 structures)
    let core = js_sys::Reflect::get(&tauri, &JsValue::from_str("core")).unwrap_or(JsValue::UNDEFINED);
    let invoke_fn = if !core.is_undefined() {
        js_sys::Reflect::get(&core, &JsValue::from_str("invoke")).unwrap()
    } else {
         let tauri_mod = js_sys::Reflect::get(&tauri, &JsValue::from_str("tauri")).unwrap_or(JsValue::UNDEFINED);
         if !tauri_mod.is_undefined() {
             js_sys::Reflect::get(&tauri_mod, &JsValue::from_str("invoke")).unwrap()
         } else {
             return Err("Invoke function not found".to_string());
         }
    };
    
    let invoke_fn_func = invoke_fn.dyn_into::<js_sys::Function>()
        .map_err(|_| "Invoke is not a function".to_string())?;
    
    // Call invoke("check_for_updates")
    let cmd = JsValue::from_str("check_for_updates");
    let promise = invoke_fn_func.call1(&JsValue::NULL, &cmd)
        .map_err(|e| format!("{:?}", e))?;
        
    // wasm_bindgen_futures::JsFuture requires imports in Cargo.toml. 
    // I Will handle Cargo.toml in next step.
    let result = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(promise))
        .await
        .map_err(|e| format!("{:?}", e))?;
        
    Ok(result.as_string().unwrap_or_default())
}

#[function_component(Settings)]
pub fn settings() -> Html {
    let toast = use_context::<ToastContext>();
    let settings = use_state(|| Store::get_settings());
    
    let clinic_name = use_state(|| settings.clinic_name.clone());
    let clinic_address = use_state(|| settings.clinic_address.clone());
    let clinic_phone = use_state(|| settings.clinic_phone.clone());
    let clinic_tax_id = use_state(|| settings.clinic_tax_id.clone());
    let staff_name = use_state(|| settings.staff_name.clone());
    let staff_position = use_state(|| settings.staff_position.clone());
    let license_number = use_state(|| settings.license_number.clone());
    let font_size = use_state(|| settings.font_size.clone());
    let sticker_size = use_state(|| settings.sticker_size.clone());
    
    // Stats for display
    let patient_count = Store::get_patients().len();
    let record_count = Store::get_records().len();
    let drug_count = Store::get_drugs().len();
    
    // Validation
    let tax_id_valid = (*clinic_tax_id).is_empty() || (*clinic_tax_id).len() == 13;
    let phone_valid = (*clinic_phone).is_empty() || (*clinic_phone).len() >= 9;
    
    // Apply font size immediately when changed
    let on_font_change = {
        let font_size = font_size.clone();
        Callback::from(move |e: Event| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            font_size.set(value.clone());
            apply_font_size(&value);
        })
    };
    
    let on_save = {
        let clinic_name = clinic_name.clone();
        let clinic_address = clinic_address.clone();
        let clinic_phone = clinic_phone.clone();
        let clinic_tax_id = clinic_tax_id.clone();
        let staff_name = staff_name.clone();
        let staff_position = staff_position.clone();
        let license_number = license_number.clone();
        let font_size = font_size.clone();
        let sticker_size = sticker_size.clone();
        let settings = settings.clone();
        let toast = toast.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let new_settings = ClinicSettings {
                clinic_name: (*clinic_name).clone(),
                clinic_address: (*clinic_address).clone(),
                clinic_phone: (*clinic_phone).clone(),
                clinic_tax_id: (*clinic_tax_id).clone(),
                staff_name: (*staff_name).clone(),
                staff_position: (*staff_position).clone(),
                license_number: (*license_number).clone(),
                font_size: (*font_size).clone(),
                theme: "light".to_string(),
                sticker_size: (*sticker_size).clone(),
                next_receipt_no: settings.next_receipt_no,
            };
            
            Store::save_settings(new_settings.clone());
            settings.set(new_settings);
            
            if let Some(ref t) = toast {
                t.dispatch(ToastAction::Add(
                    "✅ บันทึกการตั้งค่าเรียบร้อยแล้ว!".to_string(),
                    ToastType::Success
                ));
            }
        })
    };
    
    // Backup handler
    let on_backup = {
        let toast = toast.clone();
        Callback::from(move |_: MouseEvent| {
            let backup_data = create_backup();
            download_backup(&backup_data);
            
            if let Some(ref t) = toast {
                t.dispatch(ToastAction::Add(
                    "📥 ดาวน์โหลดไฟล์สำรองข้อมูลเรียบร้อย!".to_string(),
                    ToastType::Success
                ));
            }
        })
    };
    
    // Restore handler
    let on_restore = {
        let toast = toast.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let toast = toast.clone();
            
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let reader = web_sys::FileReader::new().unwrap();
                    let reader_clone = reader.clone();
                    
                    let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                        if let Ok(result) = reader_clone.result() {
                            if let Some(text) = result.as_string() {
                                match restore_from_json(&text) {
                                    Ok((patients, records, drugs)) => {
                                        if let Some(ref t) = toast {
                                            t.dispatch(ToastAction::Add(
                                                format!("✅ กู้คืนข้อมูลเรียบร้อย! ผู้ป่วย {} คน, การรักษา {} รายการ, ยา {} รายการ", 
                                                    patients, records, drugs),
                                                ToastType::Success
                                            ));
                                        }
                                        // Reload page to show restored data
                                        let _ = web_sys::window().unwrap().location().reload();
                                    }
                                    Err(err) => {
                                        if let Some(ref t) = toast {
                                            t.dispatch(ToastAction::Add(
                                                format!("❌ กู้คืนล้มเหลว: {}", err),
                                                ToastType::Error
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }) as Box<dyn FnMut(_)>);
                    
                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    onload.forget();
                    
                    let _ = reader.read_as_text(&file);
                }
            }
            
            // Clear the input so same file can be selected again
            input.set_value("");
        })
    };

    let on_check_update = {
        let toast = toast.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let toast = toast.clone();
            
            if let Some(ref t) = toast {
                t.dispatch(ToastAction::Add("⏳ กำลังตรวจสอบการอัปเดต...".to_string(), ToastType::Success));
            }

            wasm_bindgen_futures::spawn_local(async move {
                match invoke_check_update().await {
                   Ok(msg) => {
                       if let Some(ref t) = toast {
                           t.dispatch(ToastAction::Add(msg, ToastType::Success));
                       }
                   },
                   Err(err) => {
                       if let Some(ref t) = toast {
                           t.dispatch(ToastAction::Add(format!("❌ เกิดข้อผิดพลาด: {}", err), ToastType::Error));
                       }
                   }
                }
            });
        })
    };

    html! {
        <>
            <div class="page-header">
                <h1 class="page-title">{ "⚙️ ตั้งค่าคลินิก" }</h1>
                <p class="page-subtitle">{ "ปรับแต่งข้อมูลคลินิกและการแสดงผล" }</p>
            </div>
            
            // BACKUP SECTION - IMPORTANT!
            <div class="card mb-6" style="border: 3px solid var(--color-warning); background: var(--color-warning-light);">
                <div class="card-header">
                    <h3 class="card-title">{ "💾 สำรองข้อมูล (สำคัญมาก!)" }</h3>
                    <p class="card-subtitle">{ "สำรองข้อมูลเป็นไฟล์ไว้ในคอม เพื่อไม่ให้ข้อมูลหาย" }</p>
                </div>
                
                <div class="alert alert-warning mb-4">
                    <span class="alert-icon">{ "⚠️" }</span>
                    <span>{ "ข้อมูลเก็บใน Browser ถ้าล้าง Cache หรือเปลี่ยน Browser ข้อมูลจะหาย! กรุณาสำรองข้อมูลเป็นประจำ" }</span>
                </div>
                
                <div class="stats-grid" style="margin-bottom: 1rem;">
                    <div class="stat-card">
                        <div class="stat-card-icon accent">{ "👥" }</div>
                        <div class="stat-card-value">{ patient_count }</div>
                        <div class="stat-card-label">{ "ผู้ป่วย" }</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-card-icon success">{ "📋" }</div>
                        <div class="stat-card-value">{ record_count }</div>
                        <div class="stat-card-label">{ "การรักษา" }</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-card-icon warning">{ "💊" }</div>
                        <div class="stat-card-value">{ drug_count }</div>
                        <div class="stat-card-label">{ "รายการยา" }</div>
                    </div>
                </div>
                
                <div class="grid grid-cols-2 gap-4">
                    <div>
                        <button class="btn btn-primary btn-lg w-full" onclick={on_backup} 
                            style="font-size: 1.3rem; padding: 1.5rem;">
                            { "📥 ดาวน์โหลดไฟล์สำรอง" }
                        </button>
                        <p class="text-muted" style="margin-top: 0.5rem; text-align: center;">
                            { "คลิกเพื่อดาวน์โหลดไฟล์ .json" }
                        </p>
                    </div>
                    
                    <div>
                        <label class="btn btn-secondary btn-lg w-full" 
                            style="font-size: 1.3rem; padding: 1.5rem; cursor: pointer;">
                            { "📤 กู้คืนจากไฟล์สำรอง" }
                            <input type="file" accept=".json" onchange={on_restore}
                                style="display: none;" />
                        </label>
                        <p class="text-muted" style="margin-top: 0.5rem; text-align: center;">
                            { "เลือกไฟล์ .json ที่เคยสำรองไว้" }
                        </p>
                    </div>
                </div>
                
                <div class="alert alert-success" style="margin-top: 1rem;">
                    <span class="alert-icon">{ "💡" }</span>
                    <span>{ "แนะนำ: สำรองข้อมูลทุกวัน หรืออย่างน้อยสัปดาห์ละครั้ง และเก็บไฟล์ไว้หลายๆ ที่" }</span>
                </div>
            </div>
            
            <form onsubmit={on_save}>
                // Clinic Information
                <div class="card mb-6">
                    <div class="card-header">
                        <h3 class="card-title">{ "🏥 ข้อมูลคลินิก" }</h3>
                        <p class="card-subtitle">{ "ข้อมูลนี้จะแสดงในใบเสร็จและเอกสารต่างๆ" }</p>
                    </div>
                    
                    <div class="grid grid-cols-2 gap-4">
                        <div class="form-group" style="grid-column: 1 / -1;">
                            <label class="form-label">{ "ชื่อคลินิก" }</label>
                            <input type="text" value={(*clinic_name).clone()}
                                placeholder="เช่น คลินิกหมอสมชาย"
                                oninput={{
                                    let clinic_name = clinic_name.clone();
                                    Callback::from(move |e: InputEvent| {
                                        clinic_name.set(e.target_unchecked_into::<HtmlInputElement>().value());
                                    })
                                }} />
                        </div>
                        
                        <div class="form-group" style="grid-column: 1 / -1;">
                            <label class="form-label">{ "ที่อยู่" }</label>
                            <textarea value={(*clinic_address).clone()}
                                placeholder="ที่อยู่เต็ม รวมรหัสไปรษณีย์"
                                oninput={{
                                    let clinic_address = clinic_address.clone();
                                    Callback::from(move |e: InputEvent| {
                                        clinic_address.set(e.target_unchecked_into::<HtmlInputElement>().value());
                                    })
                                }} />
                        </div>
                        
                        <div class="form-group">
                            <label class="form-label">
                                { "เบอร์โทรศัพท์ " }
                                { if !(*clinic_phone).is_empty() && !phone_valid {
                                    html! { <span class="badge badge-warning">{ "ควรมี 9-10 หลัก" }</span> }
                                } else { html! {} }}
                            </label>
                            <input 
                                type="tel" 
                                inputmode="numeric"
                                maxlength="10"
                                value={(*clinic_phone).clone()}
                                placeholder="02-XXX-XXXX หรือ 08X-XXX-XXXX"
                                oninput={{
                                    let clinic_phone = clinic_phone.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        let filtered = digits_max(&input.value(), 10);
                                        clinic_phone.set(filtered.clone());
                                        input.set_value(&filtered);
                                    })
                                }} />
                        </div>
                        
                        <div class="form-group">
                            <label class="form-label">
                                { "เลขประจำตัวผู้เสียภาษี " }
                                { if !(*clinic_tax_id).is_empty() {
                                    html! {
                                        <span class={if tax_id_valid { "badge badge-success" } else { "badge badge-warning" }}>
                                            { format!("{}/13 หลัก", (*clinic_tax_id).len()) }
                                        </span>
                                    }
                                } else { html! {} }}
                            </label>
                            <input 
                                type="text" 
                                inputmode="numeric"
                                pattern="[0-9]*"
                                maxlength="13"
                                value={(*clinic_tax_id).clone()}
                                placeholder="13 หลัก"
                                oninput={{
                                    let clinic_tax_id = clinic_tax_id.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        let filtered = digits_max(&input.value(), 13);
                                        clinic_tax_id.set(filtered.clone());
                                        input.set_value(&filtered);
                                    })
                                }} />
                        </div>
                    </div>
                </div>
                
                // Staff Information for Certificates
                <div class="card mb-6">
                    <div class="card-header">
                        <h3 class="card-title">{ "👩‍⚕️ ข้อมูลผู้ออกเอกสาร" }</h3>
                        <p class="card-subtitle">{ "ข้อมูลนี้จะแสดงในใบรับรองแพทย์และเอกสารอื่นๆ" }</p>
                    </div>
                    
                    <div class="grid grid-cols-2 gap-4">
                        <div class="form-group" style="grid-column: 1 / -1;">
                            <label class="form-label">{ "ชื่อ-นามสกุล (พยาบาล/แพทย์)" }</label>
                            <input type="text" value={(*staff_name).clone()}
                                placeholder="เช่น นางสมหญิง วีระจินตนา"
                                oninput={{
                                    let staff_name = staff_name.clone();
                                    Callback::from(move |e: InputEvent| {
                                        staff_name.set(e.target_unchecked_into::<HtmlInputElement>().value());
                                    })
                                }} />
                        </div>
                        
                        <div class="form-group">
                            <label class="form-label">{ "ตำแหน่ง" }</label>
                            <input type="text" value={(*staff_position).clone()}
                                placeholder="เช่น พยาบาลวิชาชีพชำนาญการ"
                                oninput={{
                                    let staff_position = staff_position.clone();
                                    Callback::from(move |e: InputEvent| {
                                        staff_position.set(e.target_unchecked_into::<HtmlInputElement>().value());
                                    })
                                }} />
                        </div>
                        
                        <div class="form-group">
                            <label class="form-label">{ "เลขใบอนุญาตประกอบวิชาชีพ" }</label>
                            <input type="text" value={(*license_number).clone()}
                                placeholder="เช่น 4511055362"
                                oninput={{
                                    let license_number = license_number.clone();
                                    Callback::from(move |e: InputEvent| {
                                        license_number.set(e.target_unchecked_into::<HtmlInputElement>().value());
                                    })
                                }} />
                        </div>
                    </div>
                    
                    <div class="alert alert-success" style="margin-top: 1rem;">
                        <span class="alert-icon">{ "💡" }</span>
                        <span>{ "ข้อมูลนี้จะเติมอัตโนมัติในใบรับรองแพทย์ ไม่ต้องเขียนเพิ่ม" }</span>
                    </div>
                </div>
                
                // Display Settings
                <div class="card mb-6">
                    <div class="card-header">
                        <h3 class="card-title">{ "🎨 การแสดงผล" }</h3>
                        <p class="card-subtitle">{ "ปรับขนาดตัวอักษรให้เหมาะกับการใช้งาน (เปลี่ยนทันที)" }</p>
                    </div>
                    
                    <div class="grid grid-cols-2 gap-4">
                        <div class="form-group">
                            <label class="form-label">{ "ขนาดตัวอักษร" }</label>
                            <select onchange={on_font_change}>
                                <option value="medium" selected={*font_size == "medium"}>{ "ปกติ" }</option>
                                <option value="large" selected={*font_size == "large"}>{ "ใหญ่ (แนะนำ)" }</option>
                                <option value="xlarge" selected={*font_size == "xlarge"}>{ "ใหญ่มาก" }</option>
                            </select>
                        </div>
                        
                        <div class="form-group">
                            <label class="form-label">{ "ขนาดสติกเกอร์เริ่มต้น" }</label>
                            <select onchange={{
                                let sticker_size = sticker_size.clone();
                                Callback::from(move |e: Event| {
                                    sticker_size.set(e.target_unchecked_into::<HtmlInputElement>().value());
                                })
                            }}>
                                <option value="small" selected={*sticker_size == "small"}>{ "เล็ก (60x30mm)" }</option>
                                <option value="medium" selected={*sticker_size == "medium"}>{ "กลาง (80x40mm)" }</option>
                                <option value="large" selected={*sticker_size == "large"}>{ "ใหญ่ (100x50mm)" }</option>
                            </select>
                        </div>
                    </div>
                    
                    <div class="alert alert-success" style="margin-top: 1rem;">
                        <span class="alert-icon">{ "👀" }</span>
                        <span>{ "ขนาดตัวอักษรจะเปลี่ยนทันทีเมื่อเลือก ลองดูขนาดที่เหมาะสมแล้วกดบันทึก" }</span>
                    </div>
                </div>
                
                // Receipt Number Info
                <div class="card mb-6">
                    <div class="card-header">
                        <h3 class="card-title">{ "🧾 เลขที่ใบเสร็จ" }</h3>
                    </div>
                    <div class="flex items-center gap-4">
                        <div>
                            <p>{ "เลขที่ใบเสร็จล่าสุด:" }</p>
                            <p style="font-size: 1.5rem; font-weight: 700;" class="font-mono">
                                { format!("RCP-{}-{:04}", chrono::Local::now().format("%Y%m%d"), settings.next_receipt_no) }
                            </p>
                        </div>
                        <p class="text-muted" style="flex: 1;">
                            { "เลขที่ใบเสร็จจะเพิ่มขึ้นอัตโนมัติทุกครั้งที่ออกใบเสร็จ" }
                        </p>
                    </div>
                </div>
                
                // System Update Section
                <div class="card mb-6">
                    <div class="card-header">
                        <h3 class="card-title">{ "🔄 ระบบและการอัปเดต" }</h3>
                        <p class="card-subtitle">{ "ตรวจสอบเวอร์ชันใหม่ของโปรแกรม" }</p>
                    </div>
                    <div class="flex items-center justify-between">
                        <div>
                            <p>{ "เวอร์ชันปัจจุบัน: " }<strong>{ "1.0.6" }</strong></p>
                            <p class="text-muted">{ "หากมีเวอร์ชันใหม่ ระบบจะทำการดาวน์โหลดและติดตั้งให้ โดยท่านต้องเปิดแอปใหม่เอง" }</p>
                        </div>
                        <button type="button" class="btn btn-secondary" onclick={on_check_update}>
                            { "🔄 ตรวจสอบอัปเดต" }
                        </button>
                    </div>
                </div>
                
                // Save Button
                <div class="flex justify-between items-center">
                    <p class="text-muted">{ "⚡ ขนาดตัวอักษรเปลี่ยนทันที กด \"บันทึก\" เพื่อจำค่าถาวร" }</p>
                    <button type="submit" class="btn btn-success btn-lg">
                        { "💾 บันทึกการตั้งค่า" }
                    </button>
                </div>
            </form>
        </>
    }
}
