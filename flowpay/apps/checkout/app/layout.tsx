import type {Metadata} from "next";
import type {ReactNode} from "react";
import "./style.scss";
export const metadata:Metadata={title:"FlowPay Checkout",description:"Secure crypto checkout and recovery by FlowPay"};
export default function Layout({children}:{children:ReactNode}){return <html lang="en"><body>{children}</body></html>}
