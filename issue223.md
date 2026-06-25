`profile/page.tsx` fetches user campaigns/donations but lacks dedicated loading and error UI, so a slow RPC looks like a blank or broken page.

**Requirements**
- Add skeletons while `get_campaigns_by_creator` and donation queries load
- Add an error state with retry when the RPC fails
- Handle the "wallet not connected" case with a connect prompt

**Suggested execution**
```bash
git checkout -b feat/profile-loading-error
```
- Derive `isLoading`/`isError` in the profile queries and branch the render
- Reuse `ui/skeleton.tsx` and the connect button

**Files to touch**
- `frontend/src/app/profile/page.tsx`
- `frontend/src/hooks/useSoroban.ts`

**Acceptance criteria**
- [ ] Throttling the RPC shows the error + retry
- [ ] Disconnecting shows the connect prompt
- [ ] Loading shows skeletons

**Example commit message**
```
feat: add loading, error, and disconnected states to Profile page
```
`Closes #223`

