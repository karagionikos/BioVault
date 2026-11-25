<!-- Outside contributions aren't accepted: BioVault is all rights reserved (README § Copyright and license, D83). -->
<!-- Title: the feature ID or roadmap step, e.g. "F-04: derive paper status". One ID per PR. -->

Closes #

## What changed


## Done when (from docs/02-features-and-flows.md)

- [ ] 

## Definition of done (docs/09 § Definition of done)

- [ ] Every "done when" criterion above is met
- [ ] Tests at the right layers exist and pass; the sync simulator passes if this touches sync
- [ ] Works on both devices (from P2 on; before P2, on the desktop)
- [ ] Decision log and docs updated if this changed a decision (D-number: )

## Guardrails (docs/10 § Working with AI agents)

- [ ] CI is green
- [ ] No new dependency, or each new one is permissively licensed and listed in THIRD_PARTY_NOTICES.md
- [ ] Nothing from the never-commit list: model weights, signing keys, `.env`, vault data, exports, notebook outputs
- [ ] No human-owned area touched, or the owner has reviewed every line (sync merge rules, conflict detection, simulator assertions, link-model evaluation, the takeaway rule)
