`AddressLink` shows raw/truncated addresses; resolving human-readable Soroban Domain names improves readability and trust everywhere addresses appear.

**Requirements**
- Attempt to resolve a name for the address in `AddressLink.tsx`; fall back to the truncated address
- Cache lookups to avoid repeat RPC calls; never block render on resolution
- Keep the StellarExpert link intact

**Suggested execution**
```bash
git checkout -b feat/address-domain-resolution
```
- Add a `useResolvedName(address)` hook with caching
- Resolve asynchronously and swap in the name when ready

**Files to touch**
- `frontend/src/components/AddressLink.tsx`
- `frontend/src/hooks/useSoroban.ts`

**Acceptance criteria**
- [ ] A named address shows the name; an unnamed one shows the truncation
- [ ] Lookups are cached and never block initial render

**Example commit message**
```
feat: resolve Soroban Domain names in AddressLink with fallback
```
`Closes #224`

