# Native interrupted reset

2026-10-04. Explicit Preserve-files acceptance passes for this disposable interrupted-reset fixture; this is not a parity certificate. Normal native Reset has earlier bounded evidence in [integration](integration.md).

## Current fixture

The unsigned macOS ARM64 bundle at `068a60a1`, frontend generation `15b620ae006a`, executable SHA256 `9232f4dbd08939f716fc00a4fb9b66cbf08a6a9ee867a0259e23160df5555ef8`, uses only generated profile `/private/tmp/axial-reset-current.YLMKr6/profile`. A fresh current API seeds the profile and exits normally before interruption setup. It has no accounts, instances or game processes. The existing `reset_tests::accepted_reset_crash_helper` prepares the real owner's reset intent and deliberately exits 73; no fence is handwritten or copied. This helper does not prove interruption of active game shutdown.

Seven file witnesses cover the profile marker, reset intent, root lease, SQLite database, top-level/nested payload canaries and outside sibling. The fingerprint records canonical path, size, device/inode, modification time, mode and SHA256, checking stability across each read. Evidence is under `.rewrite-logs/`.

## Qualified first attempt

Native PID58496 reports interrupted Reset and presents the system warning (`native-reset-current-runtime.log`). Root performs only UI discovery and a denied attempt to select UserNotificationCenter; it sends no affirmative click or key. Later, the intent and payload canaries disappear, the database is recreated, and the observed parent exits 0. Marker and outside sibling remain unchanged. The user confirms clicking or pressing a key while the warning was open, without identifying the exact choice. This run establishes neither an automatic-reset defect nor explicit acceptance of Reset or Preserve files. Its failed/empty waiting fingerprint is not preservation proof.

Independent source review finds confirmation gated by the existing first-choice atomic decision and completed dialog callback, with unconfirmed event-loop return preserving ownership. No confirmation-code change or diagnostic framework is justified by this externally influenced run.

## Explicit preservation attempt

Only the absent generated payload canaries are recreated. The same owner helper publishes a fresh intent and exits 73 (`native-reset-preserve-crash-helper.log`). Native PID76438 reports the interrupted reset and waits (`native-reset-preserve-runtime.log`). The post-helper baseline and waiting snapshot match exactly for all seven files (`native-reset-preserve-{before,waiting}.json`).

Computer use explicitly denies access to macOS UserNotificationCenter, which owns the parentless warning. The user completes the actual Preserve files request; Return can activate default Reset and is not a substitute. Native PID76438 exits 1, as expected for this preserved startup refusal, and the process inventory has no native successor. The log reports application files preserved at 18:15:09.311734Z. All seven post-exit witnesses match the baseline exactly (`native-reset-preserve-after.json`), including the retained intent and SQLite database. This passes the explicit choice and bounded file/process check, not every reset interruption boundary or installed-release recovery.

Hosted [run37222500584](https://github.com/mateoltd/axial/actions/runs/37222500584) passes both application and delivery-contract jobs for exact `068a60a1`. That CI does not prove this native choice, gameplay, installed-release behavior or the four-platform matrix. No legacy profile, unrelated data, release or deployment is changed.
