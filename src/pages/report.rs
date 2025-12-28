use yew::prelude::*;
use crate::store::Store;
use chrono::prelude::*;

#[function_component(Report)]
pub fn report() -> Html {
    let records = Store::get_records();
    let patients = Store::get_patients();
    
    // Current month filter
    let selected_month = use_state(|| Local::now().format("%Y-%m").to_string());
    
    // Filter records by selected month
    let month_records: Vec<_> = records.iter().filter(|r| {
        r.date.with_timezone(&Local).format("%Y-%m").to_string() == *selected_month
    }).collect();
    
    // Calculate stats
    let total_revenue: f64 = month_records.iter().map(|r| r.price).sum();
    let total_visits = month_records.len();
    let unique_patients: std::collections::HashSet<_> = month_records.iter().map(|r| r.patient_id.clone()).collect();
    let unique_patient_count = unique_patients.len();
    
    // Average per visit
    let avg_per_visit = if total_visits > 0 { total_revenue / total_visits as f64 } else { 0.0 };
    
    // Diagnosis frequency
    let mut diagnosis_count: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    for r in &month_records {
        if !r.diagnosis.is_empty() {
            *diagnosis_count.entry(r.diagnosis.clone()).or_insert(0) += 1;
        }
    }
    let mut diagnosis_sorted: Vec<_> = diagnosis_count.into_iter().collect();
    diagnosis_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Daily breakdown
    let mut daily_revenue: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    let mut daily_visits: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    for r in &month_records {
        let day = r.date.with_timezone(&Local).format("%d").to_string();
        *daily_revenue.entry(day.clone()).or_insert(0.0) += r.price;
        *daily_visits.entry(day).or_insert(0) += 1;
    }
    
    // Get available months
    let mut months: Vec<String> = records.iter()
        .map(|r| r.date.with_timezone(&Local).format("%Y-%m").to_string())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    months.sort();
    months.reverse();
    
    // Add current month if not exists
    let current_month = Local::now().format("%Y-%m").to_string();
    if !months.contains(&current_month) {
        months.insert(0, current_month);
    }

    html! {
        <>
            <div class="page-header flex justify-between items-center flex-wrap gap-4">
                <div>
                    <h1 class="page-title">{ "📊 รายงานประจำเดือน" }</h1>
                    <p class="page-subtitle">{ "สรุปรายได้และสถิติการรักษา" }</p>
                </div>
                
                // Month selector
                <div class="form-group" style="margin-bottom: 0; min-width: 200px;">
                    <select onchange={{
                        let selected_month = selected_month.clone();
                        move |e: Event| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            selected_month.set(input.value());
                        }
                    }}>
                        { for months.iter().map(|m| {
                            let display = format_month_thai(m);
                            html! {
                                <option value={m.clone()} selected={*selected_month == *m}>
                                    { display }
                                </option>
                            }
                        })}
                    </select>
                </div>
            </div>
            
            // Summary Stats
            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-card-icon success">{ "💰" }</div>
                    <div class="stat-card-value">{ format!("฿{:.0}", total_revenue) }</div>
                    <div class="stat-card-label">{ "รายได้รวม" }</div>
                </div>
                
                <div class="stat-card">
                    <div class="stat-card-icon accent">{ "📋" }</div>
                    <div class="stat-card-value">{ total_visits }</div>
                    <div class="stat-card-label">{ "จำนวนครั้งที่รักษา" }</div>
                </div>
                
                <div class="stat-card">
                    <div class="stat-card-icon accent">{ "👥" }</div>
                    <div class="stat-card-value">{ unique_patient_count }</div>
                    <div class="stat-card-label">{ "ผู้ป่วยไม่ซ้ำ" }</div>
                </div>
                
                <div class="stat-card">
                    <div class="stat-card-icon warning">{ "📈" }</div>
                    <div class="stat-card-value">{ format!("฿{:.0}", avg_per_visit) }</div>
                    <div class="stat-card-label">{ "เฉลี่ยต่อครั้ง" }</div>
                </div>
            </div>
            
            <div class="grid grid-cols-2 gap-5">
                // Top Diagnoses
                <div class="card">
                    <div class="card-header">
                        <h3 class="card-title">{ "🏥 โรค/อาการที่พบบ่อย" }</h3>
                    </div>
                    { if diagnosis_sorted.is_empty() {
                        html! { <p class="text-muted">{ "ไม่มีข้อมูล" }</p> }
                    } else {
                        html! {
                            <table class="data-table">
                                <thead>
                                    <tr>
                                        <th>{ "อันดับ" }</th>
                                        <th>{ "การวินิจฉัย" }</th>
                                        <th>{ "จำนวน" }</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    { for diagnosis_sorted.iter().take(10).enumerate().map(|(i, (diag, count))| {
                                        html! {
                                            <tr>
                                                <td>{ i + 1 }</td>
                                                <td>{ diag }</td>
                                                <td><span class="badge badge-accent">{ format!("{} ครั้ง", count) }</span></td>
                                            </tr>
                                        }
                                    })}
                                </tbody>
                            </table>
                        }
                    }}
                </div>
                
                // Recent Transactions
                <div class="card">
                    <div class="card-header">
                        <h3 class="card-title">{ "💳 รายการล่าสุด" }</h3>
                    </div>
                    { if month_records.is_empty() {
                        html! { <p class="text-muted">{ "ไม่มีข้อมูลในเดือนนี้" }</p> }
                    } else {
                        html! {
                            <table class="data-table">
                                <thead>
                                    <tr>
                                        <th>{ "วันที่" }</th>
                                        <th>{ "ผู้ป่วย" }</th>
                                        <th>{ "จำนวนเงิน" }</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    { for month_records.iter().rev().take(10).map(|r| {
                                        let patient_name = patients.iter()
                                            .find(|p| p.id == r.patient_id)
                                            .map(|p| format!("{}{}", p.first_name, p.last_name))
                                            .unwrap_or_else(|| "-".to_string());
                                        let date = r.date.with_timezone(&Local).format("%d/%m %H:%M").to_string();
                                        html! {
                                            <tr>
                                                <td>{ date }</td>
                                                <td>{ patient_name }</td>
                                                <td class="text-success font-bold">{ format!("฿{:.0}", r.price) }</td>
                                            </tr>
                                        }
                                    })}
                                </tbody>
                            </table>
                        }
                    }}
                </div>
            </div>
            
            // Print Button
            <div class="card mt-6">
                <div class="flex justify-between items-center">
                    <div>
                        <h3 style="margin: 0;">{ "🖨️ พิมพ์รายงาน" }</h3>
                        <p class="text-muted">{ "พิมพ์สรุปรายงานประจำเดือน" }</p>
                    </div>
                    <button class="btn btn-primary btn-lg" onclick={|_| {
                        let _ = web_sys::window().unwrap().print();
                    }}>
                        { "🖨️ พิมพ์รายงาน" }
                    </button>
                </div>
            </div>
        </>
    }
}

fn format_month_thai(ym: &str) -> String {
    let parts: Vec<&str> = ym.split('-').collect();
    if parts.len() != 2 { return ym.to_string(); }
    
    let year: i32 = parts[0].parse().unwrap_or(2024);
    let month: u32 = parts[1].parse().unwrap_or(1);
    
    let thai_year = year + 543;
    let month_name = match month {
        1 => "มกราคม",
        2 => "กุมภาพันธ์",
        3 => "มีนาคม",
        4 => "เมษายน",
        5 => "พฤษภาคม",
        6 => "มิถุนายน",
        7 => "กรกฎาคม",
        8 => "สิงหาคม",
        9 => "กันยายน",
        10 => "ตุลาคม",
        11 => "พฤศจิกายน",
        12 => "ธันวาคม",
        _ => "ไม่ทราบ",
    };
    
    format!("{} {}", month_name, thai_year)
}
