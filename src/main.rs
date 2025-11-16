use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Item {
    code: String,
    description: String,
    price: u32,
}

#[derive(Debug, Clone)]
struct SelectedItem {
    code: Arc<String>,
    item: Item,
    amount: f32,
}

fn main() {
    dioxus_desktop::launch::launch(app, vec![], dioxus_desktop::Config::default());
}

fn app() -> Element {
    let items = use_resource(|| async {
        let mut rdr = csv::Reader::from_path("items.csv").unwrap();
        let mut items = Vec::new();
        for result in rdr.deserialize() {
            let item: Item = result.unwrap();
            items.push(item);
        }
        items
    });

    let mut search_query = use_signal(|| String::new());
    let mut selected_items = use_signal(|| HashMap::<Arc<String>, SelectedItem>::new());
    let mut print_output = use_signal(|| String::new());

    let filtered_items = use_memo(move || {
        let query = search_query().to_lowercase();
        let all_items = items.value().as_ref().map(|r| (**r).to_vec()).unwrap_or_else(|| vec![]);
        if query.is_empty() {
            all_items
        } else {
            all_items
                .into_iter()
                .filter(|item| {
                    item.code.to_lowercase().contains(&query)
                        || item.description.to_lowercase().contains(&query)
                })
                .collect::<Vec<_>>()
        }
    });

    let total_cost = use_memo(move || {
        selected_items()
            .values()
            .map(|si| (si.item.price as f32) * si.amount)
            .sum::<f32>() as u32
    });

    rsx! {
        div { class: "container",
            h1 { "Market Shopping App" }

            input {
                r#type: "text",
                placeholder: "Search items...",
                value: "{search_query}",
                oninput: move |e| search_query.set(e.value())
            }

            div { class: "items-list",
                for item in filtered_items.read().clone() {
                    div { class: "item",
                        span { "{item.code}: {item.description} - {item.price} won" }
                        input {
                            r#type: "number",
                            step: "1",
                            placeholder: "Amount",
                            oninput: move |e| {
                                if let Ok(amount) = e.value().parse::<f32>() {
                                    if amount > 0.0 {
                                        let code_arc = Arc::new(item.code.clone());
                                        selected_items.write().insert(
                                            code_arc.clone(),
                                            SelectedItem {
                                                code: code_arc,
                                                item: item.clone(),
                                                amount,
                                            },
                                        );
                                    } else {
                                        let code_arc = Arc::new(item.code.clone());
                                        selected_items.write().remove(&code_arc);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            h2 { "Selected Items" }
            div { class: "selected-list",
                for (_code, si) in selected_items().clone().into_iter() {
                    div { class: "selected-item",
                        "{si.code} {si.item.description} ({si.amount} phần)"
                        input {
                            r#type: "number",
                            step: "1",
                            value: "{si.amount}",
                            oninput: move |e| {
                                let code = si.code.clone();
                                let item = si.item.clone();
                                if let Ok(amt) = e.value().parse::<f32>() {
                                    if amt > 0.0 {
                                        selected_items.write().insert(code.clone(), SelectedItem {
                                            code: code.clone(),
                                            item: item.clone(),
                                            amount: amt,
                                        });
                                    } else {
                                        selected_items.write().remove(&code);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            h2 { "Total Cost: {total_cost} won" }

            button {
                onclick: move |_| {
                    let selected = selected_items();
                    let output = selected.values().map(|si| {
                        format!("{} {} ({:.0} phần)", si.code, si.item.description, si.amount)
                    }).collect::<Vec<_>>().join("\n");
                    print_output.set(output);
                },
                "Print List"
            }

            if !print_output().is_empty() {
                pre { "{print_output}" }
            }
        }
    }
}
