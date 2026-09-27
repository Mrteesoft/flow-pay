# FlowPay frontends

This repository contains the two FlowPay Next.js applications. Deploy them as
separate Vercel projects from the same Git repository.

| Vercel project | Root Directory | Required environment variables |
| --- | --- | --- |
| FlowPay Merchant | `apps/merchant` | `FLOWPAY_API_URL`, `FLOWPAY_API_KEY` |
| FlowPay Checkout | `apps/checkout` | `FLOWPAY_API_URL`, `FLOWPAY_CHECKOUT_API_KEY` |

Production domains:

| Vercel project | Domain |
| --- | --- |
| FlowPay Checkout | `pixuno.xyz` and `www.pixuno.xyz` |
| FlowPay Merchant | `dashboard.pixuno.xyz` |

Set `FLOWPAY_API_URL=https://api.pixuno.xyz` in both Vercel projects. The
API keys are server-only variables; do not prefix them with `NEXT_PUBLIC_`.

For each Vercel project, import this repository, select the root directory from
the table, add the environment variables for Production and Preview, and
deploy. Vercel detects Next.js and uses `npm run build` automatically.

The backend must set `FLOWPAY_CHECKOUT_BASE_URL=https://pixuno.xyz` so every
payment response returns the customer URL as `https://pixuno.xyz/pay/<id>`.

Local verification:

```powershell
cd apps/merchant
Copy-Item .env.example .env.local
npm ci
npm run build

cd ../checkout
Copy-Item .env.example .env.local
npm ci
npm run build
```
