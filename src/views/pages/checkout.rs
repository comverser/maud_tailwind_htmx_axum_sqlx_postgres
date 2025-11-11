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
        div class="max-w-lg mx-auto" {
            h1 class="text-xl mb-3" { "Checkout" }

            div class="space-y-3" {
                div class="space-y-1 text-sm" {
                    div class="flex justify-between" {
                        span class="text-gray-600" { "File" }
                        span { (order.filename) }
                    }
                    div class="flex justify-between" {
                        span class="text-gray-600" { "Characters" }
                        span { (order.text_length.to_string()) }
                    }
                }

                div class="border-t pt-3 mb-3" {
                    div class="flex justify-between items-center" {
                        span { "Total" }
                        span class="text-xl text-indigo-600" { "₩" (format_price(order.price_amount)) }
                    }
                }

                div id="payment-method" class="mb-3" {}
                div id="agreement" class="mb-3" {}

                button
                    id="payment-button"
                    class="w-full bg-indigo-600 text-white px-3 py-2 hover:bg-indigo-700 disabled:bg-gray-400 disabled:cursor-not-allowed"
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

                    const paymentParams = {{
                        amount: {},
                        orderId: '{}',
                        orderName: '{} - {}',
                        successUrl: window.location.origin + '{}',
                        failUrl: window.location.origin + '{}'
                    }};

                    button.addEventListener('click', function() {{
                        console.log('Payment request parameters:', paymentParams);

                        tossPayments.requestPayment('카드', paymentParams)
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
                fail_url
            )))
        }
    };

    base::base_layout(current_user, flash, site_name, "Checkout", "Complete your payment", content)
}
