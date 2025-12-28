use yew::prelude::*;
use crate::models::DrugItem;
use crate::store::Store;
use crate::components::{ToastContext, ToastAction, ToastType};
use web_sys::HtmlInputElement;
use uuid::Uuid;

#[function_component(Drugs)]
pub fn drugs() -> Html {
    let toast = use_context::<ToastContext>();
    let drugs = use_state(|| Store::get_drugs());
    let show_form = use_state(|| false);
    let editing = use_state(|| None::<DrugItem>);
    
    // Form state
    let name = use_state(|| String::new());
    let unit = use_state(|| "เม็ด".to_string());
    let stock = use_state(|| "0".to_string());
    let min_stock = use_state(|| "10".to_string());
    let cost_price = use_state(|| "0".to_string());
    let sell_price = use_state(|| "0".to_string());
    let category = use_state(|| "ยาทั่วไป".to_string());
    let default_usage = use_state(|| String::new());
    let warning = use_state(|| String::new());
    
    let low_stock = Store::get_low_stock_drugs();
    let expiring = Store::get_expiring_drugs();
    
    let clear_form = {
        let name = name.clone();
        let unit = unit.clone();
        let stock = stock.clone();
        let min_stock = min_stock.clone();
        let cost_price = cost_price.clone();
        let sell_price = sell_price.clone();
        let category = category.clone();
        let default_usage = default_usage.clone();
        let warning = warning.clone();
        let editing = editing.clone();
        
        Callback::from(move |_| {
            name.set(String::new());
            unit.set("เม็ด".to_string());
            stock.set("0".to_string());
            min_stock.set("10".to_string());
            cost_price.set("0".to_string());
            sell_price.set("0".to_string());
            category.set("ยาทั่วไป".to_string());
            default_usage.set(String::new());
            warning.set(String::new());
            editing.set(None);
        })
    };
    
    let on_add_new = {
        let show_form = show_form.clone();
        let clear_form = clear_form.clone();
        Callback::from(move |_| {
            clear_form.emit(());
            show_form.set(true);
        })
    };
    
    let on_edit = {
        let show_form = show_form.clone();
        let editing = editing.clone();
        let name = name.clone();
        let unit = unit.clone();
        let stock = stock.clone();
        let min_stock = min_stock.clone();
        let cost_price = cost_price.clone();
        let sell_price = sell_price.clone();
        let category = category.clone();
        let default_usage = default_usage.clone();
        let warning = warning.clone();
        
        Callback::from(move |drug: DrugItem| {
            name.set(drug.name.clone());
            unit.set(drug.unit.clone());
            stock.set(drug.stock.to_string());
            min_stock.set(drug.min_stock.to_string());
            cost_price.set(drug.cost_price.to_string());
            sell_price.set(drug.sell_price.to_string());
            category.set(drug.category.clone());
            default_usage.set(drug.default_usage.clone());
            warning.set(drug.warning.clone());
            editing.set(Some(drug));
            show_form.set(true);
        })
    };
    
    let on_delete = {
        let drugs = drugs.clone();
        let toast = toast.clone();
        Callback::from(move |drug_id: String| {
            Store::delete_drug(&drug_id);
            drugs.set(Store::get_drugs());
            if let Some(ref t) = toast {
                t.dispatch(ToastAction::Add(
                    "🗑️ ลบยาเรียบร้อยแล้ว".to_string(),
                    ToastType::Success
                ));
            }
        })
    };
    
    let on_submit = {
        let drugs = drugs.clone();
        let show_form = show_form.clone();
        let editing = editing.clone();
        let name = name.clone();
        let unit = unit.clone();
        let stock = stock.clone();
        let min_stock = min_stock.clone();
        let cost_price = cost_price.clone();
        let sell_price = sell_price.clone();
        let category = category.clone();
        let default_usage = default_usage.clone();
        let warning = warning.clone();
        let clear_form = clear_form.clone();
        let toast = toast.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let drug = DrugItem {
                id: editing.as_ref().map(|d| d.id.clone()).unwrap_or_else(|| Uuid::new_v4().to_string()),
                name: (*name).clone(),
                unit: (*unit).clone(),
                stock: stock.parse().unwrap_or(0),
                min_stock: min_stock.parse().unwrap_or(10),
                cost_price: cost_price.parse().unwrap_or(0.0),
                sell_price: sell_price.parse().unwrap_or(0.0),
                expiry_date: None,
                category: (*category).clone(),
                description: String::new(),
                default_usage: (*default_usage).clone(),
                warning: (*warning).clone(),
            };
            
            let msg = if editing.is_some() {
                Store::update_drug(drug);
                "✅ แก้ไขข้อมูลยาเรียบร้อยแล้ว!"
            } else {
                Store::save_drug(drug);
                "✅ เพิ่มยาใหม่เรียบร้อยแล้ว!"
            };
            
            if let Some(ref t) = toast {
                t.dispatch(ToastAction::Add(msg.to_string(), ToastType::Success));
            }
            
            drugs.set(Store::get_drugs());
            clear_form.emit(());
            show_form.set(false);
        })
    };

    html! {
        <>
            <div class="page-header flex justify-between items-center">
                <div>
                    <h1 class="page-title">{ "💊 คลังยา" }</h1>
                    <p class="page-subtitle">{ format!("มียาทั้งหมด {} รายการ", drugs.len()) }</p>
                </div>
                <button class="btn btn-primary btn-lg" onclick={on_add_new}>
                    { "➕ เพิ่มยาใหม่" }
                </button>
            </div>
            
            // Alerts
            { if !low_stock.is_empty() {
                html! {
                    <div class="alert alert-warning">
                        <span class="alert-icon">{ "⚠️" }</span>
                        <span>{ format!("มียาใกล้หมด {} รายการ: {}", low_stock.len(), 
                            low_stock.iter().map(|d| d.name.as_str()).collect::<Vec<_>>().join(", ")) }</span>
                    </div>
                }
            } else { html! {} }}
            
            { if !expiring.is_empty() {
                html! {
                    <div class="alert alert-error">
                        <span class="alert-icon">{ "🚨" }</span>
                        <span>{ format!("มียาใกล้หมดอายุ {} รายการ", expiring.len()) }</span>
                    </div>
                }
            } else { html! {} }}
            
            // Add/Edit Form
            { if *show_form {
                html! {
                    <div class="card mb-6">
                        <div class="card-header">
                            <h3 class="card-title">{ if editing.is_some() { "✏️ แก้ไขยา" } else { "➕ เพิ่มยาใหม่" } }</h3>
                            <button class="btn btn-ghost" onclick={{
                                let show_form = show_form.clone();
                                let clear_form = clear_form.clone();
                                move |_| {
                                    clear_form.emit(());
                                    show_form.set(false);
                                }
                            }}>{ "✕ ปิด" }</button>
                        </div>
                        
                        <form onsubmit={on_submit}>
                            <div class="grid grid-cols-2 gap-4">
                                <div class="form-group" style="grid-column: 1 / -1;">
                                    <label class="form-label">{ "ชื่อยา *" }</label>
                                    <input type="text" required=true value={(*name).clone()}
                                        placeholder="เช่น Paracetamol 500mg"
                                        oninput={let name = name.clone(); Callback::from(move |e: InputEvent| name.set(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{ "หน่วย" }</label>
                                    <select onchange={let unit = unit.clone(); Callback::from(move |e: Event| unit.set(e.target_unchecked_into::<HtmlInputElement>().value()))}>
                                        <option value="เม็ด" selected={*unit == "เม็ด"}>{ "เม็ด" }</option>
                                        <option value="แคปซูล" selected={*unit == "แคปซูล"}>{ "แคปซูล" }</option>
                                        <option value="ซอง" selected={*unit == "ซอง"}>{ "ซอง" }</option>
                                        <option value="ขวด" selected={*unit == "ขวด"}>{ "ขวด" }</option>
                                        <option value="หลอด" selected={*unit == "หลอด"}>{ "หลอด" }</option>
                                        <option value="กล่อง" selected={*unit == "กล่อง"}>{ "กล่อง" }</option>
                                        <option value="ชิ้น" selected={*unit == "ชิ้น"}>{ "ชิ้น" }</option>
                                    </select>
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{ "หมวดหมู่" }</label>
                                    <select onchange={let category = category.clone(); Callback::from(move |e: Event| category.set(e.target_unchecked_into::<HtmlInputElement>().value()))}>
                                        <option value="ยาทั่วไป">{ "ยาทั่วไป" }</option>
                                        <option value="ยาแก้ปวด">{ "ยาแก้ปวด" }</option>
                                        <option value="ยาลดไข้">{ "ยาลดไข้" }</option>
                                        <option value="ยาปฏิชีวนะ">{ "ยาปฏิชีวนะ" }</option>
                                        <option value="ยาแก้แพ้">{ "ยาแก้แพ้" }</option>
                                        <option value="วิตามิน">{ "วิตามิน" }</option>
                                        <option value="ยาทา">{ "ยาทา" }</option>
                                        <option value="ยาหยอด">{ "ยาหยอด" }</option>
                                        <option value="อื่นๆ">{ "อื่นๆ" }</option>
                                    </select>
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{ "จำนวนคงเหลือ" }</label>
                                    <input type="number" min="0" inputmode="numeric" value={(*stock).clone()}
                                        oninput={let stock = stock.clone(); Callback::from(move |e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            let filtered: String = input.value().chars().filter(|c| c.is_ascii_digit()).collect();
                                            stock.set(filtered);
                                        })} />
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{ "แจ้งเตือนเมื่อต่ำกว่า" }</label>
                                    <input type="number" min="0" inputmode="numeric" value={(*min_stock).clone()}
                                        oninput={let min_stock = min_stock.clone(); Callback::from(move |e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            let filtered: String = input.value().chars().filter(|c| c.is_ascii_digit()).collect();
                                            min_stock.set(filtered);
                                        })} />
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{ "ราคาทุน (บาท)" }</label>
                                    <input type="number" step="0.01" min="0" inputmode="decimal" value={(*cost_price).clone()}
                                        oninput={let cost_price = cost_price.clone(); Callback::from(move |e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            let filtered: String = input.value().chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
                                            cost_price.set(filtered);
                                        })} />
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{ "ราคาขาย (บาท)" }</label>
                                    <input type="number" step="0.01" min="0" inputmode="decimal" value={(*sell_price).clone()}
                                        oninput={let sell_price = sell_price.clone(); Callback::from(move |e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            let filtered: String = input.value().chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
                                            sell_price.set(filtered);
                                        })} />
                                </div>
                                
                                <div class="form-group" style="grid-column: 1 / -1;">
                                    <label class="form-label">{ "วิธีใช้เริ่มต้น" }</label>
                                    <input type="text" value={(*default_usage).clone()}
                                        placeholder="เช่น รับประทานครั้งละ 1 เม็ด วันละ 3 ครั้ง หลังอาหาร"
                                        oninput={let default_usage = default_usage.clone(); Callback::from(move |e: InputEvent| default_usage.set(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                                </div>
                                
                                <div class="form-group" style="grid-column: 1 / -1;">
                                    <label class="form-label">{ "คำเตือน" }</label>
                                    <input type="text" value={(*warning).clone()}
                                        placeholder="เช่น ห้ามดื่มแอลกอฮอล์, ทำให้ง่วงซึม"
                                        oninput={let warning = warning.clone(); Callback::from(move |e: InputEvent| warning.set(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                                </div>
                            </div>
                            
                            <div class="flex justify-between items-center mt-6">
                                <button type="button" class="btn btn-ghost" onclick={{
                                    let show_form = show_form.clone();
                                    let clear_form = clear_form.clone();
                                    move |_| {
                                        clear_form.emit(());
                                        show_form.set(false);
                                    }
                                }}>
                                    { "← ยกเลิก" }
                                </button>
                                <button type="submit" class="btn btn-primary btn-lg">
                                    { "💾 บันทึก" }
                                </button>
                            </div>
                        </form>
                    </div>
                }
            } else { html! {} }}
            
            // Drug List
            <div class="card">
                { if drugs.is_empty() {
                    html! {
                        <div class="empty-state">
                            <div class="empty-state-icon">{ "💊" }</div>
                            <h3 class="empty-state-title">{ "ยังไม่มีรายการยา" }</h3>
                            <p class="empty-state-text">{ "กด \"เพิ่มยาใหม่\" เพื่อเริ่มบันทึกรายการยา" }</p>
                        </div>
                    }
                } else {
                    html! {
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <th>{ "ชื่อยา" }</th>
                                    <th>{ "หมวดหมู่" }</th>
                                    <th>{ "คงเหลือ" }</th>
                                    <th>{ "ราคาขาย" }</th>
                                    <th>{ "จัดการ" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for drugs.iter().map(|d| {
                                    let is_low = d.stock <= d.min_stock;
                                    let drug_for_edit = d.clone();
                                    let drug_id = d.id.clone();
                                    let on_edit = on_edit.clone();
                                    let on_delete = on_delete.clone();
                                    
                                    html! {
                                        <tr key={d.id.clone()}>
                                            <td>
                                                <div class="font-bold">{ &d.name }</div>
                                                { if !d.warning.is_empty() {
                                                    html! { <div class="text-error" style="font-size: 0.9rem;">{ format!("⚠️ {}", d.warning) }</div> }
                                                } else { html! {} }}
                                            </td>
                                            <td>
                                                <span class="badge badge-accent">{ &d.category }</span>
                                            </td>
                                            <td>
                                                { if is_low {
                                                    html! { <span class="badge badge-error">{ format!("{} {}", d.stock, d.unit) }</span> }
                                                } else {
                                                    html! { <span>{ format!("{} {}", d.stock, d.unit) }</span> }
                                                }}
                                            </td>
                                            <td class="font-bold">{ format!("฿{:.2}", d.sell_price) }</td>
                                            <td>
                                                <div class="flex gap-2">
                                                    <button class="btn btn-secondary btn-sm" 
                                                        onclick={move |_| on_edit.emit(drug_for_edit.clone())}>
                                                        { "✏️ แก้ไข" }
                                                    </button>
                                                    <button class="btn btn-danger btn-sm"
                                                        onclick={move |_| on_delete.emit(drug_id.clone())}>
                                                        { "🗑️" }
                                                    </button>
                                                </div>
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
