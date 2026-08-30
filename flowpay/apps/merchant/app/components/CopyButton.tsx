"use client";
import {useState} from "react";
import {CheckIcon} from "./Icons";

export function CopyButton({value,label="Copy"}:{value:string;label?:string}){
  const [copied,setCopied]=useState(false);
  async function copy(){await navigator.clipboard.writeText(value);setCopied(true);window.setTimeout(()=>setCopied(false),1600)}
  return <button type="button" className="copy-button" onClick={()=>void copy()}>{copied?<><CheckIcon/>Copied</>:label}</button>;
}
