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
        "receipt" => html! {
            <div class="print-document">
                // Header
                <div style="text-align: center; border-bottom: 3px solid black; padding-bottom: 1rem; margin-bottom: 1.5rem;">
                    <h1 style="margin: 0; font-size: 1.75rem;">{ &settings.clinic_name }</h1>
                    <p style="margin: 0.5rem 0 0; font-size: 1rem;">{ &settings.clinic_address }</p>
                    <p style="margin: 0; font-size: 0.95rem;">
                        { format!("โทร: {} • เลขประจำตัวผู้เสียภาษี: {}", settings.clinic_phone, settings.clinic_tax_id) }
                    </p>
                </div>
                
                // Title with Receipt Number
                <div style="text-align: center; margin-bottom: 1.5rem;">
                    <h2 style="margin: 0; font-size: 1.5rem; border: 3px solid black; display: inline-block; padding: 0.75rem 2rem;">
                        { "ใบเสร็จรับเงิน / RECEIPT" }
                    </h2>
                </div>
                
                // Receipt Number & Date
                <div style="display: flex; justify-content: space-between; margin-bottom: 1.5rem; padding: 1rem; background: #f5f5f5; border-radius: 4px;">
                    <div>
                        <p style="margin: 0; font-size: 1.1rem;"><strong>{ "เลขที่ใบเสร็จ:" }</strong></p>
                        <p style="margin: 0; font-size: 1.5rem; font-weight: bold; font-family: monospace;">{ &receipt_no }</p>
                    </div>
                    <div style="text-align: right;">
                        <p style="margin: 0;"><strong>{ "วันที่:" }</strong></p>
                        <p style="margin: 0; font-size: 1.1rem;">{ &date_str }</p>
                    </div>
                </div>
                
                // Patient Info
                <div style="display: flex; justify-content: space-between; margin-bottom: 1.5rem; border: 1px solid #ddd; padding: 1rem; border-radius: 4px;">
                    <div>
                        <p style="margin: 0;"><strong>{ "ผู้รับบริการ:" }</strong></p>
                        <p style="margin: 0.25rem 0 0; font-size: 1.2rem;">{ format!("{}{} {}", p.title, p.first_name, p.last_name) }</p>
                        <p style="margin: 0;">{ format!("HN: {}", p.hn) }</p>
                    </div>
                </div>
                
                // Items Table - Itemized
                <table class="print-table">
                    <thead>
                        <tr>
                            <th style="width: 50px; text-align: center;">{ "ลำดับ" }</th>
                            <th style="width: 60%;">{ "รายการ" }</th>
                            <th style="text-align: right;">{ "จำนวนเงิน (บาท)" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        // 1. ค่าบริการทางการพยาบาล (Fixed 50)
                        <tr>
                            <td style="text-align: center;">{ "1" }</td>
                            <td>
                                <div style="font-weight: bold;">{ "ค่าบริการทางการพยาบาล" }</div>
                            </td>
                            <td style="text-align: right; font-size: 1.1rem;">{ "50.00" }</td>
                        </tr>
                        
                        // 2. ค่ายา (Remaining)
                        { if drug_cost > 0.0 {
                            html! {
                                <tr>
                                    <td style="text-align: center;">{ "2" }</td>
                                    <td>
                                        <div style="font-weight: bold;">{ "ค่ายาและเวชภัณฑ์" }</div>
                                        <ul style="margin: 0.25rem 0 0; padding-left: 1.5rem; font-size: 0.85rem; color: #444;">
                                            { for r.prescriptions.iter().map(|rx| html! {
                                                <li>{ format!("{} ({})", rx.name, rx.amount) }</li>
                                            })}
                                        </ul>
                                    </td>
                                    <td style="text-align: right; font-size: 1.1rem;">{ format!("{:.2}", drug_cost) }</td>
                                </tr>
                            }
                        } else { html! {} }}
                    </tbody>
                    <tfoot>
                        <tr style="background: #f0f0f0;">
                            <td colspan="2" style="text-align: right; font-weight: bold; font-size: 1.2rem;">{ "รวมทั้งสิ้น" }</td>
                            <td style="text-align: right; font-weight: bold; font-size: 1.5rem;">{ format!("{:.2}", r.price) }</td>
                        </tr>
                    </tfoot>
                </table>
                
                // Amount in Thai words
                <div style="margin: 1rem 0; padding: 0.75rem; border: 1px solid #ddd; border-radius: 4px; background: #fafafa;">
                    <strong>{ "จำนวนเงินตัวอักษร: " }</strong>
                    { format_thai_baht(r.price) }
                </div>
                
                // Signatures
                <div style="display: flex; justify-content: space-between; margin-top: 4rem;">
                    <div style="text-align: center; width: 200px;">
                        <div style="border-bottom: 1px solid black; height: 50px; margin-bottom: 8px;"></div>
                        <p style="margin: 0;">{ "ลงชื่อผู้รับบริการ" }</p>
                        <p style="margin: 0; font-size: 0.9rem; color: #666;">{ "(.......................................)​" }</p>
                    </div>
                    <div style="text-align: center; width: 200px;">
                        <div style="border-bottom: 1px solid black; height: 50px; margin-bottom: 8px;"></div>
                        <p style="margin: 0;">{ "ลงชื่อผู้รับเงิน" }</p>
                        <p style="margin: 0; font-size: 0.9rem; color: #666;">{ "(.......................................)​" }</p>
                    </div>
                </div>
            </div>
        },
        "prescription" => html! {
            <div class="print-document">
                // Header
                <div style="display: flex; justify-content: space-between; align-items: flex-end; border-bottom: 3px solid black; padding-bottom: 1rem; margin-bottom: 1.5rem;">
                    <div>
                        <h1 style="margin: 0; font-size: 1.5rem; color: #2563eb;">{ &settings.clinic_name }</h1>
                        <p style="margin: 0.25rem 0 0; font-size: 0.9rem;">{ &settings.clinic_address }</p>
                        <p style="margin: 0; font-size: 0.9rem;">{ format!("โทร: {}", settings.clinic_phone) }</p>
                    </div>
                    <div style="font-size: 2.5rem; font-weight: bold;">{ "ใบสั่งยา" }</div>
                </div>
                
                // Patient Info
                <div style="display: flex; justify-content: space-between; margin-bottom: 1.5rem; padding: 1rem; border: 1px solid #ddd; border-radius: 4px;">
                    <div>
                        <p style="margin: 0;"><strong>{ "ผู้ป่วย:" }</strong> { format!("{}{} {}", p.title, p.first_name, p.last_name) }</p>
                        <p style="margin: 0.25rem 0 0;"><strong>{ "HN:" }</strong> { &p.hn }</p>
                        { if !p.drug_allergy.is_empty() && p.drug_allergy != "ไม่มี" {
                            html! { <p style="margin: 0.25rem 0 0; color: #dc2626;"><strong>{ "⚠️ แพ้ยา:" }</strong> { &p.drug_allergy }</p> }
                        } else { html! {} }}
                    </div>
                    <div style="text-align: right;">
                        <p style="margin: 0;"><strong>{ "วันที่:" }</strong> { &date_str }</p>
                    </div>
                </div>
                
                // Prescription List
                <div style="margin-bottom: 2rem;">
                    <h3 style="margin: 0 0 1rem; border-bottom: 2px solid #ddd; padding-bottom: 0.5rem;">{ "💊 รายการยา" }</h3>
                    <table style="width: 100%; border-collapse: collapse;">
                        <thead>
                            <tr style="background: #f5f5f5;">
                                <th style="padding: 0.75rem; text-align: left; border: 1px solid #ddd; width: 40px;">{ "ลำดับ" }</th>
                                <th style="padding: 0.75rem; text-align: left; border: 1px solid #ddd;">{ "ชื่อยา" }</th>
                                <th style="padding: 0.75rem; text-align: left; border: 1px solid #ddd;">{ "วิธีใช้" }</th>
                                <th style="padding: 0.75rem; text-align: center; border: 1px solid #ddd; width: 80px;">{ "จำนวน" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for r.prescriptions.iter().enumerate().map(|(i, rx)| {
                                let dosage = if rx.morning > 0 || rx.noon > 0 || rx.evening > 0 || rx.before_bed > 0 {
                                    format!("เช้า {} กลางวัน {} เย็น {} ก่อนนอน {} ({})", 
                                        rx.morning, rx.noon, rx.evening, rx.before_bed,
                                        if rx.timing.is_empty() { "หลังอาหาร" } else { &rx.timing })
                                } else {
                                    if rx.timing.is_empty() { rx.usage.clone() } 
                                    else { format!("{} ({})", rx.usage, rx.timing) }
                                };
                                html! {
                                    <tr>
                                        <td style="padding: 0.75rem; border: 1px solid #ddd; text-align: center;">{ i + 1 }</td>
                                        <td style="padding: 0.75rem; border: 1px solid #ddd;">
                                            <div style="font-weight: bold;">{ &rx.name }</div>
                                            { if !rx.warning.is_empty() {
                                                html! { <div style="color: #dc2626; font-size: 0.9rem;">{ format!("⚠️ {}", rx.warning) }</div> }
                                            } else { html! {} }}
                                        </td>
                                        <td style="padding: 0.75rem; border: 1px solid #ddd;">{ dosage }</td>
                                        <td style="padding: 0.75rem; border: 1px solid #ddd; text-align: center; font-weight: bold;">{ &rx.amount }</td>
                                    </tr>
                                }
                            })}
                        </tbody>
                    </table>
                </div>
                
                // Signature
                <div style="display: flex; justify-content: flex-end; margin-top: 3rem;">
                    <div style="text-align: center; width: 250px;">
                        <div style="border-bottom: 1px solid black; height: 60px; margin-bottom: 8px;"></div>
                        <p style="margin: 0;"><strong>{ "ลงชื่อแพทย์ผู้สั่งยา" }</strong></p>
                        <p style="margin: 0.25rem 0 0; font-size: 0.9rem; color: #666;">{ "(.......................................)​" }</p>
                        <p style="margin: 0.25rem 0 0; font-size: 0.9rem; color: #666;">{ "ใบอนุญาตเลขที่ ______________" }</p>
                    </div>
                </div>
            </div>
        },
        "cert" => {
            // Medical Certificate - ใบรับรองการรักษาพยาบาล
            // Use staff info from settings
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
                <div class="print-document" style="font-family: 'Sarabun', 'TH Sarabun New', sans-serif;">
                    // Header Title
                    <div style="text-align: center; margin-bottom: 1rem;">
                        <h1 style="margin: 0; font-size: 1.8rem; font-weight: bold;">{ "ใบรับรองการรักษาพยาบาล" }</h1>
                    </div>
                    
                    // Clinic Name & Address
                    <div style="text-align: center; margin-bottom: 2rem; font-size: 1.1rem;">
                        <div style="margin-bottom: 0.2rem;">{ format!("ชื่อสถานพยาบาล {}", settings.clinic_name) }</div>
                        <div>{ format!("ตั้งอยู่เลขที่ {}", settings.clinic_address) }</div>
                    </div>
                    
                    // Content
                    <div style="margin-bottom: 1.5rem; line-height: 2; font-size: 1.2rem;">
                        <div>
                            { "ข้าพเจ้า " }
                            <span style="display: inline-block; min-width: 200px; text-align: center;">{ &staff_display }</span>
                        </div>
                        <div>
                          <p style="margin: 0;">
                            { "ได้ทำการพยาบาลและหรือการผดุงครรภ์ " }
                            <span style="border-bottom: 1px dotted #000; padding: 0 5px; font-weight: bold;">
                                { format!(" ให้แก่ {}{} {}", p.title, p.first_name, p.last_name) }
                            </span>
                            { " HN: " }
                            <span style="border-bottom: 1px dotted #000; padding: 0 5px; font-weight: bold;">
                                { &p.hn }
                            </span>
                        </p>
                        </div>
                        <div>
                            { "เมื่อวันที่ " }
                            <span style="display: inline-block; min-width: 200px; text-align: center; border-bottom: 1px dotted #000;">{ &date_only }</span>
                        </div>
                        <div style="display: flex; align-items: baseline;">
                            <span style="white-space: nowrap;">{ "ด้วยอาการที่มาพบ " }</span>
                            <span style="flex-grow: 1; border-bottom: 1px dotted #000; text-align: left; padding-left: 10px;">{ &r.symptoms }</span>
                        </div>
                        <div style="border-bottom: 1px dotted #000; min-height: 1.5rem;">{ &r.diagnosis }</div>
                        
                        <div style="margin-top: 1rem;">
                            { "ใบอนุญาตประกอบวิชาชีพเลขที่ " }
                            <span>{ &license_display }</span>
                        </div>
                        
                        // Resting period (Manual Fill)
                        <div style="margin-top: 1rem;">
                            { "เห็นสมควรให้พักตั้งแต่" }
                            <span style="display: inline-block; min-width: 150px; border-bottom: 1px dotted #000; margin: 0 5px;"></span>
                            { "ถึง" }
                            <span style="display: inline-block; min-width: 150px; border-bottom: 1px dotted #000; margin: 0 5px;"></span>
                        </div>
                        <div>
                            { "เป็นเวลา" }
                            <span style="display: inline-block; min-width: 50px; border-bottom: 1px dotted #000; margin: 0 5px; text-align: center;"></span>
                            { "วัน" }
                        </div>
                        
                        // Certification
                        <div style="margin-top: 1rem;">
                            { "ข้าพเจ้าขอรับรองว่า " }
                            <span style="display: inline-block; min-width: 200px; text-align: center; border-bottom: 1px dotted #000;">
                                { format!("{}{} {}", p.title, p.first_name, p.last_name) }
                            </span>
                        </div>
                        <div>
                            { "มารับการรักษากับข้าพเจ้าตามข้อความข้างต้นจริง" }
                        </div>
                    </div>
                    
                    // Signatures
                    <div style="display: flex; flex-direction: column; align-items: flex-end; margin-top: 3rem; margin-right: 2rem;">
                        <div style="text-align: center; width: 300px;">
                            <div style="margin-bottom: 0.5rem;">
                                { format!("( {} )", staff_display) }
                            </div>
                            <div style="margin-bottom: 0.5rem;">
                                { format!("ตำแหน่ง {}", position_display) }
                            </div>
                            <div>
                                { "วันที่ " }
                                <span style="display: inline-block; min-width: 120px; border-bottom: 1px dotted #000;">{ &date_only }</span>
                            </div>
                        </div>
                    </div>
                    
                    // Footer Notes
                    <div style="margin-top: 4rem; font-size: 1rem;">
                        <div>{ "หมายเหตุ 1. ให้ประทับตราสถานพยาบาล (ถ้ามี )" }</div>
                        <div>{ "2. กรณีสมควรให้พักต้องไม่เกิน 2 วัน ทั้งนี้รวมวันที่มารับการตรวจด้วย" }</div>
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
