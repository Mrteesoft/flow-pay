import {redirect} from "next/navigation";
import {api} from "../lib/api";
import {CheckoutClient} from "./pay/[id]/CheckoutClient";
import {Brand,LanguageButton} from "./components/Brand";
import {InfoIcon} from "./components/Icons";

type PaymentSummary={id:string};

export default async function Home({searchParams}:{searchParams:Promise<{payment_id?:string}>}){
  const query=await searchParams;
  if(query.payment_id)redirect(`/pay/${encodeURIComponent(query.payment_id)}`);

  try{
    const result=await api("/v1/payments?limit=1") as {data?:PaymentSummary[]};
    const paymentId=result.data?.[0]?.id;
    if(paymentId)return <CheckoutClient paymentId={paymentId} home/>;
  }catch{
    // Render an operational empty state below. No sample payment data is substituted.
  }

  return <main className="checkout-shell checkout-preview">
    <header className="payment-header"><span/><Brand/><LanguageButton/></header>
    <section className="checkout-empty">
      <div className="icon-bubble"><InfoIcon/></div>
      <span className="violet-label">FlowPay checkout</span>
      <h1>No active checkout</h1>
      <p>Create a payment from the merchant dashboard or open a valid FlowPay checkout link.</p>
    </section>
  </main>;
}
