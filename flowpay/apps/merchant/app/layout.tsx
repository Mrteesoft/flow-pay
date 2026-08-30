import "./globals.scss";
import type {ReactNode} from "react";
import {BellIcon,SearchIcon} from "./components/Icons";
import {Sidebar} from "./components/Sidebar";
export default function Layout({children}:{children:ReactNode}){return <html lang="en"><body><div className="app-shell"><Sidebar/><div className="workspace"><header className="topbar"><div className="search"><SearchIcon/><span>Search payments or claims</span><kbd>⌘ K</kbd></div><div className="top-actions"><button aria-label="Notifications"><BellIcon/><span className="notification-dot"/></button><div className="top-avatar">LB</div></div></header><main className="main">{children}</main></div></div></body></html>}
