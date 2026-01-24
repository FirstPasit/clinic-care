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
    
    // Monthly expenses
    let year_i32: i32 = selected_month.split('-').next().unwrap_or("2026").parse().unwrap_or(2026);
    let month_u32: u32 = selected_month.split('-').nth(1).unwrap_or("1").parse().unwrap_or(1);
    let monthly_expenses = Store::get_monthly_expenses(year_i32, month_u32);
    let total_expense: f64 = monthly_expenses.iter().map(|e| e.amount).sum();
    let net_profit = total_revenue - total_expense;

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
            
            // Daily Revenue Chart (Simple CSS bar chart)
            <div class="card mt-5">
                <div class="card-header">
                    <h3 class="card-title">{ "📈 รายได้รายวัน" }</h3>
                </div>
                { if daily_revenue.is_empty() {
                    html! { <p class="text-muted">{ "ไม่มีข้อมูล" }</p> }
                } else {
                    let max_revenue: f64 = daily_revenue.values().cloned().fold(0.0_f64, f64::max);
                    let mut days: Vec<(String, f64, u32)> = daily_revenue.iter()
                        .map(|(d, r)| (d.clone(), *r, *daily_visits.get(d).unwrap_or(&0)))
                        .collect();
                    days.sort_by(|a, b| a.0.cmp(&b.0));
                    
                    html! {
                        <div style="display: flex; align-items: flex-end; gap: 4px; height: 200px; padding: 10px 0;">
                            { for days.iter().map(|(day, revenue, visits)| {
                                let height = if max_revenue > 0.0 { (*revenue / max_revenue * 160.0).max(20.0) } else { 20.0 };
                                html! {
                                    <div style="display: flex; flex-direction: column; align-items: center; flex: 1; min-width: 30px;">
                                        <div style="font-size: 0.7rem; color: var(--color-success); font-weight: bold;">
                                            { format!("฿{:.0}", revenue) }
                                        </div>
                                        <div style={format!("width: 100%; height: {}px; background: linear-gradient(to top, var(--color-primary), var(--color-accent)); border-radius: 4px 4px 0 0; margin: 4px 0;", height)} 
                                             title={format!("วันที่ {}: ฿{:.0} ({} ครั้ง)", day, revenue, visits)}>
                                        </div>
                                        <div style="font-size: 0.75rem; color: #666;">{ day }</div>
                                        <div style="font-size: 0.65rem; color: #999;">{ format!("{}ครั้ง", visits) }</div>
                                    </div>
                                }
                            })}
                        </div>
                    }
                }}
            </div>
            
            // Monthly Summary with Expenses
            <div class="card mt-5">
                <div class="card-header">
                    <h3 class="card-title">{ "💵 สรุปกำไร/ขาดทุน" }</h3>
                </div>
                <div class="grid grid-cols-3 gap-4" style="text-align: center;">
                    <div style="padding: 1.5rem; background: #f0fdf4; border-radius: 12px;">
                        <div style="font-size: 0.9rem; color: #16a34a;">{ "รายได้" }</div>
                        <div style="font-size: 2rem; font-weight: bold; color: #16a34a;">{ format!("฿{:.0}", total_revenue) }</div>
                    </div>
                    <div style="padding: 1.5rem; background: #fef2f2; border-radius: 12px;">
                        <div style="font-size: 0.9rem; color: #dc2626;">{ "ค่าใช้จ่าย" }</div>
                        <div style="font-size: 2rem; font-weight: bold; color: #dc2626;">{ format!("฿{:.0}", total_expense) }</div>
                    </div>
                    <div style={format!("padding: 1.5rem; background: {}; border-radius: 12px;", if net_profit >= 0.0 { "#f0fdf4" } else { "#fef2f2" })}>
                        <div style={format!("font-size: 0.9rem; color: {};", if net_profit >= 0.0 { "#16a34a" } else { "#dc2626" })}>
                            { if net_profit >= 0.0 { "กำไรสุทธิ ✅" } else { "ขาดทุน ❌" } }
                        </div>
                        <div style={format!("font-size: 2rem; font-weight: bold; color: {};", if net_profit >= 0.0 { "#16a34a" } else { "#dc2626" })}>
                            { format!("฿{:.0}", net_profit.abs()) }
                        </div>
                    </div>
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
