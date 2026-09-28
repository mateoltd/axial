# Library lifecycle integration

Status: implemented private boundary; integration and native platform parity remain subject to shared verification.

`LibraryLifecycle` owns the isolated application root, selected library generation, retiring generations and unresolved external root acquisition. New admission closes during selection changes. Accepted generation pins, scoped file outcomes and retained Minecraft operations remain tied to the exact original root through settlement. The native adapter shares application-root authority across managed selections, preventing independent publication coordination for one physical root.

Application-root admission has its own pin count. Runtime cache construction creates the fixed `runtime` child through the retained filesystem implementation; cache clones, runtime components and launch receipts retain this application pin. They do not pin the selected external library. Shutdown/reset checks both application and generation pins before releasing native authority. Root sessions drop after their effect owners.

Prepared selections also pin the application root. Abandoning a prepared external root transfers it to explicit retirement instead of dropping its native session. A late completion cannot reopen admission after closure. An unsuccessful external acquisition remains owned and visibly blocks reset; `cleanup_failed_admission` attempts only the exact uncommitted acquisition cleanup and retains failure.

Composition uses `open_with_id(path, persisted_id)` or `from_root_session_at(session, persisted_id, path)`. Read projections validate the admitted physical directory before exposing process paths. `from_root_session` without a supplied projection remains available for capability-only consumers and cannot produce a launch path.

Focused lifecycle tests cover switch admission, escaped pins, external retirement, aborted preparation, closure versus late commit, cloned Minecraft operations/witnesses preventing reset, and a cloned runtime cache retaining the application root while an old external library is released. Requested shared command: `cargo test -p axial-app library::tests`.

Startup remains a typed ownership boundary: `LibraryOpenOutcome::Unresolved` carries a native root obligation. Transport must retain that obligation when refusing startup, or successfully acknowledge preservation. Converting an ambiguous failure directly to a string loses authority. State-successor recovery still requires the owning feature's durable receipt validation; this module does not replay arbitrary domain transactions. Full download/session/switch/reset failure matrices and installed-platform evidence are not yet complete.

`try_preserve` closes admission and settles managed roots after producers join. Composition settles its separate runtime cache first and retains it on error. Long-lived service pins remain valid; when all pins have gone, preservation attempts native root revocation and retains any ambiguous revocation outcome for retry. Focused checks cover startup refusal releasing a clean root and shutdown settlement while a runtime cache remains alive.

Native dialog/drop code can synchronously capture `ApplicationRootPin::admit_native_file(path, bound)`, then move its read-only `NativeFileAdmission` into a blocking task. Reading consumes the admission and checks the captured revision before and after bounded reads. A replaced selection is rejected without touching either file; native selected paths never become transport authority.
