import {CheckoutClient} from "./CheckoutClient";

export default async function PaymentPage({params}:{params:Promise<{id:string}>}){
  const {id}=await params;
  return <CheckoutClient paymentId={id}/>;
}
