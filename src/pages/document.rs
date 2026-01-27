use yew::prelude::*;
use crate::models::TreatmentRecord;
use crate::store::Store;
use chrono::prelude::*;
use gloo::timers::callback::Timeout;
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub doc_type: String,
    pub id: String,
}

#[function_component(Document)]
pub fn document(props: &Props) -> Html {
    let record = use_state(|| -> Option<TreatmentRecord> {
        Store::get_records().into_iter().find(|r| r.id == props.id)
    });
    
    let settings = Store::get_settings();
    let navigator = use_navigator().unwrap();

    // Auto print on load
    use_effect_with((), move |_| {
         let timeout = Timeout::new(500, move || {
             let _ = web_sys::window().unwrap().print();
         });
         timeout.forget();
         || ()
    });

    if record.is_none() {
        return html! { <div class="print-document"><p>{ "ไม่พบข้อมูล" }</p></div> };
    }
    let r = record.as_ref().unwrap();
    let patient = Store::get_patients().into_iter().find(|p| p.id == r.patient_id);
    if patient.is_none() {
        return html! { <div class="print-document"><p>{ "ไม่พบข้อมูลผู้ป่วย" }</p></div> };
    }
    let p = patient.unwrap();
    let date_str = r.date.with_timezone(&Local).format("%d/%m/%Y เวลา %H:%M น.").to_string();
    let date_only = r.date.with_timezone(&Local).format("%d/%m/%Y").to_string();
    
    // Generate receipt number
    let receipt_no = format!("RCP-{}-{}", 
        r.date.with_timezone(&Local).format("%Y%m%d"),
        &r.id[0..4].to_uppercase()
    );
    
    // Calculate itemized costs
    let _drug_count = r.prescriptions.len();
    let service_fee = 50.0; // Fixed Service Fee
    let drug_cost = (r.price - service_fee).max(0.0);
    
    let content = match props.doc_type.as_str() {
        // ==================== RECEIPT (A5) ====================
        "receipt" => html! {
            <div class="print-document print-a5">
                // Header
                <div style="text-align: center; border-bottom: 2px solid black; padding-bottom: 0.5rem; margin-bottom: 1rem;">
                    <h1 style="margin: 0; font-size: 1.3rem;">{ &settings.clinic_name }</h1>
                    <p style="margin: 0.25rem 0 0; font-size: 0.85rem;">{ &settings.clinic_address }</p>
                    <p style="margin: 0; font-size: 0.8rem;">
                        { format!("โทร: {} • Tax ID: {}", settings.clinic_phone, settings.clinic_tax_id) }
                    </p>
                </div>
                
                // Title with multilingual
                <div style="text-align: center; margin-bottom: 1rem;">
                    <h2 style="margin: 0; font-size: 1.2rem; border: 2px solid black; display: inline-block; padding: 0.4rem 1.5rem;">
                        { "ใบเสร็จรับเงิน" }
                    </h2>
                    <p style="margin: 0.25rem 0 0; font-size: 0.85rem; color: #444;">
                        { "RECEIPT / လက်ခံပြေစာ" }
                    </p>
                </div>
                
                // Receipt Number & Date
                <div style="display: flex; justify-content: space-between; margin-bottom: 1rem; padding: 0.5rem; background: #f5f5f5; border-radius: 4px; font-size: 0.9rem;">
                    <div>
                        <p style="margin: 0;"><strong>{ "เลขที่/No.:" }</strong></p>
                        <p style="margin: 0; font-size: 1rem; font-weight: bold; font-family: monospace;">{ &receipt_no }</p>
                    </div>
                    <div style="text-align: right;">
                        <p style="margin: 0;"><strong>{ "วันที่/Date:" }</strong></p>
                        <p style="margin: 0; font-size: 0.9rem;">{ &date_str }</p>
                    </div>
                </div>
                
                // Patient Info - Multilingual
                <div style="margin-bottom: 1rem; border: 1px solid #ddd; padding: 0.5rem; border-radius: 4px; font-size: 0.9rem;">
                    <p style="margin: 0;">
                        <strong>{ "ผู้รับบริการ/Patient/လူနာ:" }</strong>
                    </p>
                    <p style="margin: 0.15rem 0 0; font-size: 1rem;">{ format!("{}{} {}", p.title, p.first_name, p.last_name) }</p>
                    <p style="margin: 0; font-size: 0.85rem;">{ format!("HN: {}", p.hn) }</p>
                </div>
                
                // Items Table - Compact with Multilingual
                <table class="print-table" style="font-size: 0.85rem;">
                    <thead>
                        <tr>
                            <th style="width: 35px; text-align: center; padding: 0.4rem;">{ "#" }</th>
                            <th style="padding: 0.4rem;">{ "รายการ/Item/ပစ္စည်း" }</th>
                            <th style="text-align: right; padding: 0.4rem; width: 70px;">{ "บาท/THB" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        // 1. Nursing Service Fee
                        <tr>
                            <td style="text-align: center; padding: 0.4rem;">{ "1" }</td>
                            <td style="padding: 0.4rem;">
                                <div>{ "ค่าบริการทางการพยาบาล" }</div>
                                <div style="font-size: 0.75rem; color: #666;">{ "Nursing Service Fee / သူနာပြုဝန်ဆောင်မှုကြေး" }</div>
                            </td>
                            <td style="text-align: right; padding: 0.4rem;">{ "50.00" }</td>
                        </tr>
                        
                        // 2. Drug Cost
                        { if drug_cost > 0.0 {
                            html! {
                                <tr>
                                    <td style="text-align: center; padding: 0.4rem;">{ "2" }</td>
                                    <td style="padding: 0.4rem;">
                                        <div>{ "ค่ายาและเวชภัณฑ์" }</div>
                                        <div style="font-size: 0.75rem; color: #666;">{ "Medicines / ဆေးဝါးများ" }</div>
                                        <ul style="margin: 0.15rem 0 0; padding-left: 1rem; font-size: 0.75rem; color: #555;">
                                            { for r.prescriptions.iter().map(|rx| html! {
                                                <li>{ format!("{} ({})", rx.name, rx.amount) }</li>
                                            })}
                                        </ul>
                                    </td>
                                    <td style="text-align: right; padding: 0.4rem;">{ format!("{:.2}", drug_cost) }</td>
                                </tr>
                            }
                        } else { html! {} }}
                    </tbody>
                    <tfoot>
                        <tr style="background: #f0f0f0;">
                            <td colspan="2" style="text-align: right; font-weight: bold; padding: 0.5rem;">
                                { "รวม/Total/စုစုပေါင်း" }
                            </td>
                            <td style="text-align: right; font-weight: bold; font-size: 1.1rem; padding: 0.5rem;">
                                { format!("{:.2}", r.price) }
                            </td>
                        </tr>
                    </tfoot>
                </table>
                
                // Amount in Thai words
                <div style="margin: 0.5rem 0; padding: 0.4rem; border: 1px solid #ddd; border-radius: 4px; background: #fafafa; font-size: 0.85rem;">
                    <strong>{ "จำนวนเงินตัวอักษร: " }</strong>
                    { format_thai_baht(r.price) }
                </div>
                
                // Signatures - Compact
                <div style="display: flex; justify-content: space-between; margin-top: 2rem; font-size: 0.85rem;">
                    <div style="text-align: center; width: 45%;">
                        <div style="border-bottom: 1px solid black; height: 30px; margin-bottom: 4px;"></div>
                        <p style="margin: 0;">{ "ลงชื่อผู้รับบริการ" }</p>
                        <p style="margin: 0; font-size: 0.75rem; color: #666;">{ "Patient Signature" }</p>
                    </div>
                    <div style="text-align: center; width: 45%;">
                        <div style="border-bottom: 1px solid black; height: 30px; margin-bottom: 4px;"></div>
                        <p style="margin: 0;">{ "ลงชื่อผู้รับเงิน" }</p>
                        <p style="margin: 0; font-size: 0.75rem; color: #666;">{ "Cashier Signature" }</p>
                    </div>
                </div>
            </div>
        },
        
        // ==================== PRESCRIPTION (A5) ====================
        "prescription" => html! {
            <div class="print-document print-a5">
                // Header
                <div style="display: flex; justify-content: space-between; align-items: flex-end; border-bottom: 2px solid black; padding-bottom: 0.5rem; margin-bottom: 1rem;">
                    <div>
                        <h1 style="margin: 0; font-size: 1.2rem; color: #2563eb;">{ &settings.clinic_name }</h1>
                        <p style="margin: 0.15rem 0 0; font-size: 0.8rem;">{ &settings.clinic_address }</p>
                        <p style="margin: 0; font-size: 0.8rem;">{ format!("โทร: {}", settings.clinic_phone) }</p>
                    </div>
                    <div style="text-align: right;">
                        <div style="font-size: 1.5rem; font-weight: bold;">{ "ใบสั่งยา" }</div>
                        <div style="font-size: 0.85rem; color: #666;">{ "PRESCRIPTION / ဆေးညွှန်း" }</div>
                    </div>
                </div>
                
                // Patient Info - Multilingual
                <div style="display: flex; justify-content: space-between; margin-bottom: 1rem; padding: 0.5rem; border: 1px solid #ddd; border-radius: 4px; font-size: 0.85rem;">
                    <div>
                        <p style="margin: 0;"><strong>{ "ผู้ป่วย/Patient/လူနာ:" }</strong> { format!("{}{} {}", p.title, p.first_name, p.last_name) }</p>
                        <p style="margin: 0.15rem 0 0;"><strong>{ "HN:" }</strong> { &p.hn }</p>
                        { if !p.drug_allergy.is_empty() && p.drug_allergy != "ไม่มี" {
                            html! { <p style="margin: 0.15rem 0 0; color: #dc2626;"><strong>{ "⚠️ แพ้ยา/Allergy:" }</strong> { &p.drug_allergy }</p> }
                        } else { html! {} }}
                    </div>
                    <div style="text-align: right;">
                        <p style="margin: 0;"><strong>{ "วันที่/Date:" }</strong></p>
                        <p style="margin: 0;">{ &date_str }</p>
                    </div>
                </div>
                
                // Prescription List - Compact Multilingual
                <div style="margin-bottom: 1.5rem;">
                    <h3 style="margin: 0 0 0.5rem; border-bottom: 1px solid #ddd; padding-bottom: 0.25rem; font-size: 1rem;">
                        { "💊 รายการยา / Medicines / ဆေးဝါးများ" }
                    </h3>
                    <table style="width: 100%; border-collapse: collapse; font-size: 0.8rem;">
                        <thead>
                            <tr style="background: #f5f5f5;">
                                <th style="padding: 0.4rem; text-align: left; border: 1px solid #ddd; width: 30px;">{ "#" }</th>
                                <th style="padding: 0.4rem; text-align: left; border: 1px solid #ddd;">{ "ชื่อยา/Medicine" }</th>
                                <th style="padding: 0.4rem; text-align: left; border: 1px solid #ddd;">{ "วิธีใช้/Dosage/သောက်ပုံ" }</th>
                                <th style="padding: 0.4rem; text-align: center; border: 1px solid #ddd; width: 50px;">{ "จำนวน" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for r.prescriptions.iter().enumerate().map(|(i, rx)| {
                                let dosage_th = if rx.morning > 0.0 || rx.noon > 0.0 || rx.evening > 0.0 || rx.before_bed > 0.0 {
                                    format!("เช้า {} กลางวัน {} เย็น {} ก่อนนอน {}", 
                                        rx.morning, rx.noon, rx.evening, rx.before_bed)
                                } else {
                                    rx.usage.clone()
                                };
                                let dosage_en = if rx.morning > 0.0 || rx.noon > 0.0 || rx.evening > 0.0 || rx.before_bed > 0.0 {
                                    format!("M:{} N:{} E:{} B:{}", rx.morning, rx.noon, rx.evening, rx.before_bed)
                                } else {
                                    "As directed".to_string()
                                };
                                let timing = if rx.timing.is_empty() { "หลังอาหาร / After meal" } else { &rx.timing };
                                html! {
                                    <tr>
                                        <td style="padding: 0.4rem; border: 1px solid #ddd; text-align: center;">{ i + 1 }</td>
                                        <td style="padding: 0.4rem; border: 1px solid #ddd;">
                                            <div style="font-weight: bold;">{ &rx.name }</div>
                                            { if !rx.warning.is_empty() {
                                                html! { <div style="color: #dc2626; font-size: 0.75rem;">{ format!("⚠️ {}", rx.warning) }</div> }
                                            } else { html! {} }}
                                        </td>
                                        <td style="padding: 0.4rem; border: 1px solid #ddd;">
                                            <div>{ &dosage_th }</div>
                                            <div style="font-size: 0.7rem; color: #666;">{ dosage_en }</div>
                                            <div style="font-size: 0.7rem; color: #888;">{ timing }</div>
                                        </td>
                                        <td style="padding: 0.4rem; border: 1px solid #ddd; text-align: center; font-weight: bold;">{ &rx.amount }</td>
                                    </tr>
                                }
                            })}
                        </tbody>
                    </table>
                </div>
                
                // Signature - Compact
                <div style="display: flex; justify-content: flex-end; margin-top: 1.5rem;">
                    <div style="text-align: center; width: 180px; font-size: 0.85rem;">
                        <div style="border-bottom: 1px solid black; height: 35px; margin-bottom: 4px;"></div>
                        <p style="margin: 0;"><strong>{ "ลงชื่อแพทย์ผู้สั่งยา" }</strong></p>
                        <p style="margin: 0; font-size: 0.75rem; color: #666;">{ "Prescriber / ဆရာဝန် လက်မှတ်" }</p>
                        <p style="margin: 0.15rem 0 0; font-size: 0.75rem; color: #666;">{ "License No. ______________" }</p>
                    </div>
                </div>
            </div>
        },
        
        // ==================== MEDICAL CERTIFICATE (A4) ====================
        "cert" => {
            // Medical Certificate - ใบรับรองการรักษาพยาบาล
            let staff_display = if settings.staff_name.is_empty() {
                "นางสมหญิง วีระจินตนา".to_string()
            } else {
                settings.staff_name.clone()
            };
            let license_display = if settings.license_number.is_empty() {
                "4511055362".to_string()
            } else {
                settings.license_number.clone()
            };
            let position_display = if settings.staff_position.is_empty() {
                "พยาบาลวิชาชีพชำนาญการ".to_string()
            } else {
                settings.staff_position.clone()
            };
            
            html! {
                <div class="print-document print-cert">
                    // Header Title
                    <div style="text-align: center; margin-bottom: 1.5rem;">
                        <h1 style="margin: 0; font-size: 1.8rem; font-weight: bold;">{ "ใบรับรองการรักษาพยาบาล" }</h1>
                    </div>
                    
                    // Clinic Name & Address
                    <div style="margin-bottom: 2rem; font-size: 1rem;">
                        <div style="margin-bottom: 0.3rem;">
                            { "ชื่อสถานพยาบาล " }
                            <span style="margin-left: 0.5rem;">{ &settings.clinic_name }</span>
                        </div>
                        <div>
                            { "ตั้งอยู่เลขที่ " }
                            <span style="margin-left: 0.5rem;">{ &settings.clinic_address }</span>
                        </div>
                    </div>
                    
                    // Staff Info Line
                    <div style="margin-bottom: 1.5rem; font-size: 1rem;">
                        <div style="margin-bottom: 0.5rem;">
                            { "ข้าพเจ้า " }
                            <span style="border-bottom: 1px dotted #000; padding: 0 10px;">{ &staff_display }</span>
                            { " ใบอนุญาตประกอบวิชาชีพเลขที่ " }
                            <span style="border-bottom: 1px dotted #000; padding: 0 10px;">{ &license_display }</span>
                        </div>
                    </div>
                    
                    // Main Content
                    <div style="margin-bottom: 1.5rem; font-size: 1rem; line-height: 2;">
                        <div style="margin-bottom: 0.5rem;">
                            { "ได้ทำการรักษาพยาบาลให้แก่ " }
                            <span style="border-bottom: 1px dotted #000; padding: 0 10px; font-weight: bold;">
                                { format!("{}{} {}", p.title, p.first_name, p.last_name) }
                            </span>
                            { " HN: " }
                            <span style="border-bottom: 1px dotted #000; padding: 0 10px; font-weight: bold;">
                                { &p.hn }
                            </span>
                        </div>
                        
                        <div style="margin-bottom: 0.5rem;">
                            { "เมื่อวันที่ " }
                            <span style="border-bottom: 1px dotted #000; padding: 0 30px; min-width: 150px; display: inline-block; text-align: center;">
                                { &date_only }
                            </span>
                        </div>
                        
                        <div style="margin-bottom: 0.5rem;">
                            { "ด้วยอาการที่มาพบ " }
                            <span style="border-bottom: 1px dotted #000; padding: 0 5px; display: inline-block; min-width: 400px;">
                                { &r.symptoms }
                            </span>
                        </div>
                        
                        // Diagnosis line (blank if none)
                        <div style="border-bottom: 1px dotted #000; min-height: 1.5rem; margin-bottom: 0.75rem;">
                            { &r.diagnosis }
                        </div>
                    </div>
                    
                    // Rest Period Section
                    <div style="margin-bottom: 1.5rem; font-size: 1rem; line-height: 2;">
                        <div style="margin-bottom: 0.5rem;">
                            { "เห็นสมควรให้พักตั้งแต่" }
                            <span style="border-bottom: 1px dotted #000; padding: 0 10px; min-width: 100px; display: inline-block;"></span>
                            { " ถึง " }
                            <span style="border-bottom: 1px dotted #000; padding: 0 10px; min-width: 100px; display: inline-block;"></span>
                            { " เป็นเวลา " }
                            <span style="border-bottom: 1px dotted #000; padding: 0 10px; min-width: 30px; display: inline-block;"></span>
                            { " วัน" }
                        </div>
                        
                        <div>
                            { "ข้าพเจ้าขอรับรองว่า" }
                            <span style="border-bottom: 1px dotted #000; padding: 0 10px; min-width: 200px; display: inline-block;"></span>
                            { "มารับการรักษากับข้าพเจ้าตามข้อความ" }
                        </div>
                        <div>{ "ข้างต้นจริง" }</div>
                    </div>
                    
                    // Signature Section - Right Aligned
                    <div style="display: flex; flex-direction: column; align-items: flex-end; margin-top: 4rem; margin-right: 3rem;">
                        <div style="text-align: center; width: 280px;">
                            <div style="height: 2rem;"></div>
                            <div style="margin-bottom: 0.3rem;">
                                { format!("( {} )", staff_display) }
                            </div>
                            <div style="margin-bottom: 0.3rem;">
                                { format!("ตำแหน่ง {}", position_display) }
                            </div>
                            <div>
                                { "วันที่ " }
                                <span style="border-bottom: 1px dotted #000; padding: 0 20px; min-width: 100px; display: inline-block;">
                                    { &date_only }
                                </span>
                            </div>
                        </div>
                    </div>
                    
                    // Footer Notes
                    <div style="margin-top: 5rem; font-size: 0.95rem;">
                        <div style="text-decoration: underline; margin-bottom: 0.3rem;">{ "หมายเหตุ" }</div>
                        <div style="padding-left: 1rem;">{ "1. ให้ประทับตราสถานพยาบาล (ถ้ามี)" }</div>
                        <div style="padding-left: 1rem;">{ "2. กรณีสมควรให้พักต้องไม่เกิน 2 วัน ทั้งนี้รวมวันที่มารับการตรวจด้วย" }</div>
                    </div>
                </div>
            }
        },
        _ => html! { <div class="print-document"><p>{ "ไม่พบประเภทเอกสาร" }</p></div> }
    };
    
    html! {
        <>
            <div class="no-print" style="position: fixed; top: 20px; right: 20px; z-index: 1000; display: flex; gap: 10px; background: rgba(255,255,255,0.9); padding: 10px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1);">
                <button class="btn btn-secondary" onclick={move |_| navigator.back()}>
                    { "← ปิด/ย้อนกลับ" }
                </button>
                <button class="btn btn-primary" onclick={|_| { let _ = web_sys::window().unwrap().print(); }}>
                    { "🖨️ พิมพ์อีกครั้ง" }
                </button>
            </div>
            { content }
        </>
    }
}

fn format_thai_baht(amount: f64) -> String {
    let baht = amount.floor() as i64;
    let satang = ((amount - baht as f64) * 100.0).round() as i64;
    
    let baht_text = number_to_thai(baht);
    
    if satang > 0 {
        format!("{}บาท{}สตางค์ถ้วน", baht_text, number_to_thai(satang))
    } else {
        format!("{}บาทถ้วน", baht_text)
    }
}

fn number_to_thai(n: i64) -> String {
    if n == 0 { return "ศูนย์".to_string(); }
    
    let digits = ["", "หนึ่ง", "สอง", "สาม", "สี่", "ห้า", "หก", "เจ็ด", "แปด", "เก้า"];
    let positions = ["", "สิบ", "ร้อย", "พัน", "หมื่น", "แสน", "ล้าน"];
    
    let mut result = String::new();
    let s = n.to_string();
    let len = s.len();
    
    for (i, c) in s.chars().enumerate() {
        let d = c.to_digit(10).unwrap() as usize;
        let pos = len - i - 1;
        
        if d == 0 { continue; }
        
        // Special cases
        if d == 1 && pos == 0 && len > 1 {
            result.push_str("เอ็ด");
        } else if d == 2 && pos == 1 {
            result.push_str("ยี่สิบ");
        } else if d == 1 && pos == 1 {
            result.push_str("สิบ");
        } else {
            result.push_str(digits[d]);
            if pos < positions.len() {
                result.push_str(positions[pos]);
            }
        }
    }
    
    result
}
