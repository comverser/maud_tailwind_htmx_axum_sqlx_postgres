use crate::{auth::CurrentUser, constants::{cdn, payment}, flash::FlashMessage, formatting::format_price, models::order::Order, paths, views::layout::base};
use maud::{Markup, PreEscaped, html};

pub fn checkout(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    order: &Order,
    client_key: &str,
) -> Markup {
    let success_url = format!("/actions/payment/verify");
    let fail_url = paths::with_param(paths::pages::QUOTE, "order_id", &order.order_id);

    let content = html! {
        div class="max-w-2xl mx-auto" {
            h1 class="text-3xl font-bold text-gray-900 mb-6" { "Checkout" }

            div class="bg-white rounded-lg shadow-md p-6 mb-6" {
                div class="space-y-2 mb-4" {
                    div class="flex justify-between text-sm" {
                        span class="text-gray-600" { "File" }
                        span class="font-medium text-gray-900" { (order.filename) }
                    }
                    div class="flex justify-between text-sm" {
                        span class="text-gray-600" { "Characters" }
                        span class="font-medium text-gray-900" { (order.text_length.to_string()) }
                    }
                }

                div class="border-t pt-4 mb-6" {
                    div class="flex justify-between items-center" {
                        span class="text-lg font-bold text-gray-900" { "Total" }
                        span class="text-2xl font-bold text-indigo-600" { "₩" (format_price(order.price_amount)) }
                    }
                }

                div id="payment-method" class="mb-6" {}
                div id="agreement" class="mb-6" {}

                button
                    id="payment-button"
                    class="w-full bg-indigo-600 text-white py-3 px-4 rounded-lg hover:bg-indigo-700 font-semibold disabled:bg-gray-400 disabled:cursor-not-allowed"
                    { "Pay Now" }
            }
        }

        script src=(cdn::TOSS_PAYMENTS_SDK_URL) {}
        script {
            (PreEscaped(format!(r#"
                const button = document.getElementById('payment-button');

                try {{
                    const tossPayments = TossPayments('{}');
                    button.disabled = false;

                    button.addEventListener('click', function() {{
                        console.log('Payment request parameters:', {{
                            amount: {},
                            orderId: '{}',
                            orderName: '{} - {}',
                            successUrl: window.location.origin + '{}',
                            failUrl: window.location.origin + '{}'
                        }});

                        tossPayments.requestPayment('카드', {{
                            amount: {},
                            orderId: '{}',
                            orderName: '{} - {}',
                            successUrl: window.location.origin + '{}',
                            failUrl: window.location.origin + '{}'
                        }})
                        .catch(function(error) {{
                            console.error('Payment request failed:', error);
                            alert('결제 요청 실패: ' + (error.message || error.code));
                        }});
                    }});
                }} catch (error) {{
                    console.error('Toss Payments initialization failed:', error);
                    button.disabled = true;
                    button.textContent = 'Payment Error';
                }}
            "#,
                client_key,
                order.price_amount,
                order.order_number,
                payment::ORDER_NAME_PREFIX,
                order.filename,
                success_url,
                fail_url,
                order.price_amount,
                order.order_number,
                payment::ORDER_NAME_PREFIX,
                order.filename,
                success_url,
                fail_url
            )))
        }
    };

    base::base_layout(current_user, flash, site_name, "Checkout", "Complete your payment", content)
}
