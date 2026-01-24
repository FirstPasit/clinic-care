use gloo::storage::{LocalStorage, Storage};
use crate::models::{Patient, TreatmentRecord, DrugItem, ClinicSettings, Expense, DrugPurchase, Appointment};

const KEY_PATIENTS: &str = "clinic_patients";
const KEY_RECORDS: &str = "clinic_records";
#[allow(dead_code)]
const KEY_LAST_HN: &str = "clinic_last_hn";
const KEY_DRUGS: &str = "clinic_drugs";
const KEY_SETTINGS: &str = "clinic_settings";
const KEY_EXPENSES: &str = "clinic_expenses";
const KEY_DRUG_PURCHASES: &str = "clinic_drug_purchases";
const KEY_APPOINTMENTS: &str = "clinic_appointments";


pub struct Store;

impl Store {
    // ========== Patients ==========
    pub fn get_patients() -> Vec<Patient> {
        LocalStorage::get(KEY_PATIENTS).unwrap_or_else(|_| Vec::new())
    }

    pub fn save_patient(patient: Patient) {
        let mut patients = Self::get_patients();
        patients.push(patient);
        let _ = LocalStorage::set(KEY_PATIENTS, patients);
    }
    
    pub fn delete_patient(patient_id: &str) {
        // 1. Delete Patient
        let patients: Vec<Patient> = Self::get_patients()
            .into_iter()
            .filter(|p| p.id != patient_id)
            .collect();
        let _ = LocalStorage::set(KEY_PATIENTS, patients);

        // 2. Delete Related Records
        let records: Vec<TreatmentRecord> = Self::get_records()
            .into_iter()
            .filter(|r| r.patient_id != patient_id)
            .collect();
        let _ = LocalStorage::set(KEY_RECORDS, records);
    }
    
    pub fn update_patient(updated: Patient) {
        let mut patients = Self::get_patients();
        if let Some(pos) = patients.iter().position(|p| p.id == updated.id) {
            patients[pos] = updated;
            let _ = LocalStorage::set(KEY_PATIENTS, patients);
        }
    }
    


    // ========== Treatment Records ==========
    pub fn get_records() -> Vec<TreatmentRecord> {
        LocalStorage::get(KEY_RECORDS).unwrap_or_else(|_| Vec::new())
    }
    
    pub fn get_records_by_patient(patient_id: &str) -> Vec<TreatmentRecord> {
        Self::get_records()
            .into_iter()
            .filter(|r| r.patient_id == patient_id)
            .collect()
    }

    pub fn save_record(record: TreatmentRecord) {
        // Reduce drug stock for each prescription
        for rx in &record.prescriptions {
            Self::reduce_drug_stock_by_name(&rx.name, &rx.amount);
        }
        
        let mut records = Self::get_records();
        records.push(record);
        let _ = LocalStorage::set(KEY_RECORDS, records);
    }
    
    /// Reduce drug stock by name and amount string (e.g., "10 เม็ด")
    pub fn reduce_drug_stock_by_name(drug_name: &str, amount_str: &str) {
        // Parse amount from string like "10 เม็ด" or "5 ซอง"
        let amount: u32 = amount_str
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap_or(0);
        
        if amount == 0 { return; }
        
        let mut drugs = Self::get_drugs();
        if let Some(pos) = drugs.iter().position(|d| d.name == drug_name) {
            // Use saturating_sub to prevent underflow
            drugs[pos].stock = drugs[pos].stock.saturating_sub(amount);
            let _ = LocalStorage::set(KEY_DRUGS, drugs);
        }
    }
    


    #[allow(dead_code)]
    pub fn next_hn() -> String {
        let last_hn: u32 = LocalStorage::get(KEY_LAST_HN).unwrap_or(0);
        let next = last_hn + 1;
        let _ = LocalStorage::set(KEY_LAST_HN, next);
        format!("HN-{:05}", next)
    }

    // ========== Drug Inventory ==========
    pub fn get_drugs() -> Vec<DrugItem> {
        LocalStorage::get(KEY_DRUGS).unwrap_or_else(|_| Vec::new())
    }

    pub fn save_drug(drug: DrugItem) {
        let mut drugs = Self::get_drugs();
        drugs.push(drug);
        let _ = LocalStorage::set(KEY_DRUGS, drugs);
    }
    
    pub fn update_drug(updated: DrugItem) {
        let mut drugs = Self::get_drugs();
        if let Some(pos) = drugs.iter().position(|d| d.id == updated.id) {
            drugs[pos] = updated;
            let _ = LocalStorage::set(KEY_DRUGS, drugs);
        }
    }
    
    pub fn delete_drug(drug_id: &str) {
        let drugs: Vec<DrugItem> = Self::get_drugs()
            .into_iter()
            .filter(|d| d.id != drug_id)
            .collect();
        let _ = LocalStorage::set(KEY_DRUGS, drugs);
    }
    
    pub fn get_low_stock_drugs() -> Vec<DrugItem> {
        Self::get_drugs()
            .into_iter()
            .filter(|d| d.stock <= d.min_stock)
            .collect()
    }
    
    pub fn get_expiring_drugs() -> Vec<DrugItem> {
        use chrono::{Local, Duration};
        let warning_date = Local::now().naive_local().date() + Duration::days(30);
        Self::get_drugs()
            .into_iter()
            .filter(|d| {
                if let Some(exp) = d.expiry_date {
                    exp <= warning_date
                } else {
                    false
                }
            })
            .collect()
    }

    // ========== Settings ==========
    pub fn get_settings() -> ClinicSettings {
        LocalStorage::get(KEY_SETTINGS).unwrap_or_else(|_| ClinicSettings::default())
    }
    
    pub fn save_settings(settings: ClinicSettings) {
        let _ = LocalStorage::set(KEY_SETTINGS, settings);
    }

    // ========== Receipt Number ==========

    
    // ========== Statistics ==========
    pub fn get_today_revenue() -> f64 {
        use chrono::Local;
        let today = Local::now().date_naive();
        Self::get_records()
            .into_iter()
            .filter(|r| r.date.with_timezone(&Local).date_naive() == today)
            .map(|r| r.price)
            .sum()
    }
    

    
    pub fn get_today_patient_count() -> usize {
        use chrono::Local;
        let today = Local::now().date_naive();
        Self::get_records()
            .into_iter()
            .filter(|r| r.date.with_timezone(&Local).date_naive() == today)
            .count()
    }
    
    // ========== Expenses ==========
    pub fn get_expenses() -> Vec<Expense> {
        LocalStorage::get(KEY_EXPENSES).unwrap_or_else(|_| Vec::new())
    }
    
    pub fn save_expense(expense: Expense) {
        let mut expenses = Self::get_expenses();
        expenses.push(expense);
        let _ = LocalStorage::set(KEY_EXPENSES, expenses);
    }
    
    pub fn delete_expense(expense_id: &str) {
        let expenses: Vec<Expense> = Self::get_expenses()
            .into_iter()
            .filter(|e| e.id != expense_id)
            .collect();
        let _ = LocalStorage::set(KEY_EXPENSES, expenses);
    }
    
    pub fn get_monthly_expenses(year: i32, month: u32) -> Vec<Expense> {
        use chrono::Datelike;
        Self::get_expenses()
            .into_iter()
            .filter(|e| {
                let d = e.date.with_timezone(&chrono::Local);
                d.year() == year && d.month() == month
            })
            .collect()
    }
    
    // ========== Drug Purchases ==========
    pub fn get_drug_purchases() -> Vec<DrugPurchase> {
        LocalStorage::get(KEY_DRUG_PURCHASES).unwrap_or_else(|_| Vec::new())
    }
    
    pub fn save_drug_purchase(purchase: DrugPurchase) {
        // Also increase drug stock
        let mut drugs = Self::get_drugs();
        if let Some(pos) = drugs.iter().position(|d| d.id == purchase.drug_id) {
            drugs[pos].stock += purchase.quantity;
            // Update expiry date if provided
            if purchase.expiry_date.is_some() {
                drugs[pos].expiry_date = purchase.expiry_date;
            }
            let _ = LocalStorage::set(KEY_DRUGS, drugs);
        }
        
        let mut purchases = Self::get_drug_purchases();
        purchases.push(purchase);
        let _ = LocalStorage::set(KEY_DRUG_PURCHASES, purchases);
    }
    
    pub fn get_purchases_by_drug(drug_id: &str) -> Vec<DrugPurchase> {
        Self::get_drug_purchases()
            .into_iter()
            .filter(|p| p.drug_id == drug_id)
            .collect()
    }
    
    // ========== Appointments ==========
    pub fn get_appointments() -> Vec<Appointment> {
        LocalStorage::get(KEY_APPOINTMENTS).unwrap_or_else(|_| Vec::new())
    }
    
    pub fn save_appointment(appointment: Appointment) {
        let mut appointments = Self::get_appointments();
        appointments.push(appointment);
        let _ = LocalStorage::set(KEY_APPOINTMENTS, appointments);
    }
    
    pub fn update_appointment(updated: Appointment) {
        let mut appointments = Self::get_appointments();
        if let Some(pos) = appointments.iter().position(|a| a.id == updated.id) {
            appointments[pos] = updated;
            let _ = LocalStorage::set(KEY_APPOINTMENTS, appointments);
        }
    }
    
    pub fn delete_appointment(appointment_id: &str) {
        let appointments: Vec<Appointment> = Self::get_appointments()
            .into_iter()
            .filter(|a| a.id != appointment_id)
            .collect();
        let _ = LocalStorage::set(KEY_APPOINTMENTS, appointments);
    }
    
    pub fn get_today_appointments() -> Vec<Appointment> {
        let today = chrono::Local::now().date_naive();
        Self::get_appointments()
            .into_iter()
            .filter(|a| a.date == today && a.status == "pending")
            .collect()
    }
    
    pub fn get_appointments_by_date(date: chrono::NaiveDate) -> Vec<Appointment> {
        Self::get_appointments()
            .into_iter()
            .filter(|a| a.date == date)
            .collect()
    }
    
    // ========== Records by Date Range ==========
    pub fn get_records_by_date_range(start: chrono::NaiveDate, end: chrono::NaiveDate) -> Vec<TreatmentRecord> {
        use chrono::Local;
        Self::get_records()
            .into_iter()
            .filter(|r| {
                let d = r.date.with_timezone(&Local).date_naive();
                d >= start && d <= end
            })
            .collect()
    }
    
    pub fn get_monthly_revenue(year: i32, month: u32) -> f64 {
        use chrono::Datelike;
        Self::get_records()
            .into_iter()
            .filter(|r| {
                let d = r.date.with_timezone(&chrono::Local);
                d.year() == year && d.month() == month
            })
            .map(|r| r.price)
            .sum()
    }
    
    pub fn get_monthly_patient_count(year: i32, month: u32) -> usize {
        use chrono::Datelike;
        Self::get_records()
            .into_iter()
            .filter(|r| {
                let d = r.date.with_timezone(&chrono::Local);
                d.year() == year && d.month() == month
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_drug_amount() {
        // Helper logic check
        fn parse(s: &str) -> u32 {
            s.chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap_or(0)
        }
        assert_eq!(parse("10"), 10);
        assert_eq!(parse("5 เม็ด"), 5); 
        assert_eq!(parse("100ml"), 100);
        assert_eq!(parse(""), 0);
    }

    #[test]
    fn test_hn_format() {
        let next = 16;
        assert_eq!(format!("HN-{:05}", next), "HN-00016");
    }
}
