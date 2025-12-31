use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Patient {
    pub id: String,
    pub hn: String,
    pub citizen_id: String,
    pub title: String,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    #[serde(default)]
    pub blood_group: String,
    #[serde(default)]
    pub underlying_disease: String, // โรคประจำตัว
    pub drug_allergy: String,
    pub phone: String,
    pub address: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct PrescriptionItem {
    pub name: String,
    pub amount: String,
    pub usage: String,
    pub duration_days: Option<u32>,
    // New fields for drug sticker
    #[serde(default)]
    pub morning: u32,
    #[serde(default)]
    pub noon: u32,
    #[serde(default)]
    pub evening: u32,
    #[serde(default)]
    pub before_bed: u32,
    #[serde(default)]
    pub timing: String, // ก่อนอาหาร, หลังอาหาร, พร้อมอาหาร
    #[serde(default)]
    pub warning: String, // คำเตือน
}

impl Default for PrescriptionItem {
    fn default() -> Self {
        Self {
            name: String::new(),
            amount: String::new(),
            usage: String::new(),
            duration_days: None,
            morning: 0,
            noon: 0,
            evening: 0,
            before_bed: 0,
            timing: "หลังอาหาร".to_string(),
            warning: String::new(),
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct InjectionItem {
    pub name: String,
    pub dose: String,
    pub site: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct TreatmentRecord {
    pub id: String,
    pub patient_id: String,
    pub date: DateTime<Utc>,
    pub symptoms: String,
    pub diagnosis: String,
    pub weight: Option<f32>,
    pub pressure: String,
    pub prescriptions: Vec<PrescriptionItem>,
    pub injections: Vec<InjectionItem>,
    pub doctor_note: String,
    pub price: f64,
}

// ========== NEW: Drug Inventory System ==========

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct DrugItem {
    pub id: String,
    pub name: String,
    pub unit: String,           // เม็ด, ขวด, หลอด, ซอง
    pub stock: u32,             // จำนวนคงเหลือ
    pub min_stock: u32,         // แจ้งเตือนเมื่อต่ำกว่านี้
    pub cost_price: f64,        // ราคาทุน
    pub sell_price: f64,        // ราคาขาย
    pub expiry_date: Option<NaiveDate>, // วันหมดอายุ
    pub category: String,       // หมวดหมู่ยา
    pub description: String,    // คำอธิบาย
    pub default_usage: String,  // วิธีใช้เริ่มต้น
    pub warning: String,        // คำเตือนเริ่มต้น
}

impl Default for DrugItem {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            unit: "เม็ด".to_string(),
            stock: 0,
            min_stock: 10,
            cost_price: 0.0,
            sell_price: 0.0,
            expiry_date: None,
            category: "ยาทั่วไป".to_string(),
            description: String::new(),
            default_usage: String::new(),
            warning: String::new(),
        }
    }
}

// Deleted unused structs: Appointment, ReceiptRecord, ReceiptItem

// ========== NEW: Settings ==========

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ClinicSettings {
    pub clinic_name: String,
    pub clinic_address: String,
    pub clinic_phone: String,
    pub clinic_tax_id: String,
    // Staff info for certificates
    #[serde(default)]
    pub staff_name: String,       // ชื่อพยาบาล/แพทย์
    #[serde(default)]
    pub staff_position: String,   // ตำแหน่ง
    #[serde(default)]
    pub license_number: String,   // เลขใบอนุญาต
    // Display settings
    pub font_size: String,        // small, medium, large, xlarge
    pub theme: String,            // light, dark, high-contrast
    pub sticker_size: String,     // small, medium, large
    pub next_receipt_no: u32,
}

impl Default for ClinicSettings {
    fn default() -> Self {
        Self {
            clinic_name: "ญ.หญิงคลินิกการพยาบาลและการผดุงครรภ์".to_string(),
            clinic_address: "83/9 หมู่ 7 ต.กุยบุรี อ.กุยบุรี จ.ประจวบคีรีขันธ์".to_string(),
            clinic_phone: "081-014-1551".to_string(),
            clinic_tax_id: "".to_string(),
            staff_name: "นางสมหญิง วีระจินตนา".to_string(),
            staff_position: "พยาบาลวิชาชีพชำนาญการ".to_string(),
            license_number: "4511055362".to_string(),
            font_size: "large".to_string(),
            theme: "light".to_string(),
            sticker_size: "large".to_string(),
            next_receipt_no: 1,
        }
    }
}

