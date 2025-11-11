use crate::{
    auth::CurrentUser,
    flash::FlashMessage,
    models::order::OrderSummary,
    paths,
    views::layout::base::base_layout,
};
use maud::{html, Markup};
use time::format_description::well_known::Rfc3339;

pub fn dashboard(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    recent_orders: Vec<OrderSummary>,
) -> Markup {
    let content = html! {
        div class="max-w-4xl mx-auto" {
            h1 class="text-2xl font-bold mb-4" { "Orders" }

            div class="bg-white border rounded-lg p-6" {

                @if recent_orders.is_empty() {
                    p class="text-gray-500 text-center py-8" { "No orders yet" }
                } @else {
                    div class="overflow-x-auto" {
                        table class="w-full" {
                            thead class="border-b" {
                                tr {
                                    th class="text-left py-2 px-3" { "Order #" }
                                    th class="text-right py-2 px-3" { "Price" }
                                    th class="text-center py-2 px-3" { "Status" }
                                    th class="text-center py-2 px-3" { "Date" }
                                }
                            }
                            tbody {
                                @for order in recent_orders {
                                    (order_row(&order))
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    base_layout(current_user, flash, site_name, "Orders", "Your order history", content)
}

fn order_row(order: &OrderSummary) -> Markup {
    let status_class = order.payment_status.css_class();
    let status_text = order.payment_status.display_text();

    let formatted_date = order.created_at
        .format(&Rfc3339)
        .unwrap_or_else(|_| String::from("Invalid date"));
    let datetime_parts: Vec<&str> = formatted_date.split('T').collect();
    let date_part = datetime_parts.first().unwrap_or(&"");
    let time_part = datetime_parts.get(1).and_then(|t| t.split('.').next()).unwrap_or("");
    let date_display = if !time_part.is_empty() {
        format!("{} {}", date_part, time_part)
    } else {
        date_part.to_string()
    };

    html! {
        tr class="border-b hover:bg-gray-50" {
            td class="py-3 px-3" {
                a href=(paths::with_param(paths::pages::QUOTE, "order_id", &order.order_id))
                    class="text-blue-600 hover:underline"
                {
                    (order.order_number)
                }
            }
            td class="py-3 px-3 text-right font-medium" { "₩" (order.price_amount) }
            td class="py-3 px-3 text-center" {
                span class={"px-2 py-1 rounded text-xs font-medium " (status_class)} {
                    (status_text)
                }
            }
            td class="py-3 px-3 text-center text-sm text-gray-600" { (date_display) }
        }
    }
}
