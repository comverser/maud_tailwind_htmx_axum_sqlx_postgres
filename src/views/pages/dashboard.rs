use crate::{
    auth::CurrentUser,
    flash::FlashMessage,
    models::order::{OrderStats, OrderSummary, PaymentStatus},
    paths,
    views::layout::base::base_layout,
};
use maud::{html, Markup};
use time::format_description::well_known::Rfc3339;

pub fn dashboard(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    email: &str,
    stats: OrderStats,
    recent_orders: Vec<OrderSummary>,
) -> Markup {
    let content = html! {
        div class="max-w-6xl mx-auto space-y-6" {
            h1 class="text-3xl font-bold" { "Dashboard" }

            div class="bg-white border rounded-lg p-6" {
                h2 class="text-lg font-semibold mb-2" { "Account" }
                p class="text-gray-600" { (email) }
            }

            div class="grid grid-cols-1 md:grid-cols-3 gap-4" {
                (stat_card("Total Orders", &stats.total_orders.to_string(), "text-blue-600"))
                (stat_card("Paid Orders", &stats.paid_orders_count.to_string(), "text-green-600"))
                (stat_card("Total Spent", &format!("₩{}", stats.total_spent), "text-purple-600"))
            }

            div class="bg-white border rounded-lg p-6" {
                h2 class="text-lg font-semibold mb-4" { "Recent Orders" }

                @if recent_orders.is_empty() {
                    p class="text-gray-500 text-center py-8" { "No orders yet" }
                } @else {
                    div class="overflow-x-auto" {
                        table class="w-full" {
                            thead class="border-b" {
                                tr {
                                    th class="text-left py-2 px-3" { "Order #" }
                                    th class="text-left py-2 px-3" { "File" }
                                    th class="text-right py-2 px-3" { "Characters" }
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

            div class="flex gap-3" {
                a href=(paths::pages::TEXT_ANALYZER)
                    class="inline-block px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                {
                    "New Analysis"
                }
                a href=(paths::pages::TODOS)
                    class="inline-block px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300"
                {
                    "View Todos"
                }
            }
        }
    };

    base_layout(current_user, flash, site_name, "Dashboard", "Your account dashboard", content)
}

fn stat_card(label: &str, value: &str, color_class: &str) -> Markup {
    html! {
        div class="bg-white border rounded-lg p-6" {
            h3 class="text-sm text-gray-600 mb-1" { (label) }
            p class={"text-2xl font-bold " (color_class)} { (value) }
        }
    }
}

fn order_row(order: &OrderSummary) -> Markup {
    let status_class = match order.payment_status {
        PaymentStatus::Paid => "text-green-600 bg-green-50",
        PaymentStatus::Pending => "text-yellow-600 bg-yellow-50",
        PaymentStatus::Failed => "text-red-600 bg-red-50",
        PaymentStatus::Cancelled => "text-gray-600 bg-gray-50",
    };

    let status_text = match order.payment_status {
        PaymentStatus::Paid => "Paid",
        PaymentStatus::Pending => "Pending",
        PaymentStatus::Failed => "Failed",
        PaymentStatus::Cancelled => "Cancelled",
    };

    let formatted_date = order.created_at
        .format(&Rfc3339)
        .unwrap_or_else(|_| String::from("Invalid date"));
    let date_parts: Vec<&str> = formatted_date.split('T').collect();
    let date_display = date_parts.first().unwrap_or(&"");

    html! {
        tr class="border-b hover:bg-gray-50" {
            td class="py-3 px-3" {
                a href=(paths::with_param(paths::pages::QUOTE, "order_id", &order.order_id))
                    class="text-blue-600 hover:underline"
                {
                    (order.order_number)
                }
            }
            td class="py-3 px-3 text-gray-600" { (order.filename) }
            td class="py-3 px-3 text-right text-gray-600" { (order.text_length) }
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
