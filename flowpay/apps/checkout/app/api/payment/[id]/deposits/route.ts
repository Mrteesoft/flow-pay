import {NextResponse} from "next/server";
import {api} from "../../../../../lib/api";
export async function GET(_:Request,{params}:{params:Promise<{id:string}>}){try{const {id}=await params;return NextResponse.json(await api(`/v1/payments/${encodeURIComponent(id)}/deposits`));}catch(e){return NextResponse.json({error:e instanceof Error?e.message:"Unable to load deposits"},{status:502});}}
