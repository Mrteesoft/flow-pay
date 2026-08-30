"use client";
import {usePathname} from "next/navigation";
import {ClaimIcon,CodeIcon,HomeIcon,PaymentIcon,SettingsIcon} from "./Icons";
const items=[
  ["/","Overview",HomeIcon],
  ["/payments","Payments",PaymentIcon],
  ["/claims","Claims",ClaimIcon],
  ["/developers","Developers",CodeIcon],
  ["/settings","Settings",SettingsIcon],
] as const;
export function Sidebar(){const pathname=usePathname();return <aside className="sidebar">
  <a className="logo" href="/"><img src="/assets/flowpay-mark.svg" alt=""/><span>FlowPay</span></a>
  <nav>{items.map(([href,label,Icon])=>{const active=href==="/"?pathname===href:pathname.startsWith(href);return <a key={href} href={href} className={active?"active":""}><Icon/><span>{label}</span></a>})}</nav>
  <div className="sidebar-bottom"><div className="test-chip"><span/>Test mode</div><div className="merchant-mini"><div className="avatar">LB</div><div><strong>LumaBot</strong><span>Developer account</span></div></div></div>
</aside>}
