On mobile, the donate button scrolls out of view on long campaign pages, hurting conversion. The primary action should always be one tap away on small screens.

**Requirements**
- Add a sticky bottom bar on `campaign/[id]/page.tsx` for small screens with a "Donate" CTA
- Hide it on desktop and when the campaign is not active
- Ensure it doesn't overlap the footer or system UI (safe-area insets)

**Suggested execution**
```bash
git checkout -b feat/sticky-mobile-donate
```
- Add a responsive `StickyDonateBar.tsx` shown under a mobile breakpoint
- Open the existing `DonateModal` on tap

**Files to touch**
- `frontend/src/app/campaign/[id]/page.tsx`
- `frontend/src/components/StickyDonateBar.tsx` (new)

**Acceptance criteria**
- [ ] On mobile the CTA stays visible while scrolling and opens DonateModal
- [ ] On desktop it's absent
- [ ] It respects safe-area insets and doesn't overlap the footer

**Example commit message**
```
feat: add sticky mobile donate CTA to campaign detail
```
`Closes #222`

