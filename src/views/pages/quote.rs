use crate::{auth::CurrentUser, flash::FlashMessage, models::order::Order, paths, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn quote(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    order: &Order,
) -> Markup {
    let content = html! {
        div class="max-w-lg mx-auto" {
            h1 class="text-3xl font-bold text-gray-900 mb-6" { "Quote" }

            div class="bg-white rounded-lg shadow-md p-6 space-y-4" {
                div class="space-y-2" {
                    div class="flex justify-between text-sm" {
                        span class="text-gray-600" { "File" }
                        span class="font-medium text-gray-900" { (order.filename) }
                    }
                    div class="flex justify-between text-sm" {
                        span class="text-gray-600" { "Size" }
                        span class="font-medium text-gray-900" { (format_file_size(order.file_size)) }
                    }
                    div class="flex justify-between text-sm" {
                        span class="text-gray-600" { "Characters" }
                        span class="font-medium text-gray-900" { (order.text_length.to_string()) }
                    }
                }

                div class="border-t pt-4" {
                    div class="flex justify-between items-center" {
                        span class="text-lg font-bold text-gray-900" { "Total" }
                        span class="text-2xl font-bold text-indigo-600" { "₩" (format_price(order.price_amount)) }
                    }
                }

                form method="post" action=(paths::actions::PAYMENT_INITIATE) {
                    input type="hidden" name="order_id" value=(order.order_id.to_string());
                    button
                        type="submit"
                        class="w-full bg-indigo-600 text-white py-3 px-4 rounded-lg hover:bg-indigo-700 font-semibold"
                        { "Pay Now" }
                }
            }
        }
    };

    base_layout(current_user, flash, site_name, "Quote", "Review your quote", content)
}

fn format_file_size(bytes: i32) -> String {
    let bytes = bytes as f64;
    if bytes < 1024.0 {
        format!("{} B", bytes)
    } else if bytes < 1024.0 * 1024.0 {
        format!("{:.2} KB", bytes / 1024.0)
    } else {
        format!("{:.2} MB", bytes / (1024.0 * 1024.0))
    }
}

fn format_price(amount: i32) -> String {
    amount.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}
