"use client";
import {useState} from "react";
import {CheckIcon,PlusIcon} from "../components/Icons";
import {CopyButton} from "../components/CopyButton";

type Key={id:string;name:string;prefix:string;revoked:boolean};

export function ApiKeysPanel({initialKeys}:{initialKeys:Key[]}){
  const [keys,setKeys]=useState(initialKeys);
  const [secret,setSecret]=useState("");
  const [busy,setBusy]=useState(false);
  const [error,setError]=useState("");
  async function create(){
    setBusy(true);setError("");
    try{const response=await fetch("/api/api-keys",{method:"POST",headers:{"content-type":"application/json"},body:JSON.stringify({name:`Dashboard key ${new Date().toLocaleDateString()}`})});const body=await response.json();if(!response.ok)throw new Error(body?.error?.message??body?.error??"Could not create key");setSecret(body.api_key);setKeys(current=>[{id:body.id,name:"Dashboard key",prefix:body.api_key.split(".")[0],revoked:false},...current]);}catch(value){setError(value instanceof Error?value.message:"Could not create key")}finally{setBusy(false)}
  }
  async function revoke(id:string){const response=await fetch(`/api/api-keys?id=${encodeURIComponent(id)}`,{method:"DELETE"});if(response.ok)setKeys(current=>current.map(key=>key.id===id?{...key,revoked:true}:key));}
  return <section className="panel api-key-panel"><div className="panel-head"><h2>API keys</h2><button className="btn primary" type="button" disabled={busy} onClick={()=>void create()}><PlusIcon/>{busy?"Creating…":"Create API key"}</button></div>{secret?<div className="secret-reveal"><div className="security-icon"><CheckIcon/></div><div><strong>API key created</strong><span>Copy it now. FlowPay will not show this secret again.</span><code>{secret}</code></div><CopyButton value={secret} label="Copy key"/></div>:null}{error?<div className="form-error">{error}</div>:null}<div className="table-wrap"><table className="table"><thead><tr><th>Name</th><th>Prefix</th><th>State</th><th/></tr></thead><tbody>{keys.map(key=><tr key={key.id}><td>{key.name}</td><td className="mono">{key.prefix}</td><td><span className={`status ${key.revoked?"bad":"success"}`}>{key.revoked?"Revoked":"Active"}</span></td><td>{!key.revoked?<button className="text-button danger" onClick={()=>void revoke(key.id)}>Revoke</button>:null}</td></tr>)}</tbody></table>{keys.length===0?<div className="empty-note">No API keys created yet.</div>:null}</div></section>;
}
