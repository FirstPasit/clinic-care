use yew::prelude::*;
use web_sys::HtmlInputElement;
use chrono::Utc;
use uuid::Uuid;
use crate::models::Expense;
use crate::store::Store;
use crate::components::{ToastContext, ToastAction, ToastType};
use chrono::prelude::*;

#[function_component(Expenses)]
pub fn expenses() -> Html {
    let toast = use_context::<ToastContext>();
    let expenses = use_state(|| Store::get_expenses());
    let show_form = use_state(|| false);
    
    // Form state
    let category = use_state(|| "อื่นๆ".to_string());
    let description = use_state(|| String::new());
    let amount = use_state(|| String::new());
    let note = use_state(|| String::new());
    let date_str = use_state(|| Local::now().format("%Y-%m-%d").to_string());
    
    // Filter state
    let filter_month = use_state(|| Local::now().month());
    let filter_year = use_state(|| Local::now().year());
    
    let filtered_expenses = {
        let year = *filter_year;
        let month = *filter_month;
        Store::get_monthly_expenses(year, month)
    };
    
    let total_expenses: f64 = filtered_expenses.iter().map(|e| e.amount).sum();
    
    let on_save = {
        let category = category.clone();
        let description = description.clone();
        let amount = amount.clone();
        let note = note.clone();
        let date_str = date_str.clone();
        let expenses = expenses.clone();
        let show_form = show_form.clone();
        let toast = toast.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let amount_val: f64 = (*amount).parse().unwrap_or(0.0);
            if amount_val <= 0.0 {
                if let Some(ref t) = toast {
                    t.dispatch(ToastAction::Add("❌ กรุณากรอกจำนวนเงิน".to_string(), ToastType::Error));
                }
                return;
            }
            
            let date = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
                .map(|dt| chrono::Utc.from_utc_datetime(&dt))
                .unwrap_or_else(|_| Utc::now());
            
            let expense = Expense {
                id: Uuid::new_v4().to_string(),
                date,
                category: (*category).clone(),
                description: (*description).clone(),
                amount: amount_val,
                note: (*note).clone(),
            };
            
            Store::save_expense(expense);
            expenses.set(Store::get_expenses());
            
            // Reset form
            category.set("อื่นๆ".to_string());
            description.set(String::new());
            amount.set(String::new());
            note.set(String::new());
            show_form.set(false);
            
            if let Some(ref t) = toast {
                t.dispatch(ToastAction::Add("✅ บันทึกค่าใช้จ่ายแล้ว".to_string(), ToastType::Success));
            }
        })
    };
    
    let on_delete = {
        let expenses = expenses.clone();
        let toast = toast.clone();
        
        Callback::from(move |id: String| {
            if web_sys::window().unwrap().confirm_with_message("ยืนยันการลบรายการนี้?").unwrap_or(false) {
                Store::delete_expense(&id);
                expenses.set(Store::get_expenses());
                if let Some(ref t) = toast {
                    t.dispatch(ToastAction::Add("🗑️ ลบรายการแล้ว".to_string(), ToastType::Success));
                }
            }
        })
    };
    
    let expense_categories = vec!["ค่าเช่า", "ค่าน้ำ", "ค่าไฟ", "ค่ายา", "ค่าอุปกรณ์", "ค่าจ้าง", "อื่นๆ"];

    html! {
        <>
            <div class="page-header flex justify-between items-center">
                <div>
                    <h1 class="page-title">{ "💰 ค่าใช้จ่ายคลินิก" }</h1>
                    <p class="page-subtitle">{ "บันทึกรายจ่ายต่างๆ ของคลินิก" }</p>
                </div>
                <button class="btn btn-primary btn-lg" onclick={{
                    let show_form = show_form.clone();
                    move |_| show_form.set(!*show_form)
                }}>
                    { if *show_form { "❌ ยกเลิก" } else { "➕ เพิ่มรายจ่าย" } }
                </button>
            </div>
            
            // Add expense form
            { if *show_form {
                html! {
                    <div class="card mb-4">
                        <h3 class="mb-4">{ "📝 เพิ่มรายจ่ายใหม่" }</h3>
                        <form onsubmit={on_save}>
                            <div class="grid grid-cols-2 gap-4">
                                <div class="form-group">
                                    <label class="form-label">{ "วันที่" }</label>
                                    <input type="date" value={(*date_str).clone()} oninput={{
                                        let date_str = date_str.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            date_str.set(input.value());
                                        })
                                    }} />
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{ "หมวดหมู่" }</label>
                                    <select onchange={{
                                        let category = category.clone();
                                        Callback::from(move |e: Event| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            category.set(input.value());
                                        })
                                    }}>
                                        { for expense_categories.iter().map(|c| {
                                            html! { <option value={*c} selected={*category == *c}>{ c }</option> }
                                        })}
                                    </select>
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{ "รายละเอียด *" }</label>
                                    <input type="text" required=true value={(*description).clone()}
                                        placeholder="เช่น ค่าไฟเดือน ม.ค."
                                        oninput={{
                                            let description = description.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                description.set(input.value());
                                            })
                                        }} />
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{ "จำนวนเงิน (บาท) *" }</label>
                                    <input type="number" step="0.01" required=true value={(*amount).clone()}
                                        placeholder="0.00"
                                        oninput={{
                                            let amount = amount.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                amount.set(input.value());
                                            })
                                        }} />
                                </div>
                                
                                <div class="form-group" style="grid-column: 1 / -1;">
                                    <label class="form-label">{ "หมายเหตุ" }</label>
                                    <input type="text" value={(*note).clone()}
                                        placeholder="หมายเหตุเพิ่มเติม"
                                        oninput={{
                                            let note = note.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                note.set(input.value());
                                            })
                                        }} />
                                </div>
                            </div>
                            
                            <div class="flex justify-end mt-4">
                                <button type="submit" class="btn btn-primary btn-lg">{ "💾 บันทึก" }</button>
                            </div>
                        </form>
                    </div>
                }
            } else { html! {} }}
            
            // Filter
            <div class="card mb-4">
                <div class="flex items-center gap-4">
                    <span class="font-bold">{ "📅 ดูเดือน:" }</span>
                    <select value={filter_month.to_string()} onchange={{
                        let filter_month = filter_month.clone();
                        Callback::from(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            filter_month.set(input.value().parse().unwrap_or(1));
                        })
                    }}>
                        { for (1..=12).map(|m| {
                            let month_names = ["", "ม.ค.", "ก.พ.", "มี.ค.", "เม.ย.", "พ.ค.", "มิ.ย.", 
                                              "ก.ค.", "ส.ค.", "ก.ย.", "ต.ค.", "พ.ย.", "ธ.ค."];
                            html! { <option value={m.to_string()} selected={*filter_month == m}>{ month_names[m as usize] }</option> }
                        })}
                    </select>
                    <select value={filter_year.to_string()} onchange={{
                        let filter_year = filter_year.clone();
                        Callback::from(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            filter_year.set(input.value().parse().unwrap_or(2026));
                        })
                    }}>
                        { for (2024..=2030).map(|y| {
                            html! { <option value={y.to_string()} selected={*filter_year == y}>{ y + 543 }</option> }
                        })}
                    </select>
                    <div class="ml-auto" style="font-size: 1.5rem; font-weight: bold; color: var(--color-error);">
                        { format!("รวม: ฿{:.2}", total_expenses) }
                    </div>
                </div>
            </div>
            
            // Expense list
            <div class="card">
                { if filtered_expenses.is_empty() {
                    html! {
                        <div class="empty-state">
                            <div class="empty-state-icon">{ "💸" }</div>
                            <h3 class="empty-state-title">{ "ยังไม่มีรายจ่ายในเดือนนี้" }</h3>
                            <p class="empty-state-text">{ "กดปุ่ม \"เพิ่มรายจ่าย\" เพื่อบันทึกค่าใช้จ่าย" }</p>
                        </div>
                    }
                } else {
                    html! {
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <th>{ "วันที่" }</th>
                                    <th>{ "หมวดหมู่" }</th>
                                    <th>{ "รายละเอียด" }</th>
                                    <th>{ "จำนวนเงิน" }</th>
                                    <th>{ "หมายเหตุ" }</th>
                                    <th>{ "" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for filtered_expenses.iter().map(|exp| {
                                    let id = exp.id.clone();
                                    let on_delete = on_delete.clone();
                                    html! {
                                        <tr>
                                            <td>{ exp.date.with_timezone(&Local).format("%d/%m/%y").to_string() }</td>
                                            <td><span class="badge">{ &exp.category }</span></td>
                                            <td>{ &exp.description }</td>
                                            <td style="font-weight: bold; color: var(--color-error);">{ format!("฿{:.2}", exp.amount) }</td>
                                            <td>{ &exp.note }</td>
                                            <td>
                                                <button class="btn btn-error btn-sm" onclick={move |_| on_delete.emit(id.clone())}>
                                                    { "🗑️" }
                                                </button>
                                            </td>
                                        </tr>
                                    }
                                })}
                            </tbody>
                        </table>
                    }
                }}
            </div>
        </>
    }
}
