use yew::prelude::*;
use crate::models::Patient;
use crate::store::Store;
use yew_router::prelude::Link;
use crate::Route;
use web_sys::HtmlInputElement;

#[function_component(Search)]
pub fn search() -> Html {
    let patients = use_state(|| Store::get_patients());
    let search_term = use_state(|| String::new());
    
    let onsearch = {
        let search_term = search_term.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            search_term.set(input.value());
        })
    };
    
    let filtered_patients: Vec<Patient> = patients.iter().filter(|p| {
        let term = search_term.to_lowercase();
        p.hn.to_lowercase().contains(&term) ||
        p.first_name.to_lowercase().contains(&term) ||
        p.last_name.to_lowercase().contains(&term) ||
        p.citizen_id.contains(&term)
    }).cloned().collect();

    html! {
        <>
            <div class="page-header flex justify-between items-center">
                <div>
                    <h1 class="page-title">{ "รายชื่อผู้ป่วย" }</h1>
                    <p class="page-subtitle">{ format!("ทั้งหมด {} คน", patients.len()) }</p>
                </div>
                <Link<Route> to={Route::Register} classes="btn btn-primary">
                    { "+ ลงทะเบียนใหม่" }
                </Link<Route>>
            </div>
            
            <div class="card">
                <div class="search-input-wrap mb-6">
                    <span class="search-input-icon">{ "🔍" }</span>
                    <input type="search" 
                        placeholder="ค้นหาด้วยชื่อ, HN, หรือเลขบัตรประชาชน..." 
                        oninput={onsearch} />
                </div>
                
                { if filtered_patients.is_empty() {
                    html! {
                        <div class="empty-state">
                            <div class="empty-state-icon">{ "👥" }</div>
                            <h3 class="empty-state-title">{ "ไม่พบผู้ป่วย" }</h3>
                            <p class="empty-state-text">{ "ลองค้นหาใหม่ หรือลงทะเบียนผู้ป่วยใหม่" }</p>
                            <Link<Route> to={Route::Register} classes="btn btn-primary">
                                { "ลงทะเบียนผู้ป่วยใหม่" }
                            </Link<Route>>
                        </div>
                    }
                } else {
                    html! {
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <th>{ "HN" }</th>
                                    <th>{ "ชื่อ-นามสกุล" }</th>
                                    <th>{ "เลขบัตรประชาชน" }</th>
                                    <th>{ "แพ้ยา" }</th>
                                    <th>{ "จัดการ" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for filtered_patients.iter().map(|p| {
                                    let allergy = if p.drug_allergy.is_empty() || p.drug_allergy.to_lowercase() == "none" || p.drug_allergy == "ไม่มี" {
                                        html! { <span class="text-muted">{ "ไม่มี" }</span> }
                                    } else {
                                        html! { <span class="badge badge-error">{ &p.drug_allergy }</span> }
                                    };
                                    
                                    html! {
                                        <tr key={p.id.clone()}>
                                            <td>
                                                <span class="font-mono font-semibold">{ &p.hn }</span>
                                            </td>
                                            <td>
                                                <div class="font-semibold">{ format!("{}{} {}", p.title, p.first_name, p.last_name) }</div>
                                            </td>
                                            <td class="font-mono">{ &p.citizen_id }</td>
                                            <td>{ allergy }</td>
                                            <td>
                                                <div class="flex gap-2">
                                                    <Link<Route> to={Route::Treatment { id: p.id.clone() }} classes="btn btn-primary btn-sm">
                                                        { "💊 รักษา" }
                                                    </Link<Route>>
                                                    <Link<Route> to={Route::History { id: p.id.clone() }} classes="btn btn-secondary btn-sm">
                                                        { "📋 ประวัติ" }
                                                    </Link<Route>>
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
