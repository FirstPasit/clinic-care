use yew::prelude::*;
use crate::store::Store;
use chrono::prelude::*;
use gloo::timers::callback::Timeout;

#[derive(Clone, PartialEq)]
pub enum Language {
    Thai,
    English,
    Myanmar,
}

// Translation struct for sticker labels
struct StickerLabels {
    drug_name: &'static str,
    date: &'static str,
    indication: &'static str,
    dosage: &'static str,
    tablets: &'static str,
    times_per_day: &'static str,
    times: &'static str,           // ครั้ง / times / ကြိမ်
    hours: &'static str,           // ชม. / hrs. / နာရီ
    before_meal_30: &'static str,
    after_meal: &'static str,
    every_hours: &'static str,
    morning: &'static str,
    noon: &'static str,
    evening: &'static str,
    before_bed: &'static str,
    take_after_meal_immediately: &'static str,
    take_when_symptoms: &'static str,
    no_milk_antacid: &'static str,
    continue_until_finish: &'static str,
    may_cause_drowsy: &'static str,
    shake_before_use: &'static str,
}

fn get_labels(lang: &Language) -> StickerLabels {
    match lang {
        Language::Thai => StickerLabels {
            drug_name: "ชื่อยา",
            date: "วันที่",
            indication: "ข้อบ่งใช้",
            dosage: "กินครั้งละ",
            tablets: "เม็ด/ช้อนชา/มล.",
            times_per_day: "วันละ",
            times: "ครั้ง",
            hours: "ชม.",
            before_meal_30: "ก่อนอาหาร 30 นาที",
            after_meal: "หลังอาหาร",
            every_hours: "ทุก",
            morning: "เช้า",
            noon: "กลางวัน",
            evening: "เย็น",
            before_bed: "ก่อนนอน",
            take_after_meal_immediately: "กินยาหลังอาหารทันที",
            take_when_symptoms: "กินเฉพาะเวลามีอาการ",
            no_milk_antacid: "ห้ามกินพร้อมนม/ยาลดกรด/แคลเซียม",
            continue_until_finish: "กินยาต่อเนื่องจนหมด",
            may_cause_drowsy: "กินยานี้แล้วอาจง่วงซึม",
            shake_before_use: "เขย่าขวดก่อนกินยา",
        },
        Language::English => StickerLabels {
            drug_name: "Drug Name",
            date: "Date",
            indication: "Indication",
            dosage: "Take",
            tablets: "tablet(s)/tsp/ml",
            times_per_day: "times/day",
            times: "times",
            hours: "hrs.",
            before_meal_30: "30 min before meal",
            after_meal: "After meal",
            every_hours: "Every",
            morning: "Morning",
            noon: "Noon",
            evening: "Evening",
            before_bed: "Bedtime",
            take_after_meal_immediately: "Take immediately after meal",
            take_when_symptoms: "Take only when symptoms occur",
            no_milk_antacid: "Do not take with milk/antacid/calcium",
            continue_until_finish: "Continue until finished",
            may_cause_drowsy: "May cause drowsiness",
            shake_before_use: "Shake well before use",
        },
        Language::Myanmar => StickerLabels {
            drug_name: "ဆေးအမည်",
            date: "ရက်စွဲ",
            indication: "သုံးစွဲရန်",
            dosage: "တစ်ကြိမ်",
            tablets: "လုံး/ဇွန်း/မီလီ",
            times_per_day: "ကြိမ်/နေ့",
            times: "ကြိမ်",
            hours: "နာရီ",
            before_meal_30: "ထမင်းမစားခင် ၃၀ မိနစ်",
            after_meal: "ထမင်းစားပြီး",
            every_hours: "နာရီတိုင်း",
            morning: "မနက်",
            noon: "နေ့လည်",
            evening: "ညနေ",
            before_bed: "အိပ်ခါနီး",
            take_after_meal_immediately: "ထမင်းစားပြီးချက်ချင်းသောက်ပါ",
            take_when_symptoms: "လက္ခဏာရှိမှသောက်ပါ",
            no_milk_antacid: "နို့/အက်ဆစ်ဆေးနဲ့မသောက်ပါနဲ့",
            continue_until_finish: "ဆေးကုန်သည်အထိသောက်ပါ",
            may_cause_drowsy: "ငိုက်မျဉ်းစေနိုင်သည်",
            shake_before_use: "မသောက်ခင်လှုပ်ပါ",
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub record_id: String,
    pub drug_index: usize,
}

#[function_component(Sticker)]
pub fn sticker(props: &Props) -> Html {
    let size = use_state(|| "large".to_string());
    let language = use_state(|| Language::Thai);
    
    // Auto print
    use_effect_with((), move |_| {
        let timeout = Timeout::new(800, move || {
            let _ = web_sys::window().unwrap().print();
        });
        timeout.forget();
        || ()
    });

    let record = Store::get_records().into_iter().find(|r| r.id == props.record_id);
    
    if record.is_none() {
        return html! { <div class="print-document"><p>{ "ไม่พบข้อมูล" }</p></div> };
    }
    
    let r = record.unwrap();
    let patient = Store::get_patients().into_iter().find(|p| p.id == r.patient_id);
    
    if patient.is_none() {
        return html! { <div class="print-document"><p>{ "ไม่พบข้อมูลผู้ป่วย" }</p></div> };
    }
    
    let _p = patient.unwrap();
    
    if props.drug_index >= r.prescriptions.len() {
        return html! { <div class="print-document"><p>{ "ไม่พบข้อมูลยา" }</p></div> };
    }
    
    let drug = &r.prescriptions[props.drug_index];
    let date_str = r.date.with_timezone(&Local).format("%d/%m/%Y").to_string();
    
    let sticker_class = match (*size).as_str() {
        "small" => "sticker-small",
        "medium" => "sticker-medium",
        _ => "sticker-large",
    };
    
    let labels = get_labels(&language);
    
    // Calculate total doses per day
    let total_doses = drug.morning + drug.noon + drug.evening + drug.before_bed;
    let dose_per_time = if drug.morning > 0 { drug.morning } 
        else if drug.noon > 0 { drug.noon }
        else if drug.evening > 0 { drug.evening }
        else { drug.before_bed };

    html! {
        <div style="padding: 20px;">
            // Controls (hidden on print)
            <div class="no-print" style="margin-bottom: 20px;">
                // Size selector
                <div style="display: flex; gap: 10px; align-items: center; margin-bottom: 15px; flex-wrap: wrap;">
                    <span style="font-size: 1.2rem; font-weight: bold; min-width: 140px;">{ "📐 ขนาด:" }</span>
                    <button class={classes!("btn", if *size == "small" { "btn-primary" } else { "btn-secondary" })}
                        onclick={let size = size.clone(); move |_| size.set("small".to_string())}>
                        { "เล็ก" }
                    </button>
                    <button class={classes!("btn", if *size == "medium" { "btn-primary" } else { "btn-secondary" })}
                        onclick={let size = size.clone(); move |_| size.set("medium".to_string())}>
                        { "กลาง" }
                    </button>
                    <button class={classes!("btn", if *size == "large" { "btn-primary" } else { "btn-secondary" })}
                        onclick={let size = size.clone(); move |_| size.set("large".to_string())}>
                        { "ใหญ่ (แนะนำ)" }
                    </button>
                </div>
                
                // Language selector
                <div style="display: flex; gap: 10px; align-items: center; margin-bottom: 15px; flex-wrap: wrap;">
                    <span style="font-size: 1.2rem; font-weight: bold; min-width: 140px;">{ "🌐 ภาษา:" }</span>
                    <button class={classes!("btn", "btn-lg", if *language == Language::Thai { "btn-primary" } else { "btn-secondary" })}
                        onclick={let language = language.clone(); move |_| language.set(Language::Thai)}>
                        { "🇹🇭 ไทย" }
                    </button>
                    <button class={classes!("btn", "btn-lg", if *language == Language::English { "btn-primary" } else { "btn-secondary" })}
                        onclick={let language = language.clone(); move |_| language.set(Language::English)}>
                        { "🇬🇧 English" }
                    </button>
                    <button class={classes!("btn", "btn-lg", if *language == Language::Myanmar { "btn-primary" } else { "btn-secondary" })}
                        onclick={let language = language.clone(); move |_| language.set(Language::Myanmar)}>
                        { "🇲🇲 မြန်မာ" }
                    </button>
                </div>
                
                // Print button
                <button class="btn btn-success btn-lg" style="width: 100%; max-width: 400px;" onclick={|_| {
                    let _ = web_sys::window().unwrap().print();
                }}>
                    { "🖨️ พิมพ์สติกเกอร์" }
                </button>
            </div>
            
            // Sticker Preview - New Format with Checkboxes
            <div class={classes!("sticker-preview", sticker_class)} style="background: #f5f5dc; border: 2px solid #8b8b00; font-family: 'Sarabun', sans-serif;">
                <div class="sticker-content" style="padding: 2mm;">
                    // Row 1: Drug Name & Date
                    <div style="display: flex; justify-content: space-between; border-bottom: 1px solid #000; padding-bottom: 1mm; margin-bottom: 1mm;">
                        <div>
                            <span style="font-weight: bold;">{ &labels.drug_name }</span>
                            <span style="margin-left: 2mm;">{ &drug.name }</span>
                        </div>
                        <div>
                            <span style="font-weight: bold;">{ &labels.date }</span>
                            <span style="margin-left: 2mm;">{ &date_str }</span>
                        </div>
                    </div>
                    
                    // Row 2: Indication
                    <div style="border-bottom: 1px solid #ccc; padding-bottom: 1mm; margin-bottom: 1mm;">
                        <span style="font-weight: bold;">{ &labels.indication }</span>
                        <span style="margin-left: 2mm;">{ &drug.usage }</span>
                    </div>
                    
                    // Row 3: Dosage
                    <div style="border-bottom: 1px solid #ccc; padding-bottom: 1mm; margin-bottom: 1mm;">
                        <span style="font-weight: bold;">{ &labels.dosage }</span>
                        <span style="margin-left: 2mm; text-decoration: underline;">{ dose_per_time }</span>
                        <span style="margin-left: 1mm;">{ &labels.tablets }</span>
                        <span style="margin-left: 3mm; font-weight: bold;">{ &labels.times_per_day }</span>
                        <span style="margin-left: 2mm; text-decoration: underline;">{ if total_doses > 0 { total_doses.to_string() } else { "...".to_string() } }</span>
                        <span style="margin-left: 1mm;">{ &labels.times }</span>
                    </div>
                    
                    // Row 4: Timing Checkboxes
                    <div style="display: flex; gap: 3mm; margin-bottom: 1mm; flex-wrap: wrap;">
                        <label style="display: flex; align-items: center; gap: 1mm;">
                            <span style={if drug.timing == "ก่อนอาหาร 30 นาที" { "font-weight: bold;" } else { "" }}>
                                { if drug.timing == "ก่อนอาหาร 30 นาที" { "●" } else { "○" } }
                            </span>
                            { &labels.before_meal_30 }
                        </label>
                        <label style="display: flex; align-items: center; gap: 1mm;">
                            <span style={if drug.timing == "หลังอาหาร" || drug.timing.is_empty() { "font-weight: bold;" } else { "" }}>
                                { if drug.timing == "หลังอาหาร" || drug.timing.is_empty() { "●" } else { "○" } }
                            </span>
                            { &labels.after_meal }
                        </label>
                        <label style="display: flex; align-items: center; gap: 1mm;">
                            <span>{ "○" }</span>
                            { &labels.every_hours }
                            <span>{ format!("........{}", labels.hours) }</span>
                        </label>
                    </div>
                    
                    // Row 5: Time of Day Checkboxes
                    <div style="display: flex; gap: 3mm; margin-bottom: 1mm; padding: 1mm; background: #e8e8d0; border-radius: 2px;">
                        <label style="display: flex; align-items: center; gap: 1mm;">
                            <span style={if drug.morning > 0 { "font-weight: bold;" } else { "" }}>
                                { if drug.morning > 0 { "●" } else { "○" } }
                            </span>
                            { &labels.morning }
                        </label>
                        <label style="display: flex; align-items: center; gap: 1mm;">
                            <span style={if drug.noon > 0 { "font-weight: bold;" } else { "" }}>
                                { if drug.noon > 0 { "●" } else { "○" } }
                            </span>
                            { &labels.noon }
                        </label>
                        <label style="display: flex; align-items: center; gap: 1mm;">
                            <span style={if drug.evening > 0 { "font-weight: bold;" } else { "" }}>
                                { if drug.evening > 0 { "●" } else { "○" } }
                            </span>
                            { &labels.evening }
                        </label>
                        <label style="display: flex; align-items: center; gap: 1mm;">
                            <span style={if drug.before_bed > 0 { "font-weight: bold;" } else { "" }}>
                                { if drug.before_bed > 0 { "●" } else { "○" } }
                            </span>
                            { &labels.before_bed }
                        </label>
                    </div>
                    
                    // Row 6: Special Instructions Checkboxes (2 columns)
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1mm; font-size: 0.85em;">
                        <label style="display: flex; align-items: center; gap: 1mm;">
                            <span>{ "☐" }</span>
                            { &labels.take_after_meal_immediately }
                        </label>
                        <label style="display: flex; align-items: center; gap: 1mm;">
                            <span>{ "☐" }</span>
                            { &labels.continue_until_finish }
                        </label>
                        <label style="display: flex; align-items: center; gap: 1mm;">
                            <span>{ "☐" }</span>
                            { &labels.take_when_symptoms }
                        </label>
                        <label style="display: flex; align-items: center; gap: 1mm;">
                            <span>{ if drug.warning.contains("ง่วง") { "☑" } else { "☐" } }</span>
                            { &labels.may_cause_drowsy }
                        </label>
                        <label style="display: flex; align-items: center; gap: 1mm;">
                            <span>{ "☐" }</span>
                            { &labels.no_milk_antacid }
                        </label>
                        <label style="display: flex; align-items: center; gap: 1mm;">
                            <span>{ "☐" }</span>
                            { &labels.shake_before_use }
                        </label>
                    </div>
                    
                    // Warning if any
                    { if !drug.warning.is_empty() {
                        html! {
                            <div style="margin-top: 1mm; padding: 1mm; background: #ffcccc; border: 1px solid #cc0000; border-radius: 2px; text-align: center; font-weight: bold; color: #cc0000;">
                                { format!("⚠️ {}", drug.warning) }
                            </div>
                        }
                    } else { html! {} }}
                </div>
            </div>
            
            // Tips
            <div class="no-print" style="margin-top: 20px;">
                <div class="card">
                    <h3>{ "💡 คำแนะนำ" }</h3>
                    <ul style="line-height: 2; font-size: 1.1rem;">
                        <li>{ "● = ติ๊กเลือก (เติมอัตโนมัติจากข้อมูลยา)" }</li>
                        <li>{ "☐ = ไม่เลือก (ติ๊กด้วยปากกาถ้าต้องการ)" }</li>
                        <li>{ "เลือกภาษาที่ผู้ป่วยอ่านได้" }</li>
                        <li>{ "กดปุ่ม \"พิมพ์สติกเกอร์\" หรือ Ctrl+P เพื่อพิมพ์" }</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

// Helper component to render sticker for any PrescriptionItem
// StickerCard component removed (unused)

